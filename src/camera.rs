use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn get_ray(&self, uv: (f64, f64)) -> Ray {
        let (u, v) = uv;
        Ray::new(
            self.origin,
            (self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
                .unitize(),
        )
    }
}
