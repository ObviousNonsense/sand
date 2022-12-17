use super::*;

pub enum ParticleType {
    Sand,
    Concrete,
}

pub struct Particle {
    pub particle_type: ParticleType,
    pub id: usize,
    position: (usize, usize),
    color: Color,
}

impl Particle {
    pub fn new(x: usize, y: usize, particle_type: ParticleType, id: usize) -> Self {
        let color = match particle_type {
            ParticleType::Concrete => GRAY,
            ParticleType::Sand => YELLOW,
        };

        Self {
            particle_type,
            id,
            position: (x, y),
            color,
        }
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }

    pub fn draw(&self) {
        let (x, y) = self.position;
        let xpt = PIXELS_PER_PARTICLE * x as f32;
        let ypt = PIXELS_PER_PARTICLE * y as f32;
        draw_rectangle(
            xpt,
            ypt,
            PIXELS_PER_PARTICLE,
            PIXELS_PER_PARTICLE,
            self.color,
        );
    }

    pub fn update(&mut self) {
        todo!();
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
