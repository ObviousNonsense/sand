use super::*;
use ::rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Clone, Copy)]
// The immutable properties of a particle type
pub struct ParticleTypeProperties {
    pub label: &'static str,
    pub base_color: Color,
    pub weight: f32,
    pub moves: bool,
    pub auto_move: bool,
    pub fluid: bool,
    pub condensates: bool,
    pub flammability: f32,
    pub wet_flammability: Option<f32>,
    pub base_fuel: Option<i16>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(usize)]
pub enum ParticleType {
    Border = 0,
    Concrete = 1,
    Empty = 2,
    Sand = 3,
    Water = 4,
    Steam = 5,
    Fungus = 6,
    Flame = 7,
    Methane = 8,
    Gunpowder = 9,
    Oil = 10,
    Wood = 11,
    Acid = 12,
}

const PROPERTIES: [ParticleTypeProperties; 13] = [
    // Border = 0
    ParticleTypeProperties {
        label: "Border",
        base_color: GRAY,
        weight: f32::INFINITY,
        moves: false,
        auto_move: false,
        fluid: false,
        condensates: false,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
    },
    // Concrete = 1
    ParticleTypeProperties {
        label: "Concrete",
        base_color: GRAY,
        weight: f32::INFINITY,
        moves: false,
        auto_move: false,
        fluid: false,
        condensates: false,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
    },
    // Empty = 2
    ParticleTypeProperties {
        label: "Empty",
        base_color: Color::new(0.2, 0.2, 0.2, 1.0),
        weight: 1.0,
        moves: false,
        auto_move: false,
        fluid: false,
        condensates: false,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
    },
    // Sand = 3
    ParticleTypeProperties {
        label: "Sand",
        base_color: YELLOW,
        weight: 90.0,
        moves: true,
        auto_move: true,
        fluid: false,
        condensates: false,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
    },
    // Water = 4
    ParticleTypeProperties {
        label: "Water",
        base_color: BLUE,
        weight: 60.0,
        moves: true,
        auto_move: true,
        fluid: true,
        condensates: false,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
    },
    // Steam = 5
    ParticleTypeProperties {
        label: "Steam",
        base_color: Color::new(0.753, 0.824, 0.949, 1.0),
        weight: 0.5,
        moves: true,
        auto_move: false,
        fluid: true,
        condensates: true,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
    },
    // Fungus = 6
    ParticleTypeProperties {
        label: "Fungus",
        base_color: Color::new(0.41, 0.58, 0.51, 1.0),
        weight: f32::INFINITY,
        moves: false,
        auto_move: false,
        fluid: false,
        condensates: false,
        flammability: 0.125,
        wet_flammability: Some(0.015),
        base_fuel: Some(35),
    },
    // Flame = 7
    ParticleTypeProperties {
        label: "Flame",
        base_color: Color::new(1.0, 0.47, 0.0, 1.0),
        weight: f32::INFINITY,
        moves: false,
        auto_move: false,
        fluid: false,
        condensates: false,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: Some(0),
    },
    // Methane = 8
    ParticleTypeProperties {
        label: "Methane",
        base_color: Color::new(0.58, 0.47, 0.66, 1.0),
        weight: 0.2,
        moves: true,
        auto_move: true,
        fluid: true,
        condensates: false,
        flammability: 0.95,
        wet_flammability: None,
        base_fuel: Some(6),
    },
    // Gunpowder = 9
    ParticleTypeProperties {
        label: "Gunpowder",
        base_color: BLACK,
        weight: 90.0,
        moves: true,
        auto_move: true,
        fluid: false,
        condensates: false,
        flammability: 0.6,
        wet_flammability: None,
        base_fuel: Some(35),
    },
    // Oil = 10
    ParticleTypeProperties {
        label: "Oil",
        base_color: Color::new(0.44, 0.34, 0.18, 1.0),
        weight: 50.0,
        moves: true,
        auto_move: true,
        fluid: true,
        condensates: false,
        flammability: 0.9,
        wet_flammability: None,
        base_fuel: Some(25),
    },
    // Wood = 11
    ParticleTypeProperties {
        label: "Wood",
        base_color: Color::new(0.3, 0.22, 0.17, 1.0),
        weight: f32::INFINITY,
        moves: false,
        auto_move: false,
        fluid: false,
        condensates: false,
        flammability: 0.1,
        wet_flammability: None,
        base_fuel: Some(200),
    },
    // Acid = 12
    ParticleTypeProperties {
        label: "Acid",
        base_color: Color::new(0.67, 0.98, 0.25, 1.0),
        weight: 80.0,
        moves: true,
        auto_move: false,
        fluid: true,
        condensates: false,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
    },
];

