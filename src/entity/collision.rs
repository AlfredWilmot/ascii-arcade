use core::f32;

use crate::entity::collision_geometry::Square;
use crate::entity::{Entities, Entity};

use super::vector::EuclidianVector;

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
    /// Determines whether this entity is colliding with some other entity, and if so,
    /// updates this entity with the forces experienced due to the change in velocity
    /// resulting from the collision.
    pub fn handle_collision(&mut self, target: &Entity) {
        //
        // are our hitboxes intersecting?
        let my_hitbox = Square::new(&self.pos, &self.hit_radius);
        let thy_hitbox = Square::new(&target.pos, &target.hit_radius);
        if my_hitbox.intersects(&thy_hitbox) {
            //
            // am I travelling towards the target?
            let me_to_you = EuclidianVector::from(self.pos, target.pos).unit();
            //
            // is the target travelling towards me?
            let you_to_me = EuclidianVector::from(target.pos, self.pos).unit();
            //
            // do either of us have velocity components directed towards the other?
            if self.vel.dot(&me_to_you) > 0.0 || target.vel.dot(&you_to_me) > 0.0 {
                let resultant_vel = collision_calc(
                    EuclidianVector::new(
                        me_to_you.x * self.vel.x,
                        me_to_you.y * self.vel.y,
                    ),
                    self.mass,
                    EuclidianVector::new(
                        you_to_me.x * target.vel.x,
                        you_to_me.y * target.vel.y,
                    ),
                    target.mass,
                );

                self.target_vel(resultant_vel.x, resultant_vel.y);
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
fn collision_calc(va: EuclidianVector, ma: f32, vb: EuclidianVector, mb: f32) -> EuclidianVector {
    EuclidianVector::new(
        (va.x * (ma - mb) + 2.0 * mb * vb.x) / (ma + mb),
        (va.y * (ma - mb) + 2.0 * mb * vb.y) / (ma + mb),
    )
}
