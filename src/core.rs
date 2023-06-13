use crate::utils::cpu::CPU;

use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window};
use sdl2::{EventPump, Sdl, VideoSubsystem};
use std::thread::sleep;
use std::time::Duration;

const WIDTH: u16 = 1024;
const HEIGHT: u16 = 512;

pub fn init() {
    let sdl_context: Sdl = match sdl2::init() {
        Ok(sdl_context) => sdl_context,
        Err(_) => panic!("failed to initialise sdl context"),
    };

    let video_subsystem: VideoSubsystem = match sdl_context.video() {
        Ok(video_subsystem) => video_subsystem,
        Err(_) => panic!("failed to initialise video subsystem"),
    };

    let window: Window = video_subsystem
        .window("test", WIDTH.into(), HEIGHT.into())
        .position_centered()
        .build()
        .expect("failed to initialise window");

    let event_pump: EventPump = match sdl_context.event_pump() {
        Ok(event_pump) => event_pump,
        Err(_) => panic!("failed to initialise event pump"),
    };

    let canvas: Canvas<Window> = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(_) => panic!("failed to create canvas"),
    };

    let cpu = CPU {
        opcode: 0,
        memory: [0; 4096],
        graphics: [0; 2048],

        register: [0; 16],
        index_register: 0,
        program_counter: 0,

        delay_timer: 0,
        sound_timer: 0,

        stack: [0; 16],
        stack_pointer: 0,

        keys: [0; 16],
    };

    // todo: load rom
    main_loop(event_pump, canvas);
}

fn main_loop(mut event_pump: EventPump, mut canvas: Canvas<Window>) {
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
