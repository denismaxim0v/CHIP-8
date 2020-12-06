extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::consts::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::cpu::Cpu;

pub struct Display {
  display: sdl2::render::WindowCanvas,
  scale: usize
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl, size: usize) -> Self {
      let video_subsystem = sdl_context.video().unwrap();

      let window = video_subsystem
          .window(
              "Chip8",
              (DISPLAY_WIDTH * size) as u32,
              (DISPLAY_HEIGHT * size) as u32,
          )
          .position_centered()
          .build()
          .unwrap();

      let mut ui = Display {
          display: window.into_canvas().build().unwrap(),
          scale: size,
      };

      ui.display.set_draw_color(Color::RGB(0, 0, 0));
      ui.display.clear();
      ui.display.present();

      ui

    }
    pub fn render(&mut self, chip8: &Cpu) {
      self.display.set_draw_color(Color::RGB(255, 0, 0));
      self.display.clear();
      for i in 0..(DISPLAY_WIDTH*DISPLAY_HEIGHT) {
          let pixel = chip8.display[i];
          let x = i % DISPLAY_WIDTH * self.scale; //get x position of pixel
          let y = i / DISPLAY_WIDTH * self.scale; //get y position of pixel

          self.display.set_draw_color(Color::RGB(0, 0, 0));
          if pixel == 1 {
              self.display.set_draw_color(Color::RGB(255, 255, 255));
          }

          let _ = self.display.fill_rect(Rect::new(
              x as i32,
              y as i32,
              self.scale as u32,
              self.scale as u32,
          )); //Draw the pixel as a square
      }

      self.display.present(); //display changes in window
  }
}