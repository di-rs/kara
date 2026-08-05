use std::fmt::Display;

use crate::editor::{
    buffer::Buffer,
    terminal::{Size, Terminal},
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    needs_redraw: bool,
    size: Size,
}

impl View {
    pub fn new() -> Self {
        Self {
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }

    pub const fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    pub fn render(&mut self, buffer: &Buffer) {
        if !self.needs_redraw {
            return;
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        for i in 0..height {
            match buffer.get_line(i) {
                Some(line) => {
                    let truncated_line = if line.len() >= width {
                        &line.chars().take(width).collect()
                    } else {
                        line
                    };
                    Self::render_line(i, truncated_line);
                }
                None => Self::render_line(i, "~"),
            }
        }

        if buffer.is_empty() {
            self.draw_welcome_message();
        }

        self.needs_redraw = false;
    }

    fn render_line(at: usize, line_text: impl Display) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    fn draw_welcome_message(&self) {
        let Size { height, width } = self.size;

        let welcome_message = format!("{NAME} v{VERSION}");
        let message_len = welcome_message.len();

        let y = height / 3;
        let padding = width.saturating_sub(message_len).saturating_sub(1) / 2;

        let mut message = format!("~{}{}", " ".repeat(padding), welcome_message);
        message.truncate(width);

        Self::render_line(y, message);
    }
}
