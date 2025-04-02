use std::sync::mpsc;
use std::time::Duration;
use std::{io, thread};

use termion::event::{Event, Key};
use termion::input::TermRead;

use crate::entity::EntityType;

#[derive(PartialEq)]
pub enum Cmd {
    STOP,
    MOVE(i8, i8),
    DEBUG(Event),
    SPAWN(u16, u16, EntityType),
    SELECT,
    RETURN,
    EXIT,
}
/// Aliases for direction vectors
impl Cmd {
    pub const RIGHT: Cmd = Cmd::MOVE(1, 0);
    pub const LEFT: Cmd = Cmd::MOVE(-1, 0);
    pub const UP: Cmd = Cmd::MOVE(0, -1);
    pub const DOWN: Cmd = Cmd::MOVE(0, 1);
}

/// creates threads for monitoring various event sources and forwards
/// those events over a channel to be ingested by a separate thread.
/// https://stackoverflow.com/a/55201400
/// https://doc.rust-lang.org/std/io/struct.Stdin.html#method.lock
pub fn create_data_channel() -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel::<Event>();

    // thread for checking user keyboard and mouse events
    let tx_usr_input = tx.clone();
    thread::spawn(move || loop {
        // Transmit any valid Events over channel, ignoring Errors
        for input_event in (&mut io::stdin().events()).flatten() {
            let _ = tx_usr_input.send(input_event);
        }
    });

    // thread for checking terminal resizes
    let tx_term_resize = tx.clone();
    let sleep_interval: u64 = 500; // milliseconds
    thread::spawn(move || {
        let mut term_size: (u16, u16) = (0, 0);
        loop {
            thread::sleep(Duration::from_millis(sleep_interval));
            if let Ok(new_size) = termion::terminal_size() {
                if new_size != term_size {
                    term_size = new_size;
                    let _ = tx_term_resize.send(Event::Key(Key::Null));
                } else {
                    continue;
                };
            };
        }
    });

    rx
}
