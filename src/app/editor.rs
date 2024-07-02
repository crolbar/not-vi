mod cursor;
mod normal;
mod insert;
mod scroll;
use anyhow::Result;
use cursor::Cursor;
use crossterm::{execute, cursor::SetCursorStyle};
use ratatui::layout::Rect;

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
     pub window: Rect,
     pub cursor: Cursor,
     buffered_char: Option<char>,
     pub conf: EditorConfing,
     scroll: (u16, u16),
}

impl Editor {
    pub fn new() -> Result<Self> {
        let buf = std::fs::read_to_string("src/app/editor.rs")?
            .split('\n')
            .map(|x| x.to_string())
            .collect();

        Ok(Self {
            buf,
            mode: EditorMode::Normal,
            cursor: Cursor::new(),
            window: Rect::default(),
            buffered_char: None,
            scroll: (0, 0),
            conf: EditorConfing {
                scrolloff: 15,
                sidescrolloff: 35
            },
        })
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

    pub fn set_rect(&mut self, rect: Rect) {
        self.window = rect;
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
