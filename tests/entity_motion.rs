#[cfg(test)]
mod test_entity_motion {
    use ascii_arcade::entity::{vector::EuclidianVector, *};
    #[test]
    fn test_target_pos_x_same_initial_xy() {
        let initial: (f32, f32) = (5.0, 5.0);
        for i in 1 .. 50 { // test from 1-to-5 in steps of 0.1 with a two-decimal resolution
            let mut entity = Entity::new(EntityType::Npc, initial);
            let i = 100.0 * (i as f32 * 0.1).round() / 100.0;
            let expected = (initial.0 + i as f32, initial.1);
            let force = entity.target_pos(expected.0, expected.1);
            entity.apply_force(force.clone());
            entity.update();
            assert_eq!(entity.pos, expected);
        }
    }
    #[test]
    fn test_target_pos_y_same_initial_xy() {
        let initial: (f32, f32) = (5.0, 2.0);
        for i in 1 .. 50 { // test from 1-to-5 in steps of 0.1 with a two-decimal resolution
            let mut entity = Entity::new(EntityType::Npc, initial);
            let i = 100.0 * (i as f32 * 0.1).round() / 100.0;
            let expected = (initial.0, initial.1 + i as f32);
            let force = entity.target_pos(expected.0, expected.1);
            entity.apply_force(force.clone());
            entity.update();
            assert_eq!(entity.pos, expected);
        }
    }
    #[test]
    fn test_target_pos_x_different_initial_xy() {
        let initial: (f32, f32) = (40.0, 10.0);
        for i in 1 .. 50 { // test from 1-to-5 in steps of 0.1 with a two-decimal resolution
            let mut entity = Entity::new(EntityType::Npc, initial);
            let i = 100.0 * (i as f32 * 0.1).round() / 100.0;
            let expected = (initial.0 - i as f32, initial.1);
            let force = entity.target_pos(expected.0, expected.1);
            entity.apply_force(force.clone());
            entity.update();
            assert_eq!(entity.pos, expected);
        }
    }
    #[test]
    fn test_target_pos_y_different_initial_xy() {
        let initial: (f32, f32) = (10.0, 10.0);
        for i in 1 .. 50 { // test from 1-to-5 in steps of 0.1 with a two-decimal resolution
            let mut entity = Entity::new(EntityType::Npc, initial);
            let i = 100.0 * (i as f32 * 0.1).round() / 100.0;
            let expected = (initial.0 , initial.1 - i as f32);
            let force = entity.target_pos(expected.0, expected.1);
            entity.apply_force(force.clone());
            entity.update();
            assert_eq!(entity.pos, expected);
        }
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
