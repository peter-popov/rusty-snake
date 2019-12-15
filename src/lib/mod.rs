use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub mod types;
pub mod snake;

use types::{Cell, Grid};

// this function initializes the canvas
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

pub fn grid_init(nx_cells: u32, ny_cells: u32) -> Grid {
    let mut grid_vector = Vec::new();

    for row in 0..ny_cells {
        grid_vector.push(Vec::new());
        for _column in 0..nx_cells {
            grid_vector[row as usize].push(Cell {
                r: 35_u8,
                g: 15_u8,
                b: 13_u8,
            });
        }
    }
    let grid = Grid { grid: grid_vector };

    grid
}


pub fn display_cell(
    renderer: &mut Canvas<Window>,
    row: u32,
    col: u32,
    grid_data: &Grid,
    cell_width: &u32,
) {
    let cell_height = cell_width;

    let grid = &grid_data.grid;

    let x = cell_width * col;
    let y = cell_width * row;

    // For now, we want random colors, to see what happens.
    let cell_color = &grid[row as usize][col as usize];
    let drawing_color = Color::RGB(cell_color.r, cell_color.g, cell_color.b);
    
    renderer.set_draw_color(drawing_color);
    let square = renderer.fill_rect(Rect::new(x as i32, y as i32, *cell_width, *cell_height));
    match square {
        Ok(()) => {}
        Err(error) => println!("{}", error),
    }
}


pub fn display_frame(
    renderer: &mut Canvas<Window>,
    grid: &Grid,
    nx_cells: &u32,
    ny_cells: &u32,
    cell_width: &u32,
) {
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    for row in 0..*ny_cells {
        for column in 0..*nx_cells {
            display_cell(renderer, row, column, &grid, &cell_width)
        }
    }
    renderer.present();
}