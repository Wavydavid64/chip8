use crate::constants::instruction::Instruction;
use crate::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Display};
use crate::memory::{Memory, STARTING_ADDRESS};
use std::io;

pub struct CPU {
    program_counter: usize,
    variable_registers: [u8; 16],
    index_register: usize,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            program_counter: STARTING_ADDRESS,
            variable_registers: [0; 16],
            index_register: 0,
        }
    }

    pub fn fetch(&mut self, memory: &Memory) -> u16 {
        let instruction = ((memory.read(self.program_counter) as u16) << 8)
            | memory.read(self.program_counter + 1) as u16;
        self.program_counter += 2;
        instruction
    }

    pub fn decode(&self, instruction: u16) -> Result<Instruction, io::Error> {
        let hex_vals: (u16, u16, u16, u16) = (
            (instruction & 0xF000) >> 12,
            (instruction & 0x0F00) >> 8,
            (instruction & 0x00F0) >> 4,
            instruction & 0x000F,
        );
        match hex_vals {
            (0x0, 0x0, 0xe, 0x0) => Ok(Instruction::ClearScreen),
            (0x1, a, b, c) => {
                let address = a << 8 | b << 4 | c;
                Ok(Instruction::Jump(address as usize))
            }
            (0x6, register, b, c) => {
                let register = register as u8;
                let value = (b << 4 | c) as u8;
                Ok(Instruction::Set { register, value })
            }
            (0x7, register, b, c) => {
                let register = register as u8;
                let value = (b << 4 | c) as u8;
                Ok(Instruction::Add { register, value })
            }
            (0xa, a, b, c) => {
                let value: u16 = a << 8 | b << 4 | c;
                Ok(Instruction::SetIndex(value as usize))
            }
            (0xd, x_register, y_register, height) => Ok(Instruction::Draw {
                x_register: x_register as u8,
                y_register: y_register as u8,
                height: height as u8,
            }),
            (a, b, c, d) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid Instuction Seen! (0x{a}{b}{c}{d})"),
            )),
        }
    }

    pub fn execute(&mut self, instruction: Instruction, display: &mut Display, memory: &Memory) {
        match instruction {
            Instruction::ClearScreen => display.clear_screen(),
            Instruction::Jump(address) => self.program_counter = address,
            Instruction::Set { register, value } => {
                self.variable_registers[register as usize] = value
            }
            Instruction::Add { register, value } => {
                self.variable_registers[register as usize] += value
            }
            Instruction::SetIndex(value) => self.index_register = value,
            Instruction::Draw {
                x_register,
                y_register,
                height,
            } => {
                let x_coord =
                    (self.variable_registers[x_register as usize] as usize) % DISPLAY_WIDTH;
                let y_coord =
                    (self.variable_registers[y_register as usize] as usize) % DISPLAY_HEIGHT;
                let sprite_address = self.index_register;
                self.variable_registers[15] =
                    display.draw(x_coord, y_coord, height as usize, sprite_address, memory);
            }
        }
    }
}
