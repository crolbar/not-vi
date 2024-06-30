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
            }

            KeyCode::Up => { self.cursor_move_up(false) },
            KeyCode::Down => { self.cursor_move_down() },
            KeyCode::Right => { self.cursor_move_right() },
            KeyCode::Left => { self.cursor_move_left() },

            KeyCode::Char(char) => { self.insert_char(char) },
            KeyCode::Backspace | KeyCode::Delete => { self.del_char(key.code == KeyCode::Delete) },
            KeyCode::Enter => { self.insert_nl() }
            KeyCode::Tab => { self.insert_tab() }

            _ => ()
        }

        Ok(())
    }


    pub fn insert_tab(&mut self) {
        if let Some(line) = self.buf.get_mut(self.cursor.get_y()) {
            line.insert_str(self.cursor.get_x(), "    ");
            self.cursor_move_x_to(self.cursor.get_x() + 4);
        }
    }

    pub fn insert_nl(&mut self) {
        let x = self.cursor.get_x();
        let y = self.cursor.get_y();
        let line = self.buf.get_mut(y).unwrap();

        if x != 0 {
            let rem = line.split_off(x);

            self.buf.insert(y + 1, rem);
        } else {
            self.buf.insert(y, String::new());
        }

        self.cursor_move_down();
        self.cursor_move_x_to(0);
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

                    if c == 3 {
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
