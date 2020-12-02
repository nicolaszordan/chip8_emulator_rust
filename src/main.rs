use chip8::emulator::Emulator;
use clap::{App, Arg};

fn main() {
    let args_matches = App::new("chip8_emulator")
        .arg(
            Arg::with_name("ROM")
                .required(true)
                .index(1)
                .help("rom file to launch"),
        )
        .get_matches();

    let rom_file = args_matches.value_of("ROM").unwrap();
    let mut chip8 = Emulator::new(20);
    chip8.load_game(rom_file).expect("failed to load game");
    chip8.run();
}
