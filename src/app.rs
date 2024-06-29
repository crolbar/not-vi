pub mod editor;

use {
    anyhow::Result,
    editor::Editor
};

pub struct App {
    pub exit: bool,
    pub editor: Editor,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            exit: false,
            editor: Editor::new()?,
        })
    }
}

