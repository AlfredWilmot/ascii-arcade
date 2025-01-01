use core::f32;

// -------------------------------------------------------------------------- //
// -------------------------------- ANGLES ---------------------------------- //
// -------------------------------------------------------------------------- //

/// defines discrete orientations with respect to a point
#[derive(Debug, PartialEq, Eq)]
pub enum ORIENTATION {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl ORIENTATION {
    /// returns the corresponding ORIENTATION label for a given angle
    pub fn from_angle(deg: &f32) -> Option<ORIENTATION> {
        // normalize the angle to fit into one rotation
        let deg = *deg % 360.0;

        // split the full rotation into 45 degree segements for each of the 8 possible orientations
        let result = if (deg >= 0.0 && deg < 22.5) || (deg > 337.5 && deg <= 360.0) {
            Some(ORIENTATION::East)
        } else if deg >= 22.5 && deg <= 67.5 {
            Some(ORIENTATION::NorthEast)
        } else if deg > 67.5 && deg < 112.5 {
            Some(ORIENTATION::North)
        } else if deg >= 112.5 && deg <= 157.5 {
            Some(ORIENTATION::NorthWest)
        } else if deg > 157.5 && deg < 202.5 {
            Some(ORIENTATION::West)
        } else if deg >= 202.5 && deg <= 247.5 {
            Some(ORIENTATION::SouthWest)
        } else if deg > 247.5 && deg < 292.5 {
            Some(ORIENTATION::South)
        } else if deg >= 292.5 && deg <= 337.5 {
            Some(ORIENTATION::SouthEast)
        } else {
            None
        };
        return result;
    }
}

/// returns Some(angle) of the line connecting point 'a' to point 'b' in the xy plane.
/// returns None if the points are the same.
/// e.g.
///   * --------------> (+x)
///   |
///   |   (ax, ay)
///   |   x
///   |
///   |              x
///   |               (bx, by)
///   v
///  (+y)
///
pub fn get_angle(a: &(f32, f32), b: &(f32, f32)) -> Option<f32> {
    let (xa, ya) = a;
    let (xb, yb) = b;

    // ensure the deltas are only the absolute differences
    let dy = (ya.abs() - yb.abs()).abs();
    let dx = (xa.abs() - xb.abs()).abs();

    let result: f32;

    // point 'a' and 'b are actually the same point,
    // and therefore no angle exists between them.
    if dx == 0.0 && dy == 0.0 {
        return None;
    }

    // the two points make a vertical line
    if dx == 0.0 {
        if yb < ya {
            // yb is "above" ya
            result = 90.0;
        } else {
            // yb is "below" ya
            result = 270.0;
        }
        return Some(result);
    }

    // the two points make a horizontal line
    if dy == 0.0 {
        if xb > xa {
            // xb is "right" of xa
            result = 0.0;
        } else {
            // xb is "left" of xa
            result = 180.0;
        }
        return Some(result);
    }

    // only positive deltas means the atan only returns angles [0, 90[.
    // this initial angle can then be rotated by 90 degree increments as needed.
    result = (dy / dx).atan() * 180.0 / f32::consts::PI;

    if xb < xa && yb < ya {
        // is the line oriented top-left? (Q2: 90->180)
        return Some(result + 90.0);
    } else if xb < xa && (yb > ya || dy == 0.0) {
        // is the line oriented bottom-left? (Q3: 180->270)
        return Some(result + 180.0);
    } else if xb > xa && yb > ya {
        // is the line oriented bottom-right? (Q4: 270->360)
        return Some(result + 270.0);
    } else {
        // is the line oriented top-right? (Q1: 0->90)
        return Some(result);
    }
}

#[cfg(test)]
mod tests_angle {
    use super::*;

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
            (0.0, 0.0), // origin
            (6.66, -6.66), // Q1 (top-right)
            (-6.66, 6.66), // Q2 (top-left)
            (-6.66, 6.66), // Q3 (btm-left)
            (6.66, 6.66), // Q4 (btm-right)
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

// -------------------------------------------------------------------------- //
// --------------- INTERSECTION TESTS FOR DIFFERENT SHAPES ------------------ //
// -------------------------------------------------------------------------- //

pub struct Circle<'a> {
    centroid: &'a (f32, f32),
    radius: &'a f32,
}

impl Circle<'_> {
    /// creates a new circle using references to data owned by caller.
    pub fn new<'a>(centroid: &'a (f32, f32), radius: &'a f32) -> Circle<'a> {
        Circle { centroid, radius }
    }

