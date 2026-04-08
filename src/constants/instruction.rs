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
    JumpWithOffset {
        register: usize,
        address: usize,
    },
    Random {
        register: usize,
        value: u8,
    },
    Set {
        register: u8,
        value: u8,
    },
    SetReg {
        register_1: usize,
        register_2: usize,
    },
    SetRegToDT {
        register: usize,
    },
    SetDTToReg {
        register: usize,
    },
    SetSTToReg {
        register: usize,
    },
    Or {
        register_1: usize,
        register_2: usize,
    },
    And {
        register_1: usize,
        register_2: usize,
    },
    Xor {
        register_1: usize,
        register_2: usize,
    },
    Add {
        register: u8,
        value: u8,
    },
    AddReg {
        register_1: usize,
        register_2: usize,
    },
    Sub {
        register_1: usize,
        register_2: usize,
    },
    SubN {
        register_1: usize,
        register_2: usize,
    },
    ShiftLeft {
        register_1: usize,
        register_2: usize,
    },
    ShiftRight {
        register_1: usize,
        register_2: usize,
    },
    StoreMemory {
        register: usize,
    },
    LoadMemory {
        register: usize,
    },
    DecimalConversion {
        register: usize,
    },
    SkipIfKey {
        register: usize,
    },
    SkipIfNotKey {
        register: usize,
    },
    GetKey {
        register: usize,
    },
    FontCharacter {
        register: usize,
    },
    SetIndex(usize),
    AddIndex {
        register: usize,
    },
    Draw {
        x_register: u8,
        y_register: u8,
        height: u8,
    },
}
