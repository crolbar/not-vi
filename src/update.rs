use crossterm::event::{KeyCode, KeyEvent};
use crate::{app::App, tui::Tui};
use anyhow::Result;

pub fn update(app: &mut App, tui: &mut Tui, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') => {app.exit = true; },

        _ => ()
    }

    Ok(())
}
