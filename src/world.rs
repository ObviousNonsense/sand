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
        let mut color: Color = self.particle_type.properties().base_color.into();
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

    pub fn swap_with(&mut self, dxdy: (isize, isize)) {
        let other_xy = self.world.relative_xy(self.xy, dxdy);
        self.world
            .put_particle(self.xy, self.world.get_particle(other_xy).clone());
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
        self.world.put_particle(xy, particle);
    }

    pub fn update_in_world(&mut self, particle: Particle) {
        self.world.put_particle(self.xy, particle);
    }

    pub fn xy(&self) -> &(usize, usize) {
        &self.xy
    }
}

#[derive(Clone)]
struct WorldChunk {
    particle_grid: Array2D<Particle>,
    update_this_frame: bool,
    update_next_frame: bool,
}

impl WorldChunk {
    fn new(chunk_size: usize, rng: &mut ThreadRng) -> Self {
        let particle_grid = Array2D::filled_with(
            Particle::new(ParticleType::Empty, rng),
            chunk_size,
            chunk_size,
        );

        Self {
            particle_grid,
            update_this_frame: true,
            update_next_frame: true,
        }
    }

    fn shift_update_flag(&mut self) {
        self.update_this_frame = self.update_next_frame;
        self.update_next_frame = false;
    }

    fn refresh_all_particles(&mut self) {
        for x in 0..self.particle_grid.row_len() {
            for y in 0..self.particle_grid.column_len() {
                self.particle_grid[(x, y)].refresh();
            }
        }
    }
}

// ─── World ─────────────────────────────────────────────────────────────────────────────────── ✣ ─
pub struct World {
    // particle_grid: Array2D<Particle>,
    chunk_grid: Array2D<WorldChunk>,
    source_grid: Array2D<Option<ParticleSource>>,
    portal_grid: Array2D<Option<Portal>>,
    chunk_size: usize,
    width: usize,
    height: usize,
    rng: ThreadRng,
}

