use crate::entity::Entity;

/// calculate resultant velocity of e1 when colliding with e2, from ...
/// >> conservation of kinetic energy:
/// > > 0.5*m1*v_1a^2 + 0.5*m2*v_2a^2 = 0.5*m1*v_1b^2 + 0.5*m2*v_2b^2
/// >> conservation of momentum :
/// > > m1*v_1a + m2*v_2a = m1*v_1b + m2*v_2b
///
pub fn collision_calc(e1: &Entity, e2: &Entity) -> ((f32, f32), (f32, f32)) {
    let v_1ax = e1.vel.0;
    let v_2ax = e2.vel.0;
    let m1 = e1.mass;

    let v_1ay = e1.vel.1;
    let v_2ay = e2.vel.1;
    let m2 = e2.mass;

    // resulting velocity of e1
    let v_1bx = (v_1ax * (m1 - m2) + 2.0 * m2 * v_2ax) / (m1 + m2);
    let v_1by = (v_1ay * (m1 - m2) + 2.0 * m2 * v_2ay) / (m1 + m2);

    // resulting velocity of e2
    let v_2bx = (v_2ax * (m2 - m1) + 2.0 * m1 * v_1ax) / (m2 + m1);
    let v_2by = (v_2ay * (m2 - m1) + 2.0 * m1 * v_1ay) / (m2 + m1);

    // return the resulting velocities of both entities along both axes
    ((v_1bx, v_1by), (v_2bx, v_2by))
}
