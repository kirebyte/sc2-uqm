//! Game state management
//!
//! This module manages the overall game state, including player resources,
//! location, time, and progression flags.

use super::{GameFlags, GameTime, Point, StarIndex};
use serde::{Deserialize, Serialize};

/// Maximum fuel capacity
pub const MAX_FUEL: u32 = 10_000;

/// Maximum crew capacity  
pub const MAX_CREW: u32 = 100;

/// Maximum cargo hold capacity (minerals, bio data, etc.)
pub const MAX_CARGO: u32 = 500;

/// Fuel cost per unit of distance traveled in HyperSpace
pub const FUEL_PER_DISTANCE_UNIT: f32 = 0.5;

/// Main game state
///
/// Contains all persistent game data including player position,
/// resources, time, and progression flags.
///
/// # Examples
///
/// ```
/// use uqm::core::GameState;
///
/// let mut state = GameState::new();
/// state.add_fuel(100);
/// assert_eq!(state.fuel(), 1100);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Current location in HyperSpace
    location: Point,
    /// Current star system (if any)
    current_system: Option<StarIndex>,
    /// Available fuel units
    fuel: u32,
    /// Crew members
    crew: u32,
    /// Resource units (RU - Resource Units)
    resource_units: u32,
    /// Cargo space used (minerals, bio data, etc.)
    cargo_used: u32,
    /// Biological data units collected
    bio_data: u32,
    /// In-game time
    time: GameTime,
    /// Global progression flags
    flags: GameFlags,
    /// Visited star systems
    visited_systems: Vec<StarIndex>,
}

