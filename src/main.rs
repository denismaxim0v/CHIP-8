mod consts;
mod cpu;
mod display;
mod keypad;

extern crate getopts;
extern crate sdl2;

use cpu::Cpu;
use display::Display;
use keypad::Keypad;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::env;
use std::time::Duration;

fn main() {
    let mut cpu: Cpu;
    cpu = Cpu::new();
    let args: Vec<String> = env::args().collect();

    cpu.load_rom(&args[1]);

    let sdl_context = sdl2::init().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut quit = false;

    while quit == false {
        //cpu.emulation_cycle();

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
                } => match keycode_map(keycode) {
                    Some(keycode) => cpu.keypad.key_down(keycode),
                    None => {}
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode_map(keycode) {
                    Some(keycode) => cpu.keypad.key_down(keycode),
                    None => {}
                },

                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
    }
}

fn keycode_map(keycode: Keycode) -> Option<u8> {
    match keycode {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xc),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xd),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xe),
        Keycode::Z => Some(0xa),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xb),
        Keycode::V => Some(0xf),
        _ => None,
    }
}
