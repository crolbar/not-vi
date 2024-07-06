mod cursor;
mod normal;
mod insert;
mod scroll;
use std::rc::Rc;
use anyhow::Result;
use cursor::Cursor;
use crossterm::{execute, cursor::SetCursorStyle};
use ratatui::prelude::*;

#[derive(PartialEq)]
pub enum EditorMode {
    Normal,
    Insert,
    Replace, 
}


pub struct EditorConfing {
    scrolloff: u16,
    sidescrolloff: u16,
    relativenumber: bool,
}

pub struct Editor {
     buf: Vec<String>,
     mode: EditorMode,
     window: Rect,
     line_num_window: Rect,
     pub cursor: Cursor,
     buffered_char: Option<char>,
     pub conf: EditorConfing,
     scroll: (u16, u16),
     pub replaced_chars: Vec<char>,

     pub dbg: String,
}

impl Editor {
    pub fn new(v: Rc<[Rect]>) -> Result<Self> {
        let buf = std::fs::read_to_string("src/app/editor.rs")?
            .split('\n')
            .map(|x| x.to_string())
            .collect();

        let h = Self::create_rects(v, &buf);

        Ok(Self {
            buf,
            mode: EditorMode::Normal,
            cursor: Cursor::new(&h[2]),
            window: h[2],
            line_num_window: h[0],
            buffered_char: None,
            scroll: (0, 0),
            conf: EditorConfing {
                scrolloff: 15,
                sidescrolloff: 35,
                relativenumber: true,
            },
            replaced_chars: Vec::new(),

            dbg: String::new(),
        })
    }

    pub fn get_window(&mut self) -> Rect  { self.window }
    pub fn get_line_num_win(&mut self) -> Rect  { self.line_num_window }
    pub fn get_curr_line_len(&self) -> usize { 
        self.buf
        .get(self.cursor.get_y())
        .unwrap_or(&"".to_string())
        .len() 
    }

    pub fn update_rects(&mut self, v: Rc<[Rect]>) {
        let h = Self::create_rects(v, &self.buf);

        self.window = h[2];
        self.line_num_window = h[0];

        self.cursor.update_min_max(self.window);
    }
    
    fn create_rects(v: Rc<[Rect]>, buf: &Vec<String>) -> Rc<[Rect]>  {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min((buf.len() as f32).log10().floor() as u16 + 1),
                Constraint::Min(1),
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ]).split(v[0])
    }


    pub fn buffer_char(&mut self, char: char) -> Result<()> {
        execute!(std::io::stderr(), SetCursorStyle::SteadyUnderScore)?;
        self.buffered_char = Some(char);
        Ok(())
    }

    pub fn remove_bufferd_char(&mut self) -> Result<()> {
        execute!(std::io::stderr(), SetCursorStyle::SteadyBlock)?;
        self.buffered_char = None;
        Ok(())
    }

    pub fn get_buf(&self) -> &Vec<String> { &self.buf }

    pub fn get_scroll(&self) -> (u16, u16) { self.scroll }
    pub fn get_mode(&self) -> &EditorMode { &self.mode }

    pub fn is_normal(&self) -> bool { self.mode == EditorMode::Normal }
    pub fn is_insert(&self) -> bool { self.mode == EditorMode::Insert }
    pub fn is_replace(&self) -> bool { self.mode == EditorMode::Replace }

    pub fn enter_replace(&mut self) -> Result<()> {
        self.mode = EditorMode::Replace;
        execute!(std::io::stderr(), SetCursorStyle::SteadyUnderScore)?;

        Ok(())
    }

    pub fn enter_normal(&mut self) -> Result<()> {
        self.mode = EditorMode::Normal;
        execute!(std::io::stderr(), SetCursorStyle::SteadyBlock)?;

        Ok(())
    }

    pub fn enter_insert(&mut self) -> Result<()> {
        self.mode = EditorMode::Insert;
        execute!(std::io::stderr(), SetCursorStyle::SteadyBar)?;

        Ok(())
    }
}

impl EditorConfing {
    pub fn uses_relativenumber(&self) -> bool { self.relativenumber == true }
}
