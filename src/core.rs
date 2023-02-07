use super::*;
use ::rand::{
    distributions::{
        uniform::{SampleRange, SampleUniform},
        Slice,
    },
    prelude::Distribution,
    rngs::ThreadRng,
    seq::SliceRandom,
    thread_rng, Rng,
};
use array2d::Array2D;

// #[derive(Debug, PartialEq)]
// pub enum Placeable {
//     Particle,
//     Source,
// }

#[derive(Debug, Clone, Copy, PartialEq)]
// #[repr(usize)]
pub enum ParticleType {
    Empty,
    Border,
    Sand,
    Water,
    Concrete,
    Steam,
    Fungus,
}

#[derive(Debug, Clone, Copy)]
// The immutable properties of a particle type
pub struct ParticleTypeProperties {
    pub base_color: Color,
    pub weight: f32,
    moves: bool,
    fluid: bool,
    condensates: bool,
}

impl ParticleType {
    pub const fn properties(&self) -> ParticleTypeProperties {
        match self {
            ParticleType::Border => ParticleTypeProperties {
                base_color: GRAY,
                weight: f32::INFINITY,
                moves: false,
                fluid: false,
                condensates: false,
            },
            ParticleType::Concrete => ParticleTypeProperties {
                base_color: GRAY,
                weight: f32::INFINITY,
                moves: false,
                fluid: false,
                condensates: false,
            },
            ParticleType::Empty => ParticleTypeProperties {
                base_color: Color::new(0.2, 0.2, 0.2, 1.0),
                weight: 1.0,
                moves: false,
                fluid: false,
                condensates: false,
            },
            ParticleType::Sand => ParticleTypeProperties {
                base_color: YELLOW,
                weight: 90.0,
                moves: true,
                fluid: false,
                condensates: false,
            },
            ParticleType::Water => ParticleTypeProperties {
                base_color: BLUE,
                weight: 60.0,
                moves: true,
                fluid: true,
                condensates: false,
            },
            ParticleType::Steam => ParticleTypeProperties {
                base_color: Color::new(0.753, 0.824, 0.949, 1.0),
                weight: 0.5,
                moves: true,
                fluid: true,
                condensates: true,
            },
            ParticleType::Fungus => ParticleTypeProperties {
                base_color: Color::new(0.41, 0.58, 0.51, 1.0),
                weight: f32::INFINITY,
                moves: false,
                fluid: false,
                condensates: false,
            },
        }
    }
}

fn scale_hsl_of_color(c: Color, scale_h: f32, scale_s: f32, scale_l: f32) -> Color {
    let color_hsl = rgb_to_hsl(c);
    hsl_to_rgb(
        color_hsl.0 * scale_h,
        color_hsl.1 * scale_s,
        color_hsl.2 * scale_l,
    )
}

#[derive(PartialEq)]
enum Deleted {
    True,
    False,
}

// #[derive(Debug)]
#[derive(Debug, Clone)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub color: Color,
    original_color: Color,
    updated: bool,
    moved: Option<bool>,
    moving_right: Option<bool>,
    condensation_countdown: Option<i16>,
    initial_condensation_countdown: Option<i16>,
    watered: Option<bool>,
}

// General Particle Methods
impl Particle {
    fn new(particle_type: ParticleType, rng: &mut ThreadRng) -> Self {
        // TODO: modulate individual particle color relative to base_color

        let moved = if particle_type.properties().moves {
            Some(false)
        } else {
            None
        };

        let moving_right = if particle_type.properties().fluid {
            Some(rng.gen())
        } else {
            None
        };

        let condensation_countdown = if particle_type.properties().condensates {
            Some(100 + rng.gen_range(-30..30))
        } else {
            None
        };

        let watered = if particle_type == ParticleType::Fungus {
            Some(false)
        } else {
            None
        };

        let color = scale_hsl_of_color(
            particle_type.properties().base_color,
            1.0,
            rng.gen_range(0.95..1.05),
            rng.gen_range(0.98..1.02),
        );

        Self {
            particle_type,
            color,
            original_color: color,
            updated: false,
            moved,
            moving_right,
            condensation_countdown,
            initial_condensation_countdown: condensation_countdown,
            watered,
        }
    }

