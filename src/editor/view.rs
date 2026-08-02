use crate::editor::terminal::{Position, Terminal, TerminalSize};
use buffer::Buffer;

mod buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn render(&self) -> Result<(), std::io::Error> {
        let TerminalSize { height, .. } = Terminal::size()?;

        for i in 0..height {
            Terminal::move_to(Position { x: 0, y: i })?;
            Terminal::clear_line()?;

            match self.buffer.line(i) {
                Some(line) => Terminal::print(line)?,
                None => Terminal::print("~")?,
            }
        }

        // Always drawing it for now
        Self::draw_welcome_message()?;

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
