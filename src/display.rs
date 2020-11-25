pub enum DrawResult {
    Overwrite,
    NoOverwrite,
}

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    memory: [u8; WIDTH * HEIGHT],
}

impl Default for Display {
    fn default() -> Display {
        Display::new()
    }
}

impl Display {
    pub fn new() -> Display {
        Display {
            memory: [0; WIDTH * HEIGHT],
        }
    }

    pub fn reset(&mut self) {
        self.clear()
    }

    pub fn dump(&self) {
        println!("{:0X?}", self.memory);
    }

    pub fn clear(&mut self) {
        self.memory.iter_mut().for_each(|m| *m = 0)
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> DrawResult {
        let mut res = DrawResult::NoOverwrite;
        for (j, byte) in sprite.iter().enumerate() {
            for i in 0..8 {
                let new_value = byte >> (7 - i) & 1;
                let old_value = self.is_pixel_set(x + i, y + j);
                if old_value {
                    res = DrawResult::Overwrite;
                }
                self.set_pixel(x + i, y + j, (new_value == 1) ^ old_value);
            }
        }
        res
    }

    pub fn is_pixel_set(&self, x: usize, y: usize) -> bool {
        self.memory[(x % WIDTH) + (y % HEIGHT) * WIDTH] == 1
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, state: bool) {
        self.memory[(x % WIDTH) + (y % HEIGHT) * WIDTH] = state as u8;
    }
}
