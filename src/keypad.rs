pub struct Keypad {
    keys: [(bool, bool); 16],
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            keys: [(false, false); 16],
        }
    }

    pub fn set_key_state(&mut self, key: usize, is_pressed: bool) {
        self.keys[key].0 = self.keys[key].1;
        self.keys[key].1 = is_pressed;
    }

    pub fn get_key_state(&self, key: usize) -> bool {
        self.keys[key].1
    }

    pub fn get_any_pressed_key(&self) -> Option<usize> {
        for (key_index, key) in self.keys.iter().enumerate() {
            if key.1 {
                return Some(key_index);
            }
        }
        None
    }

    pub fn get_any_released_key(&self) -> Option<usize> {
        for (key_index, key) in self.keys.iter().enumerate() {
            if key.0 && !key.1 {
                return Some(key_index);
            }
        }
        None
    }
}
