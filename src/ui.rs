use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("hi"),
        frame.size()
    )
}
