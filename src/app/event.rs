pub type EventLoop = fn(Event);
pub enum Event {
    Step,
    Draw,
}

#[derive(Default)]
pub struct EventHandler {
    pub step: Option<EventLoop>,
    pub draw: Option<EventLoop>,
}
