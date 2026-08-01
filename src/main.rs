#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use crate::editor::Editor;

mod editor;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
