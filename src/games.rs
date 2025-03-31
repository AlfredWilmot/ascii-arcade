use std::{sync::mpsc::Receiver, thread, time::Duration};

use termion::event::{Event, Key, MouseButton, MouseEvent};

use crate::{
    entity::{update, vector::EuclidianVector, Entities, Entity, EntityType},
    scene,
    user_input::Cmd,
};

// the different games the user can play
#[derive(Clone, Copy)]
pub enum Game {
    Sandbox,
    Pong,
}
pub const GAME_COUNT: usize = 2;

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

    /// Activate the game loop.
    pub fn play(input_reader: Receiver<Event>) {
        //
        // INITIALISATION
        //

        const TIME_DELTA_MS: u64 = 10;
        let dt = Duration::from_millis(TIME_DELTA_MS).as_secs_f32();

        // keep this up-to-date on every game-loop cycle so we can query the scene by coordinates
        let mut entities_now: Entities = Vec::new();
        let mut entities_then: Entities;

        // player to be controlled by user
        let player = Entity::new(EntityType::Player, (1.0, 1.0));
        entities_now.push(player);

        //
        // GAME LOOP
        //
        'game: loop {
            // capture the current state of the scene
            entities_then = entities_now.to_vec();

            // extract the player from the entity pool.
            let mut player = entities_now.pop().unwrap();

            // process user input.
            if let Ok(event) = input_reader.try_recv() {
                let cmd = SandboxGame::parse_event(event);
                if let Cmd::EXIT | Cmd::RETURN =
                    SandboxGame::process_cmds(&mut player, &mut entities_now, cmd)
                {
                    break 'game;
                }
            }

            // reinsert the player to the entity pool.
            entities_now.push(player);

            // apply global acceleration rules
            for entity in entities_now.iter_mut() {
                // assume the earth is beneath our feet
                let gravity = entity.target_acc(0.0, 9.81);
                entity.apply_force(gravity);

                // simulate frictional forces
                let friction: EuclidianVector = if entity.grounded {
                    entity.target_vel(entity.vel.x * 0.9, entity.vel.y)
                } else {
                    entity.target_vel(entity.vel.x * 0.99, entity.vel.y)
                };
                entity.apply_force(friction);
            }

            // resolve physics calculations
            update(&mut entities_now);

            // physics calculations done, render!
            scene::render(&entities_then, &entities_now);
            thread::sleep(Duration::from_secs_f32(dt));
        }
    }
}
