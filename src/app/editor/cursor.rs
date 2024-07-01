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
        if 
            (self.frame_rect.width - 1) as usize > self.cursor.x &&
            self.buf[self.cursor.y].len().saturating_sub(!self.is_insert() as usize) > self.cursor.x
        {
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
        if 
            (self.frame_rect.width - 1) as usize > self.cursor.y &&
            self.buf.len().saturating_sub(2) > self.cursor.y
        {
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

    fn get_curr_next_word(&mut self, shift: bool) -> (bool, Vec<(usize, String)>) {
        let mut line = self.buf.get(self.cursor.y).unwrap().clone();
        let origin_line_len = line.len();
        if let Some(next_line) = self.buf.get(self.cursor.y + 1) {
            line.push(' ');
            line.push_str(&next_line);
        }

        let mut iter_from_x = line.chars()
            .enumerate()
            .skip(self.cursor.x)
            .skip_while(|(_, c)| *c == ' ');


        if let Some((curr_word_start, curr_char)) = iter_from_x.next() {

            let current_word: String = iter_from_x.clone()
                .take_while(|(_, c)|{
                    if shift {
                        *c != ' '
                    } else {
                        *c != ' ' && c.is_alphabetic() == curr_char.is_alphabetic()
                    }
                }).map(|i| i.1)
            .collect();


            if let Some((mut next_word_start, mut next_char)) = iter_from_x.nth(current_word.len()) {
                if next_char == ' ' {
                   (next_word_start, next_char) = iter_from_x.find(|(_, c)| *c != ' ').unwrap_or((0, ' '));
                }
                let mut has_moved_down = false;
                if next_word_start > origin_line_len {
                    next_word_start -= origin_line_len + 1;
                    has_moved_down = true;
                }

                let next_word: String = iter_from_x
                    .skip_while(|(_, c)| *c == ' ')
                    .take_while(|(_, c)|{
                        if shift {
                            *c != ' '
                        } else {
                            *c != ' ' && c.is_alphabetic() == next_char.is_alphabetic()
                        }
                    })
                    .map(|i| i.1).collect();

                return (has_moved_down, vec![(curr_word_start, current_word), (next_word_start, next_word)]);
            }

            return (false, vec![(curr_word_start, current_word)]);
        }

        (false, vec![])
    }

    pub fn cursor_move_to_next_word_start(&mut self, shift: bool) {
        let (has_moved_down, words) = self.get_curr_next_word(shift);

        if words.len() > 1 {
            if has_moved_down {
                self.cursor_move_down();
            }
            self.cursor.x = words[1].0;
        }
    }

    pub fn cursor_move_to_curr_word_end(&mut self, shift: bool) {
        let (has_moved_down, words) = self.get_curr_next_word(shift);

        if words.len() > 0 {
            if words[0].1.len() == 0 && words.len() == 2 {
                self.cursor.x = words[1].0 + words[1].1.len();
            } else {
                self.cursor.x = words[0].0 + words[0].1.len();
            }
            if has_moved_down {
                self.cursor_move_down();
            }
        }
    }
}
