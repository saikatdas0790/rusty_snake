use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, EventPump};
use types::Grid;

pub mod types;

pub fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
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

pub fn display_rectangle(renderer: &mut Canvas<Window>, canvas_width: &u32, canvas_height: &u32) {
    let red: u8 = rand::random();
    let green: u8 = rand::random();
    let blue: u8 = rand::random();

    renderer.clear();

    let drawing_color = Color::RGB(red, green, blue);
    renderer.set_draw_color(drawing_color);

    let square_definition = Rect::new(0, 0, *canvas_width, *canvas_height);
    let square = renderer.fill_rect(square_definition);

    match square {
        Ok(()) => {}
        Err(error) => println!("{}", error),
    }

    renderer.present();
}

pub fn grid_init(nx_cells: u32, ny_cells: u32) -> Grid {
    let mut grid_vector = Vec::new();

    for row in 0..ny_cells {
        grid_vector.push(Vec::new());
        for _column in 0..nx_cells {
            grid_vector[row as usize].push(types::Cell {
                red: 35,
                green: 15,
                blue: 13,
            });
        }
    }
    let grid = Grid { grid: grid_vector };

    grid
}
