use crate::structures::Direction;
use console::Term;

pub fn read_arrow() -> Option<Direction> {
    let key = Term::stdout().read_key();
    if let Err(_) = key {
        return None;
    }

    return match key.unwrap() {
        console::Key::ArrowUp => Some(Direction::Up),
        console::Key::ArrowDown => Some(Direction::Down),
        console::Key::ArrowLeft => Some(Direction::Left),
        console::Key::ArrowRight => Some(Direction::Right),
        console::Key::Escape => Some(Direction::Escape),
        _ => None,
    };
}
