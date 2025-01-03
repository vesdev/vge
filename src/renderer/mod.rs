use crate::Result;
use thiserror::Error;
use wgpu::{CreateSurfaceError, SurfaceTarget, util::RenderEncoder};

pub enum Gfx<'a> {
    Wgpu(WgpuContext<'a>),
    None,
}

impl Gfx<'_> {
    pub(crate) fn render(&mut self) -> Result {
        match self {
            Gfx::Wgpu(wgpu_context) => wgpu_context.render(),
            Gfx::None => todo!(),
        }
    }
}

pub(crate) fn wgpu<'a>(target: impl Into<SurfaceTarget<'a>>, size: (u32, u32)) -> Result<Gfx<'a>> {
    let ctx = WgpuContext::new(target, size)?;
    Ok(Gfx::Wgpu(ctx))
}

pub(crate) struct WgpuContext<'a> {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'a>,
    surface_configured: bool,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
}

impl<'a> WgpuContext<'a> {
    fn new(target: impl Into<SurfaceTarget<'a>>, size: (u32, u32)) -> Result<Self> {
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

        let pipeline = Self::create_pipeline(&device, &config);

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            config,
            surface_configured: false,
            pipeline,
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
    ) -> Result<wgpu::Surface<'a>> {
        let surface = instance
            .create_surface(target.into())
            .map_err(RenderError::CreateSurface)?;
        Ok(surface)
    }

    fn create_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface) -> Result<wgpu::Adapter> {
        let adapter = smol::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: Default::default(),
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        }))
        .ok_or(RenderError::Adapter)?;
        Ok(adapter)
    }

    fn create_device(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue)> {
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
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(
                r#"
                    struct VertexOutput {
                        @builtin(position) clip_position: vec4<f32>,
                        @location(0) vert_pos: vec3<f32>,
                    }

                    @vertex
                    fn vs_main(
                        @builtin(vertex_index) in_vertex_index: u32,
                    ) -> VertexOutput {
                        var out: VertexOutput;
                        let x = f32(1 - i32(in_vertex_index)) * 0.5;
                        let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
                        out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
                        out.vert_pos = out.clip_position.xyz;
                        return out;
                    }

                    @fragment
                    fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                        return vec4<f32>(0.3, 0.2, 0.1, 1.0);
                    }
                "#
                .into(),
            ),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
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

    pub(crate) fn set_surface_size(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.surface_configured = true;
    }

    pub(crate) fn render(&mut self) -> Result {
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
