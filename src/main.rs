mod consts;
mod cpu;
mod display;
mod keypad;

extern crate getopts;
extern crate sdl2;

use consts::{DISPLAY_HEIGHT, DISPLAY_WIDTH, SCALE};
use cpu::Cpu;
use display::Display;
use keypad::Keypad;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::env;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut cpu = Cpu::new();

    cpu.load_rom();
 
    let height =  SCALE * DISPLAY_WIDTH;
    let width =  SCALE * DISPLAY_HEIGHT;
    let window = video_subsystem.window("Chip 8 Emulator", height as u32, width as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode_map(keycode) {
                        Some(key) => { cpu.keypad.key_down(key) },
                        None => {}
                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    match keycode_map(keycode) {
                        Some(key) => { cpu.keypad.key_up(key) },
                        None => {}
                    }
                },
                _ => {}
            }
        }

        for _ in 0..8 { cpu.execute_cycle(); }
        cpu.decrement_timers();

        for y in 0..(DISPLAY_HEIGHT as u32) {
            for x in 0..(DISPLAY_WIDTH as u32) {
                let idx = y * DISPLAY_WIDTH as u32 + x;
                canvas.set_draw_color(color(cpu.display.memory[idx as usize]));
                
                let _ = canvas.fill_rect(
                    Rect::new((SCALE as u32 * x) as i32 - 1, (SCALE as u32 * y) as i32 - 1, SCALE as u32, SCALE as u32)
                );
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn color(value: bool) -> Color {
    if value {
        Color::RGB(0, 204, 204)
    } else {
        Color::RGBA(0, 0, 0, 51)
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
