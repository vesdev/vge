use std::ops::Add;

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self::splat(0.0);

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self::new(v, v)
    }

    #[inline]
    pub fn dot(&self, rhs: Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        Self::new(self.x / len, self.y / len)
    }

    #[inline]
    pub fn normalize_or_zero(&self) -> Self {
        self.normalize_or(Self::ZERO)
    }

    #[inline]
    pub fn normalize_or(&self, fallback: Self) -> Self {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        if len > 0.0 {
            Self::new(self.x / len, self.y / len)
        } else {
            fallback
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self::splat(0.0);

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self::new(v, v, v)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }
}
