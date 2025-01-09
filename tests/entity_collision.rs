#[cfg(test)]
mod tests_collision_handling {
    use ascii_arcade::entity::{Entity, EntityType};

    #[test]
    fn test_force_api_doesnt_affect_motion_until_update() {
        // arrange
        let mut ent_a = Entity::new(EntityType::Player, (1.0, 1.0));
        let expect_vel = ent_a.vel;
        let expect_acc = ent_a.acc;
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

    #[test]
    fn test_basic_collision_handling() {
        let mut ent_a = Entity::new(EntityType::Player, (1.0, 1.0));
        let mut ent_b = Entity::new(EntityType::Player, (2.0, 1.0));

        // initially there should be no collision
        ent_a.handle_collision(&ent_b);
        ent_b.handle_collision(&ent_a);
        assert!(ent_a.pos == (1.0, 1.0));
        assert!(ent_b.pos == (2.0, 1.0));

        // apply force such that ent_a is moving towards ent_b
        ent_a.target_vel(1.0, 0.0);
        ent_a.handle_collision(&ent_b);
        ent_b.handle_collision(&ent_a);
        ent_a.update();
        ent_b.update();
        assert!(ent_a.vel == (1.0, 0.0));

        // entities should now be colliding
        ent_a.handle_collision(&ent_b);
        ent_b.handle_collision(&ent_a);
        println!("ent_a: {:?}\nent_b: {:?}\n", ent_a, ent_b);
        assert!(ent_a.force.0 < 0.0 && ent_a.force.1 == 0.0);
        assert!(ent_b.force.0 > 0.0 && ent_b.force.1 == 0.0); // BUG: ent_b not experiencing
                                                              // collision force?
        ent_a.update();
        ent_b.update();

        // forces experienced during the collision makes ent_a stop,
        // and ent_b move to the right, respectively
        println!("ent_a: {:?}\nent_b: {:?}\n", ent_a, ent_b);
        assert!(ent_a.vel == (0.0, 0.0));
        assert!(ent_b.vel == (1.0, 0.0));
        //assert!(ent_a.pos == (1.0, 1.0));
        //assert!(ent_b.pos == (2.0, 1.0));
    }
}
