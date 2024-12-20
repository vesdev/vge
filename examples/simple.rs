use vge::prelude::*;

fn main() -> Result<(), vge::Error> {
    tracing_subscriber::fmt::init();
    let options = Options {
        window: Window::Winit,
        renderer: Renderer::Wgpu,
    };

    vge::run(options, Simple::default())
}

#[derive(Default)]
struct Simple {}

impl App for Simple {
    fn create() {
        todo!()
    }

    fn step() {
        todo!()
    }

    fn draw(gfx: Gfx) {
        todo!()
    }

    fn event(event: Event) {
        todo!()
    }
}
