use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;
use rand::Rng;
use crate::common::item::item;
use crate::common::item::item::{Item, Prototype, StatTracker};

lazy_static! {
    static ref PROTOTYPES: HashMap<String, Arc<item::Prototype>> = init_prototype_map();
    static ref SERIES: Vec<CrateSeries> = init_crate_series();
}

pub struct CrateSeries {
    pub name: String,
    pub number: u32,
    pub entries: Vec<CrateSeriesEntry>,
    total_odds: i32,
}

impl CrateSeries {
    pub fn new(name: String, number: u32, entries: Vec<CrateSeriesEntry>) -> Self {
        let total_odds = entries.iter().fold(0, |acc, e| acc + e.odds);
        Self {
            name,
            number,
            entries,
            total_odds,
        }
    }
    
    pub fn unbox_one(&self) -> Option<item::Item> {
        // Choose a random number between 0 and the total odds.
        let random_number = rand::thread_rng().gen_range(0..self.total_odds);

        // Iterate through the entries, subtracting the odds from the random number until we find the entry that contains the random number.
        let mut current_odds = 0;
        for entry in &self.entries {
            current_odds += entry.odds;
            if random_number < current_odds {
                let key = &entry.prototype_key;
                let prototype = PROTOTYPES.get(key)?.clone();

                let mut item = Prototype::as_item(prototype);
                if let Some(stat_tracker) = &entry.stat_tracker {
                    item.stat_tracker = Some(stat_tracker.clone());
                }
                // TODO: mutate item to add particle effect
                return Some(item);
            }
        }
        
        None
    }
}

pub struct CrateSeriesEntry {
    pub prototype_key: String,
    pub particle_effect_key: Option<String>,
    pub stat_tracker: Option<StatTracker>,
    pub odds: i32,
}

impl CrateSeriesEntry {
    pub fn new(prototype_key: &str, odds: i32) -> CrateSeriesEntry {
        Self {
            prototype_key: prototype_key.to_owned(),
            particle_effect_key: None,
            stat_tracker: None,
            odds,
        }
    }

    pub fn new_with(prototype_key: &str, odds: i32, particle_effect_key: Option<String>, stat_tracker: Option<StatTracker>) -> CrateSeriesEntry {
        Self {
            prototype_key: prototype_key.to_owned(),
            particle_effect_key,
            stat_tracker,
            odds,
        }
    }
}

fn init_crate_series() -> Vec<CrateSeries> {
    let mut series = Vec::new();

    series.push(CrateSeries::new(
        "".to_string(),
        0,
        vec![
            CrateSeriesEntry::new("shotgun", 200),
            CrateSeriesEntry::new("medigun", 100),
            CrateSeriesEntry::new("top_hat", 100),
        ],
    ));

    series.push(CrateSeries::new(
        "".to_string(),
        1,
        vec![
            CrateSeriesEntry::new_with("shotgun", 200, None, Some(StatTracker::default_points())),
            CrateSeriesEntry::new_with("medigun", 100, None, Some(StatTracker::default_healing())),
        ],
    ));

    series
}

fn init_prototype_map() -> HashMap<String, Arc<Prototype>> {
    let mut prototypes: HashMap<String, Arc<Prototype>> = HashMap::new();

    prototypes.insert("shotgun".to_owned(), Arc::new(Prototype {
        name_key: "shotgun".to_owned(),
        stock: true,
        trade_restriction: false,
    }));
    prototypes.insert("medigun".to_owned(), Arc::new(Prototype {
        name_key: "medigun".to_owned(),
        stock: true,
        trade_restriction: false,
    }));
    prototypes.insert("top_hat".to_owned(), Arc::new(Prototype {
        name_key: "top_hat".to_owned(),
        stock: true,
        trade_restriction: false,
    }));

    prototypes
}

pub fn unlock(series: u32) -> Option<item::Item> {
    if SERIES.len() <= series as usize {
        return None;
    }
    let series = &SERIES[series as usize];
    series.unbox_one()
}
