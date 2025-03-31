use std::{
    default, fmt,
    sync::{Arc, RwLock},
};

use ratatui::widgets::TableState;
use rustemon::{error::Error, model::pokemon::Pokemon};
use tokio::sync::mpsc::UnboundedSender;

use crate::{events::navigation::NavDirection, pokemon::get_client};
use crate::{
    events::{AppEvent, Event, navigation::Navigation},
    pokemon::PokemonName,
};

use super::{abilities::AbilitiesWidget, moves::MovesWidget};

#[derive(Debug, Clone, Default)]
pub enum LoadingState {
    #[default]
    Idle,
    Loading(PokemonName),
    Loaded(Pokemon),
    Error(String),
}

impl fmt::Display for LoadingState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadingState::Idle => write!(f, "Idle"),
            LoadingState::Loading(name) => write!(f, "Loading {0}", name),
            LoadingState::Loaded(pokemon) => write!(f, "Loaded {0}", pokemon.name),
            LoadingState::Error(error) => write!(f, "Error {0}", error),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DetailsWidget {
    sender: UnboundedSender<Event>,
    pub abilities: AbilitiesWidget,
    pub moves : MovesWidget,
    pub state: Arc<RwLock<DetailsState>>,
}

impl DetailsWidget {
    async fn fetch_mon(self, name: String) {
        let rustemon_client = get_client();
        self.set_loading_state(LoadingState::Loading(name.clone()));

        //self.set_loading_state(LoadingState::Loading);
        match rustemon::pokemon::pokemon::get_by_name(name.as_str(), &rustemon_client).await {
            Ok(mon) => self.on_load(mon),
            Err(err) => self.on_err(err),
        }
    }

    pub fn set_mon(&self, name: PokemonName) {
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
        let mut state = self.state.write().unwrap();
        match state.loading_state.clone() {
            LoadingState::Loading(name) => {
                if name == mon.name {
                    self.abilities.set_abilities(mon.abilities.clone());
                    self.moves.set_moves(mon.moves.clone());
                    state.loading_state = LoadingState::Loaded(mon);
                    let _ = self.sender.send(Event::App(AppEvent::Redraw));
                }
            }
            _ => {}
        }
    }

    pub fn new(sender: UnboundedSender<Event>) -> Self {
        Self {
            sender: sender.clone(),
            abilities: AbilitiesWidget::new(sender.clone()),
            moves: MovesWidget::new(sender.clone()),
            state: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum DetailsFocus {
    #[default]
    Abilities,
    Moves,
}

#[derive(Debug, Default)]
pub struct DetailsState {
    focused: bool,
    current_focus: DetailsFocus,
    loading_state: LoadingState,
    pub ability_table_state: TableState,
}

impl DetailsState {
    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn loading_state(&self) -> &LoadingState {
        &self.loading_state
    }
}

impl Navigation for &DetailsWidget {
    fn handle_navigation_input(self, direction: NavDirection) -> bool {
        let mut state = self.state.write().unwrap();
        match (state.current_focus, direction) {
            (DetailsFocus::Abilities, NavDirection::Tab) => {
                self.abilities.unfocus();
                state.current_focus = DetailsFocus::Moves;
                self.moves.focus();

                true
            }
            (DetailsFocus::Abilities, NavDirection::BackTab) => false,
            (DetailsFocus::Abilities, direction) => {
                self.abilities.handle_navigation_input(direction)
            }
            (DetailsFocus::Moves, NavDirection::Up | NavDirection::Down) => self.moves.handle_navigation_input(direction), //todo
            (DetailsFocus::Moves, NavDirection::Tab) => false,
            (DetailsFocus::Moves, NavDirection::BackTab) => {
                self.moves.unfocus();
                state.current_focus = DetailsFocus::Abilities;
                self.abilities.focus();
                true
            }
        }
    }

    fn focus(self) {
        self.state.write().unwrap().focused = true;
        self.state.write().unwrap().current_focus = DetailsFocus::Abilities;
        self.abilities.focus();
        let _ = self.sender.send(Event::App(AppEvent::Redraw));
    }

    fn unfocus(self) {
        self.abilities.unfocus();
        self.state.write().unwrap().focused= false;
        let _ = self.sender.send(Event::App(AppEvent::Redraw));
    }
}
