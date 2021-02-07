extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use std::fmt;
use std::ops::{Index, IndexMut, RangeTo, Range, RangeInclusive};
use std::time::{Duration, Instant};

use rand::prelude::*;
use std::process::exit;

struct PixelSize {
    width: u32,
    height: u32,
}

struct Video {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    memory: [u64; 32],
    draw_flag: bool,
    pixinfo: PixelSize,
}

impl Video {
    const BLACK: sdl2::pixels::Color = Color::RGB(0, 0, 0);
    const WHITE: sdl2::pixels::Color = Color::RGB(255, 255, 255);

    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let width = 800;
        let height = 600;

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo", width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.present();

        Self {
            canvas,
            memory: [0; 32],
            draw_flag: true,
            pixinfo: PixelSize {
                width: width / 64,
                height: height / 32,
            }
        }
    }

    pub fn clear(&mut self) {
        self.memory = [0; 32];
    }

    pub fn draw_sprite(&mut self, sprite: &Vec<u8>, x: u8, y: u8) -> u8 {
        self.draw_flag = true;
        let mut collision : u8 = 0;

        for (sprite_line_index, sprite_pixel) in sprite.iter().enumerate() {
            let line_num = y + sprite_line_index as u8;

            for xi in 0..8 {
                if sprite_pixel & (0x80 >> xi) != 0 {
                    let offset = 63 - x - xi;
                    let display_bit_p = 1 << offset;

                    if (self.memory[line_num as usize] & display_bit_p) > 0 { collision = 1; }

                    self.memory[line_num as usize] = self.memory[line_num as usize] ^ display_bit_p;
                }
            }
        }

        collision
    }

    pub fn refresh(&mut self) {
        if !self.draw_flag { return; }

        self.draw_flag = false;
        self.canvas.clear();

        for (line_index, line) in self.memory.iter().enumerate() {
            for pixel_index in 0..64 {
                let offset = 63 - pixel_index;
                let pixel_value = (line & (1_u64 << offset)) >> offset;

                if pixel_value == 1 {
                    self.canvas.set_draw_color(Self::BLACK);
                } else {
                    self.canvas.set_draw_color(Self::WHITE);
                }

                let rect = Rect::new(
                    pixel_index * self.pixinfo.width as i32,
                    line_index as i32 * self.pixinfo.height as i32,
                    self.pixinfo.width, self.pixinfo.height);

                self.canvas.fill_rect(rect);
            }
        }

        self.canvas.present();
    }
}

struct Font {
    memory: [u8; 80],
}

impl Font {
    pub const START: u16 = 120;
    pub const DEFAULT: [u8; 80] = [
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
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];

    pub fn new() -> Self {
        Self { memory: Self::DEFAULT }
    }
}

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
        self.pointer = self.pointer - 1;
        self.values[self.pointer]
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, " stack_p: {}, stack_value: {:#x}, ", self.pointer, self.values[self.pointer])
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

impl Index<Range<usize>> for Memory {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &[u8] {
        &self.0[..][index]
    }
}

impl Index<Range<u16>> for Memory {
    type Output = [u8];

