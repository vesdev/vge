use vge::prelude::*;

fn main() {
    tracing_subscriber::fmt::init();

    vge::run(Simple::default()).unwrap();
}

#[derive(Default)]
pub struct Simple {}

impl App for Simple {
    fn init(&mut self, ctx: &mut Ctx, gfx: &mut Gfx) {
        // todo!()
    }

    fn step(&mut self, ctx: &mut Ctx) {
        // todo!()
    }
}
