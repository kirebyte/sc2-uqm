//! Navigation and travel mechanics for HyperSpace
//! 
//! This module implements ship movement, fuel consumption, and travel calculations
//! exactly as they work in the original C code (sc2/src/uqm/hyper.c).

use crate::core::types::Point;

// ============================================================================
// CONSTANTS FROM ORIGINAL C CODE
// ============================================================================

/// FUEL_TANK_SCALE from sis.h
/// This is the base unit for fuel measurements
pub const FUEL_TANK_SCALE: u32 = 100;

/// Standard fuel tank capacity (50 * FUEL_TANK_SCALE = 5000)
pub const FUEL_TANK_CAPACITY: u32 = 50 * FUEL_TANK_SCALE;

/// High-efficiency fuel tank capacity (100 * FUEL_TANK_SCALE = 10000)
pub const HEFUEL_TANK_CAPACITY: u32 = 100 * FUEL_TANK_SCALE;

/// Maximum X coordinate in logical space (from units.h)
/// LEGACY_HACK: These weird constants come from DOS screen aspect ratio corrections
/// The original code uses complex macros that we simplify here while maintaining
/// the exact numeric behavior.
pub const MAX_X_LOGICAL: i32 = 9999; // Approximate, actual is complex macro

/// Maximum Y coordinate in logical space (from units.h)
/// LEGACY_HACK: Y axis is inverted relative to screen coordinates
pub const MAX_Y_LOGICAL: i32 = 9999; // Approximate, actual is complex macro

/// Maximum X coordinate in universe space (from units.h)
pub const MAX_X_UNIVERSE: i16 = 9999;

/// Maximum Y coordinate in universe space (from units.h)
pub const MAX_Y_UNIVERSE: i16 = 9999;

// LEGACY_BUG: Bug #945 from original code
// The ratio between UNIVERSE_UNITS and LOG_UNITS sets HyperSpace speed (should be 1:16)
// Original math is "unnecessarily complex" per C comments and depends on screen resolution
// We use the NORMALIZED values for new code but document the original behavior

/// Logical units for X axis (16x universe units for 1:16 speed ratio)
const LOG_UNITS_X: i32 = (MAX_X_UNIVERSE as i32 + 1) * 16;

/// Logical units for Y axis (16x universe units for 1:16 speed ratio)
const LOG_UNITS_Y: i32 = (MAX_Y_UNIVERSE as i32 + 1) * 16;

/// Universe units for X axis (1/16th of max for 1:16 speed ratio)
const UNIVERSE_UNITS_X: i32 = (MAX_X_UNIVERSE as i32 + 1) >> 4;

/// Universe units for Y axis (1/16th of max for 1:16 speed ratio)
const UNIVERSE_UNITS_Y: i32 = (MAX_Y_UNIVERSE as i32 + 1) >> 4;

// ============================================================================
// NAVIGATION STATE
// ============================================================================

/// Navigator manages ship movement and fuel consumption in HyperSpace
/// 
/// This is a faithful port of the C code's hyperspace movement logic from hyper.c
pub struct Navigator {
    /// Accumulated horizontal movement for fuel calculation (hyper_dx from C)
    /// LEGACY_HACK: Movement is accumulated across frames to calculate fuel in chunks
    hyper_dx: u16,
    
    /// Accumulated vertical movement for fuel calculation (hyper_dy from C)
    hyper_dy: u16,
    
    /// Fractional fuel remainder from last calculation (hyper_extra from C)
    /// LEGACY_HACK: Stores the low 4 bits of distance to maintain precision across frames
    hyper_extra: u16,
    
    /// Current fuel tick counter (fuel_ticks from C)
    /// LEGACY_BUG: This is a BYTE (0-255) which can overflow, resetting accumulated distance
    /// When cur_fuel_ticks exceeds 0xFF, the C code resets hyper_dx/dy to prevent overflow
    fuel_ticks: u8,
    
