use core::f32;

use crate::entity::collision_geometry::Square;
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
    for i in 0..entities.len() {
        // perform collision detection against ALL other entities in the scene (n^2)
        for j in 0..entities.len() {
            // an entity cannot collide with itself!
            if i == j {
                continue;
            }
            let other = entities[j].clone();
            entities[i].handle_collision(&other);
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
            // am I travelling towards the target, is the target travelling towards me?

            let resultant_vels = collision_calc(
                (self.vel.x, self.vel.y),
                self.mass,
                (target.vel.x, target.vel.y),
                target.mass,
            );
            self.target_vel(resultant_vels.0, resultant_vels.1);
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
