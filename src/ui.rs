use std::str::FromStr;

use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Gauge, List, ListDirection, ListItem, ListState, Paragraph, Row, StatefulWidget, Table, TableState, Widget},
};
use rustemon::model::pokemon::{PokemonAbility, PokemonMove, PokemonStat, PokemonType};

use crate::{
    app::{App, CurrentScreen, LoadingState, PokedexListWidget, PokedexViewWidget, PokedexWidget},
    pokemon::{MonStat, MonType},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [title_area, body_area] = vertical.areas(area);
        let title = Line::from("RustyDex").centered();
        title.render(title_area, buf);
        match &self.current_screen {
            CurrentScreen::Pokedex(widget) => widget.render(body_area, buf),
        }
    }
}

impl Widget for &PokedexViewWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = self.state.write().unwrap();
        let loading_state =
            Line::from(format!("{:?}", state.loading_state)).alignment(Alignment::Right);
        let block = Block::bordered()
            .title(state.name.clone())
            .title(loading_state);
        match state.loading_state {
            LoadingState::Loading => {
                block.render(area, buf);
            }
            LoadingState::Loaded => match &state.pokemon.clone() {
                None => {
                    block.render(area, buf);
                }
                Some(mon) => {
                    let [left, right] =
                        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                            .areas(block.inner(area));
                    let [name, types,_, stats, abilities] = Layout::vertical([
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(8),
                        Constraint::Length(8),
                    ])
                    .areas(left);
                    Span::from(mon.name.to_uppercase()).bold().render(name, buf);
                    render_types(&mon.types, types, buf);
                    render_stats(&mon.stats, stats, buf);
                    render_abilities(&mon.abilities,&mut state.ability_table_state,abilities, buf);
                    render_moves(&mon.moves, right, buf);
                    block.render(area, buf);
                }
            },
            _ => {}
        }
    }
}

fn render_stats(stats: &Vec<PokemonStat>, area: Rect, buf: &mut Buffer) {
    fn render_stat(stat: MonStat, value: i64, area: Rect, buf: &mut Buffer) {
        let [label_area, value_area, gauge_area] = Layout::horizontal([
            Constraint::Length(6),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(area);
        Text::from(stat.to_short_string()).render(label_area, buf);
        Text::from(value.to_string()).render(value_area, buf);
        Gauge::default()
            .gauge_style(stat.bg())
            .label("")
            .ratio((value) as f64 / 255f64)
            .use_unicode(true)
            .render(gauge_area, buf)
    }
    let [hp, atk, def, satk, sdef, spd] =
        Layout::vertical([Constraint::Length(1); 6]).areas(area);
    render_stat(MonStat::HP, stats[0].base_stat, hp, buf);
    render_stat(MonStat::Attack, stats[1].base_stat, atk, buf);
    render_stat(MonStat::Defense, stats[2].base_stat, def, buf);
    render_stat(MonStat::SpecialAttack, stats[3].base_stat, satk, buf);
    render_stat(MonStat::SpecialDefense, stats[4].base_stat, sdef, buf);
    render_stat(MonStat::Speed, stats[5].base_stat, spd, buf);
}
fn render_abilities(abilities: &Vec<PokemonAbility>,state :&mut TableState, area: Rect, buf: &mut Buffer) {
    let block = Block::bordered().title("Abilities");
    let rows: Vec<Row> = abilities.iter().map(|a| Row::new(vec![a.ability.name.to_uppercase(), if a.is_hidden {String::from("Hidden")} else {String::new()} ])).collect();
    let widths = [Constraint::Fill(1), Constraint::Length(6)];
    let list = Table::new(rows, widths).block(block);
    StatefulWidget::render(list, area, buf, state);
}
fn render_moves(moves: &Vec<PokemonMove>, area: Rect, buf: &mut Buffer) {
    let block = Block::bordered().title("Moves");
    block.render(area, buf)
}
fn render_types(types: &Vec<PokemonType>, area: Rect, buf: &mut Buffer) {
    let chunks = Layout::horizontal([ratatui::layout::Constraint::Length(8); 2]).split(area);
    for t in types {
        type_span(MonType::from_str(&t.type_.name).unwrap())
            .render(chunks[(t.slot - 1) as usize], buf);
    }
}

fn type_span<'a>(type_: MonType) -> Span<'a> {
    Span::styled(
        type_.to_string().to_uppercase(),
        Style::default().bg(type_.bg()).fg(type_.fg()),
    )
}

impl Widget for &PokedexWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::horizontal([Constraint::Length(24), Constraint::Min(0)]).split(area);
        let mut state = self.state.write().unwrap();
        state.list_widget.render(chunks[0], buf);
        state.pokemon_view_widget.render(chunks[1], buf);
    }
}

impl Widget for &mut PokedexListWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [list_area, query] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(area);
        let block = Block::bordered()
            .title("Pokedex")
            .title_bottom("j/k to scroll");
        let rows: Vec<Row> = self
            .entries
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
        StatefulWidget::render(table, list_area, buf, &mut self.table_state);
        let block = Block::bordered();
        let p = Paragraph::new(Text::from(self.query.clone())).block(block);
        p.render(query, buf);
    }
}
