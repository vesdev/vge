use event::Event;

use crate::prelude::Gfx;

pub mod event;

pub trait App {
    fn create();
    fn step();
    fn draw(gfx: Gfx);
    fn event(event: Event);
}
