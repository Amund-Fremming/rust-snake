use super::Point;
use rand::random_range;

pub struct Board {
    pub size: i32,
    pub food: Point,
}

impl Board {
    pub fn new(size: &i32) -> Board {
        let random_food = Point { x: 2, y: 1 };

        Board {
            size: *size,
            food: random_food,
        }
    }

    pub fn outside_perimiter(&self, head: &Point) -> bool {
        let outside = |value: i32| -> bool { value < 0 || value >= self.size };
        if outside(head.x) || outside(head.y) {
            return true;
        }

        false
    }

    pub fn update_food(&mut self) {
        self.food = Point {
            x: random_range(0..self.size),
            y: random_range(0..self.size),
        }
    }
}
