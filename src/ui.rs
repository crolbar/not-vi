use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(app: &mut App, frame: &mut Frame) {

    app.editor.set_rect(frame.size());
    frame.set_cursor(app.editor.cursor.get_x() as u16, app.editor.cursor.get_y() as u16);

    frame.render_widget(
        Paragraph::new(app.editor.get_buf().join("\n")),
        frame.size()
    )
}
