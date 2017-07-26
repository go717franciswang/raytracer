extern crate image;

pub mod scene;

use image::{GenericImage, DynamicImage};
use scene::{Vec3, Sphere, Scene};

const MAX_RAY_DEPTH: u32 = 5;

pub fn trace(rayorig: Vec3, raydir: Vec3, spheres: &Vec<Sphere>, depth: u32) -> Vec3 {
    let mut tnear = std::f32::INFINITY;
    let mut sphere: Option<&Sphere> = None;
    for s in spheres {
        let mut t0 = std::f32::INFINITY;
        let mut t1 = std::f32::INFINITY;
        if s.intersect(rayorig, raydir, &mut t0, &mut t1) {
            if t0 < 0.0 { t0 = t1 }
            if t0 < tnear {
                tnear = t0;
                sphere = Some(s);
            }
        }
    }

    if sphere.is_none() {
        return Vec3::new(2.0);
    }
    let sphere = sphere.unwrap();

    let mut surface_color = Vec3::new(0.0);
    let phit = rayorig + raydir * tnear;
    let mut nhit = phit - sphere.center;
    nhit.normalize();
    let bias = 1e-4_f32;
    let mut inside = false;

    if raydir.dot(nhit) > 0.0 {
        nhit = nhit * -1.0;
        inside = true;
    }

    if (sphere.transparency > 0.0 || sphere.reflection > 0.0) && depth < MAX_RAY_DEPTH {
        let facingratio: f32 = (raydir*-1.0).dot(nhit);
        let fresneleffect: f32 = mix((1.0 - facingratio).powi(3), 1.0, 0.1);
        let mut refldir = raydir - nhit * 2.0 * raydir.dot(nhit);
        refldir.normalize();
        let reflection = trace(phit + nhit * bias, refldir, spheres, depth + 1);
        let mut refraction = Vec3::new(0.0);

        if sphere.transparency != 0.0 {
            let ior = 1.1;
            let eta = if inside { ior } else { 1.0 / ior };
            let cosi = nhit.dot(raydir) * -1.0;
            let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
            let mut refrdir = raydir * eta + nhit * (eta * cosi - k.sqrt());
            refrdir.normalize();
            refraction = trace(phit - nhit * bias, refrdir, spheres, depth + 1);
        }

        surface_color = sphere.surface_color * (reflection * fresneleffect + refraction * (1.0 - fresneleffect) * sphere.transparency);
    } else {
        for (i, s) in spheres.iter().enumerate() {
            let mut transmission = 1.0;
            let mut light_direction = s.center - phit;
            light_direction.normalize();

            for (j, s2) in spheres.iter().enumerate() {
                if i != j {
                    let mut t0 = 0.0;
                    let mut t1 = 0.0;
                    if s2.intersect(phit + nhit * bias, light_direction, &mut t0, &mut t1) {
                        transmission = 0.0;
                        break;
                    }
                }
            }

            surface_color = surface_color + sphere.surface_color * transmission * (s.emission_color * nhit.dot(light_direction).max(0.0));
        }
    }

    surface_color + sphere.emission_color
}

fn mix(a: f32, b: f32, mix: f32) -> f32 {
    b * mix + a * (1.0 - mix)
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let angle = (std::f32::consts::PI * 0.5 * scene.fov / 180.).tan();
    let inv_width = 1.0 / scene.width as f32;
    let inv_height = 1.0 / scene.height as f32;
    let aspect_ratio = scene.width as f32 / scene.height as f32;

    for x in 0..scene.width {
        for y in 0..scene.height {
            let xx = (2.0 * ((x as f32 + 0.5) * inv_width) - 1.0) * angle * aspect_ratio;
            let yy = (1.0 - 2.0 * ((y as f32 + 0.5) * inv_height)) * angle;
            let mut raydir = Vec3 { x: xx, y: yy, z: -1.0 };
            raydir.normalize();
            image.put_pixel(x, y, trace(Vec3::new(0.0), raydir, &scene.spheres, 0).to_rgba());
        }
    }
    image
}
