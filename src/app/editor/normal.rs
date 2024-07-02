use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::tui::Tui;
use anyhow::Result;
use super::Editor;

impl Editor {
    pub fn normal_update(&mut self, tui: &mut Tui, key: KeyEvent) -> Result<()> {
        if let Some(buf_char) = self.buffered_char {
            if let KeyCode::Char(char) = key.code {
                match buf_char {
                    'f' => { self.cursor_move_to_char(char) },

                    'g' => { if char == 'g' { self.cursor_move_top(); } },

                    'd' => { 
                        match char {
                            'l' => { self.remove_char_at_cursor() },
                            'h' => { 
                                self.cursor_move_left();
                                self.remove_char_at_cursor();
                            },
                            'd' => { self.remove_line_at_cursor(); },
                            'j' => {
                                if self.cursor.get_y() < self.buf.len().saturating_sub(2) {
                                    self.remove_line_at_cursor();
                                    self.remove_line_at_cursor();
                                    self.cursor_move_down();
                                    if self.cursor.get_y() <= self.buf.len().saturating_sub(1) {
                                        self.cursor_move_up(false);
                                    }
                                }
                            },
                            'k' => {
                                if self.cursor.get_y() != 0 {
                                    self.remove_line_at_cursor();
                                    self.cursor_move_up(false);
                                    self.remove_line_at_cursor();
                                    self.cursor_move_down();
                                    if self.cursor.get_y() <= self.buf.len().saturating_sub(1) {
                                        self.cursor_move_up(false);
                                    }
                                }
                            },

                            _ => ()
                        }
                    },

                    _ => ()
                }
            }
            self.remove_bufferd_char()?;
            tui.term.set_cursor(self.cursor.get_x() as u16, self.cursor.get_y() as u16)?;
        } else {
            match key.code {
                KeyCode::Char('h') => { self.cursor_move_left(); },
                KeyCode::Char('l') => { self.cursor_move_right(); },
                KeyCode::Char('k') => { self.cursor_move_up(false); },
                KeyCode::Char('j') => { self.cursor_move_down(); },

                KeyCode::Char('f') => { self.buffer_char('f')? },
                KeyCode::Char('d') => { self.buffer_char('d')? },

                KeyCode::Char('x') => { self.remove_char_at_cursor() },

                KeyCode::Char('g') => { self.buffer_char('g')? },
                KeyCode::Char('G') => { self.cursor_move_bottom() },

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

    fn remove_line_at_cursor(&mut self) {
        let y = self.cursor.get_y();
        if y >= self.buf.len() {
            self.buf.pop();
        } else {
            self.buf.remove(y);
        }
    }

    fn remove_char_at_cursor(&mut self) {
        let line = &mut self.buf[self.cursor.get_y()];
        let x = self.cursor.get_x();
        if line.len().saturating_sub(1) <= x {
            line.pop();
            self.cursor_move_left();
        } else {
            line.remove(self.cursor.get_x());
        }
    }
}
