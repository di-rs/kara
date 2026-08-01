use crossterm::cursor::MoveTo;
use crossterm::terminal;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use std::io::{Write, stdout};

#[macro_export]
macro_rules! queue {
    ($($command:expr),*) => {{
        use ::std::io::Write;
        use ::std::io::stdout;

        // This allows the macro to take both mut impl Write and &mut impl Write.
        Ok(stdout().by_ref())
            $(.and_then(|writer| crossterm::QueueableCommand::queue(writer, $command)))*
            .map(|_| ())
    }}
}

#[derive(Clone, Copy)]
pub struct TerminalSize {
    pub height: u16,
    pub width: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal {}

type Result<T> = std::result::Result<T, std::io::Error>;

impl Terminal {
    pub fn initialize() -> Result<()> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<()> {
        Self::execute()?;
        disable_raw_mode()
    }

    pub fn clear_screen() -> Result<()> {
        queue!(Clear(ClearType::All))
    }

    pub fn move_cursor_to(position: Position) -> Result<()> {
        queue!(MoveTo(position.x, position.y))
    }

    pub fn size() -> Result<TerminalSize> {
        let size = terminal::size()?;
        Ok(TerminalSize {
            height: size.1,
            width: size.0,
        })
    }

    pub fn execute() -> Result<()> {
        stdout().flush()?;
        Ok(())
    }
}
