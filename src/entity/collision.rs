use core::f32;

use super::vector::EuclidianVector;
use crate::entity::primitives::Square;
use crate::entity::{Entities, Entity};

/// Applies forces generated due to contact with other entities.
pub fn pairwise(this_entity: &mut Entity, other_entities: &Entities) {
    for that_entity in other_entities {
        // define hitboxes
        let this_inner_hitbox = Square::new(&this_entity.pos, &this_entity.hit_radius);
        let that_inner_hitbox = Square::new(&that_entity.pos, &that_entity.hit_radius);
        //let this_outer_hitbox = Square::new(&this_entity.pos, &(this_entity.hit_radius*1.5));
        //let that_outer_hitbox = Square::new(&that_entity.pos, &(that_entity.hit_radius*1.5));

        if this_inner_hitbox.overlap(&that_inner_hitbox).is_some() {
            // velocity change force due to the current encounter
            if let Some(force) = this_entity.collision_force(that_entity) {
                this_entity.apply_force(force.x, force.y);
            }
        }

        // position change force due to the current encounter
        //if let Some(force) = entity_under_test.sticking_force(entity_to_compare) {
        //    entity_under_test.apply_force(force.x, force.y);
        //}
    }
}

impl Entity {
    /// Apply a displacement force based on degree of overlap, and relative position, between self and target
    fn _sticking_force(&mut self, target: &Entity) -> Option<EuclidianVector> {
        let _me_to_you = EuclidianVector::from(self.pos, target.pos).unit();
        let _inner_hitbox: f32 = self.hit_radius;

        None
    }

    /// Determines whether this entity is colliding with some other entity, and if so,
    /// updates this entity with the forces experienced due to the change in velocity
    /// resulting from the collision.
    fn collision_force(&mut self, target: &Entity) -> Option<EuclidianVector> {
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
