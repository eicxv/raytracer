use crate::{ray::Ray, shape::hittable::HitRecord, vec3::Vec3};

pub trait Scatterable: std::fmt::Debug {
    fn scatter(&self, ray: &Ray, record: HitRecord) -> Option<(Ray, Vec3)>;
}
