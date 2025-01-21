use core::f32;

use super::vector::EuclidianVector;
use crate::entity::collision_geometry::Square;
use crate::entity::{Entities, Entity, EntityType};

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
        if entities[i].id == EntityType::Static {
            continue;
        }
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
    /// Determines whether this Entity's hitbox is interseting with that of the target
    pub fn colliding(&self, target: &Entity) -> (f32, f32) {
        let my_hitbox = Square::new(&self.pos, &self.hit_radius);
        let thy_hitbox = Square::new(&target.pos, &target.hit_radius);
        my_hitbox.overlap_size(&thy_hitbox)
    }

    /// Determines whether this entity is colliding with some other entity, and if so,
    /// updates this entity with the forces experienced due to the change in velocity
    /// resulting from the collision.
    pub fn handle_collision(&mut self, target: &Entity) {
        //
        // are we even near each other?
        let overlap = self.colliding(target);
        if overlap.0 == 0.0 && overlap.1 == 0.0 {
            return;
        }

        // where are we relative to one another?
        let me_to_you = EuclidianVector::from(self.pos, target.pos).unit();
        let you_to_me = EuclidianVector::from(target.pos, self.pos).unit();

        // we shouldn't be intersecting, so let's adjust for that!
        self.target_pos(
            self.pos.0 + me_to_you.x * overlap.0 * 0.5,
            self.pos.1 - me_to_you.y * overlap.1 * 0.5,
        );

        // are both our trajectories either orthogonal to or in the opposite direction of one-another?
        // if so, then we're NOT moving forther into the collision, so there's no velocity changes.
        if self.vel.dot(&me_to_you) <= 0.0 && target.vel.dot(&you_to_me) <= 0.0 {
            // hey! are you pushing me?!
            if target.force.dot(&you_to_me) > 0.0 {
                let force_directed_at_me = EuclidianVector::new(
                    you_to_me.x.abs() * target.force.x,
                    you_to_me.y.abs() * target.force.y,
                );
                self.apply_force(force_directed_at_me.x, force_directed_at_me.y);
            }
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
