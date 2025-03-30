use termion::event::Event;

use crate::{
    games::{Game, SandboxGame},
    ui::MainMenu,
    user_input::Cmd,
};

pub enum Mode {
    Default,
    Debug,
}

pub enum State {
    Playing(Game),
    MenuSelection(Game),
}

/// Represents the state of the TUI application (manages persistent data)
pub struct App {
    pub state: State, // indicates the current state of the app
    pub mode: Mode,   // determines the mode to run the app in
}

impl App {
    /// create a new App instance.
    pub fn new(mode: Mode) -> App {
        App {
            state: State::MenuSelection(Game::Sandbox),
            mode,
        }
    }
    /// update the state of the app based on user input and current state.
    pub fn update(&mut self, usr_input: Event) -> Cmd {
        // get the command based on the app state and user-input
        match &self.state {
            // controlling the game that is being played.
            State::Playing(game) => match game {
                Game::Sandbox => SandboxGame::parse_event(usr_input),
                Game::Pong => Cmd::EXIT,
            },

            // controlling main menu if no game is at play.
            State::MenuSelection(_) => MainMenu::parse_event(usr_input),
        }
    }
}
