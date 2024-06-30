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


    pub fn move_x_to(&mut self, line_len: usize) {
        self.x = line_len;
    }

    pub fn move_right(&mut self, maxx: u16, buf: Vec<String>, is_insert: bool) {
        if 
            (maxx - 1) as usize > self.x &&
            buf[self.y].len().saturating_sub(!is_insert as usize) > self.x
        {
            self.x += 1;
        }

        self.un_trunc_x = Some(self.x);
    }

    pub fn move_left(&mut self) {
        self.x = self.x.saturating_sub(1);

        self.un_trunc_x = Some(self.x);
    }


    fn handle_virt_move_x(&mut self, buf: Vec<String>) {
        if let Some(l) = buf.get(self.y as usize) {
            if self.x >= l.len() {
                self.x = l.len().saturating_sub(1)
            } else {
                if let Some(x) = self.un_trunc_x {
                    if x < l.len() {
                        self.x = x;
                    }
                }
            }
        }
    }

    pub fn move_down(&mut self, maxy: u16, buf: Vec<String>) {
        if 
            (maxy - 1) as usize > self.y &&
            buf.len().saturating_sub(2) > self.y
        {
            self.y += 1;
        }

        self.handle_virt_move_x(buf);
    }

    pub fn move_up(&mut self, buf: Vec<String>, ignore_un_trunc_x: bool) {
        self.y = self.y.saturating_sub(1);

        if !ignore_un_trunc_x {
            self.handle_virt_move_x(buf);
        }
    }
}