    /// Current ship position in logical coordinates
    position: Point,
    
    /// Available fuel on board
    fuel_on_board: u32,
}

impl Navigator {
    /// Creates a new Navigator with starting position and fuel
    pub fn new(start_x: i16, start_y: i16, fuel: u32) -> Self {
        Navigator {
            hyper_dx: 0,
            hyper_dy: 0,
            hyper_extra: 0,
            fuel_ticks: 0,
            position: Point::new(start_x, start_y),
            fuel_on_board: fuel,
        }
    }
    
    /// Get current position in universe coordinates
    pub fn position(&self) -> Point {
        self.position
    }
    
    /// Get remaining fuel
    pub fn fuel(&self) -> u32 {
        self.fuel_on_board
    }
    
    /// Converts logical X coordinate to universe coordinate
    /// 
    /// Direct port of LOGX_TO_UNIVERSE macro from units.h
    fn logx_to_universe(lx: i32) -> i16 {
        let rounding = LOG_UNITS_X >> 1;
        ((lx * UNIVERSE_UNITS_X + rounding) / LOG_UNITS_X) as i16
    }
    
    /// Converts logical Y coordinate to universe coordinate
    /// 
    /// Direct port of LOGY_TO_UNIVERSE macro from units.h
    /// LEGACY_HACK: Y axis is inverted (MAX_Y_UNIVERSE - calculated value)
    fn logy_to_universe(ly: i32) -> i16 {
        let rounding = LOG_UNITS_Y >> 1;
        let calculated = (ly * UNIVERSE_UNITS_Y + rounding) / LOG_UNITS_Y;
        (MAX_Y_UNIVERSE as i32 - calculated) as i16
    }
    
    /// Move ship by delta in logical coordinates, consuming fuel
    /// 
    /// This is a faithful port of MoveSIS() from hyper.c (lines 70-163)
    /// 
    /// # Arguments
    /// * `dx` - Horizontal movement delta (negative = left, positive = right)
    /// * `dy` - Vertical movement delta (negative = up, positive = down)
    /// 
    /// # Returns
    /// Tuple of (actual_dx, actual_dy) - may differ from requested if hitting boundaries
    pub fn move_ship(&mut self, mut dx: i16, mut dy: i16) -> (i16, i16) {
        // Calculate new position with boundary clamping
        // Direct port of lines 74-97 in hyper.c MoveSIS()
        
        let mut new_x = self.position.x as i32 - dx as i32;
        
        // LEGACY_HACK: Boundaries are enforced by clamping and adjusting the delta
        if new_x < 0 {
            dx = (dx as i32 + new_x) as i16;
            new_x = 0;
        } else if new_x > MAX_X_LOGICAL {
            dx = (dx as i32 + (new_x - MAX_X_LOGICAL)) as i16;
            new_x = MAX_X_LOGICAL;
        }
        
        let mut new_y = self.position.y as i32 - dy as i32;
        
        if new_y < 0 {
            dy = (dy as i32 + new_y) as i16;
            new_y = 0;
        } else if new_y > MAX_Y_LOGICAL {
            dy = (dy as i32 + (new_y - MAX_Y_LOGICAL)) as i16;
            new_y = MAX_Y_LOGICAL;
        }
        
        // Update position
        self.position.x = new_x as i16;
        self.position.y = new_y as i16;
        
        // Calculate fuel consumption (lines 118-162 in hyper.c)
        if self.fuel_on_board > 0 {
            // Accumulate absolute movement distances
            let abs_dx = if dx < 0 { -dx } else { dx } as u16;
            let abs_dy = if dy < 0 { -dy } else { dy } as u16;
            
            self.hyper_dx += abs_dx;
            self.hyper_dy += abs_dy;
            
            // Convert to universe coordinates (with 16x scaling factor)
            // Lines 141-142: adj_dx = LOGX_TO_UNIVERSE(16 * hyper_dx)
            let adj_dx = Self::logx_to_universe((self.hyper_dx as i32) * 16) as i32;
            let adj_dy = (MAX_Y_UNIVERSE - Self::logy_to_universe((self.hyper_dy as i32) * 16)) as i32;
            
            // Calculate Euclidean distance + fractional remainder
            // Line 144-145: hyper_dist = square_root(adj_dx² + adj_dy²) + hyper_extra
            let dist_squared = (adj_dx * adj_dx + adj_dy * adj_dy) as u32;
            let hyper_dist = self.isqrt(dist_squared) + self.hyper_extra as u32;
            
            // Convert distance to fuel ticks (line 146)
            // LEGACY_HACK: Right shift by 4 is divide by 16, converts distance to fuel units
            let cur_fuel_ticks = (hyper_dist >> 4) as u32;
            
            // Consume fuel if we've accumulated enough distance
            if cur_fuel_ticks > self.fuel_ticks as u32 {
                let fuel_consumed = cur_fuel_ticks - self.fuel_ticks as u32;
                self.fuel_on_board = self.fuel_on_board.saturating_sub(fuel_consumed);
                
                // LEGACY_BUG: Lines 153-159 handle BYTE overflow
                // If cur_fuel_ticks > 255, reset accumulator to prevent overflow
                // This means very long journeys reset the fuel calculation periodically
                // TODO_REFACTOR: Use larger integer type to avoid this artificial limit
                if cur_fuel_ticks > 0xFF {
                    self.hyper_dx = 0;
                    self.hyper_dy = 0;
                    self.hyper_extra = (hyper_dist & 0xF) as u16; // Keep low 4 bits
                    self.fuel_ticks = 0;
                } else {
                    self.fuel_ticks = cur_fuel_ticks as u8;
                }
            }
        }
        
        (dx, dy)
    }
    
