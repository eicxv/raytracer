use crate::{
    bvh::aabb::AxisAlignedBoundingBox, material::scatterable::Scatterable, ray::Ray, vec3::Vec3,
};
use std::ops::Range;

pub struct HitRecord<'a> {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Scatterable,
}
pub trait Hittable: std::fmt::Debug {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord>;
    fn bounding_box(&self) -> AxisAlignedBoundingBox;
}

impl<T> Hittable for &[T]
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let mut closest = t_range.end;
        let mut record: Option<HitRecord> = None;
        for item in self.iter() {
            record = match item.hit(ray, t_range.start..closest) {
                Some(rec) => {
                    closest = rec.t;
                    Some(rec)
                }
                None => record,
            }
        }
        record
    }

    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        let bounding_boxes = self.iter().map(|hittable| hittable.bounding_box());
        AxisAlignedBoundingBox::from_boxes(bounding_boxes)
    }
}

impl<T> Hittable for &mut [T]
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let mut closest = t_range.end;
        let mut record: Option<HitRecord> = None;
        for item in self.iter() {
            record = match item.hit(ray, t_range.start..closest) {
                Some(rec) => {
                    closest = rec.t;
                    Some(rec)
                }
                None => record,
            }
        }
        record
    }

    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        let bounding_boxes = self.iter().map(|hittable| hittable.bounding_box());
        AxisAlignedBoundingBox::from_boxes(bounding_boxes)
    }
}

impl<T> Hittable for Vec<T>
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let mut closest = t_range.end;
        let mut record: Option<HitRecord> = None;
        for item in self.into_iter() {
            record = match item.hit(ray, t_range.start..closest) {
                Some(rec) => {
                    closest = rec.t;
                    Some(rec)
                }
                None => record,
            }
        }
        record
    }
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        self.as_slice().bounding_box()
    }
}

impl Hittable for Box<dyn Hittable> {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        (**self).hit(ray, t_range)
    }
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        (**self).bounding_box()
    }
}
