use super::*;
use ::rand::{
    distributions::uniform::SampleRange, prelude::Distribution, rngs::ThreadRng, seq::SliceRandom,
    thread_rng, Rng,
};
use array2d::Array2D;

/* #region  */
#[derive(Debug, Clone)]
struct ParticleSource {
    particle_type: ParticleType,
    replaces: bool,
}

impl ParticleSource {
    fn draw(&self, x: usize, y: usize, painter: &Painter) {
        let mut color = self.particle_type.properties().base_color;
        color.a = 0.5;
        color.r -= 0.1;
        color.g -= 0.1;
        color.b -= 0.1;

        painter.draw_source(
            x,
            y,
            color,
            self.replaces,
            self.particle_type == ParticleType::Empty,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn dxdy(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Portal {
    partner_xy: Option<(usize, usize)>,
    // if you're standing where the portal is, which direction do you go to walk through it
    direction: Direction,
    color: Color,
}

impl Portal {
    fn draw(&self, x: usize, y: usize, painter: &Painter) {
        painter.draw_portal(x, y, self.direction, self.color);
    }
}

/* #endregion */

pub struct WorldApi<'a> {
    world: &'a mut World,
    xy: (usize, usize),
}

impl<'a> WorldApi<'a> {
    pub fn random<T>(&mut self) -> T
    where
        ::rand::distributions::Standard: Distribution<T>,
    {
        self.world.rng.gen::<T>()
    }

    pub fn random_range<T, R>(&mut self, slice: R) -> T
    where
        T: ::rand::distributions::uniform::SampleUniform,
        R: SampleRange<T>,
    {
        self.world.rng.gen_range::<T, R>(slice)
    }

    pub fn neighbour(&self, dxdy: (isize, isize)) -> &Particle {
        self.world.relative_particle(self.xy, dxdy)
    }

    pub fn neighbour_mut(&mut self, dxdy: (isize, isize)) -> &mut Particle {
        self.world.relative_particle_mut(self.xy, dxdy)
    }

    /// Swaps the particle with the one dxdy away.
    /// Do not attempt to mutate the particle after calling this.
    // fn swap_with(&mut self, dxdy: (isize, isize), particle: Particle) {
    pub fn swap_with(&mut self, dxdy: (isize, isize)) {
        let other_xy = self.world.relative_xy(self.xy, dxdy);
        let other_p = self.world.particle_grid[other_xy].clone();
        self.world.particle_grid[self.xy] = other_p;
        // self.world.particle_grid[other_xy] = particle;
        self.xy = other_xy;
    }

    pub fn replace_with_new(&mut self, dxdy: (isize, isize), particle_type: ParticleType) {
        let xy = self.world.relative_xy(self.xy, dxdy);
        self.world.add_new_particle(particle_type, xy, true);
    }

    pub fn new_particle(&mut self, particle_type: ParticleType) -> Particle {
        Particle::new(particle_type, &mut self.world.rng)
    }

    pub fn replace_with(&mut self, dxdy: (isize, isize), particle: Particle) {
        let xy = self.world.relative_xy(self.xy, dxdy);
        self.world.particle_grid[xy] = particle;
    }

    pub fn update_in_world(&mut self, particle: Particle) {
        self.world.particle_grid[self.xy] = particle;
    }

    pub fn xy(&self) -> &(usize, usize) {
        &self.xy
    }
}

// ─── World ─────────────────────────────────────────────────────────────────────────────────── ✣ ─
pub struct World {
    particle_grid: Array2D<Particle>,
    source_grid: Array2D<Option<ParticleSource>>,
    portal_grid: Array2D<Option<Portal>>,
    width: usize,
    height: usize,
    rng: ThreadRng,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = thread_rng();

        let mut particle_grid =
            Array2D::filled_with(Particle::new(ParticleType::Empty, &mut rng), width, height);
        let source_grid = Array2D::filled_with(None, width, height);
        let portal_grid = Array2D::filled_with(None, width, height);

        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    // println!("x: {:?}, y: {:?}", x, y);
                    particle_grid[(x, y)] = Particle::new(ParticleType::Border, &mut rng);
                }
            }
        }

