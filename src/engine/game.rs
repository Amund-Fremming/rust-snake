use crate::{
    structures::{Direction, board::Board, snake::Snake},
    ui::{cli::read_arrow, renderer::Renderer},
};
use std::{
    sync::mpsc::{self, Receiver, Sender},
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
            board: Board::new(&20),
            snake: Snake::new(),
            event_reciever: reciever,
        }
    }

    pub fn run(&mut self) {
        let renderer = Renderer::new();
        loop {
            match self.event_reciever.recv_timeout(Duration::from_secs(1)) {
                Ok(dir) if dir == Direction::Escape => break,
                Ok(dir) => {
                    renderer.update_move(&dir);
                    self.handle_move(&dir);
                }
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(error) => {
                    eprintln!("Error: {}", error);
                    break;
                }
            }
        }
    }

    fn handle_move(&self, direction: &Direction) {
        print!("");
        /*
           er neste mat
               spis / flytt

           hvis flytt
               er den utenfor?
                   ferdig
               ikke utenfor
                   neste runde
        */
    }
}
