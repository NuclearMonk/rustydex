use std::{ops::Index, sync::{Arc, RwLock}, time::Duration};

use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Style, Stylize}, text::Line, widgets::{Block, List, ListItem, Paragraph, Row, StatefulWidget, Table, TableState, Widget}, DefaultTerminal, Frame
};
use rustemon::{
    error::Error,
    model::{games::{Pokedex, PokemonEntry}, pokemon::Pokemon},
};
use tokio_stream::StreamExt;
#[derive(Debug, Default)]
pub enum LoadingState {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error(String),
}

#[derive(Debug)]
pub enum CurrentScreen {
    Pokedex(PokedexWidget),
}

impl Default for CurrentScreen {
    fn default() -> Self {
        CurrentScreen::Pokedex(PokedexWidget::default())
    }
}
#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub current_screen: CurrentScreen,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        match &self.current_screen {
            CurrentScreen::Pokedex(dex) => dex.run(),
        }

        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();
        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => { terminal.draw(|frame| self.render(frame))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [title_area, body_area] = vertical.areas(frame.area());
        let title = Line::from("Rustydex").centered().bold();
        frame.render_widget(title, title_area);
        match &self.current_screen {
            CurrentScreen::Pokedex(widget) => frame.render_widget(widget, body_area),
        }
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                    KeyCode::Char('j') | KeyCode::Down => self.scroll_down(),
                    KeyCode::Char('k') | KeyCode::Up => self.scroll_up(),
                    _ => {}
                }
            }
        }
    }

    fn scroll_down(&self) {
        match &self.current_screen {
            CurrentScreen::Pokedex(widget) => widget.scroll_down(),
        }
    }
    fn scroll_up(&self) {
        match &self.current_screen {
            CurrentScreen::Pokedex(widget) => widget.scroll_up(),
        }
    }
}

#[derive(Debug)]
pub struct PokedexWidget {
    list_widget: PokedexListWidget,
    pokemon_widget : PokedexPokemonWidget
}

impl Default for PokedexWidget {
    fn default() -> Self {
        Self {
            list_widget: PokedexListWidget::new(1),
            pokemon_widget: PokedexPokemonWidget::default(),
        }
    }
}

impl Widget for &PokedexWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::horizontal([Constraint::Length(24), Constraint::Min(0)]).split(area);
        self.list_widget.render(chunks[0], buf);
        self.pokemon_widget.render(chunks[1], buf);
    }
}



impl PokedexWidget {
    fn run(&self) {
        self.list_widget.run();
    }

    fn set_dex_id(&mut self, id: i64) {
        self.list_widget.set_dex(id);
    }

    fn scroll_down(&self) {
        match self.list_widget.scroll_down(){
            Some(name) => self.pokemon_widget.set_mon(name),
            None=>{},
        };
    }
    fn scroll_up(&self) {
        match self.list_widget.scroll_up(){
            Some(name) => self.pokemon_widget.set_mon(name),
            None=>{},
        };
    }
}

#[derive(Debug, Clone)]
pub struct PokedexListWidget {
    state: Arc<RwLock<PokedexListState>>,
}

impl Widget for &PokedexListWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = self.state.write().unwrap();
        let loading_state = Line::from(format!("{:?}", state.loading_state));
        let block = Block::bordered()
            .title("Pokedex")
            .title(loading_state)
            .title_bottom("j/k to scroll");
        let rows: Vec<Row> = state
            .pokedex
            .pokemon_entries
            .iter()
            .map(|entry| {
                Row::new(vec![
                    format!("#{:0>4}", entry.entry_number),
                    entry.pokemon_species.name.clone(),
                ])
            })
            .collect();
        let widths = [Constraint::Length(5), Constraint::Fill(1)];
        let table = Table::new(rows, widths)
            .block(block)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .highlight_symbol(">>")
            .row_highlight_style(Style::new().black().on_blue());
        StatefulWidget::render(table, area, buf, &mut state.table_state);
    }
}

impl PokedexListWidget {
    fn run(&self) {
        let this = self.clone();
        tokio::spawn(this.fetch_dex());
    }

