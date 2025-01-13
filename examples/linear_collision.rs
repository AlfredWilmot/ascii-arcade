use ascii_arcade::entity::{collision, Entities, Entity, EntityType};
use ascii_arcade::scene::debug_print;
use ascii_arcade::user_input::Cmd;
use ascii_arcade::{scene, user_input};
use std::ops::BitXor;
use std::thread;
use std::time::Duration;

const TIME_DELTA_MS: u64 = 10;

fn main() {
    //
    // INITIALISATION
    //
    let mut debug_mode: bool = false;

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
                // WIP
                if x == 0 && y != 0 {
                    // can only jump when on the ground
                    if player.grounded {
                        player.target_vel(player.vel.x, 10.0 * y as f32);
                    }
                } else if y == 0 && x != 0 {
                    player.target_vel(20.0 * x as f32, player.vel.y);
                } else {
                    player.target_vel(20.0 * x as f32, 10.0 * y as f32);
                }
            }
            Cmd::STOP => {}
            Cmd::EXIT => {
                break 'game;
            }
            Cmd::DEBUG(_) => {
                //scene::debug_print(key, 1);
                debug_mode = debug_mode.bitxor(true);
            }
            // spawn an entity of some type at some location
            Cmd::SPAWN(x, y, id) => {
                entities_now.push(Entity::new(id, (x as f32, y as f32)));
            }
        }

        // apply global acceleration rules
        for entity in entities_now.iter_mut() {
            entity.target_acc(0.0, 9.81);
            if entity.grounded {
                // simulates fricion
                entity.target_vel(entity.vel.x * 0.9, entity.vel.y);
            } else {
                // simulates less friction when airborne
                entity.target_vel(entity.vel.x * 0.99, entity.vel.y);
            }
        }

        // update rules based on collision state
        collision::resolve(&mut entities_now);

        // resolve physics calculations
        for entity in entities_now.iter_mut() {
            if debug_mode && entity.id == EntityType::Player {
                debug_print(
                    format!("force: ({:.1}, {:.1}) ", entity.force.x, entity.force.y),
                    1,
                );
            }
            entity.update();
            if debug_mode && entity.id == EntityType::Player {
                debug_print(
                    format!("pos: ({:.1}, {:.1}) ", entity.pos.0, entity.pos.1),
                    2,
                );
                debug_print(
                    format!("vel: ({:.1}, {:.1}) ", entity.vel.x, entity.vel.y),
                    3,
                );
                debug_print(
                    format!("acc: ({:.1}, {:.1}) ", entity.acc.x, entity.acc.y),
                    4,
                );
            }
        }

        // physics calculations done, render!
        scene::render(&entities_then, &entities_now);
        thread::sleep(Duration::from_secs_f32(dt));
    }
    scene::close();
}
