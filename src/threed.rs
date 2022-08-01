use std::fmt;
use std::ops::{Add, AddAssign, Sub};

#[derive(PartialEq, Clone, Copy)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub struct SPoint {
    pub phi: f32,
    pub theta: f32,
    pub rad: f32,
}

#[derive(PartialEq)]
pub struct Triangle3D {
    pub a: Point3D,
    pub b: Point3D,
    pub c: Point3D,
}
impl Add for Point3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl AddAssign for Point3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x={}, y={}, y={})", self.x, self.y, self.z)
    }
}

impl Point3D {
    pub fn new() -> Point3D {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from_coords(x: f32, y: f32, z: f32) -> Point3D {
        Point3D { x, y, z }
    }

    pub fn from_spherical(r: f32, theta: f32, phi: f32) -> Point3D {
        Point3D {
            x: r * theta.cos() * phi.cos(),
            y: r * theta.sin() * phi.cos(),
            z: r * phi.sin(),
        }
    }

    pub fn scale(&self, scalar: f32) -> Point3D {
        Point3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn radius(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn theta(&self) -> f32 {
        self.y.atan2(self.x)
    }
    pub fn to_spherical(&self) -> SPoint {
        let r = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        let t = self.y.atan2(self.x);
        let p = self.z.atan2(self.x * self.x + self.y * self.y);
        SPoint {
            phi: p,
            theta: t,
            rad: r,
        }
    }
}

impl Triangle3D {
    pub fn matches(&self, other: &Triangle3D) -> bool {
        (self.a == other.a || self.a == other.b || self.a == other.c)
            && (self.b == other.a || self.b == other.b || self.b == other.c)
            && (self.c == other.a || self.c == other.b || self.c == other.c)
    }
    pub fn is_in(&self, list: Vec<Triangle3D>) -> bool {
        for tri in list {
            if self.matches(&tri) {
                return true;
            }
        }
        return false;
    }
}

impl SPoint {
    pub fn from_cartesian(c_pt: Point3D) -> SPoint {
        let r = (c_pt.x * c_pt.x + c_pt.y * c_pt.y + c_pt.z * c_pt.z).sqrt();
        let t = c_pt.y.atan2(c_pt.x);
        let p = c_pt.z.atan2(c_pt.x * c_pt.x + c_pt.y * c_pt.y);
        SPoint {
            phi: p,
            theta: t,
            rad: r,
        }
    }
    pub fn to_cartesian(&self) -> Point3D {
        Point3D {
            x: self.rad * self.theta.cos() * self.phi.cos(),
            y: self.rad * self.theta.sin() * self.phi.cos(),
            z: self.rad * self.phi.sin(),
        }
    }
    pub fn rotate_sub(&self, other: &SPoint) -> SPoint {
        SPoint {
            phi: self.phi - other.phi,
            theta: self.theta - other.theta,
            rad: self.rad - other.rad,
        }
    }
}
