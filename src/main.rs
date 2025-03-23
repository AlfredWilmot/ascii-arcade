use ascii_arcade::{
    games::{Game, MainMenu, SandboxGame},
    scene,
    user_input::{self, Cmd},
};
use ratatui::{
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use termion::event::Event;

const WELCOME: &str = r#"
    _             _ _    _                      _
   / \   ___  ___(_|_)  / \   _ __ ___ __ _  __| | ___
  / _ \ / __|/ __| | | / _ \ | '__/ __/ _` |/ _` |/ _ \
 / ___ \\__ \ (__| | |/ ___ \| | | (_| (_| | (_| |  __/
/_/   \_\___/\___|_|_/_/   \_\_|  \___\__,_|\__,_|\___|

(press 'q' to exit)

"#;

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
        let cmd = match &self.state {
            // controlling the game that is being played.
            State::Playing(game) => match game {
                Game::Sandbox => SandboxGame::parse_event(usr_input),
                Game::Pong => Cmd::EXIT,
            },

            // controlling main menu if no game is at play.
            State::MenuSelection(_) => MainMenu::parse_event(usr_input),
        };

        // update the app state based on the generated command
        match &self.state {
            State::Playing(_) => match cmd {
                Cmd::RETURN => {
                    self.state = State::MenuSelection(Game::Sandbox);
                    cmd
                }
                _ => cmd,
            },
            State::MenuSelection(game) => match cmd {
                Cmd::SELECT => {
                    self.state = State::Playing(*game);
                    cmd
                }
                _ => cmd,
            },
        }
    }
}

/// Generate and render a fame based on the current state of the app.
pub fn ui(frame: &mut Frame, app: &App) {
    let background = Block::default().borders(Borders::ALL).style(Style::new());
    let greeting = Paragraph::new(WELCOME.to_string()).block(background);
    frame.render_widget(greeting, frame.area());
    let _ = app;
}

fn main() {
    let mut terminal = scene::init().unwrap();
    let rx = user_input::create_data_channel();
    let mut app = App::new(Mode::Default);

    'menu: loop {
        terminal
            .draw(|frame| {
                ui(frame, &app);
            })
            .unwrap();

        if let Ok(event) = rx.try_recv() {
            if let Cmd::EXIT = app.update(event) {
                break 'menu;
            };
        }
    }

    scene::close(terminal);
}
