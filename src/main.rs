extern crate sdl2;
mod chip8;

use std::{env, thread, usize};
use std::time::Duration;

const CHIP8_DEFAULT_WIDTH: usize = 64;
const CHIP8_DEFAULT_HEIGHT: usize = 32;
const CHIP8_MEMORY_SIZE: usize = 4096;

pub fn main() {
    let sleep_duration = Duration::from_millis(17);
    let sdl_context = sdl2::init().expect("Couldn't init SDL2.");

    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a ROM file.");

    let rom = chip8::rom::read_file(&filename);
    let mut display = chip8::display::Display::new(&sdl_context);
    let mut keyboardinput = chip8::keyboard::KeyboardInput::new(&sdl_context);
    let mut cpu = chip8::cpu::Cpu::new();
    cpu.load(&rom);

    let mut count: u8 = 0;
    while let Ok(keys) = keyboardinput.poll() {
        let graphics_changed = cpu.mainloop(keys);
        
        count += 1;

        if graphics_changed {
            display.draw(&cpu.graphics);
            cpu.graphics_changed = false;
        }

        if count >= 16 {
            count = 0;
            if cpu.sound_timer > 0 {cpu.sound_timer -=1}
            if cpu.delay_timer > 0 {cpu.delay_timer -=1}
            thread::sleep(sleep_duration);
        }
    }
}

