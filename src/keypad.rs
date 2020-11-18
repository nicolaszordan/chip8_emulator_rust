#[derive(PartialEq, Eq, Copy, Clone)]
pub enum KeyState {
    Up,
    Down,
}

pub struct Keypad {
    pub keys: [KeyState; 16],
}

impl Default for Keypad {
    fn default() -> Keypad {
        Keypad::new()
    }
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: [KeyState::Up; 16],
        }
    }

    pub fn reset(&mut self) {
        self.keys.iter_mut().for_each(|key| *key = KeyState::Up)
    }

    pub fn is_key_up(&self, key: usize) -> bool {
        self.keys[key].is_key_up()
    }

    pub fn is_key_down(&self, key: usize) -> bool {
        self.keys[key].is_key_down()
    }

    pub fn set_key_up(&mut self, key: usize) {
        self.keys[key].set_key_up()
    }

    pub fn set_key_down(&mut self, key: usize) {
        self.keys[key].set_key_down()
    }
}

impl KeyState {
    pub fn is_key_up(&self) -> bool {
        *self == KeyState::Up
    }

    pub fn is_key_down(&self) -> bool {
        *self == KeyState::Down
    }

    pub fn set_key_up(&mut self) {
        *self = KeyState::Up
    }

    pub fn set_key_down(&mut self) {
        *self = KeyState::Down
    }
}
