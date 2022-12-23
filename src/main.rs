use ::rand::{random, rngs::ThreadRng, seq::SliceRandom, thread_rng};
use color_eyre::eyre::Result;
use macroquad::prelude::*;
use particle::*;

mod particle;
// mod world;

const GRID_WIDTH: usize = 75;
const GRID_HEIGHT: usize = 100;
const WORLD_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;
const PIXELS_PER_PARTICLE: f32 = 4.0;

const MINIMUM_UPDATE_TIME: f64 = 1. / 90.;
// const MINIMUM_UPDATE_TIME: f64 = 1. / 1.;
const LIMIT_UPDATE_RATE: bool = false;
// const BRUSH_SIZE: f32 = 1.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sand".to_owned(),
        window_resizable: false,
        high_dpi: false,
        sample_count: 0,
        window_width: (GRID_WIDTH as f32 * PIXELS_PER_PARTICLE) as i32,
        window_height: (GRID_HEIGHT as f32 * PIXELS_PER_PARTICLE) as i32,
        ..Default::default()
    }
}

struct Settings {
    paused: bool,
    brush_size: f32,
    highlight_brush: bool,
    display_fps: bool,
    placement_type: ParticleType,
}

// ─── Main ──────────────────────────────────────────────────────────────────────────────────── ✣ ─
#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    // color_eyre::install()?;

    // request_new_screen_size(
    //     GRID_WIDTH as f32 * PIXELS_PER_PARTICLE,
    //     GRID_HEIGHT as f32 * PIXELS_PER_PARTICLE,
    // );

    let mut rng = thread_rng();

    // Initialize Grid
    let mut world = World::new();

    let mut tic = get_time();
    let mut fps_counter = 0.0;
    let mut frame_time_sum = 0.0;

    let mut settings = Settings {
        paused: false,
        brush_size: 1.0,
        highlight_brush: true,
        display_fps: false,
        placement_type: ParticleType::Sand,
    };

    loop {
        let frame_time = get_time() - tic;

        // ─── Drawing ─────────────────────────────────────────────────────────────
        // clear_background(BLACK);
        world.draw_and_reset_all_particles();
        // ─────────────────────────────────────────────────────────────────────────

        // ─── Input ───────────────────────────────────────────────────────────────
        handle_input(&mut settings, &mut world, &mut rng);
        // ─────────────────────────────────────────────────────────────────────────

        if !LIMIT_UPDATE_RATE || frame_time >= MINIMUM_UPDATE_TIME {
            // ─── Limiting And Printing Fps ───────────────────────────────
            tic = get_time();
            fps_counter += 1.0;
            frame_time_sum += frame_time;

            if fps_counter >= 50.0 {
                let fps = 50.0 / frame_time_sum;
                fps_counter = 0.0;
                frame_time_sum = 0.0;
                if settings.display_fps {
                    println!("{:.2}", fps);
                }

                println!("width = {}, height = {}", screen_width(), screen_height());
            }
            // ─────────────────────────────────────────────────────────────

            // ─── Update All Particles ────────────────────────────────────
            if !settings.paused {
                world.update_all_particles(&mut rng);
            }
            // ─────────────────────────────────────────────────────────────
        }
        next_frame().await
    }
    // Ok(())
}
// ───────────────────────────────────────────────────────────────────────────────────────────── ✣ ─

// ─── Grid Functions ────────────────────────────────────────────────────────────────────────── ✣ ─
struct World {
    grid: Vec<Particle>,
}