    fn index(&self, index: Range<u16>) -> &[u8] {
        &self.0[..][(index.start as usize)..(index.end as usize)]
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

impl Index<Range<usize>> for Registers {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &[u8] {
        &self.0[..][index]
    }
}

impl Index<RangeInclusive<usize>> for Registers {
    type Output = [u8];

    fn index(&self, index: RangeInclusive<usize>) -> &[u8] {
        &self.0[..][index]
    }
}

impl Index<RangeTo<usize>> for Registers {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &[u8] {
        &self.0[..][index]
    }
}

impl fmt::LowerHex for Registers {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[");
        for e in self.0.iter() { write!(fmt, "{:x}, ", *e); }
        write!(fmt, "]")
    }
}

struct Keyboard {
    event_pump: sdl2::EventPump,
    bindings: [(u8, Keycode); 16],
    memory: [u8; 16],
}

impl Keyboard {
    // (Chip8 key, keyboard key)
    const BINDINGS: [(u8, Keycode); 16] = [
        (0x1, Keycode::Num1),
        (0x2, Keycode::Num2),
        (0x3, Keycode::Num3),
        (0xC, Keycode::Num4),

        (0x4, Keycode::Q),
        (0x5, Keycode::W),
        (0x6, Keycode::E),
        (0xD, Keycode::R),

        (0x7, Keycode::A),
        (0x8, Keycode::S),
        (0x9, Keycode::D),
        (0xE, Keycode::F),

        (0xA, Keycode::Z),
        (0x0, Keycode::X),
        (0xB, Keycode::C),
        (0xF, Keycode::V),
    ];

    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        Self {
            event_pump: sdl_context.event_pump().unwrap(),
            bindings: Self::BINDINGS,
            memory: [0; 16]
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.memory[key as usize] == 1
    }

    pub fn first_pressed_key(&self) -> Option<u8> {
        self.memory.iter().enumerate().find_map(|(i, &e)| if e == 1 { Some(i as u8) } else { None } )
    }

    pub fn is_any_key_pressed(&self) -> bool {
        self.memory.iter().any(|&e| e == 1)
    }

    pub fn pool(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    exit(0);
                },
                Event::KeyDown { keycode, ..} => {
                    println!("DOWN: {:?}", keycode.unwrap());

                    let result = self.bindings.iter().find_map(|&(int, ext)| {
                      if keycode.unwrap() == ext { Some(int) } else { None }
                    });

                    match result {
                        Some(internal_number) => { self.memory[internal_number as usize] = 1; },
                        _ => {}
                    }
                },
                Event::KeyUp { keycode, ..} => {
                    println!("UP: {:?}", keycode.unwrap());

                    let result = self.bindings.iter().find_map(|&(int, ext)| {
                        if keycode.unwrap() == ext { Some(int) } else { None }
                    });

                    match result {
                        Some(internal_number) => { self.memory[internal_number as usize] = 0; },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }
}

pub struct Emulator {
    font: Font,
    video: Video,
    keyboard: Keyboard,
    registers: Registers,
    memory: Memory,
    stack: Stack,
    pc: usize,
    i: u16,
    delay_timer: u16,
}

impl Emulator {
    const ROM_START: u16 = 512;
    const TIMER_HZ: u32 = 60;
    const HZ: u32 = 500;

    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        Self {
            font: Font::new(),
            video: Video::new(&sdl_context),
            keyboard: Keyboard::new(&sdl_context),
            registers: Registers::new(),
            memory: Memory::new(),
            stack: Stack::new(),
            pc: 0,
            i: 0,
            delay_timer: 0,
        }
    }

    pub fn run(&mut self) {
        let mut emulator_step_duration = Duration::new(
            0, 1_000_000_000u32 / Self::HZ);

        while self.is_running() {
            let start = Instant::now();
            for _ in 0..Self::HZ {
                self.exec_opcode(&self.read_opcode());
                self.keyboard.pool();
                self.video.refresh();
            }
            let opcodes_exec_time = Instant::now().duration_since(start);

            if opcodes_exec_time >= emulator_step_duration {
                std::thread::sleep(Duration::new(0, 0));
            } else {
                let sleep_time = emulator_step_duration - opcodes_exec_time;
                std::thread::sleep(Duration::new(0, sleep_time.as_nanos() as u32));
            }
        }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        let ustart = Self::ROM_START as usize;
        for (i, e) in rom.iter().enumerate() { self.memory[ustart + i] = *e; }
        self.pc = ustart as usize;
    }

    pub fn load_font(&mut self) {
        for (i, e) in self.font.memory.iter().enumerate() {
            self.memory[Font::START as usize + i] = *e;
        }
    }

    pub fn is_running(&self) -> bool {
        self.pc < self.memory.size() // && self.pc != 0x3dc
    }

    fn exec_opcode(&mut self, opcode : &Opcode) {
        match opcode.nibbles() {
            (0x0, _, _, 0x0) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "CLEAR");
                self.video.clear();
                self.increment_pc();
            }
            (0x0, _, _, 0xE) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "self.pc = self.stack.pop() as usize;");
                self.pc = self.stack.pop() as usize;
                self.increment_pc();
            }
            (0x1, _, _, _) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "JUMP");
                self.pc = opcode.nnn() as usize;
            }
            (0x2, _, _, _) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "CALL");
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
                let sprite = self.memory[self.i..(self.i + opcode.n() as u16)].to_vec();

                self.registers[0xF_u16] = self.video.draw_sprite(&sprite,
                    self.registers[opcode.x()],
                    self.registers[opcode.y()]);

                self.increment_pc();
            }
            (0xE, _, 0x9, 0xE) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "SKP");
                if self.keyboard.is_key_pressed(self.registers[opcode.x()]) {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            (0xE, _, 0xA, 0x1) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "SKNP");
                if !self.keyboard.is_key_pressed(self.registers[opcode.x()]) {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            (0xF, _, 0x0, 0x7) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                self.registers[opcode.x()] = 0_u8;
                self.increment_pc();
            }
            (0xF, _, 0x0, 0xA) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                if !self.keyboard.is_any_key_pressed() { return; }

                self.registers[opcode.x()] = self.keyboard.first_pressed_key().unwrap();
                self.increment_pc();
            }
            (0xF, _, 0x1, 0x5) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                // TODO
                self.increment_pc();
            }
            (0xF, _, 0x1, 0x8) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                //  TODO
                self.increment_pc();
            }
            (0xF, _, 0x1, 0xE) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "ADD");
                self.i = self.i + self.registers[opcode.x()] as u16;
                self.increment_pc();
            }
            (0xF, _, 0x2, 0x9) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                self.i = Font::START + (self.memory[self.registers[opcode.x()] as usize] * 5) as u16;
                self.increment_pc();
            }
            (0xF, _, 0x3, 0x3) => {
                print!("{}, op: {:x}, mem: {} ||=> ", self.state(), opcode, "LD");
                let reg_x = self.registers[opcode.x()];

                print!("reg_x: {}, ", reg_x);
                print!("BEFORE:, memory[i..i+2] = [{}, {}, {}] => ",
                    self.memory[self.i],
                    self.memory[self.i + 1],
                    self.memory[self.i + 2]);

                self.memory[self.i] = reg_x / 100;
                self.memory[self.i + 1] = (reg_x / 10) % 10;
                self.memory[self.i + 2] = reg_x % 10;

                println!("AFTER: memory[i..i+2] = [{}, {}, {}]",
                     self.memory[self.i],
                     self.memory[self.i + 1],
                     self.memory[self.i + 2]);

                self.increment_pc();
            }
            (0xF, _, 0x5, 0x5) => {
                print!("{}, op: {:x}, mem: {} ||=> ", self.state(), opcode, "LD");
                let x = opcode.x() as usize;

                print!("x: {}, registers[0..=x] = [", x);
                for (i, e) in self.registers[0..=x].iter().enumerate() {
                    print!("{} ", *e);
                    self.memory[self.i + i as u16] = *e;
                }
                println!("]");

                self.increment_pc();
            }
            (0xF, _, 0x6, 0x5) => {
                println!("{}, op: {:x}, mem: {}", self.state(), opcode, "LD");
                for i in 0..=opcode.x() {
                    self.registers[i] = self.memory[self.i + i as u16];
                }
                self.increment_pc();
            }
            (_,_,_,_) => {
                self.increment_pc();
                println!("UNKNOWN: {:x}", opcode);
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

    pub fn state(&self) -> String {
        format!("pc: {:#x}, reg: {:x}, stack: {}", self.pc, self.registers, self.stack)
    }
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
