use std::f32::consts::{FRAC_PI_4, PI};

use super::*;
use ::rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};
use array2d::Array2D;

const GRAVITY: Vec2 = vec2(0.0, 0.5);

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
}

#[derive(Debug, Clone, Copy)]
// The immutable properties of a particle type
pub struct ParticleTypeProperties {
    pub base_color: Color,
    pub weight: f32,
    moves: bool,
    fluid: bool,
}

pub fn base_properties(particle_type: ParticleType) -> ParticleTypeProperties {
    match particle_type {
        ParticleType::Border => ParticleTypeProperties {
            base_color: GRAY,
            weight: f32::INFINITY,
            moves: false,
            fluid: false,
        },
        ParticleType::Concrete => ParticleTypeProperties {
            base_color: GRAY,
            weight: f32::INFINITY,
            moves: false,
            fluid: false,
        },
        ParticleType::Empty => ParticleTypeProperties {
            base_color: Color::new(0.2, 0.2, 0.2, 1.0),
            weight: 1.0,
            moves: true,
            fluid: true,
        },
        ParticleType::Sand => ParticleTypeProperties {
            base_color: YELLOW,
            weight: 90.0,
            moves: true,
            fluid: false,
        },
        ParticleType::Water { .. } => ParticleTypeProperties {
            base_color: BLUE,
            weight: 60.0,
            moves: true,
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
    velocity: Option<Vec2>,
    moving_right: Option<bool>,
}

impl Particle {
    fn new(particle_type: ParticleType, rng: &mut ThreadRng) -> Self {
        // TODO: modulate individual particle color relative to base_color

        let moved;
        let velocity: Option<Vec2>;

        if base_properties(particle_type).moves {
            moved = Some(false);
            velocity = Some(Vec2::new(0.0, 0.0));
        } else {
            moved = None;
            velocity = None;
        }

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
            velocity,
            moving_right,
        }
    }

    fn set_moved(&mut self, val: bool) {
        if base_properties(self.particle_type).moves {
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
        draw_particle(x, y, base_properties(self.particle_type).base_color);
        if base_properties(self.particle_type).moves {
            let (px, py) = xy_to_pixels(x, y);
            let px_center = px + PIXELS_PER_PARTICLE / 2.0;
            let py_center = py + PIXELS_PER_PARTICLE / 2.0;
            let pv = self.velocity.unwrap() * PIXELS_PER_PARTICLE / 2.0;
            draw_line(
                px_center,
                py_center,
                px_center + pv.x,
                py_center + pv.y,
                2.0,
                RED,
            )
        }
    }
}

pub fn draw_particle(x: usize, y: usize, color: Color) {
    let (px, py) = xy_to_pixels(x, y);
    draw_rectangle(px, py, PIXELS_PER_PARTICLE, PIXELS_PER_PARTICLE, color);
}

#[derive(Debug, Clone)]
struct ParticleSource {
    particle_type: ParticleType,
    replaces: bool,
}

impl ParticleSource {
    fn draw(&self, x: usize, y: usize) {
        let mut color = base_properties(self.particle_type).base_color;
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
            let idx = *idx;
            let xy = self.index_to_xy(idx);
            let particle_clone = self.particle_grid[xy].clone();

            if particle_clone.updated {
                continue;
            }
            self.particle_grid[xy].updated = true;

            match particle_clone.particle_type {
                // ParticleType::Empty => {
                //     self.apply_gravity(xy);
                //     // self.apply_pressure(xy);
                // }
                ParticleType::Sand => {
                    self.apply_gravity(xy);
                    // self.apply_pressure(xy);
                    // self.movement(xy, particle_clone);
                    self.sand_movement(xy, particle_clone);
                }
                ParticleType::Water => {
                    self.fluid_movement(xy, particle_clone);
                }
                _ => {}
            }
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

    // ─── Movement Methods ────────────────────────────────────────────────────────────────
    fn try_grid_position(
        &mut self,
        xy1: (usize, usize),
        xy2: (usize, usize),
        // try_swap: bool, // Not relevant until there are things that move up
    ) -> bool {
        let other_p = &self.particle_grid[xy2];
        let my_weight = self.weight_at(xy1);
        let other_weight = self.weight_at(xy2);

        // If the other position is empty, move into it
        if other_p.particle_type == ParticleType::Empty {
            // This particle has moved
            self.particle_grid[xy1].set_moved(true);
            self.swap_particles(xy1, xy2);
            return true;
        } else if self.movable_at(xy2) && !other_p.moved().unwrap() {
            // If there's something there and it's movable and hasn't already moved,
            // try to displace it
            if my_weight * self.rng.gen::<f32>() > other_weight {
                // Try getting the other particle to move before we take its place:
                // If we get here, both particles will definitely move:
                self.particle_grid[xy1].set_moved(true);
                self.particle_grid[xy2].set_moved(true);
                self.displace_particle(xy1, xy2);
                return true;
            }
        }
        false
    }

    fn displace_particle(&mut self, xy1: (usize, usize), xy2: (usize, usize)) {
        // xy1 is the location of the particle initially trying to move
        // xy2 is the location of the particle being displaced

        // First try moving down, then down+right, down+left, right, left, up+right, up+left, up
        let positions_to_try = vec![
            (0, 1),
            (1, 1),
            (-1, 1),
            (1, 0),
            (-1, 0),
            (1, -1),
            (-1, -1),
            (0, -1),
        ];
        // let mut moved = false;
        for pos in positions_to_try {
            // xy3 is the location we're checking if we can move to
            let xy3 = self.relative_xy(xy2, pos);
            // If it's the same location as the particle that's trying to
            // displace us, don't bother (infinite loop?)
            if xy1 == xy3 {
                continue;
            }
            // Try moving there
            let moved = self.try_grid_position(xy2, xy3);

            // If we moved, we're done
            if moved {
                // self.particle_grid[xy3].set_moved(true);
                break;
            }
        }

        // If the particle at xy2 moved, we take its place.
        // If it didn't, we swap with it.
        // Either way, we swap with whatever's now at xy2
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
        base_properties(self.particle_grid[xy].particle_type).moves
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

    fn apply_gravity(&mut self, xy: (usize, usize)) {
        if !base_properties(self.particle_grid[xy].particle_type).moves {
            unreachable!(
                "Tried applying gravity to a particle that doesn't move: {:?}",
                self.particle_grid[xy]
            )
        }
        // Weight needs to factor in here?
        // self.particle_grid
        //     .get_mut(xy.0, xy.1)
        //     .map(|x| x.velocity.as_mut().unwrap().y += GRAVITY);
        self.apply_force(xy, GRAVITY);
        // .momentum.unwrap()[(0, 1)] += 1.0;
    }

    // I don't think this function is what I want actually. Probably I should
    // have the moving particle apply pressure to other things?
    fn apply_pressure(&mut self, xy: (usize, usize)) {
        if !base_properties(self.particle_grid[xy].particle_type).moves {
            unreachable!(
                "Tried applying pressure to a particle that doesn't move: {:?}",
                self.particle_grid[xy]
            )
        }

        let mut pressure = vec2(0.0, 0.0);
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let other_particle = self.particle_grid[self.relative_xy(xy, (dx, dy))].clone();
                if base_properties(other_particle.particle_type).moves {
                    pressure += other_particle.velocity.unwrap();
                }
            }
        }

        self.apply_force(xy, pressure / 8.0);
    }

    fn apply_force(&mut self, xy: (usize, usize), val: Vec2) {
        if !base_properties(self.particle_grid[xy].particle_type).moves {
            unreachable!(
                "Tried applying force to a particle that doesn't move: {:?}",
                self.particle_grid[xy]
            )
        }

        // Mass should probably be applied here
        self.particle_grid.get_mut(xy.0, xy.1).map(|x| {
            let v = (x.velocity.unwrap() + val).clamp_length_max(1.0);
            *x.velocity.as_mut().unwrap() = v;
        });
    }

    fn rotate_velocity(&mut self, xy: (usize, usize), angle: f32) {
        if !base_properties(self.particle_grid[xy].particle_type).moves {
            unreachable!(
                "Tried rotating velocity on a particle that doesn't move: {:?}",
                self.particle_grid[xy]
            )
        }
        self.particle_grid.get_mut(xy.0, xy.1).map(|x| {
            let v = Vec2::from_angle(angle)
                .rotate(x.velocity.unwrap())
                .clamp_length_max(1.0);

            // println!("-------------------");
            // dbg!(angle);
            // dbg!(x.velocity.unwrap().length());
            // dbg!(x.velocity.unwrap().angle_between(vec2(1.0, 0.0)));
            // dbg!(v);
            // dbg!(v).angle_between(vec2(0.0, 0.0));
            *x.velocity.as_mut().unwrap() = v;
        });
    }

    fn set_velocity(&mut self, xy: (usize, usize), val: Vec2) {
        self.particle_grid.get_mut(xy.0, xy.1).map(|x| {
            *x.velocity.as_mut().unwrap() = val;
        });
    }

    fn collision(&mut self, xy1: (usize, usize), xy2: (usize, usize)) {}

    fn movement_direction(&mut self, xy: (usize, usize)) -> ((isize, isize), f32) {
        let v = self.particle_grid[xy].velocity.unwrap();
        let angle = v.y.atan2(v.x);
        // let v_polar = cartesian_to_polar(v_cart);
        // dbg!(v_polar);
        // let angle = v_polar.y;
        let angle_round = (angle / FRAC_PI_4).round() * FRAC_PI_4;
        let rem = angle - angle_round;
        // dbg!(angle_round);
        let v_cart_round = polar_to_cartesian(1.0, angle_round).round();
        // dbg!(v_cart_round);
        (v_cart_round.as_isize().clamp_one(), rem)
    }

    // fn movement(&mut self, xy: (usize, usize), particle_clone: Particle) {
    //     // Look at velocity to decide which direction to try moving
    //     // -
    //     let original_velocity = self.particle_grid[xy].velocity.unwrap();
    //     let r = self.rng.gen();
    //     // let r = false;
    //     let angles = if r {
    //         vec![0.0, -FRAC_PI_4, FRAC_PI_4]
    //     } else {
    //         vec![0.0, FRAC_PI_4, -FRAC_PI_4]
    //     };

    //     let mut moved = false;
    //     let mut dxdy = (0, 0);

    //     for theta in angles.into_iter() {
    //         // println!("-------------------------------------");
    //         // dbg!(self.particle_grid[xy].velocity.unwrap());
    //         self.rotate_velocity(xy, theta);
    //         // dbg!(self.particle_grid[xy].velocity.unwrap());
    //         dxdy = self.movement_direction(xy);
    //         // dbg!(dxdy);
    //         let other_xy = self.relative_xy(xy, dxdy);
    //         if self.particle_grid[other_xy].particle_type == ParticleType::Empty {
    //             self.swap_particles(xy, other_xy);
    //             moved = true;
    //             break;
    //         }

    //         // if it didn't move, set the velocity back to the original
    //         self.set_velocity(xy, original_velocity);
    //     }

    //     if !moved {
    //         self.set_velocity(xy, vec2(0.0, 0.0));
    //     } else if dxdy.0 != 0 {
    //         let xy_new = self.relative_xy(xy, dxdy);
    //         self.apply_force(xy_new, -0.5 * self.particle_grid[xy_new].velocity.unwrap());
    //     }
    // }

    fn sand_movement(&mut self, xy: (usize, usize), particle_clone: Particle) {
        if particle_clone.moved().unwrap() {
            return;
        }
        let v = particle_clone.velocity.unwrap();
        let v_snap = v.snap_to_pi_8();
        let direction = v_snap.as_isize().clamp_one();
        let mut check_directions = vec![direction];

        match direction {
            (_, 1) => {}
            (-1, 0) => {
                if v.y < v_snap.y {
                    check_directions.push((-1, -1));
                }
            }
            (-1, -1) => {
                if v.y < v_snap.y {
                    check_directions.push((0, -1));
                }
            }
            _ => unreachable!(),
        };

        // let r = self.rng.gen();
        // let right: isize = if r { -1 } else { 1 };
        // let check_directions = vec![(0, 1), (right, 1), (0 - right, 1)];

        for dxdy in check_directions.iter() {
            let other_xy = self.relative_xy(xy, *dxdy);
            let moved = self.try_grid_position(xy, other_xy);
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

        for (dxdy, k) in check_directions.iter().zip(0..5) {
            let other_xy = self.relative_xy(xy, *dxdy);
            let moved = self.try_grid_position(xy, other_xy);

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
                if base_properties(ptype).moves {
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
}

trait HelperMethods {
    fn as_isize(&self) -> (isize, isize);
    fn snap_to_pi_8(&self) -> Self;
}

impl HelperMethods for Vec2 {
    fn as_isize(&self) -> (isize, isize) {
        (self.x as isize, self.y as isize)
    }

    fn snap_to_pi_8(&self) -> Vec2 {
        let angle = self.y.atan2(self.x);
        let angle_round = (angle / FRAC_PI_4).round() * FRAC_PI_4;
        let v_snapped = polar_to_cartesian(self.length(), angle_round);
        v_snapped
    }
}

trait ClampOne {
    fn clamp_one(&self) -> (isize, isize);
}

impl ClampOne for (isize, isize) {
    fn clamp_one(&self) -> (isize, isize) {
        (self.0.clamp(-1, 1), self.1.clamp(-1, 1))
    }
}

// ───────────────────────────────────────────────────────────────────────────────────────────── ✣ ─
