use self::split_method::Split;
use super::aabb::AxisAlignedBoundingBox;
use crate::{
    ray::Ray,
    shape::hittable::{HitRecord, Hittable, Shape},
    vec3::Vec3,
};
use rand::Rng;
use std::cmp::Ordering;
use std::ops::Range;

#[derive(Debug)]
pub struct Bvh {
    bounds: AxisAlignedBoundingBox,
    contents: BvhContents,
}

#[derive(Debug)]
pub enum BvhContents {
    Node { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Shape),
}

#[derive(Debug)]
pub struct BvhBuild<'a> {
    bounds: AxisAlignedBoundingBox,
    contents: BvhBuildContents<'a>,
}

#[derive(Debug)]
pub enum BvhBuildContents<'a> {
    Node {
        left: Box<BvhBuild<'a>>,
        right: Box<BvhBuild<'a>>,
    },
    Leaf(&'a [Shape]),
    // Leaf(&'a dyn Hittable),
}

struct HittableInfo {
    bounds: AxisAlignedBoundingBox,
    center: Vec3,
    index: usize,
}

mod split_method {
    use crate::{
        bvh::aabb::AxisAlignedBoundingBox,
        shape::hittable::{Hittable, Shape},
    };

    use super::{select_axis, HittableInfo};
    use partition::partition;
    use std::cmp::Ordering;
    pub trait Split {
        fn split<'a>(
            &self,
            primitives: &'a mut [Shape],
        ) -> (&'a mut [Shape], Option<&'a mut [Shape]>);
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Equal;
    impl Split for Equal {
        fn split<'a>(
            &self,
            primitives: &'a mut [Shape],
        ) -> (&'a mut [Shape], Option<&'a mut [Shape]>) {
            let (axis, _) = select_axis(primitives);
            let middle = primitives.len() / 2;
            primitives.select_nth_unstable_by(middle, |a, b| {
                a.bounding_box().center()[axis]
                    .partial_cmp(&b.bounding_box().center()[axis])
                    .unwrap()
            });
            let (left, right) = primitives.split_at_mut(middle);
            (left, Some(right))
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Middle;
    impl Split for Middle {
        fn split<'a>(
            &self,
            primitives: &'a mut [Shape],
        ) -> (&'a mut [Shape], Option<&'a mut [Shape]>) {
            let (axis, centroid_bounds) = select_axis(primitives);
            let middle = centroid_bounds.center()[axis];

            let (left, right) = partition(primitives, |prim| {
                prim.bounding_box().center()[axis] < middle
            });

            (left, Some(right))
        }
    }

    struct BucketInfo {
        count: u32,
        bounds: AxisAlignedBoundingBox,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct SurfaceArea;

    impl SurfaceArea {
        fn cost(buckets0: &[BucketInfo], buckets1: &[BucketInfo], parent_area: f64) -> f64 {
            let fld = |buckets: &[BucketInfo]| {
                buckets.iter().fold(
                    (AxisAlignedBoundingBox::null_box(), 0),
                    |(bounds, count), b| (bounds.union_box(b.bounds), count + b.count),
                )
            };
            let (b0, c0) = fld(buckets0);
            let (b1, c1) = fld(buckets1);
            0.125 + (c0 as f64 * b0.surface_area() + c1 as f64 * b1.surface_area()) / parent_area
        }
    }

    impl Split for SurfaceArea {
        fn split<'a>(
            &self,
            primitives: &'a mut [Shape],
        ) -> (&'a mut [Shape], Option<&'a mut [Shape]>) {
            if primitives.len() <= 4 {
                return Equal.split(primitives);
            }

            let (axis, centroid_bounds) = select_axis(primitives);
            let bounds = primitives.bounding_box();
            let area = bounds.surface_area();

            let nBuckets = 12;
            let mut buckets: Vec<BucketInfo> = (0..nBuckets)
                .map(|_| BucketInfo {
                    count: 0,
                    bounds: AxisAlignedBoundingBox::null_box(),
                })
                .collect();

            primitives.iter().for_each(|prim| {
                let bounds = prim.bounding_box();
                let center = bounds.center();
                let mut b = (nBuckets as f64 * centroid_bounds.offset(center)[axis]) as usize;
                if b == nBuckets {
                    b -= 1;
                }
                buckets[b].count += 1;
                buckets[b].bounds = buckets[b].bounds.union_box(bounds)
            });

            let costs: Vec<f64> = (0..buckets.len())
                .map(|i| {
                    let (buckets0, buckets1) = buckets.split_at(i);
                    SurfaceArea::cost(buckets0, buckets1, area)
                })
                .collect();

            let (index, min_cost) = costs
                .into_iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .unwrap();

            let leaf_cost = primitives.len();
            if (leaf_cost as f64) < min_cost {
                return (primitives, None);
            }
            let (left, right) = primitives.split_at_mut(index);
            (left, Some(right))
        }
    }
}

fn select_axis(primitives: &[Shape]) -> (usize, AxisAlignedBoundingBox) {
    let bounds = primitives
        .iter()
        .fold(AxisAlignedBoundingBox::null_box(), |acc, prim| {
            acc.union_point(prim.bounding_box().center())
        });

    let axis = (bounds.min - bounds.max)
        .into_iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();

    (axis, bounds)
}

impl<'a> BvhBuild<'a> {
    fn recursive_build<T: Split + Copy>(primitives: &mut [Shape], method: T) -> BvhBuild {
        match primitives.len() {
            0 => panic!("No primitives"),
            1..=2 => BvhBuild {
                bounds: primitives.bounding_box(),
                contents: BvhBuildContents::Leaf(primitives),
            },
            _ => {
                let bounds = primitives.bounding_box();

                match method.split(primitives) {
                    (left, Some(right)) => BvhBuild {
                        bounds,
                        contents: BvhBuildContents::Node {
                            left: Box::new(BvhBuild::recursive_build(left, method)),
                            right: Box::new(BvhBuild::recursive_build(right, method)),
                        },
                    },
                    (left, None) => {
                        return BvhBuild {
                            bounds,
                            contents: BvhBuildContents::Leaf(left),
                        }
                    }
                }
            }
        }
    }
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
                bounds: hittables[0].bounding_box(),
                contents: BvhContents::Leaf(hittables.pop().unwrap()),
            },
            _ => {
                let bounds = hittables.bounding_box();
                let right = Box::new(Bvh::new(hittables.split_off(hittables.len() / 2)));
                let left = Box::new(Bvh::new(hittables));
                Bvh {
                    bounds,
                    contents: BvhContents::Node { left, right },
                }
            }
        }
    }

    pub fn build(hittables: Vec<Shape>) {
        let z = hittables.into_iter().enumerate().map(|(index, hittable)| {
            let bounds = hittable.bounding_box();
            let center = (bounds.min + bounds.max) / 2.0;
            HittableInfo {
                bounds,
                center,
                index,
            }
        });

        todo!();
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        if !self.bounds.hit(ray, t_range.clone()) {
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
        self.bounds
    }
}
