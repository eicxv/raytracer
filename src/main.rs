use rand::Rng;
use raytrace_rust::material::scatterable::Scatterable;
use raytrace_rust::shape::hittable::Hittable;
use std::f64::INFINITY;
use std::path::Path;
use std::time::Instant;

use raytrace_rust::bvh::bvh::Bvh;
use raytrace_rust::create_scene::create_book_1_final_scene;
use raytrace_rust::ray::Ray;
use raytrace_rust::utility::lerp;
use raytrace_rust::vec3::Vec3;

fn main() {
    let start = Instant::now();
    let size = (200, 100);
    let buffer = render(size.0, size.1);
    let duration = start.elapsed();
    println!("Time: {}", duration.as_secs_f64());
    let path = Path::new("./renders/img.png");
    save_png(path, size, &buffer);
}

fn render(width: u32, height: u32) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(width as usize * height as usize * 3);

    let ns = 100;
    let (camera, world) = create_book_1_final_scene(width as f64, height as f64);
    let bvh = Bvh::new(world);

    let mut rng = rand::thread_rng();
    for j in (0..height).rev() {
        for i in 0..width {
            let mut col = Vec3::origin();
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / width as f64;
                let v = (j as f64 + rng.gen::<f64>()) / height as f64;
                let r = camera.get_ray((u, v));
                col += color(&r, &bvh, 0) * 1.0 / (ns as f64);
            }
            buffer.extend_from_slice(&to_srgb_bytes(col));
        }
    }
    buffer
}

fn color<T: Hittable>(ray: &Ray, scene: &T, depth: i32) -> Vec3 {
    let max_depth = 50;
    if depth >= max_depth {
        return Vec3::origin();
    }
    match scene.hit(ray, 0.0001..INFINITY) {
        Some(rec) => match rec.material.scatter(ray, rec) {
            Some((scattered, attenuation)) => {
                return attenuation * color(&scattered, scene, depth + 1);
            }
            None => return Vec3::origin(),
        },
        None => (),
    };

    let dir = ray.direction.unitize();
    let t = 0.5 * dir.y + 1.0;
    lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
}

fn to_srgb_bytes(v: Vec3) -> [u8; 3] {
    [
        (linear_srgb_transfer_function(v.x) * 255.99) as u8,
        (linear_srgb_transfer_function(v.y) * 255.99) as u8,
        (linear_srgb_transfer_function(v.z) * 255.99) as u8,
    ]
}

fn linear_srgb_transfer_function(linear: f64) -> f64 {
    if linear < 0.0031308 {
        linear * 12.92
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    }
}

fn save_png(path: &Path, size: (u32, u32), buffer: &[u8]) {
    image::save_buffer(path, buffer, size.0, size.1, image::ColorType::Rgb8).unwrap()
}
