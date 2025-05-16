use crate::{
    structures::Direction,
    ui::{cli::read_arrow, renderer::Renderer},
};
use std::{sync::mpsc, thread, time::Duration};

fn game_loop() {
    let (sender, reciever) = mpsc::channel();
    let sender_clone = sender.clone();

    let thread = thread::spawn(move || {
        loop {
            if let Some(value) = read_arrow() {
                sender_clone.send(value).unwrap();
            }
        }
    });

    let renderer = Renderer::new();
    loop {
        match reciever.recv_timeout(Duration::from_secs(1)) {
            Ok(key) if key == Direction::Escape => break,
            Ok(key) => renderer.update_move(&key),
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(error) => {
                eprintln!("Error: {}", error);
                break;
            }
        }
    }

    drop(thread);
}

pub fn run() {
    game_loop();
}
