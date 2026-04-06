use crate::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Display};
use minifb::{Key, Scale, Window, WindowOptions};

pub const FRAME_RATE: usize = 60;

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

    pub fn update(&mut self, display: &Display) {
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

    pub fn window_is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }
}
