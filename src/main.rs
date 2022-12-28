use ::rand::{random, rngs::ThreadRng, seq::SliceRandom, thread_rng};
// use color_eyre::eyre::Result;
use egui_macroquad::*;
use macroquad::prelude::*;
use particle::*;

mod particle;
// mod world;

const GRID_WIDTH_: usize = 75;
const GRID_HEIGHT_: usize = 100;
// const WORLD_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;
const PIXELS_PER_PARTICLE: f32 = 4.0;
const WORLD_PX0: f32 = 300.0;
const WORLD_PY0: f32 = 0.0;

const MINIMUM_UPDATE_TIME: f64 = 1. / 90.;
// const MINIMUM_UPDATE_TIME: f64 = 1. / 1.;
const LIMIT_UPDATE_RATE: bool = false;
// const BRUSH_SIZE: f32 = 1.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sand".to_owned(),
        window_resizable: false,
        high_dpi: true,
        sample_count: 0,
        window_width: (WORLD_PX0 + GRID_WIDTH_ as f32 * PIXELS_PER_PARTICLE) as i32,
        window_height: (WORLD_PY0 + GRID_HEIGHT_ as f32 * PIXELS_PER_PARTICLE) as i32,
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
async fn main() {
    // color_eyre::install()?;

    // Something wrong with this on Mac for some reason
    // request_new_screen_size(
    //     GRID_WIDTH as f32 * PIXELS_PER_PARTICLE,
    //     GRID_HEIGHT as f32 * PIXELS_PER_PARTICLE,
    // );

    let mut rng = thread_rng();

    // Initialize Grid
    let mut world = World::new(GRID_WIDTH_, GRID_HEIGHT_);

    let mut tic = get_time();
    let mut fps_counter = 0.0;
    let mut frame_time_sum = 0.0;
    let mut fps = 0.0;

    let mut settings = Settings {
        paused: false,
        brush_size: 1.0,
        highlight_brush: true,
        display_fps: false,
        placement_type: ParticleType::Sand,
    };

    loop {
        let frame_time = get_time() - tic;

        egui_macroquad::ui(|ctx| setup_ui(ctx, &mut settings, &mut rng, &mut world, fps));

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
                fps = 50.0 / frame_time_sum;
                fps_counter = 0.0;
                frame_time_sum = 0.0;
                if settings.display_fps {
                    println!("{:.2}", fps);
                }

                // println!("width = {}, height = {}", screen_width(), screen_height());
            }
            // ─────────────────────────────────────────────────────────────

            // ─── Update All Particles ────────────────────────────────────
            if !settings.paused {
                world.update_all_particles(&mut rng);
            }
            // ─────────────────────────────────────────────────────────────
        }
        egui_macroquad::draw();
        next_frame().await
    }
    // Ok(())
}
// ───────────────────────────────────────────────────────────────────────────────────────────── ✣ ─

// ─── Grid Functions ────────────────────────────────────────────────────────────────────────── ✣ ─
struct World {
    grid: Vec<Particle>,
    width: usize,
    height: usize,
}

