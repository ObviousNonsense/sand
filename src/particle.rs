use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleType {
    Empty = 0,
    Border = 1,
    Sand = 2,
    Water = 3,
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
    color: Color,
    pub moved: bool,
    pub bool_state: [bool; 2],
}

impl Particle {
    pub fn new(particle_type: ParticleType) -> Self {
        let color = match particle_type {
            ParticleType::Empty => BLACK,
            ParticleType::Sand => YELLOW,
            ParticleType::Border => GRAY,
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
