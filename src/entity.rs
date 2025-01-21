pub mod collision;
pub mod collision_geometry;
pub mod vector;

use core::{f32, fmt};
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Debug;

use vector::EuclidianVector;

use crate::scene::debug_print;

pub const BACKGROUND: char = ' ';

const TIME_STEP: f32 = 0.01; // defines the interval of the physics calculation
pub const DEFAULT_WINDOW: (u16, u16) = (50, 10); // defines the viewing area and physical boundary
const MAX_VEL: f32 = 20.0;
const MAX_ACC: f32 = 1000.0;
const MAX_FORCE: f32 = 1000.0;

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
    pub vel: EuclidianVector,
    pub acc: EuclidianVector,
    pub mass: f32,
    pub hit_radius: f32,
    pub force: EuclidianVector,
    pub next_force: EuclidianVector,
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
            vel: EuclidianVector::new(0.0, 0.0),
            acc: EuclidianVector::new(0.0, 0.0),
            mass: 1.0,
            force: EuclidianVector::new(0.0, 0.0),
            next_force: EuclidianVector::new(0.0, 0.0),
            hit_radius: 0.5,
            grounded: false,
        }
    }
}

/// performs force and motion calculations on all the passed entities
pub fn update(entities: &mut Vec<Entity>, debug: bool) {
    // handle additional forces generated due to a collision
    collision::resolve(entities);
    // update motion parameters based on the applied forces
    for entity in entities.iter_mut() {
        if entity.id == EntityType::Static {
            continue;
        }
        if debug && entity.id == EntityType::Player {
            debug_print(format!("force: {:.1?} ", entity.force), 1);
        }
        entity.update();
        if debug && entity.id == EntityType::Player {
            debug_print(format!("pos: {:.1?} ", entity.pos), 2);
            debug_print(format!("vel: {:.1?} ", entity.vel), 3);
            debug_print(format!("acc: {:.1?} ", entity.acc), 4);
        }
    }
}

impl Entity {
    /// apply a force vector to the associated entity to affect its acceleration vector
    /// F = m * a
    pub fn apply_force(&mut self, fx: f32, fy: f32) {
        self.next_force.x += fx;
        self.next_force.y += fy;
        constraint(&mut self.next_force.x, -MAX_FORCE, MAX_FORCE);
        constraint(&mut self.next_force.y, -MAX_FORCE, MAX_FORCE);
    }

    /// define a set-point acceleration that the entity should try to get to
    pub fn target_acc(&mut self, ax: f32, ay: f32) {
        let fx = self.mass * ax;
        let fy = self.mass * ay;
        self.apply_force(fx, fy);
    }

    /// define a set-point velocity that the entity should try to get to
    pub fn target_vel(&mut self, vx: f32, vy: f32) {
        let fx = self.mass * (vx - self.vel.x) / TIME_STEP;
        let fy = self.mass * (vy - self.vel.y) / TIME_STEP;
        self.apply_force(fx, fy);
    }

    /// define a set-point position that the entity should try to get to
    pub fn target_pos(&mut self, x: f32, y: f32) {
        let vx = (self.pos.0 - x) / TIME_STEP;
        let vy = (self.pos.1 - y) / TIME_STEP;
        self.target_vel(vx, vy);
    }

    /// update entity position using motion equations and Newton's 2nd Law:
    /// x1 = x0 + vt + 0.5at^2
    /// v1 = v0 + at
    /// F = m * a
    fn update(&mut self) {
        // determine the resultant acceleration from the applied forces
        // constant force means constant acceleration
        self.acc.x = self.next_force.x / self.mass;
        self.acc.y = self.next_force.y / self.mass;

        // determine entity motion
        // constant velocity means no force is being applied
        self.vel.x += self.acc.x * TIME_STEP;
        self.vel.y += self.acc.y * TIME_STEP;
        self.pos.0 += self.vel.x * TIME_STEP + 0.5 * self.acc.x * TIME_STEP * TIME_STEP;
        self.pos.1 += self.vel.y * TIME_STEP + 0.5 * self.acc.y * TIME_STEP * TIME_STEP;

        // "consume" the applied forces
        self.force = self.next_force.clone();
        self.next_force = EuclidianVector::new(0.0, 0.0);
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
        constraint(&mut self.vel.x, -MAX_VEL, MAX_VEL);
        constraint(&mut self.vel.y, -MAX_VEL, MAX_VEL);
        //
        // limit acceleration
        constraint(&mut self.acc.x, -MAX_ACC, MAX_ACC);
        constraint(&mut self.acc.y, -MAX_ACC, MAX_ACC);
        //
        // limit position to window
        let window = termion::terminal_size().unwrap_or(DEFAULT_WINDOW);
        if constraint(&mut self.pos.0, 0.0_f32, (window.0 - 1) as f32) {
            //
            // simulates a totally inelastic collision along the x-axis
            self.vel.x = 0.0;
            self.apply_force(-self.force.x, 0.0);
            self.grounded = true;
        }
        if constraint(&mut self.pos.1, 0.0_f32, (window.1 - 1) as f32) {
            //
            // simulates a totally inelastic collision along the xyaxis
            self.vel.y = 0.0;
            self.apply_force(0.0, -self.force.y);
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
