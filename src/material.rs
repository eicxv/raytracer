use self::{
    dielectric::Dielectric, lambertian::Lambertian, metal::Metal, scatterable::Scatterable,
};
use crate::{ray::Ray, shape::hittable::HitRecord, vec3::Vec3};

pub mod dielectric;
pub mod lambertian;
pub mod metal;
pub mod scatterable;

#[derive(Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, record: HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian(mat) => mat.scatter(ray, record),
            Material::Metal(mat) => mat.scatter(ray, record),
            Material::Dielectric(mat) => mat.scatter(ray, record),
        }
    }
}
