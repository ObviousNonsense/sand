use std::ops::{Add, AddAssign, Sub, SubAssign};

// ─── I8vec2 ────────────────────────────────────────────────────────────────────────────────── ✣ ─
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct I8Vec2 {
    pub x: i8,
    pub y: i8,
}

#[inline(always)]
pub fn i8vec2<Tx, Ty>(x: Tx, y: Ty) -> I8Vec2
where
    i8: From<Tx>,
    i8: From<Ty>,
{
    I8Vec2::new(x.into(), y.into())
}

pub fn i8vec2_vector<C, T>(input_collection: C) -> Vec<I8Vec2>
where
    C: IntoIterator<Item = T>,
    I8Vec2: From<T>,
{
    input_collection.into_iter().map(|x| x.into()).collect()
}

pub const UP: I8Vec2 = I8Vec2::NEG_Y;
pub const DN: I8Vec2 = I8Vec2::Y;
pub const LEFT: I8Vec2 = I8Vec2::NEG_X;
pub const RIGHT: I8Vec2 = I8Vec2::X;
pub const UP_L: I8Vec2 = I8Vec2::new(-1, -1);
pub const UP_R: I8Vec2 = I8Vec2::new(1, -1);
pub const DN_L: I8Vec2 = I8Vec2::new(-1, 1);
pub const DN_R: I8Vec2 = I8Vec2::new(1, 1);

// Copied from glam IVec2
impl I8Vec2 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    /// All ones.
    pub const ONE: Self = Self::splat(1);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1);

    /// A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self::new(1, 0);

    /// A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0, 1);

    /// A unit-length vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1, 0);

    /// A unit-length vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0, -1);

    /// The unit axes.
    pub const AXES: [Self; 2] = [Self::X, Self::Y];

    /// Creates a new vector.
    #[inline(always)]
    pub const fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    pub const fn splat(v: i8) -> Self {
        Self { x: v, y: v }
    }

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    /// Component-wise clamping of values, similar to [`i32::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        assert!(
            min.x <= max.x && min.y <= max.y,
            "clamp: expected min <= max"
        );
        self.max(min).min(max)
    }

    pub fn length_sq(&self) -> u16 {
        (self.x as i16 * self.x as i16 + self.y as i16 * self.y as i16) as u16
    }
}

impl From<I8Vec2> for (i16, i16) {
    fn from(value: I8Vec2) -> Self {
        (value.x as i16, value.y as i16)
    }
}

impl<T> From<(T, T)> for I8Vec2
where
    i8: From<T>,
{
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

impl Add<I8Vec2> for I8Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl AddAssign<I8Vec2> for I8Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl Add<i8> for I8Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: i8) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}

impl AddAssign<i8> for I8Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: i8) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
    }
}

impl Add<I8Vec2> for i8 {
    type Output = I8Vec2;
    #[inline]
    fn add(self, rhs: I8Vec2) -> I8Vec2 {
        I8Vec2 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}

impl Sub<I8Vec2> for I8Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl SubAssign<I8Vec2> for I8Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: I8Vec2) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl Sub<i8> for I8Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: i8) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}

impl SubAssign<i8> for I8Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: i8) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
    }
}

impl Sub<I8Vec2> for i8 {
    type Output = I8Vec2;
    #[inline]
    fn sub(self, rhs: I8Vec2) -> I8Vec2 {
        I8Vec2 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}

impl std::fmt::Debug for I8Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I8Vec2 {{ x: {}, y: {}}}", self.x, self.y)
    }
}

// ─── Pcolor ────────────────────────────────────────────────────────────────────────────────── ✣ ─
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl std::fmt::Debug for PColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PColor {{ r: {}, g: {}, b: {} }}",
            self.r, self.g, self.b
        )
    }
}

impl PColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    // ChatGPT wrote these methods. "The formula used in the implementation is
    // based on the description provided in the Wikipedia article on HSL and
    // HSV."
    fn into_hsv(self) -> (f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;
        let cmax = r.max(g).max(b);
        let cmin = r.min(g).min(b);
        let delta = cmax - cmin;
        let h = if delta == 0.0 {
            0.0
        } else if cmax == r {
            60.0 * ((g - b) / delta % 6.0)
        } else if cmax == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };
        let s = if cmax == 0.0 { 0.0 } else { delta / cmax };
        let v = cmax;
        (h, s, v)
    }

    fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        PColor {
            r: ((r + m) * 255.0) as u8,
            g: ((g + m) * 255.0) as u8,
            b: ((b + m) * 255.0) as u8,
        }
    }

    pub fn scale_hsv(&self, rotate_h: f32, scale_s: f32, scale_v: f32) -> Self {
        let (h, s, v) = self.into_hsv();
        let h = (h + rotate_h) % 360.0;
        let s = (s * scale_s).clamp(0.0, 1.0);
        let v = (v * scale_v).clamp(0.0, 1.0);
        PColor::from_hsv(h, s, v)
    }

    pub fn add_hsv(&self, rotate_h: f32, add_s: f32, add_v: f32) -> Self {
        let (h, s, v) = self.into_hsv();
        let h = (h + rotate_h) % 360.0;
        let s = (s + add_s).clamp(0.0, 1.0);
        let v = (v + add_v).clamp(0.0, 1.0);
        PColor::from_hsv(h, s, v)
    }
}
