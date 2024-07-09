use ratatui::layout::Rect;

use super::Editor;

pub struct Cursor {
    x: usize,
    y: usize,

    maxx: usize,
    minx: usize,
    maxy: usize,
    miny: usize,

    un_trunc_x: Option<usize>,
}

impl Cursor {
    pub fn new(win: &Rect) -> Self {
        Self {
            x: 0,
            y: 0,

            maxx: win.width as usize,
            minx: win.x as usize,
            maxy: win.height as usize,
            miny: win.y as usize,

            un_trunc_x: None,
        }
    }
    pub fn get_y(&self) -> usize { self.y }
    pub fn get_x(&self) -> usize { self.x }

    pub fn get_display_x(&self) -> usize { self.x + self.minx }
    pub fn get_display_y(&self) -> usize { self.y + self.miny }

    pub fn update_min_max(&mut self, win: Rect) {
        self.minx = win.x as usize;
        self.miny = win.y as usize;
        self.maxx = win.width as usize;
        self.maxy = win.height as usize;
    }
}

impl Editor {
    pub fn cursor_move_x_to(&mut self, n: usize) {
        self.cursor.x = n;
    }

    pub fn cursor_move_y_to(&mut self, n: usize) {
        self.cursor.y = n;
    }

    pub fn cursor_move_right(&mut self) {
        self.cursor.x = self.get_x_n_chars_right(1);
        self.cursor.un_trunc_x = Some(self.cursor.x);
    }

    pub fn cursor_move_left(&mut self) {
        self.cursor.x = self.get_x_n_chars_left(1);
        self.cursor.un_trunc_x = Some(self.cursor.x);
    }


    pub fn handle_virt_move_x(&mut self) {
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
        self.cursor.y = self.get_y_n_lines_down(1);
        self.handle_virt_move_x();
    }

    pub fn cursor_move_up(&mut self, ignore_un_trunc_x: bool) {
        self.cursor.y = self.get_y_n_lines_up(1);
        if !ignore_un_trunc_x {
            self.handle_virt_move_x();
        }
    }


    pub fn cursor_move_top(&mut self) { 
        self.cursor.y = 0; 
        self.handle_virt_move_x();
    }
    pub fn cursor_move_bottom(&mut self) {
        self.cursor.y = self.buf.len().saturating_sub(1);
        self.handle_virt_move_x();
    }

    pub fn cursor_move_up_half_win(&mut self) {
        let half_sub_y = self.cursor.y.saturating_sub((self.window.height / 2) as usize);

        if half_sub_y <= 0 {
            self.cursor.y = 0;
        } else {
            self.cursor.y = half_sub_y;
            self.scroll.0 = (half_sub_y as u16 + self.conf.scrolloff).saturating_sub(self.window.height);
        }
        self.handle_virt_move_x();
    }

    pub fn cursor_move_down_half_win(&mut self) {
        let half_down = (self.window.height / 2) as usize;
        self.cursor.y = self.get_y_n_lines_down(half_down);

        if self.cursor.y <= self.buf.len().saturating_sub(1) {
            self.scroll.0 = (self.cursor.y as u16).saturating_sub(self.conf.scrolloff);
        }
        self.handle_virt_move_x();
    }

    pub fn cursor_move_to_next_empty_line(&mut self) {
        self.cursor.y = self.get_y_next_empty_line(1);
        self.cursor.x = 0;
    }

    pub fn cursor_move_to_prev_empty_line(&mut self) {
        self.cursor.y = self.get_y_prev_empty_line(1);
        self.cursor.x = 0;
    }

    pub fn cursor_move_to_char(&mut self, char: char, rev: bool, till: bool) {
        if let Some(line) = self.buf.get(self.cursor.y) {
            self.cursor.x = 
                if rev {
                    line.chars()
                        .rev()
                        .skip(line.len() - self.cursor.x)
                        .enumerate()
                        .find(|(_, c)| *c == char)
                        .map(|i| self.cursor.x - (i.0 + 1))
                        .unwrap_or(self.cursor.x)
                        .saturating_add(till as usize)
                } else {
                    line.chars()
                        .enumerate()
                        .skip(self.cursor.x + 1)
                        .find(|(_, c)| *c == char)
                        .map(|i| i.0)
                        .unwrap_or(self.cursor.x)
                        .saturating_sub(till as usize)
                };
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
