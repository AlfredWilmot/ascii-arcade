use crate::entity::{Entity, BACKGROUND};
use ratatui::prelude::TermionBackend;
use ratatui::Terminal;
use std::error::Error;
use std::fmt::Debug;
use std::io::{self, Stdout};
use std::iter;
use termion;
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};

/// Ratatui Terminal with Mouse-support, using Termion as the Backend
type MouseEnabledTerminal = Terminal<TermionBackend<MouseTerminal<RawTerminal<Stdout>>>>;

/// initialize terminal
pub fn init() -> Result<MouseEnabledTerminal, Box<dyn Error>> {
    // Set the TTY into "Raw mode":
    // - stdin is no longer printed to terminal
    // - stdin is read one-byte at a time (for handling of individual key-presses)
    // References:
    // - https://docs.rs/termion/1.5.2/termion/raw/index.html
    // - https://stackoverflow.com/a/55881770
    let stdout = MouseTerminal::from(io::stdout().into_raw_mode()?);
    println!("{}{}", termion::cursor::Hide, termion::clear::All);
    let terminal = Terminal::new(TermionBackend::new(stdout))?;

    Ok(terminal)
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
    let offset = if offset == 0 { 1 } else { offset };
    println!(
        "{}{}{:?}",
        termion::cursor::Goto(1, offset),
        termion::clear::CurrentLine,
        text
    );
}

/// converts a tuple of floating point values to the equivalent terminal coordinate value.
/// the rendering plane is as follows and our origin starts at (1,1):
///
//   (1,1) --> (+x)
//   |
//   v
//  (+y)
//
pub fn term_coords(pos: (f32, f32)) -> (u16, u16) {
    let (x, y) = pos;
    let x = if x.round() <= 0.0 { 1.0 } else { x };
    let y = if y.round() <= 0.0 { 1.0 } else { y };
    (x.round() as u16, y.round() as u16)
}

/// display all the entities in the scene
pub fn render(then: &Vec<Entity>, now: &Vec<Entity>) {
    for (old, new) in iter::zip(then, now) {
        let (x0, y0) = term_coords(old.pos);
        let (x1, y1) = term_coords(new.pos);

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
