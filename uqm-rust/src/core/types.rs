//! Core type definitions
//!
//! This module defines the fundamental types used throughout the game.
//! These types provide type safety and prevent common bugs through Rust's
//! strong type system.

use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

/// A 2D point with integer coordinates
///
/// Used for positions in the galaxy map and UI coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    /// X coordinate
    pub x: i16,
    /// Y coordinate
    pub y: i16,
}

impl Point {
    /// Create a new point
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Point;
    /// let point = Point::new(100, 200);
    /// assert_eq!(point.x, 100);
    /// assert_eq!(point.y, 200);
    /// ```
    #[must_use]
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    /// Origin point (0, 0)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Point;
    /// let origin = Point::zero();
    /// assert_eq!(origin.x, 0);
    /// assert_eq!(origin.y, 0);
    /// ```
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0, 0)
    }

    /// Calculate Euclidean distance to another point
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Point;
    /// let p1 = Point::new(0, 0);
    /// let p2 = Point::new(3, 4);
    /// assert!((p1.distance_to(p2) - 5.0).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn distance_to(self, other: Self) -> f32 {
        let dx = f32::from(self.x - other.x);
        let dy = f32::from(self.y - other.y);
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate squared distance to another point (faster, no sqrt)
    ///
    /// Useful for distance comparisons without needing the exact distance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Point;
    /// let p1 = Point::new(0, 0);
    /// let p2 = Point::new(3, 4);
    /// assert_eq!(p1.distance_squared(p2), 25);
    /// ```
    #[must_use]
    pub fn distance_squared(self, other: Self) -> i32 {
        let dx = i32::from(self.x - other.x);
        let dy = i32::from(self.y - other.y);
        dx * dx + dy * dy
    }

    /// Calculate Manhattan distance to another point
    ///
    /// Manhattan distance (also called taxicab distance) is the sum of the
    /// absolute differences of coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Point;
    /// let p1 = Point::new(0, 0);
    /// let p2 = Point::new(3, 4);
    /// assert_eq!(p1.manhattan_distance(p2), 7);
    /// ```
    #[must_use]
    pub fn manhattan_distance(self, other: Self) -> i16 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Calculate the midpoint between this point and another
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Point;
    /// let p1 = Point::new(0, 0);
    /// let p2 = Point::new(10, 20);
    /// let mid = p1.midpoint(p2);
    /// assert_eq!(mid, Point::new(5, 10));
    /// ```
    #[must_use]
    pub fn midpoint(self, other: Self) -> Self {
        Self {
            x: (self.x + other.x) / 2,
            y: (self.y + other.y) / 2,
        }
    }

    /// Linearly interpolate between this point and another
    ///
    /// # Arguments
    ///
    /// * `other` - The target point
    /// * `t` - Interpolation factor (0.0 = self, 1.0 = other)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Point;
    /// let start = Point::new(0, 0);
    /// let end = Point::new(100, 100);
    /// let mid = start.lerp(end, 0.5);
    /// assert_eq!(mid, Point::new(50, 50));
    /// ```
    #[must_use]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + ((f32::from(other.x - self.x) * t) as i16),
            y: self.y + ((f32::from(other.y - self.y) * t) as i16),
        }
    }

    /// Scale this point by a factor
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Point;
    /// let p = Point::new(10, 20);
    /// let scaled = p.scale(2.0);
    /// assert_eq!(scaled, Point::new(20, 40));
    /// ```
    #[must_use]
    pub fn scale(self, factor: f32) -> Self {
        Self {
            x: (f32::from(self.x) * factor) as i16,
            y: (f32::from(self.y) * factor) as i16,
        }
    }

    /// Clamp this point to be within a rectangular bounds
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum bounds (inclusive)
    /// * `max` - Maximum bounds (inclusive)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Point;
    /// let p = Point::new(150, 250);
    /// let clamped = p.clamp(Point::new(0, 0), Point::new(100, 200));
    /// assert_eq!(clamped, Point::new(100, 200));
    /// ```
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }

    /// Convert to floating-point Vec2
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::{Point, Vec2};
    /// let p = Point::new(10, 20);
    /// let v = p.to_vec2();
    /// assert_eq!(v, Vec2::new(10.0, 20.0));
    /// ```
    #[must_use]
    pub fn to_vec2(self) -> Vec2 {
        Vec2::new(f32::from(self.x), f32::from(self.y))
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// A 2D vector with floating-point coordinates
///
/// Used for physics calculations, velocities, and smooth interpolation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    /// X component
    pub x: f32,
    /// Y component
    pub y: f32,
}

