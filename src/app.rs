pub mod editor;

use {
    anyhow::Result, editor::Editor, ratatui::prelude::*, std::rc::Rc
};

pub struct App {
    pub exit: bool,
     bar_win: Rect,
    pub editor: Editor,
}

impl App {
    pub fn new(window: Rect) -> Result<Self> {
        let v = Self::create_rects(window);

        Ok(Self {
            exit: false,
            bar_win: v[1],
            editor: Editor::new(v)?,
        })
    }

    pub fn get_bar_win(&mut self) -> Rect  {
        self.bar_win
    }

    pub fn update_rects(&mut self, frame_win: Rect) {
        let v = Self::create_rects(frame_win);
        self.bar_win = v[1];

        self.editor.update_rects(v);
    }

    fn create_rects(window: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100),
                Constraint::Min(1),
            ]).split(window)
    }
}
