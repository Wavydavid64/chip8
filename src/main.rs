mod audio;
mod constants;
mod cpu;
mod display;
mod memory;
mod renderer;

use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::audio::Audio;
use crate::cpu::CPU;
use crate::display::Display;
use crate::memory::Memory;
use crate::renderer::{FRAME_RATE, Renderer};

fn main() {
    let filepath = "/Users/david/Desktop/code/chip8/src/programs/chip8-test-rom-with-audio.ch8";
    let mut cpu = CPU::new(false);
    let mut memory = Memory::new();
    let mut display = Display::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut renderer = Renderer::new();
    let mut audio = Audio::new();

    memory.load_program(filepath).expect("File failed to load!");

    let mut tick = Instant::now();
    let rate = Duration::from_secs_f64(1.0 / (FRAME_RATE as f64));

    while renderer.window_is_open() {
        let instruction = cpu.fetch(&memory);
        println!("{instruction:x}");
        let instruction = cpu.decode(instruction).expect("Invalid instruction seen!");
        println!("{instruction:?}");
        cpu.execute(instruction, &mut display, &mut memory, &mut stack);
        renderer.update(&display);
        if Instant::now() > tick + rate {
            cpu.decrement_timers();
            tick = Instant::now();
        }
        let sound_timer = cpu.get_sound_timer();
        if sound_timer > 0 {
            audio.play_tone();
        } else {
            audio.pause_tone();
        }
        sleep(Duration::from_millis(50));
    }
}
