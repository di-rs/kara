use std::fmt::Display;

use crate::editor::{
    buffer::Buffer,
    terminal::{Position, Size, Terminal},
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    needs_redraw: bool,
    size: Size,
}

impl View {
    pub const fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    pub fn render(&mut self, document: &Buffer) -> Result<(), std::io::Error> {
        if !self.needs_redraw {
            return Ok(());
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }

        for i in 0..height {
            match document.get_line(i) {
                Some(line) => {
                    let truncated_line = if line.len() >= width {
                        &line.chars().take(width).collect()
                    } else {
                        line
                    };
                    Self::render_line(i, truncated_line)?;
                }
                None => Self::render_line(i, "~")?,
            }
        }

        if document.is_empty() {
            Self::draw_welcome_message()?;
        }

        self.needs_redraw = false;
        Ok(())
    }

    fn render_line(at: usize, line_text: impl Display) -> Result<(), std::io::Error> {
        Terminal::move_to(Position { x: 0, y: at })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let size = Terminal::size()?;

        let mut welcome_message = format!("{NAME} v{VERSION}");
        let message_len = welcome_message.len();

        let y = size.height / 3;
        let x = size.width.saturating_sub(message_len).saturating_sub(1) / 2;

        welcome_message.truncate(size.width);

        Terminal::move_to(Position { x, y })?;
        Terminal::print(welcome_message)?;

        Ok(())
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
