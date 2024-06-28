mod event;
mod update;
mod tui;
mod app;
use app::App;
mod ui;
use update::update;
use anyhow::Result;
use crossterm::event::Event;

fn main() -> Result<()> {
    let mut app = App::default();
    let mut tui = tui::Tui::enter()?;

    while !app.exit {
        tui.draw(&mut app)?;

        match tui.event_handler.recv()? {
            Event::Key(key) => update(&mut app, &mut tui, key)?,
            _ => ()
        }
    }

    tui.exit()?;
    Ok(())
}
