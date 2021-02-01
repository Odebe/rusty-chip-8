use std::fmt;

// ============================
pub struct Emulator {
    pub memory: Vec<u8>,
    pc: usize,
}

impl Emulator {
    pub fn new(program: &mut Vec<u8>) -> Self {
        Self {
            memory: program.to_vec(),
            pc: 0,
        }
    }

    pub fn is_running(&self) -> bool {
        self.pc < self.memory.len()
    }

    pub fn read_opcode(&self) -> Opcode {
        Opcode { number: (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16) }
    }

    pub fn next_opcode(&mut self) {
        self.pc = self.pc + 2;
    }

    pub fn state(&self) -> String { format!("pc: {}", self.pc) }
}

// ============================
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

    pub fn nnn(&self) -> u16 { self.number & 0x00ff }
    pub fn kk(&self) -> u16 { self.number & 0x0fff }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "number: {}, nibbles: {:?}", self.number, self.nibbles())
    }
}
