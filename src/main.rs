mod constants;
mod cpu;
mod display;
mod memory;
mod renderer;

use std::thread::sleep;
use std::time::Duration;

use crate::cpu::CPU;
use crate::display::Display;
use crate::memory::Memory;
use crate::renderer::Renderer;

fn main() {
    let filepath = "/Users/david/Desktop/code/chip8/src/programs/test_opcode.ch8";
    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut renderer = Renderer::new();

    memory.load_program(filepath).expect("File failed to load!");

    while renderer.window_is_open() {
        let instruction = cpu.fetch(&memory);
        println!("{instruction:x}");
        let instruction = cpu.decode(instruction).expect("Invalid instruction seen!");
        println!("{instruction:?}");
        cpu.execute(instruction, &mut display, &memory, &mut stack);
        renderer.update(&display);
        sleep(Duration::from_millis(200));
    }
}
