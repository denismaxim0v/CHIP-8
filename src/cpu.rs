use crate::consts::{ROM_LOCATION, DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_LOCATION, FONT_SET};

pub struct Cpu {
  pub index_reg: usize,
  pub program_counter: usize,
  pub memory: [usize; 4096],
  pub reg: [u8; 16],
  pub keypad: [bool; 16],
  pub display: [u8; DISPLAY_HEIGHT * DISPLAY_WIDTH],
  pub stack: [u16; 16],
  pub stack_pointer: u8,
  pub delay: u8
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
      delay: 0
    };

    for x in 0..80 {
      cpu.memory[FONT_LOCATION + x] = FONT_SET[x];
    }

    cpu 
  }
}