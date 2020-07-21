use raytrace::{Vec3, Camera, render, Hittable, Sphere, Lambertian, Metal, Dielectric};
use rand::prelude::*;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let mut rng = rand::thread_rng();

    let width = 640;
    let height = 480;

    let camera = Camera::new(
        Vec3(13.0, 2.0, 3.0),
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        20f64.to_radians(),
        (width as f64) / (height as f64),
        0.1,
    );

    let mut scene = Vec::<Box<dyn Hittable>>::new();

    scene.push(Box::new(Sphere {
        center: Vec3(0.0, -1000.0, -1.0),
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: Vec3(0.5, 0.5, 0.5),
        }),
    }));

    for a in -11..=11 {
        for b in -11..=11 {
            let center = Vec3((a as f64) + 0.9 * rng.gen::<f64>(), 0.2, (b as f64) + 0.9*rng.gen::<f64>());

            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat: f64 = rng.gen();
                if mat < 0.8 {
                    scene.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Lambertian {
                            albedo: Vec3(rng.gen(), rng.gen(), rng.gen()) * Vec3(rng.gen(), rng.gen(), rng.gen()),
                        })
                    }));
                } else if mat < 0.95 {
                    scene.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metal {
                            albedo: Vec3(rng.gen(), rng.gen(), rng.gen()) * 0.5 + Vec3(0.5, 0.5, 0.5),
                            fuzz: rng.gen::<f64>() * 0.5,
                        })
                    }));
                } else {
                    scene.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric {
                            refraction_index: 1.5,
                        })
                    }));
                }
            }
        }
    }

    scene.push(Box::new(Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric {
            refraction_index: 1.5,
        }),
    }));

    scene.push(Box::new(Sphere {
        center: Vec3(-4.0, 1.0, -0.0),
        radius: 1.0,
        material: Box::new(Lambertian {
            albedo: Vec3(0.4, 0.2, 0.1),
        }),
    }));

    scene.push(Box::new(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal {
            albedo: Vec3(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    let pixels = render(width, height, &camera, &scene);

    let file = File::create("output.png").unwrap();
    let mut encoder = png::Encoder::new(BufWriter::new(file), width as u32, height as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()
        .unwrap()
        .write_image_data(&pixels)
        .unwrap();
}
