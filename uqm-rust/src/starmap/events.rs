// EventManager: Quest triggers and story events
// Faithful port from sc2/src/uqm/gameev.c and clock.c
// Manages time-based events and story progression

// LEGACY_HACK: Event system based on game clock (month, day, year)
// Events can be ABSOLUTE (fixed date) or RELATIVE (offset from current time)

/// Event types from gameev.c
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventType {
    HyperspaceEncounter = 0,
    ArilouEntrance = 1,
    ArilouExit = 2,
    KohrAhVictorious = 3,
    KohrAhGenocide = 4,
    AdvancePkunkMission = 5,
    AdvanceThaddMission = 6,
    ZoqfotDistress = 7,
    ZoqfotDeath = 8,
    ShofixtiReturn = 9,
    YehatRebel = 10,
    SpathiShield = 11,
    AdvanceIlwrathMission = 12,
    AdvanceUtwigSupoxMission = 13,
    AdvanceMyconMission = 14,
    ArilouUmgahCheck = 15,
    SlylnadroRampUp = 16,
    SlylnadroRampDown = 17,
}

/// Event scheduling type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventSchedule {
    Absolute, // Triggers at specific date
    Relative, // Triggers after time offset
}

/// Game event structure
#[derive(Debug, Clone, PartialEq)]
pub struct GameEvent {
    pub schedule: EventSchedule,
    pub event_type: EventType,
    pub month: u8,
    pub day: u8,
    pub year: u16,
}

impl GameEvent {
    pub fn new(schedule: EventSchedule, event_type: EventType, month: u8, day: u8, year: u16) -> Self {
        GameEvent {
            schedule,
            event_type,
            month,
            day,
            year,
        }
    }
}

/// Game clock for time progression
/// LEGACY_HACK: Original uses month_index, day_index, year_index
#[derive(Debug, Clone, PartialEq)]
pub struct GameClock {
    pub month: u8,  // 0-11 (12 months per year)
    pub day: u8,    // 0-29 (30 days per month)
    pub year: u16,
}

impl GameClock {
    pub fn new(month: u8, day: u8, year: u16) -> Self {
        GameClock { month, day, year }
    }

    /// Advance clock by specified amount
    pub fn advance(&mut self, months: u8, days: u8, years: u16) {
        self.day += days;
        if self.day >= 30 {
            self.month += self.day / 30;
            self.day %= 30;
        }

        self.month += months;
        if self.month >= 12 {
            self.year += (self.month / 12) as u16;
            self.month %= 12;
        }

        self.year += years;
    }

    /// Check if this clock is at or past target time
    pub fn is_at_or_past(&self, target: &GameClock) -> bool {
        if self.year > target.year {
            return true;
        }
        if self.year < target.year {
            return false;
        }
        if self.month > target.month {
            return true;
        }
        if self.month < target.month {
            return false;
        }
        self.day >= target.day
    }
}

/// Event manager for story progression
pub struct EventManager {
    events: Vec<GameEvent>,
    clock: GameClock,
}

impl EventManager {
    pub fn new(start_year: u16) -> Self {
        EventManager {
            events: Vec::new(),
            clock: GameClock::new(1, 15, start_year), // Start: Feb 15
        }
    }

    /// Get current game clock
    pub fn get_clock(&self) -> &GameClock {
        &self.clock
    }

    /// Advance game clock
    pub fn advance_time(&mut self, months: u8, days: u8, years: u16) {
        self.clock.advance(months, days, years);
    }

    /// Add an absolute event (triggers at specific date)
    pub fn add_absolute_event(&mut self, event_type: EventType, month: u8, day: u8, year: u16) {
        self.events.push(GameEvent::new(
            EventSchedule::Absolute,
            event_type,
            month,
            day,
            year,
        ));
    }

    /// Add a relative event (triggers after time offset from now)
    pub fn add_relative_event(&mut self, event_type: EventType, months: u8, days: u8, years: u16) {
        let mut trigger_time = self.clock.clone();
        trigger_time.advance(months, days, years);
        self.events.push(GameEvent::new(
            EventSchedule::Relative,
            event_type,
            trigger_time.month,
            trigger_time.day,
            trigger_time.year,
        ));
    }

    /// Check for triggered events (returns and removes them)
    pub fn check_events(&mut self) -> Vec<EventType> {
        let mut triggered = Vec::new();
        let current_time = self.clock.clone();

        // Find triggered events
        let mut i = 0;
        while i < self.events.len() {
            let event = &self.events[i];
            let event_time = GameClock::new(event.month, event.day, event.year);

            if current_time.is_at_or_past(&event_time) {
                triggered.push(event.event_type);
                self.events.remove(i);
            } else {
                i += 1;
            }
        }

        triggered
    }

    /// Add initial game events (from AddInitialGameEvents in gameev.c)
    pub fn add_initial_events(&mut self, start_year: u16) {
        // HYPERSPACE_ENCOUNTER_EVENT: Daily check
        self.add_relative_event(EventType::HyperspaceEncounter, 0, 1, 0);

        // ARILOU_ENTRANCE_EVENT: March 17 of start year
        // LEGACY_HACK: Month 3 = April (0-indexed), but comment says March 17
        // This is Bug in original code or intentional offset?
        self.add_absolute_event(EventType::ArilouEntrance, 3, 17, start_year);

        // KOHR_AH_VICTORIOUS_EVENT: Years to Kohr-Ah victory
        // LEGACY_HACK: Hardcoded to 5 years in original
        const YEARS_TO_KOHRAH_VICTORY: u16 = 5;
        self.add_relative_event(EventType::KohrAhVictorious, 0, 0, YEARS_TO_KOHRAH_VICTORY);

        // SLYLANDRO_RAMP_UP: Immediate
        self.add_relative_event(EventType::SlylnadroRampUp, 0, 0, 0);
    }