        Self {
            particle_grid,
            source_grid,
            portal_grid,
            width,
            height,
            rng,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn particle_at(&self, xy: (usize, usize)) -> &Particle {
        &self.particle_grid[xy]
    }

    // ─── Update Methods ──────────────────────────────────────────────────────────────────
    pub fn update_all(&mut self) {
        self.update_all_sources();
        self.update_all_particles();
    }

    fn update_all_sources(&mut self) {
        for x in 1..self.width {
            for y in 1..self.height {
                let xy = (x, y);
                if let Some(source) = self.source_grid[xy].clone() {
                    if self.rng.gen() {
                        self.add_new_particle(source.particle_type, xy, source.replaces);
                    }
                }
            }
        }
    }

    fn update_all_particles(&mut self) {
        // TODO: Consider pre-generating this and storing it (either pass it
        // into the function or store it in the struct and clone it here)
        let mut idx_range: Vec<usize> =
            ((self.width + 1)..(self.width * self.height - 2)).collect();
        idx_range.shuffle(&mut self.rng);
        for idx in idx_range.into_iter() {
            // let idx = *idx;
            let xy = self.index_to_xy(idx);

            let mut particle_clone = self.particle_grid[xy].clone();

            if particle_clone.updated {
                continue;
            }

            particle_clone.update(WorldApi { world: self, xy });
        }
    }

    // ─── Creation Methods ────────────────────────────────────────────────────────────────
    pub fn add_new_particle(
        &mut self,
        new_particle_type: ParticleType,
        xy: (usize, usize),
        replace: bool,
    ) {
        let old_particle_type = self.particle_grid[xy].particle_type;

        match (new_particle_type, old_particle_type) {
            (_, ParticleType::Border) => {}
            (ParticleType::Empty, _) | (_, ParticleType::Empty) => {
                self.particle_grid[xy] = Particle::new(new_particle_type, &mut self.rng);
            }
            _ => {
                if replace {
                    self.particle_grid[xy] = Particle::new(new_particle_type, &mut self.rng);
                }
            }
        }
    }

    pub fn add_new_source(
        &mut self,
        source_type: ParticleType,
        xy: (usize, usize),
        source_replaces: bool,
        replace: bool,
    ) {
        if self.source_grid[xy].is_some() && !replace {
            return;
        };

        self.source_grid[xy] = Some(ParticleSource {
            particle_type: source_type,
            replaces: source_replaces,
        })
    }

    pub fn add_new_portal(
        &mut self,
        xy: (usize, usize),
        partner_xy: Option<(usize, usize)>,
        direction: Direction,
        color: Color,
    ) -> bool {
        if self.portal_exists_at(xy) {
            return false;
        }

        if let Some(partner_xy) = partner_xy {
            if let Some(ref mut partner) = self.portal_grid[partner_xy] {
                partner.partner_xy = Some(xy);
            } else {
                unreachable!("New portal purported partner does not exist")
            }
        }

        self.portal_grid[xy] = Some(Portal {
            partner_xy,
            direction,
            color,
        });
        true
    }

    pub fn portal_exists_at(&self, xy: (usize, usize)) -> bool {
        if self.portal_grid[xy].is_some() {
            return true;
        }
        false
    }

    // ─── Deletion Methods ────────────────────────────────────────────────────────────────
    pub fn delete_source(&mut self, xy: (usize, usize)) {
        self.source_grid[xy] = None;
    }

    // ─── Other ───────────────────────────────────────────────────────────────────────────
    pub fn draw_and_reset_all_particles(&mut self, painter: &Painter) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.particle_grid[(x, y)].draw_and_refresh(x, y, painter);
                if let Some(portal) = &self.portal_grid[(x, y)] {
                    portal.draw(x, y, painter);
                }
                if let Some(source) = &self.source_grid[(x, y)] {
                    source.draw(x, y, painter);
                }
            }
        }
    }

    // TODO: Consider pre-calculating this and storing it as a vector
    fn index_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    fn relative_particle(&self, xy: (usize, usize), dxdy: (isize, isize)) -> &Particle {
        &self.particle_grid[self.relative_xy(xy, dxdy)]
    }

    fn relative_particle_mut(&mut self, xy: (usize, usize), dxdy: (isize, isize)) -> &mut Particle {
        let (new_x, new_y) = self.relative_xy(xy, dxdy);
        self.particle_grid.get_mut(new_x, new_y).unwrap()
    }

    fn relative_xy(&self, xy: (usize, usize), dxdy: (isize, isize)) -> (usize, usize) {
        // dbg!(xy, dxdy);
        match &self.portal_grid[xy] {
            Some(portal) => {
                if let Some(xy2) = portal.partner_xy {
                    let portal_dxdy = portal.direction.dxdy();
                    // dbg!(xy2, portal_dxdy);
                    if portal_dxdy.0 == dxdy.0 && portal_dxdy.1 == dxdy.1 {
                        return xy2;
                    }
                }
            }
            None => {}
        };
        (
            (xy.0 as isize + dxdy.0) as usize,
            (xy.1 as isize + dxdy.1) as usize,
        )
    }
}
// ───────────────────────────────────────────────────────────────────────────────────────────── ✣ ─
