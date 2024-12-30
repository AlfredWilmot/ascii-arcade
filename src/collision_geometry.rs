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
