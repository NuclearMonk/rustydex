use ratatui::style::Style;
use rustemon::model::{moves::Move, pokemon::PokemonMove};

#[derive(Debug, Clone)]
pub enum LoadingState {
    Loading(PokemonMove),
    Loaded(Move),
}

#[derive(Debug, Clone)]
pub struct MoveWidget {
    style: Style,
    loading_state: LoadingState,
}

impl MoveWidget {
    pub fn new(move_: PokemonMove) -> Self {
        Self {
            style:Default::default(),
            loading_state: LoadingState::Loading(move_),
        }
    }

    pub fn set_move(&mut self, move_: Move) {
        self.loading_state = LoadingState::Loaded(move_);
    }
    
    pub fn loading_state(&self) -> &LoadingState {
        &self.loading_state
    }
    
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }
    
    pub fn style(&self) -> Style {
        self.style
    }
}
