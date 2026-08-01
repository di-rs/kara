use crossterm::cursor::SetCursorStyle::BlinkingBlock;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, KeyEvent, KeyModifiers};
use crossterm::event::{Event::Key, KeyCode::Char, read};
use crossterm::style::Print;

use crate::queue;
use terminal::Terminal;

mod terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
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
            queue!(Print("Goodbye.\r\n"))?;
        } else {
            Self::draw_rows()?;
            queue!(MoveTo(1, 0), BlinkingBlock)?;
        }

        queue!(Show)?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let size = Terminal::size()?;

        for i in 0..size.height {
            queue!(MoveTo(0, i), Print("~"))?;
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
            && *code == Char('q')
            && *modifiers == KeyModifiers::CONTROL
        {
            self.should_quit = true;
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}
