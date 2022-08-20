use std::fmt;
use std::ops::{Add, AddAssign, Sub};

// use macroquad::math::Vec2;
use macroquad::prelude::{vec2, RED};
use macroquad::shapes::draw_triangle_lines;

#[derive(Clone, Copy)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

pub struct Triangle2D {
    pub a: Point2D,
    pub b: Point2D,
    pub c: Point2D,
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
impl Triangle2D {
    pub fn center(&self) -> Point2D {
        (self.a + self.b + self.c).scale(1.0 / 3.0)
    }
    pub fn highest_pt(&self) -> Point2D {
        let mut g = self.a;
        if self.b.y > g.y {
            g = self.b;
        }
        if self.c.y > g.y {
            g = self.c;
        }
        g
    }
    pub fn rotation(&self) -> f32 {
        (self.highest_pt() - self.center()).angle()
    }
    pub fn radius(&self) -> f32 {
        (self.a - self.center()).radius()
    }
    pub fn draw(&self) {
        draw_triangle_lines(
            vec2(self.a.x, self.a.y),
            vec2(self.b.x, self.b.y),
            vec2(self.c.x, self.c.y),
            3.0,
            RED,
        )
    }
}
