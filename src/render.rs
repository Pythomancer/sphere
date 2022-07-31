use crate::geometry::*;
use crate::nvol::*;
use macroquad::prelude;

enum RenderType {
    FlatNoDepth,
    Simple,
    Traced,
    Marched,
    None,
}
pub struct Camera {
    position: Point,
    point_at: Point,
}

pub struct World {
    geometries: Vec<Geometry>,
    camera: Camera,
    render_mode: RenderType,
}

impl World {
    pub fn new () -> World{
        World {
            geometries: Vec<Geometry>::new(),
            Camera { 
                Point {x: 0.0, y: 0.0, z: 0.0},
                Point {x: 0.0, y: 0.0, z: 1.0},
            },
            None,
        }
    }
    pub fn render (&mut self){
        match self.render_mode {
            FlatNoDepth => {
                for geometry in self.geometries {

                }
            }
            _type => {
                println!("{} is not yet implemented.", _type);
            }
        }
    }
}
