use color_eyre::owo_colors::colors::xterm::Blue;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleType {
    Empty = 0,
    Border = 1,
    Sand = 2,
    Water = 3,
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
            ParticleType::Water => BLUE,
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
        //     1.0,
        //     BLACK,
        // );
    }
}