    async fn fetch_dex(self) {
        let rustemon_client = rustemon::client::RustemonClient::default();
        self.set_loading_state(LoadingState::Loading);
        let id = self.state.read().unwrap().dex_id;
        match rustemon::games::pokedex::get_by_id(id, &rustemon_client).await {
            Ok(dex) => self.on_load(dex),
            Err(err) => self.on_err(err),
        }
    }
    fn new(id: i64) -> Self {
        Self {
            state: Arc::new(RwLock::new(PokedexListState::new(id))),
        }
    }

    fn set_dex(&self, id: i64) {
        self.set_loading_state(LoadingState::Loading);
        self.state.write().unwrap().dex_id = id;
    }
    fn set_loading_state(&self, state: LoadingState) {
        self.state.write().unwrap().loading_state = state;
    }

    fn on_err(&self, err: Error) {
        self.set_loading_state(LoadingState::Error(err.to_string()));
    }

    fn on_load(&self, dex: Pokedex) {
        self.set_loading_state(LoadingState::Loaded);
        let mut state = self.state.write().unwrap();
        state.pokedex = dex;
        state.table_state.select(Some(0));
    }

    pub fn scroll_down(&self) -> Option<String>{
        let mut state =self.state.write().unwrap();
        state.table_state.scroll_down_by(1);
        match state.table_state.selected()
        {
            Some(index)=> {return Some(state.pokedex.pokemon_entries[index].pokemon_species.name.clone())}
            None=> {return  None;}
        }
    }
    pub fn scroll_up(&self) -> Option<String>{
        let mut state =self.state.write().unwrap();
        state.table_state.scroll_up_by(1);
        match state.table_state.selected()
        {
            Some(index)=> {return Some(state.pokedex.pokemon_entries[index].pokemon_species.name.clone())}
            None=> {return  None;}
        }
    }
}

#[derive(Debug)]
struct PokedexListState {
    dex_id: i64,
    loading_state: LoadingState,
    pokedex: Pokedex,
    table_state: TableState,
}

impl PokedexListState {
    pub fn new(id: i64) -> Self {
        Self {
            dex_id: id,
            loading_state: LoadingState::default(),
            pokedex: Pokedex::default(),
            table_state: TableState::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct PokedexPokemonWidget {
    state: Arc<RwLock<PokedexPokemonState>>,
}

impl PokedexPokemonWidget {
    fn run(&self) {
        let this = self.clone();
        tokio::spawn(this.fetch_mon());
    }

    async fn fetch_mon(self) {
        let rustemon_client = rustemon::client::RustemonClient::default();
        self.set_loading_state(LoadingState::Loading);
        let name = self.state.read().unwrap().name.clone();
        match rustemon::pokemon::pokemon::get_by_name(name.as_str(), &rustemon_client).await {
            Ok(mon) => self.on_load(mon),
            Err(err) => self.on_err(err),
        }
    }

    fn set_mon(&self,name: String)
    {
        self.set_loading_state(LoadingState::Loading);
        self.state.write().unwrap().name = name;
        self.run();
    }

    fn set_loading_state(&self, state: LoadingState) {
        self.state.write().unwrap().loading_state = state;
    }

    fn on_err(&self, err: Error) {
        self.set_loading_state(LoadingState::Error(err.to_string()));
    }

    fn on_load(&self, mon: Pokemon) {
        self.set_loading_state(LoadingState::Loaded);
        let mut state = self.state.write().unwrap();
        state.pokemon = Some(mon);
    }
}

impl Widget for &PokedexPokemonWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let state = self.state.read().unwrap();
        let loading_state = Line::from(format!("{:?}", state.loading_state));
        let block = Block::bordered()
            .title(state.name.clone())
            .title(loading_state);
        match &state.pokemon {
            None=> {block.render(area, buf);}
            Some(mon)=>
            {
                let lines : Vec<Line> = mon.stats.iter().map(|stat| Line::from(format!("{:}:{:}", stat.stat.name, stat.base_stat))).collect();
                let paragraph = Paragraph::new(lines).block(block);
                paragraph.render(area, buf);
            }
        }
    }
}

#[derive(Debug)]
#[derive(Default)]
struct PokedexPokemonState {
    name: String,
    pokemon: Option<Pokemon>,
    loading_state: LoadingState,
}
