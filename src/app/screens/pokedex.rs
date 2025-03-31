use std::{fmt, sync::{Arc, RwLock}};

use rustemon::{error::Error, model::games::Pokedex};
use tokio::sync::mpsc::UnboundedSender;

use crate::{app::widgets::pokedex::{detail::DetailsWidget, entries::EntriesWidget}, events::{navigation::{NavDirection, Navigation}, AppEvent, Event}, pokemon::get_client};



#[derive(Debug)]
pub enum LoadingState
{
    Loading(String),
    Loaded(Pokedex),
    Error(String)
}

impl fmt::Display for LoadingState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            LoadingState::Loading(name) => write!(f, "Loading {0}", name),
            LoadingState::Loaded(dex) => write!(f, "Loaded {0}", dex.name),
            LoadingState::Error(error) => write!(f, "Error {0}", error),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PokedexScreen {
    pub sender: UnboundedSender<Event>,
    pub entries: EntriesWidget,
    pub detail_view: DetailsWidget,
    pub state: Arc<RwLock<PokedexState>>,
}

impl PokedexScreen {
    pub fn new(sender: UnboundedSender<Event>) -> Self {
        Self {
            sender: sender.clone(),
            state: Arc::new(RwLock::new(PokedexState::default())),
            entries: EntriesWidget::new(sender.clone()),
            detail_view: DetailsWidget::new(sender.clone()),
        }
    }

    pub fn run(&self) {
        let this = self.clone();
        tokio::spawn(this.fetch_dex("national".to_owned()));
    }
    pub fn set_dex(self, name: String) {
        let this = self.clone();
        tokio::spawn(this.fetch_dex(name));
    }

    async fn fetch_dex(self, name: String) {
        let rustemon_client = get_client();
        self.set_loading_state(LoadingState::Loading(name.clone()));
        match rustemon::games::pokedex::get_by_name(&name, &rustemon_client).await {
            Ok(dex) => self.set_loading_state(LoadingState::Loaded(dex)),
            Err(err) => self.on_err(err),
        }
    }

    fn set_loading_state(&self, state: LoadingState) {
        match state
        {
            LoadingState::Loading(_) =>  self.state.write().unwrap().loading_state = state,
            LoadingState::Loaded(dex) => {
                match self.entries.set_entries(&dex.pokemon_entries) { //This is Shite
                    Some(mon_name) => self.detail_view.set_mon(mon_name),
                    None => {}
                }
                self.state.write().unwrap().loading_state = LoadingState::Loaded(dex);
                let _ = self.sender.send(Event::App(AppEvent::Redraw));
                
            },
            _ => {},
        }
       
    }


    fn on_err(&self, err: Error) {
        self.set_loading_state(LoadingState::Error(err.to_string()));
    }

}

#[derive(Debug, Default, Clone, Copy)]
enum PokedexScreenFocus {
    #[default]
    List,
    Details,
}


#[derive(Debug)]
pub struct PokedexState {
    loading_state: LoadingState,
    current_focus: PokedexScreenFocus,
    focused: bool,
}

impl PokedexState {
    pub fn loading_state(&self) -> &LoadingState {
        &self.loading_state
    }
}

impl Default for PokedexState {
    fn default() -> Self {
        Self {
            loading_state: LoadingState::Loading(String::from("national")),
            current_focus: Default::default(),
            focused: Default::default()
        }
    }
}

impl Navigation for &PokedexScreen
{
    fn handle_navigation_input(self, direction: NavDirection)-> bool
    {
        let mut state = self.state.write().unwrap();
        let used = match state.current_focus {
            PokedexScreenFocus::List => {
                let used = self.entries.handle_navigation_input(direction);
                if used {
                    match self.entries.get_selectected() {
                        Some(mon_name) => self.detail_view.set_mon(mon_name),
                        None => {}
                    }
                }

                used
            }
            PokedexScreenFocus::Details => self
                .detail_view
                .handle_navigation_input(direction),
        };
        if used {
            return true;
        };
        match (direction, state.current_focus) {
            (NavDirection::Up | NavDirection::Down, _) => false,
            (NavDirection::Tab, PokedexScreenFocus::List) => {
                self.entries.unfocus();
                state.current_focus = PokedexScreenFocus::Details;
                self.detail_view.focus();
                true
            }
            (NavDirection::Tab, PokedexScreenFocus::Details) => false,
            (NavDirection::BackTab, PokedexScreenFocus::List) => false,
            (NavDirection::BackTab, PokedexScreenFocus::Details) => {
                self.detail_view.unfocus();
                state.current_focus = PokedexScreenFocus::List;
                self.entries.focus();
                true
            }
        }

    }
    
    fn focus(self) {
        let mut state = self.state.write().unwrap();
        state.focused= true;
        state.current_focus = PokedexScreenFocus::List;
        self.entries.focus();
        let _ = self.sender.send(Event::App(AppEvent::Redraw));

    }
    
    fn unfocus(self) {
        self.state.write().unwrap().focused= false;
        let _ = self.sender.send(Event::App(AppEvent::Redraw));

    }
    
}