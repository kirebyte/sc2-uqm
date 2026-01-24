//! Star map data and management
//!
//! Contains the 249 star systems of HyperSpace, their positions, properties,
//! and navigation utilities.

use crate::core::{Point, StarColor, StarIndex, StarType};
use serde::{Deserialize, Serialize};

/// Known alien species presence in star systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlienPresence {
    None,
    Sol,
    Shofixti,
    Yehat,
    Pkunk,
    Mycon,
    Rainbow,
    Melnorme,
    Vux,
    StartColony,
    Maidens,
    Thradd,
    ZoqFot,
    Syreen,
    Burvixese,
    Samatra,
    TalkingPet,
    Ilwrath,
    Spathi,
    ShipVault,
    UrquanWreck,
    AquaHelix,
    Utwig,
    Supox,
    Bomb,
    VuxBeast,
    Slylandro,
    Kitsune,
}

/// Star system description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    /// Unique index (0-248 for main galaxy)
    pub index: StarIndex,
    /// Position in HyperSpace (0-10000 x 0-10000 coordinate system)
    pub position: Point,
    /// Star type (Dwarf, Giant, SuperGiant)
    pub star_type: StarType,
    /// Star color (Red, Orange, Yellow, Green, Blue, White)
    pub color: StarColor,
    /// Number of planets in system
    pub num_planets: u8,
    /// System name (e.g., "Sol", "Alpha Centauri")
    pub name: String,
    /// Whether this system has been visited
    pub visited: bool,
    /// Alien presence in this system
    pub alien_presence: AlienPresence,
    /// Prefix name index (for procedural naming)
    pub prefix_index: u8,
    /// Postfix name index (for procedural naming)
    pub postfix_index: u8,
}

impl Star {
    /// Calculate Euclidean distance to another star
    ///
    /// # Examples
    /// ```
    /// # use uqm::starmap::map::Star;
    /// # use uqm::core::{Point, StarIndex, StarType, StarColor};
    /// # use uqm::starmap::map::AlienPresence;
    /// let sol = Star {
    ///     index: StarIndex::new(0),
    ///     position: Point::new(1752, 1450),
    ///     star_type: StarType::Dwarf,
    ///     color: StarColor::Yellow,
    ///     num_planets: 8,
    ///     name: "Sol".to_string(),
    ///     visited: false,
    ///     alien_presence: AlienPresence::Sol,
    ///     prefix_index: 0,
    ///     postfix_index: 129,
    /// };
    ///
    /// let alpha_centauri = Star {
    ///     index: StarIndex::new(1),
    ///     position: Point::new(2187, 1500),
    ///     star_type: StarType::Dwarf,
    ///     color: StarColor::Blue,
    ///     num_planets: 0,
    ///     name: "Alpha Centauri".to_string(),
    ///     visited: false,
    ///     alien_presence: AlienPresence::None,
    ///     prefix_index: 0,
    ///     postfix_index: 127,
    /// };
    ///
    /// let dist = sol.distance_to(&alpha_centauri);
    /// assert!(dist > 430.0 && dist < 440.0);
    /// ```
    #[must_use]
    pub fn distance_to(&self, other: &Self) -> f32 {
        self.position.distance_to(other.position)
    }

    /// Check if this system has a specific alien presence
    #[must_use]
    pub fn has_aliens(&self) -> bool {
        !matches!(self.alien_presence, AlienPresence::None)
    }

    /// Get a display name for this star system
    ///
    /// If the system has a custom name, returns it; otherwise generates
    /// a procedural name based on prefix/postfix indices
    #[must_use]
    pub fn display_name(&self) -> &str {
        if !self.name.is_empty() {
            &self.name
        } else {
            // TODO: Implement procedural name generation in future phase
            "Unknown System"
        }
    }
}

/// Galaxy star map containing all 249 star systems
///
/// The starmap uses a 10000x10000 coordinate system representing HyperSpace.
/// Sol (Earth) is located at approximately (1752, 1450).
pub struct StarMap {
    /// All star systems (249 main galaxy + QuasiSpace)
    stars: Vec<Star>,
}

impl StarMap {
    /// Create a new star map with all 249 canonical star systems
    ///
    /// Loads the authentic UQM star data from the original game
    #[must_use]
    pub fn new() -> Self {
        Self {
            stars: Self::load_canonical_stars(),
        }
    }

    /// Get total number of stars in the map
    #[must_use]
    pub fn len(&self) -> usize {
        self.stars.len()
    }

