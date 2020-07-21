use crate::{random, Hit, Ray3, Vec3};

pub trait Material {
    fn scatter(&self, ray: Ray3, hit: Hit) -> Scatter;
}

pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray3,
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray3, hit: Hit) -> Scatter {
        Scatter {
            attenuation: self.albedo,
            scattered: Ray3 {
                origin: hit.point,
                direction: (hit.normal + random::unit_vec3()).normalized(),
            },
        }
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: Ray3, hit: Hit) -> Scatter {
        Scatter {
            attenuation: self.albedo,
            scattered: Ray3 {
                origin: hit.point,
                direction: (ray.direction.reflect(hit.normal)
                    + self.fuzz * random::vec3_in_unit_sphere())
                .normalized(),
            },
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray3, hit: Hit) -> Scatter {
        let e = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta = (-ray.direction.dot(hit.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let direction = if e * sin_theta > 1.0
            || schlik(cos_theta, e) > rand::random::<f64>()
        {
            ray.direction.reflect(hit.normal)
        } else {
            let perpendicular = e * (ray.direction + cos_theta * hit.normal);
            let parallel = -(1.0 - perpendicular.length_squared()).abs().sqrt() * hit.normal;
            perpendicular + parallel
        };

        Scatter {
            attenuation: Vec3(1.0, 1.0, 1.0),
            scattered: Ray3 {
                origin: hit.point,
                direction: direction.normalized(),
            },
        }
    }
}

fn schlik(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
