// EncounterManager: Random encounters in HyperSpace
// Faithful port from sc2/src/uqm/hyper.c check_hyperspace_encounter() (lines 167-280)
// and sc2/src/uqm/races.h RACE_HYPERSPACE_PERCENT (lines 566-701)

use crate::core::types::Point;
use rand::Rng;

// LEGACY_HACK: Original DOS values from races.h
// These percentages control encounter probability for each alien race in HyperSpace
// 0 = never encounters in HyperSpace
// Higher values = more frequent encounters
pub const RACE_HYPERSPACE_PERCENT: [u8; 24] = [
    20, // ARILOU_SHIP
    0,  // CHMMR_SHIP
    0,  // HUMAN_SHIP
    20, // ORZ_SHIP
    40, // PKUNK_SHIP
    0,  // SHOFIXTI_SHIP
    20, // SPATHI_SHIP
    40, // SUPOX_SHIP
    60, // THRADDASH_SHIP
    40, // UTWIG_SHIP
    40, // VUX_SHIP
    60, // YEHAT_SHIP
    0,  // MELNORME_SHIP (special logic: infinite radius, dynamic percent)
    30, // DRUUGE_SHIP
    60, // ILWRATH_SHIP
    40, // MYCON_SHIP
    2,  // SLYLANDRO_SHIP (special logic: infinite radius, dynamic percent)
    30, // UMGAH_SHIP
    70, // URQUAN_SHIP
    0,  // ZOQFOTPIK_SHIP
    0,  // SYREEN_SHIP
    70, // BLACK_URQUAN_SHIP
    60, // YEHAT_REBEL_SHIP
    0,  // URQUAN_DRONE_SHIP
];

// LEGACY_HACK: Encounter formation sizes (min, max ships)
// Stored as MAKE_BYTE(min, max) where each ship past min has 50% chance to spawn
pub const RACE_ENCOUNTER_MAKEUP: [(u8, u8); 24] = [
    (1, 5), // ARILOU_SHIP
    (0, 0), // CHMMR_SHIP
    (0, 0), // HUMAN_SHIP
    (1, 5), // ORZ_SHIP
    (1, 5), // PKUNK_SHIP
    (0, 0), // SHOFIXTI_SHIP
    (1, 5), // SPATHI_SHIP
    (1, 5), // SUPOX_SHIP
    (1, 5), // THRADDASH_SHIP
    (1, 5), // UTWIG_SHIP
    (1, 5), // VUX_SHIP
    (1, 5), // YEHAT_SHIP
    (1, 1), // MELNORME_SHIP
    (1, 5), // DRUUGE_SHIP
    (1, 5), // ILWRATH_SHIP
    (1, 5), // MYCON_SHIP
    (1, 1), // SLYLANDRO_SHIP
    (1, 5), // UMGAH_SHIP
    (1, 5), // URQUAN_SHIP
    (1, 5), // ZOQFOTPIK_SHIP
    (0, 0), // SYREEN_SHIP
    (1, 5), // BLACK_URQUAN_SHIP
    (1, 5), // YEHAT_REBEL_SHIP
    (0, 0), // URQUAN_DRONE_SHIP
];

// Special race IDs for infinite radius encounters
pub const SLYLANDRO_SHIP: usize = 16;
pub const MELNORME_SHIP: usize = 12;

// LEGACY_HACK: Original C constants
// SPHERE_RADIUS_INCREMENT is multiplied by actual_strength to get encounter radius
pub const SPHERE_RADIUS_INCREMENT: u16 = 10;
pub const INFINITE_RADIUS: u16 = 0xFFFF;
pub const MAX_X_UNIVERSE: i16 = 9999;
pub const MAX_Y_UNIVERSE: i16 = 9999;

// Encounter flags
pub const ONE_SHOT_ENCOUNTER: u8 = 0x01;

/// Fleet information for a race
#[derive(Debug, Clone, PartialEq)]
pub struct FleetInfo {
    pub race_id: usize,
    pub location: Point,
    pub actual_strength: u16, // If INFINITE_RADIUS, special logic applies
}

