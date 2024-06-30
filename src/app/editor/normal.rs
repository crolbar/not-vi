use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::tui::Tui;
use anyhow::Result;
use super::Editor;

impl Editor {
    pub fn normal_update(&mut self, _tui: &mut Tui, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('h') => { self.cursor_move_left(); },
            KeyCode::Char('l') => { self.cursor_move_right(); },
            KeyCode::Char('k') => { self.cursor_move_up(false); },
            KeyCode::Char('j') => { self.cursor_move_down(); },

            KeyCode::Char('i') => { self.enter_insert()? },
            KeyCode::Char('a') => { 
                self.enter_insert()? 
            },

            KeyCode::Char('w') | KeyCode::Char('W') => {
                self.cursor_move_to_next_word_start(key.modifiers == KeyModifiers::SHIFT)
            },
            KeyCode::Char('e') | KeyCode::Char('E') => {
                self.cursor_move_to_curr_word_end(key.modifiers == KeyModifiers::SHIFT)
            },

            _ => ()
        }
        
        Ok(())
    }
}
