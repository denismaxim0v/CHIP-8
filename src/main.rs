mod cpu;
mod consts;
mod display;

extern crate sdl2;
extern crate getopts;

use getopts::Options;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            std::process::exit(1);
        },
    };

    let filename = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        std::process::exit(1);
    };

    let rom = read(&filename);

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut display = display::Display::new(&sdl_context, 2 as usize);
}

fn read<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    match File::open(path) {
        Ok(ref mut file) => {
            file.read_to_end(&mut buf).unwrap();
        },
        Err(_) => {
            std::process::exit(2);
        }
    }

    buf
}