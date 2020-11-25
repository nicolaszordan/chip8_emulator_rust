use chip8::emulator::Emulator;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut chip8 = Emulator::new(20);
    chip8
        .load_game(&args[1])
        .expect("failed to load game");

    chip8.run();
}
