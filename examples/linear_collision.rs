use ascii_arcade::entity::vector::EuclidianVector;
use ascii_arcade::entity::{update, Entities, Entity, EntityType};
use ascii_arcade::games::process_cmds;
use ascii_arcade::user_input::Cmd;
use ascii_arcade::{scene, user_input};
use std::thread;
use std::time::Duration;

const TIME_DELTA_MS: u64 = 10;

fn main() {
    //
    // INITIALISATION
    //

    // keep the RawTerminal in scope until we exit the game
    let terminal = scene::init();
    let rx = user_input::create_data_channel();

    // keep this up-to-date on every game-loop cycle so we can query the scene by coordinates
    let mut entities_now: Entities = Vec::new();
    let mut entities_then: Entities;

    // player to be controlled by user
    let player = Entity::new(EntityType::Player, (1.0, 1.0));
    entities_now.push(player);

    //
    // GAME LOOP
    //
    let dt = Duration::from_millis(TIME_DELTA_MS).as_secs_f32();
    'game: loop {
        // capture the current state of the scene
        entities_then = entities_now.to_vec();

        // extract the player from the entity pool.
        let mut player = entities_now.pop().unwrap();

        // process user input.
        let cmd = user_input::keyboard_control(&rx);
        if let Cmd::EXIT = process_cmds(&mut player, &mut entities_now, cmd) {
            break 'game;
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
    scene::close(terminal.unwrap());
}
