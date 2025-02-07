pub mod collision;
pub mod collision_geometry;
pub mod vector;

use core::{f32, fmt};
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::sync::LazyLock;

use vector::EuclidianVector;

pub const BACKGROUND: char = ' ';

const TIME_STEP: f32 = 0.01; // defines the interval of the physics calculation
pub const DEFAULT_WINDOW: (u16, u16) = (50, 10); // defines the viewing area and physical boundary
const MAX_VEL: f32 = 20.0;
const MAX_ACC: f32 = 1_000.0;
const MAX_FORCE: f32 = 5_000.0;
const _MAX_MASS: f32 = 1_000.0;

// initialise the window boundary at once runtime by checking the size of the terminal
static WINDOW: LazyLock<(u16, u16)> =
    LazyLock::new(|| termion::terminal_size().unwrap_or(DEFAULT_WINDOW));

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
    // these affect both physics calculations and rendering behaviour
    pub id: EntityType,
    pub state: EntityState,

    // these drive physics calculations (unlikely to change much)
    pub pos: (f32, f32),
    pub vel: EuclidianVector,
    pub acc: EuclidianVector,
    pub mass: f32,
    pub hit_radius: f32,

    // forces applied to and exerted by entity
    pub input_force: EuclidianVector,
    pub reaction_force: EuclidianVector,

    // misc fields (subject to imminent change)
    pub grounded: bool,
}

impl Entity {
    pub fn new(id: EntityType, pos: (f32, f32)) -> Entity {
        let mass: f32 = match id {
            EntityType::Player => 1.0,
            EntityType::Npc => 1.0,
            EntityType::Static => 1.0,
        };

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
            input_force: EuclidianVector::new(0.0, 0.0),
            reaction_force: EuclidianVector::new(0.0, 0.0),
            hit_radius: 0.5,
            grounded: false,
        }
    }
}

/// performs force and motion calculations on all the passed entities
pub fn update(entities_then: &Vec<Entity>, entities_now: &mut [Entity]) {
    // update motion parameters based on the applied forces
    for entity in entities_now.iter_mut() {
        // handle additional forces generated due to a collision
        collision::pairwise(entity, entities_then);

        if entity.id == EntityType::Static {
            continue;
        }
        entity.update();
    }
}

impl Entity {
    /// apply a force vector to the associated entity to affect its
    /// acceleration vector on the next update (F = m * a)
    pub fn apply_force(&mut self, fx: f32, fy: f32) {
        self.input_force.x += fx;
        self.input_force.y += fy;
        constraint(&mut self.input_force.x, -MAX_FORCE, MAX_FORCE);
        constraint(&mut self.input_force.y, -MAX_FORCE, MAX_FORCE);
    }

    /// returns the force required to drive the entity to the target acceleration
    pub fn target_acc(&mut self, ax: f32, ay: f32) -> EuclidianVector {
        EuclidianVector::new(self.mass * ax, self.mass * ay)
    }

    /// returns the force required to drive the entity to the target velocity
    pub fn target_vel(&mut self, vx: f32, vy: f32) -> EuclidianVector {
        EuclidianVector::new(
            self.mass * (vx - self.vel.x) / TIME_STEP,
            self.mass * (vy - self.vel.y) / TIME_STEP,
        )
    }

    /// returns the force required to drive the entity to the target position
    pub fn target_pos(&mut self, x: f32, y: f32) -> EuclidianVector {
        self.target_vel((self.pos.0 - x) / TIME_STEP, (self.pos.1 - y) / TIME_STEP)
    }

    /// update entity position using motion equations and Newton's 2nd Law:
    /// x1 = x0 + vt + 0.5at^2
    /// v1 = v0 + at
    /// F = m * a
    fn update(&mut self) {
        // determine the resultant acceleration from the applied forces
        // constant force means constant acceleration
        self.acc.x = self.input_force.x / self.mass;
        self.acc.y = self.input_force.y / self.mass;

        // determine entity motion
        // constant velocity means no force is being applied
        self.vel.x += self.acc.x * TIME_STEP;
        self.vel.y += self.acc.y * TIME_STEP;
        self.pos.0 += self.vel.x * TIME_STEP + 0.5 * self.acc.x * TIME_STEP * TIME_STEP;
        self.pos.1 += self.vel.y * TIME_STEP + 0.5 * self.acc.y * TIME_STEP * TIME_STEP;

        // "consume" the applied forces
        self.input_force = EuclidianVector::new(0.0, 0.0);

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
        if constraint(&mut self.pos.0, 1.0_f32, (WINDOW.0 - 1) as f32) {
            self.vel.x = 0.0;
        }
        if constraint(&mut self.pos.1, 1.0_f32, (WINDOW.1 - 1) as f32) {
            self.vel.y = 0.0;
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
