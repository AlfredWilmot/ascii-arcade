use crate::collision_geometry::{get_angle, Circle, ORIENTATION};
use crate::entity::{Entities, Entity, EntityType};
use crate::physics;
use crate::scene::debug_print;

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
    let my_hitbox = Circle::new(&me.pos, &me.hit_radius);
    let thy_hitbox = Circle::new(&thee.pos, &thee.hit_radius);

    if my_hitbox.intersects(&thy_hitbox) {
        // where is the other entity relative to us?
        let direction_of_target: ORIENTATION;
        if let Some(angle) = &get_angle(&me.pos, &thee.pos) {
            if me.id == EntityType::Player {
                debug_print(format!("target: {:.2}", angle), 1);
            }
            if let Some(val) = ORIENTATION::from_angle(angle) {
                direction_of_target = val;
            } else {
                return;
            }
        } else {
            return;
        }

        // we're clipping the target, so let's adjust our position...
        let overlap = my_hitbox.overlap_length(&thy_hitbox);
        let repulsion = 100.0_f32.powf(overlap);
        match direction_of_target {
            ORIENTATION::East => {
                me.acc = (-repulsion, me.acc.1);
                thee.acc = (repulsion, thee.acc.1);
            }
            ORIENTATION::NorthEast => {
                me.acc = (-repulsion, repulsion);
                thee.acc = (repulsion, -repulsion);
            }
            ORIENTATION::North => {
                me.acc = (me.acc.0, repulsion);
                thee.acc = (thee.acc.0, -repulsion);
            }
            ORIENTATION::NorthWest => {
                me.acc = (repulsion, repulsion);
                thee.acc = (-repulsion, -repulsion);
            }
            ORIENTATION::West => {
                me.acc = (repulsion, me.acc.1);
                thee.acc = (-repulsion, thee.acc.1);
            }
            ORIENTATION::SouthWest => {
                me.acc = (repulsion, -repulsion);
                thee.acc = (-repulsion, repulsion);
            }
            ORIENTATION::South => {
                me.acc = (me.acc.0, -repulsion);
                thee.acc = (thee.acc.0, repulsion);
            }
            ORIENTATION::SouthEast => {
                me.acc = (-repulsion, -repulsion);
                thee.acc = (repulsion, repulsion);
            }
        }

        // are we travelling towards the other entity?
        let direction_of_travel: ORIENTATION;
        let origin: (f32, f32) = (0.0, 0.0);
        if let Some(angle) = &get_angle(&origin, &me.vel) {
            if me.id == EntityType::Player {
                debug_print(format!("travel: {:.2}", angle), 2);
            }
            if let Some(direction) = ORIENTATION::from_angle(angle) {
                direction_of_travel = direction;
            } else {
                return;
            }
        } else {
            return;
        }

        // IF we collide, what will our resulting velocitues be along each axis?
        let (my_vel, thy_vel) = physics::collision_calc(&me, &thee);

        // IF I am travelling towards the target, then consider this a COLLISION!
        if direction_of_travel == direction_of_target {
            match direction_of_travel {
                ORIENTATION::East | ORIENTATION::West => {
                    me.vel.0 = my_vel.0;
                    thee.vel.0 = thy_vel.0;
                }
                ORIENTATION::North | ORIENTATION::South => {
                    me.vel.1 = my_vel.1;
                    thee.vel.1 = thy_vel.1;
                }
                ORIENTATION::NorthEast
                | ORIENTATION::NorthWest
                | ORIENTATION::SouthEast
                | ORIENTATION::SouthWest => {
                    me.vel = my_vel;
                    thee.vel = thy_vel;
                }
            }
        }
    }
}