    fn update(&mut self, mut api: WorldApi) {
        let mut deleted = Deleted::False;

        match self.particle_type {
            ParticleType::Sand => {
                self.movement(&mut api);
            }
            ParticleType::Water => {
                self.movement(&mut api);
            }
            ParticleType::Steam => {
                let lasty = api.xy.1;
                self.movement(&mut api);
                deleted = self.update_condensation(&mut api, lasty);
            }
            ParticleType::Fungus => {
                let dxdy_list = vec![
                    (0, -1),
                    (0, 1),
                    (1, 0),
                    (-1, 0),
                    (-1, -1),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                ];

                let dxdy = dxdy_list[api.random_range(0..dxdy_list.len())];
                let neighbour = api.neighbour_mut(dxdy);
                if self.watered.unwrap() {
                    if neighbour.particle_type == ParticleType::Empty {
                        let mut count = 0;
                        for (ddx, ddy) in dxdy_list {
                            let dxdy2 = (dxdy.0 + ddx, dxdy.1 + ddy);
                            if api.neighbour(dxdy2).particle_type == ParticleType::Fungus {
                                count += 1;
                            }
                        }

                        if count < 3 && api.random() {
                            api.replace_with_new(dxdy, ParticleType::Fungus);
                            self.set_watered(false);
                        }
                    } else if neighbour.particle_type == ParticleType::Fungus {
                        if !neighbour.watered.unwrap() {
                            neighbour.set_watered(true);
                            self.set_watered(false);
                        }
                    }
                } else if neighbour.particle_type == ParticleType::Water {
                    api.replace_with_new(dxdy, ParticleType::Empty);
                    self.set_watered(true);
                }
            }
            _ => {}
        }

        if deleted == Deleted::False {
            self.updated = true;
            api.update_in_world(self.to_owned());
        }
    }

    fn draw(&self, x: usize, y: usize) {
        draw_particle(x, y, self.color);
    }
}

pub fn draw_particle(x: usize, y: usize, color: Color) {
    let (px, py) = xy_to_pixels(x, y);
    draw_rectangle(px, py, PIXELS_PER_PARTICLE, PIXELS_PER_PARTICLE, color);
}

/// Fungus (plant?) methods
impl Particle {
    fn set_watered(&mut self, w: bool) {
        if w {
            self.color = scale_hsl_of_color(self.original_color, 1.1, 1.7, 1.0);
        } else {
            self.color = self.original_color;
        }
        self.watered = Some(w);
    }
}

/// Condensation methods
impl Particle {
    fn update_condensation(&mut self, api: &mut WorldApi, lasty: usize) -> Deleted {
        if api.xy.1 == lasty {
            if let Some(count) = self.condensation_countdown.as_mut() {
                *count -= 1;
                if *count <= 0 {
                    api.replace_with_new((0, 0), ParticleType::Water);
                    return Deleted::True;
                }
            }
        } else {
            self.condensation_countdown = self.initial_condensation_countdown.clone();
        }
        Deleted::False
    }
}

/// Movement Methods
impl Particle {
    fn movement(&mut self, api: &mut WorldApi) {
        if self.moved.unwrap() {
            return;
        }

        if self.particle_type.properties().fluid {
            self.fluid_movement(api);
        } else {
            self.sand_movement(api);
        }
    }

    fn sand_movement(&mut self, api: &mut WorldApi) {
        let r = api.random::<bool>();
        let right: isize = if r { -1 } else { 1 };
        let check_directions = vec![(0, 1), (right, 1), (0 - right, 1)];

        for dxdy in check_directions.into_iter() {
            if self.try_moving_to(dxdy, api) {
                break;
            }
        }
    }

