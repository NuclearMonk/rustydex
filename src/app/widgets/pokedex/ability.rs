use std::{default, fmt, sync::{Arc, RwLock}};

use ratatui::style::Style;
use rustemon::{error::Error, model::pokemon::{Ability, PokemonAbility}, pokemon::ability, Follow};
use tokio::sync::mpsc::UnboundedSender;

use crate::{events::{AppEvent, Event}, pokemon::{get_client, AbilityName}};


#[derive(Debug, Clone, Default)]
pub enum LoadingState
{
    #[default]
    Idle,
    Loading(PokemonAbility),
    Loaded(Ability),
    Error(String)
}

impl fmt::Display for LoadingState
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self
        {
            LoadingState::Idle => write!(f, "Unset"),
            LoadingState::Loading(ability) => write!(f, "Loading {0}", ability.ability.name),
            LoadingState::Loaded(ability) => write!(f, "Loaded {0}", ability.name),
            LoadingState::Error(error) => write!(f, "Error {0}", error),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AbilityWidget {
    pub style: Style,
    sender: UnboundedSender<Event>,
    pub state: Arc<RwLock<AbilityState>>,
}

impl AbilityWidget {
    async fn fetch(self, ability: PokemonAbility) {
        self.set_loading_state(LoadingState::Loading(ability.clone()));

        //self.set_loading_state(LoadingState::Loading);
        match ability.ability.follow( &get_client()).await {
            Ok(ability) => self.on_load(ability),
            Err(err) => self.on_err(err),
        }
    }

    pub fn set_ability(&self, ability: PokemonAbility) {
        self.state.write().unwrap().hidden = ability.is_hidden;
        let this = self.clone();
        tokio::spawn(this.fetch(ability));
    }

    fn set_loading_state(&self, state: LoadingState) {
        self.state.write().unwrap().loading_state = state;
    }

    fn on_err(&self, err: Error) {
        self.set_loading_state(LoadingState::Error(err.to_string()));
    }

    fn on_load(&self, ability: Ability) {
        let mut state = self.state.write().unwrap();
        match state.loading_state.clone()
        {
            LoadingState::Loading(pa) => 
            {
                if pa.ability.name == ability.name
                {
                    state.loading_state = LoadingState::Loaded(ability);
                    let _ = self.sender.send(Event::App(AppEvent::Redraw));

                }
            } ,
            _ => {}
        }

    }

    pub fn new(sender: UnboundedSender<Event>, ability: PokemonAbility) -> Self {
        let s = Self {
            sender: sender.clone(),
            style: Default::default(),
            state: Default::default()
        };
        s.set_ability(ability);
        s

    }

}

#[derive(Debug, Default)]
pub struct AbilityState {
    hidden: bool,
    loading_state: LoadingState,
}

impl AbilityState {


    pub fn loading_state(&self) -> &LoadingState {
        &self.loading_state
    }
    
    pub fn hidden(&self) -> bool {
        self.hidden
    }
    
    
}
