use crate::structures::Direction;
use std::io::{Write, stdout};

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        println!("");
        println!("");
        println!("  Press esc to exit");
        println!("");
        println!(" - - - - - - - - - - ");
        println!("|                    |");
        println!("|                    |");
        println!("|                    |");
        println!("|                    |");
        println!("|    ██              |");
        println!("|    ██              |");
        println!("|    ██              |");
        println!("|                    |");
        println!("|                    |");
        println!("|                    |");
        println!(" - - - - - - - - - - ");
        println!("");
        print!("Moving: {:?}", Direction::Up);
        stdout().flush().unwrap();

        Renderer {}
    }

    pub fn update_move(&self, last_move: &Direction) {
        print!("\r\x1b[2K");
        print!("\rMoving: {:?}", last_move);
        stdout().flush().unwrap();

        // TODO - oppdatere direction
    }
}