    fn fluid_movement(&mut self, api: &mut WorldApi) {
        let (check_directions, last_dir) = if self.moving_right.unwrap() {
            ([(0, 1), (1, 1), (-1, 1), (1, 0), (-1, 0)], (-1, 0))
        } else {
            ([(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)], (1, 0))
        };

        let mut last_dxdy = (0, 0);

        for dxdy in check_directions.into_iter() {
            let dxdy_new = if self.rises() {
                (dxdy.0, -dxdy.1)
            } else {
                dxdy
            };
            if self.try_moving_to(dxdy_new, api) {
                last_dxdy = dxdy;
                break;
            }
        }

        if self.moved.unwrap() {
            if last_dxdy == (-1, 1) {
                self.moving_right = Some(false)
            }
            if last_dxdy == (1, 1) {
                self.moving_right = Some(true)
            } else if last_dxdy == last_dir {
                self.moving_right = Some(!self.moving_right.unwrap());
            }
        }
    }

    fn rises(&self) -> bool {
        self.particle_type.properties().weight < ParticleType::Empty.properties().weight
    }

    /// Checks if this particle can and will move in the given direction.
    /// Assumes that if it can move there it will (sets self.moved to true)
    fn try_moving_to(&mut self, dxdy: (isize, isize), api: &mut WorldApi) -> bool {
        let rand_factor = api.random::<f32>();
        let mut other_p = api.neighbour_mut(dxdy);
        let my_weight = self.particle_type.properties().weight;
        let other_weight = other_p.particle_type.properties().weight;

        let weight_check = (!self.rises() && (my_weight * rand_factor > other_weight))
            || (self.rises() && (other_weight * rand_factor > my_weight));

        let other_empty = other_p.particle_type == ParticleType::Empty;

        // If the other position is empty, try moving into it
        if other_empty {
            // If we're moving sideways don't compare weights, just do it
            if dxdy.1 == 0 || weight_check {
                self.moved = Some(true);
                api.swap_with(dxdy);
                return true;
            }
        } else if other_p.particle_type.properties().moves && !other_p.moved.unwrap() {
            // If there's something there and it's moveable and it hasn't
            // already moved, then we might swap with it
            if weight_check {
                other_p.moved = Some(true);
                self.moved = Some(true);
                api.swap_with(dxdy);
                return true;
            }
        }
        false
    }

    fn set_moved(&mut self, val: bool) {
        if self.particle_type.properties().moves {
            self.moved = Some(val);
        } else {
            unreachable!("Called set_moved on non-movable particle {:?}", self);
        }
    }
}

/* #region  */
#[derive(Debug, Clone)]
struct ParticleSource {
    particle_type: ParticleType,
    replaces: bool,
}

impl ParticleSource {
    fn draw(&self, x: usize, y: usize) {
        let mut color = self.particle_type.properties().base_color;
        color.a = 0.5;
        color.r -= 0.1;
        color.g -= 0.1;
        color.b -= 0.1;

        draw_source(
            x,
            y,
            color,
            self.replaces,
            self.particle_type == ParticleType::Empty,
        );
    }
}

pub fn draw_source(x: usize, y: usize, color: Color, replaces: bool, sink: bool) {
    let (px, py) = xy_to_pixels(x, y);
    draw_rectangle(px, py, PIXELS_PER_PARTICLE, PIXELS_PER_PARTICLE, color);

    // if !empty {
    let hatch_color = if replaces || sink {
        Color::new(0.0, 0.0, 0.0, 0.2)
    } else {
        Color::new(1.0, 1.0, 1.0, 0.5)
    };

    draw_line(
        px,
        py,
        px + PIXELS_PER_PARTICLE,
        py + PIXELS_PER_PARTICLE,
        1.0,
        hatch_color,
    );
    // }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn dxdy(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Portal {
    partner_xy: Option<(usize, usize)>,
    // if you're standing where the portal is, which direction do you go to walk through it
    direction: Direction,
    color: Color,
}

impl Portal {
    fn draw(&self, x: usize, y: usize) {
        draw_portal(x, y, self.direction, self.color);
    }
}

pub fn draw_portal(x: usize, y: usize, direction: Direction, color: Color) {
    let (px, py) = xy_to_pixels(x, y);
    // draw_line()
    let pix_per = PIXELS_PER_PARTICLE;
    let thickness = pix_per / 4.0;
    let (ptx, pty, w, h): (f32, f32, f32, f32) = match direction {
        Direction::Up => (px, py, pix_per, thickness),
        Direction::Right => (px + pix_per - thickness, py, thickness, pix_per),
        Direction::Down => (px, py + pix_per - thickness, pix_per, thickness),
        Direction::Left => (px, py, thickness, pix_per),
    };

    draw_rectangle(ptx, pty, w, h, color);
}

