use std::fmt;
use std::ops::{Index,IndexMut,RangeTo};
use rand::prelude::*;

struct Stack {
    values: [u16; 16],
    pointer: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            values: [0; 16],
            pointer: 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        self.values[self.pointer] = value;
        self.pointer = self.pointer + 1;
    }

    pub fn pop(&mut self) -> u16 {
        let value = self.values[self.pointer];
        self.pointer = self.pointer - 1;
        value
    }
}

impl Index<usize> for Stack {
    type Output = u16;

    fn index(&self, i: usize) -> &Self::Output {
        &self.values[i]
    }
}

impl IndexMut<usize> for Stack {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.values[i]
    }
}

struct Memory([u8; 4096]);
impl Memory {
    pub fn size(&self) -> usize { 4096 }
    pub fn new() -> Self {
        Self([0; 4096])
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, i: u16) -> &Self::Output {
        &self.0[i as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, i: u16) -> &mut Self::Output {
        &mut self.0[i as usize]
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

struct Registers([u8; 16]);
impl Registers {
    pub fn new() -> Self {
        Self([0; 16])
    }
}

impl Index<u8> for Registers {
    type Output = u8;

    fn index(&self, i: u8) -> &Self::Output {
        &self.0[i as usize]
    }
}

impl Index<u16> for Registers {
    type Output = u8;

    fn index(&self, i: u16) -> &Self::Output {
        &self.0[i as usize]
    }
}

impl IndexMut<u16> for Registers {
    fn index_mut(&mut self, i: u16) -> &mut Self::Output {
        &mut self.0[i as usize]
    }
}

impl IndexMut<u8> for Registers {
    fn index_mut(&mut self, i: u8) -> &mut Self::Output {
        &mut self.0[i as usize]
    }
}

impl Index<RangeTo<usize>> for Registers {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &[u8] {
        &self.0[..][index]
    }
}


pub struct Emulator {
    registers: Registers,
    memory: Memory,
    stack: Stack,
    pc: usize,
    i: u16,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(),
            stack: Stack::new(),
            pc: 0,
            i: 0,
        }
    }

    pub fn exec_cycle(&mut self) {
        let opcode = self.read_opcode();
        self.exec_opcode(&opcode);
    }

    pub fn with_rom(rom: &Vec<u8>) -> Self {
        let mut emu = Self::new();
        emu.load_rom(rom, 512);
        emu
    }

    fn load_rom(&mut self, rom: &Vec<u8>, start: usize) {
        for (i, e) in rom.iter().enumerate() { self.memory[start + i] = *e; }
        self.pc = start as usize;
    }

    pub fn is_running(&self) -> bool {
        self.pc < self.memory.size() && self.pc != 0x30e
    }

    fn exec_opcode(&mut self, opcode : &Opcode) {
        match opcode.nibbles() {
            (0x0, 0x0, 0x0, 0x0) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "CLEAR");
                // video.clear();
                self.increment_pc();
            }
            (0x0, 0x0, 0x0, 0xE) => {
                self.pc = self.stack.pop() as usize;
                self.increment_pc();
            }
            (0x1, _, _, _) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "JUMP");
                self.pc = opcode.nnn() as usize;
            }
            (0x2, _, _, _) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "CA;;");
                self.stack.push(self.pc as u16);
                self.pc = opcode.nnn() as usize;
            }
            (0x3, _, _, _) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "SE");
                if self.registers[opcode.x()] == opcode.kk() {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            (0x4, _, _, _) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "SNE");
                if self.registers[opcode.x()] != opcode.kk() {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            (0x5, _, _, 0x0) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "SE");
                if self.registers[opcode.x()] == self.registers[opcode.y()] {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            (0x6, _, _, _) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                self.registers[opcode.x()] = opcode.kk();
                self.increment_pc();
            }
            (0x7, _, _, _) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "ADD");
                self.registers[opcode.x()] = self.registers[opcode.x()].wrapping_add(opcode.kk());
                self.increment_pc();
            }
            (0x8, _, _, 0x0) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                self.registers[opcode.x()] = self.registers[opcode.y()];
                self.increment_pc();
            }
            (0x8, _, _, 0x1) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "OR");
                self.registers[opcode.x()] = self.registers[opcode.x()] | self.registers[opcode.y()];
                self.increment_pc();
            }
            (0x8, _, _, 0x2) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "AND");
                self.registers[opcode.x()] = self.registers[opcode.x()] & self.registers[opcode.y()];
                self.increment_pc();
            }
            (0x8, _, _, 0x3) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "XOR");
                self.registers[opcode.x()] = self.registers[opcode.x()] ^ self.registers[opcode.y()];
                self.increment_pc();
            }
            (0x8, _, _, 0x4) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "ADD");
                if self.registers[opcode.y()] > u8::MAX - self.registers[opcode.x()] {
                    self.registers[0xF_u16] = 1_u8;
                } else {
                    self.registers[0xF_u16] = 0_u8;
                }
                self.registers[opcode.x()] = self.registers[opcode.x()].wrapping_add(self.registers[opcode.y()]);
                self.increment_pc();
            }
            (0x8, _, _, 0x5) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "SUB");
                if self.registers[opcode.x()] > self.registers[opcode.y()] {
                    self.registers[0xF_u16] = 1_u8;
                } else {
                    self.registers[0xF_u16] = 0_u8;
                }
                self.registers[opcode.x()] = self.registers[opcode.x()].wrapping_sub(self.registers[opcode.y()]);
                self.increment_pc();
            }
            (0x8, _, _, 0x6) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "SHR");
                self.registers[0xF_u16] = self.registers[opcode.x()] & 0x1;
                self.registers[opcode.x()] = self.registers[opcode.x()] >> 1;
                self.increment_pc();
            }
            (0x8, _, _, 0x7) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "SUBN");
                if self.registers[opcode.y()] > self.registers[opcode.x()] {
                    self.registers[0xF_u16] = 1_u8;
                } else {
                    self.registers[0xF_u16] = 0_u8;
                }
                self.increment_pc();
                self.registers[opcode.x()] = self.registers[opcode.y()].wrapping_sub(self.registers[opcode.x()]);
            }
            (0x8, _, _, 0xE) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "SHL");
                self.registers[0xF_u16] = self.registers[opcode.x()] & 0x80;
                self.registers[opcode.x()] = self.registers[opcode.x()] << 1;
                self.increment_pc();
            }
            (0x9, _, _, 0x0) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "SNE");
                if self.registers[opcode.x()] != self.registers[opcode.y()] {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            (0xA, _, _, _) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                self.i = opcode.nnn();
                self.increment_pc();
            }
            (0xB, _, _, _) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "JP");
                self.stack.push(self.pc as u16);
                self.pc = (opcode.nnn() as u8 + self.registers[0x0_u16]) as usize;
            }
            (0xC, _, _, _) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "RND");
                let mut rng = rand::thread_rng();
                self.registers[opcode.x()] = rng.gen_range(0..=255) & opcode.kk();
                self.increment_pc();
            }
            (0xD, _, _, _) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "DRW");
                self.increment_pc();
            }
            (0xE, _, 0x9, 0xE) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "SKP");
                self.increment_pc();
            }
            (0xE, _, 0xA, 0x1) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "SKNP");
                self.increment_pc();
            }
            (0xF, _, 0x0, 0x7) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                self.increment_pc();
            }
            (0xF, _, 0x0, 0xA) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                self.increment_pc();
            }
            (0xF, _, 0x1, 0x5) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                self.increment_pc();
            }
            (0xF, _, 0x1, 0x8) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                self.increment_pc();
            }
            (0xF, _, 0x1, 0xE) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "ADD");
                self.i = self.i + self.registers[opcode.x()] as u16;
                self.increment_pc();
            }
            (0xF, _, 0x2, 0x9) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                self.increment_pc();
            }
            (0xF, _, 0x3, 0x3) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                self.memory[self.i] = self.registers[opcode.x()] / 100;
                self.memory[self.i + 1] = (self.registers[opcode.x()] / 10) % 10;
                self.memory[self.i + 2] = self.registers[opcode.x()] % 10;
                self.increment_pc();
            }
            (0xF, _, 0x5, 0x5) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                let x = opcode.x() as usize;
                for (i, e) in self.registers[..x].iter().enumerate() {
                    self.memory[self.i + i as u16] = *e;
                }
                self.increment_pc();
            }
            (0xF, _, 0x6, 0x5) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                for i in 0..opcode.x() {
                    self.registers[i] = self.memory[self.i + i as u16];
                }
                self.increment_pc();
            }
            (_,_,_,_) => {
                self.increment_pc();
                println!("UNKNOWN");
            }
        }
    }

    fn read_opcode(&self) -> Opcode {
        let f_nibble = self.memory[self.pc as u16] as u16;
        let s_nibble = self.memory[self.pc as u16 + 1] as u16;

        Opcode::new(f_nibble << 8 | s_nibble)
    }

    fn increment_pc(&mut self) {
        self.pc = self.pc + 2;
    }

    pub fn state(&self) -> String { format!("pc: {:#x}", self.pc) }
}

pub struct Opcode {
    number: u16,
}

impl Opcode {
    pub fn new(number : u16) -> Self {
        Self { number }
    }

    pub fn nibbles(&self) -> (u8, u8, u8, u8) {
        (self.w(), self.x(), self.y(), self.n())
    }

    pub fn nnn(&self) -> u16 { self.number & 0x0fff }
    pub fn kk(&self) -> u8 { (self.number & 0x00ff) as u8 }

    pub fn w(&self) -> u8 { ((self.number & 0xf000) >> 12) as u8 }
    pub fn x(&self) -> u8 { ((self.number & 0x0f00) >> 8) as u8 }
    pub fn y(&self) -> u8 { ((self.number & 0x00f0) >> 4) as u8 }
    pub fn n(&self) -> u8 { (self.number & 0x000f) as u8 }
}

impl fmt::Display for Opcode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "number: {}, nibbles: {:?}", self.number, self.nibbles())
    }
}

impl fmt::LowerHex for Opcode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let nibbles = self.nibbles();
        write!(fmt, "0x{:x}{:x}{:x}{:x}", nibbles.0, nibbles.1, nibbles.2, nibbles.3)
    }
}
