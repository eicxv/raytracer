mod camera;
mod geometry;
mod ray;
mod utility;
mod vec3;
use camera::Camera;
use geometry::hittable::Hittable;
use geometry::material::{Dielectric, Lambertian, Metal};
use geometry::sphere::Sphere;
use rand::Rng;
use ray::Ray;
use std::path::Path;
use utility::lerp;
use vec3::Vec3;
fn main() {
    let size = (200, 100);
    let buffer = render(size.0, size.1);
    let path = Path::new("./renders/img-3.png");
    save_png(path, size, &buffer);
}

fn render(width: u32, height: u32) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut buffer = Vec::with_capacity(width as usize * height as usize * 3);
    let camera = Camera {
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::origin(),
    };
    let ns = 100;
    let world = [
        Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian {
                albedo: Vec3::new(0.1, 0.2, 0.5),
            }),
        ),
        Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian {
                albedo: Vec3::new(0.8, 0.8, 0.0),
            }),
        ),
        Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal {
                albedo: Vec3::new(0.8, 0.6, 0.2),
                roughness: 0.2,
            }),
        ),
        Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(Dielectric {
                index_of_refraction: 1.5,
            }),
        ),
        Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Box::new(Dielectric {
                index_of_refraction: 1.5,
            }),
        ),
    ];

    for j in (0..height).rev() {
        for i in 0..width {
            let mut col = Vec3::origin();
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / width as f64;
                let v = (j as f64 + rng.gen::<f64>()) / height as f64;
                let r = camera.get_ray((u, v));
                col += color(&r, &world.as_slice(), 0) * 1.0 / (ns as f64);
            }
            buffer.extend_from_slice(&to_srgb_bytes(col));
        }
    }
    buffer
}

fn color<T: Hittable>(ray: &Ray, scene: &T, depth: i32) -> Vec3 {
    let max_depth = 50;
    match scene.hit(ray, (0.0001, f64::INFINITY)) {
        Some(rec) => {
            if depth >= max_depth {
                return Vec3::origin();
            }
            match rec.material.scatter(ray, rec) {
                Some((scattered, attenuation)) => {
                    return attenuation * color(&scattered, scene, depth + 1);
                }
                None => return Vec3::origin(),
            }
        }
        None => (),
    };

    let dir = ray.direction.unitize();
    let t = 0.5 * dir.y + 1.0;
    lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
}

fn to_srgb_bytes(v: Vec3) -> [u8; 3] {
    [
        (v.x.sqrt() * 255.99) as u8,
        (v.y.sqrt() * 255.99) as u8,
        (v.z.sqrt() * 255.99) as u8,
    ]
}

fn save_png(path: &Path, size: (u32, u32), buffer: &[u8]) {
    image::save_buffer(path, buffer, size.0, size.1, image::ColorType::Rgb8).unwrap()
}
