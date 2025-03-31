use std::{fmt, sync::{Arc, RwLock}};

use ratatui::style::Style;
use rustemon::{error::Error, model::{moves::Move, pokemon::PokemonMove}};
use tokio::sync::mpsc::UnboundedSender;

use crate::{events::{AppEvent, Event}, pokemon::{get_client, MoveName}};


#[derive(Debug, Clone, Default)]
pub enum LoadingState
{
    #[default]
    Idle,
    Loading(MoveName),
    Loaded(Move),
    Error(String)
}

impl fmt::Display for LoadingState
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self
        {
            LoadingState::Idle => write!(f, "Unset"),
            LoadingState::Loading(name) => write!(f, "Loading {0}", name),
            LoadingState::Loaded(ability) => write!(f, "Loaded {0}", ability.name),
            LoadingState::Error(error) => write!(f, "Error {0}", error),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveWidget {
    pub style: Style,
    sender: UnboundedSender<Event>,
    pub state: Arc<RwLock<MoveState>>,
}

impl MoveWidget {
    async fn fetch(self, move_: PokemonMove) {
        let rustemon_client = get_client();
        self.set_loading_state(LoadingState::Loading(move_.move_.name.clone()));

        //self.set_loading_state(LoadingState::Loading);
        match rustemon::moves::move_::get_by_name(&move_.move_.name, &rustemon_client).await {
            Ok(move_) => self.on_load(move_),
            Err(err) => self.on_err(err),
        }
    }

    pub fn set_move(&self, move_: PokemonMove) {
        let this = self.clone();
        tokio::spawn(this.fetch(move_));
    }

    fn set_loading_state(&self, state: LoadingState) {
        self.state.write().unwrap().loading_state = state;
    }

    fn on_err(&self, err: Error) {
        self.set_loading_state(LoadingState::Error(err.to_string()));
    }

    fn on_load(&self, move_: Move) {
        let mut state = self.state.write().unwrap();
        match state.loading_state.clone()
        {
            LoadingState::Loading(name) => 
            {
                if name == move_.name
                {
                    state.loading_state = LoadingState::Loaded(move_);
                    let _ = self.sender.send(Event::App(AppEvent::Redraw));

                }
            } ,
            _ => {}
        }

    }

    pub fn new(sender: UnboundedSender<Event>, move_: PokemonMove) -> Self {
        let s = Self {
            sender: sender.clone(),
            style: Default::default(),
            state: Default::default()
        };
        s.set_move(move_);
        s

    }

}

#[derive(Debug, Default)]
pub struct MoveState {
    loading_state: LoadingState,
}

impl MoveState {


    pub fn loading_state(&self) -> &LoadingState {
        &self.loading_state
    }

    
    
}