    /// Get pending events count (for testing)
    pub fn pending_events_count(&self) -> usize {
        self.events.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_clock_creation() {
        let clock = GameClock::new(1, 15, 2155);
        assert_eq!(clock.month, 1);
        assert_eq!(clock.day, 15);
        assert_eq!(clock.year, 2155);
    }

    #[test]
    fn test_clock_advance_days() {
        let mut clock = GameClock::new(0, 0, 2155);
        clock.advance(0, 5, 0);
        assert_eq!(clock.day, 5);
    }

    #[test]
    fn test_clock_advance_months() {
        let mut clock = GameClock::new(0, 0, 2155);
        clock.advance(3, 0, 0);
        assert_eq!(clock.month, 3);
    }

    #[test]
    fn test_clock_advance_years() {
        let mut clock = GameClock::new(0, 0, 2155);
        clock.advance(0, 0, 2);
        assert_eq!(clock.year, 2157);
    }

    #[test]
    fn test_clock_day_overflow() {
        let mut clock = GameClock::new(0, 25, 2155);
        clock.advance(0, 10, 0); // 25 + 10 = 35 days
        assert_eq!(clock.month, 1); // Rolled over 1 month
        assert_eq!(clock.day, 5); // 35 % 30 = 5
    }

    #[test]
    fn test_clock_month_overflow() {
        let mut clock = GameClock::new(10, 0, 2155);
        clock.advance(5, 0, 0); // 10 + 5 = 15 months
        assert_eq!(clock.year, 2156); // Rolled over 1 year
        assert_eq!(clock.month, 3); // 15 % 12 = 3
    }

    #[test]
    fn test_clock_comparison_past() {
        let clock1 = GameClock::new(5, 10, 2155);
        let clock2 = GameClock::new(3, 10, 2155);
        assert!(clock1.is_at_or_past(&clock2));
    }

    #[test]
    fn test_clock_comparison_future() {
        let clock1 = GameClock::new(3, 10, 2155);
        let clock2 = GameClock::new(5, 10, 2155);
        assert!(!clock1.is_at_or_past(&clock2));
    }

    #[test]
    fn test_clock_comparison_equal() {
        let clock1 = GameClock::new(3, 10, 2155);
        let clock2 = GameClock::new(3, 10, 2155);
        assert!(clock1.is_at_or_past(&clock2));
    }

    #[test]
    fn test_event_manager_creation() {
        let manager = EventManager::new(2155);
        assert_eq!(manager.get_clock().year, 2155);
        assert_eq!(manager.pending_events_count(), 0);
    }

    #[test]
    fn test_add_absolute_event() {
        let mut manager = EventManager::new(2155);
        manager.add_absolute_event(EventType::ArilouEntrance, 3, 17, 2155);
        assert_eq!(manager.pending_events_count(), 1);
    }

    #[test]
    fn test_add_relative_event() {
        let mut manager = EventManager::new(2155);
        manager.add_relative_event(EventType::HyperspaceEncounter, 0, 5, 0);
        assert_eq!(manager.pending_events_count(), 1);
    }

    #[test]
    fn test_event_triggering() {
        let mut manager = EventManager::new(2155);
        // Add event for 5 days in future
        manager.add_relative_event(EventType::HyperspaceEncounter, 0, 5, 0);

        // Advance 6 days
        manager.advance_time(0, 6, 0);

        // Should trigger
        let triggered = manager.check_events();
        assert_eq!(triggered.len(), 1);
        assert_eq!(triggered[0], EventType::HyperspaceEncounter);
        assert_eq!(manager.pending_events_count(), 0);
    }

    #[test]
    fn test_event_not_triggering_yet() {
        let mut manager = EventManager::new(2155);
        // Add event for 5 days in future
        manager.add_relative_event(EventType::HyperspaceEncounter, 0, 5, 0);

        // Advance only 3 days
        manager.advance_time(0, 3, 0);

        // Should not trigger
        let triggered = manager.check_events();
        assert_eq!(triggered.len(), 0);
        assert_eq!(manager.pending_events_count(), 1);
    }

    #[test]
    fn test_initial_events() {
        let mut manager = EventManager::new(2155);
        manager.add_initial_events(2155);

        // Should have 4 initial events
        assert_eq!(manager.pending_events_count(), 4);
    }

    #[test]
    fn test_multiple_event_triggering() {
        let mut manager = EventManager::new(2155);
        manager.add_relative_event(EventType::HyperspaceEncounter, 0, 1, 0);
        manager.add_relative_event(EventType::ArilouExit, 0, 1, 0);

        manager.advance_time(0, 2, 0);

        let triggered = manager.check_events();
        assert_eq!(triggered.len(), 2);
        assert!(triggered.contains(&EventType::HyperspaceEncounter));
        assert!(triggered.contains(&EventType::ArilouExit));
    }
}
