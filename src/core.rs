use crate::utils::{CPU, FONTSET};

use rand::random;
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
                    self.stack_pointer -= 1;
                    self.program_counter = self.stack[self.stack_pointer as usize];
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
            // 0x3XKK -> skip next instruction if Vx == kk
            0x3 => {
                let x = (self.opcode & 0x0F00) >> 8;
                if self.register[x as usize] == self.opcode as u8 & 0x00FF {
                    self.increment_program_counter();
                }
                self.increment_program_counter();
            }
            // 0x4XKK -> skip next instruction if Vx != kk
            0x4 => {
                let x = (self.opcode & 0x0F00) >> 8;
                if self.register[x as usize] != self.opcode as u8 & 0x00FF {
                    self.increment_program_counter();
                }
                self.increment_program_counter();
            }
            // 0x5XY0 -> skip next instruction if Vx == Vy
            0x5 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let y = (self.opcode & 0x00F0) >> 4;
                if self.register[x as usize] == self.register[y as usize] {
                    self.increment_program_counter();
                }
                self.increment_program_counter();
            }
            // 0x6XKK -> set Vx = kk
            0x6 => {
                let x = (self.opcode & 0x0F00) >> 8;
                self.register[x as usize] = (self.opcode & 0x00FF) as u8;
                self.increment_program_counter();
            }
            // 0x7XKK -> set Vx = Vx + kk
            0x7 => {
                let x = (self.opcode & 0x0F00) >> 8;
                self.register[x as usize] += (self.opcode & 0x00FF) as u8;
                self.increment_program_counter();
            }
            // 0x8XY0 -> set Vx = Vy
            // 0x8XY1 -> set Vx = Vx | Vy
            // 0x8XY2 -> set Vx = Vx & Vy
            // 0x8XY3 -> set Vx = Vx ^ Vy
            // 0x8XY4 -> set Vx = Vx + Vy (carry)
            // 0x8XY5 -> set Vx = Vx - Vy (not borrow)
            // 0x8XY6 -> set Vx = Vx SHR 1
            // 0x8XY7 -> set Vx = Vy - Vx (not borrow)
            // 0x8XYE -> set Vx = Vx SHL 1
            0x8 => {
                let x = (self.opcode & 0x0F00) >> 8;
                let y = (self.opcode & 0x00F0) >> 4;
                let operation = self.opcode & 0x000F;

                match operation {
                    0 => self.register[x as usize] = self.register[y as usize],
                    1 => self.register[x as usize] |= self.register[y as usize],
                    2 => self.register[x as usize] &= self.register[y as usize],
                    3 => self.register[x as usize] ^= self.register[y as usize],
                    4 => {
                        let mut sum: u16 = self.register[x as usize] as u16;
                        sum += self.register[y as usize] as u16;
                        self.register[0xF] = if sum > 255 { 1 } else { 0 };
                        self.register[x as usize] = (sum & 0x00FF) as u8;
                    }
                    5 => {
                        self.register[0xF] =
                            if self.register[x as usize] > self.register[y as usize] {
                                1
                            } else {
                                0
                            };
                        self.register[x as usize] -= self.register[y as usize];
                    }
                    6 => {
                        self.register[0xF] = self.register[0xF] & 1u8;
                        self.register[x as usize] >>= 1;
                    }
                    7 => {
                        self.register[0xF] =
                            if self.register[y as usize] > self.register[x as usize] {
                                1
                            } else {
                                0
                            };
                        self.register[x as usize] =
                            self.register[y as usize] - self.register[x as usize];
                    }
                    14 => {
                        self.register[0xF] = if self.register[x as usize] & 0x80 != 0 {
                            1
                        } else {
                            0
                        };
                        self.register[x as usize] <<= 1;
                    }
                    _ => panic!("error: unimplemented instruction!"),
                }
                self.increment_program_counter();
            }
            // 0x9XY0 -> skip next instruction if Vx != Vy
            0x9 => {
                let x = self.opcode & 0x0F00 >> 8;
                let y = self.opcode & 0x00F0 >> 4;

                if self.register[x as usize] != self.register[y as usize] {
                    self.increment_program_counter();
                }
                self.increment_program_counter();
            }
            // 0xANNN -> set index = NNN
            0xA => {
                self.index_register = self.opcode & 0x0FFF;
                self.increment_program_counter();
            }
            // 0xBNNN -> jump to location NNN
            0xB => self.program_counter = self.opcode & 0x0FFF + self.register[0] as u16,
            // 0xCXKK -> set Vx = RND & kk
            0xC => {
                let x = self.opcode & 0x0F00 >> 8;
                let kk = self.opcode & 0x00FF;

                self.register[x as usize] = random::<u8>() & kk as u8;
                self.increment_program_counter();
            }
            // 0xDXYN -> display n byte sprite starting at memory location I at location Vx, Vy
            0xD => {
                self.register[0xF] = 0;
                let x = self.opcode & 0x0F00 >> 8;
                let y = self.opcode & 0x00F0 >> 4;
                let n = self.opcode & 0x000F;

                let vx = self.register[x as usize];
                let vy = self.register[y as usize];
            }
        }
    }

    pub fn increment_program_counter(mut self) {
        self.program_counter += 2; // every instruction is 2 bytes
    }

    pub fn increment_stack_pointer(mut self) {
        self.stack_pointer += 1;
    }
}
