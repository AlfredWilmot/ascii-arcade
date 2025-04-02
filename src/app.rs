use std::sync::mpsc::Receiver;

use strum::EnumCount;
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
    pub fn update(&mut self, rx: &Receiver<Event>) {
        let usr_input: Event;
        // block updating the main-menu between user-input events,
        // much easier on the cpu than a rx.try_rec() + thread::sleep()
        if let Ok(event) = rx.recv() {
            usr_input = event;
        } else {
            return;
        }

        // get the command based on the app state and user-input
        let new_state: Option<State> = match self.state {
            // controlling main menu if no game is at play.
            State::MenuSelection(game) => match MainMenu::parse_event(usr_input) {
                Cmd::SELECT => Some(State::Playing(game)),
                Cmd::UP => {
                    let mut new_game = game as usize;
                    if new_game == 0 {
                        new_game = 0;
                    } else {
                        new_game -= 1;
                    }
                    // select the new game, default to the top of the list
                    let result = State::MenuSelection(
                        Game::from_repr(new_game).unwrap_or(Game::from_repr(0).expect("ERROR")),
                    );
                    Some(result)
                }
                Cmd::DOWN => {
                    let mut new_game = game as usize;
                    if new_game >= Game::COUNT {
                        new_game = Game::COUNT;
                    } else {
                        new_game += 1;
                    }
                    // select the new game, default to the bottom of the list
                    let result = State::MenuSelection(
                        Game::from_repr(new_game)
                            .unwrap_or(Game::from_repr(Game::COUNT - 1).expect("ERROR")),
                    );
                    Some(result)
                }

                Cmd::EXIT => Some(State::Exit),
                _ => None,
            },
            State::Playing(game) => {
                let game_done = match game {
                    Game::Sandbox => SandboxGame::play(rx),
                    _ => Cmd::RETURN,
                };

                match game_done {
                    Cmd::RETURN => Some(State::MenuSelection(game)),
                    Cmd::EXIT => Some(State::Exit),
                    _ => None,
                }
            }
            _ => Some(State::Exit),
        };

        // update the app state if the new_state is valid
        if let Some(state) = new_state {
            self.state = state;
        }
    }
}
