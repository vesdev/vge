use vge::prelude::*;

fn main() -> Result<(), vge::Error> {
    tracing_subscriber::fmt::init();
    let options = Options {
        window: Window::Winit,
        renderer: Renderer::Wgpu,
    };

    smol::block_on(vge::run(options, Simple::default()))
}

#[derive(Default)]
struct Simple {}

impl App for Simple {
    fn create(&mut self, gfx: &mut Gfx) {
        // self.surf = Some(gfx.surface_create());
        todo!()
    }

    fn step(&mut self) {
        todo!()
    }

    fn draw(&mut self, gfx: &mut Gfx) {
        // gfx.surface_set_target(self.surf.as_mut().unwrap(), |gfx| {
        //     //TODO: draw something
        // });

        todo!()
    }

    fn event(&mut self, event: Event) {
        todo!()
    }
}
