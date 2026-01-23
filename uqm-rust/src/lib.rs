//! # UQM Rust - The Ur-Quan Masters Reimplementation
//!
//! This is a modern Rust reimplementation of The Ur-Quan Masters (Star Control 2),
//! a classic space adventure game from 1992.
//!
//! ## Architecture
//!
//! The codebase is organized into several core modules:
//!
//! - [`core`]: Core types, state management, and fundamental data structures
//! - [`graphics`]: Rendering engine using wgpu for modern GPU acceleration
//! - [`input`]: Input handling for keyboard, mouse, and gamepad
//! - [`audio`]: Audio system for music and sound effects
//! - [`starmap`]: Galaxy navigation and star system management
//! - [`melee`]: Space combat system
//! - [`planet`]: Planetary exploration and landing mechanics
//! - [`conversation`]: Dialogue system and narrative branching
//! - [`menu`]: UI and menu system
//! - [`resource`]: Asset loading and resource management
//!
//! ## Design Principles
//!
//! 1. **Type Safety**: Leverage Rust's type system to prevent bugs
//! 2. **Performance**: Match or exceed original C implementation
//! 3. **Maintainability**: Clear, documented, idiomatic Rust code
//! 4. **Testing**: Comprehensive test coverage with golden reference validation
//! 5. **Modularity**: Clean separation of concerns between modules

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

// Core modules
pub mod core;

// Subsystem modules
pub mod audio;
pub mod graphics;
pub mod input;

// Game logic modules
pub mod conversation;
pub mod melee;
pub mod menu;
pub mod planet;
pub mod starmap;

// Support modules
pub mod resource;
pub mod utils;

// FFI bindings (if needed for legacy C interop)
// TODO: Enable in future phases if needed
// #[cfg(feature = "ffi")]
// pub mod ffi;

// Re-export commonly used types
pub use core::{GameState, GlobalFlag, Point, StarColor, StarIndex, StarType};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Game constants
pub mod constants {
    /// Target frames per second
    pub const TARGET_FPS: u32 = 60;
    
    /// Frame time in milliseconds
    pub const FRAME_TIME_MS: f32 = 1000.0 / TARGET_FPS as f32;
    
    /// Number of star systems in the galaxy
    pub const NUM_STAR_SYSTEMS: usize = 503;
    
    /// Number of ship types available
    pub const NUM_SHIP_TYPES: usize = 25;
    
    /// Maximum number of moons per planet
    pub const MAX_MOONS: usize = 9;
    
    /// Fuel consumption rate per unit distance
    pub const FUEL_CONSUMPTION_RATE: f32 = 0.5;
    
    /// Earth radius constant for orbital calculations
    pub const EARTH_RADIUS: f32 = 100.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_exists() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_constants_valid() {
        assert!(constants::TARGET_FPS > 0);
        assert!(constants::NUM_STAR_SYSTEMS > 0);
        assert!(constants::NUM_SHIP_TYPES > 0);
    }
}
