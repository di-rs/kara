use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::event::{Event::Key, KeyCode::Char, read};

use crate::editor::terminal::Size;
use buffer::{Buffer, Direction};
use terminal::{Position, Terminal};
use view::View;

mod buffer;
mod terminal;
mod view;

mod prelude;
pub use prelude::*;

pub struct Editor {
    should_quit: bool,
    view: View,
    buffer: Buffer,
}

impl Editor {
    pub fn builder() -> EditorBuilder {
        EditorBuilder::default()
    }

    fn new(file_name: Option<String>) -> Result<Self, std::io::Error> {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::initialize()?;
        let buffer = file_name
            .and_then(|file_name| Buffer::open(&file_name).ok())
            .unwrap_or_default();

        Ok(Self {
            should_quit: false,
            view: View::new(),
            buffer,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => self.evaluate_event(&event),
                Err(err) => {
                    debug_assert!(false, "Could not read event: {err:?}");
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();

        let caret_location = self.buffer.caret_location();

        self.view.scroll_into_view(caret_location);
        self.view.render(&self.buffer);

        let location = caret_location.subtract(self.view.scroll_offset);
        let _ = Terminal::move_to(location.into());

        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    fn evaluate_event(&mut self, event: &Event) {
        match event {
            Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => match *code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | Char('j' | 'h' | 'u' | 'k')
                    if *modifiers == KeyModifiers::NONE =>
                {
                    self.move_caret(*code);
                }
                _ => (),
            },
            Event::Resize(width, height) => {
                self.view.resize(Size {
                    height: (*height).into(),
                    width: (*width).into(),
                });
            }
            _ => (),
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

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::move_to(Position { col: 0, row: 0 });
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}

#[derive(Default)]
pub struct EditorBuilder {
    file_name: Option<String>,
}

impl EditorBuilder {
    pub fn file(mut self, file_name: Option<&String>) -> Self {
        self.file_name = file_name.cloned();
        self
    }

    pub fn build(self) -> Result<Editor, std::io::Error> {
        Editor::new(self.file_name)
    }
}
