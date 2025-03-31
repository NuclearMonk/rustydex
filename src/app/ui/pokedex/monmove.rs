use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, text::{Line, Text}, widgets::{Block, Paragraph, Widget, Wrap}};

use crate::app::widgets::pokedex::monmove::{self, LoadingState, MoveWidget};

impl Widget for MoveWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let state = self.state.read().unwrap();
        let [header, body] = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);
        match state.loading_state() {
            LoadingState::Idle => Block::default().render(area, buf),
            LoadingState::Loading(pokemon_move)  | LoadingState::Lazy(pokemon_move)=> 
            {
                let block = Block::default();
                let [name_area, hidden_area] = Layout::horizontal([Constraint::Fill(1), Constraint::Length(6)]).areas(block.inner(header));
                Line::from(pokemon_move.move_.name.to_string()).render(name_area, buf);
            },
            monmove::LoadingState::Loaded(move_) => {
                let block = Block::default().style(self.style);
                let [name_area, hidden_area] = Layout::horizontal([Constraint::Fill(1), Constraint::Length(6)]).areas(block.inner(header));
                
                for name in &move_.names
                {
                    if name.language.name == "en"
                    {
                        Line::from(name.name.clone()).render(name_area, buf);

                    }
                }

                for effect in &move_.effect_entries
                {
                    if effect.language.name == "en"
                    {
                        Paragraph::new(vec![Line::from(effect.short_effect.clone())]).wrap(Wrap{trim: false}).render(body, buf);

                    }
                }
                block.render(area, buf);
            },
            monmove::LoadingState::Error(_) => {},
        }
        
    }
}
