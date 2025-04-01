use std::{
    cmp::min,
    sync::{Arc, RwLock},
};

use rustemon::model::pokemon::{PokemonAbility, PokemonMove};
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;
use tui_widget_list::ListState;

use crate::events::{
    AppEvent, Event,
    navigation::{NavDirection, Navigation},
};

use super::monmove::MoveWidget;

#[derive(Debug, Clone)]
pub struct MovesWidget {
    sender: UnboundedSender<Event>,
    pub state: Arc<RwLock<MovesState>>,
}

impl MovesWidget {
    pub fn new(sender: UnboundedSender<Event>) -> Self {
        Self {
            sender,
            state: Default::default(),
        }
    }

    fn load(&self) {
        let state = self.state.read().unwrap();
        let start = state.list_state.selected.unwrap_or(0);
        let end = usize::min(start + 15, state.widgets.len());
        for ele in state.widgets[start..end].iter() {
            ele.load(state.cancelation_token.child_token());
        }
    }

    pub fn set_moves(&self, moves: Vec<PokemonMove>) {
        {
            let mut state = self.state.write().unwrap();
            state.cancelation_token.cancel();
            state.cancelation_token = CancellationToken::new();
            state.widgets.clear();
            state.list_state = ListState::default();
            for move_ in moves {
                state
                    .widgets
                    .push(MoveWidget::new(self.sender.clone(), move_));
            }
            state.list_state.select(Some(0));
        }
        self.load();
    }
}

#[derive(Debug, Default)]
pub struct MovesState {
    focused: bool,
    cancelation_token : CancellationToken,
    pub widgets: Vec<MoveWidget>,
    pub list_state: ListState,
}

impl MovesState {
    pub fn focused(&self) -> bool {
        self.focused
    }
}

impl Navigation for &MovesWidget {
    fn handle_navigation_input(self, direction: NavDirection) -> bool {
        let consumed = match direction {
            NavDirection::Up => {
                self.state.write().unwrap().list_state.previous();
                self.load();
                true
            }
            NavDirection::Down => {
                self.state.write().unwrap().list_state.next();
                self.load();
                true
            }
            _ => false,
        };
        let _ = self.sender.send(Event::App(AppEvent::Redraw));
        consumed
    }

    fn focus(self) {
        self.state.write().unwrap().focused = true;
        self.state.write().unwrap().list_state.select(Some(0));
        let _ = self.sender.send(Event::App(AppEvent::Redraw));
    }

    fn unfocus(self) {
        self.state.write().unwrap().focused = false;
        self.state.write().unwrap().list_state.select(None);

        let _ = self.sender.send(Event::App(AppEvent::Redraw));
    }
}
