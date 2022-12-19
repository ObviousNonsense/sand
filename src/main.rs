use ::rand::random;
use color_eyre::eyre::Result;
use macroquad::prelude::*;
use std::{collections::HashMap, ops::RangeFrom};
// use std::rc::{Rc, Weak};
// use world::World;
use particle::*;

mod particle;
// mod world;

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;
const WORLD_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;
const PIXELS_PER_PARTICLE: f32 = 10.0;

// const MINIMUM_FRAME_TIME: f64 = 1. / 60.;
const MINIMUM_FRAME_TIME: f64 = 1. / 5.;
const LIMIT_FRAME_RATE: bool = true;
// const BRUSH_SIZE: f32 = 1.0;

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

    println!("Window height: {}", screen_height());
    println!("Window width: {}", screen_width());

    let mut id_generator: RangeFrom<usize> = 0..;
    let mut id_list: Vec<usize> = vec![];

    // Initialize Grid
    let (mut particle_grid, mut particle_dict) =
        initialize_empty_world(&mut id_generator, &mut id_list);
    // let mut particle_grid = initialize_empty_grid();
    add_new_particle(
        &mut particle_grid,
        &mut particle_dict,
        &mut id_generator,
        &mut id_list,
        ParticleType::Sand,
        GRID_WIDTH / 2,
        3,
    );

    add_new_particle(
        &mut particle_grid,
        &mut particle_dict,
        &mut id_generator,
        &mut id_list,
        ParticleType::Sand,
        GRID_WIDTH / 2,
        2,
    );

    add_new_particle(
        &mut particle_grid,
        &mut particle_dict,
        &mut id_generator,
        &mut id_list,
        ParticleType::Sand,
        GRID_WIDTH / 2,
        1,
    );

    let mut tic = get_time();
    let mut fps_counter = 0.0;
    let mut frame_time_sum = 0.0;
    let mut paused = false;
    let mut brush_size = 1.0;
    let mut debug_mode = false;

    loop {
        let frame_time = get_time() - tic;

        // ─── Drawing ─────────────────────────────────────
        clear_background(BLACK);
        draw_all_particles(&particle_dict);
        // ─────────────────────────────────────────────────

        // ─── Mouse Input ─────────────────────────────────
        if is_mouse_button_down(MouseButton::Left) && !debug_mode {
            let (px, py) = mouse_position();
            let mousex = px / PIXELS_PER_PARTICLE;
            let mousey = py / PIXELS_PER_PARTICLE;
            let brush_span = brush_size / 2.0;
            let mousex_min = (mousex - brush_span).clamp(0., GRID_WIDTH as f32) as usize;
            let mousex_max = (mousex + brush_span).clamp(0., GRID_WIDTH as f32) as usize;
            let mousey_min = (mousey - brush_span).clamp(0., GRID_HEIGHT as f32) as usize;
            let mousey_max = (mousey + brush_span).clamp(0., GRID_HEIGHT as f32) as usize;

            // println!("Brush span = {}", brush_span);
            for x in mousex_min..mousex_max {
                // println!("x = {}", x);
                if x >= GRID_WIDTH {
                    continue;
                }
                for y in mousey_min..mousey_max {
                    if y >= GRID_HEIGHT {
                        continue;
                    }
                    add_new_particle(
                        &mut particle_grid,
                        &mut particle_dict,
                        &mut id_generator,
                        &mut id_list,
                        ParticleType::Sand,
                        x,
                        y,
                    );
                }
            }
        }

        if debug_mode {
            let (px, py) = mouse_position();
            let mousex = px / PIXELS_PER_PARTICLE;
            let mousey = py / PIXELS_PER_PARTICLE;
            let x = mousex as usize;
            let y = mousey as usize;

            let xpt = x as f32 * PIXELS_PER_PARTICLE;
            let ypt = y as f32 * PIXELS_PER_PARTICLE;

            draw_rectangle_lines(xpt, ypt, PIXELS_PER_PARTICLE, PIXELS_PER_PARTICLE, 2.0, RED);

            if is_mouse_button_pressed(MouseButton::Left) {
                let id = particle_grid[xy_to_index(x, y)];
                match id {
                    Some(id) => {
                        let p = particle_dict.get(&id).unwrap();
                        println!("({}, {}): {:?}", x, y, p);
                    }
                    None => {
                        println!("({}, {}): no particle", x, y);
                    }
                }
            }
        }
        // ─────────────────────────────────────────────────

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
            if paused {
                println!("PAUSING");
            } else {
                println!("UNPAUSING");
            }
        }

        if is_key_pressed(KeyCode::D) {
            debug_mode = !debug_mode;
            if debug_mode {
                println!("ENTERING DEBUG MODE");
            } else {
                println!("LEAVING DEBUG MODE");
            }
        }

        let (_, mouse_wheel_y) = mouse_wheel();
        if (mouse_wheel_y - 0.0).abs() > 0.000001 {
            brush_size += mouse_wheel_y.signum();
            println!("Brush size: {}", brush_size);
        }

        if !LIMIT_FRAME_RATE || frame_time >= MINIMUM_FRAME_TIME {
            // ─── Limiting And Printing Fps ───────────────────
            tic = get_time();
            fps_counter += 1.0;
            frame_time_sum += frame_time;

            if fps_counter >= 50.0 {
                let fps = 50.0 / frame_time_sum;
                fps_counter = 0.0;
                frame_time_sum = 0.0;
                // println!("{:.2}", fps);
            }
            // ─────────────────────────────────────────────────

            // ─── Update All Particles ────────────────────────
            if !paused {
                for id in id_list.iter() {
                    let particle = particle_dict.get_mut(id).unwrap();
                    match particle.particle_type {
                        ParticleType::Concrete => {}
                        ParticleType::Sand => {
                            let r = random();
                            let right: isize = if r { -1 } else { 1 };
                            let check_directions = [(0, 1), (right, 1), (0 - right, 1)];
                            for (dx, dy) in check_directions.iter() {
                                let (other_x, other_y) =
                                    ((particle.x() as isize + dx) as usize, particle.y() + dy);
                                let other_id = get_id_by_xy(&particle_grid, other_x, other_y);
                                match other_id {
                                    None => {
                                        move_particle(
                                            &mut particle_grid,
                                            particle,
                                            other_x,
                                            other_y,
                                        );
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            // ─────────────────────────────────────────────────
        }
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
    id_list: &mut Vec<usize>,
    new_particle_type: ParticleType,
    x: usize,
    y: usize,
) {
    let old_particle_id = get_id_by_xy(grid, x, y);
    match old_particle_id {
        Some(_) => {}
        None => {
            let new_id = id_generator.next().unwrap();
            id_list.push(new_id);
            let new_particle = Particle::new(x, y, new_particle_type, new_id);
            // println!("({}, {}): New {:?}", x, y, new_particle);
            dict.insert(new_id, new_particle);
            grid[xy_to_index(x, y)] = Some(new_id);
        }
    }
}

fn move_particle(
    grid: &mut Vec<Option<usize>>,
    particle: &mut Particle,
    new_x: usize,
    new_y: usize,
) {
    grid[xy_to_index(new_x, new_y)] = Some(particle.id);
    grid[xy_to_index(particle.x(), particle.y())] = None;
    particle.move_to(new_x, new_y);
}

fn get_id_by_xy(grid: &Vec<Option<usize>>, x: usize, y: usize) -> Option<usize> {
    grid[xy_to_index(x, y)]
}

fn initialize_empty_world(
    id_generator: &mut RangeFrom<usize>,
    id_list: &mut Vec<usize>,
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
                    id_list,
                    ParticleType::Concrete,
                    x,
                    y,
                );
            }
        }
    }
    (grid, dict)
}

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
