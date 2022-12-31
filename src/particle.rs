use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Enum)]
#[repr(usize)]
pub enum ParticleType {
    Empty,
    Border,
    Sand,
    Water,
    Concrete,
}

// The immutable properties of a particle type
pub struct ParticleTypeProperties {
    pub base_color: Color,
    pub movable: bool,     // TODO not used yet
    pub replaceable: bool, // TODO not used yet
}

// #[derive(Debug)]
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub particle_type: ParticleType,
    // pub color: Color,
    pub moved: bool,
    pub bool_state: [bool; 2],
}

impl Particle {
    pub fn new(particle_type: ParticleType) -> Self {
        // TODO: modulate individual particle color relative to base_color
        // let color = match particle_type {
        //     ParticleType::Empty => BLACK,
        //     ParticleType::Border => GRAY,
        //     ParticleType::Concrete => GRAY,
        //     ParticleType::Sand => YELLOW,
        //     ParticleType::Water => BLUE,
        // };

        let bool_state = [false, false];

        Self {
            particle_type,
            // color,
            moved: false,
            bool_state,
        }
    }

    pub fn draw(&self, x: usize, y: usize, color: Color) {
        // let px = PIXELS_PER_PARTICLE * x as f32;
        // let py = PIXELS_PER_PARTICLE * y as f32;
        let (px, py) = xy_to_pixels(x, y);
        draw_rectangle(px, py, PIXELS_PER_PARTICLE, PIXELS_PER_PARTICLE, color);
    }
}