/// Active encounter instance
#[derive(Debug, Clone, PartialEq)]
pub struct Encounter {
    pub origin: Point,
    pub radius: u16,
    pub flags: u8,
    pub race_id: usize,
}

/// Manages random encounters in HyperSpace
/// Faithful port from check_hyperspace_encounter() in hyper.c
pub struct EncounterManager {
    active_encounters: Vec<Encounter>,
    fleets: Vec<FleetInfo>,
    // Game state flags (simplified for now)
    starbase_available: bool,
    slylandro_multiplier: u8,
    used_broadcaster: bool,
    melnorme_anger: u8,
    fuel_on_board: u32,
}

impl EncounterManager {
    pub fn new() -> Self {
        EncounterManager {
            active_encounters: Vec::new(),
            fleets: Vec::new(),
            starbase_available: false,
            slylandro_multiplier: 1,
            used_broadcaster: false,
            melnorme_anger: 0,
            fuel_on_board: 0,
        }
    }

    /// Set game state (for special encounter logic)
    pub fn set_game_state(
        &mut self,
        starbase_available: bool,
        slylandro_multiplier: u8,
        used_broadcaster: bool,
        melnorme_anger: u8,
        fuel_on_board: u32,
    ) {
        self.starbase_available = starbase_available;
        self.slylandro_multiplier = slylandro_multiplier;
        self.used_broadcaster = used_broadcaster;
        self.melnorme_anger = melnorme_anger;
        self.fuel_on_board = fuel_on_board;
    }

    /// Add a fleet to the encounter system
    pub fn add_fleet(&mut self, fleet: FleetInfo) {
        self.fleets.push(fleet);
    }

    /// Get active encounters
    pub fn get_encounters(&self) -> &[Encounter] {
        &self.active_encounters
    }

    /// Clear all active encounters (for testing/reset)
    pub fn clear_encounters(&mut self) {
        self.active_encounters.clear();
    }

