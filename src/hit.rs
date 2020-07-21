use crate::{Material, Ray3, Vec3};
use std::ops::Range;

pub trait Hittable {
    fn hit(&self, ray: Ray3, t_range: Range<f64>) -> Option<(Hit, &dyn Material)>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray3, t_range: Range<f64>) -> Option<(Hit, &dyn Material)> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let d = half_b.powi(2) - a * c;
        if d > 0.0 {
            let root = d.sqrt();
            for &root in &[root, -root] {
                let t = (-half_b - root) / a;
                if t_range.contains(&t) {
                    let point = ray.at(t);
                    return Some((Hit {
                        point,
                        normal: root.signum() * (point - self.center) / self.radius,
                        t,
                        front_face: root >= 0.0,
                    }, &*self.material));
                }
            }
        }
        None
    }
}

pub type HitList = [Box<dyn Hittable>];

impl Hittable for HitList {
    fn hit(&self, ray: Ray3, mut t_range: Range<f64>) -> Option<(Hit, &dyn Material)> {
        let mut hit = None;
        for thing in self {
            if let Some(h) = thing.hit(ray, t_range.clone()) {
                t_range.end = h.0.t;
                hit = Some(h);
            }
        }
        hit
    }
}
