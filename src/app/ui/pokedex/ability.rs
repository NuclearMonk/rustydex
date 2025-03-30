use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, text::{Line, Text}, widgets::{Block, Paragraph, Widget, Wrap}};
use crate::app::widgets::pokedex::ability::AbilityWidget;

impl Widget for AbilityWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let state = self.state.read().unwrap();
        let [header, body] = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);
        match state.loading_state() {
            crate::app::widgets::pokedex::ability::LoadingState::Idle => Block::default().render(area, buf),
            crate::app::widgets::pokedex::ability::LoadingState::Loading(name) => 
            {
                let block = Block::default();
                let [name_area, hidden_area] = Layout::horizontal([Constraint::Fill(1), Constraint::Length(6)]).areas(block.inner(header));
                Line::from(name.to_string()).render(name_area, buf);
                if state.hidden()
                {

                    Line::from("Hidden".to_string()).render(hidden_area, buf);
                }
            },
            crate::app::widgets::pokedex::ability::LoadingState::Loaded(ability) => {
                let block = Block::default().style(self.style);
                let [name_area, hidden_area] = Layout::horizontal([Constraint::Fill(1), Constraint::Length(6)]).areas(block.inner(header));
                
                for name in &ability.names
                {
                    if name.language.name == "en"
                    {
                        Line::from(name.name.clone()).render(name_area, buf);

                    }
                }

                for name in &ability.effect_entries
                {
                    if name.language.name == "en"
                    {
                        Paragraph::new(vec![Line::from(name.short_effect.clone())]).wrap(Wrap{trim: false}).render(body, buf);

                    }
                }

                if state.hidden()
                {

                    Line::from("Hidden".to_string()).render(hidden_area, buf);
                }
                block.render(area, buf);
            },
            crate::app::widgets::pokedex::ability::LoadingState::Error(_) => {},
        }
        
    }
}
