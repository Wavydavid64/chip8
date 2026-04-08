use crate::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Display};
use crate::keypad::Keypad;
use minifb::{Key, Scale, Window, WindowOptions};

pub const FRAME_RATE: usize = 60;

/// Keys that correspond to that index's value in hex
/// Ex. The last letter in this array represents F
/// Keyboard is written as such with Chip-8 on the left and QUERTY on the right:
/// 1/1 2/2 3/3 C/4
/// 4/Q 5/W 6/E D/R
/// 7/A 8/S 9/D E/F
/// A/Z 0/X B/C F/V
const KEYS: [Key; 16] = [
    Key::X,
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Q,
    Key::W,
    Key::E,
    Key::A,
    Key::S,
    Key::D,
    Key::Z,
    Key::C,
    Key::Key4,
    Key::R,
    Key::F,
    Key::V,
];

pub struct Renderer {
    window: Window,
    window_buffer: Vec<u32>,
}

impl Renderer {
    pub fn new() -> Self {
        let mut window = Window::new(
            "CHIP-8 Display",
            DISPLAY_WIDTH,
            DISPLAY_HEIGHT,
            WindowOptions {
                scale: Scale::X16,
                ..WindowOptions::default()
            },
        )
        .expect("Failed to make window!");
        let mut window_buffer: Vec<u32> = vec![0; DISPLAY_HEIGHT * DISPLAY_WIDTH];

        window.set_target_fps(FRAME_RATE);

        Self {
            window,
            window_buffer,
        }
    }

    pub fn update_display(&mut self, display: &Display) {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let pixel_val = if display.get_pixel(y, x) {
                    0xFFFFFFFF
                } else {
                    0x0
                };
                let buffer_index = y * DISPLAY_WIDTH + x;
                self.window_buffer[buffer_index] = pixel_val;
            }
        }
        self.window
            .update_with_buffer(&self.window_buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .expect("Failed to update window!");
    }

    pub fn update_keys(&mut self, keypad: &mut Keypad) {
        for (key_index, &key) in KEYS.iter().enumerate() {
            let is_pressed = self.window.is_key_down(key);
            keypad.set_key(key_index, is_pressed);
            if (is_pressed) {
                println!("KEY PRESSED: {key:?}");
            }
        }
    }

    pub fn window_is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }
}
