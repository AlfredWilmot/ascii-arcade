use crate::entity;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;

use entity::Entity;

pub const WINDOW: (u16, u16) = (50, 10); // defines the viewing area and physical boundary
const MAX_VEL: f32 = 20.0;
const MAX_ACC: f32 = 1000.0;


/// update player position using motion equations:
/// x1 = x0 + vt + 0.5at^2
/// v1 = v0 + at
pub fn update(ent: &mut Entity, dt: f32) {
    ent.vel.0 += ent.acc.0 * dt;
    ent.vel.1 += ent.acc.1 * dt;
    ent.pos.0 += ent.vel.0 * dt + 0.5 * ent.acc.0 * dt * dt;
    ent.pos.1 += ent.vel.1 * dt + 0.5 * ent.acc.1 * dt * dt;
}

/// calculate resultant velocity of e1 when colliding with e2, from ...
/// >> conservation of kinetic energy:
///     0.5*m1*v_1a^2 + 0.5*m2*v_2a^2 = 0.5*m1*v_1b^2 + 0.5*m2*v_2b^2
/// >> conservation of momentum :
///     m1*v_1a + m2*v_2a = m1*v_1b + m2*v_2b
///
pub fn collision_calc(e1: &Entity, e2: &Entity) -> ((f32, f32), (f32, f32)){
    let v_1ax = e1.vel.0;
    let v_2ax = e2.vel.0;
    let m1 = e1.mass;

    let v_1ay = e1.vel.1;
    let v_2ay = e2.vel.1;
    let m2 = e2.mass;

    // resulting velocity of e1
    let v_1bx = (v_1ax*(m1-m2)+2.0*m2*v_2ax)/(m1+m2);
    let v_1by = (v_1ay*(m1-m2)+2.0*m2*v_2ay)/(m1+m2);

    // resulting velocity of e2
    let v_2bx = (v_2ax*(m2-m1)+2.0*m1*v_1ax)/(m2+m1);
    let v_2by = (v_2ay*(m2-m1)+2.0*m1*v_1ay)/(m2+m1);

    // return the resulting velocities of both entities along both axes
    ((v_1bx, v_1by), (v_2bx, v_2by))

}

pub fn apply_constraints(ent: &mut Entity) {
    // limit velocity
    constrain(&mut ent.vel.0, -MAX_VEL, MAX_VEL);
    constrain(&mut ent.vel.1, -MAX_VEL, MAX_VEL);
    // limit acceleration
    constrain(&mut ent.acc.0, -MAX_ACC, MAX_ACC);
    constrain(&mut ent.acc.1, -MAX_ACC, MAX_ACC);

    // limit position to window
    let window = termion::terminal_size().unwrap_or(WINDOW);


    let mut wall = Entity::default();
    wall.mass = 1000.0;

    if constrain(&mut ent.pos.0, 0.0 as f32, (window.0-1) as f32) {
        if ent.pos.0 == 0.0 {
            wall.pos = (ent.pos.0-ent.hit_radius, ent.pos.1)
        } else {
            wall.pos = (ent.pos.0+ent.hit_radius, ent.pos.1)
        }
        ent.vel.0 = collision_calc(&ent, &wall).0.0*0.5;
    }
    if constrain(&mut ent.pos.1, 0.0 as f32, (window.1-1) as f32) {
        if ent.pos.1 == 0.0 {
            wall.pos = (ent.pos.0, ent.pos.1-ent.hit_radius)
        } else {
            wall.pos = (ent.pos.0, ent.pos.1+ent.hit_radius)
        }
        ent.vel.1 = collision_calc(&ent, &wall).0.1*0.2;
    }
}

/// Applies constraints to the passed variable.
/// Boolean return value indicates whether-or-not the variable was constrained.
fn constrain<T: PartialEq + PartialOrd>(val: &mut T, lower_limit: T, upper_limit: T) -> bool {
    if *val >= upper_limit {
        *val = upper_limit;
        return true;
    } else if *val <= lower_limit {
        *val = lower_limit;
        return true;
    }
    return false;
}
