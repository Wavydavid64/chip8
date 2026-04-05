use crate::memory::Memory;

pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;

pub struct Display {
    screen: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Self {
            screen: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }

    pub fn clear_screen(&mut self) {
        self.screen = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    pub fn draw(
        &mut self,
        x_coord: usize,
        y_coord: usize,
        height: usize,
        sprite_address: usize,
        memory: &Memory,
    ) -> u8 {
        let mut turned_pixel_off = 0;

        for row in 0..height {
            let sprite_data = memory.read(sprite_address + row);
            let display_y_coord = y_coord + row;
            if display_y_coord >= DISPLAY_HEIGHT {
                break;
            }
            for col in 0..8 {
                let bit = (sprite_data >> (7 - col)) & 1;
                let bit = bit == 1;
                let display_x_coord = x_coord + col;
                if display_x_coord >= DISPLAY_WIDTH {
                    break;
                }
                if bit {
                    if self.screen[display_y_coord][display_x_coord] {
                        turned_pixel_off = 1;
                    }
                    self.screen[display_y_coord][display_x_coord] =
                        !self.screen[display_y_coord][display_x_coord];
                }
            }
        }

        turned_pixel_off
    }

    pub fn get_pixel(&self, y: usize, x: usize) -> bool {
        self.screen[y][x]
    }
}
