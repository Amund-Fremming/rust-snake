use super::{Direction, point::Point};

pub struct Snake {
    head: Point,
    body: Vec<Point>,
    moving_direction: (i32, i32),
}

impl Snake {
    pub fn new() -> Snake {
        let head = Point { x: (4, 5), y: 5 };
        let second = Point { x: (4, 5), y: 4 };
        let third = Point { x: (4, 5), y: 3 };

        Snake {
            head: head,
            body: vec![second, third],
            moving_direction: (0, 1),
        }
    }

    pub fn update_moving_direction(&mut self, direction: Direction) {
        self.moving_direction = match direction {
            Direction::Up => (0i32, 1i32),
            Direction::Down => (0i32, -1i32),
            Direction::Left => (-1i32, 0i32),
            Direction::Right => (1i32, 0i32),
            _ => self.moving_direction,
        };
    }

    pub fn eats_self(&self) -> bool {
        let eats_self = self.body.iter().any(|b| b == &self.head);
        if eats_self {
            false;
        }

        true
    }

    pub fn next(&mut self) {
        self.head = self.get_next_pos();
        self.body.pop();
    }

    pub fn eat(&mut self) {
        self.head = self.get_next_pos();
    }

    fn get_next_pos(&self) -> Point {
        Point {
            x: (
                self.head.x.0 - self.moving_direction.0,
                self.head.x.1 - self.moving_direction.0,
            ),
            y: self.head.y - self.moving_direction.1,
        }
    }
}
