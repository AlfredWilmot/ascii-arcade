use core::{f32, fmt};
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Debug;

pub const BACKGROUND: char = ' ';

const TIME_STEP: f32 = 0.01; // defines the interval of the physics calculation
pub const DEFAULT_WINDOW: (u16, u16) = (50, 10); // defines the viewing area and physical boundary
const MAX_VEL: f32 = 20.0;
const MAX_ACC: f32 = 100.0;

/// defines a vector of entities
pub type Entities = Vec<Entity>;

#[derive(Default, Clone, Debug)]
pub enum EntityState {
    #[default]
    Alive,
    Dead,
    Hidden,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub enum EntityType {
    #[default]
    Npc,
    Player,
    Static,
}

#[derive(Clone, Debug)]
#[readonly::make]
pub struct Entity {
    // used to identiy entities
    pub id: EntityType,

    // these drive physics calculations
    pub pos: (f32, f32),
    pub vel: (f32, f32),
    pub acc: (f32, f32),
    pub mass: f32,
    pub hit_radius: f32,
    force: (f32, f32),

    // these affect both physics calculations and rendering behaviour
    pub state: EntityState,
}

impl Entity {
    pub fn new(id: EntityType, pos: (f32, f32)) -> Entity {
        let mut mass: f32 = 1.0;

        if id == EntityType::Player {
            mass = 1.0;
        }

        Entity {
            id,
            pos,
            mass,
            ..Default::default()
        }
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            id: EntityType::Npc,
            state: EntityState::Alive,
            pos: (0.0, 0.0),
            vel: (0.0, 0.0),
            acc: (0.0, 0.0),
            mass: 1.0,
            force: (0.0, 0.0),
            hit_radius: 0.5,
        }
    }
}

impl Entity {
    /// apply a force vector to the associated entity to affect its acceleration vector
    /// F = m * a
    pub fn apply_force(&mut self, fx: f32, fy: f32) {
        self.force = (self.force.0 + fx, self.force.1 + fy);
    }

    pub fn accelerate(&mut self, ax: f32, ay: f32) {
        self.acc = (ax, ay);
    }

    /// define a set-point velocity that the entity should try to get to
    pub fn target_vel(&mut self, vx: f32, vy: f32) {
        let fx = self.mass * (vx - self.vel.0) / TIME_STEP;
        let fy = self.mass * (vy - self.vel.1) / TIME_STEP;
        self.apply_force(fx, fy);
    }

    /// define a set-point position that the entity should try to get to
    pub fn target_pos(&mut self, x: f32, y: f32) {
        // TODO: calculate the required force that needs to be applied
        // to drive the entity to the desired position
        self.pos = (x, y);
    }

    /// update entity position using motion equations and Newton's 2nd Law:
    /// x1 = x0 + vt + 0.5at^2
    /// v1 = v0 + at
    /// F = m * a
    pub fn update(&mut self) {
        // determine the resultant acceleration from the applied forces
        let ax = self.force.0 / self.mass;
        let ay = self.force.1 / self.mass;
        self.acc = (self.acc.0 + ax, self.acc.1 + ay);

        // determine entity motion
        self.vel.0 += self.acc.0 * TIME_STEP;
        self.vel.1 += self.acc.1 * TIME_STEP;
        self.pos.0 += self.vel.0 * TIME_STEP + 0.5 * self.acc.0 * TIME_STEP * TIME_STEP;
        self.pos.1 += self.vel.1 * TIME_STEP + 0.5 * self.acc.1 * TIME_STEP * TIME_STEP;

        // "consume" the applied forces
        self.force = (0.0, 0.0);

        // reset acceleration for the next physics update
        self.acc = (0.0, 0.0);

        // apply constraints
        apply_constraints(self);
    }
}

/// The way the entity is displayed depends on its state
impl fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // print the ASCII char corresponding to the player's state
        match &self.id {
            EntityType::Player => match &self.state {
                EntityState::Alive => write!(f, "\x1b[42m▀\x1b[0m"),
                EntityState::Dead => write!(f, "\x1b[43m▀\x1b[0m"),
                EntityState::Hidden => write!(f, "{}", BACKGROUND),
            },
            EntityType::Npc => match &self.state {
                EntityState::Alive => write!(f, "\x1b[41m▀\x1b[0m"),
                EntityState::Dead => write!(f, "\x1b[43m▀\x1b[0m"),
                EntityState::Hidden => write!(f, "{}", BACKGROUND),
            },
            EntityType::Static => write!(f, "\x1b[47m \x1b[0m"),
        }
    }
}

#[derive(Clone, Debug)]
/// Represents a collection of entities that are treated as a single larger entity
pub struct RigidBody {
    pub parts: Entities,
}

impl fmt::Display for RigidBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entity in self.parts.iter() {
            write!(f, "{}", entity)?
        }
        Ok(())
    }
}

pub fn apply_constraints(ent: &mut Entity) {
    // limit velocity
    constrain(&mut ent.vel.0, -MAX_VEL, MAX_VEL);
    constrain(&mut ent.vel.1, -MAX_VEL, MAX_VEL);
    // limit acceleration
    constrain(&mut ent.acc.0, -MAX_ACC, MAX_ACC);
    constrain(&mut ent.acc.1, -MAX_ACC, MAX_ACC);

    // limit position to window
    let window = termion::terminal_size().unwrap_or(DEFAULT_WINDOW);

    let mut wall = Entity {
        mass: 1000.0,
        ..Default::default()
    };

    if constrain(&mut ent.pos.0, 0.0_f32, (window.0 - 1) as f32) {
        if ent.pos.0 == 0.0 {
            wall.pos = (ent.pos.0 - ent.hit_radius, ent.pos.1)
        } else {
            wall.pos = (ent.pos.0 + ent.hit_radius, ent.pos.1)
        }
        ent.vel.0 = collision_calc(ent, &wall).0 .0 * 0.5;
    }
    if constrain(&mut ent.pos.1, 0.0_f32, (window.1 - 1) as f32) {
        if ent.pos.1 == 0.0 {
            wall.pos = (ent.pos.0, ent.pos.1 - ent.hit_radius)
        } else {
            wall.pos = (ent.pos.0, ent.pos.1 + ent.hit_radius)
        }
        ent.vel.1 = collision_calc(ent, &wall).0 .1 * 0.2;
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
    false
}

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
