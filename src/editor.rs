use crossterm::event::{Event::Key, KeyCode::Char, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

impl Editor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?} \r", event);
                    if event.code == Char('q') {
                        break;
                    }
                }
                Err(e) => println!("Error: {e}"),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}
