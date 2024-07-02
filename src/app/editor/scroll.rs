use super::Editor;

impl Editor {
    pub fn set_scroll(&mut self) { 
        {
            let y = self.cursor.get_y() as u16;
            let win_height = self.window.height;
            let num_lines = self.buf.len() as u16 - 2;

            {
                let lines_till_eow: i16 = win_height.saturating_sub(1) as i16 - y.saturating_sub(self.scroll.0) as i16;
                if lines_till_eow.unsigned_abs() + y < (num_lines) {
                    if lines_till_eow.is_negative() {
                        self.scroll.0 += lines_till_eow.unsigned_abs() + self.conf.scrolloff;
                    } else if lines_till_eow.unsigned_abs() <= self.conf.scrolloff {
                        self.scroll.0 += self.conf.scrolloff - lines_till_eow.unsigned_abs()
                    }
                } else {
                    self.scroll.0 = (num_lines + 1).saturating_sub(win_height);
                };
            }
            {
                let lines_till_sow: i16 = y as i16 - self.scroll.0 as i16;
                self.scroll.0 = self.scroll.0.saturating_sub(
                    if lines_till_sow.is_negative() {
                        lines_till_sow.unsigned_abs() + self.conf.scrolloff
                    } else if lines_till_sow as u16 <= self.conf.scrolloff {
                        self.conf.scrolloff - lines_till_sow.unsigned_abs()
                    }else{0});
            }
        }

        {
            let x = self.cursor.get_x() as u16;
            let win_width = self.window.width;

                {
                    let num_chars_till_eol: i16 = win_width.saturating_sub(1) as i16 - x.saturating_sub(self.scroll.1) as i16;

                    if num_chars_till_eol.is_negative() {
                        self.scroll.1 += num_chars_till_eol.unsigned_abs() + self.conf.sidescrolloff;
                    } else if num_chars_till_eol.unsigned_abs() <= self.conf.sidescrolloff {
                        self.scroll.1 += self.conf.sidescrolloff - num_chars_till_eol.unsigned_abs();
                    }
                }
                {
                    let num_chars_till_sol: i16 = x as i16 - self.scroll.1 as i16;

                    self.scroll.1 = self.scroll.1.saturating_sub(
                        if num_chars_till_sol.is_negative() {
                            num_chars_till_sol.unsigned_abs() + self.conf.sidescrolloff
                        } else if num_chars_till_sol as u16 <= self.conf.sidescrolloff {
                            self.conf.sidescrolloff - num_chars_till_sol.unsigned_abs()
                        }else {0});
                }
        }
    }
}
