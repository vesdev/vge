use std::path::PathBuf;

use vge_math::{Rect, Vec2};
use wgpu::util::DeviceExt;

use crate::{
    Gfx, RenderError,
    primitives::{Primitive, Quad, VertexTextured},
};

pub struct TexturedQuad {
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) vtx_buf: wgpu::Buffer,
    pub(crate) idx_buf: wgpu::Buffer,
    pub(crate) quad: Quad<VertexTextured>,
}

impl TexturedQuad {
    pub fn new(gfx: &Gfx<'_>, bytes: &[u8], label: &str) -> Result<Self, RenderError> {
        Self::from_bytes(&gfx.device, &gfx.queue, bytes, label)
    }

    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self, RenderError> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Self, RenderError> {
        let image = img;
        let rgba = image.to_rgba8();

        let dimensions = rgba.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let quad = Quad::textured(Rect::new(Vec2::new(-0.5, -0.5), Vec2::new(0.5, 0.5)));

        let vtx_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex bufer"),
            contents: bytemuck::bytes_of(&quad.vertices()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let idx_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(quad.indices().unwrap()),
            usage: wgpu::BufferUsages::INDEX,
        });

        Ok(Self {
            texture,
            view,
            sampler,
            vtx_buf,
            idx_buf,
            quad,
        })
    }
}

pub struct Sprite {
    pub(crate) texture: TexturedQuad,
}

impl Sprite {
    pub fn new(gfx: &Gfx, path: PathBuf) -> Self {
        let bytes = std::fs::read(path).unwrap();
        Self {
            texture: TexturedQuad::new(gfx, &bytes, "Sprite").unwrap(),
        }
    }
}
// TODO: Make meshes work
pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}
