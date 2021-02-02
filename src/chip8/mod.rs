use std::fmt;
use std::ops::{Index,IndexMut};

struct Memory([u8; 4096]);
impl Memory {
    pub fn size(&self) -> usize { 4096 }

    pub fn new() -> Self {
        Self([0; 4096])
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

pub struct Emulator {
    memory: Memory,
    pc: usize,
    sp: usize,
    i: u16,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            pc: 0,
        }
    }

    pub fn with_rom(rom: &Vec<u8>) -> Self {
        let mut emu = Self::new();
        emu.load_rom(rom, 512);
        emu
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>, start: u16) {
        for (i, e) in rom.iter().enumerate() { self.memory[(start as usize + i) as usize] = *e; }
        self.pc = start as usize;
    }

    pub fn is_running(&self) -> bool {
        self.pc < 520 // self.memory.size()
    }

    pub fn read_opcode(&self) -> Opcode {
        Opcode::new((self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1 as usize] as u16))
    }

    pub fn next_opcode(&mut self) {
        self.pc = self.pc + 2;
    }

    pub fn state(&self) -> String { format!("pc: {}", self.pc) }
}

pub struct Opcode {
    number: u16,
}

impl Opcode {
    pub fn new(number : u16) -> Self {
        Self { number: number }
    }

    pub fn nibbles(&self) -> (u8, u8, u8, u8) {
        let w = (self.number & 0xf000) >> 12;
        let x = (self.number & 0x0f00) >> 8;
        let y = (self.number & 0x00f0) >> 4;
        let n = self.number & 0x000f;

        (w as u8, x as u8, y as u8, n as u8)
    }

    // pub fn nnn(&self) -> u16 { self.number & 0x00ff }
    // pub fn kk(&self) -> u16 { self.number & 0x0fff }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "number: {}, nibbles: {:?}", self.number, self.nibbles())
    }
}
