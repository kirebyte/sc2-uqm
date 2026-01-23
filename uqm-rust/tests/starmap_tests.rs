//! Integration tests for the starmap system

use uqm::core::{Point, StarColor, StarIndex, StarType};
use uqm::starmap::{Star, StarMap};

#[test]
fn test_star_creation() {
    let star = Star {
        index: StarIndex::new(0),
        position: Point::new(1752, 1450),
        star_type: StarType::Dwarf,
        color: StarColor::Yellow,
        num_planets: 8,
        name: "Sol".to_string(),
        visited: false,
    };
    
    assert_eq!(star.name, "Sol");
    assert_eq!(star.num_planets, 8);
    assert_eq!(star.color, StarColor::Yellow);
}

#[test]
fn test_star_distance_calculation() {
    let sol = Star {
        index: StarIndex::new(0),
        position: Point::new(0, 0),
        star_type: StarType::Dwarf,
        color: StarColor::Yellow,
        num_planets: 8,
        name: "Sol".to_string(),
        visited: false,
    };
    
    let alpha_centauri = Star {
        index: StarIndex::new(1),
        position: Point::new(300, 400),
        star_type: StarType::Dwarf,
        color: StarColor::Red,
        num_planets: 3,
        name: "Alpha Centauri".to_string(),
        visited: false,
    };
    
    let distance = sol.distance_to(&alpha_centauri);
    
    // Distance should be 500 (3-4-5 triangle)
    assert!((distance - 500.0).abs() < 0.1);
}

#[test]
fn test_starmap_creation() {
    let map = StarMap::new();
    
    // Initially empty (star data will be loaded in Phase 5)
    assert_eq!(map.stars.len(), 0);
}

#[test]
fn test_starmap_get_star() {
    let mut map = StarMap::new();
    
    let sol = Star {
        index: StarIndex::new(0),
        position: Point::new(1752, 1450),
        star_type: StarType::Dwarf,
        color: StarColor::Yellow,
        num_planets: 8,
        name: "Sol".to_string(),
        visited: false,
    };
    
    map.stars.push(sol.clone());
    
    let retrieved = map.get_star(StarIndex::new(0));
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "Sol");
}

#[test]
fn test_find_closest_star() {
    let mut map = StarMap::new();
    
    map.stars.push(Star {
        index: StarIndex::new(0),
        position: Point::new(0, 0),
        star_type: StarType::Dwarf,
        color: StarColor::Yellow,
        num_planets: 1,
        name: "Star A".to_string(),
        visited: false,
    });
    
    map.stars.push(Star {
        index: StarIndex::new(1),
        position: Point::new(100, 0),
        star_type: StarType::Giant,
        color: StarColor::Red,
        num_planets: 3,
        name: "Star B".to_string(),
        visited: false,
    });
    
    map.stars.push(Star {
        index: StarIndex::new(2),
        position: Point::new(1000, 1000),
        star_type: StarType::SuperGiant,
        color: StarColor::Blue,
        num_planets: 5,
        name: "Star C".to_string(),
        visited: false,
    });
    
    // Find closest to origin
    let closest = map.find_closest_star(Point::new(0, 0));
    assert!(closest.is_some());
    assert_eq!(closest.unwrap().name, "Star A");
    
    // Find closest to (90, 0)
    let closest = map.find_closest_star(Point::new(90, 0));
    assert!(closest.is_some());
    assert_eq!(closest.unwrap().name, "Star B");
}
