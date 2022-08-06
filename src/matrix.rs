use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
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
    pub fn from_slice(slice: &[f32; 3]) -> Vec3 {
        Vec3 {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
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
