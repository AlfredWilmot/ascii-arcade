use std::sync::mpsc;
use std::{io, thread};

use termion::event::Event;
use termion::input::TermRead;

use crate::entity::EntityType;

pub enum Cmd {
    STOP,
    MOVE(i8, i8),
    DEBUG(Event),
    SPAWN(u16, u16, EntityType),
    SELECT,
    RETURN,
    EXIT,
}

/// creates a thread for monitoring keystrokes and forwarding
/// them over a channel to be ingested by a separate thread
/// https://stackoverflow.com/a/55201400
/// https://doc.rust-lang.org/std/io/struct.Stdin.html#method.lock
pub fn create_data_channel() -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel::<Event>();

    // thread for checking user events
    thread::spawn(move || loop {
        for input_event in &mut io::stdin().events() {
            let event = input_event.unwrap();
            tx.send(event).unwrap();
        }
    });

    rx
}
