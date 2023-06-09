use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};

fn main() {
    init();
    deinit();
}

fn init() {
    let sdl: Sdl = match sdl2::init() {
        Ok(it) => it,
        Err(err) => panic!(),
    };
    let video: VideoSubsystem = match sdl.video() {
        Ok(it) => it,
        Err(err) => panic!(),
    };

    let window: Window = video
        .window("test", 100, 100)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.present();
}

fn deinit() {}
