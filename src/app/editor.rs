mod cursor;
mod normal;
mod insert;
mod scroll;
mod motion;
pub mod binds;
use std::rc::Rc;
use anyhow::Result;
use binds::{Cmd, Cmds, OP};
use cursor::Cursor;
use crossterm::{cursor::SetCursorStyle, event::{KeyCode, KeyEvent}, execute};
use ratatui::prelude::*;

#[derive(Hash, PartialEq, Eq)]
pub enum EditorMode {
    Any,
    Normal,
    Pending,
    Insert,
    Replace, 
}


impl std::fmt::Display for EditorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorMode::Normal => write!(f, "Normal"),
            EditorMode::Pending => write!(f, "Pending"),
            EditorMode::Insert => write!(f, "Insert"),
            EditorMode::Replace => write!(f, "Replace"),
            EditorMode::Any => write!(f, "Any"),
        }
    }
}

pub struct EditorConfing {
    scrolloff: u16,
    sidescrolloff: u16,
    relativenumber: bool,

    shiftwidth: usize,
    tabspop: usize,
}

pub struct Editor {
    buf: Vec<String>,
    mode: EditorMode,
    window: Rect,
    line_num_window: Rect,
    pub cursor: Cursor,
    pub conf: EditorConfing,
    scroll: (u16, u16),
    pub replaced_chars: Vec<char>,

    pub curr_cmd: Cmd,
    nkey: Option<KeyEvent>,
    pub cmds: Cmds,

    pub op_type: Option<OP>,
    pub motion_end: Option<(usize, usize)>,
    motion_start: Option<(usize, usize)>,

    pub dbg: String,
}

impl Editor {
    pub fn new(v: Rc<[Rect]>) -> Result<Self> {
        let mut buf: Vec<String> = std::fs::read_to_string("src/app/editor.rs")?
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        buf.pop();

        let h = Self::create_rects(v, &buf);

        Ok(Self {
            buf,
            mode: EditorMode::Normal,
            cursor: Cursor::new(&h[2]),
            window: h[2],
            line_num_window: h[0],
            scroll: (0, 0),
            conf: EditorConfing {
                scrolloff: 15,
                sidescrolloff: 35,
                relativenumber: true,
                shiftwidth: 4,
                tabspop: 4,
            },
            replaced_chars: Vec::new(),

            cmds: Cmds::new(),
            nkey: None,
            curr_cmd: Cmd::new(),

            op_type: None,
            motion_end: None,
            motion_start: None,

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

    pub fn get_buf(&self) -> &Vec<String> { &self.buf }

    pub fn get_scroll(&self) -> (u16, u16) { self.scroll }
    pub fn get_mode(&self) -> &EditorMode { &self.mode }

    pub fn is_normal(&self) -> bool { self.mode == EditorMode::Normal }
    pub fn is_insert(&self) -> bool { self.mode == EditorMode::Insert }
    pub fn is_replace(&self) -> bool { self.mode == EditorMode::Replace }

    pub fn enter_replace(&mut self) -> Result<()> {
        self.mode = EditorMode::Replace;
        self.curr_cmd.set_mode(EditorMode::Replace);
        execute!(std::io::stderr(), SetCursorStyle::SteadyUnderScore)?;

        Ok(())
    }

    pub fn enter_normal(&mut self) -> Result<()> {
        self.mode = EditorMode::Normal;
        self.curr_cmd.set_mode(EditorMode::Normal);
        self.curr_cmd.clear();
        execute!(std::io::stderr(), SetCursorStyle::SteadyBlock)?;

        Ok(())
    }

    pub fn enter_insert(&mut self) {
        self.mode = EditorMode::Insert;
        self.curr_cmd.set_mode(EditorMode::Insert);
        execute!(std::io::stderr(), SetCursorStyle::SteadyBar).unwrap();

        //Ok(())
    }

    pub fn enter_pending(&mut self) -> Result<()> {
        self.mode = EditorMode::Pending;
        self.curr_cmd.set_mode(EditorMode::Pending);
        execute!(std::io::stderr(), SetCursorStyle::SteadyUnderScore)?;
        Ok(())
    }

    pub fn get_nkey_char(&self) -> Option<char> {
        if let Some(key) = self.nkey {
            if let KeyCode::Char(c) = key.code {
                Some(c)
            } else {None}
        } else {None}
    }

    pub fn set_nkey(&mut self, key: KeyEvent) -> Result<()> {
        self.nkey = Some(key);
        self.enter_normal()?;
        Ok(())
    }

    pub fn do_if_contains(&mut self) -> Result<()> {
        self.dbg = format!("{:?}, {}", self.curr_cmd.keys, self.curr_cmd.mode);

        if let Some(f) = self.cmds.cmds.get(&self.curr_cmd) {
            f(self);

            self.curr_cmd.clear();
            if self.op_type.is_none() {
                self.enter_normal()?;
            }
        } else

        if self.cmds.should_get_additional(&mut self.curr_cmd) {
            if self.curr_cmd.should_get_nkey() {
                self.enter_pending().unwrap();
            }
        } else {
            self.enter_normal()?;
            self.curr_cmd.clear();
        }

        if let (Some(op), Some(me)) = (&self.op_type, self.motion_end) {
            match op {
                OP::Delete => {
                    self.op_delete(me.0)
                }
                _ => ()
            }

            self.op_type = None;
            self.motion_end = None;
            self.enter_normal()?;
        }

        Ok(())
    }
}

impl EditorConfing {
    pub fn uses_relativenumber(&self) -> bool { self.relativenumber == true }
}
