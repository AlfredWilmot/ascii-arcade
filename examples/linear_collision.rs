use ascii_arcade::entity::vector::EuclidianVector;
use ascii_arcade::entity::{update, Entities, Entity, EntityType};
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
    let mut _stdout = scene::init();
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

        // create a mutable reference to the "player" entity"
        let player = &mut entities_now[0];

        // apply control signals to player
        match user_input::keyboard_control(&rx) {
            Cmd::MOVE(x, y) => {
                // can only control movement when atop sumat
                if player.grounded {
                    // generate movement control-force based on user-input
                    let move_force: EuclidianVector = if x == 0 && y != 0 {
                        player.target_vel(player.vel.x, 10.0 * y as f32)
                    } else if y == 0 && x != 0 {
                        player.target_vel(20.0 * x as f32, player.vel.y)
                    } else {
                        player.target_vel(20.0 * x as f32, 10.0 * y as f32)
                    };
                    player.apply_force(move_force);
                }
            }
            Cmd::STOP => {}
            Cmd::EXIT => {
                break 'game;
            }
            Cmd::DEBUG(_) => {}
            // spawn an entity of some type at some location
            Cmd::SPAWN(x, y, id) => {
                entities_now.push(Entity::new(id, (x as f32, y as f32)));
            }
        }

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
    scene::close();
}
