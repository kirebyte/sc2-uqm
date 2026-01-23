//! Utility functions and helpers

/// Calculate travel time in days based on distance
#[must_use]
pub fn calculate_travel_time(distance: f32) -> u32 {
    // Simplified: 1 unit of distance = 0.1 days
    (distance * 0.1) as u32
}

/// Clamp a value between min and max
#[must_use]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_travel_time() {
        assert_eq!(calculate_travel_time(100.0), 10);
        assert_eq!(calculate_travel_time(50.0), 5);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
    }
}
