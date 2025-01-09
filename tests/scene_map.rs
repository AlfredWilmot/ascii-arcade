#[cfg(test)]
mod tests_scene_map {

    use ascii_arcade::entity::{Entity, EntityType};
    use ascii_arcade::scene_map::*;

    /// creates a vector of entities whose positions form a rectangular grid
    fn make_entity_grid(i: usize, j: usize) -> Vec<Entity> {
        let mut entities = Vec::new();
        for _i in 1..=i {
            for _j in 1..=j {
                let entity = Entity::new(EntityType::Npc, (_i as f32, _j as f32));
                entities.push(entity);
            }
        }
        entities
    }

    #[test]
    fn test_scene_map_correctly_initializes_from_vector_of_entities() {
        // there are as many map entries as there are entities at unique coordinates
        for i in 1..=8 {
            let entities = make_entity_grid(i, 1);
            let map = make_entity_map(&entities);
            assert!(map.len() == i);
        }
    }

    #[test]
    fn test_remove_returns_correct_value() {
        // create a scene with 2 entities on the grid:
        // (1,1) (2,1)
        let entities = make_entity_grid(2, 1);
        let mut scene: EntityMap = make_entity_map(&entities);

        // if a coordinate has an entity assigned to it, then the Option<> will contain that entity
        if remove(&mut scene, 1, 1).is_none() {
            panic!()
        }
        if remove(&mut scene, 2, 1).is_none() {
            panic!()
        }

        // if a coordinate does not have an entity assigned to it, then the Option<> will contain None
        if remove(&mut scene, 2, 1).is_some() {
            panic!()
        }
    }

    #[test]
    fn test_remove_moves_ownership_of_entity_to_caller() {
        // create a single entity on the scene
        let entities = make_entity_grid(1, 1);
        let mut scene = make_entity_map(&entities);

        // this removes the entity from the scene
        if remove(&mut scene, 1, 1).is_none() {
            panic!()
        }

        // so the same entity should no longer be present
        if remove(&mut scene, 1, 1).is_some() {
            panic!()
        }
    }

    #[test]
    fn test_adjacent_gives_correct_number_of_entities() {
        //create a scene with 9 entities on the grid:
        // (1,1) (2,1) (3,1)
        // (1,2) (2,2) (3,2)
        // (1,3) (2,3) (3,3)
        let entities = make_entity_grid(3, 3);
        let scene: EntityMap = make_entity_map(&entities);
        assert!(scene.len() == 9);
        assert!(adjacent(&scene, 1, 1).len() == 3);
        assert!(adjacent(&scene, 2, 1).len() == 5);
        assert!(adjacent(&scene, 3, 1).len() == 3);
        assert!(adjacent(&scene, 1, 2).len() == 5);
        assert!(adjacent(&scene, 2, 2).len() == 8);
        assert!(adjacent(&scene, 3, 2).len() == 5);
        assert!(adjacent(&scene, 1, 3).len() == 3);
        assert!(adjacent(&scene, 2, 3).len() == 5);
        assert!(adjacent(&scene, 3, 3).len() == 3);
    }

    #[test]
    fn test_moving_an_entity_in_the_scene() {
        //create a scene with 9 entities on the grid, and move on entity to the right:
        // (1,1) (2,1) (3,1)
        // (1,2) (2,2) (3,2) -> (4,2)
        // (1,3) (2,3) (3,3)
        let entities = make_entity_grid(3, 3);
        let mut scene = make_entity_map(&entities);

        // verifying the scene, and that our hero at (3,2) is adjacent to 5 entites
        assert!(scene.len() == 9);
        assert!(adjacent(&scene, 3, 2).len() == 5);

        // extract our hero from the scene, update their position (could be done by a physics
        // calculation, for instance), and then reinsert them into the scene.
        if let Some(mut goblin) = remove(&mut scene, 3, 2) {
            goblin.target_pos(4.0, 2.0);
            insert(&mut scene, goblin);
        }
        // they should now  only be adjacent to two other entities
        assert!(adjacent(&scene, 4, 2).len() == 2);
    }
}
