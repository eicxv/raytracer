use super::hittable::{HitRecord, Hittable};
use crate::bvh::aabb::AxisAlignedBoundingBox;
use crate::material::scatterable::Scatterable;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::ops::Range;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Scatterable>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Scatterable>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let b = Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let mut disc = b * b - c;
        if disc <= 0.0 {
            return None;
        }

        disc = disc.sqrt();
        let mut t = -b - disc;
        if !t_range.contains(&t) {
            t = -b + disc;
            if !t_range.contains(&t) {
                return None;
            }
        }

        let point = ray.evaluate(t);
        let normal = (point - self.center) / self.radius;
        Some(HitRecord {
            t,
            point,
            normal,
            material: &*self.material,
        })
    }

    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox {
            min: self.center - self.radius,
            max: self.center + self.radius,
        }
    }
}
