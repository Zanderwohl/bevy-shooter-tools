use std::sync::Arc;
use crate::get;

#[derive(Clone)]
pub struct Item {
    pub prototype: Arc<Prototype>,
    pub name: Option<String>,
    pub display_name_cache: Option<String>,
    pub description: Option<String>,
    pub stat_tracker: Option<StatTracker>,
    pub particle_effect: Option<Arc<ParticleEffect>>,
    pub trade_restriction: bool,
    pub crafting_restriction: bool,
    pub destroyed: bool,
}

impl Item {
    pub fn new(prototype: Arc<Prototype>) -> Self {
        Self {
            prototype,
            name: None,
            display_name_cache: None,
            description: None,
            stat_tracker: None,
            particle_effect: None,
            trade_restriction: false,
            crafting_restriction: false,
            destroyed: false,
        }
    }

    pub fn new_with(prototype: Arc<Prototype>, stat_tracker: Option<StatTracker>, particle_effect: Option<Arc<ParticleEffect>>) -> Self {
        Self {
            prototype,
            name: None,
            display_name_cache: None,
            description: None,
            stat_tracker,
            particle_effect,
            trade_restriction: false,
            crafting_restriction: false,
            destroyed: false,
        }
    }

    pub fn display_name(&self) -> String {
        if let Some(name) = &self.name {
           return name.clone();
        }

        self.display_name_default()
    }

    pub fn display_name_default(&self) -> String {
        let mut name = get!(format!("item.name.{}", self.prototype.name_key));
        if let Some(_) = &self.particle_effect {
            name = format!("{}", get!("item.particle_effect", "item", name));
        }
        if let Some(_) = &self.stat_tracker {
            name = format!("{}", get!("item.stat_tracker", "item", name));
        }

        name
    }

    pub fn display_name_cached(&mut self) -> &str {
        if self.display_name_cache.is_none() {
            let name= self.display_name_default();
            self.display_name_cache = Some(name);
        }
        if let Some(name) = &self.name {
            return name.as_ref();
        }
        self.display_name_cache.as_ref().unwrap()
    }
    
    pub fn tradeable(&self) -> bool {
         !self.trade_restriction && self.prototype.tradeable()
    }

    pub fn can_change_name(&self) -> bool {
        !self.prototype.stock
    }
    
    pub fn change_name(&mut self, name: String) {
        if !self.can_change_name() { return; }
        self.display_name_cache = None;
        self.name = Some(name);
    }
    
    pub fn can_change_description(&self) -> bool {
        !self.prototype.stock
    }
    
    pub fn change_description(&mut self, description: String) {
        if !self.can_change_description() { return; }
        self.description = Some(description);
    }
    
    pub fn can_destroy(&self) -> bool {
        !self.prototype.stock
    }
    
    pub fn destroy(&mut self) {
        self.destroyed = true;
    }
    
    pub fn can_craft(&self) -> bool {
        self.prototype.craftable() && !self.crafting_restriction
    }
}

#[derive(Clone)]
pub struct ItemNetworkable {
    pub prototype_key: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub stat_tracker: Option<StatTracker>,
    pub particle_effect_key: Option<String>,
    pub trade_restriction: bool,
    pub destroyed: bool,
}

#[derive(Clone)]
pub struct Prototype {
    pub name_key: String,
    pub stock: bool,
    pub trade_restriction: bool,
}

impl Prototype {

    pub fn tradeable(&self) -> bool {
        !self.trade_restriction && !self.stock
    }
    
    pub fn craftable(&self) -> bool { !self.stock }

    pub fn as_item(prototype: Arc<Prototype>) -> Item {
        Item {
            prototype: prototype.clone(),
            name: None,
            display_name_cache: None,
            description: None,
            stat_tracker: None,
            particle_effect: None,
            trade_restriction: false,
            crafting_restriction: false,
            destroyed: false,
        }
    }
}

#[derive(Clone)]
pub struct StatTracker {
    pub kills: Option<u32>,
    pub assists: Option<u32>,
    pub damage: Option<u32>,
    pub points: Option<u32>,
    pub healing: Option<u32>,
}

impl StatTracker {
    pub fn default_kills() -> StatTracker {
        StatTracker::default()
    }

    pub fn default_healing() -> StatTracker {
        StatTracker {
            kills: None,
            healing: Some(0),
            ..Default::default()
        }
    }

    pub fn default_points() -> StatTracker {
        StatTracker {
            kills: None,
            points: Some(0),
            ..Default::default()
        }
    }

    pub fn tracks_list(&self) -> String {
        let mut tracks: Vec<&str> = Vec::new();

        if self.kills.is_some() {
            tracks.push("kills");
        }
        if self.assists.is_some() {
            tracks.push("assists");
        }
        if self.damage.is_some() {
            tracks.push("damage");
        }
        if self.points.is_some() {
            tracks.push("points");
        }
        if self.healing.is_some() {
            tracks.push("healing");
        }

        tracks.iter().map(|key| get!(format!("stat_tracker.{}", key))).collect::<Vec<String>>().join(", ")
    }
}

impl Default for StatTracker {
    fn default() -> Self {
        Self {
            kills: Some(0),
            assists: None,
            damage: None,
            points: None,
            healing: None,
        }
    }
}

pub struct ParticleEffect {
    pub name_key: String,
}

impl ParticleEffect {
    pub fn name(&self) -> String {
        get!(format!("particle_effect.{}", self.name_key))
    }
}
