use std::collections::VecDeque;
use num::Integer;

#[derive(Clone)]
pub enum Cell {
    Background, 
    Food,
    Snake
}

#[derive(Clone)]
pub struct Position(pub i32, pub i32);

pub struct Direction(pub i32, pub i32);

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    pub nx: usize,
    pub ny: usize
}

// Rust % operator bahaves a bit different to python on negative numbers
fn modulo<I: Integer + Copy>(a: &I, b: &I) -> I {
    ((*a % *b) + *b) % *b
}

impl Grid {
    pub fn new(nx: usize, ny: usize) -> Grid {
        Grid{
            cells: vec![vec![Cell::Background; ny]; nx], 
            nx: nx, 
            ny: ny
        }
    }

    fn normilize(&self, position: &Position) -> (usize, usize) {
        let x = modulo(&position.0, &(self.nx as i32));
        let y = modulo(&position.1, &(self.ny as i32));
        (x as usize, y as usize)
    }

    pub fn set(&mut self, position: &Position, cell: Cell) {
        let (x, y) = self.normilize(position);
        self.cells[x][y] = cell;
    }

    pub fn get(&self, position: &Position) -> &Cell {
        let (x, y) = self.normilize(position);
        &self.cells[x][y]
    }
}

pub struct Snake {
    direction: Direction,
    body: VecDeque<Position>
}

impl Snake {
    pub fn new(p: Position, direction: Direction) -> Snake {
        let mut snake = Snake {
            direction: direction,
            body: VecDeque::new()
        };
        snake.body.push_back(p);
        snake
    }

    pub fn turn(&mut self, direction: Direction) {
        if self.direction.0 != direction.0 && self.direction.1 != direction.1 {
            self.direction = direction;
        }
    }

    pub fn step(&mut self) -> (Position, Position) {
        match self.body.front() {
            Some(front_position) => {
                let next = Position(front_position.0 + self.direction.0, 
                                    front_position.1 + self.direction.1);
                self.body.push_front(next.clone());
                let last = self.body.pop_back();
                println!("Next head: {}, {}", next.0, next.1);
                (next, last.unwrap())
            },
            None => panic!("Must never happen")
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Position> {
        self.body.iter()
    }

    pub fn grow(&mut self, position: Position) {
        self.body.push_back(position);
    }

    pub fn length(&self) -> usize {
        self.body.len()
    }
}
