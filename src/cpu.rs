use crate::consts::{DISPLAY_HEIGHT, DISPLAY_WIDTH, FONT_LOCATION, FONT_SET, ROM_LOCATION};
use std::fs::File;
use std::io::Read;
pub struct Cpu {
    pub index_reg: usize,
    pub program_counter: usize,
    pub memory: [usize; 4096],
    pub reg: [u8; 16],
    pub keypad: [bool; 16],
    pub display: [u8; DISPLAY_HEIGHT * DISPLAY_WIDTH],
    pub stack: [u16; 16],
    pub stack_pointer: u8,
    pub delay: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Self {
            index_reg: 0,
            program_counter: ROM_LOCATION,
            memory: [0; 4096],
            reg: [0; 16],
            display: [0; DISPLAY_HEIGHT * DISPLAY_WIDTH],
            keypad: [false; 16],
            stack: [0; 16],
            stack_pointer: 0,
            delay: 0,
        };

        for x in 0..80 {
            cpu.memory[FONT_LOCATION + x] = FONT_SET[x];
        }

        cpu
    }

    pub fn load_rom(&mut self, path: &str) {
        let mut rom = File::open(path).expect("Rom was not found");
        let mut buf = Vec::new();
        let buf_size = rom.read_to_end(&mut buf).expect("Error reading the rom");
        for i in 0..buf_size {
            self.memory[ROM_LOCATION + i] = buf[i] as usize;
        }
    }
}
