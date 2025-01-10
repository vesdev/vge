use bytemuck::{Pod, Zeroable};
use vge_math::{Rect, Vec2, Vec3};
use wgpu::VertexAttribute;

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct Color {
    pub r: f32,
    pub b: f32,
    pub g: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct VertexColored {
    pub position: Vec3,
    pub color: Color,
}

impl VertexColored {
    pub fn new(pos: Vec3, col: Color) -> Self {
        Self {
            position: pos,
            color: col,
        }
    }
}

impl From<(Vec3, Color)> for VertexColored {
    fn from(value: (Vec3, Color)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Vertex for VertexColored {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: &[VertexAttribute] =
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexColored>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: ATTRIBUTES,
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct VertexTextured {
    pub position: Vec3,
    pub tex_coords: Vec2,
}

impl VertexTextured {
    pub fn new(mut pos: Vec3, tex_coords: Vec2) -> Self {
        Self {
            position: pos,
            tex_coords,
        }
    }
}

impl From<(Vec3, Vec2)> for VertexTextured {
    fn from(value: (Vec3, Vec2)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Vertex for VertexTextured {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: &[VertexAttribute] =
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexTextured>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: ATTRIBUTES,
        }
    }
}

pub trait Vertex: Clone + Copy + Zeroable + Pod {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

pub trait Primitive<const VS: usize, const IS: usize> {
    type T;
    fn vertices(&self) -> VertexBuffer<VS, Self::T>;
    fn indices(&self) -> Option<&'static IndexBuffer<IS>>;
}

pub type VertexBuffer<const N: usize, V> = [V; N];
pub type IndexBuffer<const N: usize> = [u16; N];

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Triangle<V: Vertex>(pub V, pub V, pub V);

impl<V: Vertex> Primitive<3, 0> for Triangle<V> {
    type T = V;
    fn vertices(&self) -> VertexBuffer<3, Self::T> {
        [self.0, self.1, self.2]
    }

    // NOTE: for dynamically created primitives we might want to remove static
    fn indices(&self) -> Option<&'static IndexBuffer<0>> {
        None
    }
}

impl<V: Vertex> From<[V; 3]> for Triangle<V> {
    fn from(value: [V; 3]) -> Self {
        Self(value[0], value[1], value[2])
    }
}

impl From<[(Vec3, Color); 3]> for Triangle<VertexColored> {
    fn from(value: [(Vec3, Color); 3]) -> Self {
        Self(value[0].into(), value[1].into(), value[2].into())
    }
}

impl From<[(Vec3, Vec2); 3]> for Triangle<VertexTextured> {
    fn from(value: [(Vec3, Vec2); 3]) -> Self {
        Self(value[0].into(), value[1].into(), value[2].into())
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Quad<V: Vertex> {
    vertices: VertexBuffer<4, V>,
}

// TODO: temporary
impl Quad<VertexColored> {
    pub fn colored(rect: Rect) -> Self {
        let vertices: [VertexColored; 4] = [
            (
                Vec3::new(rect.min.x, rect.min.y, 0.0),
                Color::new(0.0, 0.0, 1.0),
            )
                .into(),
            (
                Vec3::new(rect.max.x, rect.min.y, 0.0),
                Color::new(1.0, 0.0, 0.0),
            )
                .into(),
            (
                Vec3::new(rect.min.x, rect.max.y, 0.0),
                Color::new(0.0, 1.0, 0.0),
            )
                .into(),
            (
                Vec3::new(rect.max.x, rect.max.y, 0.0),
                Color::new(0.0, 1.0, 0.0),
            )
                .into(),
        ];

        Self { vertices }
    }
}
impl Quad<VertexTextured> {
    pub fn textured(rect: Rect) -> Self {
        #[rustfmt::skip]
        let vertices: [VertexTextured; 4] = [
            (
                Vec3::new(rect.min.x, rect.min.y, 0.0),
                Vec2::new(0.0, 1.0),
            )
                .into(),
            (
                Vec3::new(rect.max.x, rect.min.y, 0.0),
                Vec2::new(1.0, 1.0),
            )
                .into(),
            (
                Vec3::new(rect.min.x, rect.max.y, 0.0),
                Vec2::new(0.0, 0.0),
            )
                .into(),
            (
                Vec3::new(rect.max.x, rect.max.y, 0.0),
                Vec2::new(1.0, 0.0),
            )
                .into(),
        ];

        Self { vertices }
    }
}

impl<V: Vertex> Primitive<4, 6> for Quad<V> {
    type T = V;
    fn vertices(&self) -> VertexBuffer<4, Self::T> {
        self.vertices
    }

    fn indices(&self) -> Option<&'static IndexBuffer<6>> {
        Some(&[0, 1, 2, 2, 1, 3])
    }
}
