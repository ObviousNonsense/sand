use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleType {
    Empty = 0,
    Border = 1,
    Sand = 2,
}

// #[derive(Debug)]
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub particle_type: ParticleType,
    color: Color,
    pub moved: bool,
}

impl Particle {
    pub fn new(particle_type: ParticleType) -> Self {
        let color = match particle_type {
            ParticleType::Empty => BLACK,
            ParticleType::Sand => YELLOW,
            ParticleType::Border => GRAY,
        };

        Self {
            particle_type,
            color,
            moved: false,
        }
    }

    // pub fn position(&self) -> (usize, usize) {
    //     self.position
    // }

    pub fn draw(&self, x: usize, y: usize) {
        let xpt = PIXELS_PER_PARTICLE * x as f32;
        let ypt = PIXELS_PER_PARTICLE * y as f32;
        draw_rectangle(
            xpt,
            ypt,
            PIXELS_PER_PARTICLE,
            PIXELS_PER_PARTICLE,
            self.color,
        );
        // draw_rectangle_lines(
        //     xpt,
        //     ypt,
        //     PIXELS_PER_PARTICLE,
        //     PIXELS_PER_PARTICLE,
        //     2.0,
        //     BLACK,
        // );
    }

    // pub fn move_to(&mut self, x: usize, y: usize) {
    //     self.x = x;
    //     self.y = y;
    // }
}

// use std::{
//     hash::{Hash, Hasher},
//     rc::Rc,
// };
// // use world::*;

// // https://stackoverflow.com/questions/69971592/set-of-rct-where-t-isnt-hash-or-ord
// pub struct RcHashParticle(pub Rc<Particle>);

// impl PartialEq for RcHashParticle {
//     fn eq(&self, other: &RcHashParticle) -> bool {
//         Rc::ptr_eq(&self.0, &other.0)
//     }
// }

// impl Eq for RcHashParticle {}

// impl Hash for RcHashParticle {
//     fn hash<H>(&self, hasher: &mut H)
//     where
//         H: Hasher,
//     {
//         hasher.write_usize(Rc::as_ptr(&self.0) as usize);
//     }
// }
