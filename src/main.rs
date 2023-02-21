use egui_macroquad::{egui, egui::RichText, *};
use macroquad::prelude::*;
use particle::*;
use std::iter::Cycle;
use world::*;

mod particle;
mod world;

const MINIMUM_UPDATE_TIME: f64 = 1. / 80.;
// const MINIMUM_UPDATE_TIME: f64 = 1. / 1.;
const LIMIT_UPDATE_RATE: bool = false;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sand".to_owned(),
        // window_resizable: false,
        // high_dpi: true,
        sample_count: 0,
        ..Default::default()
    }
}

// #[cfg(feature = "dhat-heap")]
// #[global_allocator]
// static ALLOC: dhat::Alloc = dhat::Alloc;

// ─── Main ──────────────────────────────────────────────────────────────────────────────────── ✣ ─
// #[cfg(feature = "dhat-heap")]
#[macroquad::main(window_conf)]
async fn main() {
    // let _profiler = dhat::Profiler::new_heap();
    // color_eyre::install()?;

    let mut color_cycle: Cycle<std::vec::IntoIter<particle::PColor>> = vec![
        RED.into(),
        LIME.into(),
        VIOLET.into(),
        PINK.into(),
        ORANGE.into(),
        GOLD.into(),
        BEIGE.into(),
        WHITE.into(),
    ]
    .into_iter()
    .cycle();

    let chunk_size = 16;
    let world_height = 12 * chunk_size;
    let world_width = 12 * chunk_size;
    let pixels_per_particle = 4;

    let painter = Painter::new(
        300.0,
        0.0,
        pixels_per_particle as f32,
        world_width,
        world_height,
    );

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
        debug_mode: true,
        portal_direction: Direction::Down,
        last_portal_placed: vec![],
        waiting_for_partner_portal: false,
        portal_color: color_cycle.next().unwrap(),
        portal_placement_valid: true,
        portal_color_cycle: color_cycle,
        new_pixels_per_particle: painter.pixels_per_particle,
        new_size: (world_width, world_height),
        mouse_over_gui: false,
        painter,
        drawing_style: DrawingStyle::Brush,
        draw_xy1: None,
        chunk_size,
    };

    // println!("{:#?}", settings);

    let mut world = settings.resize_world_and_screen();

    let mut tic = get_time();
    let mut fps_counter = 0.0;
    let mut frame_time_sum = 0.0;
    let mut fps = 0.0;

    loop {
        // unsafe {
        //     macroquad::window::get_internal_gl().flush();
        // }
        let time = get_time();
        let frame_time = time - tic;
        egui_macroquad::ui(|ctx| setup_ui(ctx, &mut settings, &mut world, fps));
        keys_input(&mut settings, &mut world);

        if settings.painter.pixels_per_particle != settings.new_pixels_per_particle {
            settings.rescale();
        }

        // ─── Drawing ─────────────────────────────────────────────────────────────
        clear_background(BLACK);
        world.draw_and_refresh(&mut settings.painter, settings.debug_mode);
        // ─────────────────────────────────────────────────────────────────────────

        cursor_input(&mut settings, &mut world);

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

#[derive(Debug)]
struct Settings {
    debug_mode: bool,
    paused: bool,
    brush_size: f32,
    display_fps: bool,
    placeable_selector: PlaceableSelector,
    drawing_style: DrawingStyle,
    sources_replace: bool,
    placement_type: ParticleType,
    last_placement_type: ParticleType,
    delete: bool,
    replace: bool,
    portal_direction: Direction,
    last_portal_placed: Vec<(usize, usize)>,
    waiting_for_partner_portal: bool,
    portal_color: PColor,
    portal_placement_valid: bool,
    draw_xy1: Option<(usize, usize)>,
    new_size: (usize, usize),
    new_pixels_per_particle: f32,
    chunk_size: usize,
    mouse_over_gui: bool,
    painter: Painter,
    portal_color_cycle: Cycle<std::vec::IntoIter<PColor>>,
}

impl Settings {
    fn resize_world_and_screen(&mut self) -> World {
        self.painter = Painter::new(
            self.painter.world_pxmin,
            self.painter.world_pymin,
            self.new_pixels_per_particle,
            self.new_size.0,
            self.new_size.1,
        );

        // self.painter.pixels_per_particle = self.new_pixels_per_particle;
        self.update_screen_size();

        self.last_portal_placed = vec![];
        self.waiting_for_partner_portal = false;

        World::new(self.new_size.0, self.new_size.1, self.chunk_size)
    }

    fn rescale(&mut self) {
        self.painter.pixels_per_particle = self.new_pixels_per_particle;
        self.update_screen_size();
    }

    fn update_screen_size(&self) {
        // Something wrong with this on Mac for some reason. But also without it the
        // display is wrong on windows when the 4k monitor with 150% scaling is the
        // primary monitor
        request_new_screen_size(
            300.0
                + self.painter.world_pxmin
                + self.new_size.0 as f32 * self.painter.pixels_per_particle,
            self.painter.world_pymin + self.new_size.1 as f32 * self.painter.pixels_per_particle,
        );
    }
}

pub struct Painter {
    world_pxmin: f32,
    world_pxmax: f32,
    world_pymin: f32,
    world_pymax: f32,
    pixels_per_particle: f32,
    screen_buffer: Vec<u8>,
    screen_texture: Texture2D,
}

impl core::fmt::Debug for Painter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Painter")
            .field("world_pxmin", &self.world_pxmin)
            .field("world_pxmax", &self.world_pxmax)
            .field("world_pymin", &self.world_pymin)
            .field("world_pymax", &self.world_pymax)
            .field("pixels_per_particle", &self.pixels_per_particle)
            .finish_non_exhaustive()
    }
}

