use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use termion::event::{Event, Key};

use crate::{
    app::{App, State},
    games::GAME_COUNT,
    user_input::Cmd,
};

/// welcome screen text (NOTE: trailing whitespace is necesary for centering text)
const WELCOME: &str = r#"
    _             _ _    _                      _     
   / \   ___  ___(_|_)  / \   _ __ ___ __ _  __| | ___
  / _ \ / __|/ __| | | / _ \ | '__/ __/ _` |/ _` |/ _ \
 / ___ \\__ \ (__| | |/ ___ \| | | (_| (_| | (_| |  __/
/_/   \_\___/\___|_|_/_/   \_\_|  \___\__,_|\__,_|\___|

(press 'q' to exit)"#;

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
            game_selection(frame);
        }
        State::Playing(_game) => {}
    }
}

/// Interface for the main menu.
pub struct MainMenu;

impl MainMenu {
    pub fn parse_event(event: Event) -> Cmd {
        match event {
            Event::Key(key) => match key {
                Key::Char('l') | Key::Right => Cmd::MOVE(1, 0),
                Key::Char('h') | Key::Left => Cmd::MOVE(-1, 0),
                Key::Char('k') | Key::Up => Cmd::MOVE(0, -1),
                Key::Char('j') | Key::Down => Cmd::MOVE(0, 1),
                Key::Char('q') | Key::Esc => Cmd::EXIT,
                Key::Char('\n') => Cmd::SELECT,
                _ => Cmd::DEBUG(Event::Key(key)),
            },
            _ => Cmd::DEBUG(event),
        }
    }
    pub fn process_cmds() {}
}

fn game_selection(frame: &mut Frame) {
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
