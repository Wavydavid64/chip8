use std::io;

use rand::RngExt;

use crate::constants::font::CHARACTER_LEN;
use crate::constants::instruction::Instruction;
use crate::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Display};
use crate::keypad::Keypad;
use crate::memory::{Memory, STARTING_ADDRESS};

pub struct Cpu {
    program_counter: usize,
    variable_registers: [u8; 16],
    index_register: usize,
    delay_timer: u8,
    sound_timer: u8,
    legacy_mode: bool,
}

impl Cpu {
    pub fn new(legacy_mode: bool) -> Self {
        Self {
            program_counter: STARTING_ADDRESS,
            variable_registers: [0; 16],
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            legacy_mode,
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
            (0x0, 0x0, 0xe, 0xe) => Ok(Instruction::Return),
            (0x1, a, b, c) => {
                let address = a << 8 | b << 4 | c;
                Ok(Instruction::Jump(address as usize))
            }
            (0x2, a, b, c) => {
                let address = a << 8 | b << 4 | c;
                Ok(Instruction::Call(address as usize))
            }
            (0x3, register, b, c) => {
                let value = (b << 4 | c) as u8;
                let register = register as usize;
                Ok(Instruction::JumpIfVal { register, value })
            }
            (0x4, register, b, c) => {
                let value = (b << 4 | c) as u8;
                let register = register as usize;
                Ok(Instruction::JumpIfNotVal { register, value })
            }
            (0x5, register_1, register_2, 0x0) => Ok(Instruction::JumpIfReg {
                register_1: register_1 as usize,
                register_2: register_2 as usize,
            }),
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
            (0x8, register_1, register_2, 0x0) => Ok(Instruction::SetReg {
                register_1: register_1 as usize,
                register_2: register_2 as usize,
            }),
            (0x8, register_1, register_2, 0x1) => Ok(Instruction::Or {
                register_1: register_1 as usize,
                register_2: register_2 as usize,
            }),
            (0x8, register_1, register_2, 0x2) => Ok(Instruction::And {
                register_1: register_1 as usize,
                register_2: register_2 as usize,
            }),
            (0x8, register_1, register_2, 0x3) => Ok(Instruction::Xor {
                register_1: register_1 as usize,
                register_2: register_2 as usize,
            }),
            (0x8, register_1, register_2, 0x4) => Ok(Instruction::AddReg {
                register_1: register_1 as usize,
                register_2: register_2 as usize,
            }),
            (0x8, register_1, register_2, 0x5) => Ok(Instruction::Sub {
                register_1: register_1 as usize,
                register_2: register_2 as usize,
            }),
            (0x8, register_1, register_2, 0x6) => Ok(Instruction::ShiftRight {
                register_1: register_1 as usize,
                register_2: register_2 as usize,
            }),
            (0x8, register_1, register_2, 0x7) => Ok(Instruction::SubN {
                register_1: register_1 as usize,
                register_2: register_2 as usize,
            }),
            (0x8, register_1, register_2, 0xe) => Ok(Instruction::ShiftLeft {
                register_1: register_1 as usize,
                register_2: register_2 as usize,
            }),
            (0x9, register_1, register_2, 0x0) => Ok(Instruction::JumpIfNotReg {
                register_1: register_1 as usize,
                register_2: register_2 as usize,
            }),
            (0xa, a, b, c) => {
                let value: u16 = a << 8 | b << 4 | c;
                Ok(Instruction::SetIndex(value as usize))
            }
            (0xb, register, b, c) => {
                let address: u16 = register << 8 | b << 4 | c;
                Ok(Instruction::JumpWithOffset {
                    register: register as usize,
                    address: address as usize,
                })
            }
            (0xc, register, b, c) => {
                let value: u16 = b << 4 | c;
                Ok(Instruction::Random {
                    register: register as usize,
                    value: value as u8,
                })
            }
            (0xd, x_register, y_register, height) => Ok(Instruction::Draw {
                x_register: x_register as u8,
                y_register: y_register as u8,
                height: height as u8,
            }),
            (0xe, register, 0x9, 0xe) => Ok(Instruction::SkipIfKey {
                register: register as usize,
            }),
            (0xe, register, 0xa, 0x1) => Ok(Instruction::SkipIfNotKey {
                register: register as usize,
            }),
            (0xf, register, 0x0, 0x7) => Ok(Instruction::SetRegToDT {
                register: register as usize,
            }),
            (0xf, register, 0x0, 0xa) => Ok(Instruction::GetKey {
                register: register as usize,
            }),
            (0xf, register, 0x1, 0x5) => Ok(Instruction::SetDTToReg {
                register: register as usize,
            }),
            (0xf, register, 0x1, 0x8) => Ok(Instruction::SetSTToReg {
                register: register as usize,
            }),
            (0xf, register, 0x1, 0xe) => Ok(Instruction::AddIndex {
                register: register as usize,
            }),
            (0xf, register, 0x2, 0x9) => Ok(Instruction::FontCharacter {
                register: register as usize,
            }),
            (0xf, register, 0x3, 0x3) => Ok(Instruction::DecimalConversion {
                register: register as usize,
            }),
            (0xf, register, 0x5, 0x5) => Ok(Instruction::StoreMemory {
                register: register as usize,
            }),
            (0xf, register, 0x6, 0x5) => Ok(Instruction::LoadMemory {
                register: register as usize,
            }),
            (a, b, c, d) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid Instuction Seen! (0x{a:x}{b:x}{c:x}{d:x})"),
            )),
        }
    }

    pub fn execute(
        &mut self,
        instruction: Instruction,
        display: &mut Display,
        keypad: &Keypad,
        memory: &mut Memory,
        stack: &mut Vec<usize>,
    ) -> bool {
        let mut regenerate_display = false;
        match instruction {
            Instruction::ClearScreen => {
                display.clear_screen();
                regenerate_display = true;
            }
            Instruction::Jump(address) => self.program_counter = address,
            Instruction::Call(address) => {
                stack.push(self.program_counter);
                self.program_counter = address;
            }
            Instruction::Return => {
                self.program_counter = stack.pop().expect("Nothing in the stack!");
            }
            Instruction::JumpIfVal { register, value } => {
                if self.variable_registers[register] == value {
                    self.program_counter += 2
                }
            }
            Instruction::JumpIfNotVal { register, value } => {
                if self.variable_registers[register] != value {
                    self.program_counter += 2
                }
            }
            Instruction::JumpIfReg {
                register_1,
                register_2,
            } => {
                if self.variable_registers[register_1] == self.variable_registers[register_2] {
                    self.program_counter += 2
                }
            }
            Instruction::JumpIfNotReg {
                register_1,
                register_2,
            } => {
                if self.variable_registers[register_1] != self.variable_registers[register_2] {
                    self.program_counter += 2
                }
            }
            Instruction::JumpWithOffset { register, address } => {
                let mut address = address;
                if self.legacy_mode {
                    address += self.variable_registers[0] as usize;
                } else {
                    address += self.variable_registers[register] as usize;
                }
                self.program_counter = address;
            }
            Instruction::Random { register, value } => {
                let rand_val = rand::rng().random_range(0..=u8::MAX) & value;
                self.variable_registers[register] = rand_val;
            }
            Instruction::Set { register, value } => {
                self.variable_registers[register as usize] = value
            }
            Instruction::Add { register, value } => {
                self.variable_registers[register as usize] =
                    self.variable_registers[register as usize].wrapping_add(value);
            }
            Instruction::SetReg {
                register_1,
                register_2,
            } => self.variable_registers[register_1] = self.variable_registers[register_2],
            Instruction::Or {
                register_1,
                register_2,
            } => {
                self.variable_registers[register_1] |= self.variable_registers[register_2];
                if self.legacy_mode {
                    self.variable_registers[15] = 0;
                }
            }
            Instruction::And {
                register_1,
                register_2,
            } => {
                self.variable_registers[register_1] &= self.variable_registers[register_2];
                if self.legacy_mode {
                    self.variable_registers[15] = 0;
                }
            }
            Instruction::Xor {
                register_1,
                register_2,
            } => {
                self.variable_registers[register_1] ^= self.variable_registers[register_2];
                if self.legacy_mode {
                    self.variable_registers[15] = 0;
                }
            }
            Instruction::AddReg {
                register_1,
                register_2,
            } => {
                let reg_1_val = self.variable_registers[register_1];
                let reg_2_val = self.variable_registers[register_2];
                let (val, overflowed) = reg_1_val.overflowing_add(reg_2_val);
                self.variable_registers[register_1] = val;

                if overflowed {
                    self.variable_registers[15] = 1;
                } else {
                    self.variable_registers[15] = 0;
                }
            }
            Instruction::Sub {
                register_1,
                register_2,
            } => {
                let reg_1_val = self.variable_registers[register_1];
                let reg_2_val = self.variable_registers[register_2];
                let (val, overflowed) = reg_1_val.overflowing_sub(reg_2_val);
                self.variable_registers[register_1] = val;

                if overflowed {
                    self.variable_registers[15] = 0;
                } else {
                    self.variable_registers[15] = 1;
                }
            }
            Instruction::SubN {
                register_1,
                register_2,
            } => {
                let reg_1_val = self.variable_registers[register_1];
                let reg_2_val = self.variable_registers[register_2];
                let (val, overflowed) = reg_2_val.overflowing_sub(reg_1_val);
                self.variable_registers[register_1] = val;

                if overflowed {
                    self.variable_registers[15] = 0;
                } else {
                    self.variable_registers[15] = 1;
                }
            }
            Instruction::ShiftRight {
                register_1,
                register_2,
            } => {
                if self.legacy_mode {
                    self.variable_registers[register_1] = self.variable_registers[register_2];
                }
                let reg_1_val = self.variable_registers[register_1];
                self.variable_registers[register_1] = reg_1_val >> 1;
                self.variable_registers[15] = reg_1_val & 1;
            }
            Instruction::ShiftLeft {
                register_1,
                register_2,
            } => {
                if self.legacy_mode {
                    self.variable_registers[register_1] = self.variable_registers[register_2];
                }
                let reg_1_val = self.variable_registers[register_1];
                self.variable_registers[register_1] = reg_1_val << 1;
                self.variable_registers[15] = reg_1_val & 1;
                self.variable_registers[15] = (reg_1_val >> 7) & 1;
            }
            Instruction::SetIndex(value) => self.index_register = value,
            Instruction::StoreMemory { register } => {
                for reg_num in 0..=register {
                    let reg_val = self.variable_registers[reg_num];
                    memory.write(self.index_register + reg_num, reg_val);
                }
                if self.legacy_mode {
                    self.index_register += register + 1;
                }
            }
            Instruction::LoadMemory { register } => {
                for reg_num in 0..=register {
                    let memory_val = memory.read(self.index_register + reg_num);
                    self.variable_registers[reg_num] = memory_val;
                }
                if self.legacy_mode {
                    self.index_register += register + 1;
                }
            }
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
                regenerate_display = true;
            }
            Instruction::DecimalConversion { register } => {
                let reg_val = self.variable_registers[register];
                memory.write(self.index_register, (reg_val / 100) % 10);
                memory.write(self.index_register + 1, (reg_val / 10) % 10);
                memory.write(self.index_register + 2, reg_val % 10);
            }
            Instruction::SetRegToDT { register } => {
                self.variable_registers[register] = self.delay_timer;
            }
            Instruction::SetDTToReg { register } => {
                self.delay_timer = self.variable_registers[register];
            }
            Instruction::SetSTToReg { register } => {
                self.sound_timer = self.variable_registers[register];
            }
            Instruction::AddIndex { register } => {
                self.index_register += self.variable_registers[register] as usize;
                if !self.legacy_mode && self.index_register >= 0x1000 {
                    self.variable_registers[register] = 1
                };
            }
            Instruction::SkipIfKey { register } => {
                let key = self.variable_registers[register];
                if keypad.get_key_state(key as usize) {
                    self.program_counter += 2;
                }
            }
            Instruction::SkipIfNotKey { register } => {
                let key = self.variable_registers[register];
                if !keypad.get_key_state(key as usize) {
                    self.program_counter += 2;
                }
            }
            Instruction::GetKey { register } => {
                let get_key_func = |keypad: &Keypad| {
                    if self.legacy_mode {
                        keypad.get_any_released_key()
                    } else {
                        keypad.get_any_pressed_key()
                    }
                };
                if let Some(key) = get_key_func(keypad) {
                    println!("{key}");
                    self.variable_registers[register] = key as u8;
                } else {
                    // Decrement program counter to halt till key is pressed
                    self.program_counter -= 2;
                }
            }
            Instruction::FontCharacter { register } => {
                let character = self.variable_registers[register];
                self.index_register = (character as usize) * CHARACTER_LEN;
            }
        }
        regenerate_display
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.delay_timer
    }
}