impl GameState {
    /// Create a new game with default starting conditions
    ///
    /// Starting at Sol (Earth) system with initial resources
    ///
    /// # Examples
    ///
    /// ```
    /// use uqm::core::GameState;
    ///
    /// let state = GameState::new();
    /// assert_eq!(state.fuel(), 1000);
    /// assert_eq!(state.crew(), 50);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            location: Point::new(1752, 1450), // Sol system coordinates
            current_system: Some(StarIndex::new(0)), // Sol is index 0
            fuel: 1000,
            crew: 50,
            resource_units: 0,
            cargo_used: 0,
            bio_data: 0,
            time: GameTime::new(),
            flags: GameFlags::new(),
            visited_systems: vec![StarIndex::new(0)],
        }
    }

    // === Getters ===

    /// Get current HyperSpace location
    #[must_use]
    pub fn location(&self) -> Point {
        self.location
    }

    /// Get current star system
    #[must_use]
    pub fn current_system(&self) -> Option<StarIndex> {
        self.current_system
    }

    /// Get current fuel amount
    #[must_use]
    pub fn fuel(&self) -> u32 {
        self.fuel
    }

    /// Get current crew count
    #[must_use]
    pub fn crew(&self) -> u32 {
        self.crew
    }

    /// Get resource units
    #[must_use]
    pub fn resource_units(&self) -> u32 {
        self.resource_units
    }

    /// Get cargo space used
    #[must_use]
    pub fn cargo_used(&self) -> u32 {
        self.cargo_used
    }

    /// Get available cargo space
    #[must_use]
    pub fn cargo_available(&self) -> u32 {
        MAX_CARGO.saturating_sub(self.cargo_used)
    }

    /// Get bio data collected
    #[must_use]
    pub fn bio_data(&self) -> u32 {
        self.bio_data
    }

    /// Get current game time
    #[must_use]
    pub fn time(&self) -> &GameTime {
        &self.time
    }

    /// Get game flags
    #[must_use]
    pub fn flags(&self) -> &GameFlags {
        &self.flags
    }

    /// Get mutable reference to game flags
    pub fn flags_mut(&mut self) -> &mut GameFlags {
        &mut self.flags
    }

    /// Get visited systems list
    #[must_use]
    pub fn visited_systems(&self) -> &[StarIndex] {
        &self.visited_systems
    }

    // === Location Management ===

    /// Set current HyperSpace location
    pub fn set_location(&mut self, location: Point) {
        self.location = location;
    }

    /// Set current star system
    pub fn set_current_system(&mut self, system: Option<StarIndex>) {
        self.current_system = system;
        if let Some(sys) = system {
            self.visit_system(sys);
        }
    }

    /// Calculate fuel cost to travel to a destination
    ///
    /// # Examples
    ///
    /// ```
    /// use uqm::core::{GameState, Point};
    ///
    /// let state = GameState::new();
    /// let destination = Point::new(1800, 1500);
    /// let cost = state.calculate_fuel_cost(destination);
    /// assert!(cost > 0);
    /// ```
    #[must_use]
    pub fn calculate_fuel_cost(&self, destination: Point) -> u32 {
        let distance = self.location.distance_to(destination);
        (distance * FUEL_PER_DISTANCE_UNIT).ceil() as u32
    }

    /// Check if player can afford to travel to a destination
    #[must_use]
    pub fn can_afford_travel(&self, destination: Point) -> bool {
        let cost = self.calculate_fuel_cost(destination);
        self.fuel >= cost
    }

    /// Travel to a new HyperSpace location
    ///
    /// Returns Ok(fuel_consumed) if successful, Err with remaining fuel if insufficient
    pub fn travel_to(&mut self, destination: Point) -> Result<u32, u32> {
        let fuel_cost = self.calculate_fuel_cost(destination);
        
        if self.fuel >= fuel_cost {
            self.fuel -= fuel_cost;
            self.location = destination;
            Ok(fuel_cost)
        } else {
            Err(self.fuel)
        }
    }

    // === System Tracking ===

    /// Check if a star system has been visited
    ///
    /// # Examples
    ///
    /// ```
    /// use uqm::core::{GameState, StarIndex};
    ///
    /// let state = GameState::new();
    /// assert!(state.has_visited(StarIndex::new(0))); // Sol
    /// assert!(!state.has_visited(StarIndex::new(1)));
    /// ```
    #[must_use]
    pub fn has_visited(&self, system: StarIndex) -> bool {
        self.visited_systems.contains(&system)
    }

    /// Mark a star system as visited
    pub fn visit_system(&mut self, system: StarIndex) {
        if !self.has_visited(system) {
            self.visited_systems.push(system);
        }
    }

    /// Get count of visited systems
    #[must_use]
    pub fn visited_count(&self) -> usize {
        self.visited_systems.len()
    }

    // === Fuel Management ===

    /// Add fuel (capped at MAX_FUEL)
    ///
    /// Returns the actual amount added
    pub fn add_fuel(&mut self, amount: u32) -> u32 {
        let old_fuel = self.fuel;
        self.fuel = self.fuel.saturating_add(amount).min(MAX_FUEL);
        self.fuel - old_fuel
    }

    /// Consume fuel
    ///
    /// Returns true if successful, false if insufficient fuel
    pub fn consume_fuel(&mut self, amount: u32) -> bool {
        if self.fuel >= amount {
            self.fuel -= amount;
            true
        } else {
            false
        }
    }

    /// Check if fuel tank is full
    #[must_use]
    pub fn is_fuel_full(&self) -> bool {
        self.fuel >= MAX_FUEL
    }

    /// Get fuel percentage (0.0 to 1.0)
    #[must_use]
    pub fn fuel_percentage(&self) -> f32 {
        self.fuel as f32 / MAX_FUEL as f32
    }

    // === Crew Management ===

    /// Add crew members (capped at MAX_CREW)
    ///
    /// Returns the actual amount added
    pub fn add_crew(&mut self, amount: u32) -> u32 {
        let old_crew = self.crew;
        self.crew = self.crew.saturating_add(amount).min(MAX_CREW);
        self.crew - old_crew
    }

    /// Lose crew members
    ///
    /// Returns true if successful, false if insufficient crew
    pub fn lose_crew(&mut self, amount: u32) -> bool {
        if self.crew >= amount {
            self.crew -= amount;
            true
        } else {
            false
        }
    }

    /// Check if crew quarters are full
    #[must_use]
    pub fn is_crew_full(&self) -> bool {
        self.crew >= MAX_CREW
    }

    /// Get crew percentage (0.0 to 1.0)
    #[must_use]
    pub fn crew_percentage(&self) -> f32 {
        self.crew as f32 / MAX_CREW as f32
    }

    // === Resource Management ===

    /// Add resource units
    pub fn add_resources(&mut self, amount: u32) {
        self.resource_units = self.resource_units.saturating_add(amount);
    }

    /// Spend resource units
    ///
    /// Returns true if successful, false if insufficient resources
    pub fn spend_resources(&mut self, amount: u32) -> bool {
        if self.resource_units >= amount {
            self.resource_units -= amount;
            true
        } else {
            false
        }
    }

    /// Check if player can afford a purchase
    #[must_use]
    pub fn can_afford(&self, cost: u32) -> bool {
        self.resource_units >= cost
    }

    // === Cargo Management ===

    /// Add cargo (minerals, artifacts, etc.)
    ///
    /// Returns the actual amount added (may be less if cargo is full)
    pub fn add_cargo(&mut self, amount: u32) -> u32 {
        let available = self.cargo_available();
        let to_add = amount.min(available);
        self.cargo_used += to_add;
        to_add
    }

    /// Remove cargo (selling minerals, etc.)
    ///
    /// Returns true if successful, false if insufficient cargo
    pub fn remove_cargo(&mut self, amount: u32) -> bool {
        if self.cargo_used >= amount {
            self.cargo_used -= amount;
            true
        } else {
            false
        }
    }

    /// Check if cargo hold is full
    #[must_use]
    pub fn is_cargo_full(&self) -> bool {
        self.cargo_used >= MAX_CARGO
    }

    /// Get cargo percentage (0.0 to 1.0)
    #[must_use]
    pub fn cargo_percentage(&self) -> f32 {
        self.cargo_used as f32 / MAX_CARGO as f32
    }

    /// Sell all cargo for resource units
    ///
    /// Typical conversion: 1 cargo unit = 1 RU
    /// Returns the amount of RU gained
    pub fn sell_cargo(&mut self) -> u32 {
        let ru_gained = self.cargo_used;
        self.add_resources(ru_gained);
        self.cargo_used = 0;
        ru_gained
    }

    // === Bio Data Management ===

    /// Add biological data units
    pub fn add_bio_data(&mut self, amount: u32) {
        self.bio_data = self.bio_data.saturating_add(amount);
    }

    // === Time Management ===

    /// Advance game time by days
    pub fn advance_time(&mut self, days: u32) {
        self.time.advance_days(days);
    }

    // === Game State Checks ===

    /// Check if the game is over (crew depleted)
    #[must_use]
    pub fn is_game_over(&self) -> bool {
        self.crew == 0
    }

    /// Check if player is critically low on fuel (< 10%)
    #[must_use]
    pub fn is_fuel_critical(&self) -> bool {
        self.fuel_percentage() < 0.1
    }

    /// Check if player is low on crew (< 25%)
    #[must_use]
    pub fn is_crew_critical(&self) -> bool {
        self.crew_percentage() < 0.25
    }

    /// Get overall ship health status (0.0 to 1.0)
    ///
    /// Based on crew percentage as a simple health metric
    #[must_use]
    pub fn ship_health(&self) -> f32 {
        self.crew_percentage()
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_state() {
        let state = GameState::new();
        assert_eq!(state.fuel(), 1000);
        assert_eq!(state.crew(), 50);
        assert_eq!(state.resource_units(), 0);
        assert_eq!(state.cargo_used(), 0);
        assert_eq!(state.bio_data(), 0);
        assert!(state.has_visited(StarIndex::new(0)));
        assert_eq!(state.visited_count(), 1);
    }

    #[test]
    fn test_getters() {
        let state = GameState::new();
        assert_eq!(state.location(), Point::new(1752, 1450));
        assert_eq!(state.current_system(), Some(StarIndex::new(0)));
        assert!(state.time().year() >= 2155);
    }

    #[test]
    fn test_fuel_management() {
        let mut state = GameState::new();
        
        let added = state.add_fuel(500);
        assert_eq!(added, 500);
        assert_eq!(state.fuel(), 1500);
        
        assert!(state.consume_fuel(200));
        assert_eq!(state.fuel(), 1300);
        
        assert!(!state.consume_fuel(2000));
        assert_eq!(state.fuel(), 1300);
    }

    #[test]
    fn test_fuel_cap() {
        let mut state = GameState::new();
        
        state.add_fuel(MAX_FUEL);
        assert!(state.is_fuel_full());
        assert_eq!(state.fuel(), MAX_FUEL);
        
        // Adding more shouldn't exceed cap
        state.add_fuel(1000);
        assert_eq!(state.fuel(), MAX_FUEL);
    }

    #[test]
    fn test_fuel_percentage() {
        let mut state = GameState::new();
        state.fuel = 5000;
        
        assert!((state.fuel_percentage() - 0.5).abs() < 0.01);
        
        state.fuel = MAX_FUEL;
        assert!((state.fuel_percentage() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_fuel_critical() {
        let mut state = GameState::new();
        state.fuel = 500;
        assert!(state.is_fuel_critical());
        
        state.fuel = 2000;
        assert!(!state.is_fuel_critical());
    }

    #[test]
    fn test_crew_management() {
        let mut state = GameState::new();
        
        let added = state.add_crew(10);
        assert_eq!(added, 10);
        assert_eq!(state.crew(), 60);
        
        assert!(state.lose_crew(20));
        assert_eq!(state.crew(), 40);
        
        assert!(!state.lose_crew(100));
        assert_eq!(state.crew(), 40);
    }

    #[test]
    fn test_crew_cap() {
        let mut state = GameState::new();
        
        state.add_crew(MAX_CREW);
        assert!(state.is_crew_full());
        assert_eq!(state.crew(), MAX_CREW);
        
        // Adding more shouldn't exceed cap
        state.add_crew(50);
        assert_eq!(state.crew(), MAX_CREW);
    }

    #[test]
    fn test_crew_percentage() {
        let mut state = GameState::new();
        state.crew = 50;
        
        assert!((state.crew_percentage() - 0.5).abs() < 0.01);
        
        state.crew = MAX_CREW;
        assert!((state.crew_percentage() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_crew_critical() {
        let mut state = GameState::new();
        state.crew = 20;
        assert!(state.is_crew_critical());
        
        state.crew = 50;
        assert!(!state.is_crew_critical());
    }

    #[test]
    fn test_resource_management() {
        let mut state = GameState::new();
        
        state.add_resources(1000);
        assert_eq!(state.resource_units(), 1000);
        
        assert!(state.can_afford(500));
        assert!(!state.can_afford(1500));
        
        assert!(state.spend_resources(300));
        assert_eq!(state.resource_units(), 700);
        
        assert!(!state.spend_resources(1000));
        assert_eq!(state.resource_units(), 700);
    }

    #[test]
    fn test_cargo_management() {
        let mut state = GameState::new();
        
        let added = state.add_cargo(100);
        assert_eq!(added, 100);
        assert_eq!(state.cargo_used(), 100);
        assert_eq!(state.cargo_available(), MAX_CARGO - 100);
        
        assert!(state.remove_cargo(50));
        assert_eq!(state.cargo_used(), 50);
        
        assert!(!state.remove_cargo(100));
    }

    #[test]
    fn test_cargo_cap() {
        let mut state = GameState::new();
        
        let added = state.add_cargo(MAX_CARGO + 100);
        assert_eq!(added, MAX_CARGO);
        assert!(state.is_cargo_full());
        assert_eq!(state.cargo_available(), 0);
    }

    #[test]
    fn test_cargo_percentage() {
        let mut state = GameState::new();
        state.cargo_used = 250;
        
        assert!((state.cargo_percentage() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_sell_cargo() {
        let mut state = GameState::new();
        
        state.add_cargo(200);
        let ru_gained = state.sell_cargo();
        
        assert_eq!(ru_gained, 200);
        assert_eq!(state.resource_units(), 200);
        assert_eq!(state.cargo_used(), 0);
    }

    #[test]
    fn test_bio_data() {
        let mut state = GameState::new();
        
        state.add_bio_data(50);
        assert_eq!(state.bio_data(), 50);
        
        state.add_bio_data(25);
        assert_eq!(state.bio_data(), 75);
    }

    #[test]
    fn test_location_management() {
        let mut state = GameState::new();
        
        let new_loc = Point::new(2000, 2000);
        state.set_location(new_loc);
        assert_eq!(state.location(), new_loc);
    }

    #[test]
    fn test_system_management() {
        let mut state = GameState::new();
        
        let new_system = StarIndex::new(5);
        state.set_current_system(Some(new_system));
        
        assert_eq!(state.current_system(), Some(new_system));
        assert!(state.has_visited(new_system));
    }

    #[test]
    fn test_visit_system() {
        let mut state = GameState::new();
        
        let new_system = StarIndex::new(5);
        assert!(!state.has_visited(new_system));
        assert_eq!(state.visited_count(), 1);
        
        state.visit_system(new_system);
        assert!(state.has_visited(new_system));
        assert_eq!(state.visited_count(), 2);
        
        // Visiting again shouldn't duplicate
        state.visit_system(new_system);
        assert_eq!(state.visited_count(), 2);
    }

    #[test]
    fn test_calculate_fuel_cost() {
        let state = GameState::new();
        
        let nearby = Point::new(1800, 1500);
        let cost = state.calculate_fuel_cost(nearby);
        assert!(cost > 0);
        
        let same_location = state.location();
        let zero_cost = state.calculate_fuel_cost(same_location);
        assert_eq!(zero_cost, 0);
    }

    #[test]
    fn test_can_afford_travel() {
        let mut state = GameState::new();
        state.fuel = 100;
        
        let nearby = Point::new(1760, 1460);
        assert!(state.can_afford_travel(nearby));
        
        let far_away = Point::new(5000, 5000);
        assert!(!state.can_afford_travel(far_away));
    }

    #[test]
    fn test_travel_to() {
        let mut state = GameState::new();
        let start_fuel = state.fuel();
        
        let destination = Point::new(1800, 1500);
        let result = state.travel_to(destination);
        
        assert!(result.is_ok());
        let fuel_consumed = result.unwrap();
        assert!(fuel_consumed > 0);
        assert_eq!(state.fuel(), start_fuel - fuel_consumed);
        assert_eq!(state.location(), destination);
    }

    #[test]
    fn test_travel_insufficient_fuel() {
        let mut state = GameState::new();
        state.fuel = 10;
        
        let far_away = Point::new(5000, 5000);
        let result = state.travel_to(far_away);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), 10);
    }

    #[test]
    fn test_advance_time() {
        let mut state = GameState::new();
        let start_days = state.time().total_days_since_start();
        
        state.advance_time(10);
        
        let end_days = state.time().total_days_since_start();
        assert_eq!(end_days, start_days + 10);
    }

    #[test]
    fn test_game_over() {
        let mut state = GameState::new();
        assert!(!state.is_game_over());
        
        state.lose_crew(state.crew());
        assert!(state.is_game_over());
    }

    #[test]
    fn test_ship_health() {
        let mut state = GameState::new();
        state.crew = 50;
        
        assert!((state.ship_health() - 0.5).abs() < 0.01);
        
        state.crew = MAX_CREW;
        assert!((state.ship_health() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_flags_access() {
        let mut state = GameState::new();
        
        // Test that we can access flags
        let _flags = state.flags();
        let _flags_mut = state.flags_mut();
    }

    #[test]
    fn test_serialization() {
        let state = GameState::new();
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: GameState = serde_json::from_str(&json).unwrap();
        
        assert_eq!(state.fuel(), deserialized.fuel());
        assert_eq!(state.crew(), deserialized.crew());
        assert_eq!(state.location(), deserialized.location());
    }
}
