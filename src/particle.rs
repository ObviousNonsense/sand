use super::*;
use ::rand::{rngs::ThreadRng, Rng};

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
    Flame,
    Methane,
    Gunpowder,
    Oil,
    Wood,
}

#[derive(Debug, Clone, Copy)]
// The immutable properties of a particle type
pub struct ParticleTypeProperties {
    pub base_color: Color,
    pub weight: f32,
    pub moves: bool,
    pub fluid: bool,
    pub condensates: bool,
    pub flammability: f32,
    pub base_fuel: Option<i16>,
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
                flammability: 0.0,
                base_fuel: None,
            },
            ParticleType::Concrete => ParticleTypeProperties {
                base_color: GRAY,
                weight: f32::INFINITY,
                moves: false,
                fluid: false,
                condensates: false,
                flammability: 0.0,
                base_fuel: None,
            },
            ParticleType::Empty => ParticleTypeProperties {
                base_color: Color::new(0.2, 0.2, 0.2, 1.0),
                weight: 1.0,
                moves: false,
                fluid: false,
                condensates: false,
                flammability: 0.0,
                base_fuel: None,
            },
            ParticleType::Sand => ParticleTypeProperties {
                base_color: YELLOW,
                weight: 90.0,
                moves: true,
                fluid: false,
                condensates: false,
                flammability: 0.0,
                base_fuel: None,
            },
            ParticleType::Water => ParticleTypeProperties {
                base_color: BLUE,
                weight: 60.0,
                moves: true,
                fluid: true,
                condensates: false,
                flammability: 0.0,
                base_fuel: None,
            },
            ParticleType::Steam => ParticleTypeProperties {
                base_color: Color::new(0.753, 0.824, 0.949, 1.0),
                weight: 0.5,
                moves: true,
                fluid: true,
                condensates: true,
                flammability: 0.0,
                base_fuel: None,
            },
            // TODO: Make two types: WateredFungus, Dry Fungus
            ParticleType::Fungus => ParticleTypeProperties {
                base_color: Color::new(0.41, 0.58, 0.51, 1.0),
                weight: f32::INFINITY,
                moves: false,
                fluid: false,
                condensates: false,
                flammability: 0.1,
                base_fuel: Some(35),
            },
            ParticleType::Flame => ParticleTypeProperties {
                base_color: Color::new(1.0, 0.47, 0.0, 1.0),
                weight: f32::INFINITY,
                moves: false,
                fluid: false,
                condensates: false,
                flammability: 0.0,
                base_fuel: Some(0),
            },
            ParticleType::Methane => ParticleTypeProperties {
                base_color: Color::new(0.58, 0.47, 0.66, 1.0),
                weight: 0.2,
                moves: true,
                fluid: true,
                condensates: false,
                flammability: 0.95,
                base_fuel: Some(6),
            },
            ParticleType::Gunpowder => ParticleTypeProperties {
                base_color: BLACK,
                weight: 90.0,
                moves: true,
                fluid: false,
                condensates: false,
                flammability: 0.7,
                base_fuel: Some(25),
            },
            ParticleType::Oil => ParticleTypeProperties {
                base_color: Color::new(0.44, 0.34, 0.18, 1.0),
                weight: 50.0,
                moves: true,
                fluid: true,
                condensates: false,
                flammability: 0.95,
                base_fuel: Some(15),
            },
            ParticleType::Wood => ParticleTypeProperties {
                base_color: Color::new(0.3, 0.22, 0.17, 1.0),
                weight: f32::INFINITY,
                moves: false,
                fluid: false,
                condensates: false,
                flammability: 0.1,
                base_fuel: Some(200),
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
    pub updated: bool,
    pub color: Color,
    original_color: Color,
    burning: bool,
    moved: Option<bool>,
    moving_right: Option<bool>,
    condensation_countdown: Option<i16>,
    initial_condensation_countdown: Option<i16>,
    watered: Option<bool>,
    fresh: Option<bool>,
    fuel: Option<i16>,
}

// General Particle Methods
impl Particle {
    pub fn new(particle_type: ParticleType, rng: &mut ThreadRng) -> Self {
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

        let (burning, fresh) = if particle_type == ParticleType::Flame {
            (true, Some(true))
        } else {
            (false, None)
        };

        let fuel = particle_type.properties().base_fuel;

        let color = if particle_type == ParticleType::Empty {
            particle_type.properties().base_color
        } else {
            scale_hsl_of_color(
                particle_type.properties().base_color,
                1.0,
                rng.gen_range(0.95..1.05),
                rng.gen_range(0.98..1.02),
            )
        };

        Self {
            particle_type,
            updated: false,
            color,
            original_color: color,
            burning,
            moved,
            moving_right,
            condensation_countdown,
            initial_condensation_countdown: condensation_countdown,
            watered,
            fresh,
            fuel,
        }
    }

