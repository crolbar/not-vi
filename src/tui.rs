use crossterm::{execute, event::{EnableMouseCapture, DisableMouseCapture},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::SetCursorStyle,
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io::{Stderr, stderr};
use anyhow::Result;
use crate::{event::EventHandler, ui::render};
use crate::app::App;

pub struct Tui {
    pub term: Terminal<CrosstermBackend<Stderr>>,
    pub event_handler: EventHandler,
}

impl Tui {
    pub fn enter() -> Result<Self> {
        enable_raw_mode()?;
        execute!(stderr(), EnterAlternateScreen, EnableMouseCapture, SetCursorStyle::SteadyBlock)?;
        let mut term = Terminal::new(CrosstermBackend::new(stderr()))?;
        term.clear()?;

        let panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic| {
            Self::leave_tui().unwrap();
            panic_hook(panic);
        }));

        Ok(Tui {
            term,
            event_handler: EventHandler::new()
        })
    }

    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.term.draw(|frame| render(app, frame))?;
        Ok(())
    }

    pub fn leave_tui() -> Result<()> {
        execute!(stderr(), LeaveAlternateScreen, DisableMouseCapture, SetCursorStyle::SteadyBlock)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::leave_tui()?;
        self.term.show_cursor()?;
        Ok(())
    }
}
