use super::scatterable::Scatterable;
use crate::{ray::Ray, shape::hittable::HitRecord, vec3::Vec3};

#[derive(Clone, Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, record: HitRecord) -> Option<(Ray, Vec3)> {
        let target = record.point + record.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(record.point, target - record.point);
        Some((scattered, self.albedo))
    }
}
