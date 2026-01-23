//! Game time management
//!
//! This module handles in-game time tracking, including days, months, and years.
//! The game uses a simplified calendar system:
//! - 30 days per month
//! - 12 months per year
//! - 360 days per year
//!
//! The canonical start date is February 17, 2155 (year 2155, month 2, day 17).

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

/// In-game time representation
///
/// The game tracks time as the player explores the galaxy.
/// Time advances when traveling between star systems.
///
/// # Examples
///
/// ```
/// # use uqm::core::GameTime;
/// let mut time = GameTime::new();
/// assert_eq!(time.year(), 2155);
/// assert_eq!(time.month(), 2);
/// assert_eq!(time.day(), 17);
///
/// time.advance_days(30);
/// assert_eq!(time.month(), 3); // Advanced one month
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameTime {
    /// Current day (0-29)
    day: u8,
    /// Current month (0-11)
    month: u8,
    /// Current year
    year: u16,
}

impl GameTime {
    /// Days per month in the game calendar
    pub const DAYS_PER_MONTH: u8 = 30;
    
    /// Months per year
    pub const MONTHS_PER_YEAR: u8 = 12;

    /// Days per year (simplified calendar)
    pub const DAYS_PER_YEAR: u16 = 360;

    /// Month names (0-indexed)
    pub const MONTH_NAMES: [&'static str; 12] = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];

    /// Create a new game time starting at year 2155, month 2, day 17
    /// (the canonical start date from the original game)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let time = GameTime::new();
    /// assert_eq!(time.year(), 2155);
    /// assert_eq!(time.month(), 2); // February (0-indexed)
    /// assert_eq!(time.day(), 17);
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            day: 17,
            month: 2,
            year: 2155,
        }
    }

    /// Create a custom game time
    ///
    /// # Panics
    ///
    /// Panics in debug mode if month >= 12 or day >= 30.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let time = GameTime::custom(2160, 5, 10);
    /// assert_eq!(time.year(), 2160);
    /// assert_eq!(time.month(), 5);
    /// assert_eq!(time.day(), 10);
    /// ```
    #[must_use]
    pub const fn custom(year: u16, month: u8, day: u8) -> Self {
        debug_assert!(month < Self::MONTHS_PER_YEAR, "Month must be 0-11");
        debug_assert!(day < Self::DAYS_PER_MONTH, "Day must be 0-29");
        Self { day, month, year }
    }

    /// Create from total days since start
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let time = GameTime::from_days(360); // One year after start
    /// assert_eq!(time.year(), 2156);
    /// ```
    #[must_use]
    pub fn from_days(days_since_start: u32) -> Self {
        let mut time = Self::new();
        time.advance_days(days_since_start);
        time
    }

    /// Get current day (0-29)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let time = GameTime::new();
    /// assert_eq!(time.day(), 17);
    /// ```
    #[must_use]
    pub const fn day(&self) -> u8 {
        self.day
    }

    /// Get current month (0-11, where 0 = January)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let time = GameTime::new();
    /// assert_eq!(time.month(), 2); // February
    /// ```
    #[must_use]
    pub const fn month(&self) -> u8 {
        self.month
    }

    /// Get current year
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let time = GameTime::new();
    /// assert_eq!(time.year(), 2155);
    /// ```
    #[must_use]
    pub const fn year(&self) -> u16 {
        self.year
    }

    /// Get the month name
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let time = GameTime::new();
    /// assert_eq!(time.month_name(), "March"); // Month 2 = March (0-indexed)
    /// ```
    #[must_use]
    pub fn month_name(&self) -> &'static str {
        Self::MONTH_NAMES[self.month as usize]
    }

    /// Advance time by a number of days
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let mut time = GameTime::new();
    /// time.advance_days(30);
    /// assert_eq!(time.month(), 3); // Advanced one month
    ///
    /// time.advance_days(360);
    /// assert_eq!(time.year(), 2156); // Advanced one year
    /// ```
    pub fn advance_days(&mut self, days: u32) {
        let mut remaining_days = days + u32::from(self.day);

        // Handle month overflow
        while remaining_days >= u32::from(Self::DAYS_PER_MONTH) {
            remaining_days -= u32::from(Self::DAYS_PER_MONTH);
            self.month += 1;

            // Handle year overflow
            if self.month >= Self::MONTHS_PER_YEAR {
                self.month = 0;
                self.year += 1;
            }
        }

        self.day = remaining_days as u8;
    }

    /// Advance time by a number of months
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let mut time = GameTime::new();
    /// time.advance_months(12);
    /// assert_eq!(time.year(), 2156);
    /// assert_eq!(time.month(), 2);
    /// ```
    pub fn advance_months(&mut self, months: u32) {
        let total_months = u32::from(self.month) + months;
        let years_to_add = total_months / u32::from(Self::MONTHS_PER_YEAR);
        let new_month = (total_months % u32::from(Self::MONTHS_PER_YEAR)) as u8;

        self.year += years_to_add as u16;
        self.month = new_month;
    }

    /// Advance time by a number of years
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let mut time = GameTime::new();
    /// time.advance_years(5);
    /// assert_eq!(time.year(), 2160);
    /// ```
    pub fn advance_years(&mut self, years: u16) {
        self.year += years;
    }

    /// Calculate total days elapsed since start
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let start = GameTime::new();
    /// assert_eq!(start.total_days_since_start(), 0);
    ///
    /// let mut later = start;
    /// later.advance_days(100);
    /// assert!(later.total_days_since_start() >= 100);
    /// ```
    #[must_use]
    pub fn total_days_since_start(&self) -> u32 {
        let start = Self::new();
        self.days_since(&start)
    }

    /// Calculate days between this time and another time
    ///
    /// Returns 0 if other time is after this time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let start = GameTime::new();
    /// let mut end = start;
    /// end.advance_days(50);
    /// assert_eq!(end.days_since(&start), 50);
    /// ```
    #[must_use]
    pub fn days_since(&self, other: &Self) -> u32 {
        if self < other {
            return 0;
        }

        // Calculate total days for each time from year 0
        let self_total = self.to_total_days();
        let other_total = other.to_total_days();

        self_total.saturating_sub(other_total)
    }

    /// Convert to total days since year 0 (for internal calculations)
    fn to_total_days(&self) -> u32 {
        let days_from_years = u32::from(self.year) * u32::from(Self::DAYS_PER_YEAR);
        let days_from_months = u32::from(self.month) * u32::from(Self::DAYS_PER_MONTH);
        let days = u32::from(self.day);
        days_from_years + days_from_months + days
    }

    /// Check if a certain amount of time has passed since another time
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let start = GameTime::new();
    /// let mut current = start;
    /// current.advance_days(50);
    /// assert!(current.has_elapsed(&start, 40));
    /// assert!(!current.has_elapsed(&start, 60));
    /// ```
    #[must_use]
    pub fn has_elapsed(&self, since: &Self, days: u32) -> bool {
        self.days_since(since) >= days
    }

    /// Format as a date string
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let time = GameTime::new();
    /// assert_eq!(time.format_date(), "March 17, 2155");
    /// ```
    #[must_use]
    pub fn format_date(&self) -> String {
        format!("{} {}, {}", self.month_name(), self.day, self.year)
    }

    /// Format as compact date string (YYYY-MM-DD)
    ///
    /// # Examples
    ///
    /// ```
    /// # use uqm::core::GameTime;
    /// let time = GameTime::new();
    /// assert_eq!(time.format_compact(), "2155-03-17");
    /// ```
    #[must_use]
    pub fn format_compact(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month + 1, self.day)
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialOrd for GameTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GameTime {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.year.cmp(&other.year) {
            Ordering::Equal => match self.month.cmp(&other.month) {
                Ordering::Equal => self.day.cmp(&other.day),
                other => other,
            },
            other => other,
        }
    }
}