impl Vec2 {
    /// Create a new vector
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// assert_eq!(v.x, 3.0);
    /// assert_eq!(v.y, 4.0);
    /// ```
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Zero vector (0, 0)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::zero();
    /// assert_eq!(v.x, 0.0);
    /// assert_eq!(v.y, 0.0);
    /// ```
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Unit vector pointing up (0, -1) - negative Y is up in screen coordinates
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::up();
    /// assert_eq!(v, Vec2::new(0.0, -1.0));
    /// ```
    #[must_use]
    pub const fn up() -> Self {
        Self::new(0.0, -1.0)
    }

    /// Unit vector pointing down (0, 1)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::down();
    /// assert_eq!(v, Vec2::new(0.0, 1.0));
    /// ```
    #[must_use]
    pub const fn down() -> Self {
        Self::new(0.0, 1.0)
    }

    /// Unit vector pointing right (1, 0)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::right();
    /// assert_eq!(v, Vec2::new(1.0, 0.0));
    /// ```
    #[must_use]
    pub const fn right() -> Self {
        Self::new(1.0, 0.0)
    }

    /// Unit vector pointing left (-1, 0)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::left();
    /// assert_eq!(v, Vec2::new(-1.0, 0.0));
    /// ```
    #[must_use]
    pub const fn left() -> Self {
        Self::new(-1.0, 0.0)
    }

