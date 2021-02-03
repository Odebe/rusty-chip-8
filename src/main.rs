use std::fs::File;
use std::io::Read;

mod chip8;

fn main() {
    let filename = "roms/test_opcode.ch8";
    let mut file = File::open(&filename).expect("no file found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("buffer overflow");

    let mut cpu = chip8::Emulator::with_rom(&buffer);
    while cpu.is_running() {
        cpu.exec_cycle()
    }
}
