use core::f32;

use super::vector::EuclidianVector;
use crate::entity::collision_geometry::Square;
use crate::entity::{Entities, Entity};

/// compares each entity on the scene against all other entities on the scene.
/// WARNING: comparing each entity against ALL other entities on the scene
/// yields the WORST-CASE compute performance (n^2) -- serves as the baseline.
pub fn pairwise(entity_under_test: &mut Entity, entities: &Entities) {
    // initially assume the entity is not ontop of anything
    entity_under_test.grounded = false;

    // determine the various forces experienced by the entity under test
    let mut normal_force = EuclidianVector::new(0.0, 0.0);
    let mut collision_force = EuclidianVector::new(0.0, 0.0);
    let mut encounters: u32 = 0;

    for entity_to_compare in entities {
        // fumbling in the dark; am I alone or are we just not touching?
        if let Some(overlap) = entity_under_test.overlap(entity_to_compare) {
            if entity_under_test.pos == entity_to_compare.pos {
                continue;
            }
            // treat encounter as grounding condition for simplicity (for now)
            encounters += 1;

            // normal force due to the current encounter
            let you_to_me =
                EuclidianVector::from(entity_to_compare.pos, entity_under_test.pos).unit();

            if overlap.0 >= overlap.1 {
                entity_under_test.grounded = true;
                normal_force.y += you_to_me.y;
            } else {
                normal_force.x += you_to_me.x;
            }

            // velocity change force due to the current encounter
            if let Some(force) = entity_under_test.try_collide(entity_to_compare) {
                if overlap.0 >= overlap.1 {
                    collision_force.y += force.y;
                } else {
                    collision_force.x += force.x;
                }
            }
        }
    }

    if encounters > 0 {
        // determine normal force resulting from all encounters
        let mass = entity_under_test.mass;
        entity_under_test.apply_force(
            mass * entity_under_test.acc.x.abs() * normal_force.unit().x,
            2.0 * mass * entity_under_test.acc.y.abs() * normal_force.unit().y,
        );
        // determine average force due to velocity changes resulting from all encounters
        entity_under_test.apply_force(
            collision_force.x / (encounters as f32),
            collision_force.y / (encounters as f32),
        )
    }
}

impl Entity {
    /// Determine whether this entity is colliding (intersecting) with the target entity
    /// using a hitbox of some description.
    pub fn overlap(&self, target: &Entity) -> Option<(f32, f32)> {
        let my_hitbox = Square::new(&self.pos, &self.hit_radius);
        let thy_hitbox = Square::new(&target.pos, &target.hit_radius);
        let result = my_hitbox.overlap_size(&thy_hitbox);
        if result == (0.0, 0.0) {
            None
        } else {
            Some(result)
        }
    }

    /// Determines whether this entity is colliding with some other entity, and if so,
    /// updates this entity with the forces experienced due to the change in velocity
    /// resulting from the collision.
    fn try_collide(&mut self, target: &Entity) -> Option<EuclidianVector> {
        // where are we relative to one another?
        let me_to_you = EuclidianVector::from(self.pos, target.pos).unit();
        let you_to_me = EuclidianVector::from(target.pos, self.pos).unit();

        // are both our trajectories either orthogonal to or in the opposite direction of one-another?
        // if so, then we're NOT moving further into the collision, so there's no velocity changes.
        if self.vel.dot(&me_to_you) <= 0.0 && target.vel.dot(&you_to_me) <= 0.0 {
            return None;
        }

        // what forces do I experience due to velocity changes after colliding?
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
        Some(self.target_vel(resultant_vel.x, resultant_vel.y))
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