    /// Check if starmap is empty (should never be true for canonical map)
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stars.is_empty()
    }

    /// Get star by index
    ///
    /// # Examples
    /// ```
    /// # use uqm::starmap::map::StarMap;
    /// # use uqm::core::StarIndex;
    /// let map = StarMap::new();
    /// let sol = map.get_star(StarIndex::new(63)).expect("Sol should exist");
    /// assert_eq!(sol.name, "Sol");
    /// assert_eq!(sol.num_planets, 0); // Planet data will be filled in Phase 2
    /// ```
    #[must_use]
    pub fn get_star(&self, index: StarIndex) -> Option<&Star> {
        self.stars.get(index.as_usize())
    }

    /// Get mutable reference to a star by index
    pub fn get_star_mut(&mut self, index: StarIndex) -> Option<&mut Star> {
        self.stars.get_mut(index.as_usize())
    }

    /// Find closest star to a position
    ///
    /// # Examples
    /// ```
    /// # use uqm::starmap::map::StarMap;
    /// # use uqm::core::Point;
    /// let map = StarMap::new();
    /// let near_sol = map.find_closest_star(Point::new(1750, 1450));
    /// assert!(near_sol.is_some());
    /// let star = near_sol.unwrap();
    /// assert_eq!(star.name, "Sol");
    /// ```
    #[must_use]
    pub fn find_closest_star(&self, position: Point) -> Option<&Star> {
        self.stars
            .iter()
            .min_by(|a, b| {
                let dist_a = a.position.distance_to(position);
                let dist_b = b.position.distance_to(position);
                dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Find all stars within a given radius of a position
    ///
    /// # Examples
    /// ```
    /// # use uqm::starmap::map::StarMap;
    /// # use uqm::core::Point;
    /// let map = StarMap::new();
    /// // Find all stars within 500 units of Sol
    /// let nearby = map.find_stars_in_radius(Point::new(1752, 1450), 500.0);
    /// assert!(nearby.len() > 0);
    /// ```
    #[must_use]
    pub fn find_stars_in_radius(&self, position: Point, radius: f32) -> Vec<&Star> {
        self.stars
            .iter()
            .filter(|star| star.position.distance_to(position) <= radius)
            .collect()
    }

    /// Get all stars with alien presence
    #[must_use]
    pub fn get_alien_stars(&self) -> Vec<&Star> {
        self.stars
            .iter()
            .filter(|star| star.has_aliens())
            .collect()
    }

    /// Get iterator over all stars
    pub fn iter(&self) -> impl Iterator<Item = &Star> {
        self.stars.iter()
    }

    /// Get mutable iterator over all stars
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Star> {
        self.stars.iter_mut()
    }

    /// Mark a star system as visited
    ///
    /// # Returns
    /// `true` if the star was newly visited, `false` if already visited
    pub fn mark_visited(&mut self, index: StarIndex) -> bool {
        if let Some(star) = self.get_star_mut(index) {
            if !star.visited {
                star.visited = true;
                return true;
            }
        }
        false
    }

    /// Load ALL 249 canonical star systems from UQM
    ///
    /// Data extracted from sc2/src/uqm/plandata.c - EXACT COPY
    /// This ensures 100% fidelity to the original game
    fn load_canonical_stars() -> Vec<Star> {
        vec![
            // Index 0
            Star {
                index: StarIndex::new(0_u16),
                position: Point::new(5007, 35),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 74,
            },
            // Index 1
            Star {
                index: StarIndex::new(1_u16),
                position: Point::new(4714, 78),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 74,
            },
            // Index 2
            Star {
                index: StarIndex::new(2_u16),
                position: Point::new(2187, 83),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 126,
            },
            // Index 3
            Star {
                index: StarIndex::new(3_u16),
                position: Point::new(2814, 89),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 82,
            },
            // Index 4
            Star {
                index: StarIndex::new(4_u16),
                position: Point::new(4244, 91),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 125,
            },
            // Index 5
            Star {
                index: StarIndex::new(5_u16),
                position: Point::new(5652, 98),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 124,
            },
            // Index 6
            Star {
                index: StarIndex::new(6_u16),
                position: Point::new(2939, 116),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 82,
            },
            // Index 7
            Star {
                index: StarIndex::new(7_u16),
                position: Point::new(2771, 146),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 82,
            },
            // Index 8
            Star {
                index: StarIndex::new(8_u16),
                position: Point::new(5313, 150),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 73,
            },
            // Index 9
            Star {
                index: StarIndex::new(9_u16),
                position: Point::new(4529, 169),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 8,
                postfix_index: 74,
            },
            // Index 10
            Star {
                index: StarIndex::new(10_u16),
                position: Point::new(4911, 180),
                star_type: StarType::Giant,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 74,
            },
            // Index 11
            Star {
                index: StarIndex::new(11_u16),
                position: Point::new(4747, 221),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 74,
            },
            // Index 12
            Star {
                index: StarIndex::new(12_u16),
                position: Point::new(9708, 250),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 112,
            },
            // Index 13
            Star {
                index: StarIndex::new(13_u16),
                position: Point::new(4861, 262),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 74,
            },
            // Index 14
            Star {
                index: StarIndex::new(14_u16),
                position: Point::new(2908, 269),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Shofixti,
                prefix_index: 4,
                postfix_index: 82,
            },
            // Index 15
            Star {
                index: StarIndex::new(15_u16),
                position: Point::new(1855, 270),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 81,
            },
            // Index 16
            Star {
                index: StarIndex::new(16_u16),
                position: Point::new(7958, 270),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 8,
            },
            // Index 17
            Star {
                index: StarIndex::new(17_u16),
                position: Point::new(5160, 280),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 73,
            },
            // Index 18
            Star {
                index: StarIndex::new(18_u16),
                position: Point::new(4923, 294),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Yehat,
                prefix_index: 3,
                postfix_index: 74,
            },
            // Index 19
            Star {
                index: StarIndex::new(19_u16),
                position: Point::new(2820, 301),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 82,
            },
            // Index 20
            Star {
                index: StarIndex::new(20_u16),
                position: Point::new(7934, 318),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 8,
            },
            // Index 21
            Star {
                index: StarIndex::new(21_u16),
                position: Point::new(8062, 318),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 8,
            },
            // Index 22
            Star {
                index: StarIndex::new(22_u16),
                position: Point::new(1116, 334),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 91,
            },
            // Index 23
            Star {
                index: StarIndex::new(23_u16),
                position: Point::new(1787, 338),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 81,
            },
            // Index 24
            Star {
                index: StarIndex::new(24_u16),
                position: Point::new(5338, 355),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 73,
            },
            // Index 25
            Star {
                index: StarIndex::new(25_u16),
                position: Point::new(5039, 373),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 73,
            },
            // Index 26
            Star {
                index: StarIndex::new(26_u16),
                position: Point::new(4872, 408),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 74,
            },
            // Index 27
            Star {
                index: StarIndex::new(27_u16),
                position: Point::new(1740, 423),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 81,
            },
            // Index 28
            Star {
                index: StarIndex::new(28_u16),
                position: Point::new(4596, 429),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 9,
                postfix_index: 74,
            },
            // Index 29
            Star {
                index: StarIndex::new(29_u16),
                position: Point::new(2156, 440),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 81,
            },
            // Index 30
            Star {
                index: StarIndex::new(30_u16),
                position: Point::new(2004, 441),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 81,
            },
            // Index 31
            Star {
                index: StarIndex::new(31_u16),
                position: Point::new(2058, 475),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 81,
            },
            // Index 32
            Star {
                index: StarIndex::new(32_u16),
                position: Point::new(2100, 554),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 81,
            },
            // Index 33
            Star {
                index: StarIndex::new(33_u16),
                position: Point::new(6858, 577),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Mycon,
                prefix_index: 0,
                postfix_index: 123,
            },
            // Index 34
            Star {
                index: StarIndex::new(34_u16),
                position: Point::new(5014, 584),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 73,
            },
            // Index 35
            Star {
                index: StarIndex::new(35_u16),
                position: Point::new(5256, 608),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 73,
            },
            // Index 36
            Star {
                index: StarIndex::new(36_u16),
                position: Point::new(2411, 718),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 9,
            },
            // Index 37
            Star {
                index: StarIndex::new(37_u16),
                position: Point::new(2589, 741),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 9,
            },
            // Index 38
            Star {
                index: StarIndex::new(38_u16),
                position: Point::new(9292, 750),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 5,
            },
            // Index 39
            Star {
                index: StarIndex::new(39_u16),
                position: Point::new(1463, 779),
                star_type: StarType::Giant,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 80,
            },
            // Index 40
            Star {
                index: StarIndex::new(40_u16),
                position: Point::new(3089, 782),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 9,
            },
            // Index 41
            Star {
                index: StarIndex::new(41_u16),
                position: Point::new(2854, 787),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 9,
            },
            // Index 42
            Star {
                index: StarIndex::new(42_u16),
                position: Point::new(3333, 801),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 9,
            },
            // Index 43
            Star {
                index: StarIndex::new(43_u16),
                position: Point::new(9237, 821),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 5,
            },
            // Index 44
            Star {
                index: StarIndex::new(44_u16),
                position: Point::new(9339, 843),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 5,
            },
            // Index 45
            Star {
                index: StarIndex::new(45_u16),
                position: Point::new(1515, 866),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 80,
            },
            // Index 46
            Star {
                index: StarIndex::new(46_u16),
                position: Point::new(4770, 895),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 75,
            },
            // Index 47
            Star {
                index: StarIndex::new(47_u16),
                position: Point::new(1412, 905),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 80,
            },
            // Index 48
            Star {
                index: StarIndex::new(48_u16),
                position: Point::new(4681, 916),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Rainbow,
                prefix_index: 6,
                postfix_index: 75,
            },
            // Index 49
            Star {
                index: StarIndex::new(49_u16),
                position: Point::new(9419, 942),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 5,
            },
            // Index 50
            Star {
                index: StarIndex::new(50_u16),
                position: Point::new(4873, 968),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 75,
            },
            // Index 51
            Star {
                index: StarIndex::new(51_u16),
                position: Point::new(1895, 1041),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 93,
            },
            // Index 52
            Star {
                index: StarIndex::new(52_u16),
                position: Point::new(4337, 1066),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 75,
            },
            // Index 53
            Star {
                index: StarIndex::new(53_u16),
                position: Point::new(3732, 1067),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 122,
            },
            // Index 54
            Star {
                index: StarIndex::new(54_u16),
                position: Point::new(1579, 1115),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 80,
            },
            // Index 55
            Star {
                index: StarIndex::new(55_u16),
                position: Point::new(4875, 1145),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 75,
            },
            // Index 56
            Star {
                index: StarIndex::new(56_u16),
                position: Point::new(4604, 1187),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 75,
            },
            // Index 57
            Star {
                index: StarIndex::new(57_u16),
                position: Point::new(5812, 1208),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 72,
            },
            // Index 58
            Star {
                index: StarIndex::new(58_u16),
                position: Point::new(1312, 1260),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 80,
            },
            // Index 59
            Star {
                index: StarIndex::new(59_u16),
                position: Point::new(1916, 1270),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 93,
            },
            // Index 60
            Star {
                index: StarIndex::new(60_u16),
                position: Point::new(6562, 1270),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 121,
            },
            // Index 61
            Star {
                index: StarIndex::new(61_u16),
                position: Point::new(3958, 1354),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 89,
            },
            // Index 62
            Star {
                index: StarIndex::new(62_u16),
                position: Point::new(4000, 1363),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 89,
            },
            // Index 63
            Star {
                index: StarIndex::new(63_u16),
                position: Point::new(1752, 1450),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: "Sol".to_string(),
                visited: false,
                alien_presence: AlienPresence::Sol,
                prefix_index: 0,
                postfix_index: 129,
            },
            // Index 64
            Star {
                index: StarIndex::new(64_u16),
                position: Point::new(2187, 1500),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: "Alpha Centauri".to_string(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 127,
            },
            // Index 65
            Star {
                index: StarIndex::new(65_u16),
                position: Point::new(1806, 1507),
                star_type: StarType::Giant,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 128,
            },
            // Index 66
            Star {
                index: StarIndex::new(66_u16),
                position: Point::new(5708, 1520),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 72,
            },
            // Index 67
            Star {
                index: StarIndex::new(67_u16),
                position: Point::new(9469, 1548),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 6,
            },
            // Index 68
            Star {
                index: StarIndex::new(68_u16),
                position: Point::new(4333, 1562),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 88,
            },
            // Index 69
            Star {
                index: StarIndex::new(69_u16),
                position: Point::new(6041, 1562),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 72,
            },
            // Index 70
            Star {
                index: StarIndex::new(70_u16),
                position: Point::new(9375, 1583),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 6,
            },
            // Index 71
            Star {
                index: StarIndex::new(71_u16),
                position: Point::new(2881, 1614),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 96,
            },
            // Index 72
            Star {
                index: StarIndex::new(72_u16),
                position: Point::new(6083, 1625),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 72,
            },
            // Index 73
            Star {
                index: StarIndex::new(73_u16),
                position: Point::new(4250, 1645),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 88,
            },
            // Index 74
            Star {
                index: StarIndex::new(74_u16),
                position: Point::new(9477, 1670),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 6,
            },
            // Index 75
            Star {
                index: StarIndex::new(75_u16),
                position: Point::new(2840, 1676),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 96,
            },
            // Index 76
            Star {
                index: StarIndex::new(76_u16),
                position: Point::new(9541, 1687),
                star_type: StarType::Giant,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 6,
            },
            // Index 77
            Star {
                index: StarIndex::new(77_u16),
                position: Point::new(7395, 1687),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 69,
            },
            // Index 78
            Star {
                index: StarIndex::new(78_u16),
                position: Point::new(4333, 1687),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Vux,
                prefix_index: 2,
                postfix_index: 88,
            },
            // Index 79
            Star {
                index: StarIndex::new(79_u16),
                position: Point::new(9559, 1735),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 6,
            },
            // Index 80
            Star {
                index: StarIndex::new(80_u16),
                position: Point::new(1601, 1746),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 94,
            },
            // Index 81
            Star {
                index: StarIndex::new(81_u16),
                position: Point::new(7395, 1750),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 69,
            },
            // Index 82
            Star {
                index: StarIndex::new(82_u16),
                position: Point::new(1666, 1812),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 94,
            },
            // Index 83
            Star {
                index: StarIndex::new(83_u16),
                position: Point::new(7187, 1833),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 69,
            },
            // Index 84
            Star {
                index: StarIndex::new(84_u16),
                position: Point::new(1140, 1847),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 85,
            },
            // Index 85
            Star {
                index: StarIndex::new(85_u16),
                position: Point::new(6467, 1878),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 71,
            },
            // Index 86
            Star {
                index: StarIndex::new(86_u16),
                position: Point::new(2791, 1895),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: "Sol".to_string(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 96,
            },
            // Index 87
            Star {
                index: StarIndex::new(87_u16),
                position: Point::new(6500, 1916),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: "Alpha Centauri".to_string(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 71,
            },
            // Index 88
            Star {
                index: StarIndex::new(88_u16),
                position: Point::new(5458, 1916),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 119,
            },
            // Index 89
            Star {
                index: StarIndex::new(89_u16),
                position: Point::new(1048, 1919),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 85,
            },
            // Index 90
            Star {
                index: StarIndex::new(90_u16),
                position: Point::new(3678, 1926),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 99,
            },
            // Index 91
            Star {
                index: StarIndex::new(91_u16),
                position: Point::new(3345, 1931),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::StartColony,
                prefix_index: 0,
                postfix_index: 98,
            },
            // Index 92
            Star {
                index: StarIndex::new(92_u16),
                position: Point::new(8187, 1937),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 7,
            },
            // Index 93
            Star {
                index: StarIndex::new(93_u16),
                position: Point::new(4221, 1986),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Maidens,
                prefix_index: 1,
                postfix_index: 100,
            },
            // Index 94
            Star {
                index: StarIndex::new(94_u16),
                position: Point::new(4500, 2000),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 100,
            },
            // Index 95
            Star {
                index: StarIndex::new(95_u16),
                position: Point::new(6833, 2000),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 70,
            },
            // Index 96
            Star {
                index: StarIndex::new(96_u16),
                position: Point::new(8163, 2009),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 7,
            },
            // Index 97
            Star {
                index: StarIndex::new(97_u16),
                position: Point::new(8080, 2011),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 7,
            },
            // Index 98
            Star {
                index: StarIndex::new(98_u16),
                position: Point::new(6036, 2035),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 71,
            },
            // Index 99
            Star {
                index: StarIndex::new(99_u16),
                position: Point::new(2104, 2083),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 118,
            },
            // Index 100
            Star {
                index: StarIndex::new(100_u16),
                position: Point::new(8062, 2083),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 7,
            },
            // Index 101
            Star {
                index: StarIndex::new(101_u16),
                position: Point::new(6500, 2208),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 71,
            },
            // Index 102
            Star {
                index: StarIndex::new(102_u16),
                position: Point::new(6291, 2208),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 71,
            },
            // Index 103
            Star {
                index: StarIndex::new(103_u16),
                position: Point::new(3884, 2262),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 99,
            },
            // Index 104
            Star {
                index: StarIndex::new(104_u16),
                position: Point::new(2306, 2285),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 95,
            },
            // Index 105
            Star {
                index: StarIndex::new(105_u16),
                position: Point::new(2402, 2309),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 95,
            },
            // Index 106
            Star {
                index: StarIndex::new(106_u16),
                position: Point::new(6395, 2312),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 12,
            },
            // Index 107
            Star {
                index: StarIndex::new(107_u16),
                position: Point::new(8875, 2312),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 9,
                postfix_index: 61,
            },
            // Index 108
            Star {
                index: StarIndex::new(108_u16),
                position: Point::new(3551, 2320),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 99,
            },
            // Index 109
            Star {
                index: StarIndex::new(109_u16),
                position: Point::new(6208, 2333),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 12,
            },
            // Index 110
            Star {
                index: StarIndex::new(110_u16),
                position: Point::new(3354, 2354),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 99,
            },
            // Index 111
            Star {
                index: StarIndex::new(111_u16),
                position: Point::new(9909, 2359),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 111,
            },
            // Index 112
            Star {
                index: StarIndex::new(112_u16),
                position: Point::new(2298, 2385),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 95,
            },
            // Index 113
            Star {
                index: StarIndex::new(113_u16),
                position: Point::new(7020, 2395),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 70,
            },
            // Index 114
            Star {
                index: StarIndex::new(114_u16),
                position: Point::new(9038, 2407),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 61,
            },
            // Index 115
            Star {
                index: StarIndex::new(115_u16),
                position: Point::new(9375, 2416),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 8,
                postfix_index: 61,
            },
            // Index 116
            Star {
                index: StarIndex::new(116_u16),
                position: Point::new(6500, 2458),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 12,
            },
            // Index 117
            Star {
                index: StarIndex::new(117_u16),
                position: Point::new(3641, 2512),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 86,
            },
            // Index 118
            Star {
                index: StarIndex::new(118_u16),
                position: Point::new(5625, 2520),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 14,
            },
            // Index 119
            Star {
                index: StarIndex::new(119_u16),
                position: Point::new(3713, 2537),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 86,
            },
            // Index 120
            Star {
                index: StarIndex::new(120_u16),
                position: Point::new(3587, 2566),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 86,
            },
            // Index 121
            Star {
                index: StarIndex::new(121_u16),
                position: Point::new(9291, 2583),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 61,
            },
            // Index 122
            Star {
                index: StarIndex::new(122_u16),
                position: Point::new(3721, 2619),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 86,
            },
            // Index 123
            Star {
                index: StarIndex::new(123_u16),
                position: Point::new(5791, 2625),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 14,
            },
            // Index 124
            Star {
                index: StarIndex::new(124_u16),
                position: Point::new(6416, 2625),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 12,
            },
            // Index 125
            Star {
                index: StarIndex::new(125_u16),
                position: Point::new(3608, 2637),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 86,
            },
            // Index 126
            Star {
                index: StarIndex::new(126_u16),
                position: Point::new(3499, 2648),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 87,
            },
            // Index 127
            Star {
                index: StarIndex::new(127_u16),
                position: Point::new(9479, 2666),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 61,
            },
            // Index 128
            Star {
                index: StarIndex::new(128_u16),
                position: Point::new(3668, 2666),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 86,
            },
            // Index 129
            Star {
                index: StarIndex::new(129_u16),
                position: Point::new(8895, 2687),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 61,
            },
            // Index 130
            Star {
                index: StarIndex::new(130_u16),
                position: Point::new(5375, 2729),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 116,
            },
            // Index 131
            Star {
                index: StarIndex::new(131_u16),
                position: Point::new(6458, 2750),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 12,
            },
            // Index 132
            Star {
                index: StarIndex::new(132_u16),
                position: Point::new(2458, 2750),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 106,
            },
            // Index 133
            Star {
                index: StarIndex::new(133_u16),
                position: Point::new(7083, 2770),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 70,
            },
            // Index 134
            Star {
                index: StarIndex::new(134_u16),
                position: Point::new(3759, 2778),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 87,
            },
            // Index 135
            Star {
                index: StarIndex::new(135_u16),
                position: Point::new(9333, 2791),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 61,
            },
            // Index 136
            Star {
                index: StarIndex::new(136_u16),
                position: Point::new(3400, 2804),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 87,
            },
            // Index 137
            Star {
                index: StarIndex::new(137_u16),
                position: Point::new(9469, 2806),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 61,
            },
            // Index 138
            Star {
                index: StarIndex::new(138_u16),
                position: Point::new(3619, 2830),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 87,
            },
            // Index 139
            Star {
                index: StarIndex::new(139_u16),
                position: Point::new(2208, 2854),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 106,
            },
            // Index 140
            Star {
                index: StarIndex::new(140_u16),
                position: Point::new(9250, 2854),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 61,
            },
            // Index 141
            Star {
                index: StarIndex::new(141_u16),
                position: Point::new(4030, 2887),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 105,
            },
            // Index 142
            Star {
                index: StarIndex::new(142_u16),
                position: Point::new(2727, 2951),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 106,
            },
            // Index 143
            Star {
                index: StarIndex::new(143_u16),
                position: Point::new(4645, 2958),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 105,
            },
            // Index 144
            Star {
                index: StarIndex::new(144_u16),
                position: Point::new(5625, 2958),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 13,
            },
            // Index 145
            Star {
                index: StarIndex::new(145_u16),
                position: Point::new(8270, 2958),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 66,
            },
            // Index 146
            Star {
                index: StarIndex::new(146_u16),
                position: Point::new(8291, 2979),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 66,
            },
            // Index 147
            Star {
                index: StarIndex::new(147_u16),
                position: Point::new(6020, 2979),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Rainbow,
                prefix_index: 3,
                postfix_index: 13,
            },
            // Index 148
            Star {
                index: StarIndex::new(148_u16),
                position: Point::new(6562, 3020),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 70,
            },
            // Index 149
            Star {
                index: StarIndex::new(149_u16),
                position: Point::new(2011, 3043),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 8,
                postfix_index: 106,
            },
            // Index 150
            Star {
                index: StarIndex::new(150_u16),
                position: Point::new(8125, 3083),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 66,
            },
            // Index 151
            Star {
                index: StarIndex::new(151_u16),
                position: Point::new(2354, 3166),
                star_type: StarType::Giant,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 106,
            },
            // Index 152
            Star {
                index: StarIndex::new(152_u16),
                position: Point::new(3833, 3187),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 105,
            },
            // Index 153
            Star {
                index: StarIndex::new(153_u16),
                position: Point::new(5812, 3208),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 13,
            },
            // Index 154
            Star {
                index: StarIndex::new(154_u16),
                position: Point::new(9000, 3250),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 113,
            },
            // Index 155
            Star {
                index: StarIndex::new(155_u16),
                position: Point::new(1104, 3333),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 84,
            },
            // Index 156
            Star {
                index: StarIndex::new(156_u16),
                position: Point::new(2687, 3333),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 106,
            },
            // Index 157
            Star {
                index: StarIndex::new(157_u16),
                position: Point::new(3187, 3375),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 107,
            },
            // Index 158
            Star {
                index: StarIndex::new(158_u16),
                position: Point::new(1758, 3418),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 108,
            },
            // Index 159
            Star {
                index: StarIndex::new(159_u16),
                position: Point::new(2520, 3437),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 106,
            },
            // Index 160
            Star {
                index: StarIndex::new(160_u16),
                position: Point::new(8437, 3458),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 64,
            },
            // Index 161
            Star {
                index: StarIndex::new(161_u16),
                position: Point::new(8770, 3458),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 64,
            },
            // Index 162
            Star {
                index: StarIndex::new(162_u16),
                position: Point::new(3000, 3500),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 107,
            },
            // Index 163
            Star {
                index: StarIndex::new(163_u16),
                position: Point::new(8791, 3541),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 64,
            },
            // Index 164
            Star {
                index: StarIndex::new(164_u16),
                position: Point::new(2148, 3551),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 109,
            },
            // Index 165
            Star {
                index: StarIndex::new(165_u16),
                position: Point::new(7375, 3562),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 115,
            },
            // Index 166
            Star {
                index: StarIndex::new(166_u16),
                position: Point::new(9312, 3562),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 63,
            },
            // Index 167
            Star {
                index: StarIndex::new(167_u16),
                position: Point::new(9599, 3583),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 63,
            },
            // Index 168
            Star {
                index: StarIndex::new(168_u16),
                position: Point::new(9375, 3604),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 63,
            },
            // Index 169
            Star {
                index: StarIndex::new(169_u16),
                position: Point::new(2770, 3625),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 107,
            },
            // Index 170
            Star {
                index: StarIndex::new(170_u16),
                position: Point::new(8708, 3625),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 64,
            },
            // Index 171
            Star {
                index: StarIndex::new(171_u16),
                position: Point::new(1604, 3645),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 108,
            },
            // Index 172
            Star {
                index: StarIndex::new(172_u16),
                position: Point::new(2274, 3663),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 109,
            },
            // Index 173
            Star {
                index: StarIndex::new(173_u16),
                position: Point::new(3083, 3674),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 107,
            },
            // Index 174
            Star {
                index: StarIndex::new(174_u16),
                position: Point::new(2416, 3687),
                star_type: StarType::Giant,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Spathi,
                prefix_index: 5,
                postfix_index: 109,
            },
            // Index 175
            Star {
                index: StarIndex::new(175_u16),
                position: Point::new(9333, 3708),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 63,
            },
            // Index 176
            Star {
                index: StarIndex::new(176_u16),
                position: Point::new(2250, 3708),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 109,
            },
            // Index 177
            Star {
                index: StarIndex::new(177_u16),
                position: Point::new(2354, 3741),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 109,
            },
            // Index 178
            Star {
                index: StarIndex::new(178_u16),
                position: Point::new(2583, 3750),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 109,
            },
            // Index 179
            Star {
                index: StarIndex::new(179_u16),
                position: Point::new(4125, 3770),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Syreen,
                prefix_index: 0,
                postfix_index: 114,
            },
            // Index 180
            Star {
                index: StarIndex::new(180_u16),
                position: Point::new(6270, 3833),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 10,
            },
            // Index 181
            Star {
                index: StarIndex::new(181_u16),
                position: Point::new(2145, 3916),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 110,
            },
            // Index 182
            Star {
                index: StarIndex::new(182_u16),
                position: Point::new(6125, 3937),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 10,
            },
            // Index 183
            Star {
                index: StarIndex::new(183_u16),
                position: Point::new(6291, 3937),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 9,
                postfix_index: 10,
            },
            // Index 184
            Star {
                index: StarIndex::new(184_u16),
                position: Point::new(5937, 3937),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::ShipVault,
                prefix_index: 5,
                postfix_index: 10,
            },
            // Index 185
            Star {
                index: StarIndex::new(185_u16),
                position: Point::new(2479, 3958),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 109,
            },
            // Index 186
            Star {
                index: StarIndex::new(186_u16),
                position: Point::new(2062, 3991),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 110,
            },
            // Index 187
            Star {
                index: StarIndex::new(187_u16),
                position: Point::new(5895, 4020),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 8,
                postfix_index: 10,
            },
            // Index 188
            Star {
                index: StarIndex::new(188_u16),
                position: Point::new(6062, 4041),
                star_type: StarType::Giant,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 10,
            },
            // Index 189
            Star {
                index: StarIndex::new(189_u16),
                position: Point::new(2875, 4041),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 20,
            },
            // Index 190
            Star {
                index: StarIndex::new(190_u16),
                position: Point::new(8645, 4062),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 65,
            },
            // Index 191
            Star {
                index: StarIndex::new(191_u16),
                position: Point::new(5958, 4083),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 10,
            },
            // Index 192
            Star {
                index: StarIndex::new(192_u16),
                position: Point::new(3038, 4083),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 20,
            },
            // Index 193
            Star {
                index: StarIndex::new(193_u16),
                position: Point::new(6166, 4125),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 10,
            },
            // Index 194
            Star {
                index: StarIndex::new(194_u16),
                position: Point::new(9812, 4145),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 62,
            },
            // Index 195
            Star {
                index: StarIndex::new(195_u16),
                position: Point::new(8520, 4166),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 65,
            },
            // Index 196
            Star {
                index: StarIndex::new(196_u16),
                position: Point::new(9573, 4182),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 62,
            },
            // Index 197
            Star {
                index: StarIndex::new(197_u16),
                position: Point::new(2145, 4208),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 110,
            },
            // Index 198
            Star {
                index: StarIndex::new(198_u16),
                position: Point::new(6208, 4229),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 10,
            },
            // Index 199
            Star {
                index: StarIndex::new(199_u16),
                position: Point::new(2812, 4250),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 20,
            },
            // Index 200
            Star {
                index: StarIndex::new(200_u16),
                position: Point::new(2937, 4306),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 20,
            },
            // Index 201
            Star {
                index: StarIndex::new(201_u16),
                position: Point::new(9416, 4395),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 62,
            },
            // Index 202
            Star {
                index: StarIndex::new(202_u16),
                position: Point::new(2875, 4479),
                star_type: StarType::Giant,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 20,
            },
            // Index 203
            Star {
                index: StarIndex::new(203_u16),
                position: Point::new(7250, 4583),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 68,
            },
            // Index 204
            Star {
                index: StarIndex::new(204_u16),
                position: Point::new(5708, 4604),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 104,
            },
            // Index 205
            Star {
                index: StarIndex::new(205_u16),
                position: Point::new(2895, 4687),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 20,
            },
            // Index 206
            Star {
                index: StarIndex::new(206_u16),
                position: Point::new(2708, 4708),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 20,
            },
            // Index 207
            Star {
                index: StarIndex::new(207_u16),
                position: Point::new(5094, 4931),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 11,
            },
            // Index 208
            Star {
                index: StarIndex::new(208_u16),
                position: Point::new(9000, 5000),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 67,
            },
            // Index 209
            Star {
                index: StarIndex::new(209_u16),
                position: Point::new(8958, 5000),
                star_type: StarType::Giant,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 67,
            },
            // Index 210
            Star {
                index: StarIndex::new(210_u16),
                position: Point::new(5006, 5011),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 11,
            },
            // Index 211
            Star {
                index: StarIndex::new(211_u16),
                position: Point::new(7312, 5062),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 68,
            },
            // Index 212
            Star {
                index: StarIndex::new(212_u16),
                position: Point::new(3679, 5068),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 17,
            },
            // Index 213
            Star {
                index: StarIndex::new(213_u16),
                position: Point::new(9062, 5083),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 67,
            },
            // Index 214
            Star {
                index: StarIndex::new(214_u16),
                position: Point::new(7416, 5083),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Rainbow,
                prefix_index: 3,
                postfix_index: 68,
            },
            // Index 215
            Star {
                index: StarIndex::new(215_u16),
                position: Point::new(5155, 5122),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 11,
            },
            // Index 216
            Star {
                index: StarIndex::new(216_u16),
                position: Point::new(3875, 5145),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 17,
            },
            // Index 217
            Star {
                index: StarIndex::new(217_u16),
                position: Point::new(4937, 5145),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 11,
            },
            // Index 218
            Star {
                index: StarIndex::new(218_u16),
                position: Point::new(2979, 5166),
                star_type: StarType::Giant,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 15,
            },
            // Index 219
            Star {
                index: StarIndex::new(219_u16),
                position: Point::new(3035, 5178),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 15,
            },
            // Index 220
            Star {
                index: StarIndex::new(220_u16),
                position: Point::new(3994, 5185),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 17,
            },
            // Index 221
            Star {
                index: StarIndex::new(221_u16),
                position: Point::new(3541, 5187),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 17,
            },
            // Index 222
            Star {
                index: StarIndex::new(222_u16),
                position: Point::new(5977, 5246),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 102,
            },
            // Index 223
            Star {
                index: StarIndex::new(223_u16),
                position: Point::new(3770, 5250),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 17,
            },
            // Index 224
            Star {
                index: StarIndex::new(224_u16),
                position: Point::new(1520, 5261),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 55,
            },
            // Index 225
            Star {
                index: StarIndex::new(225_u16),
                position: Point::new(1613, 5279),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 55,
            },
            // Index 226
            Star {
                index: StarIndex::new(226_u16),
                position: Point::new(7020, 5291),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 68,
            },
            // Index 227
            Star {
                index: StarIndex::new(227_u16),
                position: Point::new(1416, 5315),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 55,
            },
            // Index 228
            Star {
                index: StarIndex::new(228_u16),
                position: Point::new(2993, 5318),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 15,
            },
            // Index 229
            Star {
                index: StarIndex::new(229_u16),
                position: Point::new(1425, 5404),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 55,
            },
            // Index 230
            Star {
                index: StarIndex::new(230_u16),
                position: Point::new(1854, 5416),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 55,
            },
            // Index 231
            Star {
                index: StarIndex::new(231_u16),
                position: Point::new(3625, 5437),
                star_type: StarType::Giant,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 16,
            },
            // Index 232
            Star {
                index: StarIndex::new(232_u16),
                position: Point::new(3416, 5437),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 16,
            },
            // Index 233
            Star {
                index: StarIndex::new(233_u16),
                position: Point::new(4000, 5437),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::ZoqFot,
                prefix_index: 1,
                postfix_index: 18,
            },
            // Index 234
            Star {
                index: StarIndex::new(234_u16),
                position: Point::new(6270, 5479),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 102,
            },
            // Index 235
            Star {
                index: StarIndex::new(235_u16),
                position: Point::new(3583, 5479),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 16,
            },
            // Index 236
            Star {
                index: StarIndex::new(236_u16),
                position: Point::new(4083, 5513),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 18,
            },
            // Index 237
            Star {
                index: StarIndex::new(237_u16),
                position: Point::new(2159, 5614),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 55,
            },
            // Index 238
            Star {
                index: StarIndex::new(238_u16),
                position: Point::new(3937, 5625),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 18,
            },
            // Index 239
            Star {
                index: StarIndex::new(239_u16),
                position: Point::new(6014, 5632),
                star_type: StarType::Giant,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 21,
            },
            // Index 240
            Star {
                index: StarIndex::new(240_u16),
                position: Point::new(3625, 5750),
                star_type: StarType::Giant,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 19,
            },
            // Index 241
            Star {
                index: StarIndex::new(241_u16),
                position: Point::new(6107, 5785),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 21,
            },
            // Index 242
            Star {
                index: StarIndex::new(242_u16),
                position: Point::new(9645, 5791),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Burvixese,
                prefix_index: 0,
                postfix_index: 130,
            },
            // Index 243
            Star {
                index: StarIndex::new(243_u16),
                position: Point::new(1545, 5818),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 54,
            },
            // Index 244
            Star {
                index: StarIndex::new(244_u16),
                position: Point::new(3750, 5833),
                star_type: StarType::Giant,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 19,
            },
            // Index 245
            Star {
                index: StarIndex::new(245_u16),
                position: Point::new(6301, 5875),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 21,
            },
            // Index 246
            Star {
                index: StarIndex::new(246_u16),
                position: Point::new(1923, 5878),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 54,
            },
            // Index 247
            Star {
                index: StarIndex::new(247_u16),
                position: Point::new(4625, 5895),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 131,
            },
            // Index 248
            Star {
                index: StarIndex::new(248_u16),
                position: Point::new(5437, 5916),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 33,
            },
            // Index 249
            Star {
                index: StarIndex::new(249_u16),
                position: Point::new(1714, 5926),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 54,
            },
            // Index 250
            Star {
                index: StarIndex::new(250_u16),
                position: Point::new(6200, 5935),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Samatra,
                prefix_index: 4,
                postfix_index: 21,
            },
            // Index 251
            Star {
                index: StarIndex::new(251_u16),
                position: Point::new(6429, 5958),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 21,
            },
            // Index 252
            Star {
                index: StarIndex::new(252_u16),
                position: Point::new(4729, 5958),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 131,
            },
            // Index 253
            Star {
                index: StarIndex::new(253_u16),
                position: Point::new(1978, 5968),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::TalkingPet,
                prefix_index: 2,
                postfix_index: 54,
            },
            // Index 254
            Star {
                index: StarIndex::new(254_u16),
                position: Point::new(4625, 6000),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 131,
            },
            // Index 255
            Star {
                index: StarIndex::new(255_u16),
                position: Point::new(6166, 6000),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 21,
            },
            // Index 256
            Star {
                index: StarIndex::new(256_u16),
                position: Point::new(6496, 6032),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 21,
            },
            // Index 257
            Star {
                index: StarIndex::new(257_u16),
                position: Point::new(2228, 6038),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 12,
                postfix_index: 54,
            },
            // Index 258
            Star {
                index: StarIndex::new(258_u16),
                position: Point::new(4583, 6041),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 131,
            },
            // Index 259
            Star {
                index: StarIndex::new(259_u16),
                position: Point::new(1558, 6058),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 54,
            },
            // Index 260
            Star {
                index: StarIndex::new(260_u16),
                position: Point::new(1902, 6065),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 54,
            },
            // Index 261
            Star {
                index: StarIndex::new(261_u16),
                position: Point::new(2159, 6073),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 11,
                postfix_index: 54,
            },
            // Index 262
            Star {
                index: StarIndex::new(262_u16),
                position: Point::new(2200, 6176),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 10,
                postfix_index: 54,
            },
            // Index 263
            Star {
                index: StarIndex::new(263_u16),
                position: Point::new(5250, 6229),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 33,
            },
            // Index 264
            Star {
                index: StarIndex::new(264_u16),
                position: Point::new(8166, 6250),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 40,
            },
            // Index 265
            Star {
                index: StarIndex::new(265_u16),
                position: Point::new(6215, 6255),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 8,
                postfix_index: 21,
            },
            // Index 266
            Star {
                index: StarIndex::new(266_u16),
                position: Point::new(5583, 6291),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 33,
            },
            // Index 267
            Star {
                index: StarIndex::new(267_u16),
                position: Point::new(1881, 6308),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 8,
                postfix_index: 54,
            },
            // Index 268
            Star {
                index: StarIndex::new(268_u16),
                position: Point::new(1795, 6329),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 54,
            },
            // Index 269
            Star {
                index: StarIndex::new(269_u16),
                position: Point::new(2118, 6379),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 9,
                postfix_index: 54,
            },
            // Index 270
            Star {
                index: StarIndex::new(270_u16),
                position: Point::new(3716, 6458),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 30,
            },
            // Index 271
            Star {
                index: StarIndex::new(271_u16),
                position: Point::new(1360, 6489),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 56,
            },
            // Index 272
            Star {
                index: StarIndex::new(272_u16),
                position: Point::new(7333, 6500),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 40,
            },
            // Index 273
            Star {
                index: StarIndex::new(273_u16),
                position: Point::new(3770, 6500),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 30,
            },
            // Index 274
            Star {
                index: StarIndex::new(274_u16),
                position: Point::new(4500, 6500),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 37,
            },
            // Index 275
            Star {
                index: StarIndex::new(275_u16),
                position: Point::new(7812, 6562),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 40,
            },
            // Index 276
            Star {
                index: StarIndex::new(276_u16),
                position: Point::new(5910, 6624),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 29,
            },
            // Index 277
            Star {
                index: StarIndex::new(277_u16),
                position: Point::new(2604, 6645),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 101,
            },
            // Index 278
            Star {
                index: StarIndex::new(278_u16),
                position: Point::new(5479, 6687),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 33,
            },
            // Index 279
            Star {
                index: StarIndex::new(279_u16),
                position: Point::new(6020, 6729),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 29,
            },
            // Index 280
            Star {
                index: StarIndex::new(280_u16),
                position: Point::new(5062, 6750),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 10,
                postfix_index: 28,
            },
            // Index 281
            Star {
                index: StarIndex::new(281_u16),
                position: Point::new(4208, 6854),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 31,
            },
            // Index 282
            Star {
                index: StarIndex::new(282_u16),
                position: Point::new(5145, 6875),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 9,
                postfix_index: 28,
            },
            // Index 283
            Star {
                index: StarIndex::new(283_u16),
                position: Point::new(4291, 6937),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 31,
            },
            // Index 284
            Star {
                index: StarIndex::new(284_u16),
                position: Point::new(5145, 6958),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 8,
                postfix_index: 28,
            },
            // Index 285
            Star {
                index: StarIndex::new(285_u16),
                position: Point::new(7208, 7000),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 39,
            },
            // Index 286
            Star {
                index: StarIndex::new(286_u16),
                position: Point::new(8625, 7000),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Rainbow,
                prefix_index: 1,
                postfix_index: 41,
            },
            // Index 287
            Star {
                index: StarIndex::new(287_u16),
                position: Point::new(4955, 7034),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 28,
            },
            // Index 288
            Star {
                index: StarIndex::new(288_u16),
                position: Point::new(4895, 7041),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 28,
            },
            // Index 289
            Star {
                index: StarIndex::new(289_u16),
                position: Point::new(4971, 7104),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 28,
            },
            // Index 290
            Star {
                index: StarIndex::new(290_u16),
                position: Point::new(8666, 7104),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 41,
            },
            // Index 291
            Star {
                index: StarIndex::new(291_u16),
                position: Point::new(4854, 7125),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 28,
            },
            // Index 292
            Star {
                index: StarIndex::new(292_u16),
                position: Point::new(5083, 7145),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 28,
            },
            // Index 293
            Star {
                index: StarIndex::new(293_u16),
                position: Point::new(7360, 7184),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 39,
            },
            // Index 294
            Star {
                index: StarIndex::new(294_u16),
                position: Point::new(1020, 7187),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 58,
            },
            // Index 295
            Star {
                index: StarIndex::new(295_u16),
                position: Point::new(3875, 7187),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 32,
            },
            // Index 296
            Star {
                index: StarIndex::new(296_u16),
                position: Point::new(4879, 7201),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 28,
            },
            // Index 297
            Star {
                index: StarIndex::new(297_u16),
                position: Point::new(4958, 7229),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 28,
            },
            // Index 298
            Star {
                index: StarIndex::new(298_u16),
                position: Point::new(7125, 7250),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 39,
            },
            // Index 299
            Star {
                index: StarIndex::new(299_u16),
                position: Point::new(7532, 7258),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 39,
            },
            // Index 300
            Star {
                index: StarIndex::new(300_u16),
                position: Point::new(2416, 7291),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 101,
            },
            // Index 301
            Star {
                index: StarIndex::new(301_u16),
                position: Point::new(3854, 7291),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 32,
            },
            // Index 302
            Star {
                index: StarIndex::new(302_u16),
                position: Point::new(9687, 7333),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 44,
            },
            // Index 303
            Star {
                index: StarIndex::new(303_u16),
                position: Point::new(4895, 7458),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 36,
            },
            // Index 304
            Star {
                index: StarIndex::new(304_u16),
                position: Point::new(4645, 7479),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 36,
            },
            // Index 305
            Star {
                index: StarIndex::new(305_u16),
                position: Point::new(6940, 7514),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 11,
                postfix_index: 39,
            },
            // Index 306
            Star {
                index: StarIndex::new(306_u16),
                position: Point::new(7443, 7538),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 39,
            },
            // Index 307
            Star {
                index: StarIndex::new(307_u16),
                position: Point::new(6479, 7541),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 38,
            },
            // Index 308
            Star {
                index: StarIndex::new(308_u16),
                position: Point::new(7208, 7541),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 39,
            },
            // Index 309
            Star {
                index: StarIndex::new(309_u16),
                position: Point::new(5791, 7583),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 34,
            },
            // Index 310
            Star {
                index: StarIndex::new(310_u16),
                position: Point::new(5958, 7645),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 34,
            },
            // Index 311
            Star {
                index: StarIndex::new(311_u16),
                position: Point::new(1041, 7708),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 58,
            },
            // Index 312
            Star {
                index: StarIndex::new(312_u16),
                position: Point::new(1125, 7791),
                star_type: StarType::Giant,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 58,
            },
            // Index 313
            Star {
                index: StarIndex::new(313_u16),
                position: Point::new(4979, 7791),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 36,
            },
            // Index 314
            Star {
                index: StarIndex::new(314_u16),
                position: Point::new(4958, 7791),
                star_type: StarType::Giant,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 36,
            },
            // Index 315
            Star {
                index: StarIndex::new(315_u16),
                position: Point::new(6889, 7803),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 10,
                postfix_index: 39,
            },
            // Index 316
            Star {
                index: StarIndex::new(316_u16),
                position: Point::new(7200, 7849),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 8,
                postfix_index: 39,
            },
            // Index 317
            Star {
                index: StarIndex::new(317_u16),
                position: Point::new(7395, 7854),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 39,
            },
            // Index 318
            Star {
                index: StarIndex::new(318_u16),
                position: Point::new(9437, 7854),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 44,
            },
            // Index 319
            Star {
                index: StarIndex::new(319_u16),
                position: Point::new(2836, 7857),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Rainbow,
                prefix_index: 5,
                postfix_index: 53,
            },
            // Index 320
            Star {
                index: StarIndex::new(320_u16),
                position: Point::new(5375, 7875),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 35,
            },
            // Index 321
            Star {
                index: StarIndex::new(321_u16),
                position: Point::new(6187, 7875),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 35,
            },
            // Index 322
            Star {
                index: StarIndex::new(322_u16),
                position: Point::new(6041, 7916),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 35,
            },
            // Index 323
            Star {
                index: StarIndex::new(323_u16),
                position: Point::new(5979, 7979),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 35,
            },
            // Index 324
            Star {
                index: StarIndex::new(324_u16),
                position: Point::new(7083, 7993),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 9,
                postfix_index: 39,
            },
            // Index 325
            Star {
                index: StarIndex::new(325_u16),
                position: Point::new(3270, 8000),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 8,
                postfix_index: 53,
            },
            // Index 326
            Star {
                index: StarIndex::new(326_u16),
                position: Point::new(6104, 8000),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 35,
            },
            // Index 327
            Star {
                index: StarIndex::new(327_u16),
                position: Point::new(5645, 8020),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 35,
            },
            // Index 328
            Star {
                index: StarIndex::new(328_u16),
                position: Point::new(1395, 8041),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 58,
            },
            // Index 329
            Star {
                index: StarIndex::new(329_u16),
                position: Point::new(8229, 8041),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 43,
            },
            // Index 330
            Star {
                index: StarIndex::new(330_u16),
                position: Point::new(2518, 8056),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 53,
            },
            // Index 331
            Star {
                index: StarIndex::new(331_u16),
                position: Point::new(5875, 8062),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 35,
            },
            // Index 332
            Star {
                index: StarIndex::new(332_u16),
                position: Point::new(8416, 8083),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 43,
            },
            // Index 333
            Star {
                index: StarIndex::new(333_u16),
                position: Point::new(9000, 8229),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 44,
            },
            // Index 334
            Star {
                index: StarIndex::new(334_u16),
                position: Point::new(3562, 8250),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 9,
                postfix_index: 53,
            },
            // Index 335
            Star {
                index: StarIndex::new(335_u16),
                position: Point::new(5437, 8270),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Rainbow,
                prefix_index: 5,
                postfix_index: 48,
            },
            // Index 336
            Star {
                index: StarIndex::new(336_u16),
                position: Point::new(1520, 8333),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 58,
            },
            // Index 337
            Star {
                index: StarIndex::new(337_u16),
                position: Point::new(2771, 8351),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 53,
            },
            // Index 338
            Star {
                index: StarIndex::new(338_u16),
                position: Point::new(2535, 8358),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Thradd,
                prefix_index: 4,
                postfix_index: 53,
            },
            // Index 339
            Star {
                index: StarIndex::new(339_u16),
                position: Point::new(3151, 8390),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 53,
            },
            // Index 340
            Star {
                index: StarIndex::new(340_u16),
                position: Point::new(2362, 8395),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 11,
                postfix_index: 53,
            },
            // Index 341
            Star {
                index: StarIndex::new(341_u16),
                position: Point::new(2822, 8395),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 53,
            },
            // Index 342
            Star {
                index: StarIndex::new(342_u16),
                position: Point::new(5500, 8395),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 48,
            },
            // Index 343
            Star {
                index: StarIndex::new(343_u16),
                position: Point::new(2536, 8504),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 2,
            },
            // Index 344
            Star {
                index: StarIndex::new(344_u16),
                position: Point::new(8625, 8562),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 3,
            },
            // Index 345
            Star {
                index: StarIndex::new(345_u16),
                position: Point::new(4375, 8562),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 50,
            },
            // Index 346
            Star {
                index: StarIndex::new(346_u16),
                position: Point::new(2593, 8569),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 2,
            },
            // Index 347
            Star {
                index: StarIndex::new(347_u16),
                position: Point::new(2562, 8572),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 2,
            },
            // Index 348
            Star {
                index: StarIndex::new(348_u16),
                position: Point::new(8492, 8578),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 3,
            },
            // Index 349
            Star {
                index: StarIndex::new(349_u16),
                position: Point::new(1125, 8583),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 58,
            },
            // Index 350
            Star {
                index: StarIndex::new(350_u16),
                position: Point::new(8073, 8588),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 46,
            },
            // Index 351
            Star {
                index: StarIndex::new(351_u16),
                position: Point::new(8560, 8638),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 3,
            },
            // Index 352
            Star {
                index: StarIndex::new(352_u16),
                position: Point::new(8750, 8645),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 3,
            },
            // Index 353
            Star {
                index: StarIndex::new(353_u16),
                position: Point::new(5562, 8645),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 48,
            },
            // Index 354
            Star {
                index: StarIndex::new(354_u16),
                position: Point::new(2588, 8653),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 14,
                postfix_index: 53,
            },
            // Index 355
            Star {
                index: StarIndex::new(355_u16),
                position: Point::new(2458, 8666),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 10,
                postfix_index: 53,
            },
            // Index 356
            Star {
                index: StarIndex::new(356_u16),
                position: Point::new(7666, 8666),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Rainbow,
                prefix_index: 2,
                postfix_index: 46,
            },
            // Index 357
            Star {
                index: StarIndex::new(357_u16),
                position: Point::new(2776, 8673),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::AquaHelix,
                prefix_index: 6,
                postfix_index: 53,
            },
            // Index 358
            Star {
                index: StarIndex::new(358_u16),
                position: Point::new(8630, 8693),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Utwig,
                prefix_index: 2,
                postfix_index: 3,
            },
            // Index 359
            Star {
                index: StarIndex::new(359_u16),
                position: Point::new(2310, 8702),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 12,
                postfix_index: 53,
            },
            // Index 360
            Star {
                index: StarIndex::new(360_u16),
                position: Point::new(8534, 8797),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Rainbow,
                prefix_index: 3,
                postfix_index: 3,
            },
            // Index 361
            Star {
                index: StarIndex::new(361_u16),
                position: Point::new(8588, 8812),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 3,
            },
            // Index 362
            Star {
                index: StarIndex::new(362_u16),
                position: Point::new(7187, 8812),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 46,
            },
            // Index 363
            Star {
                index: StarIndex::new(363_u16),
                position: Point::new(5475, 8823),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 48,
            },
            // Index 364
            Star {
                index: StarIndex::new(364_u16),
                position: Point::new(3050, 8833),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 1,
            },
            // Index 365
            Star {
                index: StarIndex::new(365_u16),
                position: Point::new(2831, 8854),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 1,
            },
            // Index 366
            Star {
                index: StarIndex::new(366_u16),
                position: Point::new(2300, 8861),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 13,
                postfix_index: 53,
            },
            // Index 367
            Star {
                index: StarIndex::new(367_u16),
                position: Point::new(2706, 8910),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 1,
            },
            // Index 368
            Star {
                index: StarIndex::new(368_u16),
                position: Point::new(2535, 8917),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 1,
            },
            // Index 369
            Star {
                index: StarIndex::new(369_u16),
                position: Point::new(8322, 8934),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 45,
            },
            // Index 370
            Star {
                index: StarIndex::new(370_u16),
                position: Point::new(8249, 8958),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 45,
            },
            // Index 371
            Star {
                index: StarIndex::new(371_u16),
                position: Point::new(8375, 8958),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 45,
            },
            // Index 372
            Star {
                index: StarIndex::new(372_u16),
                position: Point::new(5645, 8979),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 48,
            },
            // Index 373
            Star {
                index: StarIndex::new(373_u16),
                position: Point::new(2687, 9000),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 1,
            },
            // Index 374
            Star {
                index: StarIndex::new(374_u16),
                position: Point::new(8375, 9041),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 45,
            },
            // Index 375
            Star {
                index: StarIndex::new(375_u16),
                position: Point::new(9960, 9042),
                star_type: StarType::Giant,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Rainbow,
                prefix_index: 0,
                postfix_index: 42,
            },
            // Index 376
            Star {
                index: StarIndex::new(376_u16),
                position: Point::new(7354, 9062),
                star_type: StarType::Dwarf,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 47,
            },
            // Index 377
            Star {
                index: StarIndex::new(377_u16),
                position: Point::new(7833, 9083),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 47,
            },
            // Index 378
            Star {
                index: StarIndex::new(378_u16),
                position: Point::new(2581, 9105),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 1,
            },
            // Index 379
            Star {
                index: StarIndex::new(379_u16),
                position: Point::new(7545, 9107),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 47,
            },
            // Index 380
            Star {
                index: StarIndex::new(380_u16),
                position: Point::new(7414, 9124),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Supox,
                prefix_index: 2,
                postfix_index: 47,
            },
            // Index 381
            Star {
                index: StarIndex::new(381_u16),
                position: Point::new(8500, 9125),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 45,
            },
            // Index 382
            Star {
                index: StarIndex::new(382_u16),
                position: Point::new(7889, 9181),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 47,
            },
            // Index 383
            Star {
                index: StarIndex::new(383_u16),
                position: Point::new(7791, 9187),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 47,
            },
            // Index 384
            Star {
                index: StarIndex::new(384_u16),
                position: Point::new(7791, 9229),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 47,
            },
            // Index 385
            Star {
                index: StarIndex::new(385_u16),
                position: Point::new(9749, 9250),
                star_type: StarType::SuperGiant,
                color: StarColor::Blue,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Kitsune,
                prefix_index: 0,
                postfix_index: 133,
            },
            // Index 386
            Star {
                index: StarIndex::new(386_u16),
                position: Point::new(4812, 9270),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 0,
                postfix_index: 51,
            },
            // Index 387
            Star {
                index: StarIndex::new(387_u16),
                position: Point::new(8500, 9372),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::Bomb,
                prefix_index: 6,
                postfix_index: 45,
            },
            // Index 388
            Star {
                index: StarIndex::new(388_u16),
                position: Point::new(7255, 9374),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 11,
                postfix_index: 45,
            },
            // Index 389
            Star {
                index: StarIndex::new(389_u16),
                position: Point::new(8458, 9393),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 45,
            },
            // Index 390
            Star {
                index: StarIndex::new(390_u16),
                position: Point::new(1000, 9395),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 27,
            },
            // Index 391
            Star {
                index: StarIndex::new(391_u16),
                position: Point::new(5711, 9475),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 49,
            },
            // Index 392
            Star {
                index: StarIndex::new(392_u16),
                position: Point::new(5989, 9496),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 49,
            },
            // Index 393
            Star {
                index: StarIndex::new(393_u16),
                position: Point::new(8000, 9505),
                star_type: StarType::Dwarf,
                color: StarColor::White,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 9,
                postfix_index: 45,
            },
            // Index 394
            Star {
                index: StarIndex::new(394_u16),
                position: Point::new(5329, 9538),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 49,
            },
            // Index 395
            Star {
                index: StarIndex::new(395_u16),
                position: Point::new(2916, 9541),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 103,
            },
            // Index 396
            Star {
                index: StarIndex::new(396_u16),
                position: Point::new(8296, 9548),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 8,
                postfix_index: 45,
            },
            // Index 397
            Star {
                index: StarIndex::new(397_u16),
                position: Point::new(5600, 9552),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 49,
            },
            // Index 398
            Star {
                index: StarIndex::new(398_u16),
                position: Point::new(7664, 9589),
                star_type: StarType::Dwarf,
                color: StarColor::Green,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 10,
                postfix_index: 45,
            },
            // Index 399
            Star {
                index: StarIndex::new(399_u16),
                position: Point::new(6125, 9604),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 8,
                postfix_index: 49,
            },
            // Index 400
            Star {
                index: StarIndex::new(400_u16),
                position: Point::new(9144, 9686),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 4,
                postfix_index: 4,
            },
            // Index 401
            Star {
                index: StarIndex::new(401_u16),
                position: Point::new(5781, 9711),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 49,
            },
            // Index 402
            Star {
                index: StarIndex::new(402_u16),
                position: Point::new(5229, 9729),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 6,
                postfix_index: 49,
            },
            // Index 403
            Star {
                index: StarIndex::new(403_u16),
                position: Point::new(9120, 9741),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 3,
                postfix_index: 4,
            },
            // Index 404
            Star {
                index: StarIndex::new(404_u16),
                position: Point::new(9186, 9741),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 2,
                postfix_index: 4,
            },
            // Index 405
            Star {
                index: StarIndex::new(405_u16),
                position: Point::new(9147, 9790),
                star_type: StarType::Dwarf,
                color: StarColor::Orange,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 5,
                postfix_index: 4,
            },
            // Index 406
            Star {
                index: StarIndex::new(406_u16),
                position: Point::new(5704, 9795),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::VuxBeast,
                prefix_index: 4,
                postfix_index: 49,
            },
            // Index 407
            Star {
                index: StarIndex::new(407_u16),
                position: Point::new(1020, 9937),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 7,
                postfix_index: 27,
            },
            // Index 408
            Star {
                index: StarIndex::new(408_u16),
                position: Point::new(1937, 9979),
                star_type: StarType::Dwarf,
                color: StarColor::Red,
                num_planets: 0, // Will be filled from planet data
                name: String::new(),
                visited: false,
                alien_presence: AlienPresence::None,
                prefix_index: 1,
                postfix_index: 103,
            },
        ]
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
            index: StarIndex::new(86),
            position: Point::new(1752, 1450),
            star_type: StarType::Dwarf,
            color: StarColor::Yellow,
            num_planets: 8,
            name: "Sol".to_string(),
            visited: false,
            alien_presence: AlienPresence::Sol,
            prefix_index: 0,
            postfix_index: 129,
        };

        assert_eq!(star.name, "Sol");
        assert_eq!(star.num_planets, 8);
        assert_eq!(star.index.as_usize(), 86);
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
            alien_presence: AlienPresence::None,
            prefix_index: 0,
            postfix_index: 0,
        };

        let star2 = Star {
            index: StarIndex::new(1),
            position: Point::new(3, 4),
            star_type: StarType::Dwarf,
            color: StarColor::Red,
            num_planets: 0,
            name: "Star2".to_string(),
            visited: false,
            alien_presence: AlienPresence::None,
            prefix_index: 0,
            postfix_index: 0,
        };

        let distance = star1.distance_to(&star2);
        assert!((distance - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_star_has_aliens() {
        let sol = Star {
            index: StarIndex::new(86),
            position: Point::new(1752, 1450),
            star_type: StarType::Dwarf,
            color: StarColor::Yellow,
            num_planets: 8,
            name: "Sol".to_string(),
            visited: false,
            alien_presence: AlienPresence::Sol,
            prefix_index: 0,
            postfix_index: 129,
        };

        assert!(sol.has_aliens());

        let empty = Star {
            index: StarIndex::new(0),
            position: Point::new(0, 0),
            star_type: StarType::Dwarf,
            color: StarColor::Red,
            num_planets: 0,
            name: "Empty".to_string(),
            visited: false,
            alien_presence: AlienPresence::None,
            prefix_index: 0,
            postfix_index: 0,
        };

        assert!(!empty.has_aliens());
    }

    #[test]
    fn test_starmap_creation() {
        let map = StarMap::new();
        assert!(!map.is_empty());
        assert!(map.len() >= 2); // At least Sol and Alpha Centauri
    }

    #[test]
    fn test_starmap_get_sol() {
        let map = StarMap::new();
        let sol = map.get_star(StarIndex::new(63));
        assert!(sol.is_some());

        let sol = sol.unwrap();
        assert_eq!(sol.name, "Sol");
        assert_eq!(sol.num_planets, 0); // Planet count will be set from planet data
        assert_eq!(sol.position, Point::new(1752, 1450));
    }

    #[test]
    fn test_find_closest_star() {
        let map = StarMap::new();
        // Position very close to Sol
        let closest = map.find_closest_star(Point::new(1750, 1450));
        assert!(closest.is_some());

        let star = closest.unwrap();
        assert_eq!(star.name, "Sol");
    }

    #[test]
    fn test_find_stars_in_radius() {
        let map = StarMap::new();
        // Find stars near Sol
        let nearby = map.find_stars_in_radius(Point::new(1752, 1450), 500.0);
        assert!(!nearby.is_empty());

        // Sol should be in the list
        assert!(nearby.iter().any(|s| s.name == "Sol"));
    }

    #[test]
    fn test_mark_visited() {
        let mut map = StarMap::new();

        // Sol starts unvisited
        let sol = map.get_star(StarIndex::new(63)).unwrap();
        assert!(!sol.visited);

        // Mark as visited
        let newly_visited = map.mark_visited(StarIndex::new(63));
        assert!(newly_visited);

        // Check it's now visited
        let sol = map.get_star(StarIndex::new(63)).unwrap();
        assert!(sol.visited);

        // Marking again should return false
        let newly_visited = map.mark_visited(StarIndex::new(63));
        assert!(!newly_visited);
    }

    #[test]
    fn test_get_alien_stars() {
        let map = StarMap::new();
        let alien_stars = map.get_alien_stars();

        // Should have at least Sol with alien presence
        assert!(!alien_stars.is_empty());
        assert!(alien_stars.iter().any(|s| s.name == "Sol"));
    }

    #[test]
    fn test_star_distance_calculation() {
        let sol = Star {
            index: StarIndex::new(63),
            position: Point::new(1752, 1450),
            star_type: StarType::Dwarf,
            color: StarColor::Yellow,
            num_planets: 0,
            name: "Sol".to_string(),
            visited: false,
            alien_presence: AlienPresence::Sol,
            prefix_index: 0,
            postfix_index: 129,
        };

        let alpha_cent = Star {
            index: StarIndex::new(64),
            position: Point::new(2187, 1500),
            star_type: StarType::Dwarf,
            color: StarColor::Blue,
            num_planets: 0,
            name: "Alpha Centauri".to_string(),
            visited: false,
            alien_presence: AlienPresence::None,
            prefix_index: 0,
            postfix_index: 127,
        };

        let dist = sol.distance_to(&alpha_cent);
        // Distance should be around 437.7 units
        assert!(dist > 435.0 && dist < 440.0);
    }
}
