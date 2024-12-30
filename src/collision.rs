use crate::collision_geometry::{get_angle, Circle, ORIENTATION};
use crate::entity::Entities;
use crate::physics;
use crate::scene;

// -------------------------------------------------------------------------- //
// ------------------------------ ALGORITHMS -------------------------------- //
// -------------------------------------------------------------------------- //

/// compares each entity on the scene against all other entities on the scene
pub fn pair_wise_comparison(now: &mut Entities) {
    for i in 0..now.len() {
        // perform collision detection against ALL other entities in the scene (n^2)
        for j in 0..now.len() {
            // an entity cannot collide with itself!
            if i == j {
                continue;
            }

            // when is the projected collision going to occur?
            // (ASSUME ON CURRENT TIME-STEP AS INTERSECTION HAS ALREADY HAPPENED)

            // We're within each other's hit radii, but how should we characterize the collision?
            let my_hitbox = Circle::new(&now[i].pos, &now[i].hit_radius);
            let thy_hitbox = Circle::new(&now[j].pos, &now[j].hit_radius);

            if my_hitbox.intersects(&thy_hitbox) {
                // where is the other entity relative to us?
                let direction_of_target: ORIENTATION;
                if let Some(val) = ORIENTATION::from_angle(&get_angle(&now[i].pos, &now[j].pos)) {
                    direction_of_target = val;
                    if i == 0 {
                        scene::debug_print(
                            format!("direction_of_target: {:?}", direction_of_target),
                            1,
                        );
                    }
                } else {
                    continue;
                }

                // we're clipping the target, so let's adjust our position...
                let overlap = my_hitbox.overlap_length(&thy_hitbox);
                let repulsion = 100.0_f32.powf(overlap);
                match direction_of_target {
                    ORIENTATION::East => {
                        now[i].acc = (-repulsion, now[i].acc.1);
                        now[j].acc = (repulsion, now[j].acc.1);
                    }
                    ORIENTATION::NorthEast => {
                        now[i].acc = (-repulsion, repulsion);
                        now[j].acc = (repulsion, -repulsion);
                    }
                    ORIENTATION::North => {
                        now[i].acc = (now[i].acc.0, repulsion);
                        now[j].acc = (now[j].acc.0, -repulsion);
                    }
                    ORIENTATION::NorthWest => {
                        now[i].acc = (repulsion, repulsion);
                        now[j].acc = (-repulsion, -repulsion);
                    }
                    ORIENTATION::West => {
                        now[i].acc = (repulsion, now[i].acc.1);
                        now[j].acc = (-repulsion, now[j].acc.1);
                    }
                    ORIENTATION::SouthWest => {
                        now[i].acc = (repulsion, -repulsion);
                        now[j].acc = (-repulsion, repulsion);
                    }
                    ORIENTATION::South => {
                        now[i].acc = (now[i].acc.0, -repulsion);
                        now[j].acc = (now[j].acc.0, repulsion);
                    }
                    ORIENTATION::SouthEast => {
                        now[i].acc = (-repulsion, -repulsion);
                        now[j].acc = (repulsion, repulsion);
                    }
                }

                // are we travelling towards the other entity?
                let direction_of_travel: ORIENTATION;
                if let Some(val) = ORIENTATION::from_angle(&now[i].direction()) {
                    direction_of_travel = val;
                    if i == 0 {
                        scene::debug_print(
                            format!("direction_of_travel: {:?}", direction_of_travel),
                            2,
                        );
                    }
                } else {
                    continue;
                }

                // IF we collide, what will our resulting velocitues be along each axis?
                let (my_vel, thy_vel) = physics::collision_calc(&now[i], &now[j]);

                // IF I am travelling towards the target, then consider this a COLLISION!
                if direction_of_travel == direction_of_target {
                    match direction_of_travel {
                        ORIENTATION::East | ORIENTATION::West => {
                            now[i].vel.0 = my_vel.0;
                            now[j].vel.0 = thy_vel.0;
                        }
                        ORIENTATION::North
                        | ORIENTATION::South
                        | ORIENTATION::NorthEast
                        | ORIENTATION::NorthWest
                        | ORIENTATION::SouthEast
                        | ORIENTATION::SouthWest => {
                            now[i].vel.1 = my_vel.1;
                            now[j].vel.1 = thy_vel.1;
                        }
                    }
                }
            }
        }
    }
}
