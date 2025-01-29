use core::f32;

use super::vector::EuclidianVector;
use crate::entity::collision_geometry::Square;
use crate::entity::{Entities, Entity, EntityType};

// -------------------------------------------------------------------------- //
// ------------------------------ ALGORITHMS -------------------------------- //
// -------------------------------------------------------------------------- //

/// compares each entity on the scene against all other entities on the scene.
/// WARNING: comparing each entity against ALL other entities on the scene
/// is the WORST-CASE scenario (n^2)
pub fn pairwise(entities_then: &Entities, entities_now: &mut Entities) {
    'outer: for (i, entity_under_test) in entities_now.iter_mut().enumerate() {
        'inner: for (j, entity_to_compare) in entities_then.iter().enumerate() {
            //
            // early-exit conditions
            if entity_under_test.id == EntityType::Static {
                continue 'outer;
            }
            if i == j {
                continue 'inner;
            }
            //
            // are we colliding? if so, resolve!
            entity_under_test.try_collide(entity_to_compare);
            //
            // TODO: handle reaction forces (F=ma)
            let _max_reaction_force = EuclidianVector::new(
                entity_under_test.mass * entity_under_test.acc.x,
                entity_under_test.mass * entity_under_test.acc.y,
            )
            .magnitude();
        }
    }
}

impl Entity {
    /// Determine whether this entity is colliding (intersecting) with the target entity
    /// using a hitbox of some description.
    pub fn colliding(&self, target: &Entity) -> bool {
        let my_hitbox = Square::new(&self.pos, &self.hit_radius);
        let thy_hitbox = Square::new(&target.pos, &target.hit_radius);
        my_hitbox.intersects(&thy_hitbox)
    }

    /// Determines whether this entity is colliding with some other entity, and if so,
    /// updates this entity with the forces experienced due to the change in velocity
    /// resulting from the collision.
    pub fn try_collide(&mut self, target: &Entity) {
        //
        // are we even near each other?
        if !self.colliding(target) {
            return;
        }

        // where are we relative to one another?
        let me_to_you = EuclidianVector::from(self.pos, target.pos).unit();
        let you_to_me = EuclidianVector::from(target.pos, self.pos).unit();

        // are both our trajectories either orthogonal to or in the opposite direction of one-another?
        // if so, then we're NOT moving further into the collision, so there's no velocity changes.
        if self.vel.dot(&me_to_you) <= 0.0 && target.vel.dot(&you_to_me) <= 0.0 {
            return;
        }

        // what forces do I experience due to volocity changes after colliding?
        let resultant_vel = collision_calc(
            &EuclidianVector::new(
                me_to_you.x.abs() * self.vel.x,
                me_to_you.y.abs() * self.vel.y,
            ),
            &self.mass,
            &EuclidianVector::new(
                you_to_me.x.abs() * target.vel.x,
                you_to_me.y.abs() * target.vel.y,
            ),
            &target.mass,
        );
        self.target_vel(resultant_vel.x, resultant_vel.y);
    }
}

/// calculate resultant velocity after colliding with a target
/// >> conservation of kinetic energy:
/// > > 0.5*m1*v_1a^2 + 0.5*m2*v_2a^2 = 0.5*m1*v_1b^2 + 0.5*m2*v_2b^2
/// >> conservation of momentum :
/// > > m1*v_1a + m2*v_2a = m1*v_1b + m2*v_2b
///
pub fn collision_calc(
    va: &EuclidianVector,
    ma: &f32,
    vb: &EuclidianVector,
    mb: &f32,
) -> EuclidianVector {
    EuclidianVector::new(
        (va.x * (ma - mb) + 2.0 * mb * vb.x) / (ma + mb),
        (va.y * (ma - mb) + 2.0 * mb * vb.y) / (ma + mb),
    )
}
