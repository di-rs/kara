use crossterm::cursor::SetCursorStyle::BlinkingBlock;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::event::{Event::Key, KeyCode::Char, read};
use crossterm::style::Print;

use crate::queue;
use buffer::{Buffer, Direction};
use terminal::{Position, Terminal};
use view::View;

mod buffer;
mod terminal;
mod view;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    buffer: Buffer,
    // view: View,
}

impl Editor {
    pub fn run(&mut self, file_name: Option<&String>) {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        let _ = Terminal::initialize();

        self.load_document(file_name);
        let _ = self.repl();
    }

    fn load_document(&mut self, file_name: Option<&String>) {
        if let Some(file_name) = file_name
            && let Ok(loaded_doc) = Buffer::open(file_name)
        {
            self.buffer = loaded_doc;
        }
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
            View::render(&self.buffer)?;

            let location = self.buffer.caret_location();
            Terminal::move_to(Position {
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
                self.buffer.move_caret(Direction::Up(1));
            }
            KeyCode::Down | Char('j') => {
                self.buffer.move_caret(Direction::Down(1));
            }
            KeyCode::Left | Char('h') => {
                self.buffer.move_caret(Direction::Left(1));
            }
            KeyCode::Right | Char('k') => {
                self.buffer.move_caret(Direction::Right(1));
            }
            _ => (),
        }
    }
}
