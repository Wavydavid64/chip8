#[derive(Debug)]
pub enum Instruction {
    ClearScreen,
    Call(usize),
    Return,
    Jump(usize),
    JumpIfVal {
        register: usize,
        value: u8,
    },
    JumpIfNotVal {
        register: usize,
        value: u8,
    },
    JumpIfReg {
        register_1: usize,
        register_2: usize,
    },
    JumpIfNotReg {
        register_1: usize,
        register_2: usize,
    },
    Set {
        register: u8,
        value: u8,
    },
    Add {
        register: u8,
        value: u8,
    },
    SetIndex(usize),
    Draw {
        x_register: u8,
        y_register: u8,
        height: u8,
    },
}
