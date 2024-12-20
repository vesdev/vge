use event::Event;

use crate::prelude::Gfx;

pub mod event;

pub trait App {
    fn create(&mut self, gfx: &mut Gfx);
    fn step(&mut self);
    fn draw(&mut self, gfx: &mut Gfx);
    fn event(&mut self, event: Event);
}

#[allow(unreachable_code, unused_variables)]
pub(crate) fn draw(app: &mut impl App, gfx: &mut Gfx) {
    let target = todo!();

    gfx.set_target(target, |gfx| {
        app.draw(gfx);
    });
}
