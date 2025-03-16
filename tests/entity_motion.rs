#[cfg(test)]
mod test_entity_motion {
    use ascii_arcade::entity::{vector::EuclidianVector, *};
    #[test]
    fn test_target_pos() {
        let mut entity = Entity::new(EntityType::Npc, (2.0, 2.0));
        let expect: (f32, f32) = (30.0, 2.0);
        let force = entity.target_pos(expect.0, expect.1);
        entity.apply_force(force);
        entity.update();

        assert_eq!(entity.pos, expect);
    }
    #[test]
    fn test_target_vel() {
        let mut entity = Entity::new(EntityType::Npc, (2.0, 2.0));
        let expect = EuclidianVector::new(10.0, 0.0);
        let force = entity.target_vel(expect.x, expect.y);
        entity.apply_force(force);
        entity.update();

        assert_eq!(entity.vel, expect);
    }
}
