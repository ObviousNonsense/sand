use super::*;
use ::rand::{random, rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};
use array2d::Array2D;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(usize)]
pub enum ParticleType {
    Empty,
    Border,
    Sand,
    Water,
    Concrete,
}

pub fn base_properties(particle_type: ParticleType) -> ParticleTypeProperties {
    match particle_type {
        ParticleType::Border => ParticleTypeProperties {
            base_color: GRAY,
            weight: f32::INFINITY,
            movable: false,
            fluid: false,
            // replaceable: false,
        },
        ParticleType::Concrete => ParticleTypeProperties {
            base_color: GRAY,
            weight: f32::INFINITY,
            movable: false,
            fluid: false,
            // replaceable: true,
        },
        ParticleType::Empty => ParticleTypeProperties {
            base_color: Color::new(0.2, 0.2, 0.2, 1.0),
            weight: 0.0,
            movable: false,
            fluid: false,
            // replaceable: true,
        },
        ParticleType::Sand => ParticleTypeProperties {
            base_color: YELLOW,
            weight: 90.0,
            movable: true,
            fluid: false,
            // replaceable: true,
        },
        ParticleType::Water { .. } => ParticleTypeProperties {
            base_color: BLUE,
            weight: 60.0,
            movable: true,
            fluid: true,
            // replaceable: true,
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
    fn new(particle_type: ParticleType) -> Self {
        // TODO: modulate individual particle color relative to base_color

        let moved = if base_properties(particle_type).movable {
            Some(false)
        } else {
            None
        };

        let moving_right = if base_properties(particle_type).fluid {
            Some(random())
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

    fn draw(&self, x: usize, y: usize, color: Color) {
        let (px, py) = xy_to_pixels(x, y);
        draw_rectangle(px, py, PIXELS_PER_PARTICLE, PIXELS_PER_PARTICLE, color);
    }
}

#[derive(Debug, Clone, Copy)]
// The immutable properties of a particle type
pub struct ParticleTypeProperties {
    pub base_color: Color,
    pub weight: f32,
    movable: bool,
    fluid: bool,
}

// ─── World ─────────────────────────────────────────────────────────────────────────────────── ✣ ─
pub struct World {
    grid: Array2D<Particle>,
    width: usize,
    height: usize,
    rng: ThreadRng,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Array2D::filled_with(Particle::new(ParticleType::Empty), width, height);

        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    // println!("x: {:?}, y: {:?}", x, y);
                    grid[(x, y)] = Particle::new(ParticleType::Border);
                }
            }
        }

        Self {
            grid,
            width,
            height,
            rng: thread_rng(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn particle_at(&self, x: usize, y: usize) -> &Particle {
        &self.grid[(x, y)]
    }

    pub fn add_new_particle(&mut self, new_particle_type: ParticleType, x: usize, y: usize) {
        let old_particle_type = self.grid[(x, y)].particle_type;

        // TODO add toggle for replace/not replace particles (other than empty & borders)
        match (new_particle_type, old_particle_type) {
            (_, ParticleType::Border) => {}
            (ParticleType::Empty, _) | (_, ParticleType::Empty) => {
                self.grid[(x, y)] = Particle::new(new_particle_type);
            }
            _ => {}
        }
    }

    fn try_grid_position(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        try_swap: bool,
    ) -> bool {
        let other_p = self.particle_at(x2, y2);
        let my_weight = self.weight_at(x1, y1);
        let other_weight = self.weight_at(x2, y2);
        if other_p.particle_type == ParticleType::Empty {
            if my_weight * self.rng.gen::<f32>() > other_weight {
                self.grid[(x1, y1)].set_moved(true);
                self.swap_particles(x1, y1, x2, y2);
                return true;
            }
        } else if try_swap && self.movable_at(x2, y2) && !other_p.updated {
            if my_weight * self.rng.gen::<f32>() > other_weight {
                self.grid[(x1, y1)].set_moved(true);
                self.grid[(x2, y2)].set_moved(true);
                // self.swap_particles(x1, y1, x2, y2);
                self.displace_particle(x1, y1, x2, y2);
                return true;
            }
        }
        false
    }

    fn displace_particle(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let positions_to_try = vec![[0, 1], [1, 1], [-1, 1], [1, 0], [-1, 0]];
        let mut moved = false;
        for pos in positions_to_try {
            let x3 = (x2 as isize + pos[0]) as usize;
            let y3 = (y2 as isize + pos[1]) as usize;
            moved = self.try_grid_position(x2, y2, x3, y3, false);
            if moved {
                self.grid[(x3, y3)].set_moved(true);
                break;
            }
        }
        if !moved {
            self.grid[(x2, y2)].set_moved(true);
        }
        self.swap_particles(x1, y1, x2, y2);
    }

    fn swap_particles(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        (self.grid[(x1, y1)], self.grid[(x2, y2)]) =
            (self.grid[(x2, y2)].clone(), self.grid[(x1, y1)].clone());
    }

    fn weight_at(&self, x: usize, y: usize) -> f32 {
        base_properties(self.grid[(x, y)].particle_type).weight
    }

    fn movable_at(&self, x: usize, y: usize) -> bool {
        base_properties(self.grid[(x, y)].particle_type).movable
    }

    pub fn update_all_particles(&mut self) {
        // TODO: Consider pre-generating this and storing it (either pass it
        // into the function or store it in the struct and clone it here)
        let mut idx_range: Vec<usize> = ((self.width + 1)..(self.width * self.height - 2))
            .rev()
            .collect();
        idx_range.shuffle(&mut self.rng);
        for idx in idx_range.iter() {
            let idx = *idx;
            let (x, y) = self.index_to_xy(idx);
            let particle = self.grid[(x, y)].clone();

            if particle.updated {
                continue;
            }

            self.grid[(x, y)].updated = true;
            match particle.particle_type {
                ParticleType::Sand => {
                    if particle.moved().unwrap() {
                        continue;
                    }
                    let r = self.rng.gen();
                    let right: isize = if r { -1 } else { 1 };
                    let check_directions = vec![(0, 1), (right, 1), (0 - right, 1)];

                    for (dx, dy) in check_directions.iter() {
                        let (other_x, other_y) =
                            ((x as isize + dx) as usize, (y as isize + dy) as usize);
                        self.try_grid_position(x, y, other_x, other_y, true);
                    }
                }
                ParticleType::Water => {
                    if particle.moved().unwrap() {
                        continue;
                    }
                    let r = self.rng.gen();
                    let right: isize = if r { -1 } else { 1 };

                    let check_directions = if particle.moving_right().unwrap() {
                        [(0, 1), (right, 1), (0 - right, 1), (1, 0), (-1, 0)]
                    } else {
                        [(0, 1), (right, 1), (0 - right, 1), (-1, 0), (1, 0)]
                    };

                    for ((dx, dy), k) in check_directions.iter().zip(0..5) {
                        let (other_x, other_y) = ((x as isize + dx) as usize, y + dy);
                        let moved = self.try_grid_position(x, y, other_x, other_y, true);

                        if moved && k == 4 {
                            self.grid[(other_x, other_y)].toggle_moving_right();
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn draw_and_reset_all_particles(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let ptype = self.grid[(x, y)].particle_type;
                self.grid[(x, y)].draw(x, y, base_properties(ptype).base_color);
                self.grid[(x, y)].updated = false;
                if base_properties(ptype).movable {
                    self.grid[(x, y)].set_moved(false);
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
