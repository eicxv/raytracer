use crate::{ray::Ray, vec3::Vec3};

use super::hittable::HitRecord;

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, record: HitRecord) -> Option<(Ray, Vec3)>;
}
