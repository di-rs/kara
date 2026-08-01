use crossterm::cursor::SetCursorStyle::BlinkingBlock;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::event::{Event::Key, KeyCode::Char, read};
use crossterm::style::Print;

use crate::queue;
use document::Document;
use terminal::{Position, Terminal};

mod document;
mod terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    document: Document,
}

impl Editor {
    pub fn run(&mut self) {
        let _ = Terminal::initialize();
        let result = self.repl();
        let _ = Terminal::terminate();
        let _ = result;
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        queue!(Hide)?;

        if self.should_quit {
            Terminal::clear_screen()?;
            queue!(MoveTo(0, 0), Print("Goodbye.\r\n"))?;
        } else {
            Self::draw_grid()?;
            Self::draw_welcome()?;

            let location = self.document.caret_location();
            Terminal::move_cursor_to(Position {
                x: location.x,
                y: location.y,
            })?;
            queue!(BlinkingBlock)?;
        }

        queue!(Show)?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_grid() -> Result<(), std::io::Error> {
        use crossterm::terminal::{Clear, ClearType};
        let size = Terminal::size()?;
        for i in 0..size.height {
            queue!(MoveTo(0, i), Clear(ClearType::CurrentLine), Print("~"))?;
        }
        Ok(())
    }

    fn draw_welcome() -> Result<(), std::io::Error> {
        let size = Terminal::size()?;

        let mut welcome_message = format!("{NAME} v{VERSION}");
        let message_len = u16::try_from(welcome_message.len()).unwrap_or_default();

        let start_y = size.height / 3;
        let start_x = size.width.saturating_sub(message_len).saturating_sub(1) / 2;

        welcome_message.truncate(size.width.into());

        queue!(MoveTo(start_x, start_y), Print(welcome_message))?;

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match *code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Down
                | Char('j' | 'l' | 'i' | 'k')
                    if *modifiers == KeyModifiers::NONE =>
                {
                    self.move_caret(*code);
                }
                _ => (),
            }
        }
    }

    fn move_caret(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up | Char('i') => {
                self.document.move_caret(document::Direction::Up(1));
            }
            KeyCode::Down | Char('k') => {
                self.document.move_caret(document::Direction::Down(1));
            }
            KeyCode::Left | Char('j') => {
                self.document.move_caret(document::Direction::Left(1));
            }
            KeyCode::Right | Char('l') => {
                self.document.move_caret(document::Direction::Right(1));
            }
            _ => (),
        }
    }
}
