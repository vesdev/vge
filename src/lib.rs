use app::App;
use options::Options;
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
    pub use crate::window::*;
}

pub fn run(opt: Options, app: impl App) -> Result {
    window::open(opt.window)
}

pub mod options {
    use crate::{renderer::Renderer, window::Window};

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
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
