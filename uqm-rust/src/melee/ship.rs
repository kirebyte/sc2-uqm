//! Ship data structures and mechanics

use crate::core::Vec2;
use serde::{Deserialize, Serialize};

/// Ship state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ShipState {
    /// Idle, not moving
    Idle,
    /// Moving with velocity
    Moving,
    /// In combat
    InCombat,
    /// Destroyed
    Dead,
}

/// Ship type identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShipType {
    /// Earthling Cruiser
    Earthling,
    /// Ur-Quan Dreadnought
    UrQuan,
    /// Arilou Skiff
    Arilou,
    /// Spathi Eluder
    Spathi,
}

/// Ship instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
    /// Ship type
    pub ship_type: ShipType,
    /// Current position
    pub position: Vec2,
    /// Current velocity
    pub velocity: Vec2,
    /// Facing direction (0-31, representing 360 degrees)
    pub facing: u8,
    /// Current energy
    pub energy: u16,
    /// Maximum energy
    pub max_energy: u16,
    /// Current crew
    pub crew: u8,
    /// Maximum crew
    pub max_crew: u8,
    /// Ship state
    pub state: ShipState,
}

impl Ship {
    /// Create a new ship
    #[must_use]
    pub fn new(ship_type: ShipType, position: Vec2) -> Self {
        let (max_crew, max_energy) = match ship_type {
            ShipType::Earthling => (18, 50),
            ShipType::UrQuan => (42, 100),
            ShipType::Arilou => (6, 40),
            ShipType::Spathi => (20, 60),
        };

        Self {
            ship_type,
            position,
            velocity: Vec2::zero(),
            facing: 0,
            energy: max_energy,
            max_energy,
            crew: max_crew,
            max_crew,
            state: ShipState::Idle,
        }
    }

    /// Take damage
    pub fn take_damage(&mut self, damage: u16) {
        self.crew = self.crew.saturating_sub(damage as u8);
        if self.crew == 0 {
            self.state = ShipState::Dead;
        }
    }

    /// Update ship state
    pub fn update(&mut self, dt: f32) {
        // Update position based on velocity
        self.position += self.velocity * dt;

        // Regenerate energy
        let regen = (5.0 * dt) as u16;
        self.energy = self.energy.saturating_add(regen).min(self.max_energy);
    }

    /// Check if ship is alive
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.crew > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship_creation() {
        let ship = Ship::new(ShipType::Earthling, Vec2::zero());
        assert_eq!(ship.crew, 18);
        assert_eq!(ship.max_energy, 50);
        assert!(ship.is_alive());
    }

    #[test]
    fn test_ship_damage() {
        let mut ship = Ship::new(ShipType::Earthling, Vec2::zero());
        ship.take_damage(10);
        assert_eq!(ship.crew, 8);
        assert!(ship.is_alive());
        
        ship.take_damage(20);
        assert_eq!(ship.crew, 0);
        assert!(!ship.is_alive());
    }
}
