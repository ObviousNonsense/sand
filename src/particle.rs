use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(usize)]
pub enum ParticleType {
    Empty = 0,
    Border = 1,
    Sand = 2,
    Water = 3,
    Concrete = 4,
}

// #[repr(usize)]
// pub enum SandProperties {
//     Moved = 0,
// }

// #[repr(usize)]
// pub enum WaterProperties {
//     MovingRight = 1,
// }

// #[derive(Debug)]
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub color: Color,
    pub moved: bool,
    pub bool_state: [bool; 2],
}

impl Particle {
    pub fn new(particle_type: ParticleType) -> Self {
        let color = match particle_type {
            ParticleType::Empty => BLACK,
            ParticleType::Border => GRAY,
            ParticleType::Concrete => GRAY,
            ParticleType::Sand => YELLOW,
            ParticleType::Water => BLUE,
        };

        let bool_state = [false, false];

        Self {
            particle_type,
            color,
            moved: false,
            bool_state,
        }
    }

    pub fn draw(&self, x: usize, y: usize) {
        let px = PIXELS_PER_PARTICLE * x as f32;
        let py = PIXELS_PER_PARTICLE * y as f32;
        draw_rectangle(px, py, PIXELS_PER_PARTICLE, PIXELS_PER_PARTICLE, self.color);
    }
}
