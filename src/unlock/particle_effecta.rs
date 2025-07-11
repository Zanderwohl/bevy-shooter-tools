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