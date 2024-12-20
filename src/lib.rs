use app::App;
use options::Options;
use prelude::{Gfx, GraphicsError};
use thiserror::Error;
use window::Window;
use winit::error::EventLoopError;

mod app;
mod renderer;
mod window;

pub mod prelude {
    pub use crate::app::event::Event;
    pub use crate::app::*;
    pub use crate::options::*;
    pub use crate::renderer::*;
}

pub async fn run(opt: Options, mut app: impl App) -> Result {
    let size = (640, 480);
    let window = Window::new(size)?;
    let mut gfx = Gfx::new(window).await;

    app::draw(&mut app, &mut gfx);
    Ok(())
}

pub mod options {
    #[derive(Default)]
    pub enum Window {
        #[default]
        Winit,
    }

    #[derive(Default)]
    pub enum Renderer {
        #[default]
        Wgpu,
    }

    #[derive(Default)]
    pub struct Options {
        pub window: Window,
        pub renderer: Renderer,
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Window(#[from] EventLoopError),
    #[error(transparent)]
    Graphics(#[from] GraphicsError),
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
