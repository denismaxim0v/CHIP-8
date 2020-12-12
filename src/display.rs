extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::consts::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::cpu::Cpu;

const SIZING: usize = 20;
// TODO: no idea what to do here yet
pub struct Display {
    value: bool,
}

impl Display {
    pub fn new() -> Self {
        Display { value: true }
    }
}
