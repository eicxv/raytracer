use rand::Rng;
use raytrace_rust::material::scatterable::Scatterable;
use raytrace_rust::shape::hittable::Hittable;
use std::f64::INFINITY;
use std::path::Path;
use std::time::Instant;

use rayon::prelude::*;
use raytrace_rust::bvh::bvh::split_method::SurfaceArea;
use raytrace_rust::bvh::bvh::Bvh;
use raytrace_rust::create_scene::create_book_1_final_scene;
use raytrace_rust::ray::Ray;
use raytrace_rust::utility::lerp;
use raytrace_rust::vec3::Vec3;

fn main() {
    let start = Instant::now();
    let size = (400, 200);
    let samples = 100;
    let buffer = render(size.0, size.1, samples);
    let duration = start.elapsed();
    println!("Time: {}", duration.as_secs_f64());
    let path = Path::new("./renders/img.png");
    save_png(path, size, &buffer);
}

fn render(width: u32, height: u32, samples_per_pixel: u32) -> Vec<u8> {
    let (camera, mut world) = create_book_1_final_scene(width as f64, height as f64);
    let bvh = Bvh::build(&mut world, SurfaceArea);

    let w = width as f64;
    let h = height as f64;
    let s = samples_per_pixel as f64;

    let pixel_iter = (0..height).into_par_iter().rev().flat_map(|j| {
        (0..width)
            .into_par_iter()
            .map(move |i| (i as f64, j as f64))
    });

    pixel_iter
        .flat_map(|(i, j)| {
            let mut rng = rand::thread_rng();
            let mut col = Vec3::origin();
            for _ in 0..samples_per_pixel {
                let u = (i + rng.gen::<f64>()) / w;
                let v = (j + rng.gen::<f64>()) / h;
                let r = camera.get_ray((u, v));
                col += color(&r, &bvh, 0) * 1.0 / s;
            }
            to_srgb_bytes(col)
        })
        .collect()
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
