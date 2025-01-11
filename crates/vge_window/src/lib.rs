use std::sync::{Arc, mpsc};

use thiserror::Error;
use tracing::info;
use vge_app::{App, Ctx};
use vge_render::Gfx;
use winit::{
    application::ApplicationHandler, error::EventLoopError, event::WindowEvent,
    event_loop::ControlFlow,
};

pub trait Window {
    fn size(&self) -> (u32, u32);
}

pub fn winit<'a, A: App>(size: (u32, u32), app: A) -> Result<WindowBackend<'a, A>, WindowError> {
    let winit = WinitWindow::new(size, app)?;
    Ok(WindowBackend::Winit(winit))
}

pub enum WindowBackend<'a, A: App> {
    Winit(WinitWindow<'a, A>),
}

impl<A: App> WindowBackend<'_, A> {
    pub fn run(&mut self) -> Result<(), WindowError> {
        match self {
            WindowBackend::Winit(winit) => winit.run(),
        }
    }
}

pub struct WinitWindow<'a, A: App> {
    pub size: (u32, u32),
    pub gfx: Option<Gfx<'a>>,
    pub window: Option<Arc<winit::window::Window>>,
    pub draw_receiver: mpsc::Receiver<i32>,
    pub draw_sender: mpsc::Sender<i32>,
    pub app: Option<A>,
}

impl<A: App> Window for WinitWindow<'_, A> {
    fn size(&self) -> (u32, u32) {
        self.size
    }
}

impl<A: App> WinitWindow<'_, A> {
    fn new(size: (u32, u32), app: A) -> Result<Self, WindowError> {
        let (draw_sender, draw_receiver) = std::sync::mpsc::channel();
        Ok(Self {
            window: None,
            size,
            gfx: None,
            draw_receiver,
            draw_sender,
            app: Some(app),
        })
    }

    pub(crate) fn run(&mut self) -> Result<(), WindowError> {
        let event_loop = winit::event_loop::EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);

        // let sender = self.draw_sender.clone();
        // let ctx = Ctx::new(gfx);

        event_loop.run_app(self)?;
        Ok(())
    }
}

impl<A: App> ApplicationHandler for WinitWindow<'_, A> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = winit::window::Window::default_attributes()
            .with_title("forsen")
            .with_visible(true);

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let mut gfx = vge_render::wgpu(window.clone(), self.size).unwrap();
        gfx.set_surface_size(window.inner_size().width, window.inner_size().height);

        if let Some(mut app) = self.app.take() {
            let mut ctx = Ctx::new(self.draw_sender.clone());
            app.init(&mut ctx, &mut gfx);

            std::thread::spawn(move || {
                app.step(&mut ctx);
            });
        }

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
                let Some(gfx) = &mut self.gfx else {
                    return;
                };

                gfx.render(&[]).unwrap();
            }
            WindowEvent::Resized(size) => {
                let Some(gfx) = &mut self.gfx else {
                    return;
                };

                gfx.set_surface_size(size.width, size.height);
            }
            _ => (),
        }
    }
}

#[derive(Error, Debug)]
pub enum WindowError {
    #[error(transparent)]
    EventLoop(#[from] EventLoopError),
}
