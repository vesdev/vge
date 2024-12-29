use vge::prelude::*;

#[rustfmt::skip]
fn main() -> vge::Result {
    tracing_subscriber::fmt::init();

    App::default()
        .create(create)
        .step(step)
        .draw(draw)
        .run()
}

pub struct State {
    text: Text,
}

/// Create event
fn create(ctx: &mut Ctx) -> State {
    //TODO: create some meshes here
    let text = ctx.create_text("forsen");

    State { text }
}

/// Step event
fn step(ctx: &mut Ctx, state: State) {
    //TODO: some simple game logic
}

/// Draw event
fn draw(ctx: &mut Ctx, state: State) {
    //TODO: draw something simple
    ctx.draw(state.text);
}
