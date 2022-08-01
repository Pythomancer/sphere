use std::fmt;
use std::ops::{Add, AddAssign, Sub};
#[derive(PartialEq)]
pub struct Triangle3D {
    pub a: Point3D,
    pub b: Point3D,
    pub c: Point3D,
}

pub struct Triangle2D {
    pub a: Point2D,
    pub b: Point2D,
    pub c: Point2D,
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

impl Triangle2D {}
