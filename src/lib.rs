use thiserror::Error;
use vge_app::App;
use vge_window::WindowError;

pub mod prelude {
    pub use vge_app::options::*;
    pub use vge_app::*;
    pub use vge_math::*;
    pub use vge_render::mesh;
    pub use vge_render::*;
}

pub fn run(app: impl App) -> Result<(), Error> {
    let mut window = vge_window::winit((640, 480), app)?;
    window.run()?;
    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Window(#[from] WindowError),
}
