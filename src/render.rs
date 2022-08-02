use crate::nvol::Geometry;
use crate::threed::*;
use crate::twod::*;
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
                x: 0.5,
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
            x: (p.theta * 2.0 / self.hfov() + 0.5) * window::screen_width(),
            y: (p.phi * 2.0 / self.vfov() + 0.5) * window::screen_height(),
        }
    }
    fn draw_tri(&self, tri: &Triangle3D) {
        let pov = (
            (tri.a - self.position).to_spherical(),
            (tri.b - self.position).to_spherical(),
            (tri.c - self.position).to_spherical(),
        );
        let t = Triangle2D {
            a: self.map2d(pov.0),
            b: self.map2d(pov.1),
            c: self.map2d(pov.2),
        };
        println!("{}", t);
        t.draw();
    }
    pub fn draw_geometry(&mut self, geometry: &mut Geometry) {
        geometry.make_tris();
        for tri in &geometry.tris_cache {
            // println!("{}", tri);
            self.draw_tri(tri);
        }
    }
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
    pub fn render_frame(&mut self) {
        for geo in &mut self.geometries {
            self.camera.draw_geometry(geo);
        }
    }
}
