// use color_eyre::eyre::Result;
use crate::core::*;
use egui_macroquad::*;
use enum_map::{enum_map, Enum, EnumMap};
use macroquad::prelude::*;

mod core;
// mod world;

const GRID_WIDTH_: usize = 250;
const GRID_HEIGHT_: usize = 150;
// const WORLD_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;
const PIXELS_PER_PARTICLE: f32 = 6.0;
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
        egui_macroquad::ui(|ctx| setup_ui(ctx, &mut settings, &mut world, fps));

        // ─── Drawing ─────────────────────────────────────────────────────────────
        // clear_background(BLACK);
        world.draw_and_reset_all_particles();
        // ─────────────────────────────────────────────────────────────────────────

        // ─── Input ───────────────────────────────────────────────────────────────
        handle_input(&mut settings, &mut world);
        // ─────────────────────────────────────────────────────────────────────────

        let time = get_time();
        let frame_time = time - tic;
        if !LIMIT_UPDATE_RATE || frame_time >= MINIMUM_UPDATE_TIME {
            // ─── Limiting And Printing Fps ───────────────────────────────
            tic = time;
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
                world.update_all_particles();
            }
            // ─────────────────────────────────────────────────────────────
        }
        egui_macroquad::draw();
        next_frame().await
    }
    // Ok(())
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
fn handle_input(settings: &mut Settings, world: &mut World) {
    // let grid_width = world.width;
    // let grid_height = world.height;

    // Change particle placement type with number keys
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
        let (mousex_min, mousex_max, mousey_min, mousey_max) =
            calculate_brush(settings.brush_size, world.width(), world.height());

        // println!("Brush span = {}", brush_span);
        for x in mousex_min..mousex_max {
            // println!("x = {}", x);
            if x >= world.width() {
                continue;
            }
            for y in mousey_min..mousey_max {
                if y >= world.height() {
                    continue;
                }
                world.add_new_particle(settings.placement_type, x, y);
            }
        }
    }
    // Highlight a box around the brush is highlight_brush is true
    if settings.highlight_brush {
        let (mousex_min, mousex_max, mousey_min, mousey_max) =
            calculate_brush(settings.brush_size, world.width(), world.height());
        let (px_min, py_min) = xy_to_pixels(mousex_min, mousey_min);
        // let xpt = mousex_min as f32 * PIXELS_PER_PARTICLE;
        // let ypt = mousey_min as f32 * PIXELS_PER_PARTICLE;
        let sizex = (mousex_max - mousex_min) as f32 * PIXELS_PER_PARTICLE;
        let sizey = (mousey_max - mousey_min) as f32 * PIXELS_PER_PARTICLE;

        let mut color = world.base_properties(settings.placement_type).base_color;
        color.a = 0.4;

        // draw_rectangle_lines(px_min, py_min, sizex, sizey, 3.0, RED);
        draw_rectangle(px_min, py_min, sizex, sizey, color);
    }
    // Print particle info on right click
    if is_mouse_button_pressed(MouseButton::Right) {
        println!("{}", debug_particle_string(world));
    }
    // Advance on "A" if paused
    if is_key_pressed(KeyCode::A) && settings.paused {
        world.draw_and_reset_all_particles();
        world.update_all_particles();
    }
    // Pause/Unpause with space
    if is_key_pressed(KeyCode::Space) {
        settings.paused = !settings.paused;
        if settings.paused {
            println!("PAUSING");
        } else {
            println!("UNPAUSING");
        }
    }
    // Reset on "R"
    if is_key_pressed(KeyCode::R) {
        *world = World::new(world.width(), world.height());
    }
    // Toggle highlighting with "H"
    if is_key_pressed(KeyCode::H) {
        settings.highlight_brush = !settings.highlight_brush;
    }
    // Display fps in console on "F"
    if is_key_pressed(KeyCode::F) {
        settings.display_fps = !settings.display_fps;
    }

    // Change brush size with mouse wheel
    let (_, mouse_wheel_y) = mouse_wheel();
    if (mouse_wheel_y - 0.0).abs() > 0.000001 {
        settings.brush_size += mouse_wheel_y.signum();
        settings.brush_size = settings
            .brush_size
            .clamp(1.0, usize::max(world.width(), world.height()) as f32);
        // println!("Brush size: {}", brush_size);
    }
}

/// Function to calculate the coordinates of the placement brush
fn calculate_brush(
    brush_size: f32,
    grid_width: usize,
    grid_height: usize,
) -> (usize, usize, usize, usize) {
    let (px, py) = mouse_position();
    let (mousex, mousey) = pixels_to_xy::<f32>(px, py);
    let brush_span = brush_size / 2.0;
    let mousex_min = (mousex - brush_span).clamp(0., grid_width as f32) as usize;
    let mousex_max = (mousex + brush_span).clamp(0., grid_width as f32) as usize;
    let mousey_min = (mousey - brush_span).clamp(0., grid_height as f32) as usize;
    let mousey_max = (mousey + brush_span).clamp(0., grid_height as f32) as usize;

    (mousex_min, mousex_max, mousey_min, mousey_max)
}

fn mouse_location(grid_width: usize, grid_height: usize) -> (usize, usize) {
    let (px, py) = mouse_position();
    let (mousex, mousey) = pixels_to_xy::<f32>(px, py);
    (
        mousex.clamp(0., grid_width as f32 - 1.0) as usize,
        mousey.clamp(0., grid_height as f32 - 1.0) as usize,
    )
}

fn debug_particle_string(world: &World) -> String {
    // let (x, _, y, _) = calculate_brush(1.0, world.width, world.height);
    let (x, y) = mouse_location(world.width(), world.height());
    let p = world.particle_at(x, y);
    format!("({}, {}): {:?}", x, y, p)
}

fn setup_ui(ctx: &egui::Context, settings: &mut Settings, world: &mut World, fps: f64) {
    egui::Window::new("")
        .resizable(false)
        .title_bar(false)
        .fixed_size([WORLD_PX0 - 13.0, WORLD_PY0])
        .resizable(false)
        .anchor(egui::Align2::LEFT_TOP, [0., 0.])
        .show(ctx, |ui| {
            // ui.label("Test");
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut settings.paused, true, "⏸");
                        ui.selectable_value(&mut settings.paused, false, "▶");
                        if ui.button("⏭").clicked() && settings.paused {
                            world.update_all_particles();
                        }
                    });
                });
                // ui.allocate_space(ui.);
                ui.group(|ui| {
                    ui.label(format!("Framerate: {:.1}", fps));
                });
            });
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut settings.placement_type,
                        ParticleType::Empty,
                        "Delete",
                    );
                    ui.selectable_value(&mut settings.placement_type, ParticleType::Sand, "Sand");
                    ui.selectable_value(&mut settings.placement_type, ParticleType::Water, "Water");
                    ui.selectable_value(
                        &mut settings.placement_type,
                        ParticleType::Concrete,
                        "Concrete",
                    );
                    ui.allocate_space(ui.available_size());
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
                    if ui.button("➕").clicked() {
                        settings.brush_size += 1.0;
                    }
                    if ui.button("➖").clicked() {
                        settings.brush_size -= 1.0;
                    }
                    ui.allocate_space(ui.available_size());
                })
            });
            ui.group(|ui| {
                ui.label(debug_particle_string(world));
            });
            ui.allocate_space(ui.available_size());
        });
}
