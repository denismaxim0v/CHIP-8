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
        // mask
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            opcode & 0x000F,
        );
        match nibbles {
            // CLS
            (0, 0, 0xE, 0) => self.clear_display(),
            // RET
            (0, 0, 0xE, 0xE) => self.ret(),
            // JP addr
            (0x1, 0, 0, 0) => self.jp(opcode),
            // CALL addr
            (0x2, 0, 0, 0) => self.call(opcode),
            // SE Vx, byte
            (0x3, _, _, _) => self.se(
                self.reg[nibbles.2 as usize],
                self.reg[(opcode & 0x00FF) as usize],
            ),
            // SNE Vx, byte
            (0x4, _, _, _) => self.sne(opcode),
            // SE Vx, Vy
            (0x5, _, _, _) => self.se(self.reg[nibbles.2 as usize], self.reg[nibbles.3 as usize]),
            // LD Vx, byte
            (0x6, _, _, _) => self.ld(
                self.reg[nibbles.2 as usize],
                self.reg[(opcode & 0x00FF) as usize],
            ),
            // ADD Vx, byte
            (0x7, _, _, _) => self.add(
                self.reg[nibbles.2 as usize],
                self.reg[(opcode & 0x0FF) as usize],
            ),
            // LD Vx, Vy
            (0x8, _, _, 0x0) => self.ld(self.reg[nibbles.2 as usize], self.reg[nibbles.3 as usize]),
            // OR Vx, Vy
            (0x8, _, _, 0x1) => self.or(self.reg[nibbles.2 as usize], self.reg[nibbles.3 as usize]),
            // AND Vx, Vy
            (0x8, _, _, 0x2) => {
                self.and(self.reg[nibbles.2 as usize], self.reg[nibbles.3 as usize])
            }
            // XOR Vx, Vy
            (0x8, _, _, 0x3) => {
                self.xor(self.reg[nibbles.2 as usize], self.reg[nibbles.3 as usize])
            }
            // ADD Vx, Vy
            (0x8, _, _, 0x4) => {
                self.add(self.reg[nibbles.2 as usize], self.reg[nibbles.3 as usize])
            }
            // SUB Vx, Vy
            (0x8, _, _, 0x5) => {
                self.sub(self.reg[nibbles.2 as usize], self.reg[nibbles.3 as usize])
            }
            // SHR Vx, {, Vy}
            // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
            // TODO: the heck?
            (0x8, _, _, 0x6) => self.shr(),
            // SUBN Vx, Vy
            (0x8, _, _, 0x7) => {
                self.subn(self.reg[nibbles.2 as usize], self.reg[nibbles.3 as usize])
            }
            // SHL Vx, {, Vy}
            // TODO: sorcery
            (0x8, _, _, 0xE) => self.shl(),
            // SNE Vx, Vy
            (0x9, _, _, 0x0) => self.sne(opcode),
            // LD I, addr
            (0xA, _, _, _) => self.ld(opcode),
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
            (0xF, _, 0x0, 0x7) => self.ld(opcode),
            // LD Vx, k
            (0xF, _, 0x0, 0xA) => self.ld(opcode),
            // LD DT, Vx
            (0xF, _, 0x1, 0x5) => self.ld(opcode),
            // LD ST, Vx
            (0xF, _, 0x1, 0x8) => self.ld(opcode),
            // ADD I, Vx
            (0xF, _, 0x1, 0xE) => self.add(self.index_reg, self.reg[nibbles.2 as usize]),
            // LD F, Vx
            (0xF, _, 0x2, 0x9) => self.ld(opcode),
            // LD B, Vx
            (0xF, _, 0x3, 0x3) => self.ld(opcode),
            // LD [I], Vx
            (0xF, _, 0x5, 0x5) => self.ld(opcode),
            // LD Vx, [I]
            (0xF, _, 0x6, 0x5) => self.ld(opcode),
        }
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

        println!(
            "upper << 8 | lower : {:?}",
            format!("{:?}", upper << 8 | lower)
        );
        println!("upper | lower : {:?}", format!("{:?}", upper | lower));
        upper << 8 | lower
    }
}
