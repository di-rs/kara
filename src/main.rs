use std::env;

use crate::editor::Editor;

mod editor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1);

    let mut editor = Editor::default();
    editor.run(file_name);
}
