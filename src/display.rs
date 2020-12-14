extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::consts::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::cpu::Cpu;

const SIZING: usize = 20;
// TODO: no idea what to do here yet
pub struct Display {
    memory: [bool; 2048],
}

impl Display {
    pub fn new() -> Self {
        Display {
            memory: [false; 2048],
        }
    }

    pub fn clear(&mut self) {
        for x in 0..DISPLAY_WIDTH {
            for y in 0..DISPLAY_HEIGHT {
                self.set_pixel(x, y, false);
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, state: bool) {
        self.memory[x * y * DISPLAY_WIDTH] = state;
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        self.memory[x * y * DISPLAY_WIDTH]
    }
}
