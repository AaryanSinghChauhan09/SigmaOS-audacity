//! Audio track definitions.

/// Track type
#[derive(Debug, Clone, PartialEq)]
pub enum TrackType {
    /// Mono track
    Mono,
    /// Stereo track
    Stereo,
    /// Multi-channel track
    MultiChannel,
}

/// Track state
#[derive(Debug, Clone, PartialEq)]
pub enum TrackState {
    /// Empty track
    Empty,
    /// Has clips
    HasClips,
    /// Muted track
    Muted,
    /// Solo track
    Solo,
}

/// Track info
#[derive(Debug, Clone)]
pub struct TrackInfo {
    /// Sample rate
    pub sample_rate: u32,
    /// Bit depth
    pub bit_depth: u16,
    /// Channels
    pub channels: u16,
    /// Duration in seconds
    pub duration: f64,
}

impl Default for TrackInfo {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            bit_depth: 16,
            channels: 2,
            duration: 0.0,
        }
    }
}

/// Track
#[derive(Debug, Clone)]
pub struct Track {
    /// Track ID
    pub id: String,
    /// Track name
    pub name: String,
    /// Track type
    pub track_type: TrackType,
    /// Track state
    pub state: TrackState,
    /// Track info
    pub info: TrackInfo,
    /// Track clips
    pub clips: Vec<String>,
    /// Track effects
    pub effects: Vec<String>,
    /// Track volume (0.0 - 1.0)
    pub volume: f64,
    /// Track pan (-1.0 to 1.0)
    pub pan: f64,
    /// Track mute
    pub muted: bool,
    /// Track solo
    pub solo: bool,
    /// Track created timestamp
    pub created_at: u64,
    /// Track modified timestamp
    pub modified_at: u64,
}

impl Track {
    /// Create new track
    pub fn new(name: String, track_type: TrackType) -> Self {
        Self {
            id: format!("track-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            track_type,
            state: TrackState::Empty,
            info: TrackInfo::default(),
            clips: vec![],
            effects: vec![],
            volume: 1.0,
            pan: 0.0,
            muted: false,
            solo: false,
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
    
    /// Add clip
    pub fn add_clip(&mut self, clip_id: String) {
        if !self.clips.contains(&clip_id) {
            self.clips.push(clip_id);
            self.update_state();
            self.mark_modified();
        }
    }
    
    /// Remove clip
    pub fn remove_clip(&mut self, clip_id: &str) -> bool {
        if let Some(pos) = self.clips.iter().position(|c| c == clip_id) {
            self.clips.remove(pos);
            self.update_state();
            self.mark_modified();
            return true;
        }
        false
    }
    
    /// Add effect
    pub fn add_effect(&mut self, effect_id: String) {
        if !self.effects.contains(&effect_id) {
            self.effects.push(effect_id);
            self.mark_modified();
        }
    }
    
    /// Remove effect
    pub fn remove_effect(&mut self, effect_id: &str) -> bool {
        if let Some(pos) = self.effects.iter().position(|e| e == effect_id) {
            self.effects.remove(pos);
            self.mark_modified();
            return true;
        }
        false
    }
    
    /// Set volume
    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume.clamp(0.0, 1.0);
        self.mark_modified();
    }
    
    /// Set pan
    pub fn set_pan(&mut self, pan: f64) {
        self.pan = pan.clamp(-1.0, 1.0);
        self.mark_modified();
    }
    
    /// Set muted
    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
        self.update_state();
        self.mark_modified();
    }
    
    /// Set solo
    pub fn set_solo(&mut self, solo: bool) {
        self.solo = solo;
        self.update_state();
        self.mark_modified();
    }
    
    /// Update state based on clips/mute/solo
    fn update_state(&mut self) {
        if self.muted {
            self.state = TrackState::Muted;
        } else if self.solo {
            self.state = TrackState::Solo;
        } else if !self.clips.is_empty() {
            self.state = TrackState::HasClips;
        } else {
            self.state = TrackState::Empty;
        }
    }
    
    /// Mark as modified
    fn mark_modified(&mut self) {
        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if track has clips
    pub fn has_clips(&self) -> bool {
        !self.clips.is_empty()
    }
    
    /// Check if track is empty
    pub fn is_empty(&self) -> bool {
        self.clips.is_empty()
    }
    
    /// Check if track is muted
    pub fn is_muted(&self) -> bool {
        self.muted
    }
    
    /// Check if track is solo
    pub fn is_solo(&self) -> bool {
        self.solo
    }
    
    /// Get clip count
    pub fn get_clip_count(&self) -> usize {
        self.clips.len()
    }
    
    /// Get effect count
    pub fn get_effect_count(&self) -> usize {
        self.effects.len()
    }
    
    /// Get track age in seconds
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
