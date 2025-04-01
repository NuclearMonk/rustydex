use rustemon::model::{moves::Move, pokemon::PokemonMove};

#[derive(Debug, Clone)]
pub enum LoadingState {
    Loading(PokemonMove),
    Loaded(Move),
}

#[derive(Debug)]
pub struct MoveWidget {
    loading_state: LoadingState,
}

impl MoveWidget {
    pub fn new(move_: PokemonMove) -> Self {
        Self {
            loading_state: LoadingState::Loading(move_),
        }
    }

    pub fn set_move(&mut self, move_: Move) {
        self.loading_state = LoadingState::Loaded(move_);
    }
}
