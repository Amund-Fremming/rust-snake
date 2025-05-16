use console::Term;
use std::io::{Write, stdin, stdout};

use crate::structures::Key;

pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    if let Err(e) = stdout().flush() {
        eprintln!("Failed to flush stout: {}", e);
    }

    let mut line = String::new();
    match stdin().read_line(&mut line) {
        Ok(_) => line = line.trim_end().to_string(),
        Err(_) => line = String::from("Failed to get input"),
    }

    line
}

pub fn read_arrow() -> Option<Key> {
    let key = Term::stdout().read_key();
    if let Err(_) = key {
        return None;
    }

    return match key.unwrap() {
        console::Key::ArrowUp => Some(Key::Up),
        console::Key::ArrowDown => Some(Key::Down),
        console::Key::ArrowLeft => Some(Key::Left),
        console::Key::ArrowRight => Some(Key::Right),
        console::Key::Escape => Some(Key::Escape),
        _ => None,
    };
}
