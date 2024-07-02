use crossterm::event::{KeyCode, KeyEvent};
use crate::{app::{App, editor::EditorMode}, tui::Tui};
use anyhow::Result;

impl App {
    pub fn update(&mut self, tui: &mut Tui, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => { self.exit = true; },

            _ => {
                match *self.editor.get_mode() {
                    EditorMode::Normal => self.editor.normal_update(tui, key)?,
                    EditorMode::Insert => self.editor.insert_update(tui, key)?,
                }
            }
        }
        self.editor.set_scroll();

        Ok(())
    }
}