    /// Calculate the magnitude (length) of this vector
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// assert!((v.magnitude() - 5.0).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Calculate the squared magnitude (avoids sqrt for performance)
    ///
    /// Useful for distance comparisons without needing exact magnitude.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// assert!((v.magnitude_squared() - 25.0).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn magnitude_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Normalize this vector to unit length
    ///
    /// Returns zero vector if magnitude is zero or very small.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// let normalized = v.normalize();
    /// assert!((normalized.magnitude() - 1.0).abs() < 0.001);
    /// assert!((normalized.x - 0.6).abs() < 0.001);
    /// assert!((normalized.y - 0.8).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        if mag > f32::EPSILON {
            Self {
                x: self.x / mag,
                y: self.y / mag,
            }
        } else {
            Self::zero()
        }
    }

    /// Calculate dot product with another vector
    ///
    /// The dot product indicates how parallel two vectors are:
    /// - 1.0: Vectors point in same direction
    /// - 0.0: Vectors are perpendicular
    /// - -1.0: Vectors point in opposite directions
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v1 = Vec2::new(1.0, 0.0);
    /// let v2 = Vec2::new(0.0, 1.0);
    /// assert!((v1.dot(v2)).abs() < 0.001); // Perpendicular
    /// ```
    #[must_use]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Calculate the 2D cross product (scalar value)
    ///
    /// Returns the z-component of the 3D cross product.
    /// Positive if `other` is counter-clockwise from `self`,
    /// negative if clockwise, zero if parallel.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v1 = Vec2::new(1.0, 0.0);
    /// let v2 = Vec2::new(0.0, 1.0);
    /// assert!((v1.cross(v2) - 1.0).abs() < 0.001); // Counter-clockwise
    /// ```
    #[must_use]
    pub fn cross(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    /// Calculate distance to another vector
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v1 = Vec2::new(0.0, 0.0);
    /// let v2 = Vec2::new(3.0, 4.0);
    /// assert!((v1.distance_to(v2) - 5.0).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn distance_to(self, other: Self) -> f32 {
        (self - other).magnitude()
    }

    /// Calculate squared distance to another vector (faster, no sqrt)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v1 = Vec2::new(0.0, 0.0);
    /// let v2 = Vec2::new(3.0, 4.0);
    /// assert!((v1.distance_squared(v2) - 25.0).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).magnitude_squared()
    }

    /// Linearly interpolate between this vector and another
    ///
    /// # Arguments
    ///
    /// * `other` - Target vector
    /// * `t` - Interpolation factor (0.0 = self, 1.0 = other)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let start = Vec2::new(0.0, 0.0);
    /// let end = Vec2::new(10.0, 20.0);
    /// let mid = start.lerp(end, 0.5);
    /// assert!((mid.x - 5.0).abs() < 0.001);
    /// assert!((mid.y - 10.0).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    /// Rotate this vector by an angle in radians
    ///
    /// Positive angles rotate counter-clockwise.
    ///
    /// # Arguments
    ///
    /// * `angle` - Rotation angle in radians
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::new(1.0, 0.0);
    /// let rotated = v.rotate(std::f32::consts::FRAC_PI_2); // 90 degrees
    /// assert!((rotated.x).abs() < 0.001);
    /// assert!((rotated.y - 1.0).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn rotate(self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    /// Calculate the angle of this vector in radians
    ///
    /// Returns angle in range [-π, π] where 0 is right (positive X).
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::new(1.0, 0.0);
    /// assert!((v.angle()).abs() < 0.001);
    /// let v = Vec2::new(0.0, 1.0);
    /// assert!((v.angle() - std::f32::consts::FRAC_PI_2).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    /// Calculate angle to another vector in radians
    ///
    /// Returns the angle from this vector to the other vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v1 = Vec2::new(1.0, 0.0);
    /// let v2 = Vec2::new(0.0, 1.0);
    /// let angle = v1.angle_to(v2);
    /// assert!((angle.abs() - std::f32::consts::FRAC_PI_2).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn angle_to(self, other: Self) -> f32 {
        // Use atan2 of cross and dot products for signed angle
        let cross = self.cross(other);
        let dot = self.dot(other);
        cross.atan2(dot)
    }

    /// Reflect this vector off a surface with the given normal
    ///
    /// The normal should be normalized for correct results.
    ///
    /// # Arguments
    ///
    /// * `normal` - Surface normal (should be normalized)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::new(1.0, -1.0);
    /// let normal = Vec2::new(0.0, 1.0); // Horizontal surface
    /// let reflected = v.reflect(normal);
    /// assert!((reflected.x - 1.0).abs() < 0.001);
    /// assert!((reflected.y - 1.0).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * (2.0 * self.dot(normal))
    }

    /// Project this vector onto another vector
    ///
    /// # Arguments
    ///
    /// * `onto` - Vector to project onto
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// let onto = Vec2::new(1.0, 0.0);
    /// let projected = v.project(onto);
    /// assert!((projected.x - 3.0).abs() < 0.001);
    /// assert!((projected.y).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn project(self, onto: Self) -> Self {
        let onto_mag_sq = onto.magnitude_squared();
        if onto_mag_sq > f32::EPSILON {
            onto * (self.dot(onto) / onto_mag_sq)
        } else {
            Self::zero()
        }
    }

    /// Clamp the magnitude of this vector
    ///
    /// If magnitude is less than `min`, scale up to `min`.
    /// If magnitude is greater than `max`, scale down to `max`.
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum magnitude
    /// * `max` - Maximum magnitude
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::new(10.0, 0.0);
    /// let clamped = v.clamp_magnitude(0.0, 5.0);
    /// assert!((clamped.magnitude() - 5.0).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn clamp_magnitude(self, min: f32, max: f32) -> Self {
        let mag = self.magnitude();
        if mag < f32::EPSILON {
            return Self::zero();
        }
        let clamped_mag = mag.clamp(min, max);
        if (mag - clamped_mag).abs() < f32::EPSILON {
            self
        } else {
            self * (clamped_mag / mag)
        }
    }

    /// Convert to integer Point (truncates fractional parts)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::{Vec2, Point};
    /// let v = Vec2::new(10.7, 20.3);
    /// let p = v.to_point();
    /// assert_eq!(p, Point::new(10, 20));
    /// ```
    #[must_use]
    pub fn to_point(self) -> Point {
        Point::new(self.x as i16, self.y as i16)
    }

    /// Create from an angle in radians
    ///
    /// Creates a unit vector pointing in the given direction.
    /// Angle of 0 points right (positive X).
    ///
    /// # Arguments
    ///
    /// * `angle` - Direction in radians
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::Vec2;
    /// let v = Vec2::from_angle(0.0);
    /// assert!((v.x - 1.0).abs() < 0.001);
    /// assert!((v.y).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn from_angle(angle: f32) -> Self {
        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }
}

