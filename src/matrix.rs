use crate::operations;

#[derive(Clone, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub struct Point4 {
    pub coords: Vector4,
}
pub struct Matrix4 {
    pub a: Vector4,
    pub b: Vector4,
    pub c: Vector4,
    pub d: Vector4,
}
pub enum Origin {
    Point4,
    CoSys,
}
pub struct CoSys {
    pub origin: Origin,
    pub i: Vector4,
    pub j: Vector4,
    pub k: Vector4,
}
impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 { x, y, z, w }
    }
    pub fn from_slice(slice: [f32; 4]) -> Vector4 {
        Vector4 {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }
    pub fn to_slice(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
    pub fn dot(&self, other: &Vector4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Vector4) -> Vector4 {
        // s1 = a2b3 - a3b2
        // s2 = a3b1 - a1b3
        // s3 = a1b2 - a2b1
        Vector4::from_slice([
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
            1.0,
        ])
    }
    pub fn transform(&self, other: &Matrix4) -> Self {
        let tp = other.transpose();
        Vector4 {
            x: tp.a.dot(self),
            y: tp.b.dot(self),
            z: tp.c.dot(self),
            w: tp.d.dot(self),
        }
    }
}
impl Matrix4 {
    pub fn from_slice(slice: &[Vector4; 4]) -> Matrix4 {
        Matrix4 {
            a: slice[0],
            b: slice[1],
            c: slice[2],
            d: slice[3],
        }
    }
    pub fn from_slices(slice: [[f32; 4]; 4]) -> Matrix4 {
        Matrix4 {
            a: Vector4::from_slice(slice[0]),
            b: Vector4::from_slice(slice[1]),
            c: Vector4::from_slice(slice[2]),
            d: Vector4::from_slice(slice[3]),
        }
    }

    fn transpose(&self) -> Matrix4 {
        Matrix4 {
            a: Vector4 {
                x: self.a.x,
                y: self.b.x,
                z: self.c.x,
                w: self.d.x,
            },
            b: Vector4 {
                x: self.a.y,
                y: self.b.y,
                z: self.c.y,
                w: self.d.y,
            },
            c: Vector4 {
                x: self.a.z,
                y: self.b.z,
                z: self.c.z,
                w: self.d.z,
            },
            d: Vector4 {
                x: self.a.w,
                y: self.b.w,
                z: self.c.w,
                w: self.d.w,
            },
        }
    }
    fn mult(&self, other: &Matrix4) -> Matrix4 {
        // self * other
        let ot = other.transpose();
        Matrix4 {
            a: Vector4 {
                x: (self.a.dot(&ot.a)),
                y: (self.a.dot(&ot.b)),
                z: (self.a.dot(&ot.c)),
                w: (self.a.dot(&ot.d)),
            },
            b: Vector4 {
                x: (self.b.dot(&ot.a)),
                y: (self.b.dot(&ot.b)),
                z: (self.b.dot(&ot.c)),
                w: (self.b.dot(&ot.d)),
            },
            c: Vector4 {
                x: (self.c.dot(&ot.a)),
                y: (self.c.dot(&ot.b)),
                z: (self.c.dot(&ot.c)),
                w: (self.c.dot(&ot.d)),
            },
            d: Vector4 {
                x: (self.d.dot(&ot.a)),
                y: (self.d.dot(&ot.b)),
                z: (self.d.dot(&ot.c)),
                w: (self.d.dot(&ot.d)),
            },
        }
    }
    pub fn rotation(alpha: f32, beta: f32, gamma: f32) -> Matrix4 {
        Matrix4::from_slices([
            [
                alpha.cos() * beta.cos(),
                alpha.sin() * beta.sin() * gamma.cos() - alpha.cos() * gamma.sin(),
                alpha.cos() * beta.sin() * gamma.sin() + alpha.sin() * gamma.sin(),
                0.0,
            ],
            [
                alpha.cos() * beta.cos(),
                alpha.sin() * beta.sin() * gamma.cos() + alpha.cos() * gamma.sin(),
                alpha.cos() * beta.sin() * gamma.sin() - alpha.sin() * gamma.sin(),
                0.0,
            ],
            [
                -1. * beta.sin(),
                alpha.sin() * beta.cos(),
                alpha.cos() * beta.sin(),
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    fn scalar(factor: f32) -> Matrix4 {
        Matrix4::from_slices([
            [factor, 0.0, 0.0, 0.0],
            [0.0, factor, 0.0, 0.0],
            [0.0, 0.0, factor, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}
