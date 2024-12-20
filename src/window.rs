use crate::{Error, Result};
use tracing::info;
use wgpu::SurfaceTarget;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ControlFlow};

pub(crate) struct Window {
    pub(crate) backend: WindowBackend,
    pub size: (u32, u32),
}

impl Window {
    pub fn new(size: (u32, u32)) -> Result<Window, Error> {
        let winit = WinitBackend::new(size)?;
        let window = Window {
            backend: WindowBackend::Winit(winit),
            size,
        };
        Ok(window)
    }
}

impl<'window> Into<SurfaceTarget<'window>> for WindowBackend {
    fn into(self) -> SurfaceTarget<'window> {
        match self {
            WindowBackend::Winit(winit_backend) => winit_backend.window.unwrap().into(),
        }
    }
}

pub(crate) enum WindowBackend {
    Winit(WinitBackend),
}

pub(crate) struct WinitBackend {
    window: Option<winit::window::Window>,
    event_loop: winit::event_loop::EventLoop<()>,
    size: (u32, u32),
}

impl WinitBackend {
    fn new(size: (u32, u32)) -> Result<Self> {
        let event_loop = winit::event_loop::EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        // event_loop.run_app(self)?;
        Ok(Self {
            window: None,
            event_loop,
            size,
        })
    }
}

impl ApplicationHandler for WinitBackend {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = winit::window::Window::default_attributes()
            .with_title("forsen")
            .with_visible(true);

        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                info!("Window was closed!");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
