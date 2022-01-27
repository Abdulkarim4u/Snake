/*This is where we draw our elements /rectangle and on screen*/

use piston_window::{rectangle,Context,G2d}; //G2d is 2d graphics.
use piston_window::types::Color;

const BLOCK_SIZE:f64 = 25.0;

pub fn  to_coord(game_coord:i32)->f64{ //this will convert one coordinate into a blocksize
    (game_coord as f64)*BLOCK_SIZE

}

pub fn to_coord_u32(game_corrd:i32)->u32{ //we convert it to unsigned integer.
    to_coord_u32(game_corrd) as u32
}

//function to draw our blocks on the screen.
pub  fn draw_block(color:Color,x:i32,y:i32,con:&Context,g:&mut G2d){
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(color,[gui_x,gui_y,BLOCK_SIZE,BLOCK_SIZE],con.transform,g);//g is the graphics
}

//if we want to draw a block we need to draw a rectangle as well.

pub fn draw_rectangle(color:Color,x:i32,y:i32,width:i32,height:i32,con: &Context, g: &mut G2d){
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(color,[gui_x,gui_y,BLOCK_SIZE *(width as f64),BLOCK_SIZE *(height as f64)],con.transform,g);
}