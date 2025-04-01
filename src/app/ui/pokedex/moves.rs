use ratatui::widgets::{Block, Paragraph};
use ratatui::{buffer::Buffer, layout::Rect};
use ratatui::prelude::*;
use tui_widget_list::{ListBuilder,ListView};

use crate::app::widgets;
use crate::app::widgets::pokedex::moves::MovesWidget;



impl Widget for &MovesWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // let mut state = self.state.write().unwrap();
        // let widgets = state.widgets.clone();
        // let focused = state.focused();
        // let builder = ListBuilder::new(move |context| {
        //     let mut widget = widgets[context.index].clone();
        //     if  focused && context.is_selected
        //     {
        //         widget.style = Style::default().bg(Color::Blue).fg(Color::Black);
        //     }
        //     (widget, 3)
        // });
        // let item_count = state.widgets.len();
        // let block = Block::bordered().border_style(if state.focused(){Style::default().fg(Color::Blue)} else {Style::default()});
        // let list = ListView::new(builder, item_count).infinite_scrolling(false).block(block);
        
        // list.render(area, buf, &mut state.list_state);
    }
}