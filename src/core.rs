use super::*;
use ::rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};
use array2d::Array2D;

#[derive(Debug, PartialEq)]
pub enum Placeable {
    Particle,
    Source,
}

#[derive(Debug, Clone, Copy, PartialEq)]
// #[repr(usize)]
pub enum ParticleType {
    Empty,
    Border,
    Sand,
    Water,
    Concrete,
}

#[derive(Debug, Clone, Copy)]
// The immutable properties of a particle type
pub struct ParticleTypeProperties {
    pub base_color: Color,
    pub weight: f32,
    movable: bool,
    fluid: bool,
}

pub fn base_properties(particle_type: ParticleType) -> ParticleTypeProperties {
    match particle_type {
        ParticleType::Border => ParticleTypeProperties {
            base_color: GRAY,
            weight: f32::INFINITY,
            movable: false,
            fluid: false,
        },
        ParticleType::Concrete => ParticleTypeProperties {
            base_color: GRAY,
            weight: f32::INFINITY,
            movable: false,
            fluid: false,
        },
        ParticleType::Empty => ParticleTypeProperties {
            base_color: Color::new(0.2, 0.2, 0.2, 1.0),
            weight: 1.0,
            movable: false,
            fluid: false,
        },
        ParticleType::Sand => ParticleTypeProperties {
            base_color: YELLOW,
            weight: 90.0,
            movable: true,
            fluid: false,
        },
        ParticleType::Water { .. } => ParticleTypeProperties {
            base_color: BLUE,
            weight: 60.0,
            movable: true,
            fluid: true,
        },
    }
}

// #[derive(Debug)]
#[derive(Debug, Clone)]
pub struct Particle {
    pub particle_type: ParticleType,
    // pub color: Color,
    updated: bool,
    moved: Option<bool>,
    moving_right: Option<bool>,
}

impl Particle {
    fn new(particle_type: ParticleType, rng: &mut ThreadRng) -> Self {
        // TODO: modulate individual particle color relative to base_color

        let moved = if base_properties(particle_type).movable {
            Some(false)
        } else {
            None
        };

        let moving_right = if base_properties(particle_type).fluid {
            Some(rng.gen())
        } else {
            None
        };

        Self {
            particle_type,
            // color,
            updated: false,
            moved,
            moving_right,
        }
    }

    fn set_moved(&mut self, val: bool) {
        if base_properties(self.particle_type).movable {
            self.moved = Some(val);
        } else {
            unreachable!("Called set_moved on non-movable particle {:?}", self);
        }
    }

    fn moved(&self) -> Option<bool> {
        self.moved
    }

    fn toggle_moving_right(&mut self) {
        if base_properties(self.particle_type).fluid {
            self.moving_right = Some(!self.moving_right.unwrap());
        } else {
            unreachable!("Called set_moving_right on non-fluid particle {:?}", self);
        }
    }

    fn moving_right(&self) -> Option<bool> {
        self.moving_right
    }

    fn draw(&self, x: usize, y: usize) {
        let (px, py) = xy_to_pixels(x, y);
        let color = base_properties(self.particle_type).base_color;
        draw_rectangle(px, py, PIXELS_PER_PARTICLE, PIXELS_PER_PARTICLE, color);
    }
}

#[derive(Debug, Clone)]
struct ParticleSource {
    particle_type: ParticleType,
    replaces: bool,
}

impl ParticleSource {
    fn draw(&self, x: usize, y: usize) {
        let (px, py) = xy_to_pixels(x, y);
        let mut color = base_properties(self.particle_type).base_color;
        color.a = 0.5;
        color.r -= 0.1;
        color.g -= 0.1;
        color.b -= 0.1;
        draw_rectangle(px, py, PIXELS_PER_PARTICLE, PIXELS_PER_PARTICLE, color);

        if self.particle_type != ParticleType::Empty {
            let hatch_color = if self.replaces {
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
        }
    }
}

// ─── World ─────────────────────────────────────────────────────────────────────────────────── ✣ ─
pub struct World {
    particle_grid: Array2D<Particle>,
    source_grid: Array2D<Option<ParticleSource>>,
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
        if let Some(_) = self.source_grid[xy] {
            if !replace {
                return;
            }
        }

        self.source_grid[xy] = Some(ParticleSource {
            particle_type: source_type,
            replaces: source_replaces,
        })
    }

    pub fn delete_source(&mut self, xy: (usize, usize)) {
        self.source_grid[xy] = None;
    }

