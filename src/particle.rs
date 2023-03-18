use std::cmp::Ordering;

use super::*;
use ::rand::{rngs::ThreadRng, Rng};
use helpers::{DOWN, DOWN_L, DOWN_R, LEFT, RIGHT};

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
    pub inertial_resistance: Option<f32>,
    pub dynamic_friction: Option<f32>,
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
    Coal = 9,
    Oil = 10,
    Wood = 11,
    Acid = 12,
}

const GRAVITY: f32 = 1.0;

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
        inertial_resistance: None,
        dynamic_friction: None,
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
        inertial_resistance: None,
        dynamic_friction: None,
    },
    // Empty = 2
    ParticleTypeProperties {
        label: "Empty",
        base_color: PColor::new(80, 80, 80),
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
        inertial_resistance: None,
        dynamic_friction: None,
    },
    // Sand = 3
    ParticleTypeProperties {
        label: "Sand",
        base_color: PColor::new(226, 188, 128),
        weight: 90.0,
        moves: true,
        auto_move: true,
        fluid: false,
        terminal_velocity_sq: Some(u16::pow(10, 2)),
        dispersion_rate: None,
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
        base_durability: Some(20),
        inertial_resistance: Some(0.025),
        dynamic_friction: Some(1.5),
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
        inertial_resistance: Some(0.0),
        dynamic_friction: Some(1.5),
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
        inertial_resistance: Some(0.0),
        dynamic_friction: Some(1.5),
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
        inertial_resistance: None,
        dynamic_friction: None,
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
        inertial_resistance: None,
        dynamic_friction: None,
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
        inertial_resistance: Some(0.0),
        dynamic_friction: Some(1.5),
    },
    // Coal = 9
    ParticleTypeProperties {
        label: "Coal",
        base_color: PColor::new(5, 5, 5),
        weight: 90.0,
        moves: true,
        auto_move: true,
        fluid: false,
        terminal_velocity_sq: Some(u16::pow(5, 2)),
        dispersion_rate: None,
        flammability: 0.05,
        wet_flammability: None,
        base_fuel: Some(1000),
        base_durability: Some(20),
        inertial_resistance: Some(0.8),
        dynamic_friction: Some(1.5),
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
        dispersion_rate: Some(5),
        flammability: 0.9,
        wet_flammability: None,
        base_fuel: Some(25),
        base_durability: None,
        inertial_resistance: Some(0.0),
        dynamic_friction: Some(1.5),
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
        inertial_resistance: None,
        dynamic_friction: None,
    },
    // Acid = 12
    ParticleTypeProperties {
        label: "Acid",
        base_color: PColor::new(166, 249, 94),
        weight: 63.0,
        moves: true,
        auto_move: true,
        fluid: true,
        terminal_velocity_sq: Some(u16::pow(5, 2)),
        dispersion_rate: Some(1),
        flammability: 0.0,
        wet_flammability: None,
        base_fuel: None,
        base_durability: Some(50),
        inertial_resistance: Some(0.0),
        dynamic_friction: Some(1.5),
    },
];

impl ParticleType {
    pub const fn properties(&self) -> ParticleTypeProperties {
        PROPERTIES[*self as usize]
    }

    fn premove_fn(&self) -> fn(&mut Particle, I8Vec2, &mut WorldApi) {
        match self {
            Self::Acid => Particle::try_decaying,
            _ => empty_premove,
        }
    }
}

