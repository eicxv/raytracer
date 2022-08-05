use rand::Rng;
use std::array::IntoIter;
use std::cmp::Ordering;
use std::{
    default::Default,
    f64::consts::TAU,
    iter::IntoIterator,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign,
    },
};

#[cfg(debug_assertions)]
use assert_approx_eq::assert_approx_eq;

macro_rules! debug_assert_approx_eq {
    ($a:expr, $b:expr, $eps:expr) => {{
        #[cfg(debug_assertions)]
        {
            assert_approx_eq!($a, $b, $eps);
        }
    }};
}

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

    pub fn origin() -> Self {
        Vec3::default()
    }

    pub fn has_nan(&self) -> bool {
        f64::is_nan(self.x) || f64::is_nan(self.y) || f64::is_nan(self.z)
    }

    pub fn abs(self) -> Vec3 {
        Vec3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn dot(self, v: Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
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

        debug_assert_ne!(len, 0.0);

        self / len
    }

    pub fn max_dim(self) -> usize {
        self.into_iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap()
    }

    pub fn permute(self, x: usize, y: usize, z: usize) -> Self {
        Vec3::new(self[x], self[y], self[z])
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        debug_assert_approx_eq!(normal.length(), 1.0, 1e-3f64);

        return *self - 2.0 * Vec3::dot(*self, normal) * normal;
    }

    pub fn refract(&self, normal: Vec3, ior_ratio: f64) -> Option<Vec3> {
        debug_assert_approx_eq!(self.length(), 1.0, 1e-3f64);
        debug_assert_approx_eq!(normal.length(), 1.0, 1e-3f64);

        let dt = self.dot(normal);
        let disc = 1.0 - ior_ratio * ior_ratio * (1.0 - dt * dt);
        match disc >= 0.0 {
            true => Some(ior_ratio * (*self - normal * dt) - normal * disc.sqrt()),
            false => None,
        }
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
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

impl IntoIterator for Vec3 {
    type Item = f64;
    type IntoIter = IntoIter<f64, 3>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([self.x, self.y, self.z])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("unknown field: {}", i),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("unknown field: {}", i),
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
        let reciprocal = 1.0 / rhs;
        self * reciprocal
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
        let reciprocal = 1.0 / rhs;
        *self *= reciprocal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use std::f64::consts::PI;

    macro_rules! vec3_assert_approx_eq {
        ($a: expr, $b: expr, $diff: expr) => {{
            assert_approx_eq!($a.x, $b.x, $diff);
            assert_approx_eq!($a.y, $b.y, $diff);
            assert_approx_eq!($a.z, $b.z, $diff);
        }};
    }

    #[test]
    fn has_nan() {
        assert!(!Vec3::new(0.0, 0.2, 1.0).has_nan());
        assert!(Vec3::new(f64::NAN, 0.2, 1.0).has_nan());
        assert!(Vec3::new(f64::NAN, f64::NAN, 1.0).has_nan());
        assert!(Vec3::new(f64::NAN, f64::NAN, f64::NAN).has_nan());
    }

    #[test]
    fn into_iter() {
        let v: Vec<f64> = Vec3::new(-1.0, 0.0, 1.0).into_iter().collect();
        assert!(v == [-1.0, 0.0, 1.0]);
    }

    #[test]
    fn add() {
        let mut a = Vec3::new(0.0, 0.2, 1.0);
        let b = Vec3::new(0.1, 0.1, 1.1);
        let res = Vec3::new(0.1, 0.3, 2.1);
        vec3_assert_approx_eq!(a + b, res, 1e-3f64);

        a += b;
        vec3_assert_approx_eq!(a, res, 1e-3f64);
    }

    #[test]
    fn add_scalar() {
        let mut a = Vec3::new(0.0, 0.2, 1.0);
        let b = 0.5;
        let res = Vec3::new(0.5, 0.7, 1.5);
        vec3_assert_approx_eq!(a + b, res, 1e-3f64);
        vec3_assert_approx_eq!(b + a, res, 1e-3f64);

        a += b;
        vec3_assert_approx_eq!(a, res, 1e-3f64);
    }

    #[test]
    fn sub() {
        let mut a = Vec3::new(0.0, 0.2, 1.0);
        let b = Vec3::new(0.1, 0.1, 1.1);
        let res = Vec3::new(-0.1, 0.1, -0.1);
        vec3_assert_approx_eq!(a - b, res, 1e-3f64);

        a -= b;
        vec3_assert_approx_eq!(a, res, 1e-3f64);
    }

    #[test]
    fn sub_scalar() {
        let mut a = Vec3::new(0.0, 0.2, 1.0);
        let b = 0.5;
        let res = Vec3::new(-0.5, -0.3, 0.5);
        vec3_assert_approx_eq!(a - b, res, 1e-3f64);
        vec3_assert_approx_eq!(b - a, -res, 1e-3f64);

        a -= b;
        vec3_assert_approx_eq!(a, res, 1e-3f64);
    }

    #[test]
    fn mul() {
        let mut a = Vec3::new(0.0, -0.2, 1.0);
        let b = Vec3::new(1.0, 3.0, 1.1);
        let res = Vec3::new(0.0, -0.6, 1.1);
        vec3_assert_approx_eq!(a * b, res, 1e-3f64);

        a *= b;
        vec3_assert_approx_eq!(a, res, 1e-3f64);
    }

    #[test]
    fn mul_scalar() {
        let mut a = Vec3::new(0.0, -0.2, 1.0);
        let b = 0.5;
        let res = Vec3::new(0.0, -0.1, 0.5);
        vec3_assert_approx_eq!(a * b, res, 1e-3f64);

        a *= b;
        vec3_assert_approx_eq!(a, res, 1e-3f64);
    }

    #[test]
    fn div() {
        let mut a = Vec3::new(0.0, -1.0, 2.5);
        let b = Vec3::new(1.0, 3.0, 0.5);
        let res = Vec3::new(0.0, -1.0 / 3.0, 5.0);
        vec3_assert_approx_eq!(a / b, res, 1e-3f64);

        a /= b;
        vec3_assert_approx_eq!(a, res, 1e-3f64);
    }

    #[test]
    fn div_scalar() {
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
    fn index() {
        let v = Vec3::new(2.0, -2.2, 0.0);
        assert_eq!(v.x, v[0]);
        assert_eq!(v.y, v[1]);
        assert_eq!(v.z, v[2]);
    }

    #[test]
    fn dot() {
        let a = Vec3::new(0.0, -1.0, 3.0);
        let b = Vec3::new(1.0, 3.0, 0.5);
        assert_approx_eq!(Vec3::dot(a, b), -1.5, 1e-3f64);
        assert_approx_eq!(Vec3::dot(b, a), -1.5, 1e-3f64);
    }

    #[test]
    fn cross() {
        let a = Vec3::new(3.0, -3.0, 1.0);
        let b = Vec3::new(4.0, 9.0, 2.0);
        let res = Vec3::new(-15.0, -2.0, 39.0);
        vec3_assert_approx_eq!(Vec3::cross(a, b), res, 1e-3f64);
    }

    #[test]
    fn abs() {
        vec3_assert_approx_eq!(
            Vec3::new(-3.0, 0.0, 1.0).abs(),
            Vec3::new(3.0, 0.0, 1.0),
            1e-3f64
        );
        vec3_assert_approx_eq!(
            Vec3::new(-3.0, -5.0, -0.5).abs(),
            Vec3::new(3.0, 5.0, 0.5),
            1e-3f64
        );
    }

    #[test]
    fn neg() {
        vec3_assert_approx_eq!(
            Vec3::new(-3.0, 0.0, 1.0).neg(),
            Vec3::new(3.0, 0.0, -1.0),
            1e-3f64
        );
        vec3_assert_approx_eq!(
            Vec3::new(-3.0, -5.0, -0.5).neg(),
            Vec3::new(3.0, 5.0, 0.5),
            1e-3f64
        );
    }

    #[test]
    fn length() {
        assert_approx_eq!(Vec3::new(-3.0, 0.0, 1.0).length(), f64::sqrt(10.0), 1e-3f64);
    }

    #[test]
    fn length_sq() {
        assert_approx_eq!(Vec3::new(-3.0, 0.0, 1.0).length_sq(), 10.0, 1e-3f64);
    }

    #[test]
    fn unitize() {
        assert_approx_eq!(Vec3::new(-3.0, 0.0, 1.0).unitize().length(), 1.0, 1e-3f64);
    }

    #[test]
    fn reflect() {
        let dir = Vec3::new(-3.0, 0.0, 1.0);
        vec3_assert_approx_eq!(dir.reflect(-dir.unitize()), -dir, 1e-3f64);
        let normal = Vec3::new(0.0, 0.0, 1.0).unitize();
        vec3_assert_approx_eq!(dir.reflect(normal), Vec3::new(-3.0, 0.0, -1.0), 1e-3f64);
    }

    #[test]
    fn refract() {
        let mut incidence = Vec3::new(-3.0, 0.5, 1.0).unitize();
        let mut normal = Vec3::new(1.0, 1.0, 0.0).unitize();
        let mut refraction = incidence.refract(normal, 1.0).unwrap();
        vec3_assert_approx_eq!(refraction, incidence, 1e-3f64);

        incidence = Vec3::new(-1.0, 0.0, 0.0).unitize();
        normal = Vec3::new(1.0, 0.0, 0.0);
        refraction = incidence.refract(normal, 2.0).unwrap();
        vec3_assert_approx_eq!(refraction, incidence, 1e-3f64);

        let incidence_angle = 30.0f64.to_radians();
        let expected_angle = 22.1;
        incidence = Vec3::new(incidence_angle.cos(), incidence_angle.sin(), 0.0);
        normal = Vec3::new(1.0, 0.0, 0.0);
        refraction = incidence.refract(normal, 1.0 / 1.33).unwrap();
        let refracted_angle = PI - f64::atan2(refraction.y, refraction.x);
        assert_approx_eq!(refracted_angle.to_degrees(), expected_angle, 1e-1f64);
    }
}
