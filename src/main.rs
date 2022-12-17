use color_eyre::eyre::Result;
use macroquad::prelude::*;
// use std::rc::{Rc, Weak};
// use world::World;
use particle::*;

mod particle;
// mod world;

const GRID_WIDTH: usize = 6;
const GRID_HEIGHT: usize = 5;
const WORLD_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;
const PIXELS_PER_PARTICLE: f32 = 50.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sand".to_owned(),
        window_height: GRID_HEIGHT as i32 * PIXELS_PER_PARTICLE as i32,
        window_width: GRID_WIDTH as i32 * PIXELS_PER_PARTICLE as i32,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    // color_eyre::install()?;

    // Initialize Grid
    let mut grid: Vec<Option<Particle>> = vec![];

    for _ in 0..WORLD_SIZE {
        grid.push(None);
    }

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            if x == 0 || x == GRID_WIDTH - 1 || y == 0 || y == GRID_HEIGHT - 1 {
                println!("x: {:?}, y: {:?}", x, y);
                add_new_particle(&mut grid, ParticleType::Concrete, x, y);
            }
        }
    }

    // let world = World::new(width, height);

    loop {
        clear_background(BLACK);
        // world.draw_all(pixels_per_particle);
        draw_all_particles(&grid);
        next_frame().await
    }
    // Ok(())
}

fn add_new_particle(
    grid: &mut Vec<Option<Particle>>,
    new_particle_type: ParticleType,
    x: usize,
    y: usize,
) {
    let old_particle = get_particle(grid, x, y);
    match old_particle {
        Some(_) => {}
        None => {
            let new_particle = Particle::new(x, y, new_particle_type);
            grid[xy_to_index(x, y)] = Some(new_particle);
        }
    }
}

fn draw_all_particles(grid: &Vec<Option<Particle>>) {
    for space in grid.iter() {
        if let Some(particle) = space {
            particle.draw();
        }
        // match space {
        //     Some(particle) => particle.draw(pixels_per_particle),
        //     None => {}
        // }
    }
}

fn get_particle(grid: &Vec<Option<Particle>>, x: usize, y: usize) -> &Option<Particle> {
    &grid[xy_to_index(x, y)]
}

fn xy_to_index(x: usize, y: usize) -> usize {
    y * GRID_WIDTH + x
}

fn index_to_xy(i: usize) -> (usize, usize) {
    (i % GRID_WIDTH, i / GRID_WIDTH)
}
