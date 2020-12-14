extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::consts::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::cpu::Cpu;

pub struct Display {
    pub memory: [bool; 2048],
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

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let rows = sprite.len();
        let mut collision = false;

        for (j, row) in sprite.iter().enumerate().take(rows) {
            for i in 0..8 {
                let new_value = row >> (7 - i) & 0x01;
                if new_value == 1 {
                    let xi = (x + i) % DISPLAY_WIDTH;
                    let yj = (y + j) % DISPLAY_HEIGHT;
                    let old_value = self.get_pixel(xi, yj);
                    if old_value {
                        collision = true;
                    }
                    self.set_pixel(xi, yj, (new_value == 1) ^ old_value);
                }
            }
        }

        collision
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, state: bool) {
        self.memory[x + y * DISPLAY_WIDTH] = state;
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        self.memory[x + y * DISPLAY_WIDTH]
    }
}
