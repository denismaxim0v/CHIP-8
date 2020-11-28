pub struct Cpu {
  pub index_reg: u16,
  pub program_counter: u16,
  pub memory: [u8, 4096],
  pub reg: [u8, 16],
  pub keypad: Keypad,
  pub display: Display,
  pub stack: [u16, 16],
  pub stack_pointer: u8,
  pub delay: u8
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      index_reg: 0,
      program_counter: 0,
      memory: [0, 4096],
      reg: [0, 16],
      display: Display::new(),
      keypad: Keypad::new(),
      stack: [0, 16],
      stack_pointer: 0,
      delay: 0
    }
  }
}