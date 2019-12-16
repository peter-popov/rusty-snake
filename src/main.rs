// Dependencies go here
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::thread;
use std::time;


pub mod snake;

use snake::types::{Grid, Snake, Position, Direction};

// this is main
fn main() {
    let (mut canvas, mut events) = snake::vis::init(1200, 1200);

    let mut grid = Grid::new(60, 60);
    let mut snake = Snake::new(Position(40, 30), Direction(1,0));

    snake::add_food(&mut grid, 3);

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
                    snake.turn(Direction(0, -1));
                    continue 'game;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    snake.turn(Direction(0, 1));
                    continue 'game;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    snake.turn(Direction(-1, 0));
                    continue 'game;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    snake.turn(Direction(1, 0));
                    continue 'game;
                },
                _ => continue 'game,
            }
        }


        let result = snake::move_snake(&mut grid, &mut snake);
        match result {
            Ok(length) => println!("Snake length is {}", length),
            Err(_) => {
                println!("Snake over!");
                break 'game;
            }
        }
        
        snake::vis::display_frame(&mut canvas, &grid);

        thread::sleep(time::Duration::from_millis(100));
    }   
}
