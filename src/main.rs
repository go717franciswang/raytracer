extern crate image;
extern crate raytracer;

use raytracer::scene::{Vec3, Scene, Sphere};
use std::fs::OpenOptions;
use image::ImageFormat;

fn main() {
    let scene = Scene {
        width: 1920,
        height: 1200,
        fov: 30.0,
        spheres: vec![
            Sphere::new(Vec3{x:  0.0, y: -10004.0, z: -20.0}, 10000.0, Vec3{x: 0.20, y: 0.20, z: 0.20}, 0.0, 0.0, 0.0),
            Sphere::new(Vec3{x:  0.0, y:      0.0, z: -20.0},     4.0, Vec3{x: 1.00, y: 0.32, z: 0.36}, 1.0, 0.5, 0.0),
            Sphere::new(Vec3{x:  5.0, y:     -1.0, z: -15.0},     2.0, Vec3{x: 0.90, y: 0.76, z: 0.46}, 1.0, 0.0, 0.0),
            Sphere::new(Vec3{x:  5.0, y:      0.0, z: -25.0},     3.0, Vec3{x: 0.65, y: 0.77, z: 0.97}, 1.0, 0.0, 0.0),
            Sphere::new(Vec3{x: -5.5, y:      0.0, z: -15.0},     3.0, Vec3{x: 0.90, y: 0.90, z: 0.90}, 0.5, 0.1, 0.0),
            Sphere::new(Vec3{x:  0.0, y:     20.0, z: -30.0},     3.0, Vec3{x: 0.00, y: 0.00, z: 0.00}, 0.0, 0.0, 3.0),
        ]
    };

    let image = raytracer::render(&scene);
    let image_path = "test.png";
    let mut image_file = OpenOptions::new()
        .write(true).truncate(true).create(true).open(image_path).unwrap();
    image.save(&mut image_file, ImageFormat::PNG).unwrap();
}
