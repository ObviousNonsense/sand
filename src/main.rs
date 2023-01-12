// use color_eyre::eyre::Result;
use crate::core::*;
use egui_macroquad::{egui::ComboBox, *};
// use enum_map::{enum_map, Enum, EnumMap};
use macroquad::prelude::*;

mod core;
// mod world;

const GRID_WIDTH_: usize = 50;
const GRID_HEIGHT_: usize = 50;
// const WORLD_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;
const PIXELS_PER_PARTICLE: f32 = 16.0;
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
    display_fps: bool,
    placeable_selector: PlaceableSelector,
    sources_replace: bool,
    placement_type: ParticleType,
    delete: bool,
    replace: bool,
    debug_mode: bool,
    portal_direction: Direction,
    last_portal_placed: Vec<(usize, usize)>,
    waiting_for_partner_portal: bool,
    portal_color: Color,
    portal_placement_valid: bool,
}

// ─── Main ──────────────────────────────────────────────────────────────────────────────────── ✣ ─
#[macroquad::main(window_conf)]
async fn main() {
    // color_eyre::install()?;

    // Something wrong with this on Mac for some reason. But also without it the
    // display is wrong on windows when the 4k monitor with 150% scaling is the
    // primary monitor
    // request_new_screen_size(
    //     WORLD_PX0 + GRID_WIDTH_ as f32 * PIXELS_PER_PARTICLE,
    //     WORLD_PY0 + GRID_HEIGHT_ as f32 * PIXELS_PER_PARTICLE,
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
        display_fps: false,
        placeable_selector: PlaceableSelector::Particle,
        sources_replace: false,
        placement_type: ParticleType::Sand,
        delete: false,
        replace: false,
        debug_mode: false,
        portal_direction: Direction::DOWN,
        last_portal_placed: vec![],
        waiting_for_partner_portal: false,
        portal_color: RED,
        portal_placement_valid: true,
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
                world.update_all();
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

    let (px, py) = mouse_position();

    if px > WORLD_PX0 && px < screen_width() && py > WORLD_PY0 && py < screen_height() {
        let (mousex_min, mousex_max, mousey_min, mousey_max) =
            calculate_brush(px, py, settings.brush_size, world.width(), world.height());

        let mousex_avg = ((mousex_min as f32 + mousex_max as f32) / 2.0).floor() as usize;
        let mousey_avg = ((mousey_min as f32 + mousey_max as f32) / 2.0).floor() as usize;

        // Check whether the location/size of the portal we're trying to place is valid
        settings.portal_placement_valid = true;
        if settings.placeable_selector == PlaceableSelector::Portal {
            match settings.portal_direction {
                Direction::UP | Direction::DOWN => {
                    for x in mousex_min..mousex_max {
                        if world.portal_exists_at((x, mousey_avg)) {
                            settings.portal_placement_valid = false;
                            break;
                        }
                    }
                }
                Direction::RIGHT | Direction::LEFT => {
                    for y in mousey_min..mousey_max {
                        if world.portal_exists_at((mousex_avg, y)) {
                            settings.portal_placement_valid = false;
                            break;
                        }
                    }
                }
            }
        }

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

                // Highlight brush
                highlight_brush(settings, x, y, mousex_avg, mousey_avg);

                // Add particles on left click
                if is_mouse_button_down(MouseButton::Left) {
                    if settings.delete {
                        world.delete_source((x, y));
                        world.add_new_particle(ParticleType::Empty, (x, y), settings.replace);
                    } else {
                        create_placeable(settings, world, (x, y), mousex_avg, mousey_avg);
                    }
                }
            }
        }
    }

    // Advance on "A" if paused
    if is_key_pressed(KeyCode::A) && settings.paused {
        world.draw_and_reset_all_particles();
        world.update_all();
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
        settings.last_portal_placed = vec![];
        settings.waiting_for_partner_portal = false;
        *world = World::new(world.width(), world.height());
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

fn highlight_brush(settings: &Settings, x: usize, y: usize, mousex: usize, mousey: usize) {
    match settings.placeable_selector {
        PlaceableSelector::Particle => {
            let mut color = base_properties(settings.placement_type).base_color;
            color.a = 0.4;
            draw_particle(x, y, color);
        }
        PlaceableSelector::Source => {
            let mut color = base_properties(settings.placement_type).base_color;
            color.a = 0.4;
            color.r -= 0.1;
            color.g -= 0.1;
            color.b -= 0.1;
            draw_source(x, y, color, settings.sources_replace, false);
        }
        PlaceableSelector::Sink => {
            let mut color = base_properties(settings.placement_type).base_color;
            color.a = 0.4;
            color.r -= 0.1;
            color.g -= 0.1;
            color.b -= 0.1;
            draw_source(x, y, color, settings.sources_replace, true);
        }
        PlaceableSelector::Portal => {
            if !settings.portal_placement_valid {
                return;
            }
            match settings.portal_direction {
                Direction::UP | Direction::DOWN => {
                    if y != mousey {
                        return;
                    }
                }
                Direction::RIGHT | Direction::LEFT => {
                    if x != mousex {
                        return;
                    }
                }
            }
            let mut color = settings.portal_color;
            color.a = 0.4;
            draw_portal(x, y, settings.portal_direction, color);
        }
    }
}

fn create_placeable(
    settings: &mut Settings,
    world: &mut World,
    xy: (usize, usize),
    mousex: usize,
    mousey: usize,
) {
    match settings.placeable_selector {
        PlaceableSelector::Particle => {
            world.add_new_particle(settings.placement_type, xy, settings.replace);
        }
        PlaceableSelector::Source => {
            world.add_new_source(
                settings.placement_type,
                xy,
                settings.sources_replace,
                settings.replace,
            );
        }
        PlaceableSelector::Sink => {
            world.add_new_source(ParticleType::Empty, xy, true, settings.replace);
        }
        PlaceableSelector::Portal => {
            if !settings.portal_placement_valid {
                return;
            }

            match settings.portal_direction {
                Direction::UP | Direction::DOWN => {
                    if xy.1 != mousey {
                        return;
                    }
                }
                Direction::RIGHT | Direction::LEFT => {
                    if xy.0 != mousex {
                        return;
                    }
                }
            }

            println!(
                "Before: waiting for partner = {}",
                settings.waiting_for_partner_portal
            );

            let partner_xy;
            if settings.waiting_for_partner_portal {
                partner_xy = settings.last_portal_placed.pop()
            } else {
                partner_xy = None;
            }

            // TODO: Since I'm checking in advance whether there's already a
            // portal there now, checking again here is redundant
            let added = world.add_new_portal(xy, partner_xy, settings.portal_direction, RED);

            if added {
                if !settings.waiting_for_partner_portal {
                    settings.last_portal_placed.push(xy);
                } else if settings.last_portal_placed.is_empty() {
                    settings.waiting_for_partner_portal = false;
                }

                if settings.last_portal_placed.len() == settings.brush_size as usize {
                    settings.waiting_for_partner_portal = true;
                }
                // let last_portal_placed = match settings.last_portal_placed {
                //     Some(_) => None,
                //     None => Some(xy),
                // };
                // settings.last_portal_placed = last_portal_placed;

                println!(
                    "Creating Portal at {:?} with partner at {:?}",
                    xy, partner_xy,
                );
                println!("Last Portal placed = {:?}", settings.last_portal_placed);
                println!(
                    "After: waiting for partner = {}",
                    settings.waiting_for_partner_portal
                );
                println!("------------------------------")
            }
        }
    }
}

/// Function to calculate the coordinates of the placement brush
fn calculate_brush(
    px: f32,
    py: f32,
    brush_size: f32,
    grid_width: usize,
    grid_height: usize,
) -> (usize, usize, usize, usize) {
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
    let p = world.particle_at((x, y));
    format!("({}, {}): {:?}", x, y, p)
}

#[derive(Debug, PartialEq)]
enum PlaceableSelector {
    Particle,
    Source,
    Sink,
    Portal,
}

impl PlaceableSelector {
    pub fn as_str(&self) -> &str {
        match self {
            PlaceableSelector::Particle => "Particle",
            PlaceableSelector::Source => "Source",
            PlaceableSelector::Sink => "Sink",
            PlaceableSelector::Portal => "Portal",
        }
    }
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
                    // ui.horizontal(|ui| {
                    ui.selectable_value(&mut settings.paused, true, "⏸");
                    ui.selectable_value(&mut settings.paused, false, "▶");
                    if ui.button("⏭").clicked() && settings.paused {
                        world.update_all();
                    }
                    // });
                });
                // ui.allocate_space(ui.);
                ui.group(|ui| {
                    ui.label(format!("Framerate: {:.1}", fps));
                });
                ui.group(|ui| ui.checkbox(&mut settings.debug_mode, "debug"));
            });
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ComboBox::from_label("")
                        .selected_text(settings.placeable_selector.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut settings.placeable_selector,
                                PlaceableSelector::Particle,
                                "Particle",
                            );
                            ui.selectable_value(
                                &mut settings.placeable_selector,
                                PlaceableSelector::Source,
                                "Source",
                            );
                            ui.selectable_value(
                                &mut settings.placeable_selector,
                                PlaceableSelector::Sink,
                                "Sink",
                            );
                            ui.selectable_value(
                                &mut settings.placeable_selector,
                                PlaceableSelector::Portal,
                                "Portal",
                            );
                        })
                });
                if settings.placeable_selector == PlaceableSelector::Source {
                    ui.group(|ui| {
                        ui.checkbox(&mut settings.sources_replace, "New Sources Replace?")
                    });
                }
            });
            ui.group(|ui| {
                ui.toggle_value(&mut settings.delete, "Delete");
            });
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    // ui.style_mut().visuals.
                    // let mut job = egui::text::LayoutJob::default();
                    // job.append(
                    //     "Sand",
                    //     0.0,
                    //     TextFormat {
                    //         background: egui::Color32::YELLOW,
                    //         ..Default::default()
                    //     },
                    // );
                    if settings.placeable_selector == PlaceableSelector::Portal {
                        ui.label("Portal Direction: ");
                        ui.selectable_value(&mut settings.portal_direction, Direction::UP, "Up");
                        ui.selectable_value(
                            &mut settings.portal_direction,
                            Direction::RIGHT,
                            "Right",
                        );
                        ui.selectable_value(
                            &mut settings.portal_direction,
                            Direction::DOWN,
                            "Down",
                        );
                        ui.selectable_value(
                            &mut settings.portal_direction,
                            Direction::LEFT,
                            "Left",
                        );
                    } else if settings.delete
                        || settings.placeable_selector == PlaceableSelector::Sink
                    {
                        settings.placement_type = ParticleType::Empty;
                        ui.label("Sand");
                        ui.label("Water");
                        ui.label("Concrete");
                    } else {
                        ui.selectable_value(
                            &mut settings.placement_type,
                            ParticleType::Sand,
                            "Sand",
                        );
                        ui.selectable_value(
                            &mut settings.placement_type,
                            ParticleType::Water,
                            "Water",
                        );
                        ui.selectable_value(
                            &mut settings.placement_type,
                            ParticleType::Concrete,
                            "Concrete",
                        );
                    }
                    ui.allocate_space(ui.available_size());
                });
            });
            ui.horizontal(|ui| {
                ui.group(|ui| {
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
                });
                ui.group(|ui| {
                    ui.checkbox(&mut settings.replace, "Replace");
                });
                ui.allocate_space(ui.available_size());
            });
            if settings.debug_mode {
                ui.group(|ui| {
                    ui.label(debug_particle_string(world));
                });
            }
            ui.allocate_space(ui.available_size());
        });
}
