use crate::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray3 {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray3 {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
