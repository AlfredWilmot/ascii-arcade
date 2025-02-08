#[cfg(test)]
mod tests_angle {
    use ascii_arcade::entity::angles::*;

    fn generate_coords_around_point_every_45_deg(
        point: &(f32, f32),
    ) -> Vec<((f32, f32), Option<f32>)> {
        let dx: f32 = 1.0;
        let dy: f32 = 1.0;
        vec![
            ((point.0 + dx, point.1), Some(0.0)),
            ((point.0 + dx, point.1 - dy), Some(45.0)),
            ((point.0, point.1 - dy), Some(90.0)),
            ((point.0 - dx, point.1 - dy), Some(135.0)),
            ((point.0 - dx, point.1), Some(180.0)),
            ((point.0 - dx, point.1 + dy), Some(225.0)),
            ((point.0, point.1 + dy), Some(270.0)),
            ((point.0 + dx, point.1 + dy), Some(315.0)),
        ]
    }

    fn generate_coords_around_point_every_60_deg(
        point: &(f32, f32),
    ) -> Vec<((f32, f32), Option<f32>)> {
        let dx: f32 = 1.0;
        let dy: f32 = f32::sqrt(3.0);
        vec![
            ((point.0 + dx, point.1), Some(0.0)),
            ((point.0 + dx, point.1 - dy), Some(60.0)),
            ((point.0 - dx, point.1 - dy), Some(120.0)),
            ((point.0 - dx, point.1), Some(180.0)),
            ((point.0 - dx, point.1 + dy), Some(240.0)),
            ((point.0 + dx, point.1 + dy), Some(300.0)),
        ]
    }

    #[test]
    fn test_get_angle_returns_correct_value_around_origin_at_regular_rotational_increments() {
        let origins = vec![
            (0.0, 0.0),    // origin
            (6.66, -6.66), // Q1 (top-right)
            (-6.66, 6.66), // Q2 (top-left)
            (-6.66, 6.66), // Q3 (btm-left)
            (6.66, 6.66),  // Q4 (btm-right)
        ];

        for origin in origins {
            // TEST: 45deg increments
            let coords_45deg_step = generate_coords_around_point_every_45_deg(&origin);
            for (coord, expect) in coords_45deg_step {
                let actual = get_angle(&origin, &coord);
                println!(
                    "coord: {:?}, expect: {:?}, actual: {:?}",
                    coord, expect, actual
                );
                assert!(expect == actual);
            }

            // TEST: 60deg increments
            let coords_60deg_step = generate_coords_around_point_every_60_deg(&origin);
            for (coord, expect) in coords_60deg_step {
                let actual = get_angle(&origin, &coord);
                println!(
                    "coord: {:?}, expect: {:?}, actual: {:?}",
                    coord, expect, actual
                );
                assert!(expect == actual);
            }
        }
    }
}

#[cfg(test)]
mod tests_rectangle {
    use ascii_arcade::entity::collision_geometry::*;

    #[test]
    fn test_correct_rectangle_overlap() {
        // all other rectangles are compared against this one
        let rect_ref = Square::new(&(0.0, 0.0), &1.0);

        // rectangles to compare against the reference one
        let rect_right = Square::new(&(1.0, 0.0), &1.0);
        let rect_top_right = Square::new(&(1.0, -1.0), &1.0);
        let rect_top = Square::new(&(0.0, -1.0), &1.0);
        let rect_top_left = Square::new(&(-1.0, -1.0), &1.0);
        let rect_left = Square::new(&(-1.0, 0.0), &1.0);
        let rect_btm_left = Square::new(&(-1.0, 1.0), &1.0);
        let rect_btm = Square::new(&(0.0, 1.0), &1.0);
        let rect_btm_right = Square::new(&(1.0, 1.0), &1.0);

        // is the overlap of each rectangle with the reference one as exptected?
        assert!(rect_ref.overlap_size(&rect_right) == (1.0, 2.0));
        assert!(rect_ref.overlap_size(&rect_top_right) == (1.0, 1.0));
        assert!(rect_ref.overlap_size(&rect_top) == (2.0, 1.0));
        assert!(rect_ref.overlap_size(&rect_top_left) == (1.0, 1.0));
        assert!(rect_ref.overlap_size(&rect_left) == (1.0, 2.0));
        assert!(rect_ref.overlap_size(&rect_btm_left) == (1.0, 1.0));
        assert!(rect_ref.overlap_size(&rect_btm) == (2.0, 1.0));
        assert!(rect_ref.overlap_size(&rect_btm_right) == (1.0, 1.0));
    }
}
