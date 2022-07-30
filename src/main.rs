use std::time::SystemTime;
use macroquad::prelude::*;

#[macroquad::main("Particles")]
async fn main() {
    println!("starting");
    let mut time = SystemTime::now();
    loop {
        clear_background(WHITE);
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
