#[cfg(test)]
mod tests_collision_handling {
    use ascii_arcade::entity::{collision::*, Entity, EntityType};

    #[test]
    fn test_basic_collision_handling() {
        let mut ent_a = Entity::new(EntityType::Player, (1.0, 1.0));
        let mut ent_b = Entity::new(EntityType::Player, (2.0, 1.0));

        // initially there should be no collision
        basic_collision_handling(&mut ent_a, &mut ent_b);
        assert!(ent_a.pos == (1.0, 1.0));
        assert!(ent_b.pos == (2.0, 1.0));

        // apply force such that ent_a is moving towards ent_b
        ent_a.target_vel(1.0, 0.0);
        ent_a.update();
        assert!(ent_a.vel == (1.0, 0.0));
        basic_collision_handling(&mut ent_a, &mut ent_b);
        println!("{:#?}", ent_a);
        assert!(ent_a.force.0 > 0.0 && ent_a.force.1 == 0.0);
        ent_a.update();
        println!("{:#?}", ent_a);
        assert!(ent_a.vel == (0.0, 0.0));
        assert!(ent_b.vel == (1.0, 0.0));
        //assert!(ent_a.pos == (1.0, 1.0));
        //assert!(ent_b.pos == (2.0, 1.0));
    }
}
