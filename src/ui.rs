use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(app: &mut App, frame: &mut Frame) {
    app.update_rects(frame.size());
    let buf = app.editor.get_buf();
    let display_buf = buf.join("\n");
    let scroll = app.editor.get_scroll();

    
    frame.render_widget(
        Paragraph::new("h"),
        app.editor.get_num_win()
    );


    frame.render_widget(
        Paragraph::new(display_buf)
        .scroll(scroll),
        app.editor.get_window()
    );

    frame.set_cursor(
        (app.editor.cursor.get_display_x() as u16).saturating_sub(app.editor.get_scroll().1),
        (app.editor.cursor.get_display_y() as u16).saturating_sub(app.editor.get_scroll().0)
    );
}
