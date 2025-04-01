use std::str::FromStr;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Gauge, Widget},
};
use rustemon::model::pokemon::{PokemonMove, PokemonStat, PokemonType};

use crate::{
    app::widgets::pokedex::detail::{LoadingState, DetailsWidget}, pokemon::{MonStat, MonType}
};

impl Widget for &DetailsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
    //     let state = self.state.write().unwrap();
    //     let loading_state =
    //         Line::from(format!("{0}", state.loading_state())).alignment(Alignment::Right);

    //     let block = Block::bordered()
    //         .title(loading_state).border_style(if state.focused(){Style::default().fg(Color::Blue)} else {Style::default()});
    //     match state.loading_state().clone() {
    //         LoadingState::Loading(name,_) => {
    //             Span::from(name.to_uppercase()).bold().render(block.inner(area), buf);
    //             block.render(area, buf);
    //         }
    //         LoadingState::Loaded(pokemon) =>{//This Shite
    //                 let [left, right] =
    //                     Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
    //                         .areas(block.inner(area));
    //                 let [name, types, _, stats, abilities] = Layout::vertical([
    //                     Constraint::Length(1),
    //                     Constraint::Length(1),
    //                     Constraint::Length(1),
    //                     Constraint::Length(8),
    //                     Constraint::Length(8),
    //                 ])
    //                 .areas(left);
    //                 Span::from(pokemon.name.to_uppercase()).bold().render(name, buf);
    //                 render_types(&pokemon.types, types, buf);
    //                 render_stats(&pokemon.stats, stats, buf);
    //                 // render_abilities(
    //                 //     &pokemon.abilities,
    //                 //     &mut state.ability_table_state,
    //                 //     abilities,
    //                 //     buf,
    //                 // );
    //                 self.abilities.clone().render(abilities, buf);
    //                 self.moves.clone().render(right, buf);
    //                 // render_moves(&pokemon.moves, right, buf);
    //                 block.render(area, buf);
                
    //         },
    //         _ => {}
    //     }
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
    let [hp, atk, def, satk, sdef, spd] = Layout::vertical([Constraint::Length(1); 6]).areas(area);
    render_stat(MonStat::HP, stats[0].base_stat, hp, buf);
    render_stat(MonStat::Attack, stats[1].base_stat, atk, buf);
    render_stat(MonStat::Defense, stats[2].base_stat, def, buf);
    render_stat(MonStat::SpecialAttack, stats[3].base_stat, satk, buf);
    render_stat(MonStat::SpecialDefense, stats[4].base_stat, sdef, buf);
    render_stat(MonStat::Speed, stats[5].base_stat, spd, buf);
}
// fn render_abilities(
//     abilities: &Vec<PokemonAbility>,
//     state: &mut TableState,
//     area: Rect,
//     buf: &mut Buffer,
// ) {
//     let block = Block::bordered().title("Abilities");
//     let rows: Vec<Row> = abilities
//         .iter()
//         .map(|a| {
//             Row::new(vec![
//                 a.ability.name.to_uppercase(),
//                 if a.is_hidden {
//                     String::from("Hidden")
//                 } else {
//                     String::new()
//                 },
//             ])
//         })
//         .collect();
//     let widths = [Constraint::Fill(1), Constraint::Length(6)];
//     let list = Table::new(rows, widths).block(block);
//     StatefulWidget::render(list, area, buf, state);
// }
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
