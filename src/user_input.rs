use std::sync::mpsc;
use std::{io, thread};

use termion::event::{Event, Key, MouseButton, MouseEvent};
use termion::input::TermRead;

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
    SPAWN(u16, u16),
    EXIT,
}

/// check key presses to drive player behaviour
/// read-in a set of key-strokes and drive velocity based on weighted average
/// corresponding to the key presesd
pub fn keyboard_control(rx: &mpsc::Receiver<termion::event::Event>) -> Cmd {
    match rx.try_recv() {
        Ok(event) => {
            match event {
                Event::Key(key) => match key {
                    Key::Char('d') => return Cmd::MOVE(1, 0),
                    Key::Char('a') => return Cmd::MOVE(-1, 0),
                    Key::Char('w') => return Cmd::MOVE(0, -1),
                    Key::Char('s') => return Cmd::MOVE(0, 1),
                    Key::Char('q') | Key::Esc => return Cmd::EXIT,
                    other_key => return Cmd::DEBUG(Event::Key(other_key)),
                },

                Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y)) => {
                    return Cmd::SPAWN(x, y)
                }
                other_event => {
                    // other keys
                    return Cmd::DEBUG(other_event);
                }
            }
        }
        Err(_) => {
            return Cmd::STOP;
        }
    }
}
