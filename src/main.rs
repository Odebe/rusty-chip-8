use std::fs::File;
use std::io::Read;
use clap::{Arg, App};

mod chip8;

fn main() {
    let opt_matches = App::new("WIP: Rusty Chip8 emulator")
        .version("0.1.0")
        .author("Mihail Odebe <derpiranha@gmail.com>")
        .about("Emulates Chip8")
        .arg(Arg::new("rom")
            .short('r')
            .long("rom")
            .value_name("ROM_PATH")
            .about("path to rom file")
            .takes_value(true))
        .get_matches();

    if let Some(rom_path) = opt_matches.value_of("rom") {
        let mut file = File::open(&rom_path).expect("no rom file found");
        let mut rom_buffer = Vec::new();
        file.read_to_end(&mut rom_buffer).expect("buffer overflow");

        let mut cpu = chip8::Emulator::new();
        cpu.load_rom(&rom_buffer, );
        cpu.load_font();
        cpu.run();
    } else {
        println!("ROM file not specified. Try run with --help flag")
    }
}
