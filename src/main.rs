mod core;
mod utils;

use sdl2::{render::Canvas, video::Window};
use sdl2::{EventPump, Sdl, VideoSubsystem};

const WIDTH: u16 = 1024;
const HEIGHT: u16 = 512;

fn main() { init(); }

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

    let mut cpu = utils::CPU::init();

    // todo: load rom
    core::main_loop(event_pump, canvas);
}
