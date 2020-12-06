mod cpu;
mod consts;
mod display;

extern crate sdl2;
extern crate getopts;

use display::Display;
use cpu::Cpu;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::env;
use std::time::Duration;


fn main() {
    let mut cpu: Cpu;
    cpu = Cpu::new();
    let args: Vec<String> = env::args().collect();
    let scale: usize = 10;


    cpu.load_rom(&args[1]);

    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::new(&sdl_context, scale);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut quit = false;

    while quit == false {
        //Emulation Cycle
        cpu.emulation_cycle();

        //render graphics
        display.render(&cpu);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    quit = true;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => display.key_press(&mut cpu, keycode),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => display.key_release(&mut cpu, keycode),

                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
    }
}