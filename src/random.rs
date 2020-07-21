use crate::Vec3;
use rand::prelude::*;
use std::f64::consts::TAU;

pub fn vec3_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = Vec3(rng.gen(), rng.gen(), rng.gen()) * 2.0 - Vec3(1.0, 1.0, 1.0);
        if v.length_squared() <= 1.0 {
            return v;
        }
    }
}

pub fn vec3_in_unit_disc() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = Vec3(rng.gen(), rng.gen(), 0.0) * 2.0 - Vec3(1.0, 1.0, 0.0);
        if v.length_squared() <= 1.0 {
            return v;
        }
    }
}

pub fn unit_vec3() -> Vec3 {
    let mut rng = rand::thread_rng();
    let a = rng.gen::<f64>() * TAU;
    let z = rng.gen::<f64>() * 2.0 - 1.0;
    let r = (1.0 - z.powi(2)).sqrt();
    Vec3(r * a.cos(), r * a.sin(), z)
}
