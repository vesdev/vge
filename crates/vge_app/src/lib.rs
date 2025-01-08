use thiserror::Error;
use vge_core::{CreateFn, DrawFn, StepFn};
use vge_window::WindowError;

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

// #[derive(Default)]
pub struct App<S> {
    create_fn: Option<CreateFn<S>>,
    step_fn: Option<StepFn<S>>,
    draw_fn: Option<DrawFn<S>>,
}

impl<T> Default for App<T> {
    fn default() -> Self {
        // todo!()
        Self {
            create_fn: None,
            step_fn: None,
            draw_fn: None,
        }
    }
}

impl<T> App<T> {
    pub fn create(mut self, cb: CreateFn<T>) -> Self {
        self.create_fn = Some(cb);
        self
    }

    pub fn draw(mut self, cb: DrawFn<T>) -> Self {
        self.draw_fn = Some(cb);
        self
    }

    pub fn step(mut self, cb: StepFn<T>) -> Self {
        self.step_fn = Some(cb);
        self
    }

    pub fn run(&mut self) -> Result<(), AppError> {
        let mut window = vge_window::winit((640, 480), self.draw_fn.take().unwrap())?;
        window.run()?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Window(#[from] WindowError),
}
