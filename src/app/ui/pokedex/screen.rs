use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, text::Line, widgets::{Block, Widget}};

use crate::app::screens::pokedex::{LoadingState, PokedexScreen};

impl Widget for &mut PokedexScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let state = self.state.read().unwrap();
        let loading_state = Line::from(format!("{0}", state.loading_state()));

        match state.loading_state() {
            LoadingState::Loaded(_) => {
                let block = Block::default().title(loading_state);
                let chunks = Layout::horizontal([Constraint::Length(24), Constraint::Min(0)])
                    .split(block.inner(area));
                self.entries.render(chunks[0], buf);
                self.detail_view.render(chunks[1], buf);
                block.render(area, buf);
            }
            _ => Block::default().title(loading_state).render(area, buf),
        }
    }
}
