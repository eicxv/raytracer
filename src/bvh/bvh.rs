use self::split_method::Split;
use super::aabb::AxisAlignedBoundingBox;
use crate::{
    ray::Ray,
    shape::hittable::{HitRecord, Hittable, Shape},
};
use std::ops::Range;

#[derive(Debug)]
pub struct Bvh<'a> {
    bounds: AxisAlignedBoundingBox,
    contents: BvhContents<'a>,
}

#[derive(Debug)]
pub enum BvhContents<'a> {
    Node {
        left: Box<Bvh<'a>>,
        right: Box<Bvh<'a>>,
    },
    Leaf(&'a [Shape]),
}

pub mod split_method {
    use crate::{
        bvh::aabb::AxisAlignedBoundingBox,
        shape::hittable::{Hittable, Shape},
    };

    use super::select_axis;
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
            if primitives.len() < 3 {
                return (primitives, None);
            }

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
            if primitives.len() < 3 {
                return (primitives, None);
            }

            let (axis, centroid_bounds) = select_axis(primitives);
            let middle = centroid_bounds.center()[axis];

            let (left, right) = partition(primitives, |prim| {
                prim.bounding_box().center()[axis] < middle
            });

            (left, Some(right))
        }
    }

    #[derive(Debug)]
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
            let (box0, count0) = fld(buckets0);
            let (box1, count1) = fld(buckets1);
            0.125
                + (count0 as f64 * box0.surface_area() + count1 as f64 * box1.surface_area())
                    / parent_area
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

            let n_buckets = 12;
            let mut buckets: Vec<BucketInfo> = (0..n_buckets)
                .map(|_| BucketInfo {
                    count: 0,
                    bounds: AxisAlignedBoundingBox::null_box(),
                })
                .collect();

            primitives.iter().for_each(|prim| {
                let bounds = prim.bounding_box();
                let center = bounds.center();
                let mut b = (n_buckets as f64 * centroid_bounds.offset(center)[axis]) as usize;
                if b == n_buckets {
                    b -= 1;
                }

                buckets[b].count += 1;
                buckets[b].bounds = buckets[b].bounds.union_box(bounds);
            });

            let costs: Vec<f64> = (1..buckets.len())
                .map(|i| {
                    let (buckets0, buckets1) = buckets.split_at(i);
                    SurfaceArea::cost(buckets0, buckets1, area)
                })
                .collect();

            let (bucket_index, min_cost) = costs
                .into_iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .unwrap();

            let leaf_cost = primitives.len();
            if (leaf_cost as f64) <= min_cost {
                return (primitives, None);
            }
            let mid: usize = buckets
                .iter()
                .take(bucket_index)
                .map(|bucket| bucket.count)
                .sum::<u32>()
                .try_into()
                .unwrap();

            primitives.select_nth_unstable_by(mid, |a, b| {
                a.bounding_box().center()[axis]
                    .partial_cmp(&b.bounding_box().center()[axis])
                    .unwrap_or(Ordering::Equal)
            });

            let (left, right) = primitives.split_at_mut(mid + 1);
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
    let axis = (bounds.max - bounds.min).max_dim();

    (axis, bounds)
}

impl<'a> Bvh<'a> {
    pub fn build<T: Split + Copy>(primitives: &mut [Shape], method: T) -> Bvh {
        assert!(primitives.len() > 0, "no primitives");

        let bounds = primitives.bounding_box();

        match method.split(primitives) {
            (left, Some(right)) => Bvh {
                bounds,
                contents: BvhContents::Node {
                    left: Box::new(Bvh::build(left, method)),
                    right: Box::new(Bvh::build(right, method)),
                },
            },
            (left, None) => {
                return Bvh {
                    bounds,
                    contents: BvhContents::Leaf(left),
                }
            }
        }
    }
}

impl<'a> Hittable for Bvh<'a> {
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
