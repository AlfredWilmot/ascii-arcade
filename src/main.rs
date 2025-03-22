use ascii_arcade::{
    scene,
    user_input::{self, menu_fsm, sandbox_game_fsm, Cmd},
};
use ratatui::widgets::Paragraph;
use termion::event::Event;

const WELCOME: &str = r#"
    _             _ _    _                      _
   / \   ___  ___(_|_)  / \   _ __ ___ __ _  __| | ___
  / _ \ / __|/ __| | | / _ \ | '__/ __/ _` |/ _` |/ _ \
 / ___ \\__ \ (__| | |/ ___ \| | | (_| (_| | (_| |  __/
/_/   \_\___/\___|_|_/_/   \_\_|  \___\__,_|\__,_|\___|

(press 'q' to exit)

"#;

// the different games the user can play
pub enum Game {
    SANDBOX,
    PONG,
}

pub enum Mode {
    Default,
    Debug,
}

pub enum State {
    Playing(Game),
    MenuSelection,
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
            state: State::MenuSelection,
            mode,
        }
    }
    /// update the state of the app based on user input and current state.
    pub fn update(&mut self, usr_input: Event) -> Cmd {
        match &self.state {
            // controlling the game that is being played.
            State::Playing(game) => match game {
                Game::SANDBOX => sandbox_game_fsm(usr_input),
                Game::PONG => Cmd::EXIT,
            },
            // controlling main menu if no game is at play.
            State::MenuSelection => menu_fsm(usr_input),
        }
    }
}

fn main() {
    let mut terminal = scene::init().unwrap();
    let rx = user_input::create_data_channel();
    let mut app = App::new(Mode::Default);

    'menu: loop {
        terminal
            .draw(|frame| {
                let greeting = Paragraph::new(WELCOME.to_string());
                frame.render_widget(greeting, frame.area());
            })
            .unwrap();

        if let Ok(event) = rx.try_recv() {
            match app.update(event) {
                Cmd::EXIT => break 'menu,
                _ => {} // TODO: implement the UI FSM that processes the CMD to generate the scene
            }
        }
    }

    scene::close(terminal);
}
