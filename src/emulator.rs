extern crate sfml;

use crate::chip8::Chip8;
use crate::display::{HEIGHT, WIDTH};
use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::{ContextSettings, Event, Key, Style, VideoMode};

pub struct Emulator {
    chip8: Chip8,
    render_window: RenderWindow,
    window_zoom_factor: usize,
    key_map: [Key; 16],
}

impl Emulator {
    pub fn new(window_zoom_factor: usize) -> Emulator {
        let mut window = RenderWindow::new(
            VideoMode::new(
                (WIDTH * window_zoom_factor) as u32,
                (HEIGHT * window_zoom_factor) as u32,
                2,
            ),
            "chip8 emulator",
            Style::CLOSE,
            &ContextSettings::default(),
        );
        window.set_framerate_limit(60);

        Emulator {
            chip8: Chip8::new(),
            render_window: window,
            window_zoom_factor,
            key_map: [
                Key::Num1,
                Key::Num2,
                Key::Num3,
                Key::Num4,
                Key::Q,
                Key::W,
                Key::E,
                Key::R,
                Key::A,
                Key::S,
                Key::D,
                Key::F,
                Key::Z,
                Key::X,
                Key::C,
                Key::V,
            ],
        }
    }

    pub fn load_game(&mut self, file_name: &str) -> std::io::Result<()> {
        self.chip8.load_game(file_name)
    }

    pub fn run(&mut self) {
        while self.render_window.is_open() {
            // Handle events
            while let Some(event) = self.render_window.poll_event() {
                match event {
                    Event::Closed => self.render_window.close(),
                    _ => { /* do nothing */ }
                }
            }

            self.chip8_frame();
            self.draw_frame();
            self.update_keys();
        }
    }

    fn chip8_frame(&mut self) {
        for _ in 0..10 {
            self.chip8.step();
        }
        self.chip8.decrement_timers();
    }

    fn draw_frame(&mut self) {
        self.render_window.clear(Color::BLACK);
        self.draw_chip8_display();
        self.render_window.display();
    }

    fn update_keys(&mut self) {
        for (i, key) in self.key_map.iter().enumerate() {
            if key.is_pressed() {
                self.chip8.keypad.set_key_up(i);
            } else {
                self.chip8.keypad.set_key_down(i);
            }
        }
    }

    fn draw_chip8_display(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if self.chip8.display.is_pixel_set(x, y) {
                    self.draw_pixel(x, y);
                }
            }
        }
    }

    fn draw_pixel(&mut self, x: usize, y: usize) {
        let x = x as f32 * self.window_zoom_factor as f32;
        let y = y as f32 * self.window_zoom_factor as f32;
        let mut rectangle = RectangleShape::new();
        rectangle.set_position((x, y));
        rectangle.set_size((
            self.window_zoom_factor as f32,
            self.window_zoom_factor as f32,
        ));
        rectangle.set_fill_color(Color::WHITE);
        self.render_window.draw(&rectangle);
    }
}
