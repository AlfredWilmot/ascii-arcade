use termion::event::{Event, Key, MouseButton, MouseEvent};

use crate::{
    entity::{vector::EuclidianVector, Entities, Entity, EntityType},
    user_input::Cmd,
};

// the different games the user can play
#[derive(Clone, Copy)]
pub enum Game {
    Sandbox,
    Pong,
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

/// Interface for the sandbox game.
pub struct SandboxGame;

impl SandboxGame {
    /// Parses an input Event into the correspinding Cmd for this game.
    pub fn parse_event(event: Event) -> Cmd {
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

    /// Apply control signals to player, and possibly modify entity pool.
    pub fn process_cmds(player: &mut Entity, entities: &mut Entities, cmd: Cmd) -> Cmd {
        match cmd {
            Cmd::MOVE(x, y) => {
                // generate movement control-force based on user-input
                let mut move_force: EuclidianVector = if x == 0 && y != 0 {
                    player.target_vel(player.vel.x, 8.0 * y as f32)
                } else if y == 0 && x != 0 {
                    player.target_vel(20.0 * x as f32, player.vel.y)
                } else {
                    player.target_vel(20.0 * x as f32, 8.0 * y as f32)
                };

                // can only apply vertical control force when not free-falling
                if !player.grounded {
                    move_force.y = 0.0;
                    move_force.x *= 0.025;
                }
                player.apply_force(move_force);
            }
            // spawn an entity of some type at some location
            Cmd::SPAWN(x, y, id) => {
                entities.push(Entity::new(id, (x as f32, y as f32)));
            }
            _ => {}
        }
        cmd
    }
}
