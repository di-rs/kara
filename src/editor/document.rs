use std::cmp::max;

#[derive(Default, Clone, Copy)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

#[derive(Default)]
pub struct Document {
    caret_location: Location,
}

impl Document {
    pub fn move_caret(&mut self, direction: Direction) {
        match direction {
            Direction::Up(step) => {
                self.caret_location.y = max(0, self.caret_location.y.saturating_sub(step));
            }
            Direction::Down(step) => {
                self.caret_location.y = self.caret_location.y.saturating_add(step);
            }
            Direction::Left(step) => {
                self.caret_location.x = max(0, self.caret_location.x.saturating_sub(step));
            }
            Direction::Right(step) => {
                self.caret_location.x = self.caret_location.x.saturating_add(step);
            }
        }
    }

    pub const fn caret_location(&self) -> Location {
        self.caret_location
    }
}
