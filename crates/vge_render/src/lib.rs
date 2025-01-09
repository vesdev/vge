use primitives::{Color, Position, Primitive, Quad, Triangle, Vertex};
use thiserror::Error;
use vge_math::{Rect, Vec2};
use wgpu::{
    CreateSurfaceError, ShaderModuleDescriptor, SurfaceTarget, include_wgsl,
    util::{DeviceExt, RenderEncoder},
};

mod primitives;

const DEFAULT_SHADER: ShaderModuleDescriptor =
    include_wgsl!("../../../assets/shaders/default.wgsl");

/// Graphical Context
pub enum Gfx<'a> {
    Wgpu(WgpuContext<'a>),
    None,
}

impl Gfx<'_> {
    pub fn render(&mut self) -> Result<(), RenderError> {
        match self {
            Gfx::Wgpu(wgpu_context) => wgpu_context.render(),
            Gfx::None => todo!(),
        }
    }
}

pub fn wgpu<'a>(
    target: impl Into<SurfaceTarget<'a>>,
    size: (u32, u32),
) -> Result<Gfx<'a>, RenderError> {
    let ctx = WgpuContext::new(target, size)?;
    Ok(Gfx::Wgpu(ctx))
}

pub struct WgpuContext<'a> {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'a>,
    surface_configured: bool,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    vbuf: wgpu::Buffer,
    ibuf: wgpu::Buffer,
    num_indices: u32,
}

impl<'a> WgpuContext<'a> {
    fn new(target: impl Into<SurfaceTarget<'a>>, size: (u32, u32)) -> Result<Self, RenderError> {
        let instance = Self::create_instance();
        let surface = Self::create_surface(&instance, target)?;
        let adapter = Self::create_adapter(&instance, &surface)?;
        let (device, queue) = Self::create_device(&adapter)?;

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
            width: size.0,
            height: size.1,
            present_mode: surface_caps.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        let shader = device.create_shader_module(DEFAULT_SHADER);
        let pipeline = Self::create_pipeline(&device, &config, &shader);

        let quad = Quad::new(Rect::new(Vec2::new(-0.5, -0.5), Vec2::new(0.5, 0.5)));

        let vbuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex bufer"),
            contents: bytemuck::bytes_of(&quad),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let ibuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(quad.indices().unwrap()),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = quad.indices().unwrap().len() as u32;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            config,
            surface_configured: false,
            pipeline,
            vbuf,
            ibuf,
            num_indices,
        })
    }

    fn create_instance() -> wgpu::Instance {
        wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        })
    }

    fn create_surface(
        instance: &wgpu::Instance,
        target: impl Into<SurfaceTarget<'a>>,
    ) -> Result<wgpu::Surface<'a>, RenderError> {
        let surface = instance
            .create_surface(target.into())
            .map_err(RenderError::CreateSurface)?;
        Ok(surface)
    }

    fn create_adapter(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface,
    ) -> Result<wgpu::Adapter, RenderError> {
        let adapter = smol::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: Default::default(),
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        }))
        .ok_or(RenderError::Adapter)?;
        Ok(adapter)
    }

    fn create_device(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue), RenderError> {
        let result = smol::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            None,
        ))
        .map_err(RenderError::Device)?;
        Ok(result)
    }

    fn create_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        })
    }

    pub fn set_surface_size(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.surface_configured = true;
    }

    pub(crate) fn render(&mut self) -> Result<(), RenderError> {
        if !self.surface_configured {
            return Ok(());
        }

        let output = self
            .surface
            .get_current_texture()
            .map_err(RenderError::Surface)?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_index_buffer(self.ibuf.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_vertex_buffer(0, self.vbuf.slice(..));
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

/// Generic wgpu texture
pub struct WgpuTexture {}

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("failed to create graphics backend")]
    CreateSurface(#[from] CreateSurfaceError),
    #[error("no compatitible adapters were found")]
    Adapter,
    #[error("could not request a graphics device")]
    Device(#[from] wgpu::RequestDeviceError),
    #[error("could fetch window surface")]
    Surface(#[from] wgpu::SurfaceError),
}
