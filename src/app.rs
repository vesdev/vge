use crate::prelude::{Gfx, Options};
use crate::{Result, window};

pub mod event;

pub type CreateFn<S> = fn(&mut Ctx) -> S;
pub type StepFn<S> = fn(&mut Ctx, S);
pub type DrawFn<S> = fn(&mut Ctx, S);

pub struct Ctx<'a> {
    gfx: &'a Gfx<'a>,
}

impl<'a> Ctx<'a> {
    pub(crate) fn new(gfx: &'a Gfx<'a>) -> Self {
        Self { gfx }
    }

    pub fn create_text(&self, text: &str) -> Text {
        Text { text: text.into() }
    }

    pub fn draw<T>(&mut self, mesh: T) {}
}

// TODO: remove this is temporary
pub struct Text {
    text: String,
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

    pub fn run(&mut self) -> Result {
        let mut window = window::winit((640, 480), self.draw_fn.take().unwrap())?;
        window.run()
    }
}
