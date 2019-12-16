
pub mod types;
pub mod vis;



pub struct GameOverErr;

use types::{Grid, Snake, Cell};

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
            Ok(snake.length())
        },
        Cell::Snake => Err(GameOverErr)
    }
}   