use crate::geometry::{Point2D, Point3D, SPoint, Triangle2D, Triangle3D};
use crate::nvol::Geometry;
use macroquad::prelude::*;
use macroquad::window;
enum RenderMode {
    FlatNoDepth,
    Simple,
    RayTrace,
    None,
}
pub struct Camera {
    position: Point3D,
    point_at: Point3D,
    hfov: f32,
    mode: RenderMode,
}
pub struct World {
    geometries: Vec<Geometry>,
    camera: Camera,
}
impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            point_at: Point3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            hfov: 90.0,
            mode: RenderMode::FlatNoDepth,
        }
    }
    fn hfov(&self) -> f32 {
        self.hfov
    }
    fn vfov(&self) -> f32 {
        self.hfov / window::screen_width() * window::screen_height()
    }
    fn map2d(&self, pt: SPoint) -> Point2D {
        let p = pt.rotate_sub(&self.point_at.to_spherical());
        Point2D {
            x: p.theta * 2.0 / self.hfov(),
            y: p.phi * 2.0 / self.vfov(),
        }
    }
    fn draw_tri(&self, tri: Triangle3D) {
        let s = self.point_at.to_spherical();
        let pov = (
            (tri.a - self.position).to_spherical(),
            (tri.b - self.position).to_spherical(),
            (tri.c - self.position).to_spherical(),
        );
        let map_tri = Triangle2D {
            a: self.map2d(pov.0),
            b: self.map2d(pov.1),
            c: self.map2d(pov.2),
        };
        // draw_poly(x, y, sides, radius, rotation, color)
    }
    pub fn render_frame(&self) {}
}
impl World {
    pub fn new() -> World {
        World {
            geometries: Vec::new(),
            camera: Camera::new(),
        }
    }
    pub fn add(&mut self, g: Geometry) {
        self.geometries.push(g);
    }
}