    fn try_grid_position(
        &mut self,
        xy1: (usize, usize),
        xy2: (usize, usize),
        try_swap: bool,
    ) -> bool {
        let other_p = self.particle_at(xy2);
        let my_weight = self.weight_at(xy1);
        let other_weight = self.weight_at(xy2);
        if other_p.particle_type == ParticleType::Empty {
            if xy1.1 == xy2.1 || my_weight * self.rng.gen::<f32>() > other_weight {
                self.particle_grid[xy1].set_moved(true);
                self.swap_particles(xy1, xy2);
                return true;
            }
        } else if try_swap && self.movable_at(xy2) && !other_p.updated {
            if my_weight * self.rng.gen::<f32>() > other_weight {
                self.particle_grid[xy1].set_moved(true);
                self.particle_grid[xy2].set_moved(true);
                // self.swap_particles(xy1, xy2);
                self.displace_particle(xy1, xy2);
                return true;
            }
        }
        false
    }

    fn displace_particle(&mut self, xy1: (usize, usize), xy2: (usize, usize)) {
        let positions_to_try = vec![[0, 1], [1, 1], [-1, 1], [1, 0], [-1, 0]];
        let mut moved = false;
        for pos in positions_to_try {
            let xy3 = (
                (xy2.0 as isize + pos[0]) as usize,
                (xy2.1 as isize + pos[1]) as usize,
            );
            moved = self.try_grid_position(xy2, xy3, false);
            if moved {
                self.particle_grid[xy3].set_moved(true);
                break;
            }
        }
        if !moved {
            self.particle_grid[xy2].set_moved(true);
        }
        self.swap_particles(xy1, xy2);
    }

    fn swap_particles(&mut self, xy1: (usize, usize), xy2: (usize, usize)) {
        (self.particle_grid[xy1], self.particle_grid[xy2]) = (
            self.particle_grid[xy2].clone(),
            self.particle_grid[xy1].clone(),
        );
    }

    fn weight_at(&self, xy: (usize, usize)) -> f32 {
        base_properties(self.particle_grid[xy].particle_type).weight
    }

    fn movable_at(&self, xy: (usize, usize)) -> bool {
        base_properties(self.particle_grid[xy].particle_type).movable
    }

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
            let idx = *idx;
            let xy = self.index_to_xy(idx);
            let particle_clone = self.particle_grid[xy].clone();

            if particle_clone.updated {
                continue;
            }
            self.particle_grid[xy].updated = true;

            match particle_clone.particle_type {
                ParticleType::Sand => {
                    self.sand_movement(xy, particle_clone);
                }
                ParticleType::Water => {
                    self.fluid_movement(xy, particle_clone);
                }
                _ => {}
            }
        }
    }

    fn sand_movement(&mut self, xy: (usize, usize), particle_clone: Particle) {
        if particle_clone.moved().unwrap() {
            return;
        }
        let r = self.rng.gen();
        let right: isize = if r { -1 } else { 1 };
        let check_directions = vec![(0, 1), (right, 1), (0 - right, 1)];

        for (dx, dy) in check_directions.iter() {
            let other_xy = ((xy.0 as isize + dx) as usize, (xy.1 as isize + dy) as usize);
            let moved = self.try_grid_position(xy, other_xy, true);
            if moved {
                break;
            }
        }
    }

    fn fluid_movement(&mut self, xy: (usize, usize), particle_clone: Particle) {
        if particle_clone.moved().unwrap() {
            return;
        }
        // let r = self.rng.gen();
        // let right: isize = if r { -1 } else { 1 };

        let check_directions = if particle_clone.moving_right().unwrap() {
            [(0, 1), (1, 1), (-1, 1), (1, 0), (-1, 0)]
        } else {
            [(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)]
        };

        for ((dx, dy), k) in check_directions.iter().zip(0..5) {
            let other_xy = ((xy.0 as isize + dx) as usize, xy.1 + dy);
            let moved = self.try_grid_position(xy, other_xy, true);

            if moved {
                if k == 4 {
                    self.particle_grid[other_xy].toggle_moving_right();
                }
                break;
            }
        }
    }

    pub fn draw_and_reset_all_particles(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let ptype = self.particle_grid[(x, y)].particle_type;
                self.particle_grid[(x, y)].updated = false;
                if base_properties(ptype).movable {
                    self.particle_grid[(x, y)].set_moved(false);
                }

                self.particle_grid[(x, y)].draw(x, y);
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
}
// ───────────────────────────────────────────────────────────────────────────────────────────── ✣ ─
