
pub struct Cell {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub struct Grid {
    pub grid: Vec<Vec<Cell>>,
}

pub struct SnakeHead {
    pub x: i32,
    pub y: i32
}