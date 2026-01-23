//! Star map data and management

use crate::core::{Point, StarColor, StarIndex, StarType};
use serde::{Deserialize, Serialize};

/// Star system description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    /// Unique index
    pub index: StarIndex,
    /// Position in HyperSpace
    pub position: Point,
    /// Star type
    pub star_type: StarType,
    /// Star color
    pub color: StarColor,
    /// Number of planets in system
    pub num_planets: u8,
    /// System name
    pub name: String,
    /// Whether this system has been visited
    pub visited: bool,
}

impl Star {
    /// Calculate distance to another star
    #[must_use]
    pub fn distance_to(&self, other: &Self) -> f32 {
        self.position.distance_to(other.position)
    }
}

/// Galaxy star map
pub struct StarMap {
    /// All star systems
    pub stars: Vec<Star>,
}

impl StarMap {
    /// Create a new star map
    ///
    /// TODO: Load actual star data in Phase 5
    #[must_use]
    pub fn new() -> Self {
        Self { stars: Vec::new() }
    }

    /// Get star by index
    #[must_use]
    pub fn get_star(&self, index: StarIndex) -> Option<&Star> {
        self.stars.get(index.as_usize())
    }

    /// Find closest star to a position
    #[must_use]
    pub fn find_closest_star(&self, position: Point) -> Option<&Star> {
        self.stars
            .iter()
            .min_by(|a, b| {
                let dist_a = a.position.distance_to(position);
                let dist_b = b.position.distance_to(position);
                dist_a.partial_cmp(&dist_b).unwrap()
            })
    }
}

impl Default for StarMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_creation() {
        let star = Star {
            index: StarIndex::new(0),
            position: Point::new(100, 200),
            star_type: StarType::Dwarf,
            color: StarColor::Yellow,
            num_planets: 8,
            name: "Sol".to_string(),
            visited: false,
        };

        assert_eq!(star.name, "Sol");
        assert_eq!(star.num_planets, 8);
    }

    #[test]
    fn test_star_distance() {
        let star1 = Star {
            index: StarIndex::new(0),
            position: Point::new(0, 0),
            star_type: StarType::Dwarf,
            color: StarColor::Yellow,
            num_planets: 0,
            name: "Star1".to_string(),
            visited: false,
        };

        let star2 = Star {
            index: StarIndex::new(1),
            position: Point::new(3, 4),
            star_type: StarType::Dwarf,
            color: StarColor::Red,
            num_planets: 0,
            name: "Star2".to_string(),
            visited: false,
        };

        let distance = star1.distance_to(&star2);
        assert!((distance - 5.0).abs() < 0.001);
    }
}
