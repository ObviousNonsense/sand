use super::*;
use world::*;

pub enum ParticleType {
    Sand,
    Concrete,
}

pub struct Particle {
    particle_type: ParticleType,
    position: (usize, usize),
    color: Color,
}

impl Particle {
    pub fn new(x: usize, y: usize, particle_type: ParticleType) -> Self {
        let color = match particle_type {
            ParticleType::Concrete => GRAY,
            ParticleType::Sand => YELLOW,
        };

        Self {
            particle_type,
            position: (x, y),
            color,
        }
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }

    pub fn draw(&self, pixels_per_particle: f32) {
        let (x, y) = self.position;
        let xpt = pixels_per_particle * x as f32;
        let ypt = pixels_per_particle * y as f32;
        draw_rectangle(
            xpt,
            ypt,
            pixels_per_particle,
            pixels_per_particle,
            self.color,
        );
    }
}
