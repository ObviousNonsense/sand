use super::*;
use ::rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Clone, Copy)]
// The immutable properties of a particle type
pub struct ParticleTypeProperties {
    pub label: &'static str,
    pub base_color: PColor,
    pub weight: f32,
    pub moves: bool,
    pub auto_move: bool,
    pub fluid: bool,
    pub dispersion_rate: Option<u8>,
    pub flammability: f32,
    pub wet_flammability: Option<f32>,
    pub base_fuel: Option<i16>,
    pub base_durability: Option<i16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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
        base_color: PColor::new(129, 129, 129),
        weight: f32::INFINITY,
        moves: false,
        auto_move: false,
        fluid: false,
        dispersion_rate: None,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
        base_durability: None,
    },
    // Concrete = 1
    ParticleTypeProperties {
        label: "Concrete",
        base_color: PColor::new(129, 129, 129),
        weight: f32::INFINITY,
        moves: false,
        auto_move: false,
        fluid: false,
        dispersion_rate: None,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
        base_durability: Some(100),
    },
    // Empty = 2
    ParticleTypeProperties {
        label: "Empty",
        base_color: PColor::new(51, 51, 51),
        weight: 1.0,
        moves: false,
        auto_move: false,
        fluid: false,
        dispersion_rate: None,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
        base_durability: None,
    },
    // Sand = 3
    ParticleTypeProperties {
        label: "Sand",
        base_color: PColor::new(251, 250, 74),
        weight: 90.0,
        moves: true,
        auto_move: true,
        fluid: false,
        dispersion_rate: None,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
        base_durability: Some(20),
    },
    // Water = 4
    ParticleTypeProperties {
        label: "Water",
        base_color: PColor::new(8, 116, 236),
        weight: 60.0,
        moves: true,
        auto_move: true,
        fluid: true,
        dispersion_rate: Some(5),
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
        base_durability: None,
    },
    // Steam = 5
    ParticleTypeProperties {
        label: "Steam",
        base_color: PColor::new(192, 209, 239),
        weight: 0.5,
        moves: true,
        auto_move: false,
        fluid: true,
        dispersion_rate: Some(10),
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
        base_durability: None,
    },
    // Fungus = 6
    ParticleTypeProperties {
        label: "Fungus",
        base_color: PColor::new(103, 147, 131),
        weight: f32::INFINITY,
        moves: false,
        auto_move: false,
        fluid: false,
        dispersion_rate: None,
        flammability: 0.125,
        wet_flammability: Some(0.015),
        base_fuel: Some(35),
        base_durability: Some(10),
    },
    // Flame = 7
    ParticleTypeProperties {
        label: "Flame",
        base_color: PColor::new(255, 123, 36),
        weight: f32::INFINITY,
        moves: false,
        auto_move: false,
        fluid: false,
        dispersion_rate: None,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: Some(0),
        base_durability: None,
    },
    // Methane = 8
    ParticleTypeProperties {
        label: "Methane",
        base_color: PColor::new(148, 119, 165),
        weight: 0.2,
        moves: true,
        auto_move: true,
        fluid: true,
        dispersion_rate: Some(7),
        flammability: 0.95,
        wet_flammability: None,
        base_fuel: Some(6),
        base_durability: None,
    },
    // Gunpowder = 9
    ParticleTypeProperties {
        label: "Gunpowder",
        base_color: PColor::new(0, 0, 0),
        weight: 90.0,
        moves: true,
        auto_move: true,
        fluid: false,
        dispersion_rate: None,
        flammability: 0.6,
        wet_flammability: None,
        base_fuel: Some(35),
        base_durability: Some(20),
    },
    // Oil = 10
    ParticleTypeProperties {
        label: "Oil",
        base_color: PColor::new(112, 87, 50),
        weight: 50.0,
        moves: true,
        auto_move: true,
        fluid: true,
        dispersion_rate: Some(10),
        flammability: 0.9,
        wet_flammability: None,
        base_fuel: Some(25),
        base_durability: None,
    },
    // Wood = 11
    ParticleTypeProperties {
        label: "Wood",
        base_color: PColor::new(87, 56, 46),
        weight: f32::INFINITY,
        moves: false,
        auto_move: false,
        fluid: false,
        dispersion_rate: None,
        flammability: 0.1,
        wet_flammability: None,
        base_fuel: Some(200),
        base_durability: Some(70),
    },
    // Acid = 12
    ParticleTypeProperties {
        label: "Acid",
        base_color: PColor::new(166, 249, 94),
        weight: 63.0,
        moves: true,
        auto_move: false,
        fluid: true,
        dispersion_rate: Some(1),
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
        base_durability: Some(50),
    },
];