impl World {
    fn new(width: usize, height: usize) -> Self {
        let mut grid: Vec<Particle> = vec![];

        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    // println!("x: {:?}, y: {:?}", x, y);
                    grid.push(Particle::new(ParticleType::Border));
                } else {
                    grid.push(Particle::new(ParticleType::Empty));
                }
            }
        }
        Self {
            grid,
            width,
            height,
        }
    }

    fn add_new_particle(&mut self, new_particle_type: ParticleType, x: usize, y: usize) {
        let idx = self.xy_to_index(x, y);
        let old_particle_type = self.grid[idx].particle_type;
        match (new_particle_type, old_particle_type) {
            (_, ParticleType::Border) => {}
            (ParticleType::Empty, _) | (_, ParticleType::Empty) => {
                self.grid[idx] = Particle::new(new_particle_type);
            }
            _ => {}
        }
    }

    fn update_all_particles(&mut self, rng: &mut ThreadRng) {
        let mut idx_range: Vec<usize> = ((self.width + 1)..(self.width * self.height - 2))
            .rev()
            .collect();
        idx_range.shuffle(rng);
        for idx in idx_range.iter() {
            let idx = *idx;
            let (x, y) = self.index_to_xy(idx);
            let particle = self.grid[idx];

            if !particle.moved {
                match particle.particle_type {
                    ParticleType::Sand => {
                        let r = random();
                        let right: isize = if r { -1 } else { 1 };
                        let check_directions = [(0, 1), (right, 1), (0 - right, 1)];

                        for (dx, dy) in check_directions.iter() {
                            let (other_x, other_y) = ((x as isize + dx) as usize, y + dy);
                            let other_type =
                                self.grid[self.xy_to_index(other_x, other_y)].particle_type;

                            match other_type {
                                ParticleType::Empty | ParticleType::Water => {
                                    self.swap_particles(x, y, other_x, other_y);
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    ParticleType::Water => {
                        let r = random();
                        let right: isize = if r { -1 } else { 1 };
                        // let mut moving_right = particle.bool_state[1];
                        let check_directions = if particle.bool_state[1] {
                            [(0, 1), (right, 1), (0 - right, 1), (1, 0), (-1, 0)]
                        } else {
                            [(0, 1), (right, 1), (0 - right, 1), (-1, 0), (1, 0)]
                        };

                        for ((dx, dy), k) in check_directions.iter().zip(0..5) {
                            let (other_x, other_y) = ((x as isize + dx) as usize, y + dy);
                            let other_type =
                                self.grid[self.xy_to_index(other_x, other_y)].particle_type;

                            match other_type {
                                ParticleType::Empty => {
                                    if k == 4 {
                                        self.grid[idx].bool_state[1] =
                                            !self.grid[idx].bool_state[1];
                                        // moving_right = !moving_right;
                                    }
                                    self.swap_particles(x, y, other_x, other_y);
                                    break;
                                }
                                _ => {}
                            }
                        }
                        // particle.bool_state[1] = moving_right;
                    }
                    _ => {}
                }
            }
        }
    }

    fn swap_particles(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let idx1 = self.xy_to_index(x1, y1);
        let idx2 = self.xy_to_index(x2, y2);
        self.grid[idx1].moved = true;
        self.grid[idx2].moved = true;
        (self.grid[idx1], self.grid[idx2]) = (self.grid[idx2], self.grid[idx1]);
    }

    fn draw_and_reset_all_particles(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.xy_to_index(x, y);
                self.grid[idx].draw(x, y);
                self.grid[idx].moved = false;
            }
        }
    }

    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn index_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }
}
// ───────────────────────────────────────────────────────────────────────────────────────────── ✣ ─
fn pixels_to_xy<T: From<f32>>(px: f32, py: f32) -> (T, T) {
    (
        ((px - WORLD_PX0) / PIXELS_PER_PARTICLE).into(),
        ((py - WORLD_PY0) / PIXELS_PER_PARTICLE).into(),
    )
}

fn xy_to_pixels(x: usize, y: usize) -> (f32, f32) {
    (
        x as f32 * PIXELS_PER_PARTICLE + WORLD_PX0,
        y as f32 * PIXELS_PER_PARTICLE + WORLD_PY0,
    )
}

