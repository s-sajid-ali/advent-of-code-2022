pub mod headtail {

    use std::fmt;

    #[derive(Copy, Clone, Debug)]
    pub struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Copy, Clone, Debug)]
    pub enum Direction {
        Left,
        Right,
        Up,
        Down,
        UpLeft,
        UpRight,
        DownLeft,
        DownRight,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "At : ({}, {});", self.x, self.y)
        }
    }

    impl Point {
        pub fn new(x1: i32, y1: i32) -> Point {
            Point { x: x1, y: y1 }
        }

        pub fn update(&mut self, direction: Direction) {
            match direction {
                Direction::Left => {
                    self.x -= 1;
                }
                Direction::Right => {
                    self.x += 1;
                }
                Direction::Up => {
                    self.y += 1;
                }
                Direction::Down => {
                    self.y -= 1;
                }
                Direction::UpLeft => {
                    self.x -= 1;
                    self.y += 1;
                }
                Direction::UpRight => {
                    self.x += 1;
                    self.y += 1;
                }
                Direction::DownLeft => {
                    self.x -= 1;
                    self.y -= 1;
                }
                Direction::DownRight => {
                    self.x += 1;
                    self.y -= 1;
                }
            }
        }

        pub fn to_move_tail(&self, tail: &Point) -> Option<Direction> {
            let dist_x: i64 = (self.x - tail.x).into();
            let dist_y: i64 = (self.y - tail.y).into();

            /*
            println!(
                "current state {}; dist_x/y are {} and {}",
                &self, dist_x, dist_y
            );*/

            if dist_x == 0 && dist_y == 0 {
                return None;
            } else if dist_x == 0 && dist_y != 0 {
                if dist_y > 1 {
                    return Some(Direction::Up);
                } else if dist_y < -1 {
                    return Some(Direction::Down);
                }
            } else if dist_y == 0 && dist_x != 0 {
                if dist_x > 1 {
                    return Some(Direction::Right);
                } else if dist_x < -1 {
                    return Some(Direction::Left);
                }
            } else if dist_x != 0 && dist_y != 0 {
                // diagnoal neighbor case!
                if dist_x.abs() == 1 && dist_y.abs() == 1 {
                    return None;
                }

                if dist_x > 0 && dist_y > 0 {
                    return Some(Direction::UpRight);
                } else if dist_x < 0 && dist_y > 0 {
                    return Some(Direction::UpLeft);
                } else if dist_x > 0 && dist_y < 0 {
                    return Some(Direction::DownRight);
                } else if dist_x < 0 && dist_y < 0 {
                    return Some(Direction::DownLeft);
                }
            }

            None
        }

        pub fn get_coords(&self) -> (i32, i32) {
            (self.x, self.y)
        }
    }
}
