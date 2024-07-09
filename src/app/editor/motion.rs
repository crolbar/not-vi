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
}