impl Painter {
    fn new(
        world_pxmin: f32,
        world_pymin: f32,
        pixels_per_particle: f32,
        world_width: usize,
        world_height: usize,
    ) -> Self {
        let screen_buffer: Vec<u8> = std::iter::repeat(255)
            .take(4 * world_width * world_height)
            .collect();

        let screen_texture =
            Texture2D::from_rgba8(world_width as u16, world_height as u16, &screen_buffer);
        screen_texture.set_filter(FilterMode::Nearest);

        Self {
            world_pxmin,
            world_pxmax: world_pxmin + pixels_per_particle * world_width as f32,
            world_pymin,
            world_pymax: world_pymin + pixels_per_particle * world_height as f32,
            pixels_per_particle,
            screen_buffer,
            screen_texture,
        }
    }

    fn pixels_to_xy<T: From<f32>>(&self, px: f32, py: f32) -> (T, T) {
        (
            ((px - self.world_pxmin) / self.pixels_per_particle).into(),
            ((py - self.world_pymin) / self.pixels_per_particle).into(),
        )
    }

    fn xy_to_pixels(&self, x: usize, y: usize) -> (f32, f32) {
        (
            x as f32 * self.pixels_per_particle + self.world_pxmin,
            y as f32 * self.pixels_per_particle + self.world_pymin,
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
        // draw_texture(self.particle_texture, px, py, color);
    }

    fn update_image_with_particle(&mut self, x: usize, y: usize, width: usize, color: PColor) {
        let idx = x + y * width;
        self.screen_buffer[4 * idx] = color.r;
        self.screen_buffer[4 * idx + 1] = color.g;
        self.screen_buffer[4 * idx + 2] = color.b;
        // self.screen_buffer[4 * idx] = 255;
    }

    fn draw_screen(&mut self, world_width: u16, world_height: u16) {
        // Don't try to create a new texture every frame - causes memory leak
        let image = Image {
            bytes: self.screen_buffer.clone(),
            width: world_width,
            height: world_height,
        };

        self.screen_texture.update(&image);

        draw_texture_ex(
            self.screen_texture,
            self.world_pxmin,
            self.world_pymin,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    self.pixels_per_particle * world_width as f32,
                    self.pixels_per_particle * world_height as f32,
                )),
                ..Default::default()
            },
        );
        // build_textures_atlas();
        // tex.delete();
        // unsafe {
        //     // macroquad::window::get_internal_gl().flush();
        //     miniquad::native::gl::glFlush();
        // };
    }

    fn debug_chunk(&self, x: usize, y: usize, width: usize, height: usize, text: &str) {
        let (px, py) = self.xy_to_pixels(x, y);
        let (mut pw, mut ph) = self.xy_to_pixels(width, height);
        pw -= self.world_pxmin;
        ph -= self.world_pymin;
        draw_rectangle_lines(px, py, pw, ph, 1.0, RED);
        draw_text(text, px, py + ph / 2.0, 16.0, WHITE);
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
fn cursor_input(settings: &mut Settings, world: &mut World) {
    let (px, py) = mouse_position();

    if px > settings.painter.world_pxmin
        && px < screen_width()
        && py > settings.painter.world_pymin
        && py < screen_height()
        && !settings.mouse_over_gui
    {
        let (mousex, mousey) = settings
            .painter
            .mouse_location(world.width(), world.height());

        let (brushx_min, brushx_max, brushy_min, brushy_max) = settings.painter.calculate_brush(
            px,
            py,
            settings.brush_size,
            world.width(),
            world.height(),
        );

        // dbg!(&settings.drawing_style);
        match settings.drawing_style {
            //
            DrawingStyle::Line => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    if let Some(xy1) = settings.draw_xy1 {
                        // If we clicked and the first point has already been set,
                        // create particles along the line
                        fill_brush_along_line(settings, world, xy1, (mousex, mousey));
                        settings.draw_xy1 = None;
                    } else {
                        // If we clicked and the first point hasn't been set, set
                        // the first point
                        settings.draw_xy1 = Some((mousex, mousey));
                    }
                }
                if let Some(xy1) = settings.draw_xy1 {
                    // If we haven't clicked and the first point has been set,
                    // highlight along the line
                    // highlight_particle_brush(settings, xy1.0, xy1.1);
                    highlight_brush(
                        settings, world, brushx_min, brushx_max, brushy_min, brushy_max,
                    );
                    iterate_over_line(xy1, (mousex, mousey), |x, y| {
                        let (px, py) = settings.painter.xy_to_pixels(x, y);
                        let (brushx_min, brushx_max, brushy_min, brushy_max) =
                            settings.painter.calculate_brush(
                                px,
                                py,
                                settings.brush_size,
                                world.width(),
                                world.height(),
                            );
                        highlight_brush(
                            settings, world, brushx_min, brushx_max, brushy_min, brushy_max,
                        );
                    })
                } else {
                    // Highlight the particle brush as normal otherwise.
                    highlight_brush(
                        settings, world, brushx_min, brushx_max, brushy_min, brushy_max,
                    );
                }
            }

            DrawingStyle::Brush => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    settings.draw_xy1 = Some((mousex, mousey));
                }

                if !is_mouse_button_down(MouseButton::Left) {
                    settings.draw_xy1 = None;
                }

                if let Some(xy1) = settings.draw_xy1 {
                    fill_brush_along_line(settings, world, xy1, (mousex, mousey));
                    settings.draw_xy1 = Some((mousex, mousey));
                }

                highlight_brush(
                    settings, world, brushx_min, brushx_max, brushy_min, brushy_max,
                );
            }

            DrawingStyle::Portal => {
                let (mut brushx_min, mut brushx_max, mut brushy_min, mut brushy_max) = settings
                    .painter
                    .calculate_brush(px, py, settings.brush_size, world.width(), world.height());

                // Check whether the location/size of the portal we're trying to place is valid
                settings.portal_placement_valid = true;
                match settings.portal_direction {
                    Direction::Up | Direction::Down => {
                        brushy_min = mousey;
                        brushy_max = mousey + 1;
                        for x in brushx_min..brushx_max {
                            if x < 1
                                || x >= world.width() - 1
                                || world.portal_exists_at((x, mousey))
                            {
                                settings.portal_placement_valid = false;
                                break;
                            }
                        }
                    }
                    Direction::Right | Direction::Left => {
                        brushx_min = mousex;
                        brushx_max = mousex + 1;
                        for y in brushy_min..brushy_max {
                            if y < 1
                                || y >= world.width() - 1
                                || world.portal_exists_at((mousex, y))
                            {
                                settings.portal_placement_valid = false;
                                break;
                            }
                        }
                    }
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    fill_brush(
                        settings, world, brushx_min, brushx_max, brushy_min, brushy_max,
                    );
                } else {
                    highlight_brush(
                        settings, world, brushx_min, brushx_max, brushy_min, brushy_max,
                    );
                }
            }
        }
    }
}