/* #endregion */

struct WorldApi<'a> {
    world: &'a mut World,
    xy: (usize, usize),
}

impl<'a> WorldApi<'a> {
    fn random<T>(&mut self) -> T
    where
        ::rand::distributions::Standard: Distribution<T>,
    {
        self.world.rng.gen::<T>()
    }

    fn random_range<T, R>(&mut self, slice: R) -> T
    where
        T: ::rand::distributions::uniform::SampleUniform,
        R: SampleRange<T>,
    {
        self.world.rng.gen_range::<T, R>(slice)
    }

    fn neighbour(&self, dxdy: (isize, isize)) -> &Particle {
        self.world.relative_particle(self.xy, dxdy)
    }

    fn neighbour_mut(&mut self, dxdy: (isize, isize)) -> &mut Particle {
        self.world.relative_particle_mut(self.xy, dxdy)
    }

    /// Swaps the particle with the one dxdy away.
    /// Do not attempt to mutate the particle after calling this.
    // fn swap_with(&mut self, dxdy: (isize, isize), particle: Particle) {
    fn swap_with(&mut self, dxdy: (isize, isize)) {
        let other_xy = self.world.relative_xy(self.xy, dxdy);
        let other_p = self.world.particle_grid[other_xy].clone();
        self.world.particle_grid[self.xy] = other_p;
        // self.world.particle_grid[other_xy] = particle;
        self.xy = other_xy;
    }

    fn replace_with_new(&mut self, dxdy: (isize, isize), particle_type: ParticleType) {
        let xy = self.world.relative_xy(self.xy, dxdy);
        self.world.add_new_particle(particle_type, xy, true);
    }

    fn update_in_world(&mut self, particle: Particle) {
        self.world.particle_grid[self.xy] = particle;
    }
}

// ─── World ─────────────────────────────────────────────────────────────────────────────────── ✣ ─
pub struct World {
    particle_grid: Array2D<Particle>,
    source_grid: Array2D<Option<ParticleSource>>,
    portal_grid: Array2D<Option<Portal>>,
    width: usize,
    height: usize,
    rng: ThreadRng,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = thread_rng();

