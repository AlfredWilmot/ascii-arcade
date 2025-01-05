use core::f32;

use crate::collision_geometry::{get_angle, Square, ORIENTATION};
use crate::entity::{collision_calc, Entities, Entity};

pub fn resolve(entities: &mut Entities) {
    pair_wise_comparison(entities, basic_collision_handling);
}

// -------------------------------------------------------------------------- //
// ------------------------------ ALGORITHMS -------------------------------- //
// -------------------------------------------------------------------------- //

/// compares each entity on the scene against all other entities on the scene.
/// WARNING: comparing each entity against ALL other entities on the scene
/// is the WORST-CASE scenario (n^2)
fn pair_wise_comparison(entities: &mut Entities, func: fn(&mut Entity, &mut Entity)) {
    let mut entity_a: Entity;
    let mut entity_b: Entity;

    for i in 0..entities.len() {
        // perform collision detection against ALL other entities in the scene (n^2)
        for j in 0..entities.len() {
            // an entity cannot collide with itself!
            if i == j {
                continue;
            }

            // NOTE: not a fan of cloning here,
            // need to find a way to mutably access >=2 entries in the entities vector
            // this will become more important if additional unique state data is created for each entity
            entity_a = entities[i].clone();
            entity_b = entities[j].clone();

            func(&mut entity_a, &mut entity_b);

            entities[i] = entity_a;
            entities[j] = entity_b;
        }
    }
}

fn basic_collision_handling(me: &mut Entity, thee: &mut Entity) {
    // when is the projected collision going to occur?
    // (ASSUME ON CURRENT TIME-STEP AS INTERSECTION HAS ALREADY HAPPENED)

    // We're within each other's hit radii, but how should we characterize the collision?
    let my_hitbox = Square::new(&me.pos, &me.hit_radius);
    let thy_hitbox = Square::new(&thee.pos, &thee.hit_radius);

    if my_hitbox.intersects(&thy_hitbox) {
        // where is the other entity relative to us?
        let direction_of_target: ORIENTATION;
        if let Some(angle) = &get_angle(&me.pos, &thee.pos) {
            if let Some(val) = ORIENTATION::from_angle(angle) {
                direction_of_target = val;
                // we're clipping the target, so let's adjust our position...
                let overlap = my_hitbox.overlap_size(&thy_hitbox);
                let repulsion_x = 100.0_f32.powf(overlap.0);
                let repulsion_y = 100.0_f32.powf(overlap.1);
                let x_dir = (angle * f32::consts::PI / 180.0).cos();
                let y_dir = (angle * f32::consts::PI / 180.0).sin();
                me.apply_force(x_dir * repulsion_x, y_dir * repulsion_y);
                thee.apply_force(-x_dir * repulsion_x, -y_dir * repulsion_y);
            } else {
                return;
            }
        } else {
            return;
        }

        // are we travelling towards the other entity?
        let direction_of_travel: ORIENTATION;
        let origin: (f32, f32) = (0.0, 0.0);
        if let Some(angle) = &get_angle(&origin, &me.vel) {
            if let Some(direction) = ORIENTATION::from_angle(angle) {
                direction_of_travel = direction;
            } else {
                return;
            }
        } else {
            return;
        }

        // IF we collide, what will our resulting velocitues be along each axis?
        let (my_vel, thy_vel) = collision_calc(me, thee);

        // IF I am travelling towards the target, then consider this a COLLISION!
        // TODO: USE DOT-PRODUCT!!
        if direction_of_travel == direction_of_target {
            match direction_of_travel {
                ORIENTATION::East | ORIENTATION::West => {
                    me.target_vel(my_vel.0, 0.0);
                    thee.target_vel(thy_vel.0, 0.0);
                }
                ORIENTATION::North | ORIENTATION::South => {
                    me.target_vel(0.0, my_vel.1);
                    thee.target_vel(0.0, thy_vel.1);
                }
                ORIENTATION::NorthEast
                | ORIENTATION::NorthWest
                | ORIENTATION::SouthEast
                | ORIENTATION::SouthWest => {
                    me.target_vel(my_vel.0, my_vel.1);
                    thee.target_vel(thy_vel.0, thy_vel.1);
                }
            }
        }
    }
}
