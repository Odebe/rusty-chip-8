use std::fs::File;
use std::io::Read;

#[path = "chip8/chip8.rs"] mod chip8;

fn main() {
    let filename = "roms/test_opcode.ch8";

    let mut file = File::open(&filename).expect("no file found");
    let mut buffer : Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("buffer overflow");

    let mut cpu = chip8::Emulator::new(&mut buffer);
    while cpu.is_running() {
        let opcode = cpu.read_opcode();
        println!("{}, {}", cpu.state(), opcode);
        cpu.next_opcode();
    }
}
