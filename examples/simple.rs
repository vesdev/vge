use vge::prelude::*;

fn main() {
    tracing_subscriber::fmt::init();

    App::default()
        .create(create)
        .step(step)
        .draw(draw)
        .run()
        .unwrap();
}

pub struct State {
    text: mesh::Text,
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
