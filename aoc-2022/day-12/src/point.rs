use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Point {
    pub fn next(&self, d: &Direction, map: &Map) -> Option<Self> {
        match d {
            Direction::Up => {
                if self.row == 0 {
                    None
                } else {
                    Some(Point {
                        row: self.row - 1,
                        col: self.col,
                    })
                }
            }
            Direction::Down => Some(Point {
                row: self.row + 1,
                col: self.col,
            }),
            Direction::Left => {
                if self.col == 0 {
                    None
                } else {
                    Some(Point {
                        row: self.row,
                        col: self.col - 1,
                    })
                }
            }
            Direction::Right => Some(Point {
                row: self.row,
                col: self.col + 1,
            }),
        }
    }
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}
