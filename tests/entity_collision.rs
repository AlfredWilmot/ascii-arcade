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
}
