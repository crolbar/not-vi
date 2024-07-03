use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(app: &mut App, frame: &mut Frame) {
    app.update_rects(frame.size());
    let buf = app.editor.get_buf();
    let display_buf = buf.join("\n");
    let scroll = app.editor.get_scroll();


    app.render_line_nums(frame);


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

impl App {
    fn render_line_nums(&mut self, frame: &mut Frame) {
        let y = self.editor.cursor.get_y();
        let first_visable_line_num = y.saturating_sub(y.saturating_sub(self.editor.get_scroll().0.into()));
        let last_visable_line_num = self.editor.get_window().height.saturating_sub(1) as usize + y.saturating_sub(y.saturating_sub(self.editor.get_scroll().0.into()));

        let iter = (first_visable_line_num..=last_visable_line_num).into_iter();
        let nums: Vec<Line> = iter
            .map(|i|
                if self.editor.conf.uses_relativenumber() {
                    if i == y {
                        Line::from(i.to_string()).alignment(Alignment::Left)
                    } else {
                        Line::from(y.abs_diff(i).to_string())
                    }
                } else {
                    Line::from(i.to_string())
                }
            ).collect();

        frame.render_widget(
            Paragraph::new(nums)
            .alignment(Alignment::Right),
            self.editor.get_line_num_win()
        );
    }
}
