#[cfg(test)]
mod test_entity_vector {
    use ascii_arcade::entity::{collision, vector::*};

    /// Represents the initial and expected velocity values preceding and following a collision
    #[derive(Debug)]
    struct TestData {
        points: ((f32, f32), (f32, f32)),
        initial_vels: (EuclidianVector, EuclidianVector),
        expect_vels: (EuclidianVector, EuclidianVector),
    }

    impl TestData {
        fn new(
            points: ((f32, f32), (f32, f32)),
            initial: ((f32, f32), (f32, f32)),
            expected: ((f32, f32), (f32, f32)),
        ) -> TestData {
            TestData {
                points,
                initial_vels: (
                    EuclidianVector::new(initial.0 .0, initial.0 .1),
                    EuclidianVector::new(initial.1 .0, initial.1 .1),
                ),
                expect_vels: (
                    EuclidianVector::new(expected.0 .0, expected.0 .1),
                    EuclidianVector::new(expected.1 .0, expected.1 .1),
                ),
            }
        }
    }

    /// Returns a Vec of point + EuclidanVector pairs
    fn make_test_data() -> Vec<TestData> {
        vec![
            // point a is moving right towards a static point b
            TestData::new(
                ((0.0, 0.0), (1.0, 0.0)),
                ((1.0, 0.0), (0.0, 0.0)),
                ((0.0, 0.0), (1.0, 0.0)),
            ),
            // point b is moving left towards a static point a
            TestData::new(
                ((0.0, 0.0), (1.0, 0.0)),
                ((0.0, 0.0), (-1.0, 0.0)),
                ((-1.0, 0.0), (0.0, 0.0)),
            ),
            // point b is moving down towards a static point a
            TestData::new(
                ((0.0, 0.0), (0.0, 1.0)),
                ((0.0, 0.0), (0.0, -1.0)),
                ((0.0, -1.0), (0.0, 0.0)),
            ),
            // point a is moving up towards a static point b
            TestData::new(
                ((0.0, 0.0), (0.0, 1.0)),
                ((0.0, 1.0), (0.0, 0.0)),
                ((0.0, 0.0), (0.0, 1.0)),
            ),
            // two points moveing towards each other along x-axis
            TestData::new(
                ((0.0, 0.0), (1.0, 0.0)),
                ((1.0, 0.0), (-1.0, 0.0)),
                ((-1.0, 0.0), (1.0, 0.0)),
            ),
        ]
    }

    #[test]
    fn test_collision_calc() {
        let ma: f32 = 1.0;
        let mb: f32 = 1.0;

        for data in make_test_data() {
            let a = data.initial_vels.0;
            let b = data.initial_vels.1;

            let new_a_vel = collision::collision_calc(&a, &ma, &b, &mb);
            let new_b_vel = collision::collision_calc(&b, &mb, &a, &ma);

            println!(
                "(a) actual: {:?}, expect: {:?}",
                new_a_vel, data.expect_vels.0
            );
            println!(
                "(b) actual: {:?}, expect: {:?}",
                new_b_vel, data.expect_vels.1
            );
            assert!((new_a_vel.x, new_a_vel.y) == (data.expect_vels.0.x, data.expect_vels.0.y));
            assert!((new_b_vel.x, new_b_vel.y) == (data.expect_vels.1.x, data.expect_vels.1.y));
        }
    }

    #[test]
    fn test_mapping_motion_onto_direction_of_collision() {
        for data in make_test_data() {
            let a_to_b = EuclidianVector::from(data.points.0, data.points.1).unit();
            let b_to_a = EuclidianVector::from(data.points.1, data.points.0).unit();
            println!("a_to_b: {:?}", a_to_b);
            println!("b_to_a: {:?}", b_to_a);

            let a_vel_mod = EuclidianVector::new(
                a_to_b.x.abs() * data.initial_vels.0.x,
                a_to_b.y.abs() * data.initial_vels.0.y,
            );
            let b_vel_mod = EuclidianVector::new(
                b_to_a.x.abs() * data.initial_vels.1.x,
                b_to_a.y.abs() * data.initial_vels.1.y,
            );
            println!("a_vel_mod: {:?}", a_vel_mod);
            println!("b_vel_mod: {:?}", b_vel_mod);

            let new_a_vel = collision::collision_calc(&a_vel_mod, &1.0, &b_vel_mod, &1.0);

            let new_b_vel = collision::collision_calc(&b_vel_mod, &1.0, &a_vel_mod, &1.0);

            println!(
                "(a) initial: {:?}, actual_new: {:?}, expect_new: {:?}",
                data.initial_vels.0, new_a_vel, data.expect_vels.0
            );
            println!(
                "(b) initial: {:?}, actual_new: {:?}, expect_new: {:?}",
                data.initial_vels.1, new_b_vel, data.expect_vels.1
            );
            println!("");
            assert!((new_a_vel.x, new_a_vel.y) == (data.expect_vels.0.x, data.expect_vels.0.y));
            assert!((new_b_vel.x, new_b_vel.y) == (data.expect_vels.1.x, data.expect_vels.1.y));
        }
    }
}
