use std::{
    f32,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

/// A geometric vector with a magnitude and direction
#[derive(Clone, Debug, PartialEq)]
pub struct EuclidianVector {
    pub x: f32,
    pub y: f32,
}

/// Implement addition operator for EuclidianVectors.
/// <https://doc.rust-lang.org/std/ops/trait.Add.html>
///
/// # Example
///
/// ```rust
/// use ascii_arcade::entity::vector::EuclidianVector;
/// let a = EuclidianVector::new(1.0, 2.0);
/// let b = EuclidianVector::new(1.0, -2.0);
/// let c: EuclidianVector = a + b;
/// assert!(c.x == 2.0);
/// assert!(c.y == 0.0);
/// ```
impl Add<EuclidianVector> for EuclidianVector {
    type Output = Self;
    fn add(self, rhs: EuclidianVector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Implement subtraction operator for EuclidianVectors.
/// <https://doc.rust-lang.org/std/ops/trait.Sub.html>
///
/// # Example
///
/// ```rust
/// use ascii_arcade::entity::vector::EuclidianVector;
/// let a = EuclidianVector::new(1.0, 2.0);
/// let b = EuclidianVector::new(1.0, -2.0);
/// let c: EuclidianVector = a - b;
/// assert!(c.x == 0.0);
/// assert!(c.y == 4.0);
/// ```
impl Sub<EuclidianVector> for EuclidianVector {
    type Output = Self;
    fn sub(self, rhs: EuclidianVector) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Implement addition assignment operator for EuclidianVectors.
/// <https://doc.rust-lang.org/std/ops/trait.AddAssign.html>
///
/// # Example
///
/// ```rust
/// use ascii_arcade::entity::vector::EuclidianVector;
/// let mut a = EuclidianVector::new(1.0, 2.0);
/// let b = EuclidianVector::new(1.0, -2.0);
/// a += b;
/// assert!(a.x == 2.0);
/// assert!(a.y == 0.0);
/// ```
impl AddAssign for EuclidianVector {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

/// Implement subtraction assignment operator for EuclidianVectors.
/// <https://doc.rust-lang.org/std/ops/trait.SubAssign.html>
///
/// # Example
///
/// ```rust
/// use ascii_arcade::entity::vector::EuclidianVector;
/// let mut a = EuclidianVector::new(1.0, 2.0);
/// let b = EuclidianVector::new(1.0, -2.0);
/// a -= b;
/// assert!(a.x == 0.0);
/// assert!(a.y == 4.0);
/// ```
impl SubAssign for EuclidianVector {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

/// Returns a vector with fields multiplied by the float for '*'
/// NOTE: the float must be on the rhs (i.e euclid_vector * f32_scalar)
///
/// # Example
///
/// ```rust
/// use ascii_arcade::entity::vector::EuclidianVector;
/// let a = EuclidianVector::new(2.0, -2.0);
/// let b = a * 10.0;
/// assert!(b.x == 20.0);
/// assert!(b.y == -20.0);
/// ```
impl Mul<f32> for EuclidianVector {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

impl MulAssign<f32> for EuclidianVector {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl EuclidianVector {
    pub fn new(x: f32, y: f32) -> EuclidianVector {
        EuclidianVector { x, y }
    }

    /// Returns the vector connecting two points, directed from the first to the second.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ascii_arcade::entity::vector::EuclidianVector;
    ///
    /// // vector from points in Q1
    /// let a: (f32, f32) = (2.0, 2.0);
    /// let b: (f32, f32) = (5.0, 6.0);
    /// let c = EuclidianVector::from(a, b);
    /// assert!((c.x, c.y) == (3.0, 4.0));
    /// let c = EuclidianVector::from(b, a);
    /// assert!((c.x, c.y) == (-3.0, -4.0));
    ///
    /// // vector from points in Q3
    /// let a: (f32, f32) = (-2.0, -2.0);
    /// let b: (f32, f32) = (-5.0, -6.0);
    /// let c = EuclidianVector::from(a, b);
    /// assert!((c.x, c.y) == (-3.0, -4.0));
    /// let c = EuclidianVector::from(b, a);
    /// assert!((c.x, c.y) == (3.0, 4.0));
    /// ```
    pub fn from(point_a: (f32, f32), point_b: (f32, f32)) -> EuclidianVector {
        EuclidianVector {
            x: point_b.0 - point_a.0,
            y: point_b.1 - point_a.1,
        }
    }

    /// Returns the vector's magnitude
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ascii_arcade::entity::vector::EuclidianVector;
    /// // pythagorean triples: https://en.wikipedia.org/wiki/Pythagorean_triple
    /// assert_eq!(EuclidianVector::new(3.0,4.0).magnitude(), 5.0);
    /// assert_eq!(EuclidianVector::new(5.0, 12.0).magnitude(), 13.0);
    /// assert_eq!(EuclidianVector::new(8.0, 15.0).magnitude(), 17.0);
    /// assert_eq!(EuclidianVector::new(7.0, 24.0).magnitude(), 25.0);
    /// ```
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
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
        if self.magnitude() <= 0.0 {
            EuclidianVector { x: 0.0, y: 0.0 }
        } else {
            EuclidianVector {
                x: self.x / self.magnitude(),
                y: self.y / self.magnitude(),
            }
        }
    }

    /// Returns a scalar that is the extent two vectors are pointing in the same direction
    /// (aka "scalar" product)
    ///
    /// [What is dot-product?](https://www.techtarget.com/whatis/definition/dot-product-scalar-product)
    ///
    /// # Example: dot-product of unit-vectors equals their magnitude if the vectors are colinear
    ///
    /// ```rust
    /// use ascii_arcade::entity::vector::EuclidianVector;
    ///
    /// let a = EuclidianVector::new(0.0, 1.0).unit();
    /// let b = EuclidianVector::new(0.0, 1.0).unit();
    /// assert_eq!(a.dot(&b), a.magnitude());
    ///
    /// let a = EuclidianVector::new(1.0, 0.0).unit();
    /// let b = EuclidianVector::new(0.0, 1.0).unit();
    /// assert_eq!(a.dot(&b), 0.0);
    ///
    /// let a = EuclidianVector::new(0.0, -1.0).unit();
    /// let b = EuclidianVector::new(0.0, 1.0).unit();
    /// assert_eq!(a.dot(&b), -a.magnitude());
    ///
    /// let a = EuclidianVector::new(-1.0, 0.0).unit();
    /// let b = EuclidianVector::new(-1.0, 0.0).unit();
    /// assert_eq!(a.dot(&b), a.magnitude());
    ///
    /// let a = EuclidianVector::new(1.0, 1.0).unit();
    /// let b = EuclidianVector::new(1.0, 1.0).unit();
    /// assert_eq!(a.dot(&b), a.magnitude());
    /// ```
    pub fn dot(&self, other: &EuclidianVector) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }
}
