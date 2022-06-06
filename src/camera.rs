use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = vertical_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect_ratio;
        let w = (look_from - look_at).unitize();
        let u = Vec3::cross(up, w).unitize();
        let v = Vec3::cross(w, u);
        Camera {
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from,
        }
    }

    pub fn get_ray(&self, uv: (f64, f64)) -> Ray {
        let (u, v) = uv;
        Ray::new(
            self.origin,
            (self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
                .unitize(),
        )
    }
}
