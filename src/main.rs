extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use draw::to_coord_u32;
use game::Game;
use piston_window::{types::Color, *};

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    println!("Hello, world!");
}