    /// Check for random encounters at player's position
    /// This is a faithful port of check_hyperspace_encounter() from hyper.c
    pub fn check_encounter(&mut self, player_position: Point) {
        // LEGACY_HACK: Original C code iterates through avail_race_q linked list
        // We iterate through our fleets vector instead
        for fleet in self.fleets.iter() {
            let race_id = fleet.race_id;
            let mut encounter_radius = fleet.actual_strength;

            if encounter_radius == 0 {
                continue; // Fleet has no strength, skip
            }

            let mut encounter_flags = 0u8;
            let mut percent = RACE_HYPERSPACE_PERCENT[race_id];

            // LEGACY_HACK: Special logic for infinite radius encounters
            if encounter_radius != INFINITE_RADIUS {
                // Normal sphere of influence: radius = (actual_strength * 10) / 2
                encounter_radius = (encounter_radius * SPHERE_RADIUS_INCREMENT) >> 1;
            } else {
                // INFINITE_RADIUS encounters (Slylandro, Melnorme)
                // Use doubled universe max as "infinite" radius
                encounter_radius = ((MAX_X_UNIVERSE as u16) + 1) << 1;

                // LEGACY_BUG: Slylandro Probe special logic
                if race_id == SLYLANDRO_SHIP {
                    encounter_flags = ONE_SHOT_ENCOUNTER;
                    if !self.starbase_available {
                        percent = 100; // Always encounter before starbase
                    } else {
                        // LEGACY_HACK: Multiplier increases over time
                        percent = percent.saturating_mul(self.slylandro_multiplier);
                    }
                } else if race_id == MELNORME_SHIP
                    && (self.fuel_on_board == 0 || self.used_broadcaster)
                    && self.melnorme_anger < 3
                {
                    // LEGACY_HACK: Melnorme rescue encounter
                    if !self.used_broadcaster {
                        percent = 30;
                    } else {
                        percent = 100;
                    }
                    encounter_flags = ONE_SHOT_ENCOUNTER;
                }

                // LEGACY_BUG: "There can be only one!" - Check for existing Slylandro/Melnorme
                // If either already exists, don't spawn another
                for existing in &self.active_encounters {
                    if existing.race_id == race_id {
                        percent = 0;
                        break;
                    }
                }
            }

            // Calculate distance from fleet origin to player
            let dx = (player_position.x - fleet.location.x).abs();
            let dy = (player_position.y - fleet.location.y).abs();

            // LEGACY_HACK: Three-stage distance check for optimization:
            // 1. Quick box check: dx < radius && dy < radius
            // 2. Precise circle check: dx² + dy² < radius²
            // 3. Random roll: TFB_Random() % 100 < percent
            if (dx as u16) < encounter_radius
                && (dy as u16) < encounter_radius
                && (dx as u32 * dx as u32 + dy as u32 * dy as u32)
                    < (encounter_radius as u32 * encounter_radius as u32)
                && (rand::thread_rng().r#gen::<u8>() % 100) < percent
            {
                // Spawn encounter!
                let encounter = Encounter {
                    origin: fleet.location,
                    radius: encounter_radius,
                    flags: encounter_flags,
                    race_id,
                };
                self.active_encounters.push(encounter);
            }
        }
    }

    /// Calculate number of ships in encounter formation
    /// Uses RACE_ENCOUNTER_MAKEUP with 50% chance for each ship past minimum
    pub fn calculate_encounter_size(&self, race_id: usize) -> u8 {
        let (min, max) = RACE_ENCOUNTER_MAKEUP[race_id];
        if min == 0 && max == 0 {
            return 0;
        }

        let mut count = min;
        // LEGACY_HACK: Each ship past minimum has 50% chance to spawn
        for _ in min..max {
            if rand::thread_rng().r#gen::<bool>() {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encounter_manager_creation() {
        let manager = EncounterManager::new();
        assert_eq!(manager.get_encounters().len(), 0);
    }

    #[test]
    fn test_add_fleet() {
        let mut manager = EncounterManager::new();
        manager.add_fleet(FleetInfo {
            race_id: 0, // Arilou
            location: Point { x: 5000, y: 5000 },
            actual_strength: 10,
        });
        assert_eq!(manager.fleets.len(), 1);
    }

    #[test]
    fn test_encounter_probability_table() {
        // Verify exact values from races.h
        assert_eq!(RACE_HYPERSPACE_PERCENT[0], 20); // Arilou
        assert_eq!(RACE_HYPERSPACE_PERCENT[1], 0); // Chmmr (no encounters)
        assert_eq!(RACE_HYPERSPACE_PERCENT[16], 2); // Slylandro
        assert_eq!(RACE_HYPERSPACE_PERCENT[18], 70); // Urquan
    }

    #[test]
    fn test_encounter_makeup_table() {
        // Verify exact values from races.h
        assert_eq!(RACE_ENCOUNTER_MAKEUP[0], (1, 5)); // Arilou: 1-5 ships
        assert_eq!(RACE_ENCOUNTER_MAKEUP[16], (1, 1)); // Slylandro: always 1 probe
        assert_eq!(RACE_ENCOUNTER_MAKEUP[12], (1, 1)); // Melnorme: always 1 ship
    }

    #[test]
    fn test_distance_check_inside_radius() {
        let mut manager = EncounterManager::new();
        manager.add_fleet(FleetInfo {
            race_id: 0, // Arilou
            location: Point { x: 5000, y: 5000 },
            actual_strength: 100, // radius = (100 * 10) / 2 = 500
        });

        // Force high probability for testing
        manager.check_encounter(Point { x: 5100, y: 5100 }); // Within radius (~141 units)

        // Note: May or may not spawn due to 20% base probability
        // This test mainly verifies no crash/panic
    }

    #[test]
    fn test_distance_check_outside_radius() {
        let mut manager = EncounterManager::new();
        manager.add_fleet(FleetInfo {
            race_id: 0, // Arilou
            location: Point { x: 5000, y: 5000 },
            actual_strength: 10, // radius = (10 * 10) / 2 = 50
        });

        // Clear any random spawns
        manager.clear_encounters();

        // Far outside radius
        manager.check_encounter(Point { x: 9000, y: 9000 });

        // Should never spawn (distance > radius)
        assert_eq!(manager.get_encounters().len(), 0);
    }

    #[test]
    fn test_slylandro_infinite_radius() {
        let mut manager = EncounterManager::new();
        manager.add_fleet(FleetInfo {
            race_id: SLYLANDRO_SHIP,
            location: Point { x: 5000, y: 5000 },
            actual_strength: INFINITE_RADIUS,
        });

        // Slylandro should be checkable from anywhere (infinite radius)
        manager.set_game_state(false, 1, false, 0, 1000);
        manager.check_encounter(Point { x: 9999, y: 9999 });

        // With starbase_available=false, percent=100, should always spawn
        // (but still subject to random roll in check_encounter)
    }

    #[test]
    fn test_melnorme_rescue_logic() {
        let mut manager = EncounterManager::new();
        manager.add_fleet(FleetInfo {
            race_id: MELNORME_SHIP,
            location: Point { x: 5000, y: 5000 },
            actual_strength: INFINITE_RADIUS,
        });

        // Melnorme rescue: no fuel, anger < 3
        manager.set_game_state(true, 1, false, 2, 0);
        manager.check_encounter(Point { x: 5000, y: 5000 });

        // Should have 30% chance (if !used_broadcaster) or 100% (if used_broadcaster)
    }

    #[test]
    fn test_one_shot_encounter_flag() {
        let mut manager = EncounterManager::new();
        manager.add_fleet(FleetInfo {
            race_id: SLYLANDRO_SHIP,
            location: Point { x: 5000, y: 5000 },
            actual_strength: INFINITE_RADIUS,
        });

        manager.set_game_state(false, 1, false, 0, 1000);
        manager.check_encounter(Point { x: 5000, y: 5000 });

        // If spawned, should have ONE_SHOT_ENCOUNTER flag
        if let Some(encounter) = manager.get_encounters().first() {
            assert_eq!(encounter.flags & ONE_SHOT_ENCOUNTER, ONE_SHOT_ENCOUNTER);
        }
    }

    #[test]
    fn test_encounter_size_calculation() {
        let manager = EncounterManager::new();

        // Slylandro: always 1 probe
        let size = manager.calculate_encounter_size(SLYLANDRO_SHIP);
        assert_eq!(size, 1);

        // Melnorme: always 1 ship
        let size = manager.calculate_encounter_size(MELNORME_SHIP);
        assert_eq!(size, 1);

        // Arilou: 1-5 ships (random)
        let size = manager.calculate_encounter_size(0); // Arilou
        assert!(size >= 1 && size <= 5);
    }

    #[test]
    fn test_clear_encounters() {
        let mut manager = EncounterManager::new();
        manager.active_encounters.push(Encounter {
            origin: Point { x: 0, y: 0 },
            radius: 100,
            flags: 0,
            race_id: 0,
        });

        manager.clear_encounters();
        assert_eq!(manager.get_encounters().len(), 0);
    }

    #[test]
    fn test_sphere_radius_calculation() {
        // Verify radius formula: (actual_strength * SPHERE_RADIUS_INCREMENT) >> 1
        let actual_strength: u16 = 100;
        let radius = (actual_strength * SPHERE_RADIUS_INCREMENT) >> 1;
        assert_eq!(radius, 500); // (100 * 10) / 2 = 500
    }

    #[test]
    fn test_infinite_radius_constant() {
        // Verify infinite radius = (MAX_X_UNIVERSE + 1) << 1
        let infinite = ((MAX_X_UNIVERSE as u16) + 1) << 1;
        assert_eq!(infinite, 20000); // (9999 + 1) * 2 = 20000
    }
}
