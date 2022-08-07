use super::hittable::{HitRecord, Hittable};
use crate::bvh::aabb::AxisAlignedBoundingBox;
use crate::material::scatterable::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use itertools::Itertools;
use std::ops::Range;

#[derive(Debug)]
pub struct TriangleMesh {
    pub vertices: Vec<Vec3>,
    pub vertex_indices: Vec<[usize; 3]>,
    pub material: Material,
}

impl TriangleMesh {
    pub fn triangles(&self) -> impl Iterator<Item = Triangle> {
        self.vertex_indices.iter().map(|indices| Triangle {
            mesh: self,
            indices,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle<'a, 'b> {
    pub mesh: &'a TriangleMesh,
    pub indices: &'b [usize; 3],
}

impl<'a, 'b> Triangle<'a, 'b> {
    pub fn get_vertices(&self) -> [Vec3; 3] {
        self.indices.map(|i| self.mesh.vertices[i])
    }

    pub fn area(&self) -> f64 {
        let vertices = self.get_vertices();
        Vec3::cross(vertices[1] - vertices[0], vertices[2] - vertices[0]).length() / 2.0
    }
}

impl<'a, 'b> Hittable for Triangle<'a, 'b> {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let v = self.get_vertices();
        let v0v1 = v[1] - v[0];
        let v0v2 = v[2] - v[0];
        let n = Vec3::cross(v0v1, v0v2);
        let n_dot_dir = n.dot(ray.direction);
        if n_dot_dir.abs() < 0.00001 {
            return None;
        }
        let d = -n.dot(v[0]);
        let t = -(n.dot(ray.origin) + d) / n_dot_dir;

        if !t_range.contains(&t) {
            return None;
        }

        let p = ray.evaluate(t);

        if v.into_iter()
            .cycle()
            .take(4)
            .tuple_windows()
            .any(|(v0, v1)| {
                let edge = v1 - v0;
                let vp = p - v0;
                let c = edge.cross(vp);
                n.dot(c) < 0.0
            })
        {
            return None;
        }

        Some(HitRecord {
            material: &self.mesh.material,
            normal: n.unitize(),
            point: p,
            t,
        })
    }

    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        let vertices = self.get_vertices();
        AxisAlignedBoundingBox::from_points(vertices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::lambertian::Lambertian;
    use crate::material::scatterable::Material;

    #[test]
    fn test_hit() {
        let mesh = TriangleMesh {
            material: Material::Lambertian(Lambertian {
                albedo: Vec3::new(0.7, 0.7, 0.7),
            }),
            vertex_indices: vec![[0, 1, 2]],
            vertices: vec![
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 1.0),
                Vec3::new(1.0, 1.0, 0.0),
            ],
        };
        let tri = mesh.triangles().collect::<Vec<_>>()[0];
        let ray = Ray::new(Vec3::new(0.0, 1.2, 0.2), Vec3::new(1.0, 0.0, 0.0).unitize());
        let h = tri.hit(&ray, 0.0..f64::INFINITY);
        assert!(h.is_none());
    }
}
