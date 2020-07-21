use crate::{random, Ray3, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        up: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
    ) -> Self {
        let h = (vfov / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - look_at).normalized();
        let u = up.cross(w).normalized();
        let v = w.cross(u);

        let focus_dist = (look_at - origin).length();

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = -horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray3 {
        let rd = self.lens_radius * random::vec3_in_unit_disc();
        let offset = self.u * rd.0 + self.v * rd.1;
        Ray3 {
            origin: self.origin + offset,
            direction: (self.lower_left_corner + s * self.horizontal + t * self.vertical - offset)
                .normalized(),
        }
    }
}
