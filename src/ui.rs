use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(app: &mut App, frame: &mut Frame) {
    let buf = app.editor.get_buf().join("\n");
    let scroll = app.editor.get_scroll();

    let o = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(100),
            Constraint::Min(1),
        ]).split(frame.size());


    app.editor.set_rect(o[0]);
    frame.render_widget(
        Paragraph::new(buf)
        .scroll(scroll),
        o[0]
    );

    frame.set_cursor(
        (app.editor.cursor.get_x() as u16).saturating_sub(app.editor.get_scroll().1),
        (app.editor.cursor.get_y() as u16).saturating_sub(app.editor.get_scroll().0)
    );
}
