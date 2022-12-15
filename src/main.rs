use color_eyre::eyre::Result;
use macroquad::prelude::*;
use std::rc::{Rc, Weak};
use world::World;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sand".to_owned(),
        window_height: 400,
        window_width: 500,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    color_eyre::install()?;
    use particle::*;
    use world::*;

    let pixels_per_particle = 10.0;
    let width = (screen_width() / pixels_per_particle) as usize;
    let height = (screen_height() / pixels_per_particle) as usize;

    let world = World::new(width, height);

    loop {
        clear_background(BLACK);
        world.draw_all_particles(pixels_per_particle);
        next_frame().await
    }
    Ok(())
}

mod particle {
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
}

mod world {
    use super::*;
    use particle::*;

    pub struct World {
        width: usize,
        height: usize,
        size: usize,
        grid: Vec<Option<Particle>>,
    }

    impl World {
        pub fn new(width: usize, height: usize) -> Self {
            let size = width * height;
            let grid: Vec<Option<Particle>> = vec![];
            let mut world = Self {
                width,
                height,
                size,
                grid,
            };

            world.initialize_grid();
            world
        }

        fn initialize_grid(&mut self) {
            for _ in 0..self.size {
                self.grid.push(None);
            }

            for x in 0..self.width {
                for y in 0..self.height {
                    if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
                        let p = Particle::new(x, y, ParticleType::Concrete);
                        println!("x: {:?}, y: {:?}", x, y);
                        self.add_particle(p);
                    }
                }
            }
        }

        pub fn add_particle(&mut self, new_particle: Particle) {
            let (x, y) = new_particle.position();
            let old_particle = self.get_particle(x, y);
            match old_particle {
                Some(_) => {}
                None => {
                    let idx = self.xy_to_index(x, y);
                    self.grid[idx] = Some(new_particle)
                }
            };
        }

        pub fn draw_all_particles(&self, pixels_per_particle: f32) {
            for space in self.grid.iter() {
                match space {
                    Some(particle) => particle.draw(pixels_per_particle),
                    None => {}
                }
            }
        }

        fn get_particle(&self, x: usize, y: usize) -> &Option<Particle> {
            &self.grid[self.xy_to_index(x, y)]
        }

        fn xy_to_index(&self, x: usize, y: usize) -> usize {
            let index = y * self.width + x;
            return index;
        }

        fn index_to_xy(&self, i: usize) -> (usize, usize) {
            let x = i % self.width;
            let y = i / self.width;
            (x, y)
        }
    }
}