fn empty_premove(_: &mut Particle, _: I8Vec2, _: &mut WorldApi) {}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
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
    status: Status,
    burning: bool,
    moved: Option<bool>,
    is_in_motion: Option<bool>,
    pub velocity: Option<Vec2>,
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
            (Some(false), Some(Vec2::ZERO))
        } else {
            (None, None)
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

        // let is_in_motion = if particle_type.properties().moves && !particle_type.properties().fluid
        let is_in_motion = if particle_type.properties().moves {
            Some(true)
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
            status: Status::Alive,
            burning,
            moved,
            is_in_motion,
            velocity,
            condensation_countdown,
            initial_condensation_countdown: condensation_countdown,
            watered,
            fuel,
            durability,
        }
    }

    pub fn update(&mut self, mut api: WorldApi) {
        if self.particle_type.properties().moves && self.particle_type.properties().auto_move {
            self.movement(&mut api);
        }

        match self.particle_type {
            ParticleType::Steam => {
                let lasty = api.xy().1;
                self.movement(&mut api);
                self.update_condensation(&mut api, lasty);
            }
            ParticleType::Fungus => {
                self.grow_fungus(&mut api);
            }
            ParticleType::Flame => {
                if !self.burning {
                    self.status = Status::Deleted;
                    api.replace_with_new((0, 0), ParticleType::Empty);
                }
            }
            _ => {}
        }

        if self.burning {
            self.burn(&mut api);
        }

        if self.status == Status::Alive {
            api.update_in_world(self.to_owned());
        }
    }

    pub fn refresh(&mut self) {
        self.updated = false;
        if self.particle_type.properties().moves {
            // TODO Come up with a way to make velocity spreading through a pile get
            // things on the other side to start moving (actually, this kind of already
            // works, see QUESTION in try_moving_one_space method)

            // if self.velocity.unwrap().length() > 10.0 {
            //     self.is_in_motion = Some(true);
            // } else {
            //     self.is_in_motion = Some(false);
            // }
            // self.velocity = Some(Vec2::new(
            //     self.velocity.unwrap().x.trunc(),
            //     self.velocity.unwrap().y.trunc(),
            // ));
            // if self.velocity.unwrap() == Vec2::ZERO {
            //     self.is_in_motion = Some(false);
            // }
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

    fn burn(&mut self, api: &mut WorldApi) {
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
                self.status.update(Status::Deleted);
                return;
            }
        }
        self.status.update(Status::Alive);
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
    fn update_condensation(&mut self, api: &mut WorldApi, lasty: usize) {
        if api.xy().1 == lasty {
            if let Some(count) = self.condensation_countdown.as_mut() {
                *count -= 1;
                if *count <= 0 {
                    api.replace_with_new((0, 0), ParticleType::Water);
                    self.status.update(Status::Deleted);
                    return;
                }
            }
        } else {
            self.condensation_countdown = self.initial_condensation_countdown;
        }
        self.status.update(Status::Alive);
    }
}

// const CHECKDIR_DN_DNR_DNL_R_L: [I8Vec2; 5] = [DOWN, DOWN_R, DOWN_L, RIGHT, LEFT];
// const CHECKDIR_DN_DNL_DNR_L_R: [I8Vec2; 5] = [DOWN, DOWN_L, DOWN_R, LEFT, RIGHT];

// const CHECKDIR_DN_DNR_DNL_R: [I8Vec2; 4] = [DOWN, DOWN_R, DOWN_L, RIGHT];
// const CHECKDIR_DN_DNL_DNR_L: [I8Vec2; 4] = [DOWN, DOWN_L, DOWN_R, LEFT];

fn mom2vel(velocity: Vec2) -> I8Vec2 {
    // I8Vec2::new((velocity.x / 10.0) as i8, (velocity.y / 10.0) as i8)
    I8Vec2::new(velocity.x as i8, velocity.y as i8)
}

/// Movement Methods
impl Particle {
    fn rises(&self) -> bool {
        self.particle_type.properties().weight < ParticleType::Empty.properties().weight
    }

