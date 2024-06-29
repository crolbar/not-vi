mod event;
mod update;
mod tui;
mod app;
use app::App;
mod ui;
use anyhow::Result;
use crossterm::event::Event;

fn main() -> Result<()> {
    let mut tui = tui::Tui::enter()?;
    let mut app = App::new()?;

    while !app.exit {
        tui.draw(&mut app)?;

        match tui.event_handler.recv()? {
            Event::Key(key) => app.update(&mut tui, key)?,
            _ => ()
        }
    }

    tui.exit()?;
    Ok(())
}
