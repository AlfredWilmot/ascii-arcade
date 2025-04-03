#[cfg(test)]
mod tests_angle {
    use ascii_arcade::entity::angles::*;

    /// Builder pattern to validate `get_angle`
    #[must_use]
    struct GetAngleTest {
        expected: Option<f32>,
        origin: (f32, f32),
        coordinate: (f32, f32),
    }

    const Q1: (f32, f32) = (6.66, -6.66); // (top-right)
    const Q2: (f32, f32) = (-6.66, 6.66); // (top-left)
    const Q3: (f32, f32) = (-6.66, 6.66); // (btm-left)
    const Q4: (f32, f32) = (6.66, 6.66); // (btm-right)

    impl GetAngleTest {
        /// Creates a test scenario.
        fn expect(angle: f32) -> Self {
            let origin: (f32, f32) = (0.0, 0.0);
            Self {
                expected: Some(angle),
                origin,
                coordinate: origin,
            }
        }

        /// Specify a new origin.
        fn in_quadrant(mut self, coord: (f32, f32)) -> Self {
            self.origin = coord;
            self.coordinate = coord;
            self
        }

        /// specify x-displacement.
        fn move_x(mut self, x: f32) -> Self {
            self.coordinate.0 += x;
            self
        }

        /// specify y-displacement.
        fn move_y(mut self, y: f32) -> Self {
            self.coordinate.1 += y;
            self
        }

        /// Consume the builder and run the unit-test.
        fn run(self) {
            let actual = get_angle(&self.origin, &self.coordinate);
            assert_eq!(self.expected, actual);
        }
    }

    // ----------------- //
    // TESTING AT ORIGIN //
    // ----------------- //

    #[test]
    fn test_0_deg_at_origin() {
        GetAngleTest::expect(0.0).move_x(1.0).move_y(0.0).run();
    }

    #[test]
    fn test_45_deg_at_origin() {
        GetAngleTest::expect(45.0).move_x(1.0).move_y(-1.0).run();
    }

