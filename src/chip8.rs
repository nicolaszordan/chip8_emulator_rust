extern crate arrayvec;
extern crate rand;

use crate::display::{Display, DrawResult};
use crate::keypad::Keypad;
use crate::ram::RAM;

use arrayvec::ArrayVec;
use std::{thread, time};

type Registers = [u8; 16];

pub struct Chip8 {
    ram: RAM,

    display: Display,

    registers: Registers,

    keypad: Keypad,

    program_counter: u16,
    index_register: u16,

    stack: ArrayVec<[u16; 16]>,

    delay_timer: u8,
    sound_timer: u8,
    // FIXME: cache random generator
}

impl Default for Chip8 {
    fn default() -> Chip8 {
        Chip8::new()
    }
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram: RAM::new(),
            display: Display::new(),
            registers: [0; 16],
            keypad: Keypad::new(),
            program_counter: 0,
            index_register: 0,
            stack: ArrayVec::new(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load_game(&mut self, file_name: &str) -> std::io::Result<()> {
        let buffer = std::fs::read(file_name)?;

        self.reset();
        self.ram.write(0x200, &buffer);

        Ok(())
    }

    pub fn reset(&mut self) {
        self.ram.reset();
        self.display.reset();
        self.registers.iter_mut().for_each(|v| *v = 0);
        self.keypad.reset();
        self.program_counter = 0x200;
        self.index_register = 0;
        self.stack.clear();
        self.delay_timer = 0;
        self.sound_timer = 0;
        for (i, font) in FONT_SET.iter().enumerate() {
            self.ram.write8(i as u16, *font);
        }
    }

    pub fn run(&mut self) {
        loop {
            for _ in 0..10 {
                self.step();
            }
            self.decrement_timers();
            // render
            // update keypad
            // sleep for remainder of frame
            thread::sleep(time::Duration::from_secs(1) / 60);
        }
    }

    pub fn step(&mut self) {
        let current_op = self.ram.read16(self.program_counter);
        self.process_op(current_op);
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn process_op(&mut self, op: u16) {
        let op_0 = (op & 0xF000) >> 12;
        let op_1 = (op & 0x0F00) >> 8;
        let op_2 = (op & 0x00F0) >> 4;
        let op_3 = op & 0x000F;

        let x = ((op & 0x0F00) >> 8) as usize;
        let y = ((op & 0x00F0) >> 4) as usize;
        let nnn = op & 0x0FFF;
        let kk = (op & 0x00FF) as u8;
        let n = (op & 0x000F) as u8;

        self.program_counter += 2;

        match (op_0, op_1, op_2, op_3) {
            // CLS
            (0x0, 0x0, 0xE, 0x0) => self.display.clear(),
            // RET
            (0x0, 0x0, 0xE, 0xE) => {
                self.program_counter = self.stack.pop().expect("pop from empty stack")
            }
            // JMP
            (0x1, _, _, _) => self.program_counter = nnn,
            // CALL
            (0x2, _, _, _) => {
                self.stack.push(self.program_counter);
                self.program_counter = nnn
            }
            // SE Vx, byte
            (0x3, _, _, _) => {
                if self.registers[x] == kk {
                    self.program_counter += 2
                }
            }
            // SNE Vx, byte
            (0x4, _, _, _) => {
                if self.registers[x] != kk {
                    self.program_counter += 2
                }
            }
            // SNE Vx, Vy
            (0x5, _, _, 0x0) => {
                if self.registers[x] == self.registers[y] {
                    self.program_counter += 2
                }
            }
            // LD Vx, byte
            (0x6, _, _, _) => self.registers[x] = kk,
            // ADD Vx, byte
            (0x7, _, _, _) => {
                let (res, _) = self.registers[x].overflowing_add(kk);
                self.registers[x] = res
            }
            // LD Vx, Vy
            (0x8, _, _, 0x0) => self.registers[x] = self.registers[y],
            // OR Vx, Vy
            (0x8, _, _, 0x1) => self.registers[x] |= self.registers[y],
            // AND Vx, Vy
            (0x8, _, _, 0x2) => self.registers[x] &= self.registers[y],
            // XOR Vx, Vy
            (0x8, _, _, 0x3) => self.registers[x] ^= self.registers[y],
            // ADD Vx, Vy
            (0x8, _, _, 0x4) => {
                let (res, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                self.registers[0xF] = if overflow { 1 } else { 0 };
                self.registers[x] = res;
            }
            // SUB Vx, Vy
            (0x8, _, _, 0x5) => {
                let (res, overflow) = self.registers[x].overflowing_sub(self.registers[y]);
                self.registers[0xF] = if overflow { 0 } else { 1 };
                self.registers[x] = res;
            }
            // SHR Vx {, Vy}
            (0x8, _, _, 0x6) => {
                self.registers[0xF] = self.registers[x] & 0x1;
                self.registers[x] >>= 1;
            }
            // SUBN Vx, Vy
            (0x8, _, _, 0x7) => {
                let (res, overflow) = self.registers[y].overflowing_sub(self.registers[x]);
                self.registers[0xF] = if overflow { 0 } else { 1 };
                self.registers[x] = res;
            }
            // SHL Vx {, Vy}
            (0x8, _, _, 0xE) => {
                self.registers[0xF] = self.registers[x] & 0x80;
                self.registers[x] <<= 1;
            }
            // SNE Vx, Vy
            (0x9, _, _, 0x0) => {
                if self.registers[x] != self.registers[y] {
                    self.program_counter += 2
                }
            }
            // LD I, addr
            (0xA, _, _, _) => self.index_register = nnn,
            // JP V0, addr
            (0xB, _, _, _) => self.program_counter = nnn + self.registers[0x0] as u16,
            // RND Vx, byte
            (0xC, _, _, _) => self.registers[x] = rand::random::<u8>() & kk, // FIXME: use random generator
            // DRW Vx, Vy, nibble
            (0xD, _, _, _) => {
                let mut buffer = [0; 16];
                self.ram
                    .read(self.index_register, &mut buffer[..n as usize]);
                self.registers[0xF] = match self.display.draw(
                    self.registers[x] as usize,
                    self.registers[y] as usize,
                    &buffer,
                ) {
                    DrawResult::Overwrite => 1,
                    DrawResult::NoOverwrite => 0,
                }
            }
            // SKP Vx
            (0xE, _, 0x9, 0xE) => {
                if self.keypad.is_key_down(self.registers[x] as usize) {
                    self.program_counter += 2;
                }
            }
            // SKNP Vx
            (0xE, _, 0xA, 0x1) => {
                if self.keypad.is_key_up(self.registers[x] as usize) {
                    self.program_counter += 2;
                }
            }
            // LD Vx, DT
            (0xF, _, 0x0, 0x7) => self.registers[x] = self.delay_timer,
            // LD Vx, K
            (0xF, _, 0x0, 0xA) => {
                self.program_counter -= 2; // reset pc on this instruction to stop all execution
                for (i, key) in self.keypad.keys.iter().enumerate() {
                    if key.is_key_down() {
                        self.registers[x] = i as u8;
                        self.program_counter += 2;
                    }
                }
            }
            // LD DT, Vx
            (0xF, _, 0x1, 0x5) => self.delay_timer = self.registers[x],
            // LD ST, Vx
            (0xF, _, 0x1, 0x8) => self.sound_timer = self.registers[x],
            // ADD I, Vx
            (0xF, _, 0x1, 0xE) => {
                self.index_register = self
                    .index_register
                    .checked_add(self.registers[x] as u16)
                    .expect("ADD I, Vx addition overflow")
            }
            // LD F, Vx -- The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx
            (0xF, _, 0x2, 0x9) => self.index_register = self.registers[x] as u16 * 5,
            // LD B, Vx
            (0xF, _, 0x3, 0x3) => {
                self.ram
                    .write8(self.index_register, self.registers[x] / 100);
                self.ram
                    .write8(self.index_register + 1, (self.registers[x] / 10) % 10);
                self.ram
                    .write8(self.index_register + 2, self.registers[x] % 10);
            }
            // LD [I], Vx
            (0xF, _, 0x5, 0x5) => self.ram.write(self.index_register, &self.registers[..x]),
            // LD Vx, [I]
            (0xF, _, 0x6, 0x5) => self.ram.read(self.index_register, &mut self.registers[..x]),

            (_, _, _, _) => panic!("unsupported opcode [opcode={:#06X}]", op),
        }
    }
}

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
