use crate::editor::{
    buffer::Buffer,
    terminal::{Position, Terminal, TerminalSize},
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {}

impl View {
    pub fn render(document: &Buffer) -> Result<(), std::io::Error> {
        let TerminalSize { height, .. } = Terminal::size()?;

        for i in 0..height {
            Terminal::move_to(Position { x: 0, y: i })?;
            Terminal::clear_line()?;

            match document.get_line(i) {
                Some(line) => Terminal::print(line)?,
                None => Terminal::print("~")?,
            }
        }

        if document.is_empty() {
            Self::draw_welcome_message()?;
        }

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
