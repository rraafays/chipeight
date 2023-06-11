use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window};
use sdl2::{EventPump, Sdl, VideoSubsystem};

fn main() {
    init();
}

fn init() {
    let sdl_context: Sdl = match sdl2::init() {
        Ok(sdl_context) => sdl_context,
        Err(_) => panic!("failed to initialise sdl context"),
    };

    let video_subsystem: VideoSubsystem = match sdl_context.video() {
        Ok(video_subsystem) => video_subsystem,
        Err(_) => panic!("failed to initialise video subsystem"),
    };

    let window: Window = video_subsystem
        .window("test", 600, 800)
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

    main_loop(event_pump, canvas);
}

fn main_loop(mut event_pump: EventPump, mut canvas: Canvas<Window>) {
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => panic!("bruh moment"),
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => panic!("bruh moment"),
                _ => {}
            }
        }
        canvas.clear();
        canvas.present();
    }
}
