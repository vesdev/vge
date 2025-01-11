use thiserror::Error;

use std::{path::PathBuf, str::FromStr, sync::mpsc};

use vge_render::{Gfx, mesh};

pub mod options {
    #[derive(Default)]
    pub enum Window {
        #[default]
        Winit,
    }

    #[derive(Default)]
    pub enum Renderer {
        #[default]
        Wgpu,
    }

    #[derive(Default)]
    pub struct Options {
        pub window: Window,
        pub renderer: Renderer,
    }
}

#[derive(Clone)]
pub struct Ctx {
    sender: mpsc::Sender<i32>,
}

impl Ctx {
    pub fn new(sender: mpsc::Sender<i32>) -> Self {
        Self { sender }
    }
}

pub trait App: Send + Sync + 'static {
    fn init(&mut self, ctx: &mut Ctx, gfx: &mut Gfx);
    fn step(&mut self, ctx: &mut Ctx);
}
