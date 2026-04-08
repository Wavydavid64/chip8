mod audio;
mod constants;
mod cpu;
mod display;
mod keypad;
mod memory;
mod renderer;

use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::audio::Audio;
use crate::cpu::Cpu;
use crate::display::Display;
use crate::keypad::Keypad;
use crate::memory::Memory;
use crate::renderer::{FRAME_RATE, Renderer};

pub const CYCLES_PER_SECOND: usize = 700;

fn main() {
    let filepath = "/Users/david/Desktop/code/chip8/src/programs/chip8-test-suite/5-quirks.ch8";
    let mut cpu = Cpu::new(true);
    let mut memory = Memory::new();
    let mut display = Display::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut renderer = Renderer::new();
    let audio = Audio::new();
    let mut keypad = Keypad::new();

    memory.load_program(filepath).expect("File failed to load!");

    let mut timer_tick: Instant = Instant::now();
    let timer_rate = Duration::from_secs_f64(1.0 / (FRAME_RATE as f64));

    let cycles_per_frame = CYCLES_PER_SECOND / FRAME_RATE;

    while renderer.window_is_open() {
        let start = Instant::now();
        for _ in 0..cycles_per_frame {
            renderer.update_keys(&mut keypad);
            let instruction = cpu.fetch(&memory);
            let instruction = cpu.decode(instruction).expect("Invalid instruction seen!");
            let regenerate_display =
                cpu.execute(instruction, &mut display, &keypad, &mut memory, &mut stack);
            let time_elapsed = Instant::now() - start;
            if regenerate_display && time_elapsed < timer_rate {
                sleep(time_elapsed);
                break;
            }
        }
        renderer.update_display(&display);
        if Instant::now() > timer_tick + timer_rate {
            cpu.decrement_timers();
            timer_tick = Instant::now();
        }
        let sound_timer = cpu.get_sound_timer();
        if sound_timer > 0 {
            audio.play_tone();
        } else {
            audio.pause_tone();
        }
    }
}
