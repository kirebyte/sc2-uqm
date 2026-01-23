//! Melee combat system
//!
//! Ship-to-ship combat mechanics.

pub mod ai;
pub mod battle;
pub mod collision;
pub mod ship;
pub mod weapons;

pub use battle::BattleContext;
pub use ship::{Ship, ShipState, ShipType};
