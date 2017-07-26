use std::ops;
use image::{Pixel, Rgba};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32) -> Vec3 {
        Vec3 { x: x, y: x, z: x }
    }

    pub fn dot(&self, v: Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn length2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) {
        let nor2 = self.length2();
        if nor2 > 0.0 {
            let inv_nor2 = 1.0 / nor2.sqrt();
            self.x *= inv_nor2;
            self.y *= inv_nor2;
            self.z *= inv_nor2;
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(
            (self.x.min(1.0) * 255.0) as u8,
            (self.y.min(1.0) * 255.0) as u8,
            (self.z.min(1.0) * 255.0) as u8,
            255)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: f32) -> Vec3 {
        Vec3 {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v
        }
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub radius2: f32,
    pub surface_color: Vec3,
    pub emission_color: Vec3,
    pub transparency: f32,
    pub reflection: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, surface_color: Vec3, transparency: f32, reflection: f32, e: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            radius2: radius * radius,
            surface_color: surface_color,
            emission_color: Vec3::new(e),
            transparency: transparency,
            reflection: reflection,
        }
    }

    pub fn intersect(&self, rayorig: Vec3, raydir: Vec3, t0: &mut f32, t1: &mut f32) -> bool {
        let l: Vec3 = self.center - rayorig;
        let tca: f32 = l.dot(raydir);
        if tca < 0.0 { return false; }
        let d2: f32 = l.dot(l) - tca * tca;
        if d2 > self.radius2 { return false; }
        let thc: f32 = (self.radius2 - d2).sqrt();
        *t0 = tca - thc;
        *t1 = tca + thc;

        true
    }
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub spheres: Vec<Sphere>,
}
