use std::{
    f64::{INFINITY, NEG_INFINITY},
    mem::swap,
    ops::Range,
};

use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct AxisAlignedBoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl AxisAlignedBoundingBox {
    pub fn hit(&self, ray: &Ray, t_range: Range<f64>) -> bool {
        let (mut t_min, mut t_max) = (t_range.start, t_range.end);
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_d;

            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }

            t_min = f64::max(t0, t_min);
            t_max = f64::min(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn from_boxes<T>(bounding_boxes: T) -> AxisAlignedBoundingBox
    where
        T: IntoIterator<Item = AxisAlignedBoundingBox>,
    {
        let mut min = [INFINITY; 3];
        let mut max = [NEG_INFINITY; 3];

        for bbox in bounding_boxes {
            for a in 0..3 {
                min[a] = min[a].min(bbox.min[a]);
                max[a] = max[a].max(bbox.max[a]);
            }
        }
        AxisAlignedBoundingBox {
            min: Vec3::new(min[0], min[1], min[2]),
            max: Vec3::new(max[0], max[1], max[2]),
        }
    }
}

#[test]
fn aabb_hit() {
    let aabb1 = AxisAlignedBoundingBox {
        min: Vec3::new(INFINITY, INFINITY, INFINITY),
        max: Vec3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
    };
    let aabb2 = AxisAlignedBoundingBox {
        min: Vec3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
        max: Vec3::new(INFINITY, INFINITY, INFINITY),
    };
    let dir = Vec3::new(1.0, 1.0, 1.0);
    let origin = Vec3::origin();

    let hit1 = aabb1.hit(&Ray::new(origin, dir), NEG_INFINITY..INFINITY);
    let hit2 = aabb2.hit(&Ray::new(origin, dir), 0.0..1.0);
    println!("{}, {}", hit1, hit2);
    assert!(hit1 == false);
    assert!(hit2 == true);
}

#[test]
fn aabb_hit_infinite() {
    let aabb = AxisAlignedBoundingBox {
        min: Vec3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
        max: Vec3::new(INFINITY, INFINITY, INFINITY),
    };

    let origin = Vec3::origin();
    let direction = Vec3::new(1.0, 1.0, 1.0);
    let mut ray = Ray { origin, direction };
    let mut hit = aabb.hit(&ray, 0.0..1.0);
    assert!(hit == true);
    ray.origin = Vec3::new(1.0, 0.0, 0.0);
    ray.direction = Vec3::new(-1.0, 0.0, 0.0);
    hit = aabb.hit(&ray, 0.0..1.0);
    assert!(hit == true);
}

#[test]
fn aabb_hit_neg_infinite() {
    let aabb = AxisAlignedBoundingBox {
        min: Vec3::new(INFINITY, INFINITY, INFINITY),
        max: Vec3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
    };

    let origin = Vec3::origin();
    let direction = Vec3::new(1.0, 1.0, 1.0);
    let mut ray = Ray { origin, direction };
    let mut hit = aabb.hit(&ray, 0.0..1.0);
    assert!(hit == false);
    ray.origin = Vec3::new(1.0, 0.0, 0.0);
    ray.direction = Vec3::new(-1.0, 0.0, 0.0);
    hit = aabb.hit(&ray, 0.0..1.0);
    assert!(hit == false);
}
