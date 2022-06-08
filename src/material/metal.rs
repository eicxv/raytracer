use super::scatterable::Scatterable;
use crate::{ray::Ray, shape::hittable::HitRecord, vec3::Vec3};

#[derive(Clone, Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub roughness: f64,
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
