use ascii_arcade::{
    games::{Game, MainMenu, SandboxGame},
    scene,
    user_input::{self, Cmd},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use termion::event::Event;

/// welcome screen text (NOTE: trailing whitespace is necesary for centering text)
const WELCOME: &str = r#"
    _             _ _    _                      _     
   / \   ___  ___(_|_)  / \   _ __ ___ __ _  __| | ___
  / _ \ / __|/ __| | | / _ \ | '__/ __/ _` |/ _` |/ _ \
 / ___ \\__ \ (__| | |/ ___ \| | | (_| (_| | (_| |  __/
/_/   \_\___/\___|_|_/_/   \_\_|  \___\__,_|\__,_|\___|

(press 'q' to exit)"#;

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
    // create a border around the entire viewport
    let outer_border = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::new());
    frame.render_widget(outer_border.white(), frame.area());

    match app.state {
        State::MenuSelection(_game) => {
            render_menu(frame);
        }
        State::Playing(_game) => {}
    }
}

fn render_menu(frame: &mut Frame) {
    // split the menu into two halves horizontally
    let line_count: u16 = WELCOME.split('\n').count() as u16;

    let menu = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(line_count), Constraint::Fill(1)]);

    let [header, footer] = menu.areas(frame.area());

    // fill the header with the WELCOME text
    let header_text = Paragraph::new(WELCOME.to_string()).centered();
    frame.render_widget(header_text, header);

    // create a pop-up for game-selection
    let game_selection = Block::default()
        .title(Line::from("< Select [ ↑↓/jk ] >").centered())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let footer_regions = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Max(31),
            Constraint::Fill(1),
        ])
        .margin(1);

    let [_, selection_area, _] = footer_regions.areas(footer);
    frame.render_widget(game_selection, selection_area);

    const GAME_COUNT: usize = 2;

    let game_options = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3); GAME_COUNT])
        .margin(2);

    let games: [Rect; GAME_COUNT] = game_options.areas(selection_area);

    let opt: usize = 0;
    for (id, game_opt) in games.into_iter().enumerate() {
        let selected_text = Paragraph::new(Line::default().spans(vec![
            "[↵]".light_green().bold(),
            format!(" Game_{}", id).black(),
        ]))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default())
                .on_dark_gray()
                .black(),
        );
        let text = Paragraph::new(Line::from(format!("  Game_{}", id)).left_aligned()).block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default()),
        );

        if opt == id {
            frame.render_widget(selected_text, game_opt);
        } else {
            frame.render_widget(text, game_opt);
        }
    }
}

fn main() {
    let mut terminal = scene::init().expect("ERROR: could not setup terminal!");
    let rx = user_input::create_data_channel();
    let mut app = App::new(Mode::Default);

    'menu: loop {
        terminal
            .draw(|frame| {
                ui(frame, &app);
            })
            .expect("ERROR: could not draw frame!");

        // block updating the main-menu between user-input events,
        // much easier on the cpu than a rx.try_rec() + thread::sleep()
        if let Ok(event) = rx.recv() {
            if let Cmd::EXIT = app.update(event) {
                break 'menu;
            };
        }
    }

    scene::close(terminal);
}
