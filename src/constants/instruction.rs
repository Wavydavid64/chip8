#[derive(Debug)]
pub enum Instruction {
    ClearScreen,
    Jump(usize),
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