    fn movement(&mut self, api: &mut WorldApi) {
        if self.moved.unwrap() {
            return;
        }

        // Apply gravity to things that don't rise
        if !self.rises() {
            if let Some(vel) = self.velocity.as_mut() {
                vel.y += GRAVITY;
            }
        }

        if self.particle_type.properties().fluid {
            // // ─── Fluid Movement ──────────────────────────────────────────
            let velx = self.velocity.unwrap().x;
            // If we're free falling check every normal direction, with bouncing
            let check_directions = if self.is_in_motion.unwrap() {
                if velx.abs() < 0.0001 {
                    // If x velocity is 0, randomly pick whether to favour moving left or right
                    if api.random() {
                        vec![DOWN, DOWN_R, DOWN_L, RIGHT, LEFT]
                    } else {
                        vec![DOWN, DOWN_L, DOWN_R, LEFT, RIGHT]
                    }
                } else if velx > 0.0 {
                    // If x velocity is positive, favour moving right
                    vec![DOWN, DOWN_R, DOWN_L, RIGHT, LEFT]
                } else {
                    // If x velocity is negative, favour moving left
                    vec![DOWN, DOWN_L, DOWN_R, LEFT, RIGHT]
                }
            } else {
                // If we're not free falling (have stopped moving), only try moving down
                vec![DOWN]
            };
            self.movement_loop_fluid(api, check_directions.into());
            if self.moved.unwrap() {
                // Particle::set_neighbours_free_falling(api);
                self.is_in_motion = Some(true)
            } else if self.velocity.unwrap() == Vec2::ZERO {
                self.is_in_motion = Some(false)
            };

            // // TODO: Right now this causes chunks to wake up unnecessarily
            // Particle::set_neighbours_free_falling(api);
        } else {
            // ─── Solid Movement ──────────────────────────────────────────
            let velx = self.velocity.unwrap().x;

            let check_directions = if self.is_in_motion.unwrap() {
                // If we're free falling check every normal direction, without bouncing
                if velx.abs() < 0.0001 {
                    // If x velocity is 0, randomly pick whether to favour moving left or right
                    if api.random() {
                        vec![DOWN, DOWN_R, DOWN_L, RIGHT]
                    } else {
                        vec![DOWN, DOWN_L, DOWN_R, LEFT]
                    }
                } else if velx > 0.0 {
                    // If x velocity is positive, favour moving right
                    vec![DOWN, DOWN_R, DOWN_L, RIGHT]
                } else {
                    // If x velocity is negative, favour moving left
                    vec![DOWN, DOWN_L, DOWN_R, LEFT]
                }
            } else {
                // If we're not free falling (have stopped moving), only try moving down
                vec![DOWN]
            };
            self.movement_loop_solid(api, check_directions);
            if self.moved.unwrap() {
                // Particle::set_neighbours_free_falling(api);
                self.is_in_motion = Some(true)
            } else if self.velocity.unwrap() == Vec2::ZERO {
                self.is_in_motion = Some(false)
            };
        }
    }

    fn set_neighbours_in_motion(api: &mut WorldApi) {
        for dxdy in [(-1, 0), (1, 0), (-1, -1), (1, -1)].into_iter() {
            let is_in_motion = api.neighbour(dxdy).is_in_motion;
            if let Some(is_in_motion) = is_in_motion {
                let ptype = api.neighbour(dxdy).particle_type;
                if !is_in_motion
                    && api.random::<f32>() > ptype.properties().inertial_resistance.unwrap()
                {
                    let neighbour = api.neighbour_mut(dxdy);
                    neighbour.is_in_motion = Some(true);
                }
            }
        }
    }

    fn movement_loop_solid(&mut self, api: &mut WorldApi, check_directions: Vec<I8Vec2>) {
        //
        for dir in check_directions.into_iter() {
            //
            let velocity = self.velocity.unwrap();
            if dir.y == 0 && velocity.x.abs() <= 0.0001 {
                // If a solid's x velocity is 0, it shouldn't try moving directly sideways
                return;
            }

            // let velocity = mom2vel(velocity);
            let dxdy;

            if dir.x == 0 {
                // If we're moving straight down, we can move more than one space at a
                // time (due to gravity)
                dxdy = i8vec2(0, velocity.y as i8 * dir.y);
                self.try_moving_along_line(dxdy, api);
            } else {
                // Otherwise we can only move one space at a time.
                dxdy = dir;
                self.try_moving_one_space(dxdy, api);
            };

            if self.moved.unwrap() {
                // If we moved, our new velocity's x-direction will be the x-direction
                // that we actually moved in
                if let Some(vel) = self.velocity.as_mut() {
                    // if dir.y == 1 {
                    //     vel.y += GRAVITY;
                    // }
                    vel.x = dxdy.x.signum() as f32 * vel.x.abs();
                }
                // We moved, so we don't need to keep looking
                return;
            } else {
                let mut vel = self.velocity.unwrap();
                if dir == DOWN {
                    // If we just tried moving down but failed, then we transfer some
                    // y-velocity to x-velocity and reduce y-velocity by the same amount
                    let friction = self.particle_type.properties().dynamic_friction.unwrap();
                    let velocity_y_to_x = vel.y - (GRAVITY + friction);
                    vel.x = if vel.x.signum() == 1.0 {
                        f32::max(vel.x + velocity_y_to_x, 0.0)
                    } else {
                        f32::min(vel.x - velocity_y_to_x, 0.0)
                    };
                    vel.y = f32::max(vel.y - (GRAVITY + friction), 0.0);
                } else if dir.y == 1 {
                    // If we tried moving down diagonally, reduce y-velocity by friction
                    let friction = self.particle_type.properties().dynamic_friction.unwrap();
                    vel.y = f32::max(vel.y - friction, 0.0);
                }
                self.velocity = Some(vel);
            }
        }
    }

