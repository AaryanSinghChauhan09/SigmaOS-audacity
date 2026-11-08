//! Audio clip definitions.

/// Clip type
#[derive(Debug, Clone, PartialEq)]
pub enum ClipType {
    /// Audio clip
    Audio,
    /// MIDI clip
    MIDI,
    /// Label clip
    Label,
    /// Silence clip
    Silence,
}

/// Audio format
#[derive(Debug, Clone, PartialEq)]
pub enum AudioFormat {
    /// WAV format
    WAV,
    /// MP3 format
    MP3,
    /// FLAC format
    FLAC,
    /// OGG format
    OGG,
    /// AIFF format
    AIFF,
}

/// Clip info
#[derive(Debug, Clone)]
pub struct ClipInfo {
    /// Source file path
    pub source_path: String,
    /// Audio format
    pub format: AudioFormat,
    /// Sample rate
    pub sample_rate: u32,
    /// Bit depth
    pub bit_depth: u16,
    /// Channels
    pub channels: u16,
    /// Duration in seconds
    pub duration: f64,
    /// Start offset in seconds
    pub start_offset: f64,
    /// End offset in seconds
    pub end_offset: f64,
}

/// Clip
#[derive(Debug, Clone)]
pub struct Clip {
    /// Clip ID
    pub id: String,
    /// Clip name
    pub name: String,
    /// Clip type
    pub clip_type: ClipType,
    /// Clip info
    pub info: ClipInfo,
    /// Clip start position in seconds
    pub start_position: f64,
    /// Clip duration in seconds
    pub duration: f64,
    /// Clip volume (0.0 - 1.0)
    pub volume: f64,
    /// Clip fade in duration
    pub fade_in: f64,
    /// Clip fade out duration
    pub fade_out: f64,
    /// Clip created timestamp
    pub created_at: u64,
    /// Clip modified timestamp
    pub modified_at: u64,
}

impl Clip {
    /// Create new clip
    pub fn new(id: String, clip_type: ClipType, sample_rate: u32, channels: u16) -> Self {
        Self {
            id,
            name: String::new(),
            clip_type,
            info: ClipInfo {
                source_path: String::new(),
                format: AudioFormat::WAV,
                sample_rate,
                bit_depth: 16,
                channels,
                duration: 0.0,
                start_offset: 0.0,
                end_offset: 0.0,
            },
            start_position: 0.0,
            duration: 0.0,
            volume: 1.0,
            fade_in: 0.0,
            fade_out: 0.0,
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
    
    /// Set info
    pub fn set_info(&mut self, info: ClipInfo) {
        self.info = info;
        self.duration = self.info.duration;
        self.mark_modified();
    }
    
    /// Set start position
    pub fn set_start_position(&mut self, position: f64) {
        self.start_position = position.max(0.0);
        self.mark_modified();
    }
    
    /// Set duration
    pub fn set_duration(&mut self, duration: f64) {
        self.duration = duration.max(0.0);
        self.mark_modified();
    }
    
    /// Set volume
    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume.clamp(0.0, 1.0);
        self.mark_modified();
    }
    
    /// Set fade in
    pub fn set_fade_in(&mut self, fade_in: f64) {
        self.fade_in = fade_in.max(0.0);
        self.mark_modified();
    }
    
    /// Set fade out
    pub fn set_fade_out(&mut self, fade_out: f64) {
        self.fade_out = fade_out.max(0.0);
        self.mark_modified();
    }
    
    /// Mark as modified
    fn mark_modified(&mut self) {
        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Get end position
    pub fn get_end_position(&self) -> f64 {
        self.start_position + self.duration
    }
    
    /// Check if clip overlaps with position
    pub fn overlaps(&self, position: f64, duration: f64) -> bool {
        let clip_end = self.get_end_position();
        let other_end = position + duration;
        self.start_position < other_end && position < clip_end
    }
    
    /// Get clip age in seconds
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
