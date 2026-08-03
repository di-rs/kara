use std::cmp::min;

mod line;
use line::Line;

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
    StartOfLine,
    EndOfLine,
}

#[derive(Default)]
pub struct Buffer {
    lines: Vec<Line>,
    caret_location: Location,
}

impl Buffer {
    pub fn open(file_name: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(file_name)?;
        let mut lines = Vec::new();
        for line in content.lines() {
            lines.push(line.into());
        }
        Ok(Self {
            lines,
            caret_location: Location::default(),
        })
    }

    pub fn move_caret(&mut self, direction: Direction) {
        let height = self.lines.len();

        match direction {
            Direction::Up(step) => {
                self.caret_location.y = self.caret_location.y.saturating_sub(step);
                let width = self.width_at(self.caret_location.y);
                self.caret_location.x = min(width, self.caret_location.x);
            }
            Direction::Down(step) => {
                let new_y = self.caret_location.y.saturating_add(step);
                self.caret_location.y = min(new_y, height);
                let width = self.width_at(self.caret_location.y);
                self.caret_location.x = min(width, self.caret_location.x);
            }
            Direction::Left(step) => {
                if self.caret_location.x > 0 {
                    self.caret_location.x = self.caret_location.x.saturating_sub(step);
                } else if self.caret_location.y > 0 {
                    // recursive move up
                    self.move_caret(Direction::Up(1));
                    self.move_caret(Direction::EndOfLine);
                }
            }
            Direction::Right(step) => {
                let width = self.width_at(self.caret_location.y);
                if self.caret_location.x < width {
                    self.caret_location.x = self.caret_location.x.saturating_add(step);
                } else if self.caret_location.y < height {
                    // recursive move down
                    self.move_caret(Direction::Down(1));
                    self.move_caret(Direction::StartOfLine);
                }
            }
            Direction::StartOfLine => {
                self.caret_location.x = 0;
            }
            Direction::EndOfLine => {
                let width = self.width_at(self.caret_location.y);
                self.caret_location.x = width;
            }
        }
    }

    fn width_at(&self, at: usize) -> usize {
        self.lines.get(at).map_or(0, Line::len)
    }

    pub const fn caret_location(&self) -> Location {
        self.caret_location
    }

    pub const fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn get_line(&self, idx: usize) -> Option<&String> {
        self.lines.get(idx).map(|l| &l.inner)
    }
}
