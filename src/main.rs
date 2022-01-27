
extern crate piston_window;  //importing the piston window dependences
extern crate rand;



mod draw; // importing the draw module
mod game; // importing the game module

use piston_window::types::Color; //importing piston window dependencies for color.
use piston_window::*;
use crate::draw::to_coord_u32;
use crate::game::Game;

const BACK_COLOR: Color = [0.5,0.5,0.5, 1.0]; //Red ,green and blue , alpha colors.



fn main() {
//creating the window
    let (width,height) = (30,30); //windows width and height
    let mut window:PistonWindow = WindowSettings::new("Snake",[to_coord_u32(width),to_coord_u32(height)])
        .exit_on_esc(true)
        .build()
        .unwrap();
    //using the window to create the game.
    let mut game = Game::new(width,height);
    while let Some(event) = window.next() {
        window.draw_2d(&event,|c,g,_|{
            clear(BACK_COLOR,g);
            game.draw(&c,g);
        });

        event.update(|arg|{
            game.update(arg.dt) //delta time it takes to update.
        });
        
    }
}
