//! Integration tests for the melee combat system

use uqm::core::Vec2;
use uqm::melee::{Ship, ShipState, ShipType};

#[test]
fn test_ship_creation() {
    let ship = Ship::new(ShipType::Earthling, Vec2::new(100.0, 200.0));
    
    assert_eq!(ship.ship_type, ShipType::Earthling);
    assert_eq!(ship.position, Vec2::new(100.0, 200.0));
    assert_eq!(ship.crew, 18);
    assert_eq!(ship.max_crew, 18);
    assert_eq!(ship.energy, 50);
    assert_eq!(ship.max_energy, 50);
    assert_eq!(ship.state, ShipState::Idle);
    assert!(ship.is_alive());
}

#[test]
fn test_ship_types_have_different_stats() {
    let earthling = Ship::new(ShipType::Earthling, Vec2::zero());
    let urquan = Ship::new(ShipType::UrQuan, Vec2::zero());
    let arilou = Ship::new(ShipType::Arilou, Vec2::zero());
    let spathi = Ship::new(ShipType::Spathi, Vec2::zero());
    
    // Each ship type should have unique stats
    assert_eq!(earthling.max_crew, 18);
    assert_eq!(urquan.max_crew, 42);
    assert_eq!(arilou.max_crew, 6);
    assert_eq!(spathi.max_crew, 20);
    
    assert_eq!(earthling.max_energy, 50);
    assert_eq!(urquan.max_energy, 100);
    assert_eq!(arilou.max_energy, 40);
    assert_eq!(spathi.max_energy, 60);
}

#[test]
fn test_ship_takes_damage() {
    let mut ship = Ship::new(ShipType::Earthling, Vec2::zero());
    
    ship.take_damage(5);
    assert_eq!(ship.crew, 13);
    assert!(ship.is_alive());
    assert_eq!(ship.state, ShipState::Idle);
    
    ship.take_damage(10);
    assert_eq!(ship.crew, 3);
    assert!(ship.is_alive());
    
    ship.take_damage(5);
    assert_eq!(ship.crew, 0);
    assert!(!ship.is_alive());
    assert_eq!(ship.state, ShipState::Dead);
}

#[test]
fn test_ship_energy_regeneration() {
    let mut ship = Ship::new(ShipType::Earthling, Vec2::zero());
    
    // Drain energy
    ship.energy = 10;
    
    // Update for 1 second
    ship.update(1.0);
    
    // Should have regenerated (5 energy per second)
    assert_eq!(ship.energy, 15);
}

#[test]
fn test_ship_position_update() {
    let mut ship = Ship::new(ShipType::Earthling, Vec2::zero());
    
    ship.velocity = Vec2::new(10.0, 20.0);
    ship.state = ShipState::Moving;
    
    // Update for 1 second
    ship.update(1.0);
    
    assert_eq!(ship.position, Vec2::new(10.0, 20.0));
    
    // Update for another 0.5 seconds
    ship.update(0.5);
    
    assert_eq!(ship.position, Vec2::new(15.0, 30.0));
}

#[test]
fn test_ship_saturation_damage() {
    let mut ship = Ship::new(ShipType::Earthling, Vec2::zero());
    
    // Try to deal more damage than crew
    ship.take_damage(100);
    
    assert_eq!(ship.crew, 0);
    assert!(!ship.is_alive());
}

#[test]
fn test_ship_energy_cap() {
    let mut ship = Ship::new(ShipType::Earthling, Vec2::zero());
    
    // Already at max energy
    assert_eq!(ship.energy, ship.max_energy);
    
    // Update should not exceed max
    ship.update(10.0);
    
    assert_eq!(ship.energy, ship.max_energy);
}