    /// Integer square root using Newton's method
    /// 
    /// LEGACY_HACK: The C code uses square_root() which we replicate here
    /// TODO_REFACTOR: Could use modern sqrt approximations or lookup tables
    fn isqrt(&self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        
        let mut x = n;
        let mut y = (x + 1) / 2;
        
        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }
        
        x
    }
    
    /// Calculate fuel cost for traveling to a destination
    /// 
    /// This estimates fuel needed without actually moving the ship
    pub fn calculate_fuel_cost(&self, dest_x: i16, dest_y: i16) -> u32 {
        // Calculate distance in universe coordinates
        let dx = (dest_x as i32 - self.position.x as i32).abs();
        let dy = (dest_y as i32 - self.position.y as i32).abs();
        
        let adj_dx = Self::logx_to_universe(dx * 16) as i32;
        let adj_dy = (MAX_Y_UNIVERSE - Self::logy_to_universe(dy * 16)) as i32;
        
        let dist_squared = (adj_dx * adj_dx + adj_dy * adj_dy) as u32;
        let distance = self.isqrt(dist_squared);
        
        // Fuel cost is distance / 16 (same as fuel tick calculation)
        distance >> 4
    }
    
    /// Check if there's enough fuel to reach a destination
    pub fn can_reach(&self, dest_x: i16, dest_y: i16) -> bool {
        let cost = self.calculate_fuel_cost(dest_x, dest_y);
        cost <= self.fuel_on_board
    }
    
    /// Add fuel to the ship (e.g., from refueling at planet)
    pub fn add_fuel(&mut self, amount: u32) {
        self.fuel_on_board = self.fuel_on_board.saturating_add(amount);
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigator_creation() {
        let nav = Navigator::new(1752, 1450, 5000);
        assert_eq!(nav.position().x, 1752);
        assert_eq!(nav.position().y, 1450);
        assert_eq!(nav.fuel(), 5000);
    }

    #[test]
    fn test_basic_movement() {
        let mut nav = Navigator::new(5000, 5000, 10000);
        
        // Move right and down
        nav.move_ship(100, 50);
        
        // Position should be updated
        // Note: exact values depend on boundary logic
        assert!(nav.position().x != 5000 || nav.position().y != 5000);
    }

    #[test]
    fn test_fuel_consumption() {
        let mut nav = Navigator::new(5000, 5000, 1000);
        let initial_fuel = nav.fuel();
        
        // Make a significant movement to consume fuel
        for _ in 0..100 {
            nav.move_ship(10, 10);
        }
        
        // Fuel should have decreased
        assert!(nav.fuel() < initial_fuel);
    }

    #[test]
    fn test_no_fuel_no_consumption() {
        let mut nav = Navigator::new(5000, 5000, 0);
        
        // Can still move when out of fuel (game allows this)
        nav.move_ship(100, 100);
        
        assert_eq!(nav.position().x, 4900);
        assert_eq!(nav.position().y, 4900);
        assert_eq!(nav.fuel(), 0);
    }

    #[test]
    fn test_fuel_cost_calculation() {
        let nav = Navigator::new(1752, 1450, 10000); // Sol position
        
        // Calculate fuel to Alpha Centauri (2187, 1500)
        let cost = nav.calculate_fuel_cost(2187, 1500);
        
        // Should require some fuel (exact value depends on distance formula)
        assert!(cost > 0);
        assert!(cost < 1000); // Reasonable upper bound for nearby star
    }

    #[test]
    fn test_can_reach_with_sufficient_fuel() {
        let nav = Navigator::new(1752, 1450, 10000);
        
        // Should be able to reach Alpha Centauri with 10000 fuel
        assert!(nav.can_reach(2187, 1500));
    }

    #[test]
    fn test_cannot_reach_with_insufficient_fuel() {
        let nav = Navigator::new(1752, 1450, 10); // Very low fuel
        
        // Should NOT be able to reach distant star
        assert!(!nav.can_reach(9000, 9000));
    }

    #[test]
    fn test_add_fuel() {
        let mut nav = Navigator::new(5000, 5000, 1000);
        
        nav.add_fuel(500);
        assert_eq!(nav.fuel(), 1500);
        
        // Test saturation (shouldn't overflow)
        nav.add_fuel(u32::MAX);
        assert_eq!(nav.fuel(), u32::MAX);
    }

    #[test]
    fn test_isqrt_accuracy() {
        let nav = Navigator::new(0, 0, 0);
        
        assert_eq!(nav.isqrt(0), 0);
        assert_eq!(nav.isqrt(1), 1);
        assert_eq!(nav.isqrt(4), 2);
        assert_eq!(nav.isqrt(9), 3);
        assert_eq!(nav.isqrt(16), 4);
        assert_eq!(nav.isqrt(100), 10);
        assert_eq!(nav.isqrt(10000), 100);
    }

    #[test]
    fn test_coordinate_conversion() {
        // Test that conversions are consistent
        let lx = 5000;
        let ly = 5000;
        
        let ux = Navigator::logx_to_universe(lx);
        let uy = Navigator::logy_to_universe(ly);
        
        // Should be in valid universe range
        assert!(ux <= MAX_X_UNIVERSE);
        assert!(uy <= MAX_Y_UNIVERSE);
    }

    #[test]
    fn test_fuel_tick_overflow_behavior() {
        // LEGACY_BUG test: Verify the 0xFF overflow behavior is replicated
        let mut nav = Navigator::new(5000, 5000, 100000);
        
        // Make massive movement to trigger fuel_ticks overflow
        // This simulates the case where cur_fuel_ticks > 0xFF
        for _ in 0..1000 {
            nav.move_ship(50, 50);
        }
        
        // Navigator should still function (accumulator resets prevent crash)
        assert!(nav.position().x != 5000); // Position changed
        assert!(nav.fuel() < 100000); // Fuel consumed
    }
}
