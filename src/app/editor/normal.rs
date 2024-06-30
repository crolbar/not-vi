use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::tui::Tui;
use anyhow::Result;
use super::Editor;

impl Editor {
    pub fn normal_update(&mut self, _tui: &mut Tui, key: KeyEvent) -> Result<()> {
        let maxx = self.frame_rect.width;
        let maxy = self.frame_rect.height;
        let buf = self.get_buf().clone();

        match key.code {
            KeyCode::Char('h') => { self.cursor.move_left(); },
            KeyCode::Char('l') => { self.cursor.move_right(maxx, buf, false); },
            KeyCode::Char('k') => { self.cursor.move_up(buf, false); },
            KeyCode::Char('j') => { self.cursor.move_down(maxy, buf); },

            KeyCode::Char('i') => { self.enter_insert()? },
            KeyCode::Char('a') => { 
                self.cursor.move_right(maxx, buf, true);
                self.enter_insert()? 
            },

            KeyCode::Char('w') | KeyCode::Char('W') => {
                self.cursor.move_to_next_word_start(maxy, buf, key.modifiers == KeyModifiers::SHIFT)
            },

            _ => ()
        }
        
        Ok(())
    }
}
