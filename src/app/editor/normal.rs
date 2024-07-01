use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::tui::Tui;
use anyhow::Result;
use super::Editor;

impl Editor {
    pub fn normal_update(&mut self, _tui: &mut Tui, key: KeyEvent) -> Result<()> {
        if let Some(ch) = self.buffered_char {
            if let KeyCode::Char(char) = key.code {
                match ch {
                    'f' => { self.cursor_move_to_char(char) },

                    _ => ()
                }
            }
            self.buffered_char = None;
        } else {
            match key.code {
                KeyCode::Char('h') => { self.cursor_move_left(); },
                KeyCode::Char('l') => { self.cursor_move_right(); },
                KeyCode::Char('k') => { self.cursor_move_up(false); },
                KeyCode::Char('j') => { self.cursor_move_down(); },

                KeyCode::Char('f') => { self.buffer_char('f') },

                KeyCode::Char('i') => { self.enter_insert()? },
                KeyCode::Char('I') => {
                    self.enter_insert()?;
                    self.cursor_move_x_to(0);
                },
                KeyCode::Char('a') => { self.enter_insert()? },
                KeyCode::Char('A') => {
                    self.enter_insert()?;
                    self.cursor_move_x_to(self.buf[self.cursor.get_y()].len());
                },

                KeyCode::Char('w') | KeyCode::Char('W') => {
                    self.cursor_move_to_next_word_start(key.modifiers == KeyModifiers::SHIFT)
                },
                KeyCode::Char('e') | KeyCode::Char('E') => {
                    self.cursor_move_to_curr_word_end(key.modifiers == KeyModifiers::SHIFT)
                },

                _ => ()
            }
        }
        
        Ok(())
    }
}
