use crate::entity::{Entity, BACKGROUND};
use std::fmt::Debug;
use std::io::{self, Stdout};
use std::iter;
use termion;
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};

/// initialize terminal
pub fn init() -> MouseTerminal<RawTerminal<Stdout>> {
    // Set the TTY into "Raw mode":
    // - stdin is no longer printed to terminal
    // - stdin is read one-byte at a time (for handling of individual key-presses)
    // References:
    // - https://docs.rs/termion/1.5.2/termion/raw/index.html
    // - https://stackoverflow.com/a/55881770
    let stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    println!("{}{}", termion::cursor::Hide, termion::clear::All);
    return stdout;
}

/// clean-up terminal
pub fn close() {
    println!(
        "{}{}{}",
        termion::cursor::Goto(1, 1),
        termion::cursor::Show,
        termion::clear::All
    );
}

/// Print diagnostic information
pub fn debug_print<T: Debug>(text: T, offset: u16) {
    let offset = if offset <= 0 { 1 } else { offset };
    println!(
        "{}{}{:?}",
        termion::cursor::Goto(1, offset),
        termion::clear::CurrentLine,
        text
    );
}

/// display all the entities in the scene
pub fn render(then: &Vec<Entity>, now: &Vec<Entity>) {
    for (old, new) in iter::zip(then, now) {
        let (x0, y0) = old.coordinates();
        let (x1, y1) = new.coordinates();

        // clear the old position if the new position has changed
        if (x0, y0) != (x1, y1) {
            println!(
                "{}{}",
                // move cursor to old position
                termion::cursor::Goto(x0, y0),
                // clear cell
                BACKGROUND,
            );
        }
        // print the entity onto the new position
        println!(
            "{}{}",
            // move cursor to new position
            termion::cursor::Goto(x1, y1),
            // insert entity
            new
        );
    }
}
