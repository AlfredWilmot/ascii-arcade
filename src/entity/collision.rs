use core::f32;

use super::vector::EuclidianVector;
use crate::entity::primitives::Square;
use crate::entity::{Entities, Entity};

/// Applies forces generated due to contact with other entities.
pub fn pairwise(entity: &mut Entity, other_entities: &Entities) {
    entity.grounded = false;

    for other_entity in other_entities {
        if entity.pos == other_entity.pos {
            continue;
        }
        // define inner hitboxes for contact force calculations
        let hitbox_a = Square::new(&entity.pos, &entity.hit_radius);
        let hitbox_b = Square::new(&other_entity.pos, &other_entity.hit_radius);

        // forces applied due to velocity change due to contact with other entity
        if hitbox_a.overlap(&hitbox_b).is_some() {
            if let Some(force) = entity.collision_force(other_entity) {
                entity.grounded = true;
                entity.apply_force(force.x, force.y);
            }
        }
    }
}

impl Entity {
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
