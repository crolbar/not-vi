use super::Editor;

pub struct Cursor {
    x: usize,
    y: usize,

    un_trunc_x: Option<usize>,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            un_trunc_x: None,
        }
    }
    pub fn get_y(&self) -> usize { self.y  }
    pub fn get_x(&self) -> usize { self.x  }


}

impl Editor {
    pub fn cursor_move_x_to(&mut self, line_len: usize) {
        self.cursor.x = line_len;
    }

    pub fn cursor_move_right(&mut self) {
        if self.buf[self.cursor.y].len().saturating_sub(!self.is_insert() as usize) > self.cursor.x {
            self.cursor.x += 1;
        }

        self.cursor.un_trunc_x = Some(self.cursor.x);
    }

    pub fn cursor_move_left(&mut self) {
        self.cursor.x = self.cursor.x.saturating_sub(1);

        self.cursor.un_trunc_x = Some(self.cursor.x);
    }


    fn handle_virt_move_x(&mut self) {
        if let Some(l) = self.buf.get(self.cursor.y as usize) {
            if self.cursor.x >= l.len() {
                self.cursor.x = l.len().saturating_sub(1)
            } else {
                if let Some(x) = self.cursor.un_trunc_x {
                    if x < l.len() {
                        self.cursor.x = x;
                    }
                }
            }
        }
    }

    pub fn cursor_move_down(&mut self) {
        if self.buf.len().saturating_sub(2) > self.cursor.y {
            self.cursor.y += 1;
        }

        self.handle_virt_move_x();
    }

    pub fn cursor_move_up(&mut self, ignore_un_trunc_x: bool) {
        self.cursor.y = self.cursor.y.saturating_sub(1);

        if !ignore_un_trunc_x {
            self.handle_virt_move_x();
        }
    }


    pub fn cursor_move_top(&mut self) { self.cursor.y = 0; }
    pub fn cursor_move_bottom(&mut self) { self.cursor.y = self.buf.len().saturating_sub(2); }

    pub fn cursor_move_up_half_win(&mut self) {
        let half_sub_y = self.cursor.y.saturating_sub((self.window.height / 2) as usize);

        if half_sub_y <= 0 {
            self.cursor.y = 0;
        } else {
            self.cursor.y = half_sub_y;
        }
    }

    pub fn cursor_move_down_half_win(&mut self) {
        let half_plus_y = self.cursor.y + (self.window.height / 2) as usize;
        let lines = self.buf.len() - 2;

        if half_plus_y > lines {
            self.cursor.y = lines;
        } else {
            self.cursor.y = half_plus_y;
        }
    }

    pub fn cursor_move_to_char(&mut self, char: char) {
        if let Some(line) = self.buf.get(self.cursor.y) {
            self.cursor.x = line
                .chars().enumerate()
                .skip(self.cursor.x + 1)
                .find(|(_, c)| *c == char)
                .map(|i| i.0)
                .unwrap_or(self.cursor.x);
        }
    }

    pub fn cursor_move_to_next_word_start(&mut self, shift: bool) {
        let x = self.cursor.x;
        let y = self.cursor.y;

        if let Some(line) = &self.buf.get(y) {
            let curr_char_is_alpha = line.chars().nth(x).unwrap_or(' ').is_alphabetic();
            let mut has_whitespace = false;

            self.cursor.x = line.chars().enumerate()
                .skip(x + 1)
                .find(|(_, c)| {
                if *c == ' ' {
                    has_whitespace = true;
                }

                if has_whitespace {
                    *c != ' '
                } else 
                    if !shift {
                        c.is_alphabetic() != curr_char_is_alpha
                    } else {
                        false 
                    }
            }).map(|(i, _)| i).unwrap_or_else(|| {
                self.cursor_move_down();
                self.buf[self.cursor.y].chars().position(|c| c != ' ').unwrap_or(0)
            });
        }
    }

    pub fn cursor_move_to_curr_word_end(&mut self, shift: bool) {
        let x = self.cursor.x;
        let y = self.cursor.y;

        if let Some(line) = &self.buf.get(y) {
            let line_len = line.len();

            let mut curr_char_is_alpha = line.chars().nth(x).unwrap_or(' ').is_alphabetic();

            if let Some(next_char) = line.chars().nth(x + 1) {
                // if we are at the end of the word move to next word
                if next_char.is_alphabetic() != curr_char_is_alpha || next_char == ' ' {
                    curr_char_is_alpha = line.chars().skip(x + 1).find(|c| *c != ' ').unwrap_or(' ').is_alphabetic()
                }
            }

            self.cursor.x = line.chars().enumerate()
                .skip(x + 1).skip_while(|c| c.1 == ' ')
                .find(|(_, c)| 
            {
                if *c == ' ' {
                    true
                } else 
                    if !shift {
                        c.is_alphabetic() != curr_char_is_alpha
                    } else {
                        false 
                    }
            }).map(|(i, _)| i - 1).unwrap_or_else(|| {
                if x == line_len.saturating_sub(1) {
                    self.cursor_move_down();

                    let iter = &mut self.buf[self.cursor.y].chars().enumerate()
                        .skip_while(|(_, c)| *c == ' ');

                    if let Some((_, start_char)) = iter.clone().next() {
                        iter.find(|(_, c)|{
                            if shift {
                                *c == ' '
                            } else {
                                c.is_alphabetic() != start_char.is_alphabetic()
                            }
                        }).map(|i| i.0 - 1)
                        .unwrap_or(0)
                    } else { 0 }
                } else {
                    line_len.saturating_sub(1)
                }
            });
        }
    }
}
