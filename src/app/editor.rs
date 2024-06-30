mod cursor;
mod normal;
mod insert;
use anyhow::Result;
use cursor::Cursor;
use crossterm::{execute, cursor::SetCursorStyle};
use ratatui::layout::Rect;

#[derive(PartialEq)]
pub enum EditorMode {
    Normal,
    Insert
}

pub struct Editor {
     buf: Vec<String>,
     mode: EditorMode,
     frame_rect: Rect,
     pub cursor: Cursor,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let buf = std::fs::read_to_string("src/tui.rs")?
            .split('\n')
            .map(|x| x.to_string())
            .collect();

        Ok(Self {
            buf,
            mode: EditorMode::Normal,
            cursor: Cursor::new(),
            frame_rect: Rect::default(),
        })
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.frame_rect = rect;
    }

    pub fn get_buf(&self) -> &Vec<String> { &self.buf }
    pub fn get_mode(&self) -> &EditorMode { &self.mode }
    pub fn is_insert(&self) -> bool { self.mode == EditorMode::Insert }

    pub fn enter_normal(&mut self) -> Result<()> {
        self.mode = EditorMode::Normal;
        execute!(std::io::stderr(), SetCursorStyle::SteadyBlock)?;

        Ok(())
    }

    pub fn enter_insert(&mut self) -> Result<()> {
        self.cursor_move_x_to(self.cursor.get_x() + 1);
        self.mode = EditorMode::Insert;
        execute!(std::io::stderr(), SetCursorStyle::SteadyBar)?;

        Ok(())
    }
}
