use crate::consts::{DISPLAY_HEIGHT, DISPLAY_WIDTH, FONT_LOCATION, FONT_SET, ROM_LOCATION};
use crate::display::Display;
use crate::keypad::Keypad;
use std::fs::File;
use std::io::Read;
pub struct Cpu {
    pub index_reg: u16,
    pub program_counter: u16,
    pub memory: [u8; 4096],
    pub reg: [u8; 16],
    pub keypad: Keypad,
    pub display: Display,
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
            display: Display::new(),
            keypad: Keypad::new(),
            stack: [0; 16],
            stack_pointer: 0,
            delay: 0,
        };

        for x in 0..80 {
            cpu.memory[FONT_LOCATION as usize + x as usize] = FONT_SET[x as usize];
        }

        cpu
    }
    pub fn process_instruction(&mut self, opcode: u16) {
        self.program_counter += 2;
    }

    pub fn load_rom(&mut self, path: &str) {
        let mut rom = File::open(path).expect("Rom was not found");
        let mut buf = Vec::new();
        rom.read_to_end(&mut buf).expect("Error reading the rom");
        for (i, &byte) in buf.iter().enumerate() {
            self.memory[ROM_LOCATION as usize + i as usize] = byte;
        }
    }
    pub fn execute_cycle(&mut self) -> u16 {
        let opcode: u16 = self.read_word(self.memory, self.program_counter);
        self.process_instruction(opcode);
        opcode
    }
    pub fn read_word(&mut self, memory: [u8; 4096], index: u16) -> u16 {
        let upper = self.memory[self.program_counter as usize] as u16;
        let lower = self.memory[(self.program_counter + 1) as usize] as u16;

        println!("upper << 8 | lower : {:?}", format!("{:?}", upper << 8 | lower));
        println!("upper | lower : {:?}", format!("{:?}", upper | lower));
        upper << 8 | lower
    }
}
