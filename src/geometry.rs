use crate::nvol::*;
use std::fmt;
use std::ops::{Add, AddAssign, Sub};

#[derive(PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(PartialEq)]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x={}, y={}, y={})", self.x, self.y, self.z)
    }
}

impl Point {
    pub fn new() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from_spherical(r: f32, theta: f32, phi: f32) -> Point {
        Point {
            x: r * theta.cos() * phi.cos(),
            y: r * theta.sin() * phi.cos(),
            z: r * phi.sin(),
        }
    }

    pub fn scale(&self, scalar: f32) -> Point {
        Point {
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
}

impl Triangle {
    pub fn contains(&self, other: &Nexus) -> bool {
        self.a == other.point || self.b == other.point || self.c == other.point
    }

    pub fn contains_both(&self, other_one: &Nexus, other_two: &Nexus) -> bool {
        (self.a == other_one.point || self.b == other_one.point || self.c == other_one.point)
            && (self.a == other_two.point || self.b == other_two.point || self.c == other_two.point)
    }

    pub fn contains_all(&self, other_one: &Nexus, other_two: &Nexus, other_three: &Nexus) -> bool {
        (self.a == other_one.point || self.b == other_one.point || self.c == other_one.point)
            && (self.a == other_two.point || self.b == other_two.point || self.c == other_two.point)
            && (self.a == other_three.point
                || self.b == other_three.point
                || self.c == other_three.point)
    }
    pub fn matches(&self, other: &Triangle) -> bool {
        (self.a == other.a || self.a == other.b || self.a == other.c)
            && (self.b == other.a || self.b == other.b || self.b == other.c)
            && (self.c == other.a || self.c == other.b || self.c == other.c)
    }
    pub fn is_in(&self, list: Vec<Triangle>) -> bool {
        for tri in list {
            if self.matches(&tri) {
                return true;
            }
        }
        return false;
    }
}
