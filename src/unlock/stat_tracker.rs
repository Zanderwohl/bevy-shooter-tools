use crate::common::item::item::StatTracker;

pub struct StatTrackerEntry {
    pub stat_tracker: Option<StatTracker>,
    pub odds: i32,
}

impl StatTrackerEntry {
    pub fn none(odds: i32) -> Self {
        Self {
            stat_tracker: None,
            odds,
        }
    }

    pub fn kills(odds: i32) -> Self {
        Self {
            stat_tracker: Some(StatTracker::default_kills()),
            odds,
        }
    }

    pub fn max_weapon(odds: i32) -> Self {
        Self {
            stat_tracker: Some(StatTracker {
                kills: Some(0),
                assists: Some(0),
                damage: Some(0),
                points: Some(0),
                healing: None,
                invulns: None,
            }),
            odds,
        }
    }

    pub fn healing(odds: i32) -> Self {
        Self {
            stat_tracker: Some(StatTracker::default_healing()),
            odds,
        }
    }

    pub fn max_medigun(odds: i32) -> Self {
        Self {
            stat_tracker: Some(StatTracker {
                kills: None,
                assists: None,
                damage: None,
                points: None,
                healing: Some(0),
                invulns: Some(0),
            }),
            odds,
        }
    }

    pub fn points(odds: i32) -> Self {
        Self {
            stat_tracker: Some(StatTracker::default_points()),
            odds,
        }
    }
}