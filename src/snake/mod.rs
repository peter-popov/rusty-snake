use rand;

pub mod types;
pub mod vis;

pub struct GameOverErr;

use types::{Grid, Snake, Cell, Position};

pub fn move_snake(grid: &mut Grid, snake: &mut Snake) -> Result<usize, GameOverErr> {
    let (next, last) = snake.step();
    match *grid.get(&next) {
        Cell::Background => {
            grid.set(&last, Cell::Background);
            grid.set(&next, Cell::Snake);
            Ok(snake.length())
        },
        Cell::Food => {
            snake.grow(last);
            grid.set(&next, Cell::Snake);
            add_food(grid, rand::random::<u32>() % 2 + 1);
            Ok(snake.length())
        },
        Cell::Snake => Err(GameOverErr)
    }
}   

pub fn add_food(grid: &mut Grid, n: u32) {  
    for _ in 0..n {
        let pos = Position(rand::random(), rand::random());
        match *grid.get(&pos) {
            Cell::Background => grid.set(&pos, Cell::Food),
            _ => (),
        }
    }
}