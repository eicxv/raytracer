use super::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::{ray::Ray, shape::hittable::HitRecord, vec3::Vec3};
use enum_dispatch::enum_dispatch;

#[enum_dispatch(Material)]
pub trait Scatterable: std::fmt::Debug {
    fn scatter(&self, ray: &Ray, record: HitRecord) -> Option<(Ray, Vec3)>;
}

#[enum_dispatch]
#[derive(Debug)]
pub enum Material {
    Lambertian,
    Dielectric,
    Metal,
}
