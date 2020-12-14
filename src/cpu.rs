use crate::consts::{FONT_SET, ROM_LOCATION};
use crate::display::Display;
use crate::keypad::Keypad;
use rand::rngs::OsRng;
use rand::Rng;

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
    pub rand: OsRng,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
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
            rand: OsRng::new().unwrap(),
        }
    }
    pub fn reset(&mut self) {
        self.index_reg = 0;
        self.program_counter = 0x200;
        self.memory = [0; 4096];
        self.reg = [0; 16];
        self.stack = [0; 16];
        self.stack_pointer = 0;
        self.delay = 0;
        self.sound = 0;
        self.display.clear();
        for i in 0..80 {
            self.memory[i] = FONT_SET[i];
        }
    }
    pub fn execute_cycle(&mut self) {
        let opcode: u16 = self.read_word(self.memory, self.program_counter);
        self.process_instruction(opcode);
    }
    pub fn decrement_timers(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }

        if self.sound > 0 {
            self.sound -= 1;
        }
    }

    pub fn process_instruction(&mut self, opcode: u16) {
        // mask
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            opcode & 0x000F,
        );
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let vx = self.reg[x];
        let vy = self.reg[y];
        let kk = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;
        let n = (opcode & 0x000F) as u8;

        self.program_counter += 2;

        match nibbles {
            // CLS
            (0, 0, 0xE, 0) => {
                self.display.clear();
            }
            // RET
            (0, 0, 0xE, 0xE) => self.ret(),
            // JP addr
            (0x1, _, _, _) => self.jp(nnn),
            // CALL addr
            (0x2, _, _, _) => self.call(opcode),
            // SE Vx, byte
            (0x3, _, _, _) => self.se(vx, kk),
            // SNE Vx, byte
            (0x4, _, _, _) => self.sne(vx, kk),
            // SE Vx, Vy
            (0x5, _, _, _) => self.se(vx, vy),
            // LD Vx, byte
            (0x6, _, _, _) => self.ld(x, kk),
            // ADD Vx, byte
            (0x7, _, _, _) => {
                self.reg[x] += kk;
            }
            // LD Vx, Vy
            (0x8, _, _, 0x0) => self.ld(x, vy),
            // OR Vx, Vy
            (0x8, _, _, 0x1) => self.or(x, vx, vy),
            // AND Vx, Vy
            (0x8, _, _, 0x2) => self.and(x, vx, vy),
            // XOR Vx, Vy
            (0x8, _, _, 0x3) => self.xor(x, vx, vy),
            // ADD Vx, Vy
            (0x8, _, _, 0x4) => self.op8xy4(x, y),
            // SUB Vx, Vy
            (0x8, _, _, 0x5) => self.sub(x, y),
            // SHR Vx, {, Vy}
            (0x8, _, _, 0x6) => self.shr(x, vx),
            // SUBN Vx, Vy
            (0x8, _, _, 0x7) => self.subn(x, y),
            // SHL Vx, {, Vy}
            (0x8, _, _, 0xE) => self.shl(x, vx),
            // SNE Vx, Vy
            (0x9, _, _, 0x0) => self.sne(vx, vy),
            // LD I, addr
            (0xA, _, _, _) => self.index_reg = nnn,
            // JP v0, addr
            (0xB, _, _, _) => self.program_counter = nnn + self.reg[0] as u16,
            // RND Vx, byte
            (0xC, _, _, _) => self.rnd(kk, x, vx),
            // DRW Vx, Vy, nibble
            (0xD, _, _, _) => self.drw(n, vx, vy),
            // SKP Vx
            (0xE, _, 0x9, 0xE) => self.skp(vx),
            // SKNP
            (0xE, _, 0xA, 0x1) => self.skpn(vx),
            // LD Vx, DT
            (0xF, _, 0x0, 0x7) => self.ld(x, self.delay),
            // LD Vx, k
            (0xF, _, 0x0, 0xA) => self.fx0a(x),
            // LD DT, Vx
            (0xF, _, 0x1, 0x5) => {
                self.delay = vx;
            }
            // LD ST, Vx
            (0xF, _, 0x1, 0x8) => {
                self.sound = vx;
            }
            // ADD I, Vx
            (0xF, _, 0x1, 0xE) => self.index_reg += vx as u16,
            // LD F, Vx
            (0xF, _, 0x2, 0x9) => self.fx29(vx),
            // LD B, Vx
            (0xF, _, 0x3, 0x3) => self.fx33(vx),
            // LD [I], Vx
            (0xF, _, 0x5, 0x5) => self.fx55(x),
            // LD Vx, [I]
            (0xF, _, 0x6, 0x5) => self.fx65(x),
            _ => (),
        }
    }

    // some general ops
    pub fn ret(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];
    }

    pub fn jp(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    pub fn call(&mut self, opcode: u16) {
        let nnn = opcode & 0x0FFF;
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = nnn;
    }

    pub fn se(&mut self, reg1: u8, reg2: u8) {
        self.program_counter += if reg1 == reg2 { 2 } else { 0 };
    }

    pub fn sne(&mut self, reg1: u8, reg2: u8) {
        self.program_counter += if reg1 != reg2 { 2 } else { 0 };
    }

    pub fn ld(&mut self, reg: usize, val: u8) {
        self.reg[reg] = val;
    }

    pub fn or(&mut self, x: usize, val1: u8, val2: u8) {
        self.reg[x] = val1 | val2;
    }

    pub fn xor(&mut self, x: usize, reg1: u8, reg2: u8) {
        self.reg[x] = reg1 ^ reg2;
    }
    pub fn and(&mut self, x: usize, val1: u8, val2: u8) {
        self.reg[x] = val1 & val2;
    }

    pub fn sub(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.reg[x].overflowing_sub(self.reg[y]);
        match overflow {
            true => self.reg[0xF] = 0,
            false => self.reg[0xF] = 1,
        }
        self.reg[x] = res;
    }

    pub fn subn(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.reg[x].overflowing_sub(self.reg[y]);
        match overflow {
            true => self.reg[0xF] = 0,
            false => self.reg[0xF] = 1,
        }
        self.reg[x] = res;
    }

    pub fn shr(&mut self, x: usize, reg: u8) {
        self.reg[0xF] = reg & 0x1;
        self.reg[x] >>= 1;
    }

    pub fn shl(&mut self, x: usize, reg: u8) {
        self.reg[0xF] = reg >> 7;
        self.reg[x] <<= 1;
    }

    pub fn skp(&mut self, reg: u8) {
        self.program_counter += if self.keypad.is_key_down(reg) { 2 } else { 0 };
    }
    pub fn skpn(&mut self, reg: u8) {
        self.program_counter += if self.keypad.is_key_down(reg) { 0 } else { 2 };
    }
    pub fn rnd(&mut self, kk: u8, x: usize, _reg: u8) {
        let mut rand = rand::thread_rng();
        self.reg[x] = kk & rand.gen::<u8>();
    }
    pub fn drw(&mut self, n: u8, vx: u8, vy: u8) {
        let collision = self.display.draw(
            vx as usize,
            vy as usize,
            &self.memory[self.index_reg as usize..(self.index_reg + u16::from(n)) as usize],
        );
        self.reg[0xF] = if collision { 1 } else { 0 };
    }

    // will put specific ops here
    pub fn fx0a(&mut self, reg: usize) {
        self.program_counter -= 2;
        for (index, key) in self.keypad.keys.iter().enumerate() {
            if *key {
                self.reg[reg] = index as u8;
                self.program_counter += 2
            }
        }
    }
    pub fn fx29(&mut self, reg: u8) {
        self.index_reg = u16::from(reg) * 5
    }
    pub fn fx33(&mut self, reg: u8) {
        self.memory[self.index_reg as usize] = reg / 100;
        self.memory[self.index_reg as usize + 1] = (reg / 10) % 10;
        self.memory[self.index_reg as usize + 2] = (reg % 100) % 10;
    }
    pub fn fx55(&mut self, x: usize) {
        self.memory[(self.index_reg as usize)..(self.index_reg + x as u16 + 1) as usize]
            .copy_from_slice(&self.reg[0..=x])
    }
    pub fn fx65(&mut self, x: usize) {
        self.reg[0..=x].copy_from_slice(
            &self.memory[(self.index_reg as usize)..(self.index_reg + x as u16 + 1) as usize],
        )
    }
    pub fn op8xy4(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.reg[x].overflowing_add(self.reg[y]);
        match overflow {
            true => self.reg[0xF] = 1,
            false => self.reg[0xF] = 0,
        }
        self.reg[x] = res;
    }

    pub fn load_rom(&mut self) {
        self.reset();
        let data = include_bytes!("roms2/INVADERS");
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.memory[0x200 + i] = byte;
            } else {
                break;
            }
        }
    }
    pub fn read_word(&mut self, memory: [u8; 4096], index: u16) -> u16 {
        (memory[index as usize] as u16) << 8 | (memory[(index + 1) as usize] as u16)
    }
}
