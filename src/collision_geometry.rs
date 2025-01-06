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
        if (0.0..22.5).contains(&deg) || (337.5..=360.0).contains(&deg) {
            Some(ORIENTATION::East)
        } else if (22.5..67.5).contains(&deg) {
            Some(ORIENTATION::NorthEast)
        } else if (67.5..112.5).contains(&deg) {
            Some(ORIENTATION::North)
        } else if (112.5..157.5).contains(&deg) {
            Some(ORIENTATION::NorthWest)
        } else if (157.5..202.5).contains(&deg) {
            Some(ORIENTATION::West)
        } else if (202.5..247.5).contains(&deg) {
            Some(ORIENTATION::SouthWest)
        } else if (247.5..292.5).contains(&deg) {
            Some(ORIENTATION::South)
        } else if (292.5..337.5).contains(&deg) {
            Some(ORIENTATION::SouthEast)
        } else {
            None
        }
    }
}

// returns Some(angle) of the line connecting point 'a' to point 'b' in the xy plane.
// returns None if the points are the same.
// e.g.
//
//   * --> (+x)
//   |
//   v         (ax, ay)
//  (+y)     x
//
//                 x
//                  (bx, by)
//
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

    // is the line oriented top-left? (Q2: 90->180)
    // reflect the result around the y-axis
    if xb < xa && yb < ya {
        Some(180.0 - result)
    // is the line oriented bottom-left? (Q3: 180->270)
    // reflect the result around both the x-axis and y-axis
    } else if xb < xa && (yb > ya || dy == 0.0) {
        Some(180.0 + result)
    // is the line oriented bottom-right? (Q4: 270->360)
    // reflect the result around both the x-axis
    } else if xb > xa && yb > ya {
        Some(360.0 - result)
    } else {
        // is the line oriented top-right? (Q1: 0->90)
        // no need to reflect into another Quadrant
        Some(result)
    }
}

/// divides the unit-circle into the specified number of segments
/// and maps the input angle to the closest segment angle.
/// the angles are assumed to be in degrees
/// (NOTE: degrees * 180 / pi = radians)
///
/// # Examples
///
/// ```rust
/// use ascii_arcade::collision_geometry::map_angle;
/// assert_eq!(map_angle(350.0, 8), 0.0);
/// assert_eq!(map_angle(10.0, 8), 0.0);
/// assert_eq!(map_angle(40.0, 8), 45.0);
/// assert_eq!(map_angle(50.0, 8), 45.0);
/// assert_eq!(map_angle(80.0, 8), 90.0);
/// assert_eq!(map_angle(100.0, 8), 90.0);
/// assert_eq!(map_angle(120.0, 8), 135.0);
/// assert_eq!(map_angle(140.0, 8), 135.0);
/// assert_eq!(map_angle(170.0, 8), 180.0);
/// assert_eq!(map_angle(190.0, 8), 180.0);
/// assert_eq!(map_angle(220.0, 8), 225.0);
/// assert_eq!(map_angle(230.0, 8), 225.0);
/// assert_eq!(map_angle(260.0, 8), 270.0);
/// assert_eq!(map_angle(280.0, 8), 270.0);
/// assert_eq!(map_angle(310.0, 8), 315.0);
/// assert_eq!(map_angle(320.0, 8), 315.0);
/// assert_eq!(map_angle(46.0, 4), 90.0);
/// ```
///
pub fn map_angle(angle: f32, segment_count: usize) -> f32 {
    // normalize the angle to fit into one rotation
    let deg = angle % 360.0;

    // determine the size of each segment
    let step = 360.0 / (segment_count as f32 * 2.0);
    let mut current_step = 0.0;

    // map the angle to the nearest segment
    let mut counter: usize = 1;
    while current_step < 360.0 {
        if (current_step..(current_step + step)).contains(&deg) {
            if counter % 2 == 0 {
                if current_step + step == 360.0 {
                    return 0.0;
                }
                return current_step + step;
            }
            return current_step;
        }
        current_step += step;
        counter += 1;
    }

    0.0
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

        dx <= r && dy <= r
    }

    /// returns length of the overlap between two circles.
    /// the greatest possible value is the radius of the smaller circle,
    /// the smallest possible value is 0.0 if the circles are not intersecting
    pub fn overlap_length(&self, other: &Circle) -> f32 {
        if !self.intersects(other) {
            return 0.0;
        }
        let dx_pow2 = (self.centroid.0 - other.centroid.0).powi(2);
        let dy_pow2 = (self.centroid.1 - other.centroid.1).powi(2);
        let centroid_separation_distance = (dx_pow2 + dy_pow2).sqrt();
        self.radius + other.radius - centroid_separation_distance
    }
}

pub struct Square<'a> {
    /// A point that is at the geometric center.
    /// (See http://enwp.org/centroid)
    centroid: &'a (f32, f32),

    /// The Shortest distance from the centroid to a side.
    /// (See http://enwp.org/apothem)
    apothem: &'a f32,
}

impl Square<'_> {
    /// creates a new circle using references to data owned by caller.
    pub fn new<'a>(centroid: &'a (f32, f32), apothem: &'a f32) -> Square<'a> {
        Square { centroid, apothem }
    }

    /// determine whether two rectangles are intersecting.
    pub fn intersects(&self, other: &Square) -> bool {
        let (dx, dy) = self.overlap_size(other);

        if dx == 0.0 || dy == 0.0 {
            return false;
        }
        true
    }

    /// returns the (length, width) of the rectangle that would be formed
    /// from the overlapping area between this Square and some other Square.
    pub fn overlap_size(&self, other: &Square) -> (f32, f32) {
        let dx = (self.centroid.0 - other.centroid.0).abs();
        let dy = (self.centroid.1 - other.centroid.1).abs();

        let x_overlap = self.apothem + other.apothem - dx;
        let y_overlap = self.apothem + other.apothem - dy;

        if x_overlap <= 0.0 || y_overlap <= 0.0 {
            return (0.0, 0.0);
        }

        (x_overlap, y_overlap)
    }
}
