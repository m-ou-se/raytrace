#![feature(tau_constant)]

mod camera;
mod hit;
mod material;
mod random;
mod ray;
mod vec;

pub use camera::Camera;
pub use hit::{Hit, HitList, Hittable, Sphere};
pub use material::{Dielectric, Lambertian, Material, Metal};
pub use ray::Ray3;
pub use vec::Vec3;

use rand::prelude::*;
use std::f64::INFINITY;

pub fn render(w: usize, h: usize, camera: &Camera, scene: &HitList) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let mut pixels = Vec::with_capacity(w * h * 4);

    let samples_per_pixel = 50;
    let max_depth = 50;

    for y in 0..h {
        for x in 0..w {
            let mut color = Vec3::ZERO;
            for _ in 0..samples_per_pixel {
                let x_offset: f64 = rng.gen();
                let y_offset: f64 = rng.gen();
                let s = (x as f64 + x_offset) / w as f64;
                let t = ((h - y) as f64 - y_offset) / h as f64;
                color += render_pixel(&scene, camera.ray(s, t), max_depth);
            }
            color /= samples_per_pixel as f64;
            color = Vec3(color.0.sqrt(), color.1.sqrt(), color.2.sqrt());
            pixels.extend_from_slice(&color.into_pixel());
        }
        println!("{:>5} / {}", y + 1, h);
    }

    pixels
}

fn render_pixel(scene: &HitList, ray: Ray3, depth: usize) -> Vec3 {
    if let Some((hit, mat)) = scene.hit(ray, 0.00001..INFINITY) {
        if depth > 0 {
            let s = mat.scatter(ray, hit);
            return s.attenuation * render_pixel(scene, s.scattered, depth - 1);
        } else {
            return Vec3::ZERO;
        }
    }
    let t = 0.5 * (ray.direction.1 + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}