// Arithmetic operations for Vec2
impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, vec: Vec2) -> Vec2 {
        Vec2 {
            x: vec.x * self,
            y: vec.y * self,
        }
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl std::ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

/// Type-safe wrapper for star system indices
///
/// This prevents accidentally using raw integers where a star index is expected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StarIndex(pub u16);

impl StarIndex {
    /// Create a new star index
    #[must_use]
    pub const fn new(index: u16) -> Self {
        Self(index)
    }

    /// Convert to usize for array indexing
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<u16> for StarIndex {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<StarIndex> for u16 {
    fn from(value: StarIndex) -> Self {
        value.0
    }
}

/// Star type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StarType {
    /// Dwarf star (small, long-lived)
    Dwarf,
    /// Giant star (large, bright)
    Giant,
    /// Super giant star (massive, very bright)
    SuperGiant,
}

/// Star color classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StarColor {
    /// Red star (cool)
    Red,
    /// Orange star
    Orange,
    /// Yellow star (like our Sun)
    Yellow,
    /// Green star
    Green,
    /// Blue star (hot)
    Blue,
    /// White star (very hot)
    White,
}

/// Planet type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlanetType {
    /// Rocky terrestrial planet
    Rocky,
    /// Gas giant
    GasGiant,
    /// Water world
    Water,
    /// Ice world
    Ice,
    /// Volcanic world
    Volcanic,
    /// Rainbow world (special)
    Rainbow,
    /// Shattered world (special)
    Shattered,
    /// Treasure world (special)
    Treasure,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // ========== Point Tests ==========

    #[test]
    fn test_point_creation() {
        let p = Point::new(10, 20);
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
    }

    #[test]
    fn test_point_zero() {
        let p = Point::zero();
        assert_eq!(p, Point::new(0, 0));
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(3, 4);
        assert_relative_eq!(p1.distance_to(p2), 5.0, epsilon = 0.001);
    }

    #[test]
    fn test_point_distance_squared() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(3, 4);
        assert_eq!(p1.distance_squared(p2), 25);
    }

    #[test]
    fn test_point_manhattan() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(3, 4);
        assert_eq!(p1.manhattan_distance(p2), 7);
    }

    #[test]
    fn test_point_midpoint() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(10, 20);
        assert_eq!(p1.midpoint(p2), Point::new(5, 10));
    }

    #[test]
    fn test_point_lerp() {
        let start = Point::new(0, 0);
        let end = Point::new(100, 100);
        assert_eq!(start.lerp(end, 0.0), start);
        assert_eq!(start.lerp(end, 1.0), end);
        assert_eq!(start.lerp(end, 0.5), Point::new(50, 50));
    }

    #[test]
    fn test_point_scale() {
        let p = Point::new(10, 20);
        assert_eq!(p.scale(2.0), Point::new(20, 40));
        assert_eq!(p.scale(0.5), Point::new(5, 10));
    }

    #[test]
    fn test_point_clamp() {
        let p = Point::new(150, 250);
        let clamped = p.clamp(Point::new(0, 0), Point::new(100, 200));
        assert_eq!(clamped, Point::new(100, 200));
        
        let p2 = Point::new(50, 100);
        let clamped2 = p2.clamp(Point::new(0, 0), Point::new(100, 200));
        assert_eq!(clamped2, Point::new(50, 100)); // Already in bounds
    }

    #[test]
    fn test_point_to_vec2() {
        let p = Point::new(10, 20);
        let v = p.to_vec2();
        assert_eq!(v, Vec2::new(10.0, 20.0));
    }

    #[test]
    fn test_point_addition() {
        let p1 = Point::new(10, 20);
        let p2 = Point::new(5, 15);
        assert_eq!(p1 + p2, Point::new(15, 35));
    }

    #[test]
    fn test_point_subtraction() {
        let p1 = Point::new(10, 20);
        let p2 = Point::new(5, 15);
        assert_eq!(p1 - p2, Point::new(5, 5));
    }

    // ========== Vec2 Tests ==========

    #[test]
    fn test_vec2_creation() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn test_vec2_zero() {
        let v = Vec2::zero();
        assert_eq!(v, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_vec2_direction_vectors() {
        assert_eq!(Vec2::up(), Vec2::new(0.0, -1.0));
        assert_eq!(Vec2::down(), Vec2::new(0.0, 1.0));
        assert_eq!(Vec2::left(), Vec2::new(-1.0, 0.0));
        assert_eq!(Vec2::right(), Vec2::new(1.0, 0.0));
    }

    #[test]
    fn test_vec2_magnitude() {
        let v = Vec2::new(3.0, 4.0);
        assert_relative_eq!(v.magnitude(), 5.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_magnitude_squared() {
        let v = Vec2::new(3.0, 4.0);
        assert_relative_eq!(v.magnitude_squared(), 25.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_normalize() {
        let v = Vec2::new(3.0, 4.0);
        let n = v.normalize();
        assert_relative_eq!(n.magnitude(), 1.0, epsilon = 0.001);
        assert_relative_eq!(n.x, 0.6, epsilon = 0.001);
        assert_relative_eq!(n.y, 0.8, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_normalize_zero() {
        let v = Vec2::zero();
        let n = v.normalize();
        assert_eq!(n, Vec2::zero());
    }

    #[test]
    fn test_vec2_dot_product() {
        let v1 = Vec2::new(1.0, 0.0);
        let v2 = Vec2::new(0.0, 1.0);
        assert_relative_eq!(v1.dot(v2), 0.0, epsilon = 0.001); // Perpendicular

        let v3 = Vec2::new(1.0, 0.0);
        let v4 = Vec2::new(1.0, 0.0);
        assert_relative_eq!(v3.dot(v4), 1.0, epsilon = 0.001); // Parallel
    }

    #[test]
    fn test_vec2_cross_product() {
        let v1 = Vec2::new(1.0, 0.0);
        let v2 = Vec2::new(0.0, 1.0);
        assert_relative_eq!(v1.cross(v2), 1.0, epsilon = 0.001);
        assert_relative_eq!(v2.cross(v1), -1.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_distance() {
        let v1 = Vec2::new(0.0, 0.0);
        let v2 = Vec2::new(3.0, 4.0);
        assert_relative_eq!(v1.distance_to(v2), 5.0, epsilon = 0.001);
        assert_relative_eq!(v1.distance_squared(v2), 25.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_lerp() {
        let start = Vec2::new(0.0, 0.0);
        let end = Vec2::new(10.0, 20.0);
        let mid = start.lerp(end, 0.5);
        assert_relative_eq!(mid.x, 5.0, epsilon = 0.001);
        assert_relative_eq!(mid.y, 10.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_rotate() {
        let v = Vec2::new(1.0, 0.0);
        let rotated = v.rotate(std::f32::consts::FRAC_PI_2); // 90 degrees
        assert_relative_eq!(rotated.x, 0.0, epsilon = 0.001);
        assert_relative_eq!(rotated.y, 1.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_angle() {
        let v1 = Vec2::new(1.0, 0.0);
        assert_relative_eq!(v1.angle(), 0.0, epsilon = 0.001);
        
        let v2 = Vec2::new(0.0, 1.0);
        assert_relative_eq!(v2.angle(), std::f32::consts::FRAC_PI_2, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_angle_to() {
        let v1 = Vec2::new(1.0, 0.0);
        let v2 = Vec2::new(0.0, 1.0);
        let angle = v1.angle_to(v2);
        // Angle from right to up is 90 degrees counter-clockwise
        assert_relative_eq!(angle.abs(), std::f32::consts::FRAC_PI_2, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_reflect() {
        let v = Vec2::new(1.0, -1.0);
        let normal = Vec2::new(0.0, 1.0); // Horizontal surface
        let reflected = v.reflect(normal);
        assert_relative_eq!(reflected.x, 1.0, epsilon = 0.001);
        assert_relative_eq!(reflected.y, 1.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_project() {
        let v = Vec2::new(3.0, 4.0);
        let onto = Vec2::new(1.0, 0.0);
        let projected = v.project(onto);
        assert_relative_eq!(projected.x, 3.0, epsilon = 0.001);
        assert_relative_eq!(projected.y, 0.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_clamp_magnitude() {
        let v = Vec2::new(10.0, 0.0);
        let clamped = v.clamp_magnitude(0.0, 5.0);
        assert_relative_eq!(clamped.magnitude(), 5.0, epsilon = 0.001);

        let v2 = Vec2::new(2.0, 0.0);
        let clamped2 = v2.clamp_magnitude(0.0, 5.0);
        assert_relative_eq!(clamped2.magnitude(), 2.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_to_point() {
        let v = Vec2::new(10.7, 20.3);
        let p = v.to_point();
        assert_eq!(p, Point::new(10, 20));
    }

    #[test]
    fn test_vec2_from_angle() {
        let v = Vec2::from_angle(0.0);
        assert_relative_eq!(v.x, 1.0, epsilon = 0.001);
        assert_relative_eq!(v.y, 0.0, epsilon = 0.001);

        let v2 = Vec2::from_angle(std::f32::consts::PI);
        assert_relative_eq!(v2.x, -1.0, epsilon = 0.001);
        assert_relative_eq!(v2.y, 0.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_addition() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        let result = v1 + v2;
        assert_relative_eq!(result.x, 4.0, epsilon = 0.001);
        assert_relative_eq!(result.y, 6.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_subtraction() {
        let v1 = Vec2::new(5.0, 7.0);
        let v2 = Vec2::new(2.0, 3.0);
        let result = v1 - v2;
        assert_relative_eq!(result.x, 3.0, epsilon = 0.001);
        assert_relative_eq!(result.y, 4.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_scalar_multiply() {
        let v = Vec2::new(2.0, 3.0);
        let result = v * 2.0;
        assert_relative_eq!(result.x, 4.0, epsilon = 0.001);
        assert_relative_eq!(result.y, 6.0, epsilon = 0.001);

        // Test commutative property
        let result2 = 2.0 * v;
        assert_relative_eq!(result2.x, 4.0, epsilon = 0.001);
        assert_relative_eq!(result2.y, 6.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_scalar_divide() {
        let v = Vec2::new(4.0, 6.0);
        let result = v / 2.0;
        assert_relative_eq!(result.x, 2.0, epsilon = 0.001);
        assert_relative_eq!(result.y, 3.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_negation() {
        let v = Vec2::new(3.0, -4.0);
        let neg = -v;
        assert_relative_eq!(neg.x, -3.0, epsilon = 0.001);
        assert_relative_eq!(neg.y, 4.0, epsilon = 0.001);
    }

    #[test]
    fn test_vec2_compound_assign() {
        let mut v = Vec2::new(1.0, 2.0);
        v += Vec2::new(3.0, 4.0);
        assert_relative_eq!(v.x, 4.0, epsilon = 0.001);
        assert_relative_eq!(v.y, 6.0, epsilon = 0.001);

        v -= Vec2::new(1.0, 2.0);
        assert_relative_eq!(v.x, 3.0, epsilon = 0.001);
        assert_relative_eq!(v.y, 4.0, epsilon = 0.001);

        v *= 2.0;
        assert_relative_eq!(v.x, 6.0, epsilon = 0.001);
        assert_relative_eq!(v.y, 8.0, epsilon = 0.001);

        v /= 2.0;
        assert_relative_eq!(v.x, 3.0, epsilon = 0.001);
        assert_relative_eq!(v.y, 4.0, epsilon = 0.001);
    }

    // ========== StarIndex Tests ==========

    #[test]
    fn test_star_index() {
        let idx = StarIndex::new(42);
        assert_eq!(idx.0, 42);
        assert_eq!(idx.as_usize(), 42);
    }

    #[test]
    fn test_star_index_conversion() {
        let idx: StarIndex = 100.into();
        assert_eq!(idx.0, 100);

        let value: u16 = idx.into();
        assert_eq!(value, 100);
    }
}

