//! Game flags and event tracking
//!
//! This module manages global game flags that track story progression,
//! quest completion, and discovered information.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Global game flags for tracking story progression
///
/// Flags are used to track which events have occurred, which aliens
/// have been encountered, which quests are completed, etc.
///
/// # Examples
///
/// ```
/// use uqm::core::{GameFlags, GlobalFlag};
///
/// let mut flags = GameFlags::new();
/// flags.set(GlobalFlag::GameStarted);
/// assert!(flags.is_set(GlobalFlag::GameStarted));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameFlags {
    /// Set of active flags
    flags: HashSet<GlobalFlag>,
}

impl GameFlags {
    /// Create a new empty set of flags
    #[must_use]
    pub fn new() -> Self {
        Self {
            flags: HashSet::new(),
        }
    }

    /// Create flags with initial game start flags
    #[must_use]
    pub fn new_game() -> Self {
        let mut flags = Self::new();
        flags.set(GlobalFlag::GameStarted);
        flags
    }

    /// Set a flag as active
    ///
    /// # Examples
    ///
    /// ```
    /// use uqm::core::{GameFlags, GlobalFlag};
    ///
    /// let mut flags = GameFlags::new();
    /// flags.set(GlobalFlag::MetUrQuan);
    /// assert!(flags.is_set(GlobalFlag::MetUrQuan));
    /// ```
    pub fn set(&mut self, flag: GlobalFlag) {
        self.flags.insert(flag);
    }

    /// Set multiple flags at once
    pub fn set_many(&mut self, flags: &[GlobalFlag]) {
        for &flag in flags {
            self.flags.insert(flag);
        }
    }

    /// Clear a flag
    pub fn clear(&mut self, flag: GlobalFlag) {
        self.flags.remove(&flag);
    }

    /// Clear multiple flags at once
    pub fn clear_many(&mut self, flags: &[GlobalFlag]) {
        for &flag in flags {
            self.flags.remove(&flag);
        }
    }

    /// Check if a flag is set
    ///
    /// # Examples
    ///
    /// ```
    /// use uqm::core::{GameFlags, GlobalFlag};
    ///
    /// let mut flags = GameFlags::new();
    /// assert!(!flags.is_set(GlobalFlag::GameStarted));
    /// flags.set(GlobalFlag::GameStarted);
    /// assert!(flags.is_set(GlobalFlag::GameStarted));
    /// ```
    #[must_use]
    pub fn is_set(&self, flag: GlobalFlag) -> bool {
        self.flags.contains(&flag)
    }

    /// Check if any of the given flags are set
    #[must_use]
    pub fn is_any_set(&self, flags: &[GlobalFlag]) -> bool {
        flags.iter().any(|flag| self.flags.contains(flag))
    }

    /// Check if all of the given flags are set
    #[must_use]
    pub fn are_all_set(&self, flags: &[GlobalFlag]) -> bool {
        flags.iter().all(|flag| self.flags.contains(flag))
    }

    /// Toggle a flag (set if clear, clear if set)
    pub fn toggle(&mut self, flag: GlobalFlag) {
        if self.flags.contains(&flag) {
            self.flags.remove(&flag);
        } else {
            self.flags.insert(flag);
        }
    }

    /// Get all active flags
    #[must_use]
    pub fn all_active(&self) -> Vec<GlobalFlag> {
        let mut result: Vec<GlobalFlag> = self.flags.iter().copied().collect();
        result.sort_by_key(|f| format!("{:?}", f));
        result
    }

    /// Get count of active flags
    #[must_use]
    pub fn count(&self) -> usize {
        self.flags.len()
    }

    /// Clear all flags
    pub fn clear_all(&mut self) {
        self.flags.clear();
    }

    /// Check if game has been completed (final flag set)
    #[must_use]
    pub fn is_game_completed(&self) -> bool {
        self.is_set(GlobalFlag::DestroyedSaMatra)
    }

    /// Get flags by category
    #[must_use]
    pub fn get_alien_encounters(&self) -> Vec<GlobalFlag> {
        use GlobalFlag::*;
        let alien_flags = [MetUrQuan, MetArilou, MetSpathi, MetPkunk, MetOrz];
        alien_flags
            .into_iter()
            .filter(|flag| self.is_set(*flag))
            .collect()
    }

    /// Get alliance status flags
    #[must_use]
    pub fn get_alliances(&self) -> Vec<GlobalFlag> {
        use GlobalFlag::*;
        let alliance_flags = [SpathiAllied, PkunkAllied, OrzAllied];
        alliance_flags
            .into_iter()
            .filter(|flag| self.is_set(*flag))
            .collect()
    }

