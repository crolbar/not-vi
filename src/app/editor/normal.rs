use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::tui::Tui;
use anyhow::Result;
use super::Editor;

impl Editor {
    pub fn normal_update(&mut self, tui: &mut Tui, key: KeyEvent) -> Result<()> {
        if let Some(buf_char) = self.buffered_char {
            if let KeyCode::Char(char) = key.code {
                match buf_char {
                    'f' | 'F' | 't' | 'T' => { self.cursor_move_to_char(char, buf_char.is_uppercase(), buf_char == 't' || buf_char == 'T') },

                    'g' => { if char == 'g' { self.cursor_move_top(); } },

                    'r' => { if let Some(_) = self.replace_char_at_cursor(char){}; },

                    '>' => { 
                        self.indend_line(true);
                    },
                    '<' => { 
                        self.indend_line(false);
                    },

                    'd' => { 
                        match char {
                            'l' => { if let Some(_) = self.remove_char_at_cursor(){}; },
                            'h' => { 
                                self.cursor_move_left();
                                self.remove_char_at_cursor();
                            },
                            'd' => { self.remove_line_at_cursor(); },
                            'j' => {
                                if self.cursor.get_y() < self.buf.len().saturating_sub(1) {
                                    self.remove_line_at_cursor();
                                    self.remove_line_at_cursor();
                                    self.handle_virt_move_x();
                                    if self.cursor.get_y() > self.buf.len().saturating_sub(1) {
                                        self.cursor_move_up(false);
                                    }
                                }
                            },
                            'k' => {
                                if self.cursor.get_y() != 0 {
                                    self.remove_line_at_cursor();
                                    self.cursor_move_up(false);
                                    self.remove_line_at_cursor();
                                    self.handle_virt_move_x();
                                    if self.cursor.get_y() > self.buf.len().saturating_sub(1) {
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

                KeyCode::Char('f') |
                KeyCode::Char('F') |
                KeyCode::Char('t') |
                KeyCode::Char('T') => 
                    { 
                        if let KeyCode::Char(char) = key.code {
                            self.buffer_char(char)? 
                        }
                    },

                KeyCode::Char('d') => { 
                    if key.modifiers == KeyModifiers::CONTROL {
                        self.cursor_move_down_half_win();
                    } else {
                        self.buffer_char('d')? 
                    }
                },

                KeyCode::Char('u') => { 
                    if key.modifiers == KeyModifiers::CONTROL {
                        self.cursor_move_up_half_win();
                    }
                },


                KeyCode::Char('O') => { 
                    self.cursor_move_x_to(self.get_curr_line_len());
                    self.insert_nl(true);
                    self.enter_insert()?;
                },

                KeyCode::Char('o') => { 
                    self.cursor_move_x_to(self.get_curr_line_len());
                    self.insert_nl(false);
                    self.enter_insert()?;
                },

                KeyCode::Char('r') => { self.buffer_char('r')? },
                KeyCode::Char('R') => { self.enter_replace()? },

                KeyCode::Char('x') => { if let Some(_) = self.remove_char_at_cursor(){}; },

                KeyCode::Char('g') => { self.buffer_char('g')? },
                KeyCode::Char('G') => { self.cursor_move_bottom() },

                KeyCode::Char('i') => { self.enter_insert()? },
                KeyCode::Char('I') => {
                    self.enter_insert()?;
                    self.cursor_move_x_to(0);
                },
                KeyCode::Char('a') => { 
                    self.cursor_move_x_to(self.cursor.get_x() + 1);
                    self.enter_insert()? 
                },
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

                KeyCode::Char('>') => { self.buffer_char('>')? },
                KeyCode::Char('<') => { self.buffer_char('<')? },

                KeyCode::Char('}') => { self.cursor_move_to_next_empty_line() },
                KeyCode::Char('{') => { self.cursor_move_to_prev_empty_line() },

                _ => ()
            }
        }

        self.set_scroll();
        Ok(())
    }

    fn indend_line(&mut self, right: bool) {
        if let Some(line) = self.buf.get_mut(self.cursor.get_y()) {
            let curr_indent_len = line.chars().take_while(|c| *c == ' ').count();

            let single_indent_len = self.conf.shiftwidth;

            let needed_spaces_till_next_stop = single_indent_len - (curr_indent_len % single_indent_len);

            if right {
                line.insert_str(0, &" ".repeat (needed_spaces_till_next_stop));
            } else if curr_indent_len > 0 {
                line.drain(0..needed_spaces_till_next_stop);
            }
        }
    }

    /// returns the replaced char
    pub fn replace_char_at_cursor(&mut self, char: char) -> Option<char> {
        if let Some(line) = self.buf.get_mut(self.cursor.get_y()) {
            let x = self.cursor.get_x();
            let c = 
                line.remove(x);
            line.insert(x, char);

            return Some(c)
        }
        None
    }

    fn remove_line_at_cursor(&mut self) {
        let y = self.cursor.get_y();
        if y >= self.buf.len() {
            self.buf.pop();
        } else {
            self.buf.remove(y);
        }
    }

    pub fn remove_char_at_cursor(&mut self) -> Option<char> {
        let line = &mut self.buf[self.cursor.get_y()];
        let x = self.cursor.get_x();
        if line.len().saturating_sub(1) <= x {
            let c = 
                line.pop();
            self.cursor_move_left();

            return c
        } else {
            Some(line.remove(self.cursor.get_x()))
        }
    }
}
