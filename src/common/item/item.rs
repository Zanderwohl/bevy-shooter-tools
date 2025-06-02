use std::sync::Arc;
use crate::get;

#[derive(Clone)]
pub struct Item {
    prototype: Arc<Prototype>,
    name: Option<String>,
    display_name_cache: Option<String>,
    description: Option<String>,
    stat_tracker: Option<StatTracker>,
    particle_effect: Option<Arc<ParticleEffect>>,
    trade_restriction: bool,
    destroyed: bool,
}

impl Item {
    pub fn display_name(&self) -> String {
        if let Some(name) = &self.name {
           return name.clone();
        }

        self.display_name_default()
    }

    pub fn display_name_default(&self) -> String {
        let mut name = get!(self.prototype.name_key);
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
}

#[derive(Clone)]
pub struct ItemNetworkable {
    prototype_key: String,
    name: Option<String>,
    description: Option<String>,
    stat_tracker: Option<StatTracker>,
    particle_effect_key: Option<String>,
    trade_restriction: bool,
    destroyed: bool,
}

#[derive(Clone)]
pub struct Prototype {
    name_key: String,
    stock: bool,
    trade_restriction: bool,
}

impl Prototype {
    pub fn tradeable(&self) -> bool {
        !self.trade_restriction && !self.stock
    }
}

#[derive(Clone)]
pub struct StatTracker {
    kills: Option<u32>,
    assists: Option<u32>,
    damage: Option<u32>,
    points: Option<u32>,
    healing: Option<u32>,
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

struct ParticleEffect {
    
}