impl World {
    fn new() -> Self {
        let mut grid: Vec<Particle> = vec![];

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if x == 0 || x == GRID_WIDTH - 1 || y == 0 || y == GRID_HEIGHT - 1 {
                    // println!("x: {:?}, y: {:?}", x, y);
                    grid.push(Particle::new(ParticleType::Border));
                } else {
                    grid.push(Particle::new(ParticleType::Empty));
                }
            }
        }
        Self { grid }
    }

    fn add_new_particle(&mut self, new_particle_type: ParticleType, x: usize, y: usize) {
        match new_particle_type {
            ParticleType::Empty => {
                self.grid[xy_to_index(x, y)] = Particle::new(new_particle_type);
            }
            _ => {
                let old_particle_type = self.grid[xy_to_index(x, y)].particle_type;
                match old_particle_type {
                    ParticleType::Empty => {
                        self.grid[xy_to_index(x, y)] = Particle::new(new_particle_type);
                    }
                    _ => {}
                }
            }
        }
    }

    fn update_all_particles(&mut self, rng: &mut ThreadRng) {
        let mut idx_range: Vec<usize> = ((GRID_WIDTH + 1)..(GRID_WIDTH * GRID_HEIGHT - 2))
            .rev()
            .collect();
        idx_range.shuffle(rng);
        for idx in idx_range.iter() {
            let idx = *idx;
            let (x, y) = index_to_xy(idx);
            let particle = self.grid[idx];

            if !particle.moved {
                match particle.particle_type {
                    ParticleType::Sand => {
                        let r = random();
                        let right: isize = if r { -1 } else { 1 };
                        let check_directions = [(0, 1), (right, 1), (0 - right, 1)];

                        for (dx, dy) in check_directions.iter() {
                            let (other_x, other_y) = ((x as isize + dx) as usize, y + dy);
                            let other_type = self.grid[xy_to_index(other_x, other_y)].particle_type;

                            match other_type {
                                ParticleType::Empty | ParticleType::Water => {
                                    self.grid[idx].moved = true;
                                    self.grid[xy_to_index(other_x, other_y)].moved = true;
                                    (self.grid[idx], self.grid[xy_to_index(other_x, other_y)]) =
                                        (self.grid[xy_to_index(other_x, other_y)], self.grid[idx]);

                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    ParticleType::Water => {
                        let r = random();
                        let right: isize = if r { -1 } else { 1 };
                        let check_directions = if particle.bool_state[1] {
                            [(0, 1), (right, 1), (0 - right, 1), (1, 0), (-1, 0)]
                        } else {
                            [(0, 1), (right, 1), (0 - right, 1), (-1, 0), (1, 0)]
                        };

                        for ((dx, dy), k) in check_directions.iter().zip(0..5) {
                            let (other_x, other_y) = ((x as isize + dx) as usize, y + dy);
                            let other_type = self.grid[xy_to_index(other_x, other_y)].particle_type;

                            match other_type {
                                ParticleType::Empty => {
                                    if k == 4 {
                                        self.grid[idx].bool_state[1] =
                                            !self.grid[idx].bool_state[1];
                                    }
                                    self.grid[idx].moved = true;
                                    (self.grid[idx], self.grid[xy_to_index(other_x, other_y)]) =
                                        (self.grid[xy_to_index(other_x, other_y)], self.grid[idx]);

                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn draw_and_reset_all_particles(&mut self) {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                self.grid[xy_to_index(x, y)].draw(x, y);
                self.grid[xy_to_index(x, y)].moved = false;
            }
        }
    }
}

fn xy_to_index(x: usize, y: usize) -> usize {
    y * GRID_WIDTH + x
}

fn index_to_xy(i: usize) -> (usize, usize) {
    (i % GRID_WIDTH, i / GRID_WIDTH)
}
// ───────────────────────────────────────────────────────────────────────────────────────────── ✣ ─

// ─── Handle Input ──────────────────────────────────────────────────────────────────────────── ✣ ─
fn handle_input(settings: &mut Settings, world: &mut World, rng: &mut ThreadRng) {
    // Function to calculate the coordinates of the placement brush
    fn calculate_brush(brush_size: f32) -> (usize, usize, usize, usize) {
        let (px, py) = mouse_position();
        let mousex = px / PIXELS_PER_PARTICLE;
        let mousey = py / PIXELS_PER_PARTICLE;
        let brush_span = brush_size / 2.0;
        let mousex_min = (mousex - brush_span).clamp(0., GRID_WIDTH as f32) as usize;
        let mousex_max = (mousex + brush_span).clamp(0., GRID_WIDTH as f32) as usize;
        let mousey_min = (mousey - brush_span).clamp(0., GRID_HEIGHT as f32) as usize;
        let mousey_max = (mousey + brush_span).clamp(0., GRID_HEIGHT as f32) as usize;

        (mousex_min, mousex_max, mousey_min, mousey_max)
    }

    if is_key_pressed(KeyCode::Key1) {
        settings.placement_type = ParticleType::Sand;
        println!("Placement Type: Sand")
    } else if is_key_pressed(KeyCode::Key2) {
        settings.placement_type = ParticleType::Water;
        println!("Placement Type: Water")
    } else if is_key_pressed(KeyCode::Key3) {
        settings.placement_type = ParticleType::Concrete;
        println!("Placement Type: Concrete")
    } else if is_key_pressed(KeyCode::Key0) {
        settings.placement_type = ParticleType::Empty;
        println!("Placement Type: Empty")
    }

    // Add particles on left click
    if is_mouse_button_down(MouseButton::Left) {
        let (mousex_min, mousex_max, mousey_min, mousey_max) = calculate_brush(settings.brush_size);

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
                world.add_new_particle(settings.placement_type, x, y);
            }
        }
    }

    // Highlight a box around the brush is highlight_brush is true
    if settings.highlight_brush {
        let (mousex_min, mousex_max, mousey_min, mousey_max) = calculate_brush(settings.brush_size);
        let xpt = mousex_min as f32 * PIXELS_PER_PARTICLE;
        let ypt = mousey_min as f32 * PIXELS_PER_PARTICLE;
        let sizex = (mousex_max - mousex_min) as f32 * PIXELS_PER_PARTICLE;
        let sizey = (mousey_max - mousey_min) as f32 * PIXELS_PER_PARTICLE;

        draw_rectangle_lines(xpt, ypt, sizex, sizey, 3.0, RED);
        draw_rectangle(xpt, ypt, sizex, sizey, Color::new(1.0, 1.0, 0.0, 0.2));
    }

    if is_mouse_button_pressed(MouseButton::Right) {
        let (x, _, y, _) = calculate_brush(1.0);

        let p = world.grid[xy_to_index(x, y)];
        println!("({}, {}): {:?}", x, y, p);
    }

    if is_key_pressed(KeyCode::A) && settings.paused {
        world.draw_and_reset_all_particles();
        world.update_all_particles(rng);
    }

    if is_key_pressed(KeyCode::Space) {
        settings.paused = !settings.paused;
        if settings.paused {
            println!("PAUSING");
        } else {
            println!("UNPAUSING");
        }
    }

    if is_key_pressed(KeyCode::R) {
        *world = World::new();
    }

    if is_key_pressed(KeyCode::H) {
        settings.highlight_brush = !settings.highlight_brush;
    }

    let (_, mouse_wheel_y) = mouse_wheel();
    if (mouse_wheel_y - 0.0).abs() > 0.000001 {
        settings.brush_size += mouse_wheel_y.signum();
        settings.brush_size = settings
            .brush_size
            .clamp(1.0, usize::max(GRID_WIDTH, GRID_HEIGHT) as f32);
        // println!("Brush size: {}", brush_size);
    }

    if is_key_pressed(KeyCode::F) {
        settings.display_fps = !settings.display_fps;
    }
}