    /// returns true if the two circles described by the input parameters are intersecting.
    // in other words: is the separation distance between their centroids,
    // along both x and y axes, less than the sum of their radii?
    pub fn intersects(&self, other: &Circle) -> bool {
        let dx = (self.centroid.0 - other.centroid.0).abs();
        let dy = (self.centroid.1 - other.centroid.1).abs();
        let r = self.radius + other.radius;

        return dx <= r && dy <= r;
    }

    /// returns length of the overlap between two circles.
    /// the greatest possible value is the radius of the smaller circle,
    /// the smallest possible value is 0.0 if the circles are not intersecting
    pub fn overlap_length(&self, other: &Circle) -> f32 {
        if !self.intersects(other) {
            return 0.0;
        }
        let dx_pow2 = (self.centroid.0 - other.centroid.0).powi(2);
        let dy_pow2 = (self.centroid.1 - self.centroid.1).powi(2);
        let centroid_separation_distance = (dx_pow2 + dy_pow2).sqrt();
        return self.radius + other.radius - centroid_separation_distance;
    }
}

pub struct Rectangle<'a> {
    /// A point that is at the rectangle's geometric center.
    /// The distance between this point and the left/right sides are identical,
    /// and the distance between this point and the top/bottom sides are identical.
    /// (See http://enwp.org/centroid)
    centroid: &'a (f32, f32),

    /// The Shortest distance from the centroid to the sides of the rectangle.
    /// The first entry is the distance to the sides orthogonal to the x-axis,
    /// and the second entry is the distance to the sides orthogonal to the y-axis.
    /// (See http://enwp.org/apothem)
    apothems: &'a (f32, f32),
}

impl Rectangle<'_> {
    /// creates a new circle using references to data owned by caller.
    pub fn new<'a>(centroid: &'a (f32, f32), apothems: &'a (f32, f32)) -> Rectangle<'a> {
        Rectangle { centroid, apothems }
    }

    /// determine whether two rectangles are intersecting.
    pub fn intersects(&self, other: &Rectangle) -> bool {
        let (dx, dy) = self.overlap_size(&other);

        if dx == 0.0 || dy == 0.0 {
            return false;
        }

        return true;
    }

    /// returns the (length, width) of the Rectangle that would be formed
    /// from the overlapping area between this Rectangle and some other Rectangle.
    pub fn overlap_size(&self, other: &Rectangle) -> (f32, f32) {
        let dx = (self.centroid.0 - other.centroid.0).abs();
        let dy = (self.centroid.1 - other.centroid.1).abs();

        let x_overlap = self.apothems.0 + other.apothems.0 - dx;
        let y_overlap = self.apothems.1 + other.apothems.1 - dy;

        if x_overlap <= 0.0 || y_overlap <= 0.0 {
            return (0.0, 0.0);
        }

        return (x_overlap, y_overlap);
    }
}
#[cfg(test)]
mod tests_rectangle {
    use super::*;

    #[test]
    fn test_correct_rectangle_overlap() {
        // all other rectangles are compared against this one
        let rect_ref = Rectangle::new(&(0.0, 0.0), &(1.0, 1.0));

        // rectangles to compare against the reference one
        let rect_right = Rectangle::new(&(1.0, 0.0), &(1.0, 1.0));
        let rect_top_right = Rectangle::new(&(1.0, -1.0), &(1.0, 1.0));
        let rect_top = Rectangle::new(&(0.0, -1.0), &(1.0, 1.0));
        let rect_top_left = Rectangle::new(&(-1.0, -1.0), &(1.0, 1.0));
        let rect_left = Rectangle::new(&(-1.0, 0.0), &(1.0, 1.0));
        let rect_btm_left = Rectangle::new(&(-1.0, 1.0), &(1.0, 1.0));
        let rect_btm = Rectangle::new(&(0.0, 1.0), &(1.0, 1.0));
        let rect_btm_right = Rectangle::new(&(1.0, 1.0), &(1.0, 1.0));

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
