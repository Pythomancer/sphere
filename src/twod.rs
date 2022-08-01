use std::fmt;
use std::ops::{Add, AddAssign, Sub};

pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

pub struct Triangle2D {
    pub a: Point2D,
    pub b: Point2D,
    pub c: Point2D,
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Point2D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    }
}

impl Point2D {
    pub fn new() -> Point2D {
        Point2D { x: 0.0, y: 0.0 }
    }

    pub fn smaller_x(&self, other: &Point2D) -> f32 {
        if self.x < other.x {
            return self.x;
        }
        other.x
    }
    pub fn greater_x(&self, other: &Point2D) -> f32 {
        if self.x > other.x {
            return self.x;
        }
        other.x
    }
    pub fn from_polar(r: f32, theta: f32) -> Point2D {
        Point2D {
            x: r * theta.cos(),
            y: r * theta.sin(),
        }
    }

    pub fn scale(&self, scalar: f32) -> Point2D {
        Point2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn radius(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn rotate(&self, ang: f32) -> Point2D {
        Point2D::from_polar(self.radius(), self.angle() + ang)
    }

    pub fn reflect_across(&self, other: &Point2D) -> Point2D {
        Point2D::from_polar(self.radius(), 2.0 * other.angle() - self.angle())
    }

    // pub fn sum(i: Iterator){
    //     let mut s =
    // }
}
impl Triangle2D {}
