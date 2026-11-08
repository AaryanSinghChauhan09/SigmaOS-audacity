//! Audio mixing definitions.

/// Mix channel
#[derive(Debug, Clone)]
pub struct MixChannel {
    /// Channel ID
    pub id: String,
    /// Channel name
    pub name: String,
    /// Channel volume (0.0 - 1.0)
    pub volume: f64,
    /// Channel pan (-1.0 to 1.0)
    pub pan: f64,
    /// Channel mute
    pub muted: bool,
    /// Channel solo
    pub solo: bool,
    /// Channel created timestamp
    pub created_at: u64,
}

impl MixChannel {
    /// Create new mix channel
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            volume: 1.0,
            pan: 0.0,
            muted: false,
            solo: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Set volume
    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume.clamp(0.0, 1.0);
    }
    
    /// Set pan
    pub fn set_pan(&mut self, pan: f64) {
        self.pan = pan.clamp(-1.0, 1.0);
    }
    
    /// Set muted
    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }
    
    /// Set solo
    pub fn set_solo(&mut self, solo: bool) {
        self.solo = solo;
    }
}

/// Mix state
#[derive(Debug, Clone, PartialEq)]
pub enum MixState {
    /// Idle mix
    Idle,
    /// Mixing
    Mixing,
    /// Recording
    Recording,
}

/// Mix level
#[derive(Debug, Clone, PartialEq)]
pub enum MixLevel {
    /// Low level
    Low,
    /// Medium level
    Medium,
    /// High level
    High,
    /// Clip level
    Clip,
}

/// Mixer
#[derive(Debug, Clone)]
pub struct Mixer {
    /// Mixer ID
    pub id: String,
    /// Mixer name
    pub name: String,
    /// Mixer state
    pub state: MixState,
    /// Mixer channels
    pub channels: Vec<MixChannel>,
    /// Master volume (0.0 - 1.0)
    pub master_volume: f64,
    /// Master mute
    pub master_muted: bool,
    /// Current mix level
    pub current_level: MixLevel,
    /// Mixer created timestamp
    pub created_at: u64,
}

impl Mixer {
    /// Create new mixer
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            state: MixState::Idle,
            channels: vec![],
            master_volume: 1.0,
            master_muted: false,
            current_level: MixLevel::Low,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Add channel
    pub fn add_channel(&mut self, channel: MixChannel) {
        self.channels.push(channel);
    }
    
    /// Remove channel
    pub fn remove_channel(&mut self, channel_id: &str) -> bool {
        if let Some(pos) = self.channels.iter().position(|c| c.id == channel_id) {
            self.channels.remove(pos);
            return true;
        }
        false
    }
    
    /// Get channel
    pub fn get_channel(&self, channel_id: &str) -> Option<&MixChannel> {
        self.channels.iter().find(|c| c.id == channel_id)
    }
    
    /// Get channel mutably
    pub fn get_channel_mut(&mut self, channel_id: &str) -> Option<&mut MixChannel> {
        self.channels.iter_mut().find(|c| c.id == channel_id)
    }
    
    /// Set master volume
    pub fn set_master_volume(&mut self, volume: f64) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }
    
    /// Set master mute
    pub fn set_master_muted(&mut self, muted: bool) {
        self.master_muted = muted;
    }
    
    /// Set mix level
    pub fn set_mix_level(&mut self, level: MixLevel) {
        self.current_level = level;
    }
    
    /// Start mixing
    pub fn start_mixing(&mut self) {
        self.state = MixState::Mixing;
    }
    
    /// Stop mixing
    pub fn stop_mixing(&mut self) {
        self.state = MixState::Idle;
    }
    
    /// Start recording
    pub fn start_recording(&mut self) {
        self.state = MixState::Recording;
    }
    
    /// Stop recording
    pub fn stop_recording(&mut self) {
        self.state = MixState::Idle;
    }
    
    /// Get channel count
    pub fn get_channel_count(&self) -> usize {
        self.channels.len()
    }
    
    /// Check if mixing
    pub fn is_mixing(&self) -> bool {
        self.state == MixState::Mixing
    }
    
    /// Check if recording
    pub fn is_recording(&self) -> bool {
        self.state == MixState::Recording
    }
    
    /// Get mixer age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
}
