use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;
use crate::common::item::item::{ParticleEffect, Prototype};

pub mod item;

lazy_static! {
    pub static ref PROTOTYPES: HashMap<String, Arc<Prototype>> = init_prototype_map();
    pub static ref PARTICLES: HashMap<String, Arc<ParticleEffect>> = init_particle_effects();
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

fn init_particle_effects() -> HashMap<String, Arc<ParticleEffect>> {
    let mut effects: HashMap<String, Arc<ParticleEffect>> = HashMap::new();

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
