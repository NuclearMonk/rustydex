mod app;

use std::{sync::{Arc, RwLock}, time::Duration};

use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use ratatui::{
    style::{Style, Stylize},
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, HighlightSpacing, Row, StatefulWidget, Table, TableState, Widget}, DefaultTerminal, Frame,
};
use rustemon::
    model::games::{Pokedex, PokemonEntry}
;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal).await;
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
struct App {
    should_quit: bool,
    pokedex: PokedexListWidget,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.pokedex.run();

        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => { terminal.draw(|frame| self.draw(frame))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [title_area, body_area] = vertical.areas(frame.area());
        let title = Line::from("Rustydex").centered();
        frame.render_widget(title, title_area);
        frame.render_widget(&self.pokedex, body_area);
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                    KeyCode::Char('j') | KeyCode::Down => self.pokedex.scroll_down(),
                    KeyCode::Char('k') | KeyCode::Up => self.pokedex.scroll_up(),
                    _ => {}
                }
            }
        }
    }
}



#[derive(Debug, Clone, Default)]
struct PokedexListWidget {
    state: Arc<RwLock<PokedexListState>>,
}

#[derive(Debug, Default)]
struct PokedexListState {
    pokemon: Vec<PokemonEntry>,
    loading_state: LoadingState,
    table_state: TableState,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum LoadingState {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error(String),
}

impl PokedexListWidget {
    fn run(&self) {
        let this = self.clone();
        tokio::spawn(this.fetch_pokedex());
    }
    fn set_loading_state(&self, state: LoadingState) {
        self.state.write().unwrap().loading_state = state;
    }
    async fn fetch_pokedex(self) {
        self.set_loading_state(LoadingState::Loading);
        let rustemon_client = rustemon::client::RustemonClient::default();
        match rustemon::games::pokedex::get_by_id( 1,&rustemon_client).await {
            Ok(dex) => self.on_load(dex).await,
            Err(err) => self.on_err(err),
        };
    }
    async fn on_load(&self, dex: Pokedex) {
        let mut state = self.state.write().unwrap();
        state.pokemon = dex.pokemon_entries;
        state.loading_state = LoadingState::Loaded;
    }


    fn on_err(&self, err: rustemon::error::Error) {
        self.set_loading_state(LoadingState::Error(err.to_string()));
    }

    fn scroll_down(&self) {
        self.state.write().unwrap().table_state.scroll_down_by(1);
    }

    fn scroll_up(&self) {
        self.state.write().unwrap().table_state.scroll_up_by(1);
    }
}



impl Widget for &PokedexListWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut state = self.state.write().unwrap();

        let loading_state = Line::from(format!("{:?}", state.loading_state)).right_aligned();
        let block = Block::bordered().title("Pokedex").title(loading_state);
        let rows = state.pokemon.iter().map(|entry| Row::new(vec![format!("#{:0>4}", entry.entry_number),format!("{}", entry.pokemon_species.name)]));
        let widths = [Constraint::Length(5), Constraint::Fill(1)];
        let table = Table::new(rows, widths)
            .block(block)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">").row_highlight_style(Style::new().black().on_blue());
        StatefulWidget::render(table, area,buf,&mut state.table_state)
    }
}
