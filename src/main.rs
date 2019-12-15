// Dependencies go here
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::thread;
use std::time;


pub mod lib;

// this is main
fn main() {
    let w: u32 = 1600;
    let h: u32 = 1200;
    let rows: u32 = 60;
    let columns: u32 = 80;
    let (mut canvas, mut events) = lib::init(w, h);

    let mut grid = lib::grid_init(columns, rows);
    let cell_width: u32 = w / columns;
    let mut direction = (1, 0);
    let mut snake = lib::snake::init_snake(columns, rows);


    thread::spawn(move || {
        // some work here
    });

    'game: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    direction = (0, -1);
                    continue 'game;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    direction = (0, 1);
                    continue 'game;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    direction = (-1, 0);
                    continue 'game;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    direction = (1, 0);
                    continue 'game;
                },
                _ => continue 'game,
            }
        }

        lib::snake::move_snake(&mut snake, &direction);
        
        grid = lib::snake::snake_to_grid(grid, &snake, columns, rows);

        lib::display_frame(&mut canvas, &grid, &columns, &rows, &cell_width);

        thread::sleep(time::Duration::from_millis(800));
    }   
}
