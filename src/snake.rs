use std::collections::LinkedList;
use piston_window::{Context, G2d, types::Color};

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];
#[derive(Copy, Clone, PartialEq)]

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

