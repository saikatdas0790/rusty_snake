use std::{thread, time::Duration};

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump,
};

fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Game", width + 1, height + 1)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
}

fn display_rectangle(renderer: &mut Canvas<Window>, canvas_width: &u32, canvas_height: &u32) {
    let red: u8 = rand::random();
    let green: u8 = rand::random();
    let blue: u8 = rand::random();

    renderer.clear();

    let drawing_color = Color::RGB(red, green, blue);
    renderer.set_draw_color(drawing_color);

    let square_definition = Rect::new(0, 0, *canvas_width, *canvas_height);
    renderer.fill_rect(square_definition).ok();

    renderer.present();
}

fn main() {
    let (mut canvas, mut events) = init(720, 720);

    thread::spawn(move || {});

    'game: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                _ => continue 'game,
            }
        }

        display_rectangle(&mut canvas, &720, &720);

        thread::sleep(Duration::from_millis(800));
    }
}
