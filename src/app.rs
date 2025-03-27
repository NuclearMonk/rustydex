use std::sync::{Arc, RwLock};

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer, layout::{Constraint, Rect}, style::{Style, Stylize}, widgets::{Block, Row, StatefulWidget, Table, TableState, Widget}, DefaultTerminal
};
use rustemon::{
    error::Error,
    model::{
        games::{Pokedex, PokemonEntry},
        pokemon::Pokemon,
    },
};
use tokio::sync::mpsc::UnboundedSender;

use crate::event::{AppEvent, Event, EventHandler};






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

impl CurrentScreen
{
    fn new(sender: UnboundedSender<Event>)-> Self
    {
        Self::Pokedex(PokedexWidget::new(sender))
    }
}
#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub events: EventHandler,
    pub current_screen: CurrentScreen,
}

impl Default for App {
    fn default() -> Self {
        let events = EventHandler::new();
        Self {
            should_quit: Default::default(),
            current_screen: CurrentScreen::new(events.sender.clone()),
            events
        }
    }
}

impl App {
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        match &self.current_screen {
            CurrentScreen::Pokedex(dex) => dex.run(),
        }

        while !self.should_quit {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Redraw=>{}
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::InputUp => self.scroll_up(),
                    AppEvent::InputDown => self.scroll_down(),
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }


    fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Up => self.events.send(AppEvent::InputUp),
            KeyCode::Down => self.events.send(AppEvent::InputDown),
            _ => {}
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.should_quit = true;
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

#[derive(Debug, Clone)]
pub struct PokedexWidget {
    pub sender : UnboundedSender<Event>,
    pub state: Arc<RwLock<PokedexState>>,
}

impl PokedexWidget {
    fn new(sender: UnboundedSender<Event>) ->Self{
        Self
        {
            sender: sender.clone(),
            state: Arc::new(RwLock::new(PokedexState::new(sender)))
        }
    }

    pub fn run(&self) {
        let this = self.clone();
        tokio::spawn(this.fetch_dex());
    }
    pub fn set_dex(self, id: i64) {
        self.state.write().unwrap().dex_id = id;
        let this = self.clone();
        tokio::spawn(this.fetch_dex());
    }

    async fn fetch_dex(self) {
        let rustemon_client = rustemon::client::RustemonClient::default();
        self.set_loading_state(LoadingState::Loading);
        let id = self.state.read().unwrap().dex_id;
        match rustemon::games::pokedex::get_by_id(id, &rustemon_client).await {
            Ok(dex) => self.on_load_dex(dex),
            Err(err) => self.on_err(err),
        }
    }

    fn set_loading_state(&self, state: LoadingState) {
        self.state.write().unwrap().loading_state = state;
    }

    fn on_load_dex(&self, dex: Pokedex) {
        self.set_loading_state(LoadingState::Loaded);
        let mut state = self.state.write().unwrap();
        state.pokedex = dex;
        state.list_widget.entries = state.pokedex.pokemon_entries.clone();
        match state.list_widget.select(Some(0)) {
            Some(mon_name) => state.pokemon_view_widget.set_mon(mon_name),
            None => {}
        }
        let _ = self.sender.send(Event::Redraw);
    }

    fn on_err(&self, err: Error) {
        self.set_loading_state(LoadingState::Error(err.to_string()));
    }

    fn scroll_up(&self) {
        let mut state = self.state.write().unwrap();
        let new_mon = state.list_widget.scroll_up();
        drop(state);
        match new_mon {
            Some(mon_name) => {
                let state = self.state.read().unwrap();
                state.pokemon_view_widget.set_mon(mon_name)
            }
            None => {}
        }
    }
    fn scroll_down(&self) {
        let mut state = self.state.write().unwrap();
        let new_mon = state.list_widget.scroll_down();
        drop(state);
        match new_mon {
            Some(mon_name) => {
                let state = self.state.read().unwrap();
                state.pokemon_view_widget.set_mon(mon_name)
            }
            None => {}
        }
    }
}

#[derive(Debug)]
pub struct PokedexState {
    dex_id: i64,
    pub loading_state: LoadingState,
    pokedex: Pokedex,
    pub list_widget: PokedexListWidget,
    pub pokemon_view_widget: PokedexViewWidget,
}



impl PokedexState
{
    fn new(sender: UnboundedSender<Event>)->Self
    {
        Self {
            dex_id: 1,
            loading_state: Default::default(),
            pokedex: Default::default(),
            list_widget: PokedexListWidget::new(sender.clone()),
            pokemon_view_widget: PokedexViewWidget::new(sender.clone()),
        }
    }
}


#[derive(Debug)]
pub struct PokedexListWidget {
    sender: UnboundedSender<Event>,
    pub entries: Vec<PokemonEntry>,
    pub table_state: TableState,
    pub query : String
}

impl PokedexListWidget {
    fn scroll_down(&mut self) -> Option<String> {
        self.table_state.scroll_down_by(1);
        match self.table_state.selected() {
            Some(index) => match self.entries.get(index) {
                Some(entry) => return Some(entry.pokemon_species.name.clone()),
                None => return None,
            },
            None => return None,
        }
    }
    fn scroll_up(&mut self) -> Option<String> {
        self.table_state.scroll_up_by(1);
        match self.table_state.selected() {
            Some(index) => match self.entries.get(index) {
                Some(entry) => return Some(entry.pokemon_species.name.clone()),
                None => return None,
            },
            None => return None,
        }
    }

    fn select(&mut self, index: Option<usize>) -> Option<String> {
        self.table_state.select(index);
        self.table_state.scroll_up_by(1);
        match self.table_state.selected() {
            Some(index) => match self.entries.get(index) {
                Some(entry) => return Some(entry.pokemon_species.name.clone()),
                None => return None,
            },
            None => return None,
        }
    }

    
    fn new(sender: UnboundedSender<Event>) -> Self {
        Self { sender, entries: Default::default(), table_state: Default::default(), query: String::default() }
    }
}



#[derive(Debug, Clone)]
pub struct PokedexViewWidget {
    sender : UnboundedSender<Event>,
    pub state: Arc<RwLock<PokedexViewState>>,
}

impl PokedexViewWidget {
    async fn fetch_mon(self, name: String) {
        let rustemon_client = rustemon::client::RustemonClient::default();
        //self.set_loading_state(LoadingState::Loading);
        match rustemon::pokemon::pokemon::get_by_name(name.as_str(), &rustemon_client).await {
            Ok(mon) => self.on_load(mon),
            Err(err) => self.on_err(err),
        }
    }

    fn set_mon(&self, name: String) {
        self.set_loading_state(LoadingState::Loading);
        let this = self.clone();
        tokio::spawn(this.fetch_mon(name));
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
        let _ = self.sender.send(Event::Redraw);
    }
    
    fn new(sender: UnboundedSender<Event>) -> Self {
        Self { sender, state: Default::default() }
    }
}

#[derive(Debug, Default)]
pub struct PokedexViewState {
    pub name: String,
    pub pokemon: Option<Pokemon>,
    pub loading_state: LoadingState,
    pub ability_table_state: TableState
}
