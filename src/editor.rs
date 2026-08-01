use crossterm::cursor::SetCursorStyle::BlinkingBlock;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, KeyEvent, KeyModifiers};
use crossterm::event::{Event::Key, KeyCode::Char, read};
use crossterm::style::Print;

use crate::queue;
use terminal::Terminal;

mod terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

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
            queue!(MoveTo(0, 0), Print("Goodbye.\r\n"))?;
        } else {
            Self::draw_grid()?;
            Self::draw_welcome()?;
            queue!(MoveTo(1, 0), BlinkingBlock)?;
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
        let start_x = size.width.saturating_sub(message_len - 1) / 2;

        welcome_message.truncate(size.width.into());

        queue!(MoveTo(start_x, start_y), Print(welcome_message))?;

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
