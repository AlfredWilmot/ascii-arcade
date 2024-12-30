use core::{f32, fmt};
use std::collections::HashMap;
use std::fmt::Debug;

pub const BACKGROUND: char = ' ';

/// defines a coord-Entity HashMap for basic spatial queries
pub type EntityMap = HashMap<(u16, u16), Entity>;

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
    InanimateObject,
}

#[derive(Clone, Debug)]
pub struct Entity {
    // TODO: make the prv_loc and cur_loc default to the pos value passed when initialised

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
    /// returns the direction of travel in degrees based on current velocity
    /// 0 deg --> right
    /// 90 deg --> up
    /// 180 deg --> left
    /// 270 deg --> down
    pub fn direction(&self) -> f32 {
        let x = self.vel.0;
        let y = self.vel.1;
        let result = (y/x).atan() * 180.0 / f32::consts::PI;

        if x < 0.0 {
            return result + 180.0;
        } else if x >= 0.0 && y < 0.0 {
            return result + 360.0;
        }
        result
    }

    /// returns the integer coordinates of the entity in space
    pub fn coordinates(&self) -> (u16, u16) {
        let (x, y) = self.pos;
        let x = if x.round() <= 0.0 { 1.0 } else { x };
        let y = if y.round() <= 0.0 { 1.0 } else { y };
        (x.round() as u16, y.round() as u16)
    }
}

/// The way the entity is displayed depends on its state
impl fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // print the ASCII char corresponding to the player's state
        match &self.id {
            EntityType::Player => match &self.state {
                EntityState::Alive => write!(f, "{}", "\x1b[42m▀\x1b[0m"),
                EntityState::Dead=> write!(f, "{}", "\x1b[43m▀\x1b[0m"),
                EntityState::Hidden => write!(f, "{}", BACKGROUND),
            },
            EntityType::Npc => match &self.state {
                EntityState::Alive => write!(f, "{}", "\x1b[41m▀\x1b[0m"),
                EntityState::Dead=> write!(f, "{}", "\x1b[43m▀\x1b[0m"),
                EntityState::Hidden => write!(f, "{}", BACKGROUND),
            },
            EntityType::InanimateObject => write!(f, "{}", BACKGROUND),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_of_travel() {
        let mut a = Entity::default();

        a.vel.0 = 1.0;
        assert!(a.direction() == 0.0);
        a.vel.1 = 1.0;
        assert!(a.direction() == 45.0);
        a.vel.0 = 0.0;
        assert!(a.direction() == 90.0);
        a.vel.0 = -1.0;
        assert!(a.direction() == 135.0);
        a.vel.1 = 0.0;
        assert!(a.direction() == 180.0);
        a.vel.1 = -1.0;
        assert!(a.direction() == 225.0);
        a.vel.0 = 0.0;
        println!("{}",a.direction());
        assert!(a.direction() == 270.0);
        a.vel.0 = 1.0;
        println!("{}",a.direction());
        assert!(a.direction() == 315.0);
    }
}