impl World {
    pub fn new(width: usize, height: usize, chunk_size: usize) -> Self {
        assert_eq!(width % chunk_size, 0);
        assert_eq!(height % chunk_size, 0);

        let mut rng = thread_rng();

        let chunk_grid = Array2D::filled_with(
            WorldChunk::new(chunk_size, &mut rng),
            width / chunk_size,
            height / chunk_size,
        );

        // let particle_grid =
        //     Array2D::filled_with(Particle::new(ParticleType::Empty, &mut rng), width, height);
        let source_grid = Array2D::filled_with(None, width, height);
        let portal_grid = Array2D::filled_with(None, width, height);

        let mut new_world = Self {
            // particle_grid,
            chunk_grid,
            source_grid,
            portal_grid,
            chunk_size,
            width,
            height,
            rng,
        };

        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    let border_particle = Particle::new(ParticleType::Border, &mut new_world.rng);
                    new_world.put_particle((x, y), border_particle);
                }
            }
        }

        new_world
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
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
        // let mut idx_range: Vec<usize> =
        //     ((self.width + 1)..(self.width * self.height - 2)).collect();
        // idx_range.shuffle(&mut self.rng);
        // for idx in idx_range.into_iter() {
        //     // let idx = *idx;
        //     let xy = self.index_to_xy(idx);

        //     let mut particle_clone = self.get_particle(xy).clone();

        //     if particle_clone.particle_type == ParticleType::Empty || particle_clone.updated {
        //         continue;
        //     }

        //     particle_clone.update(WorldApi { world: self, xy });
        // }

        let num_chunks_x = self.width / self.chunk_size;
        let num_chunks_y = self.height / self.chunk_size;

        let mut idx_range: Vec<usize> = (0..(self.chunk_size * self.chunk_size)).collect();
        let mut chunk_x_range: Vec<usize> = (0..num_chunks_x).collect();
        let mut chunk_y_range: Vec<usize> = (0..num_chunks_y).collect();

        idx_range.shuffle(&mut self.rng);
        chunk_x_range.shuffle(&mut self.rng);
        chunk_y_range.shuffle(&mut self.rng);

        for chunk_x in chunk_x_range.iter() {
            for chunk_y in chunk_y_range.iter() {
                self.chunk_grid[(*chunk_x, *chunk_y)].shift_update_flag();

                if self.chunk_grid[(*chunk_x, *chunk_y)].update_this_frame {
                    for idx in idx_range.iter() {
                        let local_xy = self.local_index_to_xy(*idx);

                        // Clone the particle and make sure it hasn't been updated
                        // let mut particle_clone =
                        //     self.chunk_grid[(*chunk_x, *chunk_y)].particle_grid[(local_xy)].clone();

                        let particle =
                            &self.chunk_grid[(*chunk_x, *chunk_y)].particle_grid[(local_xy)];

                        if particle.particle_type == ParticleType::Empty
                            || particle.particle_type == ParticleType::Border
                            || particle.updated
                        {
                            continue;
                        }

                        self.chunk_grid[(*chunk_x, *chunk_y)].particle_grid[(local_xy)].updated =
                            true;

                        let mut particle_clone =
                            self.chunk_grid[(*chunk_x, *chunk_y)].particle_grid[(local_xy)].clone();

                        let global_xy = self.chunk_xy_to_global_xy((*chunk_x, *chunk_y), local_xy);

                        particle_clone.update(WorldApi {
                            world: self,
                            xy: global_xy,
                        });
                    }
                }
            }
        }
    }

    // ─── Creation Methods ────────────────────────────────────────────────────────────────
    pub fn add_new_particle(
        &mut self,
        new_particle_type: ParticleType,
        xy: (usize, usize),
        replace: bool,
    ) {
        let old_particle_type = self.get_particle(xy).particle_type;

        match (new_particle_type, old_particle_type) {
            (_, ParticleType::Border) => {}
            (ParticleType::Empty, _) | (_, ParticleType::Empty) => {
                let new_particle = Particle::new(new_particle_type, &mut self.rng);
                self.put_particle(xy, new_particle);
            }
            _ => {
                if replace {
                    let new_particle = Particle::new(new_particle_type, &mut self.rng);
                    self.put_particle(xy, new_particle);
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
    pub fn draw_and_refresh(&mut self, painter: &mut Painter, debug_chunks: bool) {
        let num_chunks_x = self.width / self.chunk_size;
        let num_chunks_y = self.height / self.chunk_size;

        for chunk_x in 0..num_chunks_x {
            for chunk_y in 0..num_chunks_y {
                self.chunk_grid[(chunk_x, chunk_y)].refresh_all_particles();
                // if self.chunk_grid[(chunk_x, chunk_y)].update_this_frame {
                for local_y in 0..self.chunk_size {
                    for local_x in 0..self.chunk_size {
                        let (global_x, global_y) =
                            self.chunk_xy_to_global_xy((chunk_x, chunk_y), (local_x, local_y));
                        // self.chunk_grid[(chunk_x, chunk_y)].particle_grid[(local_x, local_y)]
                        //     .refresh();
                        painter.update_image_with_particle(
                            global_x,
                            global_y,
                            self.width,
                            self.chunk_grid[(chunk_x, chunk_y)].particle_grid[(local_x, local_y)]
                                .color,
                        );
                    }
                }
                // }
            }
        }

        // for y in 0..self.height {
        //     for x in 0..self.width {
        //         self.get_particle_mut((x, y)).refresh();
        //         painter.update_image_with_particle(
        //             x,
        //             y,
        //             self.width,
        //             self.get_particle((x, y)).color,
        //         );
        //     }
        // }
        // dbg!(&painter.screen_buffer);
        // painter.screen_texture.update(&painter.screen_image);
        painter.draw_screen(self.width, self.height);

        if debug_chunks {
            for chunk_x in 0..num_chunks_x {
                for chunk_y in 0..num_chunks_y {
                    if self.chunk_grid[(chunk_x, chunk_y)].update_this_frame {
                        let (global_x, global_y) =
                            self.chunk_xy_to_global_xy((chunk_x, chunk_y), (0, 0));
                        painter.debug_chunk(
                            global_x,
                            global_y,
                            self.chunk_size,
                            self.chunk_size,
                            format!("({},{})", chunk_x, chunk_y).as_str(),
                        );
                    }
                }
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
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

    fn local_index_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.chunk_size, i / self.chunk_size)
    }

    fn global_xy_to_chunk_xy(&self, xy: (usize, usize)) -> ((usize, usize), (usize, usize)) {
        let (global_x, global_y) = xy;
        let chunk_x = global_x / self.chunk_size;
        let local_x = global_x % self.chunk_size;
        let chunk_y = global_y / self.chunk_size;
        let local_y = global_y % self.chunk_size;
        ((chunk_x, chunk_y), (local_x, local_y))
    }

    fn chunk_xy_to_global_xy(
        &self,
        chunk_xy: (usize, usize),
        local_xy: (usize, usize),
    ) -> (usize, usize) {
        (
            chunk_xy.0 * self.chunk_size + local_xy.0,
            chunk_xy.1 * self.chunk_size + local_xy.1,
        )
    }

    fn get_particle(&self, xy: (usize, usize)) -> &Particle {
        let (chunk_xy, local_xy) = self.global_xy_to_chunk_xy(xy);
        &self.chunk_grid[chunk_xy].particle_grid[local_xy]
        // &self.particle_grid[xy]
    }

    fn get_particle_mut(&mut self, xy: (usize, usize)) -> &mut Particle {
        let (chunk_xy, local_xy) = self.global_xy_to_chunk_xy(xy);
        // self.chunk_grid[chunk_xy].update_next_frame = true;

        self.wake_chunk_from(chunk_xy, local_xy);
        &mut self.chunk_grid[chunk_xy].particle_grid[local_xy]
        // &mut self.particle_grid[xy]
    }

    fn put_particle(&mut self, xy: (usize, usize), particle: Particle) {
        let (chunk_xy, local_xy) = self.global_xy_to_chunk_xy(xy);
        if self.chunk_grid[chunk_xy].particle_grid[local_xy] != particle {
            // self.chunk_grid[chunk_xy].update_next_frame = true;
            self.wake_chunk_from(chunk_xy, local_xy);
            self.chunk_grid[chunk_xy].particle_grid[local_xy] = particle;
        }
        // self.particle_grid[xy] = particle;
    }

    fn wake_chunk_from(&mut self, chunk_xy: (usize, usize), local_xy: (usize, usize)) {
        self.chunk_grid[chunk_xy].update_next_frame = true;

        let (chunk_x, chunk_y) = chunk_xy;
        let (local_x, local_y) = local_xy;
        if local_x <= 1 && chunk_x != 0 {
            self.chunk_grid[(chunk_x - 1, chunk_y)].update_next_frame = true;
        } else if local_x >= self.chunk_size - 2 && chunk_x != (self.width / self.chunk_size) - 1 {
            self.chunk_grid[(chunk_x + 1, chunk_y)].update_next_frame = true;
        }

        if local_y <= 0 && chunk_y != 0 {
            self.chunk_grid[(chunk_x, chunk_y - 1)].update_next_frame = true;
        } else if local_y >= self.chunk_size - 2 && chunk_y != (self.height / self.chunk_size) - 1 {
            self.chunk_grid[(chunk_x, chunk_y + 1)].update_next_frame = true;
        }
    }

    // fn put_particle_and_set_updated(&mut self, xy: (usize, usize), particle: Particle) {

    // }

    fn relative_particle(&self, xy: (usize, usize), dxdy: (isize, isize)) -> &Particle {
        self.get_particle(self.relative_xy(xy, dxdy))
    }

    fn relative_particle_mut(&mut self, xy: (usize, usize), dxdy: (isize, isize)) -> &mut Particle {
        self.get_particle_mut(self.relative_xy(xy, dxdy))
    }

    fn relative_xy(&self, xy: (usize, usize), dxdy: (isize, isize)) -> (usize, usize) {
        // dbg!(xy, dxdy);
        if let Some(portal) = &self.portal_grid[xy] {
            if let Some(xy2) = portal.partner_xy {
                // Might want a bit more logic to make this more comprehensive
                if dxdy.0 != 0 && dxdy.1 != 0 {
                    return self
                        .relative_xy(((xy.0 as isize + dxdy.0) as usize, xy.1), (0, dxdy.1));
                }

                let portal_dxdy = portal.direction.dxdy();
                // dbg!(xy2, portal_dxdy);
                if portal_dxdy.0 == dxdy.0 && portal_dxdy.1 == dxdy.1 {
                    return xy2;
                }
            }
        }

        (
            (xy.0 as isize + dxdy.0) as usize,
            (xy.1 as isize + dxdy.1) as usize,
        )
    }
}

// ───────────────────────────────────────────────────────────────────────────────────────────── ✣ ─
// Adapted from https://gist.github.com/DavidMcLaughlin208/60e69e698e3858617c322d80a8f174e2
// via https://www.youtube.com/watch?v=5Ka3tbbT-9E&list=WL&index=28&t=1112s
fn iterate_over_line_common<F>(dx: isize, dy: isize, mut inner_function: F)
where
    F: FnMut(isize, isize, isize, isize) -> bool,
{
    if dx == 0 && dy == 0 {
        inner_function(0, 0, 0, 0);
        return;
    }

    let dx_is_larger = dx.abs() > dy.abs();

    let x_modifier = if dx < 0 { 1 } else { -1 };
    let y_modifier = if dy < 0 { 1 } else { -1 };

    let longer_side_length = isize::max(dx.abs(), dy.abs());
    let shorter_side_length = isize::min(dx.abs(), dy.abs());

    let slope = if shorter_side_length == 0 || longer_side_length == 0 {
        0.0
    } else {
        shorter_side_length as f32 / longer_side_length as f32
    };

    let mut prev_x = 0;
    let mut prev_y = 0;
    let mut shorter_side_increase;
    for i in 1..=longer_side_length {
        shorter_side_increase = (i as f32 * slope).round() as isize;
        let (x_increase, y_increase) = if dx_is_larger {
            (i, shorter_side_increase)
        } else {
            (shorter_side_increase, i)
        };

        let current_x = x_increase * x_modifier;
        let current_y = y_increase * y_modifier;
        let delta_x = current_x - prev_x;
        let delta_y = current_y - prev_y;
        if !inner_function(current_x, current_y, delta_x, delta_y) {
            break;
        };
        prev_x = current_x;
        prev_y = current_y;
    }
}

pub fn iterate_over_line<F>(xy1: (usize, usize), xy2: (usize, usize), mut inner_function: F)
where
    F: FnMut(usize, usize),
{
    let dx = xy1.0 as isize - xy2.0 as isize;
    let dy = xy1.1 as isize - xy2.1 as isize;
    iterate_over_line_common(dx, dy, |delta_x, delta_y, _, _| {
        let current_x = (xy1.0 as isize + delta_x) as usize;
        let current_y = (xy1.1 as isize + delta_y) as usize;
        inner_function(current_x, current_y);
        true // always continue iteration
    });
}

pub fn iterate_over_line_delta<F>(dxdy: (isize, isize), mut inner_function: F)
where
    F: FnMut(isize, isize) -> bool,
{
    iterate_over_line_common(dxdy.0, dxdy.1, |_, _, dx, dy| inner_function(dx, dy));
}
