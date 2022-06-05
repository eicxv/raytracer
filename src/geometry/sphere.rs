use super::hittable::{HitRecord, Hittable};
use super::scatterable::Scatterable;
use crate::ray::Ray;
use crate::vec3::Vec3;

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
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let disc = b * b - a * c;
        if disc <= 0.0 {
            return None;
        }

        let temp = (b * b - a * c).sqrt();
        let mut t = (-b - temp) / a;
        if in_range(t_range, t) {
            let point = ray.evaluate(t);
            let normal = (point - self.center) / self.radius;
            return Some(HitRecord {
                t,
                point,
                normal,
                material: &*self.material,
            });
        }
        t = (-b + temp) / a;
        if in_range(t_range, t) {
            let point = ray.evaluate(t);
            let normal = (point - self.center) / self.radius;
            return Some(HitRecord {
                t,
                point,
                normal,
                material: &*self.material,
            });
        }
        None
    }
}

fn in_range(range: (f64, f64), t: f64) -> bool {
    return t > range.0 && t < range.1;
}
