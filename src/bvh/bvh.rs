use super::aabb::AxisAlignedBoundingBox;
use crate::{
    ray::Ray,
    shape::hittable::{HitRecord, Hittable, Shape},
};
use rand::Rng;
use std::ops::Range;

#[derive(Debug)]
pub struct Bvh {
    b_box: AxisAlignedBoundingBox,
    contents: BvhContents,
}

#[derive(Debug)]
pub enum BvhContents {
    Node { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Shape),
}

impl Bvh {
    pub fn new(mut hittables: Vec<Shape>) -> Bvh {
        let axis: usize = rand::thread_rng().gen_range(0..=2);
        hittables.sort_unstable_by(|a, b| {
            a.bounding_box().min[axis]
                .partial_cmp(&b.bounding_box().min[axis])
                .unwrap()
        });

        match hittables.len() {
            0 => panic!("No hittables"),
            1 => Bvh {
                b_box: hittables[0].bounding_box(),
                contents: BvhContents::Leaf(hittables.pop().unwrap()),
            },
            _ => {
                let b_box = hittables.bounding_box();
                let right = Box::new(Bvh::new(hittables.split_off(hittables.len() / 2)));
                let left = Box::new(Bvh::new(hittables));
                Bvh {
                    b_box,
                    contents: BvhContents::Node { left, right },
                }
            }
        }
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        if !self.b_box.hit(ray, t_range.clone()) {
            return None;
        }

        match &self.contents {
            BvhContents::Node { left, right } => match left.hit(ray, t_range.clone()) {
                Some(rec_left) => match right.hit(ray, t_range.start..rec_left.t) {
                    Some(rec_right) => Some(rec_right),
                    None => Some(rec_left),
                },
                None => right.hit(ray, t_range),
            },
            BvhContents::Leaf(leaf) => leaf.hit(ray, t_range),
        }
    }
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        self.b_box
    }
}
