use std::{collections::HashMap, hash::{Hash, Hasher}};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use super::{Editor, EditorMode};

use std::cmp::PartialEq;


pub struct Cmd {
    pub mode: EditorMode,
    pub keys: Vec<KeyEvent>,
    gnkey: bool,
}

impl PartialEq for Cmd {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode 
        && self.keys == other.keys 
        && self.gnkey == other.gnkey
    }
}

impl Eq for Cmd {}

impl Hash for Cmd {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mode.hash(state);
        self.keys.hash(state);
        self.gnkey.hash(state);
    }
}

impl Cmd {
    pub fn new() -> Self {
        Cmd {
            mode: EditorMode::Normal,
            keys: vec![],
            gnkey: false,
        }
    }

    pub fn push_key(&mut self, e: KeyEvent) {
        self.keys.push(e);
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.gnkey = false;
    }

    pub fn should_get_nkey(&self) -> bool {
        self.gnkey
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    pub fn get_char_n(&self, n: usize) -> Option<char> {
        if let Some(k) = self.keys.get(n) {
            if let crossterm::event::KeyCode::Char(c) = k.code {
                Some(c)
            } else { None }
        } else { None }
    }
}

pub struct Cmds {
    pub cmds: HashMap<Cmd, fn(&mut Editor)>,
}

macro_rules! bind_key {
    ($cmds:expr, $mode:expr, $keys:expr, $get_nkey:expr, $func:expr) => {
        $cmds.insert(
            Cmd {
                mode: $mode,
                keys: $keys,
                gnkey: $get_nkey,
            }, $func
        )
    };
}

impl Cmds {
    pub fn new() -> Self {
        let mut cmds: HashMap<Cmd, fn(&mut Editor)> = HashMap::new();

        bind_key!(cmds, EditorMode::Normal,
            vec![KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL)],
            false, Editor::cursor_move_down_half_win);

        bind_key!(cmds, EditorMode::Normal,
            vec![KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL)],
            false, Editor::cursor_move_up_half_win);

        bind_key!(cmds, EditorMode::Normal,
            vec![KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE)],
            false, Editor::cursor_move_bottom);

        bind_key!(cmds, EditorMode::Normal,
            vec![
                KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
            ],
            false, Editor::cursor_move_top);

        bind_key!(cmds, EditorMode::Normal,
            vec![KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE)],
            false, Editor::cursor_move_right);

        bind_key!(cmds, EditorMode::Normal,
            vec![KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE)],
            false, Editor::cursor_move_left);

        bind_key!(cmds, EditorMode::Normal,
            vec![KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE)],
            false, Editor::cursor_move_up);

        bind_key!(cmds, EditorMode::Normal,
            vec![KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE)],
            false, Editor::cursor_move_down);

        bind_key!(cmds, EditorMode::Pending,
            vec![KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE)],
            true, Editor::cursor_move_to_char);
        bind_key!(cmds, EditorMode::Pending,
            vec![KeyEvent::new(KeyCode::Char('F'), KeyModifiers::NONE)],
            true, Editor::cursor_move_to_char);
        bind_key!(cmds, EditorMode::Pending,
            vec![KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)],
            true, Editor::cursor_move_to_char);
        bind_key!(cmds, EditorMode::Pending,
            vec![KeyEvent::new(KeyCode::Char('T'), KeyModifiers::NONE)],
            true, Editor::cursor_move_to_char);

        Self { cmds }
    }

    pub fn should_get_additional(&mut self, curr_cmd: &mut Cmd) -> bool {
        let keys = &curr_cmd.keys;

        for cmd in self.cmds.keys() {
            if keys.iter()
                .zip(cmd.keys.clone())
                    .all(|(&k, k2)| k == k2)
            {
                // if the first `keys.len` keys match but there are more keys needed for the cmd
                // like in `gg`
                if keys.len() != cmd.keys.len() {
                    return true
                // if we have and match but the cmd requires the next key press 
                // like in 'f', 't', 'm'..
                } else if cmd.gnkey {
                    curr_cmd.gnkey = true;
                    return true
                }
            }
        }

        false
    }
}
