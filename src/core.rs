use crate::utils::{CPU, FONTSET};

use sdl2::EventPump;
use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window};
use std::thread::sleep;
use std::time::Duration;

pub fn main_loop(mut event_pump: EventPump, mut canvas: Canvas<Window>) {
    'running: loop {
        // todo: emulator cycle
        for event in event_pump.poll_iter() {
            match event {
                // todo: handle key presses
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

        for (index, byte) in FONTSET.iter().enumerate() {
            cpu.memory[index] = *byte;
            println!("{index} = {byte}");
        }

        return cpu;
    }

    pub fn cycle(mut self) {
        self.opcode = (self.memory[self.program_counter as usize] << 8) as u16
            | (self.memory[self.program_counter as usize + 1]) as u16;

        let first_nibble = &self.opcode >> 12;

        match first_nibble {
            // 0x00E0 -> clear display/graphics
            // 0x00EE -> return from subroutine
            0x0 => {
                if self.opcode == 0x00E0 {
                    self.graphics = [0; 2048];
                } else if self.opcode == 0x00EE {
                }
                self.increment_program_counter();
            }
            // 0x1NNN -> jump to location NNN
            0x1 => self.program_counter = self.opcode & 0x0FFF,
            // 0x2NNN -> call subroutine at NNN
            0x2 => {
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.increment_stack_pointer();
                self.program_counter = self.opcode & 0x0FFF
            }
            _ => {}
        }
    }

    pub fn increment_program_counter(mut self) {
        self.program_counter += 2; // every instruction is 2 bytes
    }

    pub fn increment_stack_pointer(mut self) {
        self.stack_pointer += 1;
    }
}
