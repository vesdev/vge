use bytemuck::{Pod, Zeroable};
use vge_math::Rect;
use wgpu::VertexAttribute;

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

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
pub struct Vertex {
    pub position: Position,
    pub color: Color,
}

impl Vertex {
    pub fn new(pos: Position, col: Color) -> Self {
        Self {
            position: pos,
            color: col,
        }
    }
}

impl From<(Position, Color)> for Vertex {
    fn from(value: (Position, Color)) -> Self {
        Self {
            position: value.0,
            color: value.1,
        }
    }
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: &[VertexAttribute] =
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: ATTRIBUTES,
        }
    }
}

pub trait Primitive<const VS: usize, const IS: usize> {
    fn vertices(&self) -> VertexBuffer<VS>;
    fn indices(&self) -> Option<&'static IndexBuffer<IS>>;
}

pub type VertexBuffer<const N: usize> = [Vertex; N];
pub type IndexBuffer<const N: usize> = [u16; N];

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);

impl Primitive<3, 0> for Triangle {
    fn vertices(&self) -> VertexBuffer<3> {
        [self.0, self.1, self.2]
    }

    fn indices(&self) -> Option<&'static IndexBuffer<0>> {
        None
    }
}

impl From<[Vertex; 3]> for Triangle {
    fn from(value: [Vertex; 3]) -> Self {
        Self(value[0], value[1], value[2])
    }
}

impl From<[(Position, Color); 3]> for Triangle {
    fn from(value: [(Position, Color); 3]) -> Self {
        Self(value[0].into(), value[1].into(), value[2].into())
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct Quad {
    vertices: VertexBuffer<4>,
}

impl Quad {
    pub fn new(rect: Rect) -> Self {
        let vertices = [
            (
                Position::new(rect.min.x, rect.min.y, 0.0),
                Color::new(0.0, 0.0, 1.0),
            )
                .into(),
            (
                Position::new(rect.max.x, rect.min.y, 0.0),
                Color::new(1.0, 0.0, 0.0),
            )
                .into(),
            (
                Position::new(rect.min.x, rect.max.y, 0.0),
                Color::new(0.0, 1.0, 0.0),
            )
                .into(),
            (
                Position::new(rect.max.x, rect.max.y, 0.0),
                Color::new(0.0, 1.0, 0.0),
            )
                .into(),
        ];

        Self { vertices }
    }
}

impl Primitive<4, 6> for Quad {
    fn vertices(&self) -> VertexBuffer<4> {
        self.vertices
    }

    fn indices(&self) -> Option<&'static IndexBuffer<6>> {
        Some(&[0, 1, 2, 2, 1, 3])
    }
}