impl fmt::Display for GameTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_date())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_time_creation() {
        let time = GameTime::new();
        assert_eq!(time.day(), 17);
        assert_eq!(time.month(), 2);
        assert_eq!(time.year(), 2155);
    }

    #[test]
    fn test_game_time_custom() {
        let time = GameTime::custom(2160, 5, 10);
        assert_eq!(time.day(), 10);
        assert_eq!(time.month(), 5);
        assert_eq!(time.year(), 2160);
    }

    #[test]
    fn test_from_days() {
        let time = GameTime::from_days(360);
        assert_eq!(time.year(), 2156);
    }

    #[test]
    fn test_month_name() {
        let time = GameTime::new();
        assert_eq!(time.month_name(), "March");

        let time2 = GameTime::custom(2155, 0, 1);
        assert_eq!(time2.month_name(), "January");
    }

    #[test]
    fn test_advance_days_simple() {
        let mut time = GameTime::new();
        time.advance_days(5);
        assert_eq!(time.day(), 22);
        assert_eq!(time.month(), 2);
        assert_eq!(time.year(), 2155);
    }

    #[test]
    fn test_advance_days_month_overflow() {
        let mut time = GameTime::custom(2155, 2, 28);
        time.advance_days(3);
        assert_eq!(time.day(), 1);
        assert_eq!(time.month(), 3);
        assert_eq!(time.year(), 2155);
    }

    #[test]
    fn test_advance_days_year_overflow() {
        let mut time = GameTime::custom(2155, 11, 28);
        time.advance_days(5);
        assert_eq!(time.day(), 3);
        assert_eq!(time.month(), 0);
        assert_eq!(time.year(), 2156);
    }

    #[test]
    fn test_advance_days_multiple_years() {
        let mut time = GameTime::new();
        time.advance_days(720); // 2 years
        assert_eq!(time.year(), 2157);
    }

    #[test]
    fn test_advance_months() {
        let mut time = GameTime::new();
        time.advance_months(5);
        assert_eq!(time.month(), 7);
        assert_eq!(time.year(), 2155);

        time.advance_months(12);
        assert_eq!(time.month(), 7);
        assert_eq!(time.year(), 2156);
    }

    #[test]
    fn test_advance_years() {
        let mut time = GameTime::new();
        time.advance_years(10);
        assert_eq!(time.year(), 2165);
        assert_eq!(time.month(), 2);
        assert_eq!(time.day(), 17);
    }

    #[test]
    fn test_total_days_since_start() {
        let start = GameTime::new();
        assert_eq!(start.total_days_since_start(), 0);

        let mut later = start;
        later.advance_days(100);
        assert_eq!(later.total_days_since_start(), 100);

        later.advance_days(260); // Total 360 days = 1 year
        assert_eq!(later.total_days_since_start(), 360);
    }

    #[test]
    fn test_days_since() {
        let start = GameTime::new();
        let mut end = start;
        end.advance_days(50);
        assert_eq!(end.days_since(&start), 50);

        // Reverse should return 0
        assert_eq!(start.days_since(&end), 0);
    }

    #[test]
    fn test_has_elapsed() {
        let start = GameTime::new();
        let mut current = start;
        current.advance_days(50);
        
        assert!(current.has_elapsed(&start, 40));
        assert!(current.has_elapsed(&start, 50));
        assert!(!current.has_elapsed(&start, 60));
    }

    #[test]
    fn test_comparison() {
        let t1 = GameTime::new();
        let mut t2 = t1;
        t2.advance_days(10);
        let mut t3 = t1;
        t3.advance_years(1);

        assert!(t1 < t2);
        assert!(t2 < t3);
        assert!(t1 < t3);
        assert_eq!(t1, t1);
    }

    #[test]
    fn test_ordering() {
        let mut times = vec![
            GameTime::custom(2155, 5, 10),
            GameTime::custom(2155, 2, 20),
            GameTime::custom(2156, 1, 5),
            GameTime::custom(2155, 2, 17),
        ];

        times.sort();

        assert_eq!(times[0], GameTime::custom(2155, 2, 17));
        assert_eq!(times[1], GameTime::custom(2155, 2, 20));
        assert_eq!(times[2], GameTime::custom(2155, 5, 10));
        assert_eq!(times[3], GameTime::custom(2156, 1, 5));
    }

    #[test]
    fn test_format_date() {
        let time = GameTime::new();
        assert_eq!(time.format_date(), "March 17, 2155");

        let time2 = GameTime::custom(2160, 11, 25);
        assert_eq!(time2.format_date(), "December 25, 2160");
    }

    #[test]
    fn test_format_compact() {
        let time = GameTime::new();
        assert_eq!(time.format_compact(), "2155-03-17");

        let time2 = GameTime::custom(2160, 0, 1);
        assert_eq!(time2.format_compact(), "2160-01-01");
    }

    #[test]
    fn test_display_trait() {
        let time = GameTime::new();
        assert_eq!(format!("{}", time), "March 17, 2155");
    }

    #[test]
    fn test_default_trait() {
        let time = GameTime::default();
        assert_eq!(time, GameTime::new());
    }

    #[test]
    fn test_serialization() {
        let time = GameTime::new();
        let json = serde_json::to_string(&time).unwrap();
        let deserialized: GameTime = serde_json::from_str(&json).unwrap();
        assert_eq!(time, deserialized);
    }
}
