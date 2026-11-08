//! Audio project definitions.

use std::collections::HashMap;

/// Project state
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectState {
    /// New project
    New,
    /// Open project
    Open,
    /// Modified project
    Modified,
    /// Saved project
    Saved,
    /// Exported project
    Exported,
}

/// Project info
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    /// Sample rate
    pub sample_rate: u32,
    /// Bit depth
    pub bit_depth: u16,
    /// Channels
    pub channels: u16,
    /// Duration in seconds
    pub duration: f64,
    /// Tempo in BPM
    pub tempo: f64,
    /// Time signature
    pub time_signature: String,
}

impl Default for ProjectInfo {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            bit_depth: 16,
            channels: 2,
            duration: 0.0,
            tempo: 120.0,
            time_signature: "4/4".into(),
        }
    }
}

/// Project
#[derive(Debug, Clone)]
pub struct Project {
    /// Project ID
    pub id: String,
    /// Project name
    pub name: String,
    /// Project description
    pub description: String,
    /// Project state
    pub state: ProjectState,
    /// Project info
    pub info: ProjectInfo,
    /// Project tracks
    pub tracks: Vec<String>,
    /// Project metadata
    pub metadata: HashMap<String, String>,
    /// Project created timestamp
    pub created_at: u64,
    /// Project modified timestamp
    pub modified_at: u64,
    /// Project saved timestamp
    pub saved_at: Option<u64>,
    /// Project exported timestamp
    pub exported_at: Option<u64>,
}

impl Project {
    /// Create new project
    pub fn new(name: String) -> Self {
        Self {
            id: format!("proj-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            description: String::new(),
            state: ProjectState::New,
            info: ProjectInfo::default(),
            tracks: vec![],
            metadata: HashMap::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            modified_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            saved_at: None,
            exported_at: None,
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.mark_modified();
    }
    
    /// Set info
    pub fn set_info(&mut self, info: ProjectInfo) {
        self.info = info;
        self.mark_modified();
    }
    
    /// Add track
    pub fn add_track(&mut self, track_id: String) {
        if !self.tracks.contains(&track_id) {
            self.tracks.push(track_id);
            self.mark_modified();
        }
    }
    
    /// Remove track
    pub fn remove_track(&mut self, track_id: &str) -> bool {
        if let Some(pos) = self.tracks.iter().position(|t| t == track_id) {
            self.tracks.remove(pos);
            self.mark_modified();
            return true;
        }
        false
    }
    
    /// Set metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.mark_modified();
    }
    
    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
    
    /// Mark as saved
    pub fn mark_saved(&mut self) {
        self.state = ProjectState::Saved;
        self.saved_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        self.modified_at = self.saved_at.unwrap();
    }
    
    /// Mark as exported
    pub fn mark_exported(&mut self) {
        self.state = ProjectState::Exported;
        self.exported_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
    }
    
    /// Mark as modified
    pub fn mark_modified(&mut self) {
        if self.state != ProjectState::New {
            self.state = ProjectState::Modified;
        }
        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Open project
    pub fn open(&mut self) {
        self.state = ProjectState::Open;
    }
    
    /// Check if project is modified
    pub fn is_modified(&self) -> bool {
        self.state == ProjectState::Modified
    }
    
    /// Check if project is saved
    pub fn is_saved(&self) -> bool {
        matches!(self.state, ProjectState::Saved | ProjectState::Exported)
    }
    
    /// Get track count
    pub fn get_track_count(&self) -> usize {
        self.tracks.len()
    }
    
    /// Check if project has tracks
    pub fn has_tracks(&self) -> bool {
        !self.tracks.is_empty()
    }
    
    /// Get project age in seconds
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
