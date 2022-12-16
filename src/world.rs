
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
