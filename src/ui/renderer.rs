use crate::structures::{point::Point, *};
use std::io::{Write, stdout};

pub struct Renderer {
    pub moving_direction: (i8, i8),
}

impl Renderer {
    pub fn new() -> Renderer {
        println!("");
        println!("");
        println!("  Press esc to exit");
        println!("");
        println!(" - - - - - - - - - - ");
        println!("|                   |");
        println!("|                   |");
        println!("|                   |");
        println!("|                   |");
        println!("|     █             |");
        println!("|     █             |");
        println!("|     █             |");
        println!("|                   |");
        println!("|                   |");
        println!("|                   |");
        println!(" - - - - - - - - - - ");
        println!("");
        print!("Moving: {:?}", Key::Up);
        stdout().flush().unwrap();

        Renderer {
            moving_direction: (0, 1),
        }
    }

    pub fn update_move(&self, last_move: &Key) {
        print!("\r\x1b[2K");
        print!("\rMoving: {:?}", last_move);
        stdout().flush().unwrap();

        // TODO - oppdatere direction
    }
}
