use crate::{entity::Entity, scene};
use std::collections::HashMap;

/// defines a coord-Entity HashMap for basic spatial queries
pub type EntityMap = HashMap<(u16, u16), Entity>;

/// given a hashmap of entities, and a point of interest
/// returns a vector of entites that are around that point
pub fn adjacent(map: &EntityMap, x: u16, y: u16) -> Vec<&Entity> {
    let mut adjacent_entities: Vec<&Entity> = Vec::new();
    let mut adjacent_coords = vec![(x + 1, y), (x + 1, y + 1), (x, y + 1)];

    // ensure we're not trying to access beyond the grid boundary
    if x > 1 {
        adjacent_coords = [adjacent_coords, vec![(x - 1, y), (x - 1, y + 1)]].concat();
    }
    if y > 1 {
        adjacent_coords = [adjacent_coords, vec![(x + 1, y - 1), (x, y - 1)]].concat();
    }
    if x > 1 && y > 1 {
        adjacent_coords = [adjacent_coords, vec![(x - 1, y - 1)]].concat();
    }

    for coord in adjacent_coords {
        if let Some(entity) = map.get(&coord) {
            adjacent_entities.push(entity)
        };
    }
    adjacent_entities
}

/// moves ownership of the entity to the caller, removing it from the scene
/// (https://stackoverflow.com/a/62927919/22415851)
pub fn remove(map: &mut EntityMap, x: u16, y: u16) -> Option<Entity> {
    map.remove(&(x, y))
}

/// moves ownership of the entity from the caller back into the scene
pub fn insert(map: &mut EntityMap, entity: Entity) {
    _ = map.insert(scene::term_coords(entity.pos), entity)
}

/// creates a coordinate-queryable hashmap of entities from a vector of entities
pub fn make_entity_map(entities: &[Entity]) -> EntityMap {
    entities
        .to_owned()
        .clone()
        .into_iter()
        .map(|entity| (scene::term_coords(entity.pos), entity))
        .collect()
}
