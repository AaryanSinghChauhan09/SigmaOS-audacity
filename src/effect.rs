//! Audio effects definitions.

/// Effect type
#[derive(Debug, Clone, PartialEq)]
pub enum EffectType {
    /// Reverb effect
    Reverb,
    /// Delay effect
    Delay,
    /// EQ effect
    EQ,
    /// Compression effect
    Compression,
    /// Noise reduction effect
    NoiseReduction,
    /// Normalize effect
    Normalize,
    /// Amplify effect
    Amplify,
    /// Fade effect
    Fade,
}

/// Effect parameters
#[derive(Debug, Clone)]
pub struct EffectParams {
    /// Parameter name
    pub name: String,
    /// Parameter value
    pub value: f64,
    /// Parameter min value
    pub min: f64,
    /// Parameter max value
    pub max: f64,
    /// Parameter default value
    pub default: f64,
}

/// Effect
#[derive(Debug, Clone)]
pub struct Effect {
    /// Effect ID
    pub id: String,
    /// Effect name
    pub name: String,
    /// Effect type
    pub effect_type: EffectType,
    /// Effect parameters
    pub params: Vec<EffectParams>,
    /// Effect enabled
    pub enabled: bool,
    /// Effect wet/dry mix (0.0 - 1.0)
    pub mix: f64,
    /// Effect created timestamp
    pub created_at: u64,
    /// Effect modified timestamp
    pub modified_at: u64,
}

impl Effect {
    /// Create new effect
    pub fn new(id: String, effect_type: EffectType) -> Self {
        Self {
            id,
            name: format!("{:?}", effect_type),
            effect_type,
            params: vec![],
            enabled: true,
            mix: 1.0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            modified_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.mark_modified();
    }
    
    /// Add parameter
    pub fn add_param(&mut self, param: EffectParams) {
        self.params.push(param);
        self.mark_modified();
    }
    
    /// Set parameter value
    pub fn set_param(&mut self, name: &str, value: f64) -> bool {
        if let Some(param) = self.params.iter_mut().find(|p| p.name == name) {
            param.value = value.clamp(param.min, param.max);
            self.mark_modified();
            return true;
        }
        false
    }
    
    /// Get parameter
    pub fn get_param(&self, name: &str) -> Option<&EffectParams> {
        self.params.iter().find(|p| p.name == name)
    }
    
    /// Set enabled
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.mark_modified();
    }
    
    /// Set mix
    pub fn set_mix(&mut self, mix: f64) {
        self.mix = mix.clamp(0.0, 1.0);
        self.mark_modified();
    }
    
    /// Mark as modified
    fn mark_modified(&mut self) {
        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if effect is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Get parameter count
    pub fn get_param_count(&self) -> usize {
        self.params.len()
    }
    
    /// Get effect age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get time since modification in seconds
    pub fn get_time_since_modification(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.modified_at
    }
}

/// Effect chain
#[derive(Debug, Clone)]
pub struct EffectChain {
    /// Chain ID
    pub id: String,
    /// Chain name
    pub name: String,
    /// Chain effects
    pub effects: Vec<Effect>,
    /// Chain enabled
    pub enabled: bool,
    /// Chain created timestamp
    pub created_at: u64,
    /// Chain modified timestamp
    pub modified_at: u64,
}

impl EffectChain {
    /// Create new effect chain
    pub fn new() -> Self {
        Self {
            id: format!("chain-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name: "Effect Chain".into(),
            effects: vec![],
            enabled: true,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            modified_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.mark_modified();
    }
    
    /// Add effect
    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
        self.mark_modified();
    }
    
    /// Remove effect
    pub fn remove_effect(&mut self, effect_id: &str) -> bool {
        if let Some(pos) = self.effects.iter().position(|e| e.id == effect_id) {
            self.effects.remove(pos);
            self.mark_modified();
            return true;
        }
        false
    }
    
    /// Get effect
    pub fn get_effect(&self, effect_id: &str) -> Option<&Effect> {
        self.effects.iter().find(|e| e.id == effect_id)
    }
    
    /// Get effect mutably
    pub fn get_effect_mut(&mut self, effect_id: &str) -> Option<&mut Effect> {
        self.effects.iter_mut().find(|e| e.id == effect_id)
    }
    
    /// Reorder effects
    pub fn reorder_effects(&mut self, new_order: Vec<String>) {
        let mut new_effects = Vec::new();
        for id in new_order {
            if let Some(pos) = self.effects.iter().position(|e| e.id == id) {
                new_effects.push(self.effects.remove(pos));
            }
        }
        // Add remaining effects
        new_effects.extend(self.effects.drain(..));
        self.effects = new_effects;
        self.mark_modified();
    }
    
    /// Set enabled
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.mark_modified();
    }
    
    /// Mark as modified
    fn mark_modified(&mut self) {
        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Get effect count
    pub fn get_effect_count(&self) -> usize {
        self.effects.len()
    }
    
    /// Check if chain is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Get chain age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get time since modification in seconds
    pub fn get_time_since_modification(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.modified_at
    }
}

impl Default for EffectChain {
    fn default() -> Self {
        Self::new()
    }
}
