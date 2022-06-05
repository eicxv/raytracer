use super::scatterable::Scatterable;
use crate::{ray::Ray, vec3::Vec3};

pub struct HitRecord<'a> {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Scatterable,
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord>;
}

impl<T> Hittable for &[T]
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let mut closest = t_range.1;
        let mut record: Option<HitRecord> = None;
        for item in self.into_iter() {
            record = match item.hit(ray, (t_range.0, closest)) {
                Some(rec) => {
                    closest = rec.t;
                    Some(rec)
                }
                None => record,
            }
        }
        record
    }
}