// ─── Handle Input ──────────────────────────────────────────────────────────────────────────── ✣ ─
fn handle_input(settings: &mut Settings, world: &mut World, rng: &mut ThreadRng) {
    let grid_width = world.width;
    let grid_height = world.height;

    // Function to calculate the coordinates of the placement brush
    let calculate_brush = |brush_size: f32| -> (usize, usize, usize, usize) {
        let (px, py) = mouse_position();
        let (mousex, mousey) = pixels_to_xy::<f32>(px, py);
        let brush_span = brush_size / 2.0;
        let mousex_min = (mousex - brush_span).clamp(0., grid_width as f32) as usize;
        let mousex_max = (mousex + brush_span).clamp(0., grid_width as f32) as usize;
        let mousey_min = (mousey - brush_span).clamp(0., grid_height as f32) as usize;
        let mousey_max = (mousey + brush_span).clamp(0., grid_height as f32) as usize;

        (mousex_min, mousex_max, mousey_min, mousey_max)
    };

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
            if x >= world.width {
                continue;
            }
            for y in mousey_min..mousey_max {
                if y >= world.height {
                    continue;
                }
                world.add_new_particle(settings.placement_type, x, y);
            }
        }
    }

    // Highlight a box around the brush is highlight_brush is true
    if settings.highlight_brush {
        let (mousex_min, mousex_max, mousey_min, mousey_max) = calculate_brush(settings.brush_size);
        let (px_min, py_min) = xy_to_pixels(mousex_min, mousey_min);
        // let xpt = mousex_min as f32 * PIXELS_PER_PARTICLE;
        // let ypt = mousey_min as f32 * PIXELS_PER_PARTICLE;
        let sizex = (mousex_max - mousex_min) as f32 * PIXELS_PER_PARTICLE;
        let sizey = (mousey_max - mousey_min) as f32 * PIXELS_PER_PARTICLE;

        draw_rectangle_lines(px_min, py_min, sizex, sizey, 3.0, RED);
        draw_rectangle(px_min, py_min, sizex, sizey, Color::new(1.0, 1.0, 0.0, 0.2));
    }

    if is_mouse_button_pressed(MouseButton::Right) {
        let (x, _, y, _) = calculate_brush(1.0);

        let p = world.grid[world.xy_to_index(x, y)];
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
        *world = World::new(world.width, world.height);
    }

    if is_key_pressed(KeyCode::H) {
        settings.highlight_brush = !settings.highlight_brush;
    }

    let (_, mouse_wheel_y) = mouse_wheel();
    if (mouse_wheel_y - 0.0).abs() > 0.000001 {
        settings.brush_size += mouse_wheel_y.signum();
        settings.brush_size = settings
            .brush_size
            .clamp(1.0, usize::max(world.width, world.height) as f32);
        // println!("Brush size: {}", brush_size);
    }

    if is_key_pressed(KeyCode::F) {
        settings.display_fps = !settings.display_fps;
    }
}

fn setup_ui(
    ctx: &egui::Context,
    settings: &mut Settings,
    rng: &mut ThreadRng,
    world: &mut World,
    fps: f64,
) {
    egui::Window::new("")
        .resizable(false)
        .title_bar(false)
        .fixed_size([WORLD_PX0 - 13.0, WORLD_PY0])
        .resizable(false)
        .anchor(egui::Align2::LEFT_TOP, [0., 0.])
        .show(ctx, |ui| {
            // ui.label("Test");
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut settings.paused, true, "⏸");
                    ui.selectable_value(&mut settings.paused, false, "▶");
                    if ui.button("⏭").clicked() && settings.paused {
                        world.update_all_particles(rng);
                    }
                })
            });
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut settings.placement_type, ParticleType::Empty, "Empty");
                    ui.selectable_value(&mut settings.placement_type, ParticleType::Sand, "Sand");
                    ui.selectable_value(&mut settings.placement_type, ParticleType::Water, "Water");
                    ui.selectable_value(
                        &mut settings.placement_type,
                        ParticleType::Concrete,
                        "Concrete",
                    );
                });
            });
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Brush Size: ");
                    ui.add(
                        egui::DragValue::new(&mut settings.brush_size)
                            .clamp_range(1.0..=30.0)
                            .fixed_decimals(0)
                            .speed(0.2),
                    );
                })
            });
            ui.label(format!("Framerate: {:.1}", fps));
            ui.allocate_space(ui.available_size());
        });
}
