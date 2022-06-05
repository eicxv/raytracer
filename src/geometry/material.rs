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
        if Vec3::dot(reflected, record.normal) < 0.0 {
            return None;
        }
        Some((scattered, self.albedo))
    }
}
