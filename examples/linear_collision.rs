use ascii_arcade::collision;
use ascii_arcade::entity::{Entities, Entity, EntityType};
use ascii_arcade::physics;
use ascii_arcade::scene;
use ascii_arcade::user_input;
use ascii_arcade::user_input::Cmd;
use std::thread;
use std::time::Duration;

const TIME_DELTA_MS: u64 = 10;

fn main() {
    //
    // INITIALISATION
    //

    // keep the RawTerminal in scope until we exit the game
    let mut _stdout = scene::init();
    let rx = user_input::create_data_channel();

    // keep this up-to-date on every game-loop cycle so we can query the scene by coordinates
    let mut entities_now: Entities = Vec::new();
    let mut entities_then: Entities;

    // player to be controlled by user
    let player = Entity {
        id: EntityType::Player,
        ..Default::default()
    };
    entities_now.push(player);

    // copies of this entity are created when LMB is pressed
    let mut spawned = Entity {
        id: EntityType::Npc,
        ..Default::default()
    };

    //
    // GAME LOOP
    //
    let dt = Duration::from_millis(TIME_DELTA_MS).as_secs_f32();
    'game: loop {
        // capture the current state of the scene
        entities_then = entities_now.to_vec();

        // create a mutable reference to the "player" entity"
        let player = &mut entities_now[0];

        // apply control signals to player
        match user_input::keyboard_control(&rx) {
            Cmd::MOVE(x, y) => {
                // TODO: can only move laterally if ontop of something
                player.vel.0 += 20.0 * x as f32;
                // TODO: can only jump while ontop or adjacent to something
                player.vel.1 += 10.0 * y as f32;
            }
            Cmd::STOP => {}
            Cmd::EXIT => {
                break 'game;
            }
            Cmd::DEBUG(key) => {
                scene::debug_print(key, 1);
            }
            Cmd::SPAWN(x, y) => {
                spawned.pos = (x as f32, y as f32);
                entities_now.push(spawned.clone());
            }
        }

        // apply global acceleration rules
        for entity in entities_now.iter_mut() {
            entity.acc = (0.0, 9.81);
        }

        // update rules based on collision state
        collision::resolve(&mut entities_now);

        // resolve physics calculations
        for entity in entities_now.iter_mut() {
            physics::update(entity, dt);
            physics::apply_constraints(entity);
        }

        // physics calcuations done, render!
        scene::render(&entities_then, &entities_now);
        thread::sleep(Duration::from_secs_f32(dt));
    }
    scene::close();
}
