use super::{Direction, point::Point};

pub struct Snake {
    pub head: Point,
    pub body: Vec<Point>,
    moving_direction: (i32, i32),
}

impl Snake {
    pub fn new() -> Snake {
        let head = Point { x: 4, y: 5 };
        let second = Point { x: 4, y: 4 };
        let third = Point { x: 4, y: 3 };

        Snake {
            head: head,
            body: vec![second, third],
            moving_direction: (0, 1),
        }
    }

    pub fn update_moving_direction(&mut self, direction: &Direction) {
        self.moving_direction = match direction {
            Direction::Up => (0i32, 1i32),
            Direction::Down => (0i32, -1i32),
            Direction::Left => (-1i32, 0i32),
            Direction::Right => (1i32, 0i32),
            _ => self.moving_direction,
        };
    }

    pub fn eaten_self(&self) -> bool {
        let eats_self = self.body.iter().any(|b| b == &self.head);
        if eats_self {
            return true;
        }

        false
    }

    pub fn next(&mut self, food: &Point) -> bool {
        let mut result = true;
        let next_head = self.get_next_pos();

        if next_head != *food {
            self.body.pop();
            result = false;
        }

        self.body.insert(0, self.head.clone());
        self.head = next_head;

        result
    }

    fn get_next_pos(&self) -> Point {
        Point {
            x: self.head.x + self.moving_direction.0,
            y: self.head.y + self.moving_direction.1,
        }
    }
}
