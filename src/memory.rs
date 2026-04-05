use std::io;

use crate::constants::font::FONT;

/// The size of memory in bytes
const MEMORY_SIZE: usize = 4096;

/// The byte to start loading programs at.
pub const STARTING_ADDRESS: usize = 0x200;

pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Self {
            data: [0u8; MEMORY_SIZE],
        };
        memory.load_font();
        memory
    }

    pub fn read(&self, address: usize) -> u8 {
        self.data[address]
    }

    pub fn write(&mut self, address: usize, value: u8) {
        self.data[address] = value;
    }

    pub fn clear(&mut self) {
        self.data[STARTING_ADDRESS..].fill(0);
    }

    pub fn load_program(&mut self, filepath: &str) -> Result<(), io::Error> {
        self.clear();
        let bytes = std::fs::read(filepath)?;
        let bytes_len = bytes.len();
        let max_size = self.data.len() - STARTING_ADDRESS;
        if bytes_len > max_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "File provided has too much data! ({bytes_len} with max {max_size)",
            ));
        }
        self.data[STARTING_ADDRESS..STARTING_ADDRESS + bytes.len()].copy_from_slice(&bytes);
        Ok(())
    }

    fn load_font(&mut self) {
        self.data[0..FONT.len()].copy_from_slice(&FONT);
    }
}
