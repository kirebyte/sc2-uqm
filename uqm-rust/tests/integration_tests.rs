//! Integration tests for the core game state
//!
//! These tests verify that the core game state management works correctly
//! across module boundaries.

use uqm::core::{GameState, Point, StarIndex};

#[test]
fn test_new_game_initialization() {
    let state = GameState::new();
    
    // Verify initial conditions match original game
    assert_eq!(state.location, Point::new(1752, 1450), "Should start at Sol coordinates");
    assert_eq!(state.current_system, Some(StarIndex::new(0)), "Should start at Sol system");
    assert_eq!(state.fuel, 1000, "Should start with 1000 fuel");
    assert_eq!(state.crew, 50, "Should start with 50 crew");
    assert_eq!(state.resource_units, 0, "Should start with 0 RU");
    
    // Verify time starts at canonical date
    assert_eq!(state.time.year(), 2155);
    assert_eq!(state.time.month(), 2);
    assert_eq!(state.time.day(), 17);
}

#[test]
fn test_fuel_management_integration() {
    let mut state = GameState::new();
    
    // Test fuel addition
    state.add_fuel(500);
    assert_eq!(state.fuel, 1500);
    
    // Test fuel consumption success
    assert!(state.consume_fuel(100));
    assert_eq!(state.fuel, 1400);
    
    // Test fuel consumption failure
    assert!(!state.consume_fuel(2000));
    assert_eq!(state.fuel, 1400, "Fuel should not change on failed consumption");
}

#[test]
fn test_resource_economy() {
    let mut state = GameState::new();
    
    // Collect resources
    state.add_resources(500);
    assert_eq!(state.resource_units, 500);
    
    // Purchase fuel with resources
    assert!(state.spend_resources(100));
    state.add_fuel(200);
    
    assert_eq!(state.resource_units, 400);
    assert_eq!(state.fuel, 1200);
}

#[test]
fn test_star_system_exploration() {
    let mut state = GameState::new();
    
    // Should start with Sol visited
    assert!(state.has_visited(StarIndex::new(0)));
    
    // Visit new systems
    let alpha_centauri = StarIndex::new(1);
    assert!(!state.has_visited(alpha_centauri));
    
    state.visit_system(alpha_centauri);
    assert!(state.has_visited(alpha_centauri));
    
    // Visiting again should not duplicate
    state.visit_system(alpha_centauri);
    assert_eq!(state.visited_systems.len(), 2);
}

#[test]
fn test_time_progression() {
    let mut state = GameState::new();
    let initial_time = state.time;
    
    // Advance time by 10 days
    state.time.advance_days(10);
    
    assert_ne!(state.time, initial_time);
    assert_eq!(state.time.day(), 27);
}

#[test]
fn test_game_over_condition() {
    let mut state = GameState::new();
    
    assert!(!state.is_game_over());
    
    // Lose all crew
    state.lose_crew(50);
    
    assert!(state.is_game_over());
    assert_eq!(state.crew, 0);
}

#[test]
fn test_saturating_arithmetic() {
    let mut state = GameState::new();
    
    // Test fuel saturation
    state.add_fuel(u32::MAX);
    assert_eq!(state.fuel, u32::MAX);
    
    // Test resource saturation
    state.add_resources(u32::MAX);
    assert_eq!(state.resource_units, u32::MAX);
}
