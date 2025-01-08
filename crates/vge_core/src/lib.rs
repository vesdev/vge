use vge_render::Gfx;

pub type CreateFn<S> = fn(&mut Ctx) -> S;
pub type StepFn<S> = fn(&mut Ctx, S);
pub type DrawFn<S> = fn(&mut Ctx, S);

pub struct Ctx<'a> {
    gfx: &'a Gfx<'a>,
}

impl<'a> Ctx<'a> {
    pub fn new(gfx: &'a Gfx<'a>) -> Self {
        Self { gfx }
    }

    pub fn create_text(&self, text: impl Into<String>) -> mesh::Text {
        mesh::Text::new(text)
    }

    pub fn draw<T>(&mut self, mesh: T) {}
}

pub mod mesh {
    // TODO: Make meshes work
    pub struct Text {
        text: String,
    }

    impl Text {
        pub fn new(text: impl Into<String>) -> Self {
            Self { text: text.into() }
        }
    }
}
