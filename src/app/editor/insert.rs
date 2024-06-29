use anyhow::Result;
use crossterm::event::{KeyEvent, KeyCode};
use crate::tui::Tui;
use super::Editor;

impl Editor {
    pub fn insert_update(&mut self, tui: &mut Tui, key: KeyEvent) -> Result<()> {
        let maxx = self.frame_rect.width;
        let maxy = self.frame_rect.height;
        let buf = self.get_buf().clone();

        match key.code {
            KeyCode::Esc => {
                self.cursor.move_left();
                tui.term.set_cursor(self.cursor.get_x() as u16, self.cursor.get_y() as u16)?;
                self.enter_normal()?;
            }

            KeyCode::Up => { self.cursor.move_up(buf, false) },
            KeyCode::Down => { self.cursor.move_down(maxy, buf) },
            KeyCode::Right => { self.cursor.move_right(maxx, buf, true) },
            KeyCode::Left => { self.cursor.move_left() },

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
            self.cursor.move_x_to(self.cursor.get_x() + 4);
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

        self.cursor.move_down(self.frame_rect.width, self.get_buf().clone());
        self.cursor.move_x_to(0);
    }

    pub fn insert_char(&mut self, c: char) {
        if let Some(line) = self.buf.get_mut(self.cursor.get_y()) {
            line.insert(self.cursor.get_x(), c);

            self.cursor.move_right(
                self.frame_rect.width,
                self.get_buf().clone(),
                true
            );
        }
    }

    pub fn del_char(&mut self, del_prev: bool) {
        let x = self.cursor.get_x();
        let y = self.cursor.get_y();
        let buf: &mut Vec<String> = self.buf.as_mut();

        let mut removed_char = '\0';
        if let Some(line) = buf.get_mut(y) {
            if x >= line.len() && line.len() > 0 && !del_prev {
                removed_char = line.pop().unwrap();
            }  else {
                if 
                    (x == 0 && y != 0 && !del_prev) ||
                    (del_prev && x == line.len())
                {
                    let removed_line = &buf.remove(y + del_prev as usize);

                    let mod_line = buf
                        .get_mut(y - !del_prev as usize)
                        .unwrap();

                    let above_len = mod_line.len() + 1;

                    mod_line.push_str(removed_line);

                    if !del_prev {
                        self.cursor.move_x_to(above_len);
                        self.cursor.move_up(buf.clone(), true);
                    }
                    
                } else if (x != 0 && line.len() > 0 && !del_prev) || del_prev {
                    removed_char = line.remove(x - !del_prev as usize);
                }
            }


            if ' ' == removed_char {
                let c = buf[y].chars().skip(x.saturating_sub(3 + 1)).take_while(|c| *c == ' ').count();

                if c == 3 {
                    for i in 0..3 {
                        buf[y].remove(x - i - 2);
                        self.cursor.move_left();
                    }
                }
            }
        }


        if !del_prev{
            self.cursor.move_left();
        }
    }
}
