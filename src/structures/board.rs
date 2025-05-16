use super::Point;
use rand::random_range;

pub struct Board {
    width: i32,
    height: i32,
    food: Point,
}

impl Board {
    pub fn new(size: &i32) -> Board {
        let random_food = Point { x: (10, 10), y: 20 };

        Board {
            width: *size,
            height: *size,
            food: random_food,
        }
    }

    pub fn is_next_food(&self, head: &Point) -> bool {
        if self.food != *head {
            false;
        }

        true
    }

    pub fn outside_perimiter(&self, head: &Point) -> bool {
        let inside = |value: i32| -> bool { value >= self.width && value < self.width };
        if !(inside(head.x.0) && inside(head.x.1) && inside(head.y)) {
            false;
        }

        true
    }

    fn random_food(&mut self) {
        let y = random_range(0..=self.height);
        let x1 = random_range(0..=self.width);
        let x2 = match x1 {
            a if a % 2 != 0 => x1 + 1,
            b if b % 2 == 0 => x1 - 1,
            _ => -1,
        };

        self.food = Point { x: (x1, x2), y: y }
    }
}
