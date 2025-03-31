use strum::EnumCount;
use termion::event::Event;

use crate::{games::Game, ui::MainMenu, user_input::Cmd};

pub enum Mode {
    Default,
    Debug,
}

#[derive(Clone)]
pub enum State {
    Playing(Game),
    MenuSelection(Game),
    Exit,
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
    pub fn update(&mut self, usr_input: Event) -> State {
        // get the command based on the app state and user-input
        self.state = match self.state {
            // controlling main menu if no game is at play.
            State::MenuSelection(game) => match MainMenu::parse_event(usr_input) {
                Cmd::SELECT => State::Playing(game),
                Cmd::UP => {
                    let mut new_game = game as usize;
                    if new_game == 0 {
                        new_game = 0;
                    } else {
                        new_game -= 1;
                    }
                    // select the new game, default to the top of the list
                    State::MenuSelection(
                        Game::from_repr(new_game).unwrap_or(Game::from_repr(0).expect("ERROR")),
                    )
                }
                Cmd::DOWN => {
                    let mut new_game = game as usize;
                    if new_game >= Game::COUNT {
                        new_game = Game::COUNT;
                    } else {
                        new_game += 1;
                    }
                    // select the new game, default to the bottom of the list
                    State::MenuSelection(
                        Game::from_repr(new_game)
                            .unwrap_or(Game::from_repr(Game::COUNT - 1).expect("ERROR")),
                    )
                }

                Cmd::EXIT => State::Exit,
                _ => self.state.clone(),
            },
            State::Playing(game) => match MainMenu::parse_event(usr_input) {
                Cmd::RETURN => State::MenuSelection(game),
                Cmd::EXIT => State::Exit,
                _ => self.state.clone(),
            },
            _ => State::Exit,
        };
        // return the resulting state
        self.state.clone()
    }
}
