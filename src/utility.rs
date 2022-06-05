use std::ops::{Add, Mul};

pub fn lerp<T: Add<T, Output = T> + Mul<f64, Output = T>>(a: T, b: T, t: f64) -> T {
    a * (1.0 - t) + b * t
}
