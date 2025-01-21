#[cfg(test)]
mod test_entity_vector {
    use ascii_arcade::entity::*;

    fn make_test_data() -> Vec<Entity> {
        let mut a = Entity::new(EntityType::Npc, (1.0, 1.0));
        let b = Entity::new(EntityType::Npc, (2.0, 1.0));
        a.apply_force(100.0, 0.0);
        vec![a.clone(), b.clone()]
    }

    #[test]
    fn test_ensure_current_forces_used_from_other_entities_not_their_new_forces() {
        let mut entities = make_test_data();

        update(&mut entities, false);

        let a = &entities[0];
        let b = &entities[1];

        assert!(a.force.x >= 100.0);
        assert!(b.force.x == 0.0);

        update(&mut entities, false);

        let a = &entities[0];
        let b = &entities[1];

        dbg!(&b.force);
        assert!(a.force.x <= -100.0);
        assert!(b.force.x >= 100.0);
    }

    #[test]
    fn test_ensure_my_motion_params_are_updated_by_next_force() {}

    #[test]
    fn test_ensure_your_collision_with_me_uses_my_current_force_not_next_force() {}
}