    fn movement_loop_fluid(&mut self, api: &mut WorldApi, check_directions: Vec<I8Vec2>) {
        //
        let dispersion_rate = self.particle_type.properties().dispersion_rate.unwrap_or(0) as i8;
        for dir in check_directions.into_iter() {
            //
            let velocity = self.velocity.unwrap();
            // let velocity = mom2vel(velocity);
            let r = if self.rises() { -1 } else { 1 };
            let dxdy;

            if dir.y == 0 {
                // We're moving straight horizontally, so apply dispersion + whatever
                // velocity we've accumulated and move multiple spaces at once
                dxdy = i8vec2((velocity.x.abs() as i8 + dispersion_rate) * dir.x, 0);
                self.try_moving_horizontal_until_gap(dxdy, api);
            } else if r == 1 && dir.x == 0 {
                // We're moving straight down (r == 1 means we don't rise), so move more
                // than one space at a time due to gravity
                dxdy = i8vec2(0, velocity.y as i8 * dir.y);
                self.try_moving_along_line(dxdy, api);
            } else {
                // We rise and/or are moving diagonally, so move one space at a time.
                dxdy = i8vec2(dir.x, r * dir.y);
                self.try_moving_one_space(dxdy, api);
            };

            if self.moved.unwrap() {
                // If we moved, our new velocity's x-direction will be the x-direction
                // that we actually moved in
                if let Some(vel) = self.velocity.as_mut() {
                    vel.x = dxdy.x.signum() as f32 * vel.x.abs();
                }
                // We moved, so we don't need to keep looking
                return;
            } else {
                let mut vel = self.velocity.unwrap();
                if dir == DOWN {
                    // If we just tried moving down but failed, then we transfer some
                    // y-velocity to x-velocity and reduce y-velocity by the same amount
                    let friction = self.particle_type.properties().dynamic_friction.unwrap();
                    let velocity_y_to_x = vel.y - (GRAVITY + friction);
                    vel.x = if vel.x.signum() == 1.0 {
                        f32::max(vel.x + velocity_y_to_x, 0.0)
                    } else {
                        f32::min(vel.x - velocity_y_to_x, 0.0)
                    };
                    vel.y = f32::max(vel.y - (GRAVITY + friction), 0.0);
                } else if dir.y == 1 {
                    // If we tried moving down diagonally, reduce y-velocity by friction
                    let friction = self.particle_type.properties().dynamic_friction.unwrap();
                    vel.y = f32::max(vel.y - friction, 0.0);
                }
                self.velocity = Some(vel);
            }
        }
    }

    #[inline]
    fn try_moving_along_line(&mut self, dxdy: I8Vec2, api: &mut WorldApi) {
        iterate_over_line_delta(dxdy.into(), |dx, dy| {
            self.try_moving_one_space(i8vec2(dx as i8, dy as i8), api);
            self.moved.unwrap()
        })
    }

