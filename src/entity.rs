use crate::physics::collision_calc;
use core::{f32, fmt};
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Debug;

pub const BACKGROUND: char = ' ';

pub const DEFAULT_WINDOW: (u16, u16) = (50, 10); // defines the viewing area and physical boundary
const MAX_VEL: f32 = 20.0;
const MAX_ACC: f32 = 1000.0;

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

    // these affect both physics calculations and rendering behaviour
    pub state: EntityState,
}

impl Entity {
    pub fn new(id: EntityType, pos: (f32, f32)) -> Entity {
        Entity {
            id,
            pos,
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
            hit_radius: 0.5,
        }
    }
}

impl Entity {
    /// apply a force vector to the associated entity to affect its acceleration vector
    /// F = m * a
    pub fn apply_force(&mut self, fx: f32, fy: f32) {
        self.acc = (self.acc.0 + fx / self.mass, self.acc.1 + fy / self.mass)
    }

    /// define a set-point velocity that the entity should try to get to
    pub fn target_vel(&mut self, vx: f32, vy: f32) {
        // TODO: calculate the required force that needs to be applied
        // to drive the entity to the desired velocity
        self.vel = (vx, vy);
    }

    /// define a set-point position that the entity should try to get to
    pub fn target_pos(&mut self, x: f32, y: f32) {
        // TODO: calculate the required force that needs to be applied
        // to drive the entity to the desired position
        self.pos = (x, y);
    }

    /// update entity position using motion equations:
    /// x1 = x0 + vt + 0.5at^2
    /// v1 = v0 + at
    pub fn update(&mut self, dt: f32) {
        // determine entity motion
        self.vel.0 += self.acc.0 * dt;
        self.vel.1 += self.acc.1 * dt;
        self.pos.0 += self.vel.0 * dt + 0.5 * self.acc.0 * dt * dt;
        self.pos.1 += self.vel.1 * dt + 0.5 * self.acc.1 * dt * dt;
        // "consume" the applied forces
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
