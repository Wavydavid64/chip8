pub struct Keypad {
    keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn set_key(&mut self, key: usize, is_pressed: bool) {
        self.keys[key] = is_pressed;
    }

    pub fn get_key(&self, key: usize) -> bool {
        self.keys[key]
    }

    pub fn get_any_pressed_key(&self) -> Option<usize> {
        for (key_index, key) in self.keys.iter().enumerate() {
            if *key {
                return Some(key_index);
            }
        }
        return None;
    }
}