impl ParticleType {
    pub const fn properties(&self) -> ParticleTypeProperties {
        PROPERTIES[*self as usize]
    }
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
#[derive(Debug, Clone, PartialEq)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub updated: bool,
    pub color: PColor,
    original_color: PColor,
    burning: bool,
    moved: Option<bool>,
    moving_right: Option<bool>,
    condensation_countdown: Option<i16>,
    initial_condensation_countdown: Option<i16>,
    watered: Option<bool>,
    fresh: Option<bool>,
    fuel: Option<i16>,
    durability: Option<i16>,
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

        let condensation_countdown = if particle_type == ParticleType::Steam {
            Some(300 + rng.gen_range(-100..100))
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
        let durability = particle_type.properties().base_durability;

        // let color = particle_type.properties().base_color;
        let color = if particle_type == ParticleType::Empty {
            particle_type.properties().base_color
        } else {
            particle_type.properties().base_color.scale_hsv(
                0.0,
                rng.gen_range(0.94..1.06),
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
            durability,
        }
    }

    pub fn update(&mut self, mut api: WorldApi) {
        let mut deleted = Deleted::False;
        let false_premove =
            |_self: &mut Self, _dxdy: (isize, isize), _api: &mut WorldApi| Deleted::False;

        if self.particle_type.properties().moves && self.particle_type.properties().auto_move
        // && self.particle_type != ParticleType::Water
        {
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
                    particle.try_decaying(dxdy, api)
                },
            )),
            _ => {}
        }

        if self.burning {
            deleted.update(self.burn(&mut api));
        }

        if deleted == Deleted::False {
            // self.updated = true;
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
}

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
            let watered = neighbour.watered.unwrap_or(false);
            let neighbour_flammability = if !watered {
                neighbour.particle_type.properties().flammability
            } else {
                neighbour
                    .particle_type
                    .properties()
                    .wet_flammability
                    .unwrap()
            };

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

    fn burning_flicker_color(api: &mut WorldApi) -> PColor {
        ParticleType::Flame.properties().base_color.scale_hsv(
            api.random_range(-15.0..15.0),
            api.random_range(0.95..1.05),
            api.random_range(0.9..1.1),
        )
    }
}

