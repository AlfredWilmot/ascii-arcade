#[cfg(test)]
mod tests_collision_handling {
    use ascii_arcade::entity::{Entity, EntityType};

    #[test]
    fn test_force_api_doesnt_affect_motion_until_update() {
        // arrange
        let mut ent_a = Entity::new(EntityType::Player, (1.0, 1.0));
        let expect_vel = ent_a.vel.clone();
        let expect_acc = ent_a.acc.clone();
        //TODO// let expect_pos = ent_a.pos;

        // act
        ent_a.target_vel(100.0, 100.0);
        ent_a.target_acc(100.0, 100.0);
        //TODO// ent_a.target_pos(100.0, 100.0);

        // assert
        assert!(ent_a.vel == expect_vel);
        assert!(ent_a.acc == expect_acc);
        //TODO// assert!(ent_a.pos== expect_pos);
    }
}
