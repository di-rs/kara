use crossterm::{cursor::MoveTo, style::Print};

use crate::{
    editor::terminal::{Terminal, TerminalSize},
    queue,
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {}

impl View {
    pub fn render() -> Result<(), std::io::Error> {
        use crossterm::terminal::{Clear, ClearType};

        let TerminalSize { height, .. } = Terminal::size()?;

        for i in 0..height {
            queue!(MoveTo(0, i), Clear(ClearType::CurrentLine), Print("~"))?;

            if i == 0 {
                queue!(Print(" Hello, World!"))?;
            }
        }

        // Always drawing it for now
        Self::draw_welcome_message()?;

        Ok(())
    }

    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let size = Terminal::size()?;

        let mut welcome_message = format!("{NAME} v{VERSION}");
        let message_len = u16::try_from(welcome_message.len()).unwrap_or_default();

        let start_y = size.height / 3;
        let start_x = size.width.saturating_sub(message_len).saturating_sub(1) / 2;

        welcome_message.truncate(size.width.into());

        queue!(MoveTo(start_x, start_y), Print(welcome_message))?;

        Ok(())
    }
}
