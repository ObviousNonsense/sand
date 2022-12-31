use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Enum)]
#[repr(usize)]
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
    pub movable: bool,     // TODO not used yet
    pub replaceable: bool, // TODO not used yet
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum WaterBoolStateMap {
    MovingRight = 1,
}

// #[derive(Debug)]
#[derive(Debug, Clone)]
pub struct Particle {
    pub particle_type: ParticleType,
    // pub color: Color,
    pub moved: bool,
    pub bool_state: [bool; 2],
}

impl Particle {
    pub fn new(particle_type: ParticleType) -> Self {
        // TODO: modulate individual particle color relative to base_color
        // let color = match particle_type {
        //     ParticleType::Empty => BLACK,
        //     ParticleType::Border => GRAY,
        //     ParticleType::Concrete => GRAY,
        //     ParticleType::Sand => YELLOW,
        //     ParticleType::Water => BLUE,
        // };

        // let bool_state = [false, false];
        let mut bool_state = [false, false];

        match particle_type {
            ParticleType::Water => bool_state[WaterBoolStateMap::MovingRight as usize] = random(),
            _ => {}
        }

        Self {
            particle_type,
            // color,
            moved: false,
            bool_state,
        }
    }

    pub fn draw(&self, x: usize, y: usize, color: Color) {
        // let px = PIXELS_PER_PARTICLE * x as f32;
        // let py = PIXELS_PER_PARTICLE * y as f32;
        let (px, py) = xy_to_pixels(x, y);
        draw_rectangle(px, py, PIXELS_PER_PARTICLE, PIXELS_PER_PARTICLE, color);
    }
}

// ─── Grid Functions ────────────────────────────────────────────────────────────────────────── ✣ ─
pub struct World {
    grid: Vec<Particle>,
    width: usize,
    height: usize,
    base_properties: EnumMap<ParticleType, ParticleTypeProperties>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
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
            base_properties: enum_map! {
                ParticleType::Border => ParticleTypeProperties {
                    base_color: GRAY,
                    movable: false,
                    replaceable: false,
                },
                ParticleType::Concrete => ParticleTypeProperties {
                    base_color: GRAY,
                    movable: false,
                    replaceable: true,
                },
                ParticleType::Empty => ParticleTypeProperties {
                    base_color: Color::new(0.2, 0.2, 0.2, 1.0),
                    movable: true,
                    replaceable: true,
                },
                ParticleType::Sand => ParticleTypeProperties {
                    base_color: YELLOW,
                    movable: true,
                    replaceable: true,
                },
                ParticleType::Water => ParticleTypeProperties {
                    base_color: BLUE,
                    movable: true,
                    replaceable: true,
                },
            },
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn base_properties(&self, particle_type: ParticleType) -> ParticleTypeProperties {
        self.base_properties[particle_type]
    }

    pub fn particle_at(&self, x: usize, y: usize) -> Particle {
        self.grid[self.xy_to_index(x, y)].clone()
    }

    pub fn add_new_particle(&mut self, new_particle_type: ParticleType, x: usize, y: usize) {
        let idx = self.xy_to_index(x, y);
        let old_particle_type = self.grid[idx].particle_type;

        // TODO add toggle for replace/not replace particles (other than empty & borders)
        match (new_particle_type, old_particle_type) {
            (_, ParticleType::Border) => {}
            (ParticleType::Empty, _) | (_, ParticleType::Empty) => {
                self.grid[idx] = Particle::new(new_particle_type);
            }
            _ => {}
        }
    }

    pub fn update_all_particles(&mut self, rng: &mut ThreadRng) {
        let mut idx_range: Vec<usize> = ((self.width + 1)..(self.width * self.height - 2))
            .rev()
            .collect();
        idx_range.shuffle(rng);
        for idx in idx_range.iter() {
            let idx = *idx;
            let (x, y) = self.index_to_xy(idx);
            let particle = self.grid[idx].clone();

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
                        let moving_right_idx = WaterBoolStateMap::MovingRight as usize;
                        let check_directions = if particle.bool_state[moving_right_idx] {
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
                                        self.grid[idx].bool_state[moving_right_idx] =
                                            !self.grid[idx].bool_state[moving_right_idx];
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
        (self.grid[idx1], self.grid[idx2]) = (self.grid[idx2].clone(), self.grid[idx1].clone());
    }

    pub fn draw_and_reset_all_particles(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.xy_to_index(x, y);
                let ptype = self.grid[idx].particle_type;
                self.grid[idx].draw(x, y, self.base_properties[ptype].base_color);
                self.grid[idx].moved = false;
            }
        }
    }

    pub fn xy_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn index_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }
}
// ───────────────────────────────────────────────────────────────────────────────────────────── ✣ ─
