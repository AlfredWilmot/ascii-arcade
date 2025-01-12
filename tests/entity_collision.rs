#[cfg(test)]
mod tests_collision_handling {
    use ascii_arcade::entity::{vector::EuclidianVector, Entity, EntityType};

    #[test]
    fn test_force_api_doesnt_affect_motion_until_update() {
        // arrange
        let mut ent_a = Entity::new(EntityType::Player, (1.0, 1.0));
        let null_vec = EuclidianVector::new(0.0, 0.0);
        let target_vec = EuclidianVector::new(100.0, 100.0);
        //TODO// let expect_pos = ent_a.pos;

        // act
        ent_a.target_vel(target_vec.x, target_vec.y);
        ent_a.target_acc(target_vec.x, target_vec.y);
        //TODO// ent_a.target_pos(100.0, 100.0);
        assert!(ent_a.vel == null_vec);
        assert!(ent_a.acc == null_vec);
        //TODO// assert!(ent_a.pos== expect_pos);
        ent_a.update();

        // assert
        println!("{:?}", ent_a.vel);
        assert!(ent_a.vel != null_vec);
        assert!(ent_a.acc != null_vec);
    }

    #[test]
    fn test_collision_while_moving_towards_static_target_on_the_right() {
        let mut a = Entity::new(EntityType::Npc, (0.0, 0.0));
        let mut b = Entity::new(EntityType::Npc, (1.0, 0.0));

        a.target_vel(10.0, 0.0);
        a.update();
        b.update();
        a.handle_collision(&b);
        b.handle_collision(&a);
        a.update();
        b.update();

        println!("{:?}", b.vel);
        assert!(a.vel.x == 0.0);
        assert!(b.vel.x > 0.0);
    }

    #[test]
    fn test_collision_while_target_moves_towards_us_from_the_right() {
        let mut a = Entity::new(EntityType::Npc, (0.0, 0.0));
        let mut b = Entity::new(EntityType::Npc, (1.0, 0.0));

        b.target_vel(-10.0, 0.0);
        a.update();
        b.update();
        a.handle_collision(&b);
        b.handle_collision(&a);
        a.update();
        b.update();

        println!("{:?}", a.vel);
        assert!(b.vel.x == 0.0);
        assert!(a.vel.x < 0.0);
    }
}
