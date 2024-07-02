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
    Insert
}


pub struct EditorConfing {
    scrolloff: u16,
    sidescrolloff: u16,
}

pub struct Editor {
     buf: Vec<String>,
     mode: EditorMode,
     window: Rect,
     num_window: Rect,
     pub cursor: Cursor,
     buffered_char: Option<char>,
     pub conf: EditorConfing,
     scroll: (u16, u16),

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
            cursor: Cursor::new(&h[1]),
            window: h[1],
            num_window: h[0],
            buffered_char: None,
            scroll: (0, 0),
            conf: EditorConfing {
                scrolloff: 15,
                sidescrolloff: 35
            },

            dbg: String::new(),
        })
    }

    pub fn get_window(&mut self) -> Rect  { self.window }
    pub fn get_num_win(&mut self) -> Rect  { self.num_window }
        

    pub fn update_rects(&mut self, v: Rc<[Rect]>) {
        let h = Self::create_rects(v, &self.buf);

        self.window = h[1];
        self.num_window = h[0];

        self.cursor.update_min_max(self.window);
    }
    
    fn create_rects(v: Rc<[Rect]>, buf: &Vec<String>) -> Rc<[Rect]>  {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min((buf.len() as f32).log10().floor() as u16 + 1),
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

    pub fn is_insert(&self) -> bool { self.mode == EditorMode::Insert }

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
