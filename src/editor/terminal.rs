use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use std::fmt::Display;
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
    pub height: usize,
    pub width: usize,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Terminal {}

type Result<T> = std::result::Result<T, std::io::Error>;

impl Terminal {
    pub fn initialize() -> Result<()> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_to(Position { x: 0, y: 0 })?;
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

    pub fn clear_line() -> Result<()> {
        queue!(Clear(ClearType::CurrentLine))
    }

    pub fn move_to(position: Position) -> Result<()> {
        let x = u16::try_from(position.x).unwrap_or(u16::MAX);
        let y = u16::try_from(position.y).unwrap_or(u16::MAX);

        queue!(MoveTo(x, y))
    }

    pub fn print(data: impl Display) -> Result<()> {
        queue!(Print(data))
    }

    pub fn size() -> Result<TerminalSize> {
        let size = terminal::size()?;
        Ok(TerminalSize {
            height: size.1.into(),
            width: size.0.into(),
        })
    }

    pub fn execute() -> Result<()> {
        stdout().flush()?;
        Ok(())
    }
}
