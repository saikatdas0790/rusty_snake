use lib::{display_rectangle, grid_init, init};
use sdl2::{event::Event, keyboard::Keycode};
use std::{thread, time::Duration};

pub mod lib;

fn main() {
    let (mut canvas, mut events) = init(720, 720);

    let columns = 100;
    let rows = 100;
    let cell_width = 7;

    let mut grid = lib::grid_init(columns, rows);

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

        lib::display_frame(&mut canvas, &grid, &columns, &rows, &cell_width);

        thread::sleep(Duration::from_millis(800));
    }
}
