use std::sync::{Arc, RwLock};

use rustemon::model::pokemon::PokemonAbility;
use tokio::sync::mpsc::UnboundedSender;
use tui_widget_list::ListState;

use crate::events::{navigation::{NavDirection, Navigation}, AppEvent, Event};

use super::ability::AbilityWidget;




#[derive(Debug, Clone)]
pub struct AbilitiesWidget {
    sender: UnboundedSender<Event>,
    pub state: Arc<RwLock<AbilitiesState>>,
}

impl AbilitiesWidget {
    pub fn new(sender: UnboundedSender<Event>) -> Self {
        Self {
            sender,
            state: Default::default(),
        }
    }

    pub fn set_abilities(&self, abilities: Vec<PokemonAbility>) {
        let mut state = self.state.write().unwrap();
        state.widgets.clear();
        for ability in abilities {
            state.widgets.push(AbilityWidget::new(self.sender.clone(),ability));
        }

    }
    

}



#[derive(Debug, Default)]
pub struct AbilitiesState {
    focused: bool,
    pub widgets: Vec<AbilityWidget>,
    pub list_state: ListState,
}

impl AbilitiesState {
    pub fn focused(&self) -> bool {
        self.focused
    }
}



impl Navigation for &AbilitiesWidget {
    fn handle_navigation_input(self, direction: NavDirection) -> bool {
        let consumed = match direction {
            NavDirection::Up => {self.state.write().unwrap().list_state.next(); true},
            NavDirection::Down => {self.state.write().unwrap().list_state.previous(); true},
            _ => false
        };
        let _ = self.sender.send(Event::App(AppEvent::Redraw));
        consumed
    }

    fn focus(self) {
        self.state.write().unwrap().focused = true;
        let _ = self.sender.send(Event::App(AppEvent::Redraw));

    }

    fn unfocus(self) {
        self.state.write().unwrap().focused = false;
        let _ = self.sender.send(Event::App(AppEvent::Redraw));
        
    }
}
