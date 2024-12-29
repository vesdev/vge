use prelude::RenderError;
use thiserror::Error;
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
    Graphics(#[from] RenderError),
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
