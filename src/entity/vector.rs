use core::f32;

/// A geometric vector with a magnitude (length) and direction
pub struct EuclidianVector {
    pub x: f32,
    pub y: f32,
}

impl EuclidianVector {
    pub fn new(x: f32, y: f32) -> EuclidianVector {
        EuclidianVector { x, y }
    }

    /// Returns the vector's magnitude
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ascii_arcade::entity::vector::EuclidianVector;
    /// // pythagorean triples: https://en.wikipedia.org/wiki/Pythagorean_triple
    /// assert_eq!(EuclidianVector::new(3.0,4.0).length(), 5.0);
    /// assert_eq!(EuclidianVector::new(5.0, 12.0).length(), 13.0);
    /// assert_eq!(EuclidianVector::new(8.0, 15.0).length(), 17.0);
    /// assert_eq!(EuclidianVector::new(7.0, 24.0).length(), 25.0);
    /// ```
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Returns the vector's equivalent unit-vector
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ascii_arcade::entity::vector::EuclidianVector;
    /// // pythagorean triples: https://en.wikipedia.org/wiki/Pythagorean_triple
    /// let a = EuclidianVector::new(1.0, 1.0).unit();
    /// let expect = 0.5_f32.sqrt();
    /// assert_eq!(a.unit().x.to_string()[0..4], expect.to_string()[0..4]);
    /// assert_eq!(a.unit().y.to_string()[0..4], expect.to_string()[0..4]);
    ///
    /// let a = EuclidianVector::new(0.0, -100.0).unit();
    /// assert_eq!(a.unit().x, 0.0);
    /// assert_eq!(a.unit().y, -1.0);
    /// ```
    pub fn unit(&self) -> EuclidianVector {
        EuclidianVector {
            x: self.x / self.length(),
            y: self.y / self.length(),
        }
    }

    // Returns a scalar that is the extent two vectors are pointing in the same direction
    // (aka "scalar" product)
    //
    // https://www.techtarget.com/whatis/definition/dot-product-scalar-product
    //pub fn dot(&self, other) -> {
    //}
}
