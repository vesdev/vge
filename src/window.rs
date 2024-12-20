use crate::Result;
use tracing::info;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ControlFlow};

#[derive(Default)]
pub enum Window {
    #[default]
    Winit,
}

pub(crate) trait WindowBackend {
    fn create_window(&mut self) -> Result;
}

#[derive(Default)]
pub(crate) struct WinitBackend {
    window: Option<winit::window::Window>,
}

impl WindowBackend for WinitBackend {
    fn create_window(&mut self) -> Result {
        let event_loop = winit::event_loop::EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self)?;
        Ok(())
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

pub fn open(window: Window) -> Result {
    let mut be = match window {
        Window::Winit => WinitBackend::default(),
    };

    be.create_window()?;
    Ok(())
}
