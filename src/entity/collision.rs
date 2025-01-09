use core::f32;

use crate::entity::collision_geometry::{get_angle, map_angle, Square};
use crate::entity::{Entities, Entity};

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

/// TODO: unit-test this
fn basic_collision_handling(me: &mut Entity, thee: &mut Entity) {
    // when is the projected collision going to occur?
    // (ASSUME ON CURRENT TIME-STEP AS INTERSECTION HAS ALREADY HAPPENED)

    // We're within each other's hit radii, but how should we characterize the collision?
    let my_hitbox = Square::new(&me.pos, &me.hit_radius);
    let thy_hitbox = Square::new(&thee.pos, &thee.hit_radius);

    if my_hitbox.intersects(&thy_hitbox) {
        //
        // where is the other entity relative to us?
        let direction_of_target: f32;
        if let Some(angle) = &get_angle(&me.pos, &thee.pos) {
            direction_of_target = map_angle(*angle, 8);
        } else {
            return;
        }
        //
        // we're clipping the target, so let's adjust our position...
        let _overlap = my_hitbox.overlap_size(&thy_hitbox);
        //
        // are we travelling towards the other entity?
        let direction_of_travel: f32;
        let origin: (f32, f32) = (0.0, 0.0);
        if let Some(angle) = &get_angle(&origin, &me.vel) {
            direction_of_travel = map_angle(*angle, 8);
        } else {
            return;
        }
        //
        // IF I am travelling towards the target, then consider this a COLLISION!
        if direction_of_travel == direction_of_target {
            let x_dir = (direction_of_travel * f32::consts::PI / 180.0).cos();
            let y_dir = (direction_of_travel * f32::consts::PI / 180.0).sin();
            let resultant_vels = collision_calc(
                (x_dir * me.vel.0, y_dir * me.vel.1),
                (x_dir * thee.vel.0, y_dir * thee.vel.1),
                me.mass,
                thee.mass,
            );
            me.target_vel(resultant_vels.0 .0, resultant_vels.0 .1);
        }
    }
}

/// calculate resultant velocity between two point masses from ...
/// >> conservation of kinetic energy:
/// > > 0.5*m1*v_1a^2 + 0.5*m2*v_2a^2 = 0.5*m1*v_1b^2 + 0.5*m2*v_2b^2
/// >> conservation of momentum :
/// > > m1*v_1a + m2*v_2a = m1*v_1b + m2*v_2b
///
fn collision_calc(va: (f32, f32), vb: (f32, f32), ma: f32, mb: f32) -> ((f32, f32), (f32, f32)) {
    let collide = |v1: f32, m1: f32, v2: f32, m2: f32| -> f32 {
        (v1 * (m1 - m2) + 2.0 * m2 * v2) / (m1 + m2)
    };
    //
    // return the resulting velocities following the collision
    let va_new = (collide(va.0, vb.0, ma, mb), collide(va.1, vb.1, ma, mb));
    let vb_new = (collide(vb.0, va.0, ma, mb), collide(vb.1, va.1, ma, mb));
    (va_new, vb_new)
}
