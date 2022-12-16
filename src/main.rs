use color_eyre::eyre::Result;
use macroquad::prelude::*;
// use std::rc::{Rc, Weak};
// use world::World;

mod particle;
mod world;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sand".to_owned(),
        window_height: 400,
        window_width: 500,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    // color_eyre::install()?;
    // use particle::*;
    use world::*;

    let pixels_per_particle = 10.0;
    let width = (screen_width() / pixels_per_particle) as usize;
    let height = (screen_height() / pixels_per_particle) as usize;

    let world = World::new(width, height);

    loop {
        clear_background(BLACK);
        world.draw_all_particles(pixels_per_particle);
        next_frame().await
    }
    // Ok(())
}
