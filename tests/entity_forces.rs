#[cfg(test)]
mod test_entity_vector {
    use ascii_arcade::entity::*;

    fn _make_test_data() -> Vec<Entity> {
        let mut a = Entity::new(EntityType::Npc, (1.0, 1.0));
        let b = Entity::new(EntityType::Npc, (2.0, 1.0));
        a.apply_force(100.0, 0.0);
        vec![a.clone(), b.clone()]
    }

    // Tests to ensure the order of force-application is as expected
    #[test]
    fn test_reaction_force_from_velocity_delta_during_collision() {}

    #[test]
    fn test_reaction_force_from_normal_force() {}

    #[test]
    fn test_reaction_force_from_forces_applied_by_external_bodies() {}
}