    #[test]
    fn test_60_deg_at_origin() {
        GetAngleTest::expect(60.0)
            .move_x(1.0)
            .move_y(-f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_90_deg_at_origin() {
        GetAngleTest::expect(90.0).move_x(0.0).move_y(-1.0).run();
    }

    #[test]
    fn test_120_deg_at_origin() {
        GetAngleTest::expect(120.0)
            .move_x(-1.0)
            .move_y(-f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_135_deg_at_origin() {
        GetAngleTest::expect(135.0).move_x(-1.0).move_y(-1.0).run();
    }

    #[test]
    fn test_180_deg_at_origin() {
        GetAngleTest::expect(180.0).move_x(-1.0).move_y(0.0).run();
    }

    #[test]
    fn test_225_deg_at_origin() {
        GetAngleTest::expect(225.0).move_x(-1.0).move_y(1.0).run();
    }

    #[test]
    fn test_240_deg_at_origin() {
        GetAngleTest::expect(240.0)
            .move_x(-1.0)
            .move_y(f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_270_deg_at_origin() {
        GetAngleTest::expect(270.0).move_x(0.0).move_y(1.0).run();
    }

    #[test]
    fn test_300_deg_at_origin() {
        GetAngleTest::expect(300.0)
            .move_x(1.0)
            .move_y(f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_315_deg_at_origin() {
        GetAngleTest::expect(315.0).move_x(1.0).move_y(1.0).run();
    }

    // ---------------------------------- //
    // TESTING AT Q1 (TOP-RIGHT QUADRANT) //
    // ---------------------------------- //

    #[test]
    fn test_0_deg_in_q1() {
        GetAngleTest::expect(0.0)
            .in_quadrant(Q1)
            .move_x(1.0)
            .move_y(0.0)
            .run();
    }

    #[test]
    fn test_45_deg_in_q1() {
        GetAngleTest::expect(45.0)
            .in_quadrant(Q1)
            .move_x(1.0)
            .move_y(-1.0)
            .run();
    }

    #[test]
    fn test_60_deg_in_q1() {
        GetAngleTest::expect(60.0)
            .in_quadrant(Q1)
            .move_x(1.0)
            .move_y(-f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_90_deg_in_q1() {
        GetAngleTest::expect(90.0)
            .in_quadrant(Q1)
            .move_x(0.0)
            .move_y(-1.0)
            .run();
    }

    #[test]
    fn test_120_deg_in_q1() {
        GetAngleTest::expect(120.0)
            .in_quadrant(Q1)
            .move_x(-1.0)
            .move_y(-f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_135_deg_in_q1() {
        GetAngleTest::expect(135.0)
            .in_quadrant(Q1)
            .move_x(-1.0)
            .move_y(-1.0)
            .run();
    }

    #[test]
    fn test_180_deg_in_q1() {
        GetAngleTest::expect(180.0)
            .in_quadrant(Q1)
            .move_x(-1.0)
            .move_y(0.0)
            .run();
    }

    #[test]
    fn test_225_deg_in_q1() {
        GetAngleTest::expect(225.0)
            .in_quadrant(Q1)
            .move_x(-1.0)
            .move_y(1.0)
            .run();
    }

    #[test]
    fn test_240_deg_in_q1() {
        GetAngleTest::expect(240.0)
            .in_quadrant(Q1)
            .move_x(-1.0)
            .move_y(f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_270_deg_in_q1() {
        GetAngleTest::expect(270.0)
            .in_quadrant(Q1)
            .move_x(0.0)
            .move_y(1.0)
            .run();
    }

    #[test]
    fn test_300_deg_in_q1() {
        GetAngleTest::expect(300.0)
            .in_quadrant(Q1)
            .move_x(1.0)
            .move_y(f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_315_deg_in_q1() {
        GetAngleTest::expect(315.0)
            .in_quadrant(Q1)
            .move_x(1.0)
            .move_y(1.0)
            .run();
    }

    // --------------------------------- //
    // TESTING AT Q2 (TOP-LEFT QUADRANT) //
    // --------------------------------- //

    #[test]
    fn test_0_deg_in_q2() {
        GetAngleTest::expect(0.0)
            .in_quadrant(Q2)
            .move_x(1.0)
            .move_y(0.0)
            .run();
    }

    #[test]
    fn test_45_deg_in_q2() {
        GetAngleTest::expect(45.0)
            .in_quadrant(Q2)
            .move_x(1.0)
            .move_y(-1.0)
            .run();
    }

    #[test]
    fn test_60_deg_in_q2() {
        GetAngleTest::expect(60.0)
            .in_quadrant(Q2)
            .move_x(1.0)
            .move_y(-f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_90_deg_in_q2() {
        GetAngleTest::expect(90.0)
            .in_quadrant(Q2)
            .move_x(0.0)
            .move_y(-1.0)
            .run();
    }

    #[test]
    fn test_120_deg_in_q2() {
        GetAngleTest::expect(120.0)
            .in_quadrant(Q2)
            .move_x(-1.0)
            .move_y(-f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_135_deg_in_q2() {
        GetAngleTest::expect(135.0)
            .in_quadrant(Q2)
            .move_x(-1.0)
            .move_y(-1.0)
            .run();
    }

    #[test]
    fn test_180_deg_in_q2() {
        GetAngleTest::expect(180.0)
            .in_quadrant(Q2)
            .move_x(-1.0)
            .move_y(0.0)
            .run();
    }

    #[test]
    fn test_225_deg_in_q2() {
        GetAngleTest::expect(225.0)
            .in_quadrant(Q2)
            .move_x(-1.0)
            .move_y(1.0)
            .run();
    }

    #[test]
    fn test_240_deg_in_q2() {
        GetAngleTest::expect(240.0)
            .in_quadrant(Q2)
            .move_x(-1.0)
            .move_y(f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_270_deg_in_q2() {
        GetAngleTest::expect(270.0)
            .in_quadrant(Q2)
            .move_x(0.0)
            .move_y(1.0)
            .run();
    }

    #[test]
    fn test_300_deg_in_q2() {
        GetAngleTest::expect(300.0)
            .in_quadrant(Q2)
            .move_x(1.0)
            .move_y(f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_315_deg_in_q2() {
        GetAngleTest::expect(315.0)
            .in_quadrant(Q2)
            .move_x(1.0)
            .move_y(1.0)
            .run();
    }

    // --------------------------------- //
    // TESTING AT Q3 (BTM-LEFT QUADRANT) //
    // --------------------------------- //

    #[test]
    fn test_0_deg_in_q3() {
        GetAngleTest::expect(0.0)
            .in_quadrant(Q3)
            .move_x(1.0)
            .move_y(0.0)
            .run();
    }

    #[test]
    fn test_45_deg_in_q3() {
        GetAngleTest::expect(45.0)
            .in_quadrant(Q3)
            .move_x(1.0)
            .move_y(-1.0)
            .run();
    }

    #[test]
    fn test_60_deg_in_q3() {
        GetAngleTest::expect(60.0)
            .in_quadrant(Q3)
            .move_x(1.0)
            .move_y(-f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_90_deg_in_q3() {
        GetAngleTest::expect(90.0)
            .in_quadrant(Q3)
            .move_x(0.0)
            .move_y(-1.0)
            .run();
    }

    #[test]
    fn test_120_deg_in_q3() {
        GetAngleTest::expect(120.0)
            .in_quadrant(Q3)
            .move_x(-1.0)
            .move_y(-f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_135_deg_in_q3() {
        GetAngleTest::expect(135.0)
            .in_quadrant(Q3)
            .move_x(-1.0)
            .move_y(-1.0)
            .run();
    }

    #[test]
    fn test_180_deg_in_q3() {
        GetAngleTest::expect(180.0)
            .in_quadrant(Q3)
            .move_x(-1.0)
            .move_y(0.0)
            .run();
    }

    #[test]
    fn test_225_deg_in_q3() {
        GetAngleTest::expect(225.0)
            .in_quadrant(Q3)
            .move_x(-1.0)
            .move_y(1.0)
            .run();
    }

    #[test]
    fn test_240_deg_in_q3() {
        GetAngleTest::expect(240.0)
            .in_quadrant(Q3)
            .move_x(-1.0)
            .move_y(f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_270_deg_in_q3() {
        GetAngleTest::expect(270.0)
            .in_quadrant(Q3)
            .move_x(0.0)
            .move_y(1.0)
            .run();
    }

    #[test]
    fn test_300_deg_in_q3() {
        GetAngleTest::expect(300.0)
            .in_quadrant(Q3)
            .move_x(1.0)
            .move_y(f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_315_deg_in_q3() {
        GetAngleTest::expect(315.0)
            .in_quadrant(Q3)
            .move_x(1.0)
            .move_y(1.0)
            .run();
    }

    // ---------------------------------- //
    // TESTING AT Q4 (BTM-RIGHT QUADRANT) //
    // ---------------------------------- //

    #[test]
    fn test_0_deg_in_q4() {
        GetAngleTest::expect(0.0)
            .in_quadrant(Q4)
            .move_x(1.0)
            .move_y(0.0)
            .run();
    }

    #[test]
    fn test_45_deg_in_q4() {
        GetAngleTest::expect(45.0)
            .in_quadrant(Q4)
            .move_x(1.0)
            .move_y(-1.0)
            .run();
    }

    #[test]
    fn test_60_deg_in_q4() {
        GetAngleTest::expect(60.0)
            .in_quadrant(Q4)
            .move_x(1.0)
            .move_y(-f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_90_deg_in_q4() {
        GetAngleTest::expect(90.0)
            .in_quadrant(Q4)
            .move_x(0.0)
            .move_y(-1.0)
            .run();
    }

    #[test]
    fn test_120_deg_in_q4() {
        GetAngleTest::expect(120.0)
            .in_quadrant(Q4)
            .move_x(-1.0)
            .move_y(-f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_135_deg_in_q4() {
        GetAngleTest::expect(135.0)
            .in_quadrant(Q4)
            .move_x(-1.0)
            .move_y(-1.0)
            .run();
    }

    #[test]
    fn test_180_deg_in_q4() {
        GetAngleTest::expect(180.0)
            .in_quadrant(Q4)
            .move_x(-1.0)
            .move_y(0.0)
            .run();
    }

    #[test]
    fn test_225_deg_in_q4() {
        GetAngleTest::expect(225.0)
            .in_quadrant(Q4)
            .move_x(-1.0)
            .move_y(1.0)
            .run();
    }

    #[test]
    fn test_240_deg_in_q4() {
        GetAngleTest::expect(240.0)
            .in_quadrant(Q4)
            .move_x(-1.0)
            .move_y(f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_270_deg_in_q4() {
        GetAngleTest::expect(270.0)
            .in_quadrant(Q4)
            .move_x(0.0)
            .move_y(1.0)
            .run();
    }

    #[test]
    fn test_300_deg_in_q4() {
        GetAngleTest::expect(300.0)
            .in_quadrant(Q4)
            .move_x(1.0)
            .move_y(f32::sqrt(3.0))
            .run();
    }

    #[test]
    fn test_315_deg_in_q4() {
        GetAngleTest::expect(315.0)
            .in_quadrant(Q4)
            .move_x(1.0)
            .move_y(1.0)
            .run();
    }
}

#[cfg(test)]
mod tests_rectangle {
    use ascii_arcade::entity::primitives::*;

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
        let rect_no_overlap = Square::new(&(10.0, 10.0), &1.0);

        // is the overlap of each rectangle with the reference one as exptected?
        assert!(rect_ref.overlap(&rect_right).unwrap() == (1.0, 2.0));
        assert!(rect_ref.overlap(&rect_top_right).unwrap() == (1.0, 1.0));
        assert!(rect_ref.overlap(&rect_top).unwrap() == (2.0, 1.0));
        assert!(rect_ref.overlap(&rect_top_left).unwrap() == (1.0, 1.0));
        assert!(rect_ref.overlap(&rect_left).unwrap() == (1.0, 2.0));
        assert!(rect_ref.overlap(&rect_btm_left).unwrap() == (1.0, 1.0));
        assert!(rect_ref.overlap(&rect_btm).unwrap() == (2.0, 1.0));
        assert!(rect_ref.overlap(&rect_btm_right).unwrap() == (1.0, 1.0));
        assert!(rect_ref.overlap(&rect_no_overlap).is_none());
    }
}
