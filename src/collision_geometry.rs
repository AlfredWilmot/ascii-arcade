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

/// returns the angle of the line connecting point a to point b
pub fn get_angle(a: &(f32, f32), b: &(f32, f32)) -> f32 {
    let (xa, ya) = a;
    let (xb, yb) = b;
    let dy = -yb + ya;
    let dx = xb - xa; // reflecting about the x-axis (+y is down, -y is up)

    // convert to degreess
    let result = (dy / dx).atan() * 180.0 / f32::consts::PI;

    if dx < 0.0 {
        return result + 180.0;
    } else if dx >= 0.0 && dy < 0.0 {
        return result + 360.0;
    }
    result
}

// -------------------------------------------------------------------------- //
// --------------- INTERSECTION TESTS FOR DIFFERENT SHAPES ------------------ //
// -------------------------------------------------------------------------- //

pub struct Circles;
pub struct Rectangles;

impl Circles {

    /// returns true if the two circles described by the input parameters are intersecting.
    // in other words: is the separation distance between their centroids,
    // along both x and y axes, less than the sum of their radii?
    pub fn intersecting(a: &(f32, f32), b: &(f32, f32), rad_a: &f32, rad_b: &f32) -> bool {
        let dx = (a.0 - b.0).abs();
        let dy = (a.1 - b.1).abs();
        let r = rad_a + rad_b;

        return dx <= r && dy <= r;
    }

    /// returns length of the overlap between two circles.
    /// the greatest possible value is the radius of the smaller circle,
    /// the smallest possible value is 0.0 if the circles are not intersecting
    pub fn intersect_length(a: &(f32, f32), b: &(f32, f32), rad_a: &f32, rad_b: &f32) -> f32 {
        if Circles::intersecting(a, b, rad_a, rad_b) {
            return 0.0;
        }
        let dx_pow2 = (a.0 - b.0).powi(2);
        let dy_pow2 = (a.1 - b.1).powi(2);
        let centroid_separation_distance = (dx_pow2 + dy_pow2).sqrt();
        return rad_a + rad_b - centroid_separation_distance;
    }
}