    #[inline]
    fn try_moving_horizontal_until_gap(&mut self, dxdy: I8Vec2, api: &mut WorldApi) {
        iterate_over_line_delta(dxdy.into(), |dx, dy| {
            let other_type = self.try_moving_one_space(i8vec2(dx as i8, dy as i8), api);
            if let Some(other_type) = other_type {
                if other_type == ParticleType::Empty {
                    let r = if self.rises() { -1 } else { 1 };
                    if api.neighbour((0, r)).particle_type == ParticleType::Empty
                        && api.neighbour((-dx.signum(), r)).particle_type == ParticleType::Empty
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

    fn try_decaying(&mut self, dxdy: I8Vec2, api: &mut WorldApi) {
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
                    self.status.update(Status::Deleted);
                }
            }
        }
        self.status.update(Status::Alive);
    }

    /// Checks if this particle can and will move in the given direction.
    /// Assumes that if it can move there it will (sets self.moved to true)
    fn try_moving_one_space(&mut self, dxdy: I8Vec2, api: &mut WorldApi) -> Option<ParticleType> {
        let premove_fn = self.particle_type.premove_fn();
        premove_fn(self, dxdy, api);
        if self.status == Status::Deleted {
            return None;
        }

        let other_p = api.neighbour(dxdy);
        let other_weight = api.neighbour(dxdy).particle_type.properties().weight;

        let other_type = other_p.particle_type;
        let my_weight = self.particle_type.properties().weight;
        let rises = self.rises();

        // If the other position is empty, try moving into it
        if other_type == ParticleType::Empty {
            // If we're moving sideways don't compare weights, just do it
            if dxdy.y == 0 || Particle::weight_check(api, rises, my_weight, other_weight) {
                Particle::set_neighbours_in_motion(api);
                self.moved = Some(true);
                api.swap_with(dxdy);
                // return Some(other_type);
            }
        } else if other_p.particle_type.properties().moves {
            // If there's something there and it's moveable and it hasn't
            // already moved, then we might swap with it
            if !other_p.moved.unwrap()
                && Particle::weight_check(api, rises, my_weight, other_weight)
            {
                Particle::set_neighbours_in_motion(api);
                let other_p_mut = api.neighbour_mut(dxdy);
                other_p_mut.moved = Some(true);
                self.moved = Some(true);
                api.swap_with(dxdy);
                // return Some(other_type);
            }

            // If I'm moving freely and I hit something moveable and I failed to move
            // against it, then transfer half my velocity to it.

            // TODO The amount transfered should be related to weight

            // QUESTION Not checking if velocity > 0 here allows it to transfer much
            // better through solid piles, but I don't know how well that will work for
            // fluids (especially if I want to start letting liquids move up)
            if self.is_in_motion.unwrap() && !self.moved.unwrap()
            // && self.velocity.unwrap().length_squared() > 0.0001
            {
                let other_p_mut = api.neighbour_mut(dxdy);
                if let Some(is_in_motion) = other_p_mut.is_in_motion.as_mut() {
                    *is_in_motion = true;
                }

                // Not sure if projecting onto the direction I'm moving is
                // necessary, but it feels right anyway
                // let half_my_velocity = self
                //     .velocity
                //     .unwrap()
                //     .project_onto(Vec2::new(dxdy.x as f32, dxdy.y as f32))
                //     / 2.0;
                // other_p_mut.velocity = Some(other_p_mut.velocity.unwrap() + half_my_velocity);
                // self.velocity = Some(self.velocity.unwrap() - half_my_velocity);
                let u1 = self
                    .velocity
                    .unwrap()
                    .project_onto(Vec2::new(dxdy.x as f32, dxdy.y as f32));
                let u2 = other_p_mut.velocity.unwrap();
                let m1 = self.particle_type.properties().weight;
                let m2 = other_p_mut.particle_type.properties().weight;

                let (v1, v2) = if m1 == m2 {
                    (u2, u1)
                } else {
                    let m_total = m1 + m2;
                    (
                        (m1 - m2) * u1 / m_total + 2.0 * m2 * u2 / m_total,
                        2.0 * m1 * u1 / m_total + (m2 - m1) * u2 / m_total,
                    )
                };

                // HACK This 0.5 factor is pretty much random right now.
                self.velocity = Some(v1);
                other_p_mut.velocity = Some(v2);
            }
        }
        Some(other_type)
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
