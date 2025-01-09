pub mod collision;
pub mod collision_geometry;

use core::{f32, fmt};
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Debug;

pub const BACKGROUND: char = ' ';

const TIME_STEP: f32 = 0.01; // defines the interval of the physics calculation
pub const DEFAULT_WINDOW: (u16, u16) = (50, 10); // defines the viewing area and physical boundary
const MAX_VEL: f32 = 20.0;
const MAX_ACC: f32 = 10000.0;

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
    pub force: (f32, f32),
    pub grounded: bool,

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
            grounded: false,
        }
    }
}

impl Entity {
    /// apply a force vector to the associated entity to affect its acceleration vector
    /// F = m * a
    pub fn apply_force(&mut self, fx: f32, fy: f32) {
        self.force = (self.force.0 + fx, self.force.1 + fy);
    }

    /// define a set-point acceleration that the entity should try to get to
    pub fn target_acc(&mut self, ax: f32, ay: f32) {
        let fx = self.mass * ax;
        let fy = self.mass * ay;
        self.apply_force(fx, fy);
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
        // constant force means constant acceleration
        self.acc.0 = self.force.0 / self.mass;
        self.acc.1 = self.force.1 / self.mass;

        // determine entity motion
        // constant velocity means no force is being applied
        self.vel.0 += self.acc.0 * TIME_STEP;
        self.vel.1 += self.acc.1 * TIME_STEP;
        self.pos.0 += self.vel.0 * TIME_STEP + 0.5 * self.acc.0 * TIME_STEP * TIME_STEP;
        self.pos.1 += self.vel.1 * TIME_STEP + 0.5 * self.acc.1 * TIME_STEP * TIME_STEP;

        // "consume" the applied forces
        self.force = (0.0, 0.0);
        self.grounded = false;

        // apply constraints
        self.constrain();
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

impl Entity {
    fn constrain(&mut self) {
        //
        // limit velocity
        constraint(&mut self.vel.0, -MAX_VEL, MAX_VEL);
        constraint(&mut self.vel.1, -MAX_VEL, MAX_VEL);
        //
        // limit acceleration
        constraint(&mut self.acc.0, -MAX_ACC, MAX_ACC);
        constraint(&mut self.acc.1, -MAX_ACC, MAX_ACC);
        //
        // limit position to window
        let window = termion::terminal_size().unwrap_or(DEFAULT_WINDOW);
        if constraint(&mut self.pos.0, 0.0_f32, (window.0 - 1) as f32) {
            //
            // simulates a totally inelastic collision along the x-axis
            self.vel.0 = 0.0;
            self.apply_force(self.force.0, 0.0);
            self.grounded = true;
        }
        if constraint(&mut self.pos.1, 0.0_f32, (window.1 - 1) as f32) {
            //
            // simulates a totally inelastic collision along the xyaxis
            self.vel.1 = 0.0;
            self.apply_force(0.0, self.force.1);
            self.grounded = true;
        }
    }
}

/// Applies constraints to the passed variable.
/// Boolean return value indicates whether-or-not the variable was constrained.
fn constraint<T: PartialEq + PartialOrd>(val: &mut T, lower_limit: T, upper_limit: T) -> bool {
    if *val >= upper_limit {
        *val = upper_limit;
        return true;
    } else if *val <= lower_limit {
        *val = lower_limit;
        return true;
    }
    false
}