/// Fungus (plant?) methods
impl Particle {
    fn set_watered(&mut self, w: bool) {
        if w {
            self.color = self.original_color.scale_hsv(10.0, 1.7, 1.0);
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
        let neighbour = api.neighbour(dxdy);
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
                let mut neighbour = neighbour.clone();
                neighbour.set_watered(true);
                api.replace_with(dxdy, neighbour);
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
        let dispersion_rate = self.particle_type.properties().dispersion_rate.unwrap_or(1) as isize;
        for dxdy in check_directions.into_iter() {
            //

            let r = if self.rises() { -1 } else { 1 };

            let dxdy_new = if dxdy.1 == 0 {
                (dispersion_rate * dxdy.0, dxdy.1)
            } else {
                (dxdy.0, r * dxdy.1)
            };

            let deleted = premove_function(self, dxdy, api);

            if deleted == Deleted::True {
                return (deleted, None);
            } else {
                if dxdy_new.0.abs() > 1 {
                    self.disperse(dxdy_new, api);
                } else {
                    self.try_moving_to(dxdy_new, api);
                }
                if self.moved.unwrap() {
                    return (deleted, Some(dxdy));
                }
            }
        }
        (Deleted::False, None)
    }

    fn disperse(&mut self, dxdy: (isize, isize), api: &mut WorldApi) {
        iterate_over_line_delta(dxdy, |dx, dy| {
            let other_type = self.try_moving_to((dx, dy), api);
            if let Some(other_type) = other_type {
                if other_type == ParticleType::Empty {
                    let r = if self.rises() { -1 } else { 1 };
                    if api.neighbour((0, r)).particle_type == ParticleType::Empty
                        && api.neighbour((dx.signum(), r)).particle_type == ParticleType::Empty
                    {
                        return false;
                    }
                    return true;
                } else {
                    return false;
                }
            }
            false
        })
    }

    fn try_decaying(&mut self, dxdy: (isize, isize), api: &mut WorldApi) -> Deleted {
        let other_p = api.neighbour_mut(dxdy);
        if other_p.particle_type != self.particle_type {
            if let Some(other_durability) = other_p.durability.as_mut() {
                *other_durability -= 1;
                if let Some(my_durability) = self.durability.as_mut() {
                    *my_durability -= 1;
                }
                if *other_durability < 0 {
                    api.replace_with_new(dxdy, ParticleType::Empty);
                }
                if self.durability.unwrap() < 0 {
                    api.replace_with_new((0, 0), ParticleType::Empty);
                    return Deleted::True;
                }
            }
        }
        Deleted::False
    }

    /// Checks if this particle can and will move in the given direction.
    /// Assumes that if it can move there it will (sets self.moved to true)
    fn try_moving_to(&mut self, dxdy: (isize, isize), api: &mut WorldApi) -> Option<ParticleType> {
        // let rand_factor = api.random::<f32>();
        let other_p = api.neighbour(dxdy);
        let other_weight = api.neighbour(dxdy).particle_type.properties().weight;

        let other_type = other_p.particle_type;

        // If the other position is empty, try moving into it
        if other_type == ParticleType::Empty {
            // If we're moving sideways don't compare weights, just do it
            if dxdy.1 == 0 || self.weight_check(api, other_weight) {
                self.moved = Some(true);
                api.swap_with(dxdy);
                return Some(other_type);
            }
        } else if other_p.particle_type.properties().moves && !other_p.moved.unwrap() {
            // If there's something there and it's moveable and it hasn't
            // already moved, then we might swap with it
            if self.weight_check(api, other_weight) {
                let other_p_mut = api.neighbour_mut(dxdy);
                other_p_mut.moved = Some(true);
                self.moved = Some(true);
                api.swap_with(dxdy);
                return Some(other_type);
            }
        }
        None
    }

    fn weight_check(&self, api: &mut WorldApi, other_weight: f32) -> bool {
        (!self.rises()
            && (self.particle_type.properties().weight * api.random::<f32>() > other_weight))
            || (self.rises()
                && (other_weight * api.random::<f32>() > self.particle_type.properties().weight))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl std::fmt::Debug for PColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PColor {{r: {}, g: {}, b: {}}}", self.r, self.g, self.b)
    }
}

impl PColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    // ChatGPT wrote these methods. "The formula used in the implementation is
    // based on the description provided in the Wikipedia article on HSL and
    // HSV."
    fn into_hsv(self) -> (f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;
        let cmax = r.max(g).max(b);
        let cmin = r.min(g).min(b);
        let delta = cmax - cmin;
        let h = if delta == 0.0 {
            0.0
        } else if cmax == r {
            60.0 * ((g - b) / delta % 6.0)
        } else if cmax == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };
        let s = if cmax == 0.0 { 0.0 } else { delta / cmax };
        let v = cmax;
        (h, s, v)
    }

    fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        PColor {
            r: ((r + m) * 255.0) as u8,
            g: ((g + m) * 255.0) as u8,
            b: ((b + m) * 255.0) as u8,
        }
    }

    pub fn scale_hsv(&self, rotate_h: f32, scale_s: f32, scale_v: f32) -> Self {
        let (h, s, v) = self.into_hsv();
        let h = (h + rotate_h) % 360.0;
        let s = (s * scale_s).clamp(0.0, 1.0);
        let v = (v * scale_v).clamp(0.0, 1.0);
        PColor::from_hsv(h, s, v)
    }

    pub fn add_hsv(&self, rotate_h: f32, add_s: f32, add_v: f32) -> Self {
        let (h, s, v) = self.into_hsv();
        let h = (h + rotate_h) % 360.0;
        let s = (s + add_s).clamp(0.0, 1.0);
        let v = (v + add_v).clamp(0.0, 1.0);
        PColor::from_hsv(h, s, v)
    }
}
