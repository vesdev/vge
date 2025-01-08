use primitives::{Color, Position, Triangle, Vertex};
use thiserror::Error;
use wgpu::{
    CreateSurfaceError, SurfaceTarget, include_wgsl,
    util::{DeviceExt, RenderEncoder},
};

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

        //TODO: use assets path relative to root
        let shader =
            device.create_shader_module(include_wgsl!("./../../../assets/shaders/default.wgsl"));
        let pipeline = Self::create_pipeline(&device, &config, &shader);

        let trig = Triangle(
            Vertex {
                position: Position {
                    x: 0.0,
                    y: 0.5,
                    z: 0.0,
                },
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 1.0,
                },
            },
            Vertex {
                position: Position {
                    x: -0.5,
                    y: -0.5,
                    z: 0.0,
                },
                color: Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                },
            },
            Vertex {
                position: Position {
                    x: 0.5,
                    y: -0.5,
                    z: 0.0,
                },
                color: Color {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                },
            },
        );

        let vbuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex bufer"),
            contents: bytemuck::bytes_of(&trig),
            usage: wgpu::BufferUsages::VERTEX,
        });

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
            render_pass.set_vertex_buffer(0, self.vbuf.slice(..));
            render_pass.draw(0..3, 0..1);
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

pub mod primitives {
    use bytemuck::{Pod, Zeroable};
    use wgpu::VertexAttribute;

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Zeroable, Pod)]
    pub struct Position {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Zeroable, Pod)]
    pub struct Color {
        pub r: f32,
        pub b: f32,
        pub g: f32,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Zeroable, Pod)]
    pub struct Vertex {
        pub position: Position,
        pub color: Color,
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

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Zeroable, Pod)]
    pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);
}