fn keys_input(settings: &mut Settings, world: &mut World) {
    // Advance on "A" if paused
    if is_key_pressed(KeyCode::A) && settings.paused {
        println!("advance");
        world.draw_and_refresh(&mut settings.painter, settings.debug_mode);
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

fn apply_fn_in_square<F>(
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
    xbound: usize,
    ybound: usize,
    mut inner_function: F,
) where
    F: FnMut(usize, usize),
{
    // Should this be inclusive?
    for x in xmin..xmax {
        if x >= xbound {
            continue;
        }
        for y in ymin..ymax {
            if y >= ybound {
                continue;
            }
            inner_function(x, y);
        }
    }
}

fn fill_brush_along_line(
    settings: &mut Settings,
    world: &mut World,
    xy1: (usize, usize),
    xy2: (usize, usize),
) {
    iterate_over_line(xy1, xy2, |x, y| {
        let (px, py) = settings.painter.xy_to_pixels(x, y);
        let (brushx_min, brushx_max, brushy_min, brushy_max) = settings.painter.calculate_brush(
            px,
            py,
            settings.brush_size,
            world.width(),
            world.height(),
        );

        fill_brush(
            settings, world, brushx_min, brushx_max, brushy_min, brushy_max,
        );
    });
}

fn fill_brush(
    settings: &mut Settings,
    world: &mut World,
    brushx_min: usize,
    brushx_max: usize,
    brushy_min: usize,
    brushy_max: usize,
) {
    apply_fn_in_square(
        brushx_min,
        brushx_max,
        brushy_min,
        brushy_max,
        world.width(),
        world.height(),
        |x, y| {
            if settings.delete {
                world.delete_source((x, y));
                world.add_new_particle(ParticleType::Empty, (x, y), settings.replace);
            } else {
                create_placeable(settings, world, (x, y));
            }
        },
    );
}

fn highlight_brush(
    settings: &mut Settings,
    world: &mut World,
    brushx_min: usize,
    brushx_max: usize,
    brushy_min: usize,
    brushy_max: usize,
) {
    apply_fn_in_square(
        brushx_min,
        brushx_max,
        brushy_min,
        brushy_max,
        world.width(),
        world.height(),
        |x, y| highlight_selected_placeable(settings, x, y),
    );
}

fn highlight_particle(settings: &Settings, x: usize, y: usize) {
    let mut color: Color = settings.placement_type.properties().base_color.into();
    color.a = 0.4;
    settings.painter.draw_particle(x, y, color);
}

fn highlight_selected_placeable(settings: &Settings, x: usize, y: usize) {
    match settings.placeable_selector {
        PlaceableSelector::Particle => {
            highlight_particle(settings, x, y);
        }
        PlaceableSelector::Source => {
            let mut color: Color = settings.placement_type.properties().base_color.into();
            color.a = 0.4;
            color.r -= 0.1;
            color.g -= 0.1;
            color.b -= 0.1;
            settings
                .painter
                .draw_source(x, y, color, settings.sources_replace, false);
        }
        PlaceableSelector::Sink => {
            let mut color: Color = settings.placement_type.properties().base_color.into();
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

            let mut color: Color = settings.portal_color.into();
            color.a = 0.4;
            settings
                .painter
                .draw_portal(x, y, settings.portal_direction, color);
        }
    }
}

fn create_particle(settings: &Settings, world: &mut World, xy: (usize, usize)) {
    world.add_new_particle(settings.placement_type, xy, settings.replace);
}

fn create_placeable(settings: &mut Settings, world: &mut World, xy: (usize, usize)) {
    match settings.placeable_selector {
        PlaceableSelector::Particle => {
            create_particle(settings, world, xy);
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

            if settings.debug_mode {
                println!(
                    "Before: waiting for partner = {}",
                    settings.waiting_for_partner_portal
                );
            }

            let partner_xy = if settings.waiting_for_partner_portal {
                settings.portal_color = settings.portal_color.add_hsv(0.0, 0.0, 0.02);
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
                settings.portal_color.into(),
            );

            if added {
                if !settings.waiting_for_partner_portal {
                    settings.last_portal_placed.push(xy);
                    settings.portal_color = settings.portal_color.add_hsv(0.0, 0.0, -0.02);
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
    let p = world.get_particle((x, y));
    format!("({}, {}): {:#?}", x, y, p)
}

#[derive(Debug, PartialEq)]
enum DrawingStyle {
    Brush,
    Line,
    Portal,
}

impl DrawingStyle {
    pub fn as_str(&self) -> &str {
        match self {
            DrawingStyle::Brush => "Brush",
            DrawingStyle::Line => "Line",
            DrawingStyle::Portal => "Portal",
        }
    }
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
    if ctx.wants_pointer_input() || ctx.is_pointer_over_area() {
        settings.mouse_over_gui = true;
    } else {
        settings.mouse_over_gui = false;
    }

    egui::Window::new("")
        // .resizable(false)
        .title_bar(false)
        .fixed_size([
            settings.painter.world_pxmin - 13.0,
            settings.painter.world_pymin,
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

            ui.horizontal(|ui| {
                ui.label("Scale: ");
                ui.add(
                    egui::Slider::new(&mut settings.new_pixels_per_particle, 1.0..=30.0)
                        .fixed_decimals(0)
                        .step_by(1.0),
                    // egui::DragValue::new(&mut settings.new_pixels_per_particle)
                    //     .clamp_range(1.0..=30.0)
                    //     .fixed_decimals(0)
                    //     .speed(0.5),
                );
            });
            ui.separator();

            egui::Grid::new("1").num_columns(2).show(ui, |ui| {
                egui::Grid::new("")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        let chunk_size = settings.chunk_size;
                        // ui.group(|ui| {
                        // ui.horizontal(|ui| {
                        ui.label("New X: ");
                        ui.add(
                            egui::Slider::new(
                                &mut settings.new_size.0,
                                chunk_size..=60 * chunk_size,
                            )
                            .fixed_decimals(0)
                            .step_by(chunk_size as f64),
                            // egui::DragValue::new(&mut settings.new_size.0)
                            //     .clamp_range(1..=1000)
                            //     .fixed_decimals(0)
                            //     .speed(5),
                        );
                        // });
                        ui.end_row();
                        // ui.horizontal(|ui| {
                        ui.label("New Y: ");
                        ui.add(
                            egui::Slider::new(
                                &mut settings.new_size.1,
                                chunk_size..=60 * chunk_size,
                            )
                            .fixed_decimals(0)
                            .step_by(chunk_size as f64),
                            // egui::DragValue::new(&mut settings.new_size.1)
                            //     .clamp_range(1..=1000)
                            //     .fixed_decimals(0)
                            //     .speed(5),
                        );
                        // });
                        ui.end_row();

                        // ui.horizontal(|ui| {
                        // });
                        // ui.end_row();
                    });
                ui.end_row();
                if ui.add(egui::Button::new("Reset/Resize")).clicked() {
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
                                .clamp_range(1.0..=100.0)
                                .fixed_decimals(0)
                                .speed(0.2),
                        );
                    });
                    ui.end_row();
                });

            ui.separator();
            ui.horizontal(|ui| {
                if ui
                    .selectable_value(
                        &mut settings.drawing_style,
                        DrawingStyle::Brush,
                        DrawingStyle::Brush.as_str(),
                    )
                    .clicked()
                {
                    settings.draw_xy1 = None;
                    if settings.placeable_selector == PlaceableSelector::Portal {
                        settings.placeable_selector = PlaceableSelector::Particle;
                    }
                };
                if ui
                    .selectable_value(
                        &mut settings.drawing_style,
                        DrawingStyle::Line,
                        DrawingStyle::Line.as_str(),
                    )
                    .clicked()
                {
                    settings.draw_xy1 = None;
                    settings.brush_size = 1.0;
                    if settings.placeable_selector == PlaceableSelector::Portal {
                        settings.placeable_selector = PlaceableSelector::Particle;
                    }
                };
                if ui
                    .selectable_value(
                        &mut settings.drawing_style,
                        DrawingStyle::Portal,
                        DrawingStyle::Portal.as_str(),
                    )
                    .clicked()
                {
                    settings.draw_xy1 = None;
                    settings.placeable_selector = PlaceableSelector::Portal
                };
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.set_enabled(settings.drawing_style != DrawingStyle::Portal);
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
                    particle_selector(ui, ParticleType::Acid, settings);
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
        });
    // if settings.debug_mode {
    egui::Window::new("Debug Info")
        .fixed_pos([settings.painter.world_pxmax, settings.painter.world_pymin])
        .resizable(false)
        .show(ctx, |ui| {
            egui::CollapsingHeader::new("Settings Struct").show(ui, |ui| {
                ui.label(format!("{:#?}", settings));
            });
            egui::CollapsingHeader::new("Particle Under Mouse").show(ui, |ui| {
                // let (mousex, mousey) = settings.painter.mouse_location(grid_width, grid_height)
                ui.label(debug_particle_string(world, &settings.painter));
            });
        });
    // ui.group(|ui| {
    //     // ui.label(debug_particle_string(world, &settings.painter));
    // });
    // }
}

fn particle_selector(ui: &mut egui::Ui, ptype: ParticleType, settings: &mut Settings) {
    // ui.selectable_value(&mut settings.placement_type, ptype, "");
    egui::Frame::none()
        .fill(ptype.properties().base_color.into())
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

impl From<PColor> for Color {
    fn from(value: PColor) -> Self {
        Color::new(
            value.r as f32 / 255.0,
            value.g as f32 / 255.0,
            value.b as f32 / 255.0,
            1.0,
        )
    }
}

impl From<Color> for PColor {
    fn from(value: Color) -> Self {
        if value.a < 1.0 {
            println!(
                "WARNING: Converting Color to PColor ignores alpha of {}",
                value.a
            );
        }
        PColor::new(
            (value.r * 255.0) as u8,
            (value.g * 255.0) as u8,
            (value.b * 255.0) as u8,
        )
    }
}

impl From<PColor> for egui::Color32 {
    fn from(value: PColor) -> Self {
        egui::Color32::from_rgb(value.r, value.g, value.b)
    }
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
