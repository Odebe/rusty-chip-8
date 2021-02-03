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
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            pc: 0,
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

    fn load_rom(&mut self, rom: &Vec<u8>, start: u16) {
        for (i, e) in rom.iter().enumerate() { self.memory[(start as usize + i) as usize] = *e; }
        self.pc = start as usize;
    }

    pub fn is_running(&self) -> bool {
        self.pc < 550 // self.memory.size()
    }

    fn exec_opcode(&mut self, opcode : &Opcode) {
        println!("{}, op: {:x}", self.state(), opcode);

        match opcode.nibbles() {
            (0x0, 0x0, 0x0, 0x0) => {
                // video.clear();
                self.next_opcode();
            }
            (0x0, 0x0, 0x0, 0xE) => {
                // self.pc = stack.pop();
                self.next_opcode();
            }
            (0x1, _, _, _) => {
                // jimp opcode.nnn();
                self.next_opcode();
            }
            (0x2, _, _, _) => {
                // call opcode.nnn();
                self.next_opcode();
            }
            (0x3, _, _, _) => {
                // "SE v[opcode.x()] opcode.kk()
                self.next_opcode();
            }
            (0x4, _, _, _) => {
                // "SNE v[opcode.x()] opcode.kk()
                self.next_opcode();
            }
            (0x5, _, _, 0x0) => {
                // "SE v[opcode.x()] v[opcode.y()]
                self.next_opcode();
            }
            (0x6, _, _, _) => {
                // "LD v[x] kk
                self.next_opcode();
            }
            (0x7, _, _, _) => {
                // "ADD v[x] kk
                self.next_opcode();
            }
            (0x8, _, _, 0x0) => {
                // "LD V#{x}, V#{y}"
                self.next_opcode();
            }
            (0x8, _, _, 0x1) => {
                // "OR V#{x}, V#{y}"
                self.next_opcode();
            }
            (0x8, _, _, 0x2) => {
                // "AND V#{x}, V#{y}"
                self.next_opcode();
            }
            (0x8, _, _, 0x3) => {
                // "XOR V#{x}, V#{y}"
                self.next_opcode();
            }
            (0x8, _, _, 0x4) => {
                // "ADD V#{x}, V#{y}"
                self.next_opcode();
            }
            (0x8, _, _, 0x5) => {
                // "SUB V#{x}, V#{y}"
                self.next_opcode();
            }
            (0x8, _, _, 0x6) => {
                // "SHR V#{x} {, V#{y}}"
                self.next_opcode();
            }
            (0x8, _, _, 0x7) => {
                // "SUBN V#{x}, V#{y}"
                self.next_opcode();
            }
            (0x8, _, _, 0xE) => {
                // SHL V#{x} {, V#{y}}"
                self.next_opcode();
            }
            (0x9, _, _, 0x0) => {
                // "SNE V#{x}, V#{y}"
                self.next_opcode();
            }
            (0xA, _, _, _) => {
                // "LD I, #{nnn}"
                self.next_opcode();
            }
            (0xB, _, _, _) => {
                // "JP V0, #{nnn}"
                self.next_opcode();
            }
            (0xC, _, _, _) => {
                // "RND V#{x}, #{kk}"
                self.next_opcode();
            }
            (0xD, _, _, _) => {
                // "DRW V#{x}, V#{y}, #{n}"
                self.next_opcode();
            }
            (0xE, _, 0x9, 0xE) => {
                // "SKP V#{x}"
                self.next_opcode();
            }
            (0xE, _, 0xA, 0x1) => {
                // "SKNP V#{x}"
                self.next_opcode();
            }
            (0xF, _, 0x0, 0x7) => {
                // "LD V#{x}, DT"
                self.next_opcode();
            }
            (0xF, _, 0x0, 0xA) => {
                // "LD V#{x}, K"
                self.next_opcode();
            }
            (0xF, _, 0x1, 0x5) => {
                // "LD DT, V#{x}"
                self.next_opcode();
            }
            (0xF, _, 0x1, 0x8) => {
                // "LD V#{x}, DT"
                self.next_opcode();
            }
            (0xF, _, 0x1, 0xE) => {
                // "ADD I, V#{x}"
                self.next_opcode();
            }
            (0xF, _, 0x2, 0x9) => {
                // "LD F, V#{x}"
                self.next_opcode();
            }
            (0xF, _, 0x3, 0x3) => {
                // "LD B, V#{x}"
                self.next_opcode();
            }
            (0xF, _, 0x5, 0x5) => {
                // "LD [I], V#{x}"
                self.next_opcode();
            }
            (0xF, _, 0x6, 0x5) => {
                // "LD V#{x}, [I]"
                self.next_opcode();
            }
            (_,_,_,_) => {
                self.next_opcode();
                // println!("UNKNOWN");
            }
        }
    }

    fn read_opcode(&self) -> Opcode {
        Opcode::new((self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1 as usize] as u16))
    }

    fn next_opcode(&mut self) {
        self.pc = self.pc + 2;
    }

    pub fn state(&self) -> String { format!("pc: {}", self.pc) }
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
    pub fn kk(&self) -> u16 { self.number & 0x00ff }

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
