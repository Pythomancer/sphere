pub mod matrix;
pub mod nvol;
pub mod operations;
pub mod render;
pub mod threed;
pub mod twod;
use crate::matrix::Matrix4;
use crate::nvol::Geometry;
use crate::render::{Camera, World};
use crate::threed::Point3D;
use macroquad::prelude::*;
use matrix::Vector4;
use std::time::SystemTime;

#[macroquad::main("Particles")]
async fn main() {
    println!("starting");
    let mut time = SystemTime::now();
    let rot = Matrix4::rotation(34.0, 45.0, 55.0);
    println!("{}", rot);
    let a = Vector4 {
        x: 0.4,
        y: 0.3,
        z: 0.5,
        w: 0.0,
    };
    let b = Vector4 {
        x: 0.4,
        y: 0.3,
        z: 0.5,
        w: 0.0,
    };
    println!("{}", a + b);
    loop {
        clear_background(WHITE);
        let mut world = World::new();
        // world.add(Geometry::cube(
        //     0.3,
        //     Point3D {
        //         x: 0.0,
        //         y: 0.0,
        //         z: 0.0,
        //     },
        // ));
        world.render_frame();
        let t: u128 = SystemTime::now()
            .duration_since(time)
            .expect("time reversed")
            .as_millis();
        draw_text(
            &(1000.0 / {
                if t > 0 {
                    t as f32
                } else {
                    0.0001
                }
            })
            .round()
            .to_string(),
            20.0,
            20.0,
            20.0,
            DARKGRAY,
        );
        time = SystemTime::now();
        next_frame().await;
    }
}
