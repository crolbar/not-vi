use super::Editor;



impl Editor {
    pub fn get_x_n_chars_left(&self, n: usize) -> usize {
        self.cursor.get_x().saturating_sub(n)
    }

    pub fn get_x_n_chars_right(&self, n: usize) -> usize {
        if let Some(line) = self.buf.get(self.cursor.get_y()) {

            let x_plus_n = self.cursor.get_x() + n;
            let line_len = line.len().saturating_sub(self.is_normal() as usize);

            if x_plus_n < line_len {
                x_plus_n
            } else {
                line_len
            }
        } else { 0 }
    }

    pub fn get_y_n_lines_down(&self, n: usize) -> usize {
        let buf_len = self.buf.len().saturating_sub(1);
        let y_plus_n = self.cursor.get_y() + n;
        if y_plus_n < buf_len {
            y_plus_n
        } else { buf_len }
    }

    pub fn get_y_n_lines_up(&self, n: usize) -> usize {
        self.cursor.get_y().saturating_sub(n)
    }

    pub fn get_y_next_empty_line(&self, n: usize) -> usize {
        let mut prev_is_empty = false;
        if let Some(empty) = self.buf.iter()
            .enumerate()
            .skip(self.cursor.get_y())
            .filter(|(i, l)| (
                l.is_empty() && !prev_is_empty && *i != self.cursor.get_y(),
                prev_is_empty = l.is_empty()
            ).0)
            .nth(n - 1)
            .map(|i| i.0)
        {
            empty
        } else {
            self.buf.len().saturating_sub(1)
        }
    }

    pub fn get_y_prev_empty_line(&self, n: usize) -> usize {
        let mut prev_is_empty = false;
        if let Some(empty) = self.buf.iter()
            .enumerate()
            .rev()
            .skip(self.buf.len().saturating_sub(1) - self.cursor.get_y())
            .filter(|(i, l)| (
                l.is_empty() && !prev_is_empty && *i != self.cursor.get_y(),
                prev_is_empty = l.is_empty()
            ).0)
            .nth(n - 1)
            .map(|i| i.0)
        {
            empty
        } else { 0 }
    }

    pub fn get_x_at_char(&self, rev: bool, till: bool, char: char) -> usize {
        let y = self.cursor.get_y();
        let x = self.cursor.get_x();

        if let Some(line) = self.buf.get(y) {
            if rev {
                line.chars()
                    .rev()
                    .skip(line.len() - x)
                    .enumerate()
                    .find(|(_, c)| *c == char)
                    .map(|i| x - (i.0 + 1))
                    .unwrap_or(x + till as usize)
                    .saturating_add(till as usize)
            } else {
                line.chars()
                    .enumerate()
                    .skip(x + 1)
                    .find(|(_, c)| *c == char)
                    .map(|i| i.0)
                    .unwrap_or(x + till as usize)
                    .saturating_sub(till as usize)
            }
        } else {self.cursor.get_x()}
    }
}
