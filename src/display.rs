extern crate sdl2;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use chip8::consts::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub struct Display {
  canvas: Canvas<Window>
}

impl Display {
  pub fn new(context: &Sdl) -> Result<Self, String> {
    let video_subsystem = context.video().unwrap();

    let window = video_subsystem.window("chip8", DISPLAY_WIDTH, DISPLAY_HEIGHT)
      .position_centered()
      .build()
      .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();
    Ok(Display{canvas})
  }
}