use rand::Rng;
use std::{
    f64::consts::TAU,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign},
};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn abs(self) -> Vec3 {
        Vec3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn dot(a: Vec3, b: Vec3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn length_sq(self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(self) -> f64 {
        self.length_sq().sqrt()
    }

    pub fn unitize(self) -> Vec3 {
        let len = self.length();
        self / len
    }

    pub fn random(range: Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range),
        )
    }

    pub fn random_unit_vector() -> Vec3 {
        let mut rng = rand::thread_rng();
        let phi = rng.gen_range(0.0..TAU);
        let z: f64 = rng.gen_range(-1.0..1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3 {
            x: r * phi.cos(),
            y: r * phi.sin(),
            z,
        }
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        return *self - 2.0 * Vec3::dot(*self, normal) * normal;
    }

    pub fn refract(&self, normal: Vec3, ior_ratio: f64) -> Option<Vec3> {
        let v = self.unitize();
        let dt = Vec3::dot(v, normal);
        let disc = 1.0 - ior_ratio * ior_ratio * (1.0 - dt * dt);
        match disc > 0.0 {
            true => Some(ior_ratio * (v - normal * dt) - normal * disc.sqrt()),
            false => None,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[cfg(test)]
macro_rules! vec3_assert_approx_eq {
    ($a: expr, $b: expr, $diff: expr) => {{
        assert_approx_eq!($a.x, $b.x, $diff);
        assert_approx_eq!($a.y, $b.y, $diff);
        assert_approx_eq!($a.z, $b.z, $diff);
    }};
}

#[test]
fn test_add() {
    let mut a = Vec3::new(0.0, 0.2, 1.0);
    let b = Vec3::new(0.1, 0.1, 1.1);
    let res = Vec3::new(0.1, 0.3, 2.1);
    vec3_assert_approx_eq!(a + b, res, 1e-3f64);

    a += b;
    vec3_assert_approx_eq!(a, res, 1e-3f64);
}

#[test]
fn test_add_float() {
    let mut a = Vec3::new(0.0, 0.2, 1.0);
    let b = 0.5;
    let res = Vec3::new(0.5, 0.7, 1.5);
    vec3_assert_approx_eq!(a + b, res, 1e-3f64);
    vec3_assert_approx_eq!(b + a, res, 1e-3f64);

    a += b;
    vec3_assert_approx_eq!(a, res, 1e-3f64);
}

#[test]
fn test_sub() {
    let mut a = Vec3::new(0.0, 0.2, 1.0);
    let b = Vec3::new(0.1, 0.1, 1.1);
    let res = Vec3::new(-0.1, 0.1, -0.1);
    vec3_assert_approx_eq!(a - b, res, 1e-3f64);

    a -= b;
    vec3_assert_approx_eq!(a, res, 1e-3f64);
}

#[test]
fn test_sub_float() {
    let mut a = Vec3::new(0.0, 0.2, 1.0);
    let b = 0.5;
    let res = Vec3::new(-0.5, -0.3, 0.5);
    vec3_assert_approx_eq!(a - b, res, 1e-3f64);
    vec3_assert_approx_eq!(b - a, -res, 1e-3f64);

    a -= b;
    vec3_assert_approx_eq!(a, res, 1e-3f64);
}

#[test]
fn test_mul() {
    let mut a = Vec3::new(0.0, -0.2, 1.0);
    let b = Vec3::new(1.0, 3.0, 1.1);
    let res = Vec3::new(0.0, -0.6, 1.1);
    vec3_assert_approx_eq!(a * b, res, 1e-3f64);

    a *= b;
    vec3_assert_approx_eq!(a, res, 1e-3f64);
}

#[test]
fn test_mul_float() {
    let mut a = Vec3::new(0.0, -0.2, 1.0);
    let b = 0.5;
    let res = Vec3::new(0.0, -0.1, 0.5);
    vec3_assert_approx_eq!(a * b, res, 1e-3f64);

    a *= b;
    vec3_assert_approx_eq!(a, res, 1e-3f64);
}

#[test]
fn test_div() {
    let mut a = Vec3::new(0.0, -1.0, 2.5);
    let b = Vec3::new(1.0, 3.0, 0.5);
    let res = Vec3::new(0.0, -1.0 / 3.0, 5.0);
    vec3_assert_approx_eq!(a / b, res, 1e-3f64);

    a /= b;
    vec3_assert_approx_eq!(a, res, 1e-3f64);
}

#[test]
fn test_div_float() {
    let mut a = Vec3::new(2.0, -0.2, 1.0);
    let b = 0.5;
    let res1 = Vec3::new(4.0, -0.4, 2.0);
    let res2 = Vec3::new(0.25, -2.5, 0.5);
    vec3_assert_approx_eq!(a / b, res1, 1e-3f64);
    vec3_assert_approx_eq!(b / a, res2, 1e-3f64);

    a /= b;
    vec3_assert_approx_eq!(a, res1, 1e-3f64);
}

#[test]
fn test_dot() {
    let a = Vec3::new(0.0, -1.0, 3.0);
    let b = Vec3::new(1.0, 3.0, 0.5);
    assert_approx_eq!(Vec3::dot(a, b), -1.5, 1e-3f64);
    assert_approx_eq!(Vec3::dot(b, a), -1.5, 1e-3f64);
}

#[test]
fn test_cross() {
    let a = Vec3::new(3.0, -3.0, 1.0);
    let b = Vec3::new(4.0, 9.0, 2.0);
    let res = Vec3::new(-15.0, -2.0, 39.0);
    vec3_assert_approx_eq!(Vec3::cross(a, b), res, 1e-3f64);
}
