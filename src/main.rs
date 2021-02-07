use std::fs::File;
use std::io::Read;

mod chip8;

fn main() {
    // let filename = "roms/c8_test.c8";
    let filename = "roms/test_opcode.ch8";

    let mut file = File::open(&filename).expect("no file found");
    let mut rom_buffer = Vec::new();
    file.read_to_end(&mut rom_buffer).expect("buffer overflow");

    let mut cpu = chip8::Emulator::new();
    cpu.load_rom(&rom_buffer, );
    cpu.load_font();
    cpu.run();

    // while cpu.is_running() {
    //     cpu.exec_cycle()
    // }
}
