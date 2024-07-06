use anyhow::Result;
use crossterm::event::{KeyEvent, KeyCode};
use crate::tui::Tui;
use super::Editor;

impl Editor {
    pub fn insert_update(&mut self, tui: &mut Tui, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.cursor_move_left();
                tui.term.set_cursor(self.cursor.get_x() as u16, self.cursor.get_y() as u16)?;
                self.enter_normal()?;
                self.replaced_chars.clear();
            }

            KeyCode::Up => { self.cursor_move_up(false) },
            KeyCode::Down => { self.cursor_move_down() },
            KeyCode::Right => { self.cursor_move_right() },
            KeyCode::Left => { self.cursor_move_left() },

            KeyCode::Char(char) => {
                if self.is_insert() {
                    self.insert_char(char) 
                } else {
                    self.replace_char(char)
                }
            },

            KeyCode::Backspace | KeyCode::Delete => { 
                if self.is_insert() {
                    self.del_char(key.code == KeyCode::Delete)
                } else {
                    self.replace_to_origin_char()
                }
            },

            KeyCode::Enter => { self.insert_nl(false) },
            KeyCode::Tab => {
                if self.is_replace() {
                    let rc = self.remove_char_at_cursor().unwrap();
                    self.replaced_chars.push(rc);
                    self.replaced_chars.push('\t');
                }

                self.insert_tab()
            }

            _ => ()
        }

        self.set_scroll();
        Ok(())
    }


    fn replace_to_origin_char(&mut self) {
        if self.cursor.get_x() == 0 && !self.replaced_chars.is_empty() {
            self.del_char(false);

        } else {
            self.cursor_move_left();
            if let Some(char) = self.replaced_chars.pop() {
                if char == '\t' {
                    self.cursor_move_right();
                    self.del_char(false);
                    let c = self.replaced_chars.pop().unwrap();

                    if c != '\0' {
                        self.insert_char(c);
                        self.cursor_move_left();
                    }

                } else

                if char != '\0' {
                    if let Some(_) = self.replace_char_at_cursor(char){};
                } else {
                    self.cursor_move_right();
                    self.del_char(false);
                }
            }
        }
    }

    fn replace_char(&mut self, char: char) {
        if self.get_curr_line_len() == self.cursor.get_x() {
            self.insert_char(char);

            self.replaced_chars.push('\0');
        } else {
            if let Some(rep_char) = self.replace_char_at_cursor(char){
                self.replaced_chars.push(rep_char);
            };
        }
        self.cursor_move_right();
    }

    pub fn insert_tab(&mut self) {
        if let Some(line) = self.buf.get_mut(self.cursor.get_y()) {
            line.insert_str(self.cursor.get_x(), "    ");
            self.cursor_move_x_to(self.cursor.get_x() + 4);
        }
    }

    pub fn insert_nl(&mut self, above: bool) {
        let x = self.cursor.get_x();
        let y = self.cursor.get_y();

        let indent: String = std::iter::repeat(' ').take(
            self.buf.iter()
                .rev()
                .skip(self.buf.len().saturating_sub(1).saturating_sub(y))
                .find(|l| !l.is_empty())
                .map(|l| l.chars().take_while(|c| *c == ' ').count())
                .unwrap_or(0)
        ).collect();

        let line = self.buf.get_mut(y).unwrap();

        if x != 0 {
            let mut rem = line.split_off(x);

            rem.insert_str(0, &indent);
            
            self.buf.insert(y + !above as usize, rem);
        } else {
            self.buf.insert(y, String::new());
        }

        if !above { self.cursor_move_down() }
        self.cursor_move_x_to(indent.len());
        self.set_scroll();
    }

    pub fn insert_char(&mut self, c: char) {
        if let Some(line) = self.buf.get_mut(self.cursor.get_y()) {
            line.insert(self.cursor.get_x(), c);

            self.cursor_move_right();
        }
    }

    pub fn del_char(&mut self, del_prev: bool) {
        let x = self.cursor.get_x();
        let y = self.cursor.get_y();

        let mut removed_char = '\0';
        if let Some(line) = self.buf.get_mut(y) {
            if x >= line.len() && line.len() > 0 && !del_prev {
                removed_char = line.pop().unwrap();
            }  else {
                if 
                    (x == 0 && y != 0 && !del_prev) ||
                    (del_prev && x == line.len())
                {
                    let remove_line_index = y + del_prev as usize;

                    if remove_line_index < self.buf.len() {
                        let removed_line = &self.buf.remove(remove_line_index);

                        let mod_line = self.buf
                            .get_mut(y - !del_prev as usize)
                            .unwrap();

                        let above_len = mod_line.len() + 1;

                        mod_line.push_str(removed_line);

                        if !del_prev {
                            self.cursor_move_x_to(above_len);
                            self.cursor_move_up(true);
                        }
                    }
                    
                } else if (x != 0 && line.len() > 0 && !del_prev) || del_prev {
                    removed_char = line.remove(x - !del_prev as usize);
                }
            }


            if !del_prev {
                if ' ' == removed_char {
                    let c = self.buf[y].chars().skip(x.saturating_sub(3 + 1)).take_while(|c| *c == ' ').count();

                    if c >= 3 {
                        for i in 0..3 {
                            self.buf[y].remove(x - i - 2);
                            self.cursor_move_left();
                        }
                    }
                }
            }
        }


        if !del_prev{
            self.cursor_move_left();
        }
    }
}
