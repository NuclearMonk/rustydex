use std::{fmt, sync::{Arc, RwLock}};

use ratatui::style::Style;
use rustemon::{error::Error, model::{moves::Move, pokemon::PokemonMove}, Follow};
use tokio::{select, sync::mpsc::UnboundedSender};
use tokio_util::sync::CancellationToken;

use crate::{events::{AppEvent, Event}, pokemon::get_client};


#[derive(Debug, Clone, Default)]
pub enum LoadingState
{
    #[default]
    Idle,
    Lazy(PokemonMove),
    Loading(PokemonMove),
    Loaded(Move),
    Error(String)
}

impl fmt::Display for LoadingState
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self
        {
            LoadingState::Idle => write!(f, "Unset"),
            LoadingState::Lazy(pokemon_move) => write!(f, "Lazy {0}", pokemon_move.move_.name),
            LoadingState::Loading(pokemon_move) => write!(f, "Loading {0}", pokemon_move.move_.name),
            LoadingState::Loaded(move_) => write!(f, "Loaded {0}", move_.name),
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
        self.set_loading_state(LoadingState::Loading(move_.clone()));

        //self.set_loading_state(LoadingState::Loading);
        match move_.move_.follow(&get_client()).await {
            Ok(move_) => self.on_load(move_),
            Err(err) => self.on_err(err),
        }
    }

    pub fn load(&self, cancellation_token: CancellationToken)
    {
        let state = self.state.read().unwrap();
        match state.loading_state.clone(){
            LoadingState::Lazy(pokemon_move) => {
                let this = self.clone();
                tokio::spawn(this.cancelable_fetch(pokemon_move, cancellation_token));
            },
            _ => {}
        }
    }

    async fn cancelable_fetch(self, move_ :PokemonMove , token: CancellationToken)
    {
        
        // self.set_loading_state(LoadingState::Loading(name.clone(), token));
        select! {
            _= token.cancelled()=> {}
            _= self.fetch(move_)=>{}

        }
    }
    // pub fn set_move(&self, move_: PokemonMove) {
    //     let this = self.clone();
    //     tokio::spawn(this.fetch(move_));
    // }

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
            LoadingState::Loading(pokmon_move) => 
            {
                if move_.name == pokmon_move.move_.name
                {
                    state.loading_state = LoadingState::Loaded(move_);
                    let _ = self.sender.send(Event::App(AppEvent::Redraw));

                }
            } ,
            _ => {}
        }

    }

    pub fn new(sender: UnboundedSender<Event>, move_: PokemonMove) -> Self {
        Self {
            sender: sender.clone(),
            style: Default::default(),
            state: Arc::new(RwLock::new(MoveState{loading_state: LoadingState::Lazy(move_)}))}
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
