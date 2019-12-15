use crate::lib::types::{Cell, SnakeHead, Grid};

pub fn init_snake(columns: u32, rows: u32) -> SnakeHead {
    SnakeHead{x: columns as i32/2, y: rows as i32/2}
}

pub fn move_snake(snake: &mut SnakeHead, direction: &(i32, i32)) {
    let (dx, dy) = direction;
    snake.x += dx;
    snake.y += dy;
}

pub fn snake_to_grid(mut grid: Grid, snake: &SnakeHead, columns: u32, rows: u32) -> Grid {
    grid.grid[(snake.y % rows as i32) as usize][(snake.x % columns as i32) as usize] = Cell{r:144, g:12, b:12};
    grid
}