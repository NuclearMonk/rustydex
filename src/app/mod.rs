mod screens;
pub mod ui;
mod widgets;

use crate::events::{
    AppEvent, Event, EventHandler,
    navigation::{NavDirection, Navigation},
};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use screens::pokedex::PokedexScreen;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
pub enum CurrentScreen {
    Pokedex(PokedexScreen),
}

impl CurrentScreen {
    fn new(sender: UnboundedSender<Event>) -> Self {
        Self::Pokedex(PokedexScreen::new(sender))
    }
}
#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub events: EventHandler,
    pub current_screen: CurrentScreen,
}

impl Default for App {
    fn default() -> Self {
        let events = EventHandler::new();
        Self {
            should_quit: Default::default(),
            current_screen: CurrentScreen::new(events.sender.clone()),
            events,
        }
    }
}

impl App {
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        match &self.current_screen {
            CurrentScreen::Pokedex(dex) => dex.run(),
        }
        self.focus();
        while !self.should_quit {
            match self.events.next().await? {
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event),
                    _ => {
                        terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
                    }
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Redraw => {
                        terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
                    }
                    AppEvent::Quit => self.quit(),
                    AppEvent::Navigation(direction) => {
                        self.handle_navigation_input(direction);
                    }
                },
            }
        }
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.events.send(AppEvent::Navigation(NavDirection::Up))
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.events.send(AppEvent::Navigation(NavDirection::Down))
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => {
                self.events.send(AppEvent::Navigation(NavDirection::Tab))
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::BackTab=> self
                .events
                .send(AppEvent::Navigation(NavDirection::BackTab)),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }
}

impl Navigation for &App {
    fn handle_navigation_input(self, direction: NavDirection) -> bool {
        match &self.current_screen {
            CurrentScreen::Pokedex(pokedex_widget) => {
                pokedex_widget.handle_navigation_input(direction)
            }
        }
    }

    fn focus(self) {
        match &self.current_screen {
            CurrentScreen::Pokedex(pokedex_widget) => pokedex_widget.focus(),
        }
    }

    fn unfocus(self) {
        match &self.current_screen {
            CurrentScreen::Pokedex(pokedex_widget) => pokedex_widget.unfocus(),
        }
    }
}
