use crossterm::cursor::SetCursorStyle::BlinkingBlock;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::event::{Event::Key, KeyCode::Char, read};
use crossterm::style::Print;

use crate::queue;
use document::Document;
use terminal::{Position, Terminal};
use view::View;

mod document;
mod terminal;
mod view;

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
            View::render()?;

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
                | Char('j' | 'h' | 'u' | 'k')
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
            KeyCode::Up | Char('u') => {
                self.document.move_caret(document::Direction::Up(1));
            }
            KeyCode::Down | Char('j') => {
                self.document.move_caret(document::Direction::Down(1));
            }
            KeyCode::Left | Char('h') => {
                self.document.move_caret(document::Direction::Left(1));
            }
            KeyCode::Right | Char('k') => {
                self.document.move_caret(document::Direction::Right(1));
            }
            _ => (),
        }
    }
}
