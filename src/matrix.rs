use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub struct Mat3 {
    a: Vec3,
    b: Vec3,
    c: Vec3,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn from_slice(slice: [f32; 3]) -> Vec3 {
        Vec3 {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
    pub fn to_slice(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        // s1 = a2b3 - a3b2
        // s2 = a3b1 - a1b3
        // s3 = a1b2 - a2b1
        Vec3::from_slice([
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        ])
    }
}
impl Mat3 {
    pub fn from_slice(slice: &[Vec3; 3]) -> Mat3 {
        Mat3 {
            a: slice[0],
            b: slice[1],
            c: slice[2],
        }
    }
    pub fn from_slices(slice: [[f32; 3]; 3]) -> Mat3 {
        Mat3 {
            a: Vec3::from_slice(slice[0]),
            b: Vec3::from_slice(slice[1]),
            c: Vec3::from_slice(slice[2]),
        }
    }

    fn transpose(&self) -> Mat3 {
        Mat3 {
            a: Vec3 {
                x: self.a.x,
                y: self.b.x,
                z: self.c.x,
            },
            b: Vec3 {
                x: self.a.y,
                y: self.b.y,
                z: self.c.y,
            },
            c: Vec3 {
                x: self.a.z,
                y: self.b.z,
                z: self.c.z,
            },
        }
    }
    fn mult(&self, other: &Mat3) -> Mat3 {
        // self * other
        let ot = other.transpose();
        Mat3 {
            a: Vec3 {
                x: (self.a.dot(&ot.a)),
                y: (self.a.dot(&ot.b)),
                z: (self.a.dot(&ot.c)),
            },
            b: Vec3 {
                x: (self.b.dot(&ot.a)),
                y: (self.b.dot(&ot.b)),
                z: (self.b.dot(&ot.c)),
            },
            c: Vec3 {
                x: (self.c.dot(&ot.a)),
                y: (self.c.dot(&ot.b)),
                z: (self.c.dot(&ot.c)),
            },
        }
    }
    fn rotation(alpha: f32, beta: f32, gamma: f32) -> Mat3 {
        Mat3::from_slices([
            [
                alpha.cos() * beta.cos(),
                alpha.sin() * beta.sin() * gamma.cos() - alpha.cos() * gamma.sin(),
                alpha.cos() * beta.sin() * gamma.sin() + alpha.sin() * gamma.sin(),
            ],
            [
                alpha.cos() * beta.cos(),
                alpha.sin() * beta.sin() * gamma.cos() + alpha.cos() * gamma.sin(),
                alpha.cos() * beta.sin() * gamma.sin() - alpha.sin() * gamma.sin(),
            ],
            [
                -1. * beta.sin(),
                alpha.sin() * beta.cos(),
                alpha.cos() * beta.sin(),
            ],
        ])
    }
    fn scalar(factor: f32) -> Mat3 {
        Mat3::from_slices([[factor, 0.0, 0.0], [0.0, factor, 0.0], [0.0, 0.0, factor]])
    }
}
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
