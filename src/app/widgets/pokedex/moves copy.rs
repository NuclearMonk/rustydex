use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use rustemon::{
    error::Error,
    model::{moves::Move, pokemon::PokemonMove},
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tokio_stream::{StreamExt, wrappers::UnboundedReceiverStream};
use tui_widget_list::ListState;

use crate::{
    events::{
        AppEvent, Event,
        navigation::{NavDirection, Navigation},
    },
    pokemon::{MoveName, get_client},
};

use super::monmove::MoveWidget;

#[derive(Debug, Clone)]
pub struct MovesWidget {
    sender: UnboundedSender<Event>,
    background_sender: Option<UnboundedSender<String>>,
    pub state: Arc<RwLock<MovesState>>,
}

impl MovesWidget {
    pub fn new(sender: UnboundedSender<Event>) -> Self {
        Self {
            sender,
            background_sender: None,
            state: Default::default(),
        }
    }

    fn start_fetch_worker(
        recv: UnboundedReceiver<MoveName>,
    ) -> UnboundedReceiver<Result<Move, Error>> {
        let (s, r) = unbounded_channel();
        tokio::spawn(Self::fetch_thread(recv, s));
        r
    }

    pub fn run(&self){
        let this = self.clone();
        tokio::spawn( this.main_thread());
    }

    async fn main_thread(&self) {
        let (background_sender, background_reciever) = unbounded_channel();
        let main_receiver = Self::start_fetch_worker(background_reciever);
        let mut stream = UnboundedReceiverStream::new(main_receiver);
        loop {
            tokio::select! {
                    _ = background_sender.closed() => {
                        break;
                    }
                    Some(move_) = stream.next()=> {
                        match move_
                        {
                            Ok(move_) => self.load_move(move_),
                            Err(_) => {},
                        }
                }
            }
        }
    }

    async fn fetch_thread(
        recv: UnboundedReceiver<MoveName>,
        sender: UnboundedSender<Result<Move, Error>>,
    ) {
        let rustemon_client = get_client();
        let mut stream = UnboundedReceiverStream::new(recv);
        loop {
            tokio::select! {
                    _ = sender.closed() => {
                        break;
                    }
                    Some(name) = stream.next()=> {
                        let _ = sender.send(rustemon::moves::move_::get_by_name(&name, &rustemon_client).await);
                    }
            }
        }
    }

    fn fetch(&self, name: MoveName) {
        match self.background_sender.clone() {
            Some(sender) => {let _ = sender.send(name);},
            None => {}
        };
    }

    pub fn set_moves(&self, moves: Vec<PokemonMove>) {
        let mut state = self.state.write().unwrap();
        state.widgets.clear();
        for move_ in moves {
            let name = move_.move_.name.to_string();
            state
                .widgets
                .insert(name.clone(),  MoveWidget::new(move_));
            self.fetch(name)
        }
    }

    fn load_move(&self, move_: Move) {
        let mut state = self.state.write().unwrap();
        match state.widgets.get_mut(&move_.name) {
            Some(widget) => widget.set_move(move_),
            None => {}
        }
    }
}

#[derive(Debug, Default)]
pub struct MovesState {
    focused: bool,
    pub widgets: HashMap<String, MoveWidget>,
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
                // self.load();
                true
            }
            NavDirection::Down => {
                self.state.write().unwrap().list_state.next();
                // self.load();
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
