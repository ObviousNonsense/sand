use super::*;

// #[derive(Clone)]
#[derive(Debug)]
pub enum ParticleType {
    Sand,
    Water,
    Concrete,
}

// #[derive(Clone)]
#[derive(Debug)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub id: usize,
    x: usize,
    y: usize,
    color: Color,
}

impl Particle {
    pub fn new(x: usize, y: usize, particle_type: ParticleType, id: usize) -> Self {
        let color = match particle_type {
            ParticleType::Water => BLUE,
            ParticleType::Sand => YELLOW,
            ParticleType::Concrete => GRAY,
        };

        Self {
            particle_type,
            id,
            x,
            y,
            color,
        }
    }

    // pub fn position(&self) -> (usize, usize) {
    //     self.position
    // }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn draw(&self) {
        let xpt = PIXELS_PER_PARTICLE * self.x as f32;
        let ypt = PIXELS_PER_PARTICLE * self.y as f32;
        draw_rectangle(
            xpt,
            ypt,
            PIXELS_PER_PARTICLE,
            PIXELS_PER_PARTICLE,
            self.color,
        );
        draw_rectangle_lines(
            xpt,
            ypt,
            PIXELS_PER_PARTICLE,
            PIXELS_PER_PARTICLE,
            2.0,
            BLACK,
        );
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
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
