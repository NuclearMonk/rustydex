use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Style, Stylize}, widgets::{Block, Row, StatefulWidget, Table, Widget}};

use crate::app::widgets::pokedex::entries::EntriesWidget;

impl Widget for &mut EntriesWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut state = self.state.write().unwrap();
        let [list_area, query] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(area);
        let block = Block::bordered()
            .title(format!("Entries"))
            .title_bottom("j/k to scroll").border_style(if state.focused(){Style::default().fg(Color::Blue)} else {Style::default()});
        let rows: Vec<Row> = state
            .entries()
            .iter()
            .map(|entry| {
                Row::new(vec![
                    format!("#{:0>4}", entry.entry_number),
                    entry.pokemon_species.name.to_uppercase(),
                ])
            })
            .collect();
        let widths = [Constraint::Length(5), Constraint::Fill(1)];
        let table = Table::new(rows, widths)
            .block(block)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .highlight_symbol(">>")
            .row_highlight_style(Style::new().black().on_blue());
        StatefulWidget::render(table, list_area, buf, &mut state.table_state());
    }
}
