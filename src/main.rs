use lib::{display_rectangle, init};
use sdl2::{event::Event, keyboard::Keycode};
use std::{thread, time::Duration};

pub mod lib;

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
