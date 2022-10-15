use lib::{display_rectangle, grid_init, init, snake};
use sdl2::{event::Event, keyboard::Keycode};
use std::{thread, time::Duration};

pub mod lib;

fn main() {
    let (mut canvas, mut events) = init(720, 720);

    let columns = 100;
    let rows = 100;
    let cell_width = 7;

    let mut grid = lib::grid_init(columns, rows);
    let mut direction = (1, 0);
    let mut snake = snake::snake_init();

    'game: loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    direction.0 = -1;
                    direction.1 = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    direction.0 = 1;
                    direction.1 = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    direction.0 = 0;
                    direction.1 = -1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    direction.0 = 0;
                    direction.1 = 1;
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                _ => continue 'game,
            }
        }

        snake = snake::snake_moves(&mut snake, direction);
        grid = snake::draw_grid_with_snake(grid, &snake);
        lib::display_frame(&mut canvas, &grid, &columns, &rows, &cell_width);

        thread::sleep(Duration::from_millis(800));
    }
}
