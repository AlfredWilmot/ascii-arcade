use core::f32;

use super::vector::EuclidianVector;
use crate::entity::primitives::Square;
use crate::entity::Entity;

const COLLISION_TRIGGER_VEL: f32 = 10.0;

/// Applies forces generated due to contact with other entities.
pub fn pairwise(entity: &mut Entity, other_entities: &Vec<Entity>) {
    entity.grounded = false;

    let _entity = entity.clone();
    let entity_hitbox = Square::new(&_entity.pos, &_entity.hit_radius);

    // gathers a list of references to entities colliding with the entity under test
    let mut colliders: Vec<&Entity> = Vec::new();

    for other_entity in other_entities {
        if entity.uuid == other_entity.uuid {
            continue;
        }
        // define hitboxes to determine if entitie are colliding
        let other_hitbox = Square::new(&other_entity.pos, &other_entity.hit_radius);

        if entity_hitbox.overlap(&other_hitbox).is_some() {
            colliders.push(other_entity);
        }
    }

    // no colliders detected, exit
    if colliders.is_empty() {
        return;
    }

    // used to average values
    let ratio: f32 = 1.0 / colliders.len() as f32;

    // create a single equivalent collider from all the colliding entities
    let mass_avg: f32 = colliders.iter().map(|e| e.mass).sum();
    let mut pos_avg: (f32, f32) = (0.0, 0.0);
    let mut vel_avg = EuclidianVector::new(0.0, 0.0);
    let mut acc_avg = EuclidianVector::new(0.0, 0.0);

    for collider in &colliders[..] {
        pos_avg = (pos_avg.0 + collider.pos.0, pos_avg.1 + collider.pos.1);
        vel_avg += collider.vel.clone();
        acc_avg += collider.acc.clone();
    }

    let equivalent_single_entity = Entity {
        mass: mass_avg * ratio,
        pos: (pos_avg.0 * ratio, pos_avg.1 * ratio),
        vel: vel_avg * ratio,
        acc: acc_avg * ratio,
        ..Entity::default()
    };

    let other_hitbox = &Square::new(
        &equivalent_single_entity.pos,
        &equivalent_single_entity.hit_radius,
    );
    let me_to_you = EuclidianVector::from(entity.pos, equivalent_single_entity.pos).unit();

    if let Some(overlap) = entity_hitbox.overlap(other_hitbox) {
        entity.grounded = true;

        let collide = entity
            .collision_force(&equivalent_single_entity)
            .unwrap_or(EuclidianVector::new(0.0, 0.0));

        // as an approximation, the direction of overlap can (in general) be treated
        // as orthogonal to the largest overlap side.

        // colliding vertically
        if overlap.0 >= overlap.1 {
            if entity.vel.y.abs() >= COLLISION_TRIGGER_VEL
                || equivalent_single_entity.vel.y.abs() >= COLLISION_TRIGGER_VEL
            {
                entity.apply_force(EuclidianVector::new(0.0, collide.y));
            }
            if me_to_you.dot(&entity.vel) > 0.0 {
                if me_to_you.y > 0.0 {
                    entity.pos.1 -= overlap.1;
                } else {
                    entity.pos.1 += overlap.1;
                }
            }
        }

        // colliding horizontally
        if overlap.1 >= overlap.0 {
            if entity.vel.x.abs() >= COLLISION_TRIGGER_VEL
                || equivalent_single_entity.vel.x.abs() >= COLLISION_TRIGGER_VEL
            {
                entity.apply_force(EuclidianVector::new(collide.x, 0.0));
            }
            if me_to_you.dot(&entity.vel) > 0.0 {
                if me_to_you.x > 0.0 {
                    entity.pos.0 -= overlap.0;
                } else {
                    entity.pos.0 += overlap.0;
                }
            }
        }
    }
}

impl Entity {
    /// Force generated due to velocity changes during a collision.
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
