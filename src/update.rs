use crossterm::event::{KeyCode, KeyEvent};
use crate::{app::{editor::{binds::Cmd, EditorMode}, App}, tui::Tui};
use anyhow::Result;

impl App {
    pub fn update(&mut self, tui: &mut Tui, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => { self.exit = true; },
            KeyCode::Esc => { self.editor.enter_normal()? },

            _ => {
                if self.editor.curr_cmd.should_get_nkey() {
                    self.editor.set_nkey(key);
                } else {
                    self.editor.curr_cmd.push_key(key);
                }

                self.editor.do_if_contains()?;
                //match *self.editor.get_mode() {
                //    EditorMode::Normal => self.editor.normal_update(tui, key)?,
                //    EditorMode::Insert | EditorMode::Replace => self.editor.insert_update(tui, key)?,
                //}
            }
        }

        self.editor.set_scroll();
        Ok(())
    }
}
