use super::{hittable::HitRecord, scatterable::Scatterable};
use crate::{ray::Ray, vec3::Vec3};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

#[derive(Clone, Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub roughness: f64,
}

#[derive(Clone, Debug)]
pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, record: HitRecord) -> Option<(Ray, Vec3)> {
        let target = record.point + record.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(record.point, target - record.point);
        Some((scattered, self.albedo))
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, record: HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = ray.direction.reflect(record.normal);
        let scattered = Ray::new(
            record.point,
            reflected + self.roughness * Vec3::random_unit_vector(),
        );
        match Vec3::dot(scattered.direction, record.normal) > 0.0 {
            true => Some((scattered, self.albedo)),
            false => None,
        }
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray: &Ray, record: HitRecord) -> Option<(Ray, Vec3)> {
        let (ior_ratio, normal) = if front_face(ray.direction, record.normal) {
            (self.index_of_refraction, -record.normal)
        } else {
            (1.0 / self.index_of_refraction, record.normal)
        };

        let unit_direction = ray.direction.unitize();
        let cos_theta = -Vec3::dot(unit_direction, normal);

        let scattered = if schlick(cos_theta, ior_ratio) > rand::thread_rng().gen::<f64>() {
            Vec3::reflect(&ray.direction, normal)
        } else {
            match ray.direction.refract(normal, ior_ratio) {
                Some(refracted) => refracted,
                None => Vec3::reflect(&ray.direction, normal),
            }
        };

        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        Some((Ray::new(record.point, scattered), attenuation))
    }
}

fn schlick(cosine: f64, ior_ratio: f64) -> f64 {
    let r0 = ((1.0 - ior_ratio) / (1.0 + ior_ratio)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn front_face(direction: Vec3, normal: Vec3) -> bool {
    Vec3::dot(direction, normal) > 0.0
}
