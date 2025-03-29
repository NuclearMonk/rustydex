use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, text::Line, widgets::Widget};

use crate::app::{App, CurrentScreen};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [title_area, body_area] = vertical.areas(area);
        let title = Line::from("RustyDex").centered();
        title.render(title_area, buf);
        match &self.current_screen {
            CurrentScreen::Pokedex(widget) => widget.clone().render(body_area, buf),
        }
    }
}