    /// Check quest prerequisites
    ///
    /// Returns true if player can access certain quests
    #[must_use]
    pub fn can_access_quest(&self, quest: QuestType) -> bool {
        match quest {
            QuestType::ShofixtiQuest => self.is_set(GlobalFlag::MetCommander),
            QuestType::UltronQuest => self.is_set(GlobalFlag::VisitedStarbase),
            QuestType::FinalMission => {
                self.is_set(GlobalFlag::ObtainedUltron) && !self.is_set(GlobalFlag::DestroyedSaMatra)
            }
        }
    }
}

impl Default for GameFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Quest types for prerequisite checking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestType {
    /// Shofixti survivor quest
    ShofixtiQuest,
    /// Ultron device quest
    UltronQuest,
    /// Final Sa-Matra mission
    FinalMission,
}

/// Global flags for tracking game events
///
/// These correspond to major story events and quest progressions
/// in the original game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GlobalFlag {
    // Starting game flags
    /// Game has been started
    GameStarted,
    /// Intro sequence viewed
    IntroViewed,
    
    // Starbase flags
    /// Player has visited Earth Starbase
    VisitedStarbase,
    /// Commander Hayes has been met
    MetCommander,
    
    // Alien encounter flags
    /// Met the Ur-Quan
    MetUrQuan,
    /// Met the Arilou
    MetArilou,
    /// Met the Spathi
    MetSpathi,
    /// Met the Pkunk
    MetPkunk,
    /// Met the Orz
    MetOrz,
    
    // Quest progression flags
    /// Found the Shofixti survivor
    FoundShofixti,
    /// Obtained Ultron device
    ObtainedUltron,
    /// Destroyed Sa-Matra
    DestroyedSaMatra,
    
    // Special system flags
    /// Discovered Rainbow World
    FoundRainbowWorld,
    /// Found precursor artifacts
    FoundPrecursorArtifact,
    
    // Alliance flags
    /// Spathi joined alliance
    SpathiAllied,
    /// Pkunk joined alliance
    PkunkAllied,
    /// Orz joined alliance
    OrzAllied,
    
    // Combat flags
    /// Defeated Ur-Quan in combat
    DefeatedUrQuanShip,
    /// Won supermelee tournament
    WonSupermelee,
    
    // Custom flags for new content
    /// Found Kitsune system (custom system)
    FoundKitsune,
    /// Cheat mode enabled
    CheatModeActive,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags_creation() {
        let flags = GameFlags::new();
        assert!(!flags.is_set(GlobalFlag::GameStarted));
        assert_eq!(flags.count(), 0);
    }

    #[test]
    fn test_new_game() {
        let flags = GameFlags::new_game();
        assert!(flags.is_set(GlobalFlag::GameStarted));
        assert_eq!(flags.count(), 1);
    }

    #[test]
    fn test_set_flag() {
        let mut flags = GameFlags::new();
        flags.set(GlobalFlag::GameStarted);
        assert!(flags.is_set(GlobalFlag::GameStarted));
        assert_eq!(flags.count(), 1);
    }

    #[test]
    fn test_set_many() {
        let mut flags = GameFlags::new();
        flags.set_many(&[
            GlobalFlag::GameStarted,
            GlobalFlag::MetUrQuan,
            GlobalFlag::VisitedStarbase,
        ]);
        
        assert_eq!(flags.count(), 3);
        assert!(flags.is_set(GlobalFlag::GameStarted));
        assert!(flags.is_set(GlobalFlag::MetUrQuan));
        assert!(flags.is_set(GlobalFlag::VisitedStarbase));
    }

    #[test]
    fn test_clear_flag() {
        let mut flags = GameFlags::new();
        flags.set(GlobalFlag::GameStarted);
        flags.clear(GlobalFlag::GameStarted);
        assert!(!flags.is_set(GlobalFlag::GameStarted));
        assert_eq!(flags.count(), 0);
    }

    #[test]
    fn test_clear_many() {
        let mut flags = GameFlags::new();
        flags.set_many(&[
            GlobalFlag::GameStarted,
            GlobalFlag::MetUrQuan,
            GlobalFlag::VisitedStarbase,
        ]);
        
        flags.clear_many(&[GlobalFlag::GameStarted, GlobalFlag::MetUrQuan]);
        
        assert!(!flags.is_set(GlobalFlag::GameStarted));
        assert!(!flags.is_set(GlobalFlag::MetUrQuan));
        assert!(flags.is_set(GlobalFlag::VisitedStarbase));
        assert_eq!(flags.count(), 1);
    }

    #[test]
    fn test_toggle() {
        let mut flags = GameFlags::new();
        
        flags.toggle(GlobalFlag::GameStarted);
        assert!(flags.is_set(GlobalFlag::GameStarted));
        
        flags.toggle(GlobalFlag::GameStarted);
        assert!(!flags.is_set(GlobalFlag::GameStarted));
    }

    #[test]
    fn test_is_any_set() {
        let mut flags = GameFlags::new();
        flags.set(GlobalFlag::MetUrQuan);
        
        assert!(flags.is_any_set(&[GlobalFlag::MetUrQuan, GlobalFlag::MetArilou]));
        assert!(!flags.is_any_set(&[GlobalFlag::MetArilou, GlobalFlag::MetSpathi]));
    }

    #[test]
    fn test_are_all_set() {
        let mut flags = GameFlags::new();
        flags.set(GlobalFlag::MetUrQuan);
        flags.set(GlobalFlag::MetArilou);
        
        assert!(flags.are_all_set(&[GlobalFlag::MetUrQuan, GlobalFlag::MetArilou]));
        assert!(!flags.are_all_set(&[GlobalFlag::MetUrQuan, GlobalFlag::MetSpathi]));
    }

    #[test]
    fn test_multiple_flags() {
        let mut flags = GameFlags::new();
        flags.set(GlobalFlag::GameStarted);
        flags.set(GlobalFlag::MetUrQuan);
        
        assert!(flags.is_set(GlobalFlag::GameStarted));
        assert!(flags.is_set(GlobalFlag::MetUrQuan));
        assert!(!flags.is_set(GlobalFlag::MetArilou));
        assert_eq!(flags.count(), 2);
    }

    #[test]
    fn test_all_active() {
        let mut flags = GameFlags::new();
        flags.set(GlobalFlag::GameStarted);
        flags.set(GlobalFlag::MetUrQuan);
        
        let active = flags.all_active();
        assert_eq!(active.len(), 2);
        assert!(active.contains(&GlobalFlag::GameStarted));
        assert!(active.contains(&GlobalFlag::MetUrQuan));
    }

    #[test]
    fn test_clear_all() {
        let mut flags = GameFlags::new();
        flags.set(GlobalFlag::GameStarted);
        flags.set(GlobalFlag::MetUrQuan);
        flags.clear_all();
        
        assert!(!flags.is_set(GlobalFlag::GameStarted));
        assert!(!flags.is_set(GlobalFlag::MetUrQuan));
        assert_eq!(flags.count(), 0);
    }

    #[test]
    fn test_is_game_completed() {
        let mut flags = GameFlags::new();
        assert!(!flags.is_game_completed());
        
        flags.set(GlobalFlag::DestroyedSaMatra);
        assert!(flags.is_game_completed());
    }

    #[test]
    fn test_get_alien_encounters() {
        let mut flags = GameFlags::new();
        flags.set(GlobalFlag::MetUrQuan);
        flags.set(GlobalFlag::MetArilou);
        flags.set(GlobalFlag::VisitedStarbase); // Not an alien
        
        let encounters = flags.get_alien_encounters();
        assert_eq!(encounters.len(), 2);
        assert!(encounters.contains(&GlobalFlag::MetUrQuan));
        assert!(encounters.contains(&GlobalFlag::MetArilou));
    }

    #[test]
    fn test_get_alliances() {
        let mut flags = GameFlags::new();
        flags.set(GlobalFlag::SpathiAllied);
        flags.set(GlobalFlag::MetUrQuan); // Not an alliance
        
        let alliances = flags.get_alliances();
        assert_eq!(alliances.len(), 1);
        assert!(alliances.contains(&GlobalFlag::SpathiAllied));
    }

    #[test]
    fn test_can_access_shofixti_quest() {
        let mut flags = GameFlags::new();
        assert!(!flags.can_access_quest(QuestType::ShofixtiQuest));
        
        flags.set(GlobalFlag::MetCommander);
        assert!(flags.can_access_quest(QuestType::ShofixtiQuest));
    }

    #[test]
    fn test_can_access_ultron_quest() {
        let mut flags = GameFlags::new();
        assert!(!flags.can_access_quest(QuestType::UltronQuest));
        
        flags.set(GlobalFlag::VisitedStarbase);
        assert!(flags.can_access_quest(QuestType::UltronQuest));
    }

    #[test]
    fn test_can_access_final_mission() {
        let mut flags = GameFlags::new();
        assert!(!flags.can_access_quest(QuestType::FinalMission));
        
        flags.set(GlobalFlag::ObtainedUltron);
        assert!(flags.can_access_quest(QuestType::FinalMission));
        
        // After completing, quest is no longer accessible
        flags.set(GlobalFlag::DestroyedSaMatra);
        assert!(!flags.can_access_quest(QuestType::FinalMission));
    }

    #[test]
    fn test_serialization() {
        let mut flags = GameFlags::new();
        flags.set(GlobalFlag::GameStarted);
        flags.set(GlobalFlag::MetUrQuan);
        
        let json = serde_json::to_string(&flags).unwrap();
        let deserialized: GameFlags = serde_json::from_str(&json).unwrap();
        
        assert_eq!(flags, deserialized);
    }
}
