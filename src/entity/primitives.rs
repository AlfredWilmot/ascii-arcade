use core::f32;

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

/// represents a spatial coordinate
struct _Point(f32, f32);

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

    /// returns the (width, height) of the rectangle that would be formed
    /// from the overlapping area between this Square and some other Square.
    pub fn overlap(&self, other: &Square) -> Option<(f32, f32)> {
        let dx = (self.centroid.0 - other.centroid.0).abs();
        let dy = (self.centroid.1 - other.centroid.1).abs();

        let x_overlap = self.apothem + other.apothem - dx;
        let y_overlap = self.apothem + other.apothem - dy;

        if x_overlap <= 0.0 || y_overlap <= 0.0 {
            return None;
        }

        Some((x_overlap, y_overlap))
    }
}
