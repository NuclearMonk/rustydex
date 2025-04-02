use ratatui::{prelude::*};
use ratatui::widgets::{Block, Paragraph};
use ratatui::{buffer::Buffer, layout::Rect};
use tui_widget_list::{ListBuilder, ListView};

use crate::app::widgets::pokedex::moves::MovesWidget;

impl Widget for &MovesWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (builder, count) = {
            let state = self.state.write().unwrap();
            let item_count = state.moves().len();
            let widgets :Vec<_>= state.moves().iter().map(|m| state.widgets_cache.get(&m.move_.name).unwrap().clone()).collect();
            // let widgets  = state.widgets_cache.values().cloned().collect();
            let focused = state.focused();
            let builder = ListBuilder::new(move |context| {
                let mut widget = widgets[context.index].clone();
                if focused && context.is_selected {
                    widget.set_style(Style::default().bg(Color::Blue).fg(Color::Black));
                }
                (widget.clone(), 3)
            });
            (builder, item_count)
        };
        let mut state = self.state.write().unwrap();
        let block = Block::bordered().border_style(if state.focused() {
            Style::default().fg(Color::Blue)
        } else {
            Style::default()
        });
        let list = ListView::new(builder, count)
            .infinite_scrolling(false)
            .block(block);
        list.render(area, buf, &mut state.list_state);
    }
}
