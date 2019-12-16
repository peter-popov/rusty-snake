use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::snake::types::{Cell, Grid, Position};

// this function initializes the canvas
pub fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Snake game", width + 1, height + 1)
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

fn render_cell(renderer: &mut Canvas<Window>,
    position: &Position,
    size: &(u32, u32),
    cell: &Cell
) {
    let x = position.0 as u32 * size.0;
    let y = position.1 as u32 * size.1;

    let drawing_color = match cell {
        Cell::Background => Color::RGB(0x46, 0x18, 0x5f),
        Cell::Food => Color::RGB(0xe9, 0xea, 0x77),
        Cell::Snake => Color::RGB(0x29, 0x7c, 0xa0)
    };

    renderer.set_draw_color(drawing_color);
    let square = renderer.fill_rect(Rect::new(x as i32, y as i32, size.0, size.1));
    match square {
        Ok(()) => {}
        Err(error) => println!("{}", error),
    }
}

pub fn display_frame(
    renderer: &mut Canvas<Window>,
    grid: &Grid
) {
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    let (w, h) = renderer.window().size();
    let size = (w / grid.nx as u32, h / grid.ny as u32);

    for x in 0..grid.nx {
        for y in 0..grid.ny {
            let position = Position(x as i32, y as i32);
            render_cell(renderer, &position, &size, grid.get(&position));
        }
    }
    renderer.present();
}
