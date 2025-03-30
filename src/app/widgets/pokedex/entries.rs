use std::sync::{Arc, RwLock};

use ratatui::widgets::TableState;
use rustemon::model::games::PokemonEntry;
use tokio::sync::mpsc::UnboundedSender;

use crate::{events::{navigation::{NavDirection, Navigation}, AppEvent, Event}, pokemon::PokemonName};

#[derive(Debug, Clone)]
pub struct EntriesWidget {
    sender: UnboundedSender<Event>,
    pub state: Arc<RwLock<EntriesState>>,
}

impl EntriesWidget {

    fn select(&self, index: Option<usize>) -> Option<PokemonName> {
        let mut state = self.state.write().unwrap();
        state.table_state.select(index);
        match state.table_state.selected() {
            Some(index) => match state.entries.get(index) {
                Some(entry) => return Some(entry.pokemon_species.name.clone()),
                None => return None,
            },
            None => return None,
        }
    }

    pub fn get_selectected(&self) -> Option<PokemonName> {
        let state = self.state.read().unwrap();
        match state.table_state.selected() {
            Some(index) => match state.entries.get(index) {
                Some(entry) => return Some(entry.pokemon_species.name.clone()),
                None => return None,
            },
            None => return None,
        }
    }

    pub fn set_entries(&self, entries: &Vec<PokemonEntry>) -> Option<PokemonName>{
        {
            let mut state = self.state.write().unwrap();
            state.entries = entries.clone();
        }
        self.select(Some(0))
    }

    pub fn new(sender: UnboundedSender<Event>) -> Self {
        Self {
            sender,
            state: Default::default(),
        }
    }

}

#[derive(Debug, Default)]
pub struct EntriesState {
    focused : bool,
    entries: Vec<PokemonEntry>,
    table_state: TableState,
}

impl EntriesState {
    pub fn entries(&self) -> &[PokemonEntry] {
        &self.entries
    }

    pub fn table_state(&mut self) -> &mut TableState {
        &mut self.table_state
    }
    
    pub fn focused(&self) -> bool {
        self.focused
    }
}


impl Navigation for &EntriesWidget
{
    fn handle_navigation_input(self, direction: NavDirection)-> bool
    {
        match direction {
            NavDirection::Up => {
                let this = &self;
                let mut state = this.state.write().unwrap();
                state.table_state.scroll_up_by(1);
                let _ = this.sender.send(Event::App(AppEvent::Redraw));
                true
            },
            NavDirection::Down => {
                let this = &self;
                let mut state = this.state.write().unwrap();
                state.table_state.scroll_down_by(1);
                let _ = this.sender.send(Event::App(AppEvent::Redraw));
                true
            },
            _ => false,
        }

    }
    
    fn focus(self) {
        self.state.write().unwrap().focused= true;
        let _ = self.sender.send(Event::App(AppEvent::Redraw));

    }
    
    fn unfocus(self) {
        self.state.write().unwrap().focused= false;
        let _ = self.sender.send(Event::App(AppEvent::Redraw));

    }
    

    

}


