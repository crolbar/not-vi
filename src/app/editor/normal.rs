use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::tui::Tui;
use anyhow::Result;
use super::Editor;

impl Editor {
    pub fn normal_update(&mut self, tui: &mut Tui, key: KeyEvent) -> Result<()> {
        //if let Some(buf_char) = self.buffered_char {
        //    if let KeyCode::Char(char) = key.code {
        //        match buf_char {
        //            //'f' | 'F' | 't' | 'T' => { self.cursor_move_to_char(char, buf_char.is_uppercase(), buf_char == 't' || buf_char == 'T') },
        //
        //            'g' => { if char == 'g' { self.cursor_move_top(); } },
        //
        //            'r' => { if let Some(_) = self.replace_char_at_cursor(char){}; },
        //
        //            '>' | '<' => { self.shift_indent(buf_char == '>', char) }
        //
        //            'd' => { self.op_delete(char) },
        //
        //            _ => ()
        //        }
        //    }
        //    self.remove_bufferd_char()?;
        //    tui.term.set_cursor(self.cursor.get_x() as u16, self.cursor.get_y() as u16)?;
        //} else {
        //    match key.code {
        //        KeyCode::Char('h') => { self.cursor_move_left(1); },
        //        KeyCode::Char('l') => { self.cursor_move_right(1); },
        //        KeyCode::Char('k') => { self.cursor_move_up(); },
        //        KeyCode::Char('j') => { self.cursor_move_down(); },
        //
        //        KeyCode::Char('f') |
        //        KeyCode::Char('F') |
        //        KeyCode::Char('t') |
        //        KeyCode::Char('T') => 
        //            { 
        //                if let KeyCode::Char(char) = key.code {
        //                    self.buffer_char(char)? 
        //                }
        //            },
        //
        //        KeyCode::Char('d') => { 
        //            if key.modifiers == KeyModifiers::CONTROL {
        //                self.cursor_move_down_half_win();
        //            } else {
        //                self.buffer_char('d')? 
        //            }
        //        },
        //
        //        KeyCode::Char('u') => { 
        //            if key.modifiers == KeyModifiers::CONTROL {
        //                self.cursor_move_up_half_win();
        //            }
        //        },
        //
        //
        //        KeyCode::Char('O') => { 
        //            self.cursor_move_x_to(self.get_curr_line_len());
        //            self.insert_nl(true);
        //            self.enter_insert()?;
        //        },
        //
        //        KeyCode::Char('o') => { 
        //            self.cursor_move_x_to(self.get_curr_line_len());
        //            self.insert_nl(false);
        //            self.enter_insert()?;
        //        },
        //
        //        KeyCode::Char('r') => { self.buffer_char('r')? },
        //        KeyCode::Char('R') => { self.enter_replace()? },
        //
        //        KeyCode::Char('x') => { if let Some(_) = self.remove_char_at_cursor(){}; },
        //
        //        KeyCode::Char('g') => { self.buffer_char('g')? },
        //        KeyCode::Char('G') => { self.cursor_move_bottom() },
        //
        //        KeyCode::Char('i') => { self.enter_insert()? },
        //        KeyCode::Char('I') => {
        //            self.enter_insert()?;
        //            self.cursor_move_x_to(0);
        //        },
        //        KeyCode::Char('a') => { 
        //            self.cursor_move_x_to(self.cursor.get_x() + 1);
        //            self.enter_insert()? 
        //        },
        //        KeyCode::Char('A') => {
        //            self.enter_insert()?;
        //            self.cursor_move_x_to(self.buf[self.cursor.get_y()].len());
        //        },
        //
        //        KeyCode::Char('w') | KeyCode::Char('W') => {
        //            self.cursor_move_to_next_word_start(key.modifiers == KeyModifiers::SHIFT)
        //        },
        //        KeyCode::Char('e') | KeyCode::Char('E') => {
        //            self.cursor_move_to_curr_word_end(key.modifiers == KeyModifiers::SHIFT)
        //        },
        //
        //        KeyCode::Char('>') => { self.buffer_char('>')? },
        //        KeyCode::Char('<') => { self.buffer_char('<')? },
        //
        //        KeyCode::Char('}') => { self.cursor_move_to_next_empty_line() },
        //        KeyCode::Char('{') => { self.cursor_move_to_prev_empty_line() },
        //
        //        _ => ()
        //    }
        //}
        //
        //self.set_scroll();
        Ok(())
    }

    fn get_motion_end(&self, char: char) -> (usize, usize) {
        (
            match char {
                'h' => self.get_x_n_chars_left(1),
                'l' => self.get_x_n_chars_right(1),
                _ => self.cursor.get_x()
            } ,

            match char {
                'j' => self.get_y_n_lines_down(1),
                'k' => self.get_y_n_lines_up(1),

                '{' => self.get_y_prev_empty_line(1),
                '}' => self.get_y_next_empty_line(1),
                _ => self.cursor.get_y(),
            }
        )
    }

    pub fn shift_indent(&mut self, right: bool, char: char) {
        if char == 't' || char == 'd' {
            match right {
                true => self.cursor_move_x_to(self.cursor.get_x() - self.conf.shiftwidth),
                false => self.cursor_move_x_to(self.cursor.get_x() + self.conf.shiftwidth),
            }
        }

        let (_, end_y) = self.get_motion_end(char);

        let y =  self.cursor.get_y();

        let (start, end) = 
            if end_y >= y {
                (y, end_y)
            } else {
                (end_y, y.saturating_sub((self.cursor.get_x() == 0) as usize))
            };

        (start..=end).for_each(|y| {
            if let Some(line) = self.buf.get_mut(y) {
                if !line.is_empty() {
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
        });

        if end_y < self.cursor.get_y() {
            self.cursor_move_y_to(end_y);
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


    fn op_delete(&mut self, char: char) {
        let y = self.cursor.get_y();
        let x = self.cursor.get_x();

        let (mx, my) = self.get_motion_end(char);

        match char {
            'k' => self.cursor_move_up(),
            'h' => self.cursor_move_left(),
            _ => ()
        }

        if my != y {
            let (start_y, end_y) = 
                (
                    std::cmp::min(y, my),
                    std::cmp::max(y, my)
                );

            self.buf.drain(start_y..=end_y);

            self.handle_vert_move_x();
        } else 

        if char == 'd' {
            self.buf.remove(y);
        }


        if mx != x {
            if let Some(line) = self.buf.get_mut(y) {
                let (start_x, end_x) = 
                    (
                        std::cmp::min(x, mx),
                        std::cmp::max(x, mx)
                    );

                line.drain(start_x..end_x);
            }
        }
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
