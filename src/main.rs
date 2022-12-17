use color_eyre::eyre::Result;
use macroquad::prelude::*;
use std::{collections::HashMap, ops::RangeFrom};
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

// ─── Main ──────────────────────────────────────────────────────────────────────────────────── ✣ ─
#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    // color_eyre::install()?;

    let mut id_generator: RangeFrom<usize> = 0..;

    // Initialize Grid
    let (mut particle_grid, mut particle_dict) = initialize_empty_world(&mut id_generator);
    // let mut particle_grid = initialize_empty_grid();
    add_new_particle(
        &mut particle_grid,
        &mut particle_dict,
        &mut id_generator,
        ParticleType::Sand,
        GRID_WIDTH / 2,
        1,
    );

    loop {
        clear_background(BLACK);
        // draw_all_particles(&particle_grid);
        draw_all_particles(&particle_dict);

        next_frame().await
    }
    // Ok(())
}
// ───────────────────────────────────────────────────────────────────────────────────────────── ✣ ─

// ─── Grid Functions ────────────────────────────────────────────────────────────────────────── ✣ ─
fn add_new_particle(
    grid: &mut Vec<Option<usize>>,
    dict: &mut HashMap<usize, Particle>,
    id_generator: &mut RangeFrom<usize>,
    new_particle_type: ParticleType,
    x: usize,
    y: usize,
) {
    let old_particle_id = get_id_by_xy(grid, x, y);
    match old_particle_id {
        Some(_) => {}
        None => {
            let new_id = id_generator.next().unwrap();
            let new_particle = Particle::new(x, y, new_particle_type, new_id);
            dict.insert(new_id, new_particle);
            grid[xy_to_index(x, y)] = Some(new_id);
        }
    }
}

// fn get_particle_by_xy<'a>(
//     grid: Vec<Option<usize>>,
//     dict: HashMap<usize, Particle>,
//     x: usize,
//     y: usize,
// ) -> Option<&'a Particle> {
//     let id = grid[xy_to_index(x, y)];
//     match id {
//         Some(id) => dict.get(&id),
//         None => None,
//     }
// }

fn get_id_by_xy(grid: &Vec<Option<usize>>, x: usize, y: usize) -> Option<usize> {
    grid[xy_to_index(x, y)]
}

fn initialize_empty_world(
    id_generator: &mut RangeFrom<usize>,
) -> (Vec<Option<usize>>, HashMap<usize, Particle>) {
    //
    let mut grid: Vec<Option<usize>> = vec![];
    let mut dict: HashMap<usize, Particle> = HashMap::new();

    for _ in 0..WORLD_SIZE {
        grid.push(None);
    }

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            if x == 0 || x == GRID_WIDTH - 1 || y == 0 || y == GRID_HEIGHT - 1 {
                // println!("x: {:?}, y: {:?}", x, y);
                add_new_particle(
                    &mut grid,
                    &mut dict,
                    id_generator,
                    ParticleType::Concrete,
                    x,
                    y,
                );
            }
        }
    }
    (grid, dict)
}

// fn get_particle(grid: &Vec<Option<Particle>>, x: usize, y: usize) -> &Option<Particle> {
//     &grid[xy_to_index(x, y)]
// }

fn xy_to_index(x: usize, y: usize) -> usize {
    y * GRID_WIDTH + x
}

fn index_to_xy(i: usize) -> (usize, usize) {
    (i % GRID_WIDTH, i / GRID_WIDTH)
}

fn draw_all_particles(dict: &HashMap<usize, Particle>) {
    for particle in dict.values() {
        particle.draw();
    }
    // for space in grid.iter() {
    //     if let Some(particle) = space {
    //         particle.draw();
    //     }
    //     // match space {
    //     //     Some(particle) => particle.draw(pixels_per_particle),
    //     //     None => {}
    //     // }
    // }
}
// ───────────────────────────────────────────────────────────────────────────────────────────── ✣ ─