impl ParticleType {
    pub const fn properties(&self) -> ParticleTypeProperties {
        PROPERTIES[*self as usize]
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

impl Deleted {
    fn or(&self, other: Deleted) -> Deleted {
        match (self, other) {
            (Deleted::False, Deleted::False) => Deleted::False,
            _ => Deleted::True,
        }
    }

    fn update(&mut self, other: Deleted) {
        *self = self.or(other);
    }
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
        let false_premove =
            |_self: &mut Self, _dxdy: (isize, isize), _api: &mut WorldApi| Deleted::False;

        if self.particle_type.properties().moves && self.particle_type.properties().auto_move {
            deleted.update(self.movement(&mut api, false_premove));
        }

        match self.particle_type {
            ParticleType::Steam => {
                let lasty = api.xy().1;
                self.movement(&mut api, false_premove);
                deleted.update(self.update_condensation(&mut api, lasty));
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
            ParticleType::Acid => deleted.update(self.movement(
                &mut api,
                |particle: &mut Self, dxdy: (isize, isize), api: &mut WorldApi| {
                    particle.try_mutual_destruction(dxdy, api)
                },
            )),
            _ => {}
        }

        if self.burning {
            deleted.update(self.burn(&mut api));
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

            // If our neighbour is watered (i.e. a watered fungus), we use its
            // wet_flammability
            let neighbour_flammability;
            let watered = neighbour.watered.unwrap_or(false);
            if !watered {
                neighbour_flammability = neighbour.particle_type.properties().flammability;
            } else {
                neighbour_flammability = neighbour
                    .particle_type
                    .properties()
                    .wet_flammability
                    .unwrap();
            }

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
    fn rises(&self) -> bool {
        self.particle_type.properties().weight < ParticleType::Empty.properties().weight
    }

    fn movement<F>(&mut self, api: &mut WorldApi, premove_function: F) -> Deleted
    where
        F: Fn(&mut Self, (isize, isize), &mut WorldApi) -> Deleted,
    {
        if self.moved.unwrap() {
            return Deleted::False;
        }

        let check_directions;
        let deleted;

        if self.particle_type.properties().fluid {
            // self.fluid_movement(api);
            let last_dir;
            (check_directions, last_dir) = if self.moving_right.unwrap() {
                (vec![(0, 1), (1, 1), (-1, 1), (1, 0), (-1, 0)], (-1, 0))
            } else {
                (vec![(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)], (1, 0))
            };

            let last_dxdy;
            // TODO: Should maybe find a way to use deleted.update here
            (deleted, last_dxdy) = self.movement_loop(api, check_directions, premove_function);

            if let Some(last_dxdy) = last_dxdy {
                if last_dxdy == (-1, 1) {
                    self.moving_right = Some(false)
                }
                if last_dxdy == (1, 1) {
                    self.moving_right = Some(true)
                } else if last_dxdy == last_dir {
                    self.moving_right = Some(!self.moving_right.unwrap());
                }
            }
        } else {
            // self.sand_movement(api);
            let r = api.random::<bool>();
            let right: isize = if r { -1 } else { 1 };
            check_directions = vec![(0, 1), (right, 1), (0 - right, 1)];
            // TODO: Should maybe find a way to use deleted.update here
            (deleted, _) = self.movement_loop(api, check_directions, premove_function);
        }
        deleted
    }

    fn movement_loop<F>(
        &mut self,
        api: &mut WorldApi,
        check_directions: Vec<(isize, isize)>,
        premove_function: F,
    ) -> (Deleted, Option<(isize, isize)>)
    where
        F: Fn(&mut Self, (isize, isize), &mut WorldApi) -> Deleted,
    {
        //
        for dxdy in check_directions.into_iter() {
            //
            let dxdy_new = if self.rises() {
                (dxdy.0, -dxdy.1)
            } else {
                dxdy
            };

            let deleted = premove_function(self, dxdy, api);

            if deleted == Deleted::True {
                return (deleted, None);
            } else if self.try_moving_to(dxdy_new, api) {
                return (deleted, Some(dxdy));
            }
        }
        (Deleted::False, None)
    }

    fn try_mutual_destruction(&mut self, dxdy: (isize, isize), api: &mut WorldApi) -> Deleted {
        // println!("TRYING MUTUAL DESTRUCTION");
        let other_ptype = api.neighbour(dxdy).particle_type;
        if other_ptype != self.particle_type
            && other_ptype != ParticleType::Border
            && other_ptype != ParticleType::Empty
        {
            api.replace_with_new(dxdy, ParticleType::Empty);
            api.replace_with_new((0, 0), ParticleType::Empty);
            return Deleted::True;
        }
        Deleted::False
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
