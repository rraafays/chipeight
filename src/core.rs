use crate::utils;

use sdl2::EventPump;
use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window};
use std::thread::sleep;
use std::time::Duration;

pub fn main_loop(mut event_pump: EventPump, mut canvas: Canvas<Window>) {
    'running: loop {
        // todo: emulator cycle
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.clear();
        // todo: build texture
        canvas.present();
        sleep(Duration::new(0, 16 * 1000 * 1000));
    }
}

pub struct CPU {
    pub opcode: u16,
    pub memory: [u8; 4096],
    pub graphics: [u8; 64 * 32],

    pub register: [u8; 16],
    pub index_register: u16,
    pub program_counter: u16,

    pub delay_timer: u8,
    pub sound_timer: u8,

    pub stack: [u16; 16],
    pub stack_pointer: u16,

    pub keys: [u8; 16],
}

impl CPU {
    pub fn init() -> CPU {
        let mut cpu = CPU {
            opcode: 0,
            memory: [0; 4096],
            graphics: [0; 2048],

            register: [0; 16],
            index_register: 0,
            program_counter: 0x200,

            delay_timer: 0,
            sound_timer: 0,

            stack: [0; 16],
            stack_pointer: 0,

            keys: [0; 16],
        };

        for (index, byte) in utils::FONTSET.iter().enumerate() {
            cpu.memory[index] = *byte;
            println!("{index} = {byte}");
        }

        return cpu;
    }
}
