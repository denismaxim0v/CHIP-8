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
    pub sound: u8,
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
            sound: 0,
            delay: 0,
        };

        for x in 0..80 {
            cpu.memory[FONT_LOCATION as usize + x as usize] = FONT_SET[x as usize];
        }

        cpu
    }

    pub fn process_instruction(&mut self, opcode: u16) {
        // mask
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            opcode & 0x000F,
        );
        let vx = self.reg[nibbles.2 as usize];
        let vy = self.reg[nibbles.3 as usize];
        let kk = self.reg[(opcode & 0x00FF) as usize];
        match nibbles {
            // CLS
            (0, 0, 0xE, 0) => {
                self.display.clear();
            }
            // RET
            (0, 0, 0xE, 0xE) => self.ret(),
            // JP addr
            (0x1, 0, 0, 0) => self.jp(opcode),
            // CALL addr
            (0x2, 0, 0, 0) => self.call(opcode),
            // SE Vx, byte
            (0x3, _, _, _) => self.se(vx, self.reg[(opcode & 0x00FF) as usize]),
            // SNE Vx, byte
            (0x4, _, _, _) => self.sne(vx, kk),
            // SE Vx, Vy
            (0x5, _, _, _) => self.se(vx, vy),
            // LD Vx, byte
            (0x6, _, _, _) => self.ld(vx, kk),
            // ADD Vx, byte
            (0x7, _, _, _) => self.add(vx, kk),
            // LD Vx, Vy
            (0x8, _, _, 0x0) => self.ld(vx, vy),
            // OR Vx, Vy
            (0x8, _, _, 0x1) => self.or(vx, vy),
            // AND Vx, Vy
            (0x8, _, _, 0x2) => self.and(vx, vy),
            // XOR Vx, Vy
            (0x8, _, _, 0x3) => self.xor(vx, vy),
            // ADD Vx, Vy
            (0x8, _, _, 0x4) => self.add(vx, vy),
            // SUB Vx, Vy
            (0x8, _, _, 0x5) => self.sub(vx, vy),
            // SHR Vx, {, Vy}
            // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
            // TODO: the heck?
            (0x8, _, _, 0x6) => self.shr(vx),
            // SUBN Vx, Vy
            (0x8, _, _, 0x7) => self.subn(vx, vy),
            // SHL Vx, {, Vy}
            // TODO: sorcery
            (0x8, _, _, 0xE) => self.shl(vx),
            // SNE Vx, Vy
            (0x9, _, _, 0x0) => self.sne(vx, vy),
            // LD I, addr
            (0xA, _, _, _) => self.ld(self.index_reg as u8, (opcode & 0x0FFF) as u8),
            // JP v0, addr
            (0xB, _, _, _) => self.jp(opcode),
            // RND Vx, byte
            (0xC, _, _, _) => self.rnd(opcode),
            // DRW Vx, Vy, nibble
            (0xD, _, _, _) => self.drw(opcode),
            // SKP Vx
            (0xE, _, 0x9, 0xE) => self.skp(opcode),
            // SKNP
            (0xE, _, 0xA, 0x1) => self.skpn(opcode),
            // LD Vx, DT
            (0xF, _, 0x0, 0x7) => self.ld(vx, self.delay),
            // LD Vx, k
            (0xF, _, 0x0, 0xA) => self.ld(opcode), //diff
            // LD DT, Vx
            (0xF, _, 0x1, 0x5) => self.ld(self.delay, vx),
            // LD ST, Vx
            (0xF, _, 0x1, 0x8) => self.ld(self.sound, vx),
            // ADD I, Vx
            (0xF, _, 0x1, 0xE) => self.add(self.index_reg as u8, vx),
            // LD F, Vx
            (0xF, _, 0x2, 0x9) => self.ld(opcode), // diff
            // LD B, Vx
            (0xF, _, 0x3, 0x3) => self.ld(opcode), // diff
            // LD [I], Vx
            (0xF, _, 0x5, 0x5) => self.ld(opcode), // diff
            // LD Vx, [I]
            (0xF, _, 0x6, 0x5) => self.ld(opcode), // diff
        }
        self.program_counter += 2;
    }

    // some general ops
    pub fn ret(&mut self) {
        self.program_counter -= 1;
        self.stack[self.program_counter as usize];
    }

    pub fn jp(&mut self, opcode: u16) {
        let nnn = opcode & 0x0FFF;
        self.program_counter = nnn;
    }

    pub fn call(&mut self, opcode: u16) {
        let nnn = opcode & 0x0FFF;
        self.stack[self.program_counter as usize] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = nnn;
    }

    pub fn se(&mut self, reg1: u8, reg2: u8) {
        self.program_counter += if reg1 == reg2 { 2 } else { 0 };
    }

    pub fn sne(&mut self, reg1: u8, reg2: u8) {
        self.program_counter += if reg1 != reg2 { 2 } else { 0 };
    }

    pub fn add(&mut self, reg: u8, val: u8) {
        reg += val;
    }

    pub fn ld(&mut self, reg: u8, val: u8) {
        reg = val;
    }

    pub fn or(&mut self, reg1: u8, reg2: u8) {
        reg1 = reg1 | reg2;
    }

    pub fn xor(&mut self, reg1: u8, reg2: u8) {
        reg1 = reg1 ^ reg2;
    }
    pub fn and(&mut self, reg1: u8, reg2: u8) {
        reg1 = reg1 & reg2;
    }

    pub fn sub(&mut self, reg1: u8, reg2: u8) {
        self.reg[0xF] = if reg1 > reg2 { 1 } else { 0 };
        reg1 = reg1 - reg2;
    }

    pub fn subn(&mut self, reg1: u8, reg2: u8) {
        self.reg[0xF] = if reg2 > reg1 { 1 } else { 0 };
        reg1 = reg2 - reg1;
    }

    pub fn shr(&mut self, reg: u8) {
        self.reg[0xF] = reg & 0x1;
        reg >>= 1;
    }

    pub fn shl(&mut self, reg: u8) {
        self.reg[0xF] = reg >> 7;
        reg <<= 1;
    }

    pub fn skp(&mut self, reg: u8) {
        self.program_counter += if self.keypad.is_key_down(reg) { 2 } else { 0 };
    }
    pub fn skpn(&mut self, reg: u8) {
        self.program_counter += if self.keypad.is_key_down(reg) { 0 } else { 2 };
    }

    // will put specific ops here

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

        println!(
            "upper << 8 | lower : {:?}",
            format!("{:?}", upper << 8 | lower)
        );
        println!("upper | lower : {:?}", format!("{:?}", upper | lower));
        upper << 8 | lower
    }
}
