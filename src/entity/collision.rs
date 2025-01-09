use core::f32;

use crate::entity::collision_geometry::{get_angle, map_angle, Square};
use crate::entity::{Entities, Entity};

pub fn resolve(entities: &mut Entities) {
    pair_wise_comparison(entities);
}

// -------------------------------------------------------------------------- //
// ------------------------------ ALGORITHMS -------------------------------- //
// -------------------------------------------------------------------------- //

/// compares each entity on the scene against all other entities on the scene.
/// WARNING: comparing each entity against ALL other entities on the scene
/// is the WORST-CASE scenario (n^2)
fn pair_wise_comparison(entities: &mut Entities) {
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
            entity_b = entities[j].clone();
            entity_a = entities[i].clone();

            entity_a.handle_collision(&entity_b);
            entity_b.handle_collision(&entity_a);

            entities[i] = entity_a;
            entities[j] = entity_b;
        }
    }
}

impl Entity {
    /// TODO: unit-test this
    pub fn handle_collision(&mut self, target: &Entity) {
        // when is the projected collision going to occur?
        // (ASSUME ON CURRENT TIME-STEP AS INTERSECTION HAS ALREADY HAPPENED)

        // We're within each other's hit radii, but how should we characterize the collision?
        let my_hitbox = Square::new(&self.pos, &self.hit_radius);
        let thy_hitbox = Square::new(&target.pos, &target.hit_radius);

        if my_hitbox.intersects(&thy_hitbox) {
            //
            // where is the other entity relative to us?
            let direction_of_target: f32;
            if let Some(angle) = &get_angle(&self.pos, &target.pos) {
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
            if let Some(angle) = &get_angle(&origin, &self.vel) {
                direction_of_travel = map_angle(*angle, 8);
            } else {
                return;
            }
            //
            // IF I am travelling towards the target, then consider this a COLLISION!
            if direction_of_travel == direction_of_target {
                let x_dir = (direction_of_travel * f32::consts::PI / 180.0).cos().abs();
                let y_dir = (direction_of_travel * f32::consts::PI / 180.0).sin().abs();
                let resultant_vels = collision_calc(
                    (x_dir * self.vel.0, y_dir * self.vel.1),
                    self.mass,
                    (x_dir * target.vel.0, y_dir * target.vel.1),
                    target.mass,
                );
                self.target_vel(resultant_vels.0, resultant_vels.1);
            }
        }
    }
}

/// calculate resultant velocity after colliding with a target
/// >> conservation of kinetic energy:
/// > > 0.5*m1*v_1a^2 + 0.5*m2*v_2a^2 = 0.5*m1*v_1b^2 + 0.5*m2*v_2b^2
/// >> conservation of momentum :
/// > > m1*v_1a + m2*v_2a = m1*v_1b + m2*v_2b
///
fn collision_calc(va: (f32, f32), ma: f32, vb: (f32, f32), mb: f32) -> (f32, f32) {
    (
        (va.0 * (ma - mb) + 2.0 * mb * vb.0) / (ma + mb),
        (va.1 * (ma - mb) + 2.0 * mb * vb.1) / (ma + mb),
    )
}
