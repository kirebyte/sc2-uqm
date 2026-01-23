//! Core data types and state management
//!
//! This module contains the fundamental types used throughout the game,
//! including geometric primitives, game state, and type-safe wrappers.

pub mod flags;
pub mod state;
pub mod time;
pub mod types;

// Re-export commonly used types
pub use flags::{GameFlags, GlobalFlag};
pub use state::GameState;
pub use time::GameTime;
pub use types::{Point, PlanetType, StarColor, StarIndex, StarType, Vec2};
