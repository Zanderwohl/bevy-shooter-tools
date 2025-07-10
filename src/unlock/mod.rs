use std::collections::HashMap;
use std::sync::Arc;
use bevy::prelude::warn;
use bevy::ui::AlignItems::Default;
use lazy_static::lazy_static;
use rand::Rng;
use crate::common::item::item;
use crate::common::item::item::{Item, ParticleEffect, Prototype, StatTracker};
use crate::get;

lazy_static! {
    static ref PROTOTYPES: HashMap<String, Arc<item::Prototype>> = init_prototype_map();
    static ref SERIES: Vec<CrateSeries> = init_crate_series();
    static ref PARTICLES: HashMap<String, Arc<item::ParticleEffect>> = init_particle_effects();
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
                return entry.unbox_one()
            }
        }
        
        None
    }
}

pub struct CrateSeriesEntry {
    pub prototype_key: String,
    pub particle_effects: Vec<ParticleEffectEntry>,
    pub total_particle_effect_odds: i32,
    pub stat_trackers: Vec<StatTrackerEntry>,
    pub total_stat_tracker_odds: i32,
    pub odds: i32,
    pub allow_plain: bool,
}

impl CrateSeriesEntry {
    pub fn new(prototype_key: &str, odds: i32) -> CrateSeriesEntry {
        Self {
            prototype_key: prototype_key.to_owned(),
            particle_effects: Vec::new(),
            stat_trackers: Vec::new(),
            odds,
            allow_plain: false,
            total_particle_effect_odds: 0,
            total_stat_tracker_odds: 0,
        }
    }

    pub fn new_with(prototype_key: &str, odds: i32, particle_effects: Vec<ParticleEffectEntry>, stat_trackers: Vec<StatTrackerEntry>) -> Self {
        let mut entry = CrateSeriesEntry::new(prototype_key, odds);
        for particle_effect in particle_effects {
            entry.particle_effects.push(particle_effect);
        }
        for stat_tracker in stat_trackers {
            entry.stat_trackers.push(stat_tracker);
        }
        entry
    }

    pub fn add_particle_effect(&mut self, particle_effect: ParticleEffectEntry) {
        self.total_particle_effect_odds += particle_effect.odds;
        self.particle_effects.push(particle_effect);
    }
    
    pub fn add_stat_tracker(&mut self, stat_tracker: StatTrackerEntry) {
        self.total_stat_tracker_odds += stat_tracker.odds;
        self.stat_trackers.push(stat_tracker);
    }
    
    pub fn unbox_one(&self) -> Option<item::Item> {
        let key = &self.prototype_key;
        let prototype = PROTOTYPES.get(key)?.clone();

        let mut item = Item::new(prototype);

        let mut effect: Option<Arc<ParticleEffect>> = None;
        let mut stat_tracker: Option<StatTracker> = None;

        let mut loops = 0;
        
        'create: loop  {
            loops += 1;
            if self.total_particle_effect_odds > 0 {
                let mut current_odds = 0;
                let random_number = rand::thread_rng().gen_range(0..self.total_particle_effect_odds);
                for entry in &self.particle_effects {
                    current_odds += entry.odds;
                    if random_number < current_odds {
                        if let Some(key) = &entry.particle_effect_key && let Some(selected_effect) = PARTICLES.get(key) {
                            effect = Some(selected_effect.clone());
                        }
                    }
                }
            }

            if self.total_stat_tracker_odds > 0 {
                let random_number = rand::thread_rng().gen_range(0..self.total_stat_tracker_odds);
                let mut current_odds = 0;
                for entry in &self.stat_trackers {
                    current_odds += entry.odds;
                    if random_number < current_odds {
                        if let Some(new_stat_tracker) = &entry.stat_tracker {
                            stat_tracker = Some(new_stat_tracker.clone());
                        }
                    }
                }

            }

            if !self.allow_plain && self.total_stat_tracker_odds == 0 && self.total_particle_effect_odds > 0 {
                warn!("{}", get!("create_drop.error.no_odds", "item", self.prototype_key));
                break 'create
            }
            if self.allow_plain || effect.is_some() || stat_tracker.is_some() { break 'create; }
            if loops >= 10 {
                warn!("{}", get!("crate_drop.error.quality_loop", "item", self.prototype_key));
                break 'create;
            }
        }
        
        Some(item)
    }
}

pub struct ParticleEffectEntry {
    pub particle_effect_key: Option<String>,
    pub odds: i32,
}

impl ParticleEffectEntry {
    pub fn none(odds: i32) -> ParticleEffectEntry {
        Self {
            particle_effect_key: None,
            odds,
        }
    }

    pub fn some(odds: i32, particle_effect_key: String) -> ParticleEffectEntry {
        Self {
            particle_effect_key: Some(particle_effect_key),
            odds,
        }
    }
}

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

fn init_crate_series() -> Vec<CrateSeries> {
    let mut series = Vec::new();

    series.push(CrateSeries::new(
        "Stock Items".to_string(),
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
            CrateSeriesEntry::new_with("shotgun", 100,
                                       vec![],
                                       vec![
                                           StatTrackerEntry::none(100),
                                           StatTrackerEntry::kills(100),
                                           StatTrackerEntry::max_weapon(100),
                                       ],
            ),
            CrateSeriesEntry::new_with("medigun", 100,
                                       vec![],
                                       vec![
                                           StatTrackerEntry::none(100),
                                           StatTrackerEntry::healing(100),
                                           StatTrackerEntry::max_medigun(100),
                                       ],
            ),
            CrateSeriesEntry::new_with("top_hat", 100,
            vec![
                // electric, fire, fire-blue
                ParticleEffectEntry::none(100),
                ParticleEffectEntry::some(100, "electric".to_owned()),
                ParticleEffectEntry::some(100, "fire".to_owned()),
                ParticleEffectEntry::some(100, "fire-blue".to_owned()),
            ],
            vec![
                StatTrackerEntry::none(100),
                StatTrackerEntry::points(100),
            ],
            ),
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

fn init_particle_effects() -> HashMap<String, Arc<item::ParticleEffect>> {
    let mut effects: HashMap<String, Arc<item::ParticleEffect>> = HashMap::new();

    effects.insert("electric".to_owned(), Arc::new(ParticleEffect {
        name_key: "electric".to_owned(),
    }));
    effects.insert("fire".to_owned(), Arc::new(ParticleEffect {
        name_key: "fire".to_owned(),
    }));
    effects.insert("fire-blue".to_owned(), Arc::new(ParticleEffect {
        name_key: "fire-blue".to_owned(),
    }));

    effects
}

pub fn unlock(series: u32) -> Option<item::Item> {
    if SERIES.len() <= series as usize {
        return None;
    }
    let series = &SERIES[series as usize];
    series.unbox_one()
}
