use crate::matrix::*;
use std::{
    fmt,
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Sub},
};

impl Add for Vector4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}
impl Sub for Vector4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}
impl AddAssign for Vector4 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w - other.w,
        };
    }
}
impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w,
        };
    }
}
impl Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w,
        }
    }
}
impl Display for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ {:04.2}, {:04.2}, {:04.2}, {:04.2} ]",
            self.x, self.y, self.z, self.w
        )
    }
}
impl Display for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} \n {} \n {} \n {}]\n",
            self.a, self.b, self.c, self.d,
        )
    }
}
impl Sub for Point4 {
    type Output = Vector4;
    fn sub(self, other: Self) -> Vector4 {
        self.coords - other.coords
    }
}
impl Add<Vector4> for Point4 {
    type Output = Point4;
    fn add(self, rhs: Vector4) -> Self::Output {
        Point4 {
            coords: self.coords + rhs,
        }
    }
}
