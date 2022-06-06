use rand::Rng;

use crate::{ray::Ray, vec3::Vec3};

use super::{hittable::HitRecord, scatterable::Scatterable};

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
        match Vec3::dot(reflected, record.normal) >= 0.0 {
            true => Some((scattered, self.albedo)),
            false => None,
        }
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray: &Ray, record: HitRecord) -> Option<(Ray, Vec3)> {
        let (outward_normal, ior_ratio, cosine) =
            match Vec3::dot(ray.direction, record.normal) > 0.0 {
                true => (
                    -record.normal,
                    self.index_of_refraction,
                    self.index_of_refraction * Vec3::dot(ray.direction.unitize(), record.normal),
                ),
                false => (
                    record.normal,
                    1.0 / self.index_of_refraction,
                    -Vec3::dot(ray.direction.unitize(), record.normal),
                ),
            };

        let scattered = match ray.direction.refract(outward_normal, ior_ratio) {
            Some(refracted) => match rand::thread_rng().gen::<f64>() < schlick(cosine, ior_ratio) {
                true => Vec3::reflect(&ray.direction, record.normal),
                false => refracted,
            },
            None => Vec3::reflect(&ray.direction, record.normal),
        };
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        Some((Ray::new(record.point, scattered), attenuation))
    }
}

fn schlick(cosine: f64, ior: f64) -> f64 {
    let r0 = ((1.0 - ior) / (1.0 + ior)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
