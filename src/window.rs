use std::sync::Arc;

use crate::{
    Error, Result,
    app::{App, Ctx, DrawFn},
    prelude::Gfx,
    renderer,
};
use tracing::info;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ControlFlow};

pub(crate) trait Window {
    fn size(&self) -> (u32, u32);
}

pub fn winit<'a, S>(size: (u32, u32), draw: DrawFn<S>) -> Result<WindowBackend<'a, S>, Error> {
    let winit = WinitWindow::new(size, draw)?;
    Ok(WindowBackend::Winit(winit))
}

pub(crate) enum WindowBackend<'a, S> {
    Winit(WinitWindow<'a, S>),
}

impl<S> WindowBackend<'_, S> {
    pub(crate) fn run(&mut self) -> Result {
        match self {
            WindowBackend::Winit(winit) => winit.run(),
        }
    }
}

pub(crate) struct WinitWindow<'a, S> {
    pub size: (u32, u32),
    pub draw: DrawFn<S>,
    pub gfx: Option<Gfx<'a>>,
    pub window: Option<Arc<winit::window::Window>>,
}

impl<S> Window for WinitWindow<'_, S> {
    fn size(&self) -> (u32, u32) {
        self.size
    }
}

impl<S> WinitWindow<'_, S> {
    fn new(size: (u32, u32), draw: DrawFn<S>) -> Result<Self> {
        Ok(Self {
            window: None,
            size,
            draw,
            gfx: None,
        })
    }

    pub(crate) fn run(&mut self) -> Result {
        let event_loop = winit::event_loop::EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self)?;
        Ok(())
    }
}

impl<S> ApplicationHandler for WinitWindow<'_, S> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = winit::window::Window::default_attributes()
            .with_title("forsen")
            .with_visible(true);

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let mut gfx = renderer::wgpu(window.clone(), self.size).unwrap();
        if let Gfx::Wgpu(wgpu) = &mut gfx {
            wgpu.set_surface_size(window.inner_size().width, window.inner_size().height);
        };
        self.gfx = Some(gfx);
        self.window = Some(window);
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

                if let Some(gfx) = &mut self.gfx {
                    let ctx = Ctx::new(gfx);
                    // (self.draw)(ctx);

                    gfx.render().unwrap();
                }
            }
            WindowEvent::Resized(size) => {
                let Some(gfx) = &mut self.gfx else {
                    return;
                };
                let Gfx::Wgpu(wgpu) = gfx else {
                    return;
                };

                wgpu.set_surface_size(size.width, size.height);
            }
            _ => (),
        }
    }
}