    pub fn update(&mut self, mut api: WorldApi) {
        let mut deleted = Deleted::False;

        if self.particle_type.properties().moves && !self.particle_type.properties().condensates {
            self.movement(&mut api);
        }

        match self.particle_type {
            ParticleType::Steam => {
                let lasty = api.xy().1;
                self.movement(&mut api);
                deleted = self.update_condensation(&mut api, lasty);
            }
            ParticleType::Fungus => {
                self.grow_fungus(&mut api);
            }
            ParticleType::Flame => {
                if self.fresh.unwrap() {
                    self.fresh = Some(false);
                };

                if !self.burning {
                    deleted = Deleted::True;
                    api.replace_with_new((0, 0), ParticleType::Empty);
                }
            }
            _ => {}
        }

        if self.burning && deleted == Deleted::False {
            deleted = self.burn(&mut api);
        }

        if deleted == Deleted::False {
            self.updated = true;
            api.update_in_world(self.to_owned());
        }
    }

    pub fn refresh(&mut self) {
        self.updated = false;
        if self.particle_type.properties().moves {
            self.moved = Some(false);
        }
    }

    // pub fn set_moved(&mut self, val: bool) {
    //     if self.particle_type.properties().moves {
    //         self.moved = Some(val);
    //     } else {
    //         unreachable!("Called set_moved on non-movable particle {:?}", self);
    //     }
    // }

    pub fn draw(&self, x: usize, y: usize, painter: &Painter) {
        painter.draw_particle(x, y, self.color);
    }

    pub fn draw_and_refresh(&mut self, x: usize, y: usize, painter: &Painter) {
        self.refresh();
        self.draw(x, y, painter);
    }
}

// pub fn draw_particle(x: usize, y: usize, color: Color) {
//     let (px, py) = xy_to_pixels(x, y);
//     draw_rectangle(px, py, PIXELS_PER_PARTICLE, PIXELS_PER_PARTICLE, color);
// }

impl Particle {
    fn set_burning(&mut self, b: bool) {
        self.burning = b;
        if !b {
            self.color = self.original_color;
        }
    }

    fn burn(&mut self, api: &mut WorldApi) -> Deleted {
        self.color = Particle::burning_flicker_color(api);

        let dxdy_list = vec![(0, -1), (1, 0), (-1, 0), (0, 1)];

        for dxdy in dxdy_list.into_iter() {
            let r = api.random();
            let neighbour = api.neighbour_mut(dxdy);
            let neighbour_flammability = neighbour.particle_type.properties().flammability;
            if neighbour_flammability > 0.0 && !neighbour.burning {
                if neighbour_flammability * (1.0 - 0.5 * dxdy.1 as f32) > r {
                    neighbour.set_burning(true);
                }
            } else if neighbour.particle_type == ParticleType::Empty && self.fuel.unwrap() > 0 {
                if dxdy.1 < 1 && api.neighbour((-1, 0)).burning && api.neighbour((1, 0)).burning {
                    let mut new_flame = api.new_particle(ParticleType::Flame);
                    new_flame.fuel = Some(api.random_range(0..self.fuel.unwrap()));
                    api.replace_with(dxdy, new_flame);
                }
            } else if neighbour.particle_type == ParticleType::Water {
                api.replace_with_new(dxdy, ParticleType::Steam);
                self.set_burning(false);
                break;
            }
        }

        if let Some(fuel) = self.fuel.as_mut() {
            *fuel -= 1;
            if *fuel < 0 {
                api.replace_with_new((0, 0), ParticleType::Empty);
                return Deleted::True;
            }
        }
        Deleted::False
    }

    fn burning_flicker_color(api: &mut WorldApi) -> Color {
        scale_hsl_of_color(
            ParticleType::Flame.properties().base_color,
            api.random_range(0.95..1.05),
            api.random_range(0.95..1.05),
            api.random_range(0.95..1.05),
        )
    }
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

    fn grow_fungus(&mut self, api: &mut WorldApi) {
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

                if api.random_range(0..4) > count {
                    // if count < 3 && api.random() {
                    api.replace_with_new(dxdy, ParticleType::Fungus);
                    self.set_watered(false);
                }
            } else if neighbour.particle_type == ParticleType::Fungus && !neighbour.watered.unwrap()
            {
                neighbour.set_watered(true);
                self.set_watered(false);
            }
        } else if neighbour.particle_type == ParticleType::Water {
            api.replace_with_new(dxdy, ParticleType::Empty);
            self.set_watered(true);
        }
    }
}

/// Condensation methods
impl Particle {
    fn update_condensation(&mut self, api: &mut WorldApi, lasty: usize) -> Deleted {
        if api.xy().1 == lasty {
            if let Some(count) = self.condensation_countdown.as_mut() {
                *count -= 1;
                if *count <= 0 {
                    api.replace_with_new((0, 0), ParticleType::Water);
                    return Deleted::True;
                }
            }
        } else {
            self.condensation_countdown = self.initial_condensation_countdown;
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
}
