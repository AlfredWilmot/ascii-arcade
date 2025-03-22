use std::sync::mpsc;
use std::{io, thread};

use termion::event::{Event, Key, MouseButton, MouseEvent};
use termion::input::TermRead;

use crate::entity::EntityType;

/// creates a thread for monitoring keystrokes and forwarding
/// them over a channel to be ingested by a separate thread
/// https://stackoverflow.com/a/55201400
/// https://doc.rust-lang.org/std/io/struct.Stdin.html#method.lock
pub fn create_data_channel() -> mpsc::Receiver<termion::event::Event> {
    let (tx, rx) = mpsc::channel::<termion::event::Event>();

    // thread for checking user events
    thread::spawn(move || loop {
        for event in &mut io::stdin().events() {
            let event = event.unwrap();
            tx.send(event).unwrap();
        }
    });

    rx
}

pub enum Cmd {
    STOP,
    MOVE(i8, i8),
    DEBUG(Event),
    SPAWN(u16, u16, EntityType),
    SELECT,
    RETURN,
    EXIT,
}

/// check key presses to drive player behaviour
/// read-in a set of key-strokes and drive velocity based on weighted average
/// corresponding to the key presesd
pub fn keyboard_control(rx: &mpsc::Receiver<termion::event::Event>) -> Cmd {
    match rx.try_recv() {
        Ok(event) => sandbox_game_fsm(event),
        Err(_) => Cmd::STOP,
    }
}

/// Finite State Machine (FSM) for the sandbox game.
pub fn sandbox_game_fsm(event: Event) -> Cmd {
    match event {
        Event::Key(key) => match key {
            Key::Char('d') => Cmd::MOVE(1, 0),
            Key::Char('a') => Cmd::MOVE(-1, 0),
            Key::Char('w') => Cmd::MOVE(0, -1),
            Key::Char('s') => Cmd::MOVE(0, 1),
            Key::Char('q') | Key::Esc => Cmd::EXIT,
            _ => Cmd::DEBUG(Event::Key(key)),
        },
        Event::Mouse(mouse) => match mouse {
            MouseEvent::Press(MouseButton::Left, x, y) => Cmd::SPAWN(x, y, EntityType::Npc),
            MouseEvent::Press(MouseButton::Right, x, y) => Cmd::SPAWN(x, y, EntityType::Static),
            _ => Cmd::DEBUG(Event::Mouse(mouse)),
        },
        _ => Cmd::DEBUG(event),
    }
}

/// Finite State Machine (FSM) for the main menu.
pub fn menu_fsm(event: Event) -> Cmd {
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
