use chip8::chip8::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    chip8
        .load_game("test_opcode.ch8")
        .expect("failed to load game");

    chip8.run();
}
