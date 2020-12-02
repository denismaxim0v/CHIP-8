mod display;
extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

fn main() {
    println!("Hello, world!");
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut display = display::Display::new(&sdl_context);
}
