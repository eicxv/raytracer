use crate::{ray::Ray, vec3::Vec3};
use rand::{thread_rng, Rng};
use std::f64::consts::TAU;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Camera {
        let theta = vertical_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect_ratio;

        let w = (look_from - look_at).unitize();
        let u = Vec3::cross(up, w).unitize();
        let v = Vec3::cross(w, u);

        Camera {
            origin: look_from,
            lower_left_corner: look_from
                - half_width * focus_distance * u
                - half_height * focus_distance * v
                - focus_distance * w,
            horizontal: 2.0 * half_width * focus_distance * u,
            vertical: 2.0 * half_height * focus_distance * v,
            lens_radius: aperture / 2.0,
            u,
            v,
        }
    }

    pub fn get_ray(&self, st: (f64, f64)) -> Ray {
        let (s, t) = st;
        let rd = self.lens_radius * random_vector_in_unit_disc();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            (self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset)
                .unitize(),
        )
    }
}

fn random_vector_in_unit_disc() -> Vec3 {
    let mut rng = thread_rng();
    let r = rng.gen::<f64>().sqrt();
    let theta = rng.gen_range(0.0..TAU);
    Vec3::new(r * theta.cos(), r * theta.sin(), 0.0)
}
