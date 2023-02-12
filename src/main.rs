use egui_macroquad::{egui, egui::RichText, *};
use macroquad::{
    color::{hsl_to_rgb, rgb_to_hsl},
    prelude::*,
};
use particle::*;
use std::iter::Cycle;
use world::*;

mod particle;
mod world;

const MINIMUM_UPDATE_TIME: f64 = 1. / 90.;
// const MINIMUM_UPDATE_TIME: f64 = 1. / 1.;
const LIMIT_UPDATE_RATE: bool = false;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sand".to_owned(),
        window_resizable: false,
        high_dpi: true,
        sample_count: 0,
        ..Default::default()
    }
}

// ─── Main ──────────────────────────────────────────────────────────────────────────────────── ✣ ─
#[macroquad::main(window_conf)]
async fn main() {
    // color_eyre::install()?;

    let mut color_cycle = vec![RED, LIME, VIOLET, PINK, ORANGE, GOLD, BEIGE, WHITE]
        .into_iter()
        .cycle();

    let painter = Painter {
        pixels_per_particle: 6.0,
        world_px0: 225.0,
        world_py0: 0.0,
    };

    let mut settings = Settings {
        paused: false,
        brush_size: 1.0,
        display_fps: false,
        placeable_selector: PlaceableSelector::Particle,
        sources_replace: false,
        placement_type: ParticleType::Sand,
        last_placement_type: ParticleType::Sand,
        delete: false,
        replace: false,
        debug_mode: false,
        portal_direction: Direction::Down,
        last_portal_placed: vec![],
        waiting_for_partner_portal: false,
        portal_color: color_cycle.next().unwrap(),
        portal_placement_valid: true,
        portal_color_cycle: color_cycle,
        new_pixels_per_particle: painter.pixels_per_particle,
        new_size: (150, 150),
        painter,
    };

    let mut world = settings.resize_world_and_screen();

    let mut tic = get_time();
    let mut fps_counter = 0.0;
    let mut frame_time_sum = 0.0;
    let mut fps = 0.0;

    loop {
        egui_macroquad::ui(|ctx| setup_ui(ctx, &mut settings, &mut world, fps));

        // ─── Drawing ─────────────────────────────────────────────────────────────
        // clear_background(BLACK);
        world.draw_and_reset_all_particles(&settings.painter);
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

struct Settings {
    paused: bool,
    brush_size: f32,
    display_fps: bool,
    placeable_selector: PlaceableSelector,
    sources_replace: bool,
    placement_type: ParticleType,
    last_placement_type: ParticleType,
    delete: bool,
    replace: bool,
    debug_mode: bool,
    portal_direction: Direction,
    last_portal_placed: Vec<(usize, usize)>,
    waiting_for_partner_portal: bool,
    portal_color: Color,
    portal_placement_valid: bool,
    portal_color_cycle: Cycle<std::vec::IntoIter<Color>>,
    new_pixels_per_particle: f32,
    new_size: (usize, usize),
    painter: Painter,
}

impl Settings {
    fn resize_world_and_screen(&mut self) -> World {
        self.painter.pixels_per_particle = self.new_pixels_per_particle;
        // Something wrong with this on Mac for some reason. But also without it the
        // display is wrong on windows when the 4k monitor with 150% scaling is the
        // primary monitor
        request_new_screen_size(
            self.painter.world_px0 + self.new_size.0 as f32 * self.painter.pixels_per_particle,
            self.painter.world_py0 + self.new_size.1 as f32 * self.painter.pixels_per_particle,
        );

        self.last_portal_placed = vec![];
        self.waiting_for_partner_portal = false;

        World::new(self.new_size.0, self.new_size.1)
    }
}

pub struct Painter {
    world_px0: f32,
    world_py0: f32,
    pixels_per_particle: f32,
}

impl Painter {
    fn pixels_to_xy<T: From<f32>>(&self, px: f32, py: f32) -> (T, T) {
        (
            ((px - self.world_px0) / self.pixels_per_particle).into(),
            ((py - self.world_py0) / self.pixels_per_particle).into(),
        )
    }

    fn xy_to_pixels(&self, x: usize, y: usize) -> (f32, f32) {
        (
            x as f32 * self.pixels_per_particle + self.world_px0,
            y as f32 * self.pixels_per_particle + self.world_py0,
        )
    }

    /// Function to calculate the coordinates of the placement brush
    fn calculate_brush(
        &self,
        px: f32,
        py: f32,
        brush_size: f32,
        grid_width: usize,
        grid_height: usize,
    ) -> (usize, usize, usize, usize) {
        let (mousex, mousey) = self.pixels_to_xy::<f32>(px, py);
        let brush_span = brush_size / 2.0;
        let mousex_min = (mousex - brush_span).clamp(0., grid_width as f32) as usize;
        let mousex_max = (mousex + brush_span).clamp(0., grid_width as f32) as usize;
        let mousey_min = (mousey - brush_span).clamp(0., grid_height as f32) as usize;
        let mousey_max = (mousey + brush_span).clamp(0., grid_height as f32) as usize;

        (mousex_min, mousex_max, mousey_min, mousey_max)
    }

    fn mouse_location(&self, grid_width: usize, grid_height: usize) -> (usize, usize) {
        let (px, py) = mouse_position();
        let (mousex, mousey) = self.pixels_to_xy::<f32>(px, py);
        (
            mousex.clamp(0., grid_width as f32 - 1.0) as usize,
            mousey.clamp(0., grid_height as f32 - 1.0) as usize,
        )
    }

    fn draw_particle(&self, x: usize, y: usize, color: Color) {
        let (px, py) = self.xy_to_pixels(x, y);
        draw_rectangle(
            px,
            py,
            self.pixels_per_particle,
            self.pixels_per_particle,
            color,
        );
    }

    fn draw_source(&self, x: usize, y: usize, color: Color, replaces: bool, sink: bool) {
        let (px, py) = self.xy_to_pixels(x, y);
        draw_rectangle(
            px,
            py,
            self.pixels_per_particle,
            self.pixels_per_particle,
            color,
        );

        // if !empty {
        let hatch_color = if replaces || sink {
            Color::new(0.0, 0.0, 0.0, 0.2)
        } else {
            Color::new(1.0, 1.0, 1.0, 0.5)
        };

        draw_line(
            px,
            py,
            px + self.pixels_per_particle,
            py + self.pixels_per_particle,
            1.0,
            hatch_color,
        );
    }

    fn draw_portal(&self, x: usize, y: usize, direction: Direction, color: Color) {
        let (px, py) = self.xy_to_pixels(x, y);
        // draw_line()
        let pix_per = self.pixels_per_particle;
        let thickness = pix_per / 4.0;
        let (ptx, pty, w, h): (f32, f32, f32, f32) = match direction {
            Direction::Up => (px, py, pix_per, thickness),
            Direction::Right => (px + pix_per - thickness, py, thickness, pix_per),
            Direction::Down => (px, py + pix_per - thickness, pix_per, thickness),
            Direction::Left => (px, py, thickness, pix_per),
        };

        draw_rectangle(ptx, pty, w, h, color);
    }
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

    if px > settings.painter.world_px0
        && px < screen_width()
        && py > settings.painter.world_py0
        && py < screen_height()
    {
        let (mousex_min, mousex_max, mousey_min, mousey_max) = settings.painter.calculate_brush(
            px,
            py,
            settings.brush_size,
            world.width(),
            world.height(),
        );

        let (mousex, mousey) = settings
            .painter
            .mouse_location(world.width(), world.height());

        // Check whether the location/size of the portal we're trying to place is valid
        settings.portal_placement_valid = true;
        if settings.placeable_selector == PlaceableSelector::Portal {
            match settings.portal_direction {
                Direction::Up | Direction::Down => {
                    for x in mousex_min..mousex_max {
                        if x < 1 || x >= world.width() - 1 || world.portal_exists_at((x, mousey)) {
                            settings.portal_placement_valid = false;
                            break;
                        }
                    }
                }
                Direction::Right | Direction::Left => {
                    for y in mousey_min..mousey_max {
                        if y < 1 || y >= world.width() - 1 || world.portal_exists_at((mousex, y)) {
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
                highlight_brush(settings, x, y, mousex, mousey);

                // Add particles on left click
                if is_mouse_button_down(MouseButton::Left) {
                    if settings.delete {
                        world.delete_source((x, y));
                        world.add_new_particle(ParticleType::Empty, (x, y), settings.replace);
                    } else {
                        create_placeable(settings, world, (x, y), mousex, mousey);
                    }
                }
            }
        }
    }

    // Advance on "A" if paused
    if is_key_pressed(KeyCode::A) && settings.paused {
        world.draw_and_reset_all_particles(&settings.painter);
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
        *world = settings.resize_world_and_screen();
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
            let mut color = settings.placement_type.properties().base_color;
            color.a = 0.4;
            settings.painter.draw_particle(x, y, color);
        }
        PlaceableSelector::Source => {
            let mut color = settings.placement_type.properties().base_color;
            color.a = 0.4;
            color.r -= 0.1;
            color.g -= 0.1;
            color.b -= 0.1;
            settings
                .painter
                .draw_source(x, y, color, settings.sources_replace, false);
        }
        PlaceableSelector::Sink => {
            let mut color = settings.placement_type.properties().base_color;
            color.a = 0.4;
            color.r -= 0.1;
            color.g -= 0.1;
            color.b -= 0.1;
            settings
                .painter
                .draw_source(x, y, color, settings.sources_replace, true);
        }
        PlaceableSelector::Portal => {
            if !settings.portal_placement_valid {
                return;
            }
            match settings.portal_direction {
                Direction::Up | Direction::Down => {
                    if y != mousey {
                        return;
                    }
                }
                Direction::Right | Direction::Left => {
                    if x != mousex {
                        return;
                    }
                }
            }
            let mut color = settings.portal_color;
            color.a = 0.4;
            settings
                .painter
                .draw_portal(x, y, settings.portal_direction, color);
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
                Direction::Up | Direction::Down => {
                    if xy.1 != mousey {
                        return;
                    }
                }
                Direction::Right | Direction::Left => {
                    if xy.0 != mousex {
                        return;
                    }
                }
            }

            if settings.debug_mode {
                println!(
                    "Before: waiting for partner = {}",
                    settings.waiting_for_partner_portal
                );
            }

            let partner_xy = if settings.waiting_for_partner_portal {
                let mut color = rgb_to_hsl(settings.portal_color);
                color.1 += 0.01;
                color.2 += 0.01;
                settings.portal_color = hsl_to_rgb(color.0, color.1, color.2);
                settings.last_portal_placed.pop()
            } else {
                None
            };

            // TODO: Since I'm checking in advance whether there's already a
            // portal there now, checking again here is redundant
            let added = world.add_new_portal(
                xy,
                partner_xy,
                settings.portal_direction,
                settings.portal_color,
            );

            if added {
                if !settings.waiting_for_partner_portal {
                    settings.last_portal_placed.push(xy);
                    let mut color = rgb_to_hsl(settings.portal_color);
                    color.1 -= 0.01;
                    color.2 -= 0.01;
                    settings.portal_color = hsl_to_rgb(color.0, color.1, color.2);
                } else if settings.last_portal_placed.is_empty() {
                    settings.portal_color = settings.portal_color_cycle.next().unwrap();
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

                if settings.debug_mode {
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
}

fn debug_particle_string(world: &World, painter: &Painter) -> String {
    // let (x, _, y, _) = calculate_brush(1.0, world.width, world.height);
    let (x, y) = painter.mouse_location(world.width(), world.height());
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
        // .resizable(false)
        .title_bar(false)
        .fixed_size([
            settings.painter.world_px0 - 13.0,
            settings.painter.world_py0,
        ])
        .anchor(egui::Align2::LEFT_TOP, [0., 0.])
        .show(ctx, |ui| {
            // ui.label("Test");
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.selectable_value(&mut settings.paused, true, "⏸");
                    ui.selectable_value(&mut settings.paused, false, "▶");
                    if ui.button("⏭").clicked() && settings.paused {
                        world.update_all();
                    }
                });

                ui.group(|ui| {
                    ui.label(format!("FPS: {:.1}", fps));
                });
            });

            ui.separator();

            egui::Grid::new("1").num_columns(2).show(ui, |ui| {
                egui::Grid::new("")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        // ui.group(|ui| {
                        // ui.horizontal(|ui| {
                        ui.label("New Width:  ");
                        ui.add(
                            egui::DragValue::new(&mut settings.new_size.0)
                                .clamp_range(1..=1000)
                                .fixed_decimals(0)
                                .speed(5),
                        );
                        // });
                        ui.end_row();
                        // ui.horizontal(|ui| {
                        ui.label("New Height: ");
                        ui.add(
                            egui::DragValue::new(&mut settings.new_size.1)
                                .clamp_range(1..=1000)
                                .fixed_decimals(0)
                                .speed(5),
                        );
                        // });
                        ui.end_row();

                        // ui.horizontal(|ui| {
                        ui.label("New Scale: ");
                        ui.add(
                            egui::DragValue::new(&mut settings.new_pixels_per_particle)
                                .clamp_range(1.0..=30.0)
                                .fixed_decimals(0)
                                .speed(0.5),
                        );
                        // });
                        ui.end_row();
                    });
                if ui.add(egui::Button::new("Reset/\nResize")).clicked() {
                    *world = settings.resize_world_and_screen();
                }
                ui.end_row();
                // });
            });

            ui.separator();

            egui::Grid::new("2")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Debug");
                    ui.checkbox(&mut settings.debug_mode, "");
                    ui.end_row();

                    ui.label("New Sources Replace");
                    ui.checkbox(&mut settings.sources_replace, "");
                    ui.end_row();

                    ui.label("Replace");
                    ui.checkbox(&mut settings.replace, "");
                    ui.end_row();

                    ui.label("Brush Size");
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut settings.brush_size)
                                .clamp_range(1.0..=30.0)
                                .fixed_decimals(0)
                                .speed(0.2),
                        );
                        // if ui.button("➕").clicked() {
                        //     settings.brush_size += 1.0;
                        // }
                        // if ui.button("➖").clicked() {
                        //     settings.brush_size -= 1.0;
                        // }
                    });
                    ui.end_row();
                });

            // ui.label("Placement Type");
            // egui::ComboBox::from_label("")
            //     .selected_text(settings.placeable_selector.as_str())
            //     .width(1.0)
            //     .show_ui(ui, |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut settings.placeable_selector,
                    PlaceableSelector::Particle,
                    PlaceableSelector::Particle.as_str(),
                );
                ui.selectable_value(
                    &mut settings.placeable_selector,
                    PlaceableSelector::Source,
                    PlaceableSelector::Source.as_str(),
                );
                ui.selectable_value(
                    &mut settings.placeable_selector,
                    PlaceableSelector::Sink,
                    PlaceableSelector::Sink.as_str(),
                );
                ui.selectable_value(
                    &mut settings.placeable_selector,
                    PlaceableSelector::Portal,
                    PlaceableSelector::Portal.as_str(),
                );
            });
            // });
            // ui.end_row();
            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.group(|ui| {
                        ui.toggle_value(&mut settings.delete, "Delete");
                    });
                    if settings.delete || settings.placeable_selector == PlaceableSelector::Sink {
                        if settings.placement_type != ParticleType::Empty {
                            settings.last_placement_type = settings.placement_type;
                        }
                        settings.placement_type = ParticleType::Empty;
                    } else if settings.placement_type == ParticleType::Empty {
                        settings.placement_type = settings.last_placement_type;
                    }

                    // ui.visuals_mut().selection = egui::style::Selection {
                    //     bg_fill: egui::Color32::from_white_alpha(100),
                    //     stroke: egui::Stroke {
                    //         width: 10.0,
                    //         color: egui::Color32::RED,
                    //     },
                    // };

                    particle_selector(ui, ParticleType::Sand, settings);
                    particle_selector(ui, ParticleType::Water, settings);
                    particle_selector(ui, ParticleType::Concrete, settings);
                    particle_selector(ui, ParticleType::Steam, settings);
                    particle_selector(ui, ParticleType::Fungus, settings);
                    particle_selector(ui, ParticleType::Flame, settings);
                    particle_selector(ui, ParticleType::Methane, settings);
                    particle_selector(ui, ParticleType::Gunpowder, settings);
                    particle_selector(ui, ParticleType::Oil, settings);
                    particle_selector(ui, ParticleType::Wood, settings);
                });
            });
            ui.separator();
            egui::Grid::new("3").num_columns(4).show(ui, |ui| {
                ui.label("");
                ui.label("");
                ui.selectable_value(
                    &mut settings.portal_direction,
                    Direction::Up,
                    RichText::new("⮉").size(24.0),
                );
                ui.label("");
                ui.end_row();
                ui.label("Portal\nDirection:");
                ui.selectable_value(
                    &mut settings.portal_direction,
                    Direction::Left,
                    RichText::new("⮈").size(24.0),
                );
                ui.label("");
                ui.selectable_value(
                    &mut settings.portal_direction,
                    Direction::Right,
                    RichText::new("⮊").size(24.0),
                );
                ui.end_row();
                ui.label("");
                ui.label("");
                ui.selectable_value(
                    &mut settings.portal_direction,
                    Direction::Down,
                    RichText::new("⮋").size(24.0),
                );
                ui.label("");
                ui.end_row();
            });

            if settings.debug_mode {
                ui.group(|ui| {
                    ui.label(debug_particle_string(world, &settings.painter));
                });
            }
        });
}

fn particle_selector(ui: &mut egui::Ui, ptype: ParticleType, settings: &mut Settings) {
    // ui.selectable_value(&mut settings.placement_type, ptype, "");
    egui::Frame::none()
        .fill(ptype.properties().base_color.to_egui())
        .show(ui, |ui| {
            // ui.label("");
            ui.selectable_value(
                &mut settings.placement_type,
                ptype,
                RichText::new(ptype.properties().label)
                    .strong()
                    .monospace()
                    .background_color(egui::Color32::from_black_alpha(150)),
            );
            // ui.label("");
            // ui.allocate_space(ui.available_size());
        });
}

trait ToEguiColor {
    fn to_egui(&self) -> egui::color::Color32;
}

impl ToEguiColor for macroquad::color::Color {
    fn to_egui(&self) -> egui::color::Color32 {
        egui::Color32::from_rgba_unmultiplied(
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        )
    }
}
