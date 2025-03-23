use crate::{
    entity::{vector::EuclidianVector, Entities, Entity},
    user_input::Cmd,
};

// the different games the user can play
pub const GAME_COUNT: usize = 2; // this must match the number of Game enum fields.
#[derive(Clone, Copy)]
pub enum Game {
    Sandbox,
    Pong,
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
