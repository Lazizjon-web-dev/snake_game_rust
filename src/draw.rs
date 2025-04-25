use piston_window::{rectangle, Context, G2d, types::Color};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn draw_block(color: Color, x:i32, y: i32, con: &Context, g: &mut G2d){
    draw_rectangle(color, x, y,1,1, con, g);
}