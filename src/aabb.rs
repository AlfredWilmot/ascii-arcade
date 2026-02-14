//! (2D) Axis-Aligned Bounding Box (AABB)
//!
//! A bounding-volume used as part of a collision broad-phase:
//! i.e. to determine which objects _might_ be colliding.
//! References:
//! - [aabb](https://www.toptal.com/game/video-game-physics-part-ii-collision-detection-for-solid-objects)
//! - [n-body problem](https://youtu.be/nZHjD3cI-EU?si=J8bfy26JqUlDtFlb)
//! - [building collision sims](https://youtu.be/eED4bSkYCB8?si=c0KU_eGInsBPARS2)

use crate::entity::vector::EuclidianVector;

/// Axis-Aligned Bounding-Box (AABB)
#[derive(Debug, Clone)]
struct Aabb {
    min: EuclidianVector, // lower-left corner
    max: EuclidianVector, // upper-right corner
}

pub trait Intersect {
    fn intersect(&self, target: &Self) -> bool;
}

impl Aabb {
    /// Convenience method for generating a new Aabb
    fn new(xmin: f32, ymin: f32, xmax: f32, ymax: f32) -> Self {
        Aabb {
            min: EuclidianVector::new(xmin, ymin),
            max: EuclidianVector::new(xmax, ymax),
        }
    }
}

impl Default for Aabb {
    /// Returns a unit-sized Aabb at the origin by default.
    fn default() -> Self {
        Aabb::new(0.0, 0.0, 1.0, 1.0)
    }
}

impl Intersect for Aabb {
    fn intersect(&self, target: &Self) -> bool {
        let d1x: f32 = target.min.x - self.max.x;
        let d1y: f32 = target.min.y - self.max.y;
        let d2x: f32 = self.min.x - target.max.x;
        let d2y: f32 = self.min.y - target.max.y;

        !((d1x > 0.0 || d1y > 0.0) || (d2x > 0.0 || d2y > 0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[must_use]
    struct IntersectTest {
        expected: bool,
        aabb_1: Aabb,
        aabb_2: Aabb,
    }

    impl IntersectTest {
        /// Creates a test scenario.
        fn expect(result: bool) -> Self {
            Self {
                expected: result,
                aabb_1: Aabb::new(10.0, 10.0, 20.0, 20.0), // <-- Aabb UNDER TEST!
                aabb_2: Aabb::default(),
            }
        }
        /// Inserts a target for the intersection test.
        fn add_box(mut self, aabb: Aabb) -> Self {
            self.aabb_2 = aabb;
            self
        }

        /// Consumes the builder and run the unit-test.
        fn run(self) {
            let actual = self.aabb_1.intersect(&self.aabb_2);
            assert_eq!(self.expected, actual);
        }
    }

    #[test]
    fn test_intersect_top_right() {
        IntersectTest::expect(true)
            .add_box(Aabb::new(15.0, 15.0, 25.0, 25.0))
            .run();
    }
    #[test]
    fn test_intersect_btm_right() {
        IntersectTest::expect(true)
            .add_box(Aabb::new(15.0, 5.0, 25.0, 15.0))
            .run();
    }
    #[test]
    fn test_intersect_top_left() {
        IntersectTest::expect(true)
            .add_box(Aabb::new(5.0, 15.0, 15.0, 25.0))
            .run();
    }
    #[test]
    fn test_intersect_btm_left() {
        IntersectTest::expect(true)
            .add_box(Aabb::new(5.0, 5.0, 15.0, 15.0))
            .run();
    }
    #[test]
    fn test_no_intersect_top_right() {
        IntersectTest::expect(false)
            .add_box(Aabb::new(20.1, 20.1, 25.0, 25.0))
            .run();
    }
    #[test]
    fn test_no_intersect_btm_right() {
        IntersectTest::expect(false)
            .add_box(Aabb::new(15.0, 5.0, 20.1, 9.9))
            .run();
    }
    #[test]
    fn test_no_intersect_top_left() {
        IntersectTest::expect(false)
            .add_box(Aabb::new(5.0, 20.1, 9.9, 25.0))
            .run();
    }
    #[test]
    fn test_no_intersect_btm_left() {
        IntersectTest::expect(false)
            .add_box(Aabb::new(5.0, 5.0, 9.9, 9.9))
            .run();
    }
}
