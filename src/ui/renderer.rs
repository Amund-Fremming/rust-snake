use crate::structures::{Direction, board::Board, snake::Snake};
use std::io::{Write, stdout};

pub struct Renderer {
    empty_x: String,
    snake_x: String,
    food_x: String,
    size: i32,
}

impl Renderer {
    pub fn new(size: i32) -> Renderer {
        println!("");
        println!("");
        println!("  Press esc to exit");
        println!("");
        println!(" - - - - - - - - - - ");
        for _i in 0..size {
            println!("|                    |");
        }
        println!(" - - - - - - - - - - ");
        println!("");
        print!("Moving: {:?}", Direction::Up);
        stdout().flush().unwrap();

        Renderer {
            empty_x: String::from("  "),
            snake_x: String::from("██"),
            food_x: String::from("00"),
            size: size,
        }
    }

    pub fn render_game(&self, board: &Board, snake: &Snake) {
        print!("\x1b[1A");
        print!("\x1b[1A");
        print!("\x1b[1A");

        for _i in 0..self.size - 1 {
            print!("\x1b[1A");
        }

        for i in 0..self.size {
            let line = self.get_line(board, snake, 9 - i);
            print!("\r\x1b[2K");
            print!("{}", line);
            print!("\x1b[1B");
        }

        print!("\x1b[1B");
        print!("\x1b[1B");
        print!("\x1b[1B");
        stdout().flush().unwrap();
    }

    fn get_line(&self, board: &Board, snake: &Snake, y_coord: i32) -> String {
        let mut value = String::from("|");
        for i in 0..board.size {
            if snake.head.y == y_coord && snake.head.x == i {
                value.push_str(&self.snake_x);
            } else if snake
                .body
                .iter()
                .any(|item| item.y == y_coord && item.x == i)
            {
                value.push_str(&self.snake_x);
            } else if board.food.y == y_coord && board.food.x == i {
                value.push_str(&self.food_x);
            } else {
                value.push_str(&self.empty_x);
            }
        }

        value.push_str("|");
        value
    }

    pub fn update_move(&self, last_move: &Direction) {
        print!("\r\x1b[2K");
        print!("\rMoving: {:?}", last_move);
        stdout().flush().unwrap();
    }
}
