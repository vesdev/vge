use std::error::Error;

use crate::{
    Result,
    window::{Window, WindowBackend},
};
use thiserror::Error;
use wgpu::{CreateSurfaceError, SurfaceTarget};

pub struct Gfx {
    application_surface: Texture,
}

impl Gfx {
    pub async fn new(window: Window) -> Self {
        let ctx = WgpuContext::new(window);
        Self {
            application_surface: todo!(),
        }
    }

    pub fn set_target(&mut self, target: &mut Texture, draw: impl FnOnce(&mut Gfx)) {
        //TODO: set surface as target
        draw(self);
    }

    pub fn clear(&mut self) {}
}

pub enum Texture {}

pub enum Context<'a> {
    Wgpu(WgpuContext<'a>),
}

struct WgpuContext<'a> {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'a>,
    config: wgpu::SurfaceConfiguration,
}

impl<'a> WgpuContext<'a> {
    async fn new(window: Window) -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance
            .create_surface(window.backend)
            .map_err(GraphicsError::Init)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(GraphicsError::NoCompatitibleAdapter)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .map_err(GraphicsError::RequstDevice)?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.size.0,
            height: window.size.1,
            present_mode: surface_caps.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            config,
        })
    }

    fn create_surface() -> WgpuTexture {
        WgpuTexture {}
    }
}

/// Generic wgpu texture
pub struct WgpuTexture {}

#[derive(Error, Debug)]
pub enum GraphicsError {
    #[error("failed to create graphics backend")]
    Init(#[from] CreateSurfaceError),
    #[error("no compatitible adapters were found")]
    NoCompatitibleAdapter,
    #[error("could not request a graphics device")]
    RequstDevice(#[from] wgpu::RequestDeviceError),
}