        let mut particle_grid =
            Array2D::filled_with(Particle::new(ParticleType::Empty, &mut rng), width, height);
        let source_grid = Array2D::filled_with(None, width, height);
        let portal_grid = Array2D::filled_with(None, width, height);

        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    // println!("x: {:?}, y: {:?}", x, y);
                    particle_grid[(x, y)] = Particle::new(ParticleType::Border, &mut rng);
                }
            }
        }

        Self {
            particle_grid,
            source_grid,
            portal_grid,
            width,
            height,
            rng,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn particle_at(&self, xy: (usize, usize)) -> &Particle {
        &self.particle_grid[xy]
    }

    // ─── Update Methods ──────────────────────────────────────────────────────────────────
    pub fn update_all(&mut self) {
        self.update_all_sources();
        self.update_all_particles();
    }

    fn update_all_sources(&mut self) {
        for x in 1..self.width {
            for y in 1..self.height {
                let xy = (x, y);
                if let Some(source) = self.source_grid[xy].clone() {
                    if self.rng.gen() {
                        self.add_new_particle(source.particle_type, xy, source.replaces);
                    }
                }
            }
        }
    }

    fn update_all_particles(&mut self) {
        // TODO: Consider pre-generating this and storing it (either pass it
        // into the function or store it in the struct and clone it here)
        let mut idx_range: Vec<usize> = ((self.width + 1)..(self.width * self.height - 2))
            .rev()
            .collect();
        idx_range.shuffle(&mut self.rng);
        for idx in idx_range.iter() {
            // let idx = *idx;
            let xy = self.index_to_xy(*idx);

            let mut particle_clone = self.particle_grid[xy].clone();

            if particle_clone.updated {
                continue;
            }

            particle_clone.update(WorldApi { world: self, xy });
        }
    }

    // ─── Creation Methods ────────────────────────────────────────────────────────────────
    pub fn add_new_particle(
        &mut self,
        new_particle_type: ParticleType,
        xy: (usize, usize),
        replace: bool,
    ) {
        let old_particle_type = self.particle_grid[xy].particle_type;

        match (new_particle_type, old_particle_type) {
            (_, ParticleType::Border) => {}
            (ParticleType::Empty, _) | (_, ParticleType::Empty) => {
                self.particle_grid[xy] = Particle::new(new_particle_type, &mut self.rng);
            }
            _ => {
                if replace {
                    self.particle_grid[xy] = Particle::new(new_particle_type, &mut self.rng);
                }
            }
        }
    }

    pub fn add_new_source(
        &mut self,
        source_type: ParticleType,
        xy: (usize, usize),
        source_replaces: bool,
        replace: bool,
    ) {
        if self.source_grid[xy].is_some() && !replace {
            return;
        };

        self.source_grid[xy] = Some(ParticleSource {
            particle_type: source_type,
            replaces: source_replaces,
        })
    }

    pub fn add_new_portal(
        &mut self,
        xy: (usize, usize),
        partner_xy: Option<(usize, usize)>,
        direction: Direction,
        color: Color,
    ) -> bool {
        if self.portal_exists_at(xy) {
            return false;
        }

        if let Some(partner_xy) = partner_xy {
            if let Some(ref mut partner) = self.portal_grid[partner_xy] {
                partner.partner_xy = Some(xy);
            } else {
                unreachable!("New portal purported partner does not exist")
            }
        }

        self.portal_grid[xy] = Some(Portal {
            partner_xy,
            direction,
            color,
        });
        true
    }

    pub fn portal_exists_at(&self, xy: (usize, usize)) -> bool {
        if self.portal_grid[xy].is_some() {
            return true;
        }
        false
    }

    // ─── Deletion Methods ────────────────────────────────────────────────────────────────
    pub fn delete_source(&mut self, xy: (usize, usize)) {
        self.source_grid[xy] = None;
    }

    // ─── Other ───────────────────────────────────────────────────────────────────────────
    pub fn draw_and_reset_all_particles(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let ptype = self.particle_grid[(x, y)].particle_type;
                self.particle_grid[(x, y)].updated = false;
                if ptype.properties().moves {
                    self.particle_grid[(x, y)].set_moved(false);
                }

                self.particle_grid[(x, y)].draw(x, y);
                if let Some(portal) = &self.portal_grid[(x, y)] {
                    portal.draw(x, y);
                }
                if let Some(source) = &self.source_grid[(x, y)] {
                    source.draw(x, y);
                }
            }
        }
    }

    // TODO: Consider pre-calculating this and storing it as a vector
    fn index_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    fn relative_particle(&self, xy: (usize, usize), dxdy: (isize, isize)) -> &Particle {
        &self.particle_grid[self.relative_xy(xy, dxdy)]
    }

    fn relative_particle_mut(&mut self, xy: (usize, usize), dxdy: (isize, isize)) -> &mut Particle {
        let (new_x, new_y) = self.relative_xy(xy, dxdy);
        self.particle_grid.get_mut(new_x, new_y).unwrap()
    }

    fn relative_xy(&self, xy: (usize, usize), dxdy: (isize, isize)) -> (usize, usize) {
        // dbg!(xy, dxdy);
        match &self.portal_grid[xy] {
            Some(portal) => {
                if let Some(xy2) = portal.partner_xy {
                    let portal_dxdy = portal.direction.dxdy();
                    // dbg!(xy2, portal_dxdy);
                    if portal_dxdy.0 == dxdy.0 && portal_dxdy.1 == dxdy.1 {
                        return xy2;
                    }
                }
            }
            None => {}
        };
        (
            (xy.0 as isize + dxdy.0) as usize,
            (xy.1 as isize + dxdy.1) as usize,
        )
    }
}
// ───────────────────────────────────────────────────────────────────────────────────────────── ✣ ─
