use crate::{
    structures::{Direction, board::Board, snake::Snake},
    ui::{cli::read_arrow, renderer::Renderer},
};
use std::{
    sync::mpsc::{self, Receiver},
    thread::{self},
    time::Duration,
};

pub struct Game {
    board: Board,
    snake: Snake,
    event_reciever: Receiver<Direction>,
}

impl Game {
    pub fn new() -> Game {
        let (sender, reciever) = mpsc::channel();
        let sender_clone = sender.clone();

        thread::spawn(move || {
            loop {
                if let Some(value) = read_arrow() {
                    sender_clone.send(value).unwrap();
                }
            }
        });

        Game {
            board: Board::new(&10),
            snake: Snake::new(),
            event_reciever: reciever,
        }
    }

    pub fn run(&mut self) {
        let renderer = Renderer::new(10);
        loop {
            renderer.render_game(&self.board, &self.snake);

            match self.event_reciever.recv_timeout(Duration::from_millis(500)) {
                Ok(dir) if dir == Direction::Escape => break,
                Ok(dir) => {
                    renderer.update_move(&dir);
                    if let Err(error) = self.handle_move(&dir) {
                        println!("{}", error);
                        break;
                    };
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    if let Err(error) = self.update_game() {
                        println!("{}", error);
                        break;
                    }
                }
                Err(error) => {
                    eprintln!("Error: {}", error);
                    break;
                }
            }
        }
    }

    fn handle_move(&mut self, direction: &Direction) -> Result<(), String> {
        self.snake.update_moving_direction(&direction);
        self.update_game()
    }

    fn update_game(&mut self) -> Result<(), String> {
        if let true = self.snake.next(&self.board.food) {
            self.board.update_food();
        }

        if self.board.outside_perimiter(&self.snake.head) || self.snake.eaten_self() {
            return Err(String::from("Game over!"));
        }

        return Ok(());
    }
}
