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
    pub terminal_velocity_sq: Option<u16>,
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
        terminal_velocity_sq: None,
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
        terminal_velocity_sq: None,
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
        terminal_velocity_sq: None,
        dispersion_rate: None,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
        base_durability: None,
    },
    // Sand = 3
    ParticleTypeProperties {
        label: "Sand",
        base_color: PColor::new(226, 188, 128),
        weight: 90.0,
        moves: true,
        auto_move: true,
        fluid: false,
        terminal_velocity_sq: Some(u16::pow(5, 2)),
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
        terminal_velocity_sq: Some(u16::pow(5, 2)),
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
        terminal_velocity_sq: Some(u16::pow(5, 2)),
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
        terminal_velocity_sq: None,
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
        terminal_velocity_sq: None,
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
        terminal_velocity_sq: Some(u16::pow(5, 2)),
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
        terminal_velocity_sq: Some(u16::pow(5, 2)),
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
        terminal_velocity_sq: Some(u16::pow(5, 2)),
        dispersion_rate: Some(3),
        flammability: 0.9,
        wet_flammability: None,
        base_fuel: Some(25),
        base_durability: None,
    },
    // Wood = 11
    ParticleTypeProperties {
        label: "Wood",
        base_color: PColor::new(98, 57, 35),
        weight: f32::INFINITY,
        moves: false,
        auto_move: false,
        fluid: false,
        terminal_velocity_sq: None,
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
        terminal_velocity_sq: Some(u16::pow(5, 2)),
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
enum Status {
    Deleted,
    Alive,
}

impl Status {
    fn or(&self, other: Status) -> Status {
        match (self, other) {
            (Status::Alive, Status::Alive) => Status::Alive,
            _ => Status::Deleted,
        }
    }

    fn update(&mut self, other: Status) {
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
    velocity: Option<I8Vec2>,
    moving_right: Option<bool>,
    condensation_countdown: Option<i16>,
    initial_condensation_countdown: Option<i16>,
    watered: Option<bool>,
    fuel: Option<i16>,
    durability: Option<i16>,
}

// General Particle Methods
impl Particle {
    pub fn new(particle_type: ParticleType, rng: &mut ThreadRng) -> Self {
        let (moved, velocity) = if particle_type.properties().moves {
            (Some(false), Some(I8Vec2::ZERO))
        } else {
            (None, None)
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

        let burning = particle_type == ParticleType::Flame;

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
            velocity,
            moving_right,
            condensation_countdown,
            initial_condensation_countdown: condensation_countdown,
            watered,
            fuel,
            durability,
        }
    }

    pub fn update(&mut self, mut api: WorldApi) {
        let mut status = Status::Alive;
        let false_premove = |_self: &mut Self, _dxdy: I8Vec2, _api: &mut WorldApi| Status::Alive;

        if self.particle_type.properties().moves && self.particle_type.properties().auto_move {
            status.update(self.movement(&mut api, false_premove));
        }

        match self.particle_type {
            ParticleType::Steam => {
                let lasty = api.xy().1;
                self.movement(&mut api, false_premove);
                status.update(self.update_condensation(&mut api, lasty));
            }
            ParticleType::Fungus => {
                self.grow_fungus(&mut api);
            }
            ParticleType::Flame => {
                if !self.burning {
                    status = Status::Deleted;
                    api.replace_with_new((0, 0), ParticleType::Empty);
                }
            }
            ParticleType::Acid => status.update(self.movement(
                &mut api,
                |particle: &mut Self, dxdy: I8Vec2, api: &mut WorldApi| {
                    particle.try_decaying(dxdy, api)
                },
            )),
            _ => {}
        }

        if self.burning {
            status.update(self.burn(&mut api));
        }

        if status == Status::Alive {
            api.update_in_world(self.to_owned());
        }
    }

    pub fn refresh(&mut self) {
        self.updated = false;
        if self.particle_type.properties().moves {
            self.moved = Some(false);
        }
    }
}

/// Burning methods
impl Particle {
    fn set_burning(&mut self, b: bool) {
        self.burning = b;
        if !b {
            self.color = self.original_color;
        }
    }

    fn burn(&mut self, api: &mut WorldApi) -> Status {
        self.color = Particle::burning_flicker_color(api);

        let dxdy_list = i8vec2_vector([(0, -1), (1, 0), (-1, 0), (0, 1)]);

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
                if neighbour_flammability * (1.0 - 0.5 * dxdy.y as f32) > r {
                    neighbour.set_burning(true);
                }
            } else if neighbour.particle_type == ParticleType::Empty && self.fuel.unwrap() > 0 {
                if dxdy.y < 1 && api.neighbour((-1, 0)).burning && api.neighbour((1, 0)).burning {
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
                return Status::Deleted;
            }
        }
        Status::Alive
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
        let dxdy_list = i8vec2_vector([
            (0, -1),
            (0, 1),
            (1, 0),
            (-1, 0),
            (-1, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
        ]);

        let dxdy = dxdy_list[api.random_range(0..dxdy_list.len())];
        let mut neighbour_clone = api.neighbour(dxdy).clone();
        if self.watered.unwrap() {
            // This doesn't handle the edge case where every fungus particle is
            // watered and has no where to grow
            api.might_update();

            if neighbour_clone.particle_type == ParticleType::Empty {
                let mut count = 0;
                for ddxddy in dxdy_list {
                    let dxdy2 = dxdy + ddxddy;
                    if api.neighbour(dxdy2).particle_type == ParticleType::Fungus {
                        count += 1;
                    }
                }

                if api.random_range(0..4) > count {
                    // if count < 3 && api.random() {
                    api.replace_with_new(dxdy, ParticleType::Fungus);
                    self.set_watered(false);
                }
            } else if neighbour_clone.particle_type == ParticleType::Fungus
                && !neighbour_clone.watered.unwrap()
            {
                neighbour_clone.set_watered(true);
                api.replace_with(dxdy, neighbour_clone);
                self.set_watered(false);
            }
        } else if neighbour_clone.particle_type == ParticleType::Water {
            api.replace_with_new(dxdy, ParticleType::Empty);
            self.set_watered(true);
        }
    }
}

/// Condensation methods
impl Particle {
    fn update_condensation(&mut self, api: &mut WorldApi, lasty: usize) -> Status {
        if api.xy().1 == lasty {
            if let Some(count) = self.condensation_countdown.as_mut() {
                *count -= 1;
                if *count <= 0 {
                    api.replace_with_new((0, 0), ParticleType::Water);
                    return Status::Deleted;
                }
            }
        } else {
            self.condensation_countdown = self.initial_condensation_countdown;
        }
        Status::Alive
    }
}

/// Movement Methods
impl Particle {
    fn rises(&self) -> bool {
        self.particle_type.properties().weight < ParticleType::Empty.properties().weight
    }

    fn movement<F>(&mut self, api: &mut WorldApi, premove_function: F) -> Status
    where
        F: Fn(&mut Self, I8Vec2, &mut WorldApi) -> Status,
    {
        if self.moved.unwrap() {
            return Status::Alive;
        }

        // println!("here");
        let check_directions;
        let status;

        // Apply gravity to things that don't rise
        if !self.rises() {
            if let Some(vel) = self.velocity.as_mut() {
                let mag_v_sq = vel.length_sq();
                if mag_v_sq
                    < self
                        .particle_type
                        .properties()
                        .terminal_velocity_sq
                        .unwrap()
                {
                    vel.y += 1;
                }
            }
        }

        let last_dir;

        if self.particle_type.properties().fluid {
            // self.fluid_movement(api);
            check_directions = if self.moving_right.unwrap() {
                i8vec2_vector([(0, 1), (1, 1), (-1, 1), (1, 0), (-1, 0)])
            } else {
                i8vec2_vector([(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)])
            };

            let last_possible_dir = check_directions[4].clone();

            // TODO: Should maybe find a way to use status.update here
            (status, last_dir) = self.movement_loop(api, check_directions, premove_function);

            if let Some(last_dxdy) = last_dir {
                if last_dxdy == (-1, 1).into() {
                    self.moving_right = Some(false)
                }
                if last_dxdy == (1, 1).into() {
                    self.moving_right = Some(true)
                } else if last_dxdy == last_possible_dir {
                    self.moving_right = Some(!self.moving_right.unwrap());
                }
            }
        } else {
            // self.sand_movement(api);
            let r = api.random::<bool>();
            let right = if r { -1 } else { 1 };
            check_directions = i8vec2_vector([(0, 1), (right, 1), (0 - right, 1)]);
            // TODO: Should maybe find a way to use status.update here
            (status, last_dir) = self.movement_loop(api, check_directions, premove_function);
        }

        let last_dxdy = last_dir.unwrap_or(i8vec2(1, 0));

        // Reset vertical velocity if we've stopped (could maybe just slow down instead?)
        if last_dxdy.x != 0 {
            if let Some(vel) = self.velocity.as_mut() {
                vel.y = 0;
            }
        }

        status
    }

    fn movement_loop<F>(
        &mut self,
        api: &mut WorldApi,
        check_directions: Vec<I8Vec2>,
        premove_function: F,
    ) -> (Status, Option<I8Vec2>)
    where
        F: Fn(&mut Self, I8Vec2, &mut WorldApi) -> Status,
    {
        //
        let dispersion_rate = self.particle_type.properties().dispersion_rate.unwrap_or(1) as i8;
        for dxdy in check_directions.into_iter() {
            //
            let r = if self.rises() { -1 } else { 1 };

            let dxdy_new = if dxdy.y == 0 {
                i8vec2(dispersion_rate * dxdy.x, dxdy.y)
            } else if r == 1 && dxdy.x == 0 {
                i8vec2(dxdy.x, self.velocity.unwrap().y)
            } else {
                i8vec2(dxdy.x, r * dxdy.y)
            };

            let status = premove_function(self, dxdy_new, api);

            if status == Status::Deleted {
                return (status, None);
            } else {
                if dxdy_new.x.abs() > 1 {
                    self.disperse(dxdy_new, api);
                } else if dxdy_new.y > 1 {
                    self.fall_with_gravity(dxdy_new, api);
                } else {
                    self.try_moving_to(dxdy_new, api);
                }
                if self.moved.unwrap() {
                    return (status, Some(dxdy));
                }
            }
        }
        (Status::Alive, None)
    }

    fn fall_with_gravity(&mut self, dxdy: I8Vec2, api: &mut WorldApi) {
        iterate_over_line_delta(dxdy.into(), |dx, dy| {
            self.try_moving_to(i8vec2(dx as i8, dy as i8), api);
            self.moved.unwrap()
        })
    }

    fn disperse(&mut self, dxdy: I8Vec2, api: &mut WorldApi) {
        iterate_over_line_delta(dxdy.into(), |dx, dy| {
            let other_type = self.try_moving_to(i8vec2(dx as i8, dy as i8), api);
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

    fn try_decaying(&mut self, dxdy: I8Vec2, api: &mut WorldApi) -> Status {
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
                    return Status::Deleted;
                }
            }
        }
        Status::Alive
    }

    /// Checks if this particle can and will move in the given direction.
    /// Assumes that if it can move there it will (sets self.moved to true)
    fn try_moving_to(&mut self, dxdy: I8Vec2, api: &mut WorldApi) -> Option<ParticleType> {
        // let rand_factor = api.random::<f32>();
        let other_p = api.neighbour(dxdy);
        let other_weight = api.neighbour(dxdy).particle_type.properties().weight;

        let other_type = other_p.particle_type;
        let my_weight = self.particle_type.properties().weight;
        let rises = self.rises();

        // If the other position is empty, try moving into it
        if other_type == ParticleType::Empty {
            // If we're moving sideways don't compare weights, just do it
            if dxdy.y == 0 || Particle::weight_check(api, rises, my_weight, other_weight) {
                self.moved = Some(true);
                api.swap_with(dxdy);
                return Some(other_type);
            }
        } else if other_p.particle_type.properties().moves && !other_p.moved.unwrap() {
            // If there's something there and it's moveable and it hasn't
            // already moved, then we might swap with it
            if Particle::weight_check(api, rises, my_weight, other_weight) {
                let other_p_mut = api.neighbour_mut(dxdy);
                other_p_mut.moved = Some(true);
                self.moved = Some(true);
                api.swap_with(dxdy);
                return Some(other_type);
            }
        }
        None
    }

    // #[rustfmt::skip]
    fn weight_check(api: &mut WorldApi, rises: bool, my_weight: f32, other_weight: f32) -> bool {
        if rises && other_weight > my_weight {
            api.might_update();
            other_weight * api.random::<f32>() > my_weight
        } else if !rises && my_weight > other_weight {
            api.might_update();
            my_weight * api.random::<f32>() > other_weight
        } else {
            false
        }
    }
}
