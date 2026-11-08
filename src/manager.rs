//! Audio manager for SigmaOS-audacity.

use std::collections::HashMap;
use crate::project::{Project, ProjectState};
use crate::track::{Track, TrackType};
use crate::clip::{Clip, ClipType};
use crate::effect::{Effect, EffectType, EffectChain};
use crate::mix::{Mixer, MixChannel};
use crate::export::{ExportFormat, ExportConfig, ExportResult};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max projects
    pub max_projects: usize,
    /// Max tracks per project
    pub max_tracks_per_project: usize,
    /// Max clips per track
    pub max_clips_per_track: usize,
    /// Max effects per track
    pub max_effects_per_track: usize,
    /// Default sample rate
    pub default_sample_rate: u32,
    /// Default bit depth
    pub default_bit_depth: u16,
    /// Default channels
    pub default_channels: u16,
    /// Auto-save projects
    pub auto_save: bool,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_projects: 50,
            max_tracks_per_project: 100,
            max_clips_per_track: 1000,
            max_effects_per_track: 50,
            default_sample_rate: 44100,
            default_bit_depth: 16,
            default_channels: 2,
            auto_save: false,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total projects
    pub total_projects: usize,
    /// Total tracks
    pub total_tracks: usize,
    /// Total clips
    pub total_clips: usize,
    /// Total effects
    pub total_effects: usize,
    /// Total mixers
    pub total_mixers: usize,
    /// Total exports
    pub total_exports: usize,
    /// Successful exports
    pub successful_exports: usize,
    /// Failed exports
    pub failed_exports: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Audio manager
#[derive(Debug, Clone)]
pub struct AudioManager {
    /// Projects
    projects: HashMap<String, Project>,
    /// Tracks
    tracks: HashMap<String, Track>,
    /// Clips
    clips: HashMap<String, Clip>,
    /// Effects
    effects: HashMap<String, Effect>,
    /// Effect chains
    effect_chains: HashMap<String, EffectChain>,
    /// Mixers
    mixers: HashMap<String, Mixer>,
    /// Export results
    export_results: Vec<ExportResult>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self {
            projects: HashMap::new(),
            tracks: HashMap::new(),
            clips: HashMap::new(),
            effects: HashMap::new(),
            effect_chains: HashMap::new(),
            mixers: HashMap::new(),
            export_results: vec![],
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_projects: 0,
                total_tracks: 0,
                total_clips: 0,
                total_effects: 0,
                total_mixers: 0,
                total_exports: 0,
                successful_exports: 0,
                failed_exports: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl AudioManager {
    /// Create new audio manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            projects: HashMap::new(),
            tracks: HashMap::new(),
            clips: HashMap::new(),
            effects: HashMap::new(),
            effect_chains: HashMap::new(),
            mixers: HashMap::new(),
            export_results: vec![],
            options,
            statistics: ManagerStatistics {
                total_projects: 0,
                total_tracks: 0,
                total_clips: 0,
                total_effects: 0,
                total_mixers: 0,
                total_exports: 0,
                successful_exports: 0,
                failed_exports: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Create default mixer
        let mixer = Mixer::new("default-mixer".into(), "Default Mixer".into());
        self.mixers.insert("default-mixer".into(), mixer);
        self.update_statistics();
    }
    
    // Project methods
    
    /// Create project
    pub fn create_project(&mut self, name: String) -> String {
        let project = Project::new(name);
        let id = project.id.clone();
        self.projects.insert(id.clone(), project);
        self.update_statistics();
        id
    }
    
    /// Get project
    pub fn get_project(&self, project_id: &str) -> Option<&Project> {
        self.projects.get(project_id)
    }
    
    /// Get project mutably
    pub fn get_project_mut(&mut self, project_id: &str) -> Option<&mut Project> {
        self.projects.get_mut(project_id)
    }
    
    /// Get all projects
    pub fn list_projects(&self) -> Vec<&Project> {
        self.projects.values().collect()
    }
    
    /// Delete project
    pub fn delete_project(&mut self, project_id: &str) -> bool {
        if let Some(project) = self.projects.remove(project_id) {
            // Clean up associated tracks, clips, effects
            for track_id in &project.tracks {
                if let Some(track) = self.tracks.remove(track_id) {
                    for clip_id in &track.clips {
                        self.clips.remove(clip_id);
                    }
                    for effect_id in &track.effects {
                        self.effects.remove(effect_id);
                    }
                }
            }
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Save project
    pub fn save_project(&mut self, project_id: &str) -> bool {
        if let Some(project) = self.projects.get_mut(project_id) {
            project.mark_saved();
            return true;
        }
        false
    }
    
    /// Get projects by state
    pub fn get_projects_by_state(&self, state: &ProjectState) -> Vec<&Project> {
        self.projects.values()
            .filter(|p| p.state == *state)
            .collect()
    }
    
    /// Search projects
    pub fn search_projects(&self, query: &str) -> Vec<&Project> {
        let query_lower = query.to_lowercase();
        self.projects.values()
            .filter(|p| p.name.to_lowercase().contains(&query_lower) ||
                       p.description.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Track methods
    
    /// Add track to project
    pub fn add_track(&mut self, project_id: &str, name: String, track_type: TrackType) -> Option<String> {
        if let Some(project) = self.projects.get_mut(project_id) {
            if project.tracks.len() >= self.options.max_tracks_per_project {
                return None;
            }
            
            let track = Track::new(name, track_type);
            let track_id = track.id.clone();
            
            project.add_track(track_id.clone());
            self.tracks.insert(track_id.clone(), track);
            self.update_statistics();
            return Some(track_id);
        }
        None
    }
    
    /// Get track
    pub fn get_track(&self, track_id: &str) -> Option<&Track> {
        self.tracks.get(track_id)
    }
    
    /// Get track mutably
    pub fn get_track_mut(&mut self, track_id: &str) -> Option<&mut Track> {
        self.tracks.get_mut(track_id)
    }
    
    /// Get all tracks
    pub fn list_tracks(&self) -> Vec<&Track> {
        self.tracks.values().collect()
    }
    
    /// Delete track
    pub fn delete_track(&mut self, project_id: &str, track_id: &str) -> bool {
        if let Some(project) = self.projects.get_mut(project_id) {
            if project.remove_track(track_id) {
                if let Some(track) = self.tracks.remove(track_id) {
                    for clip_id in &track.clips {
                        self.clips.remove(clip_id);
                    }
                    for effect_id in &track.effects {
                        self.effects.remove(effect_id);
                    }
                }
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Get project tracks
    pub fn get_project_tracks(&self, project_id: &str) -> Vec<&Track> {
        if let Some(project) = self.projects.get(project_id) {
            project.tracks.iter()
                .filter_map(|id| self.tracks.get(id))
                .collect()
        } else {
            vec![]
        }
    }
    
    // Clip methods
    
    /// Add clip to track
    pub fn add_clip(&mut self, project_id: &str, track_id: &str, clip_type: ClipType, sample_rate: u32, channels: u16) -> Option<String> {
        if let Some(track) = self.tracks.get_mut(track_id) {
            if track.clips.len() >= self.options.max_clips_per_track {
                return None;
            }
            
            let clip = Clip::new(format!("clip-{}", self.clips.len() + 1), clip_type, sample_rate, channels);
            let clip_id = clip.id.clone();
            
            track.add_clip(clip_id.clone());
            self.clips.insert(clip_id.clone(), clip);
            self.update_statistics();
            return Some(clip_id);
        }
        None
    }
    
    /// Get clip
    pub fn get_clip(&self, clip_id: &str) -> Option<&Clip> {
        self.clips.get(clip_id)
    }
    
    /// Get clip mutably
    pub fn get_clip_mut(&mut self, clip_id: &str) -> Option<&mut Clip> {
        self.clips.get_mut(clip_id)
    }
    
    /// Get all clips
    pub fn list_clips(&self) -> Vec<&Clip> {
        self.clips.values().collect()
    }
    
    /// Delete clip
    pub fn delete_clip(&mut self, track_id: &str, clip_id: &str) -> bool {
        if let Some(track) = self.tracks.get_mut(track_id) {
            if track.remove_clip(clip_id) {
                self.clips.remove(clip_id);
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Get track clips
    pub fn get_track_clips(&self, track_id: &str) -> Vec<&Clip> {
        if let Some(track) = self.tracks.get(track_id) {
            track.clips.iter()
                .filter_map(|id| self.clips.get(id))
                .collect()
        } else {
            vec![]
        }
    }
    
    // Effect methods
    
    /// Add effect to track
    pub fn add_effect(&mut self, project_id: &str, track_id: &str, effect_type: EffectType) -> Option<String> {
        if let Some(track) = self.tracks.get_mut(track_id) {
            if track.effects.len() >= self.options.max_effects_per_track {
                return None;
            }
            
            let effect = Effect::new(format!("effect-{}", self.effects.len() + 1), effect_type);
            let effect_id = effect.id.clone();
            
            track.add_effect(effect_id.clone());
            self.effects.insert(effect_id.clone(), effect);
            self.update_statistics();
            return Some(effect_id);
        }
        None
    }
    
    /// Get effect
    pub fn get_effect(&self, effect_id: &str) -> Option<&Effect> {
        self.effects.get(effect_id)
    }
    
    /// Get effect mutably
    pub fn get_effect_mut(&mut self, effect_id: &str) -> Option<&mut Effect> {
        self.effects.get_mut(effect_id)
    }
    
    /// Get all effects
    pub fn list_effects(&self) -> Vec<&Effect> {
        self.effects.values().collect()
    }
    
    /// Delete effect
    pub fn delete_effect(&mut self, track_id: &str, effect_id: &str) -> bool {
        if let Some(track) = self.tracks.get_mut(track_id) {
            if track.remove_effect(effect_id) {
                self.effects.remove(effect_id);
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Get track effects
    pub fn get_track_effects(&self, track_id: &str) -> Vec<&Effect> {
        if let Some(track) = self.tracks.get(track_id) {
            track.effects.iter()
                .filter_map(|id| self.effects.get(id))
                .collect()
        } else {
            vec![]
        }
    }
    
    // Effect chain methods
    
    /// Create effect chain
    pub fn create_effect_chain(&mut self, name: String) -> String {
        let mut chain = EffectChain::new();
        chain.set_name(name);
        let id = chain.id.clone();
        self.effect_chains.insert(id.clone(), chain);
        self.update_statistics();
        id
    }
    
    /// Get effect chain
    pub fn get_effect_chain(&self, chain_id: &str) -> Option<&EffectChain> {
        self.effect_chains.get(chain_id)
    }
    
    /// Get effect chain mutably
    pub fn get_effect_chain_mut(&mut self, chain_id: &str) -> Option<&mut EffectChain> {
        self.effect_chains.get_mut(chain_id)
    }
    
    /// Get all effect chains
    pub fn list_effect_chains(&self) -> Vec<&EffectChain> {
        self.effect_chains.values().collect()
    }
    
    /// Delete effect chain
    pub fn delete_effect_chain(&mut self, chain_id: &str) -> bool {
        self.effect_chains.remove(chain_id).is_some()
    }
    
    // Mixer methods
    
    /// Create mixer
    pub fn create_mixer(&mut self, name: String) -> String {
        let mixer = Mixer::new(format!("mixer-{}", self.mixers.len() + 1), name);
        let id = mixer.id.clone();
        self.mixers.insert(id.clone(), mixer);
        self.update_statistics();
        id
    }
    
    /// Get mixer
    pub fn get_mixer(&self, mixer_id: &str) -> Option<&Mixer> {
        self.mixers.get(mixer_id)
    }
    
    /// Get mixer mutably
    pub fn get_mixer_mut(&mut self, mixer_id: &str) -> Option<&mut Mixer> {
        self.mixers.get_mut(mixer_id)
    }
    
    /// Get all mixers
    pub fn list_mixers(&self) -> Vec<&Mixer> {
        self.mixers.values().collect()
    }
    
    /// Delete mixer
    pub fn delete_mixer(&mut self, mixer_id: &str) -> bool {
        self.mixers.remove(mixer_id).is_some()
    }
    
    /// Add channel to mixer
    pub fn add_mixer_channel(&mut self, mixer_id: &str, name: String) -> Option<String> {
        if let Some(mixer) = self.mixers.get_mut(mixer_id) {
            let channel = MixChannel::new(format!("ch-{}", mixer.channels.len() + 1), name);
            let channel_id = channel.id.clone();
            mixer.add_channel(channel);
            return Some(channel_id);
        }
        None
    }
    
    // Export methods
    
    /// Export project
    pub fn export_project(&mut self, project_id: &str, format: ExportFormat, output_path: String) -> Option<ExportResult> {
        if let Some(project) = self.projects.get_mut(project_id) {
            let config = ExportConfig {
                format,
                output_path: output_path.clone(),
                ..Default::default()
            };
            
            // Simulate export
            let result = ExportResult::success(output_path, 1024 * 1024, 10.0);
            
            project.mark_exported();
            self.export_results.push(result.clone());
            self.update_statistics();
            return Some(result);
        }
        None
    }
    
    /// Get export results
    pub fn get_export_results(&self) -> &[ExportResult] {
        &self.export_results
    }
    
    /// Get successful exports
    pub fn get_successful_exports(&self) -> Vec<&ExportResult> {
        self.export_results.iter()
            .filter(|r| r.is_success())
            .collect()
    }
    
    /// Get failed exports
    pub fn get_failed_exports(&self) -> Vec<&ExportResult> {
        self.export_results.iter()
            .filter(|r| !r.is_success())
            .collect()
    }
    
    /// Clear export results
    pub fn clear_export_results(&mut self) {
        self.export_results.clear();
        self.update_statistics();
    }
    
    // Statistics methods
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_projects = self.projects.len();
        self.statistics.total_tracks = self.tracks.len();
        self.statistics.total_clips = self.clips.len();
        self.statistics.total_effects = self.effects.len();
        self.statistics.total_mixers = self.mixers.len();
        self.statistics.total_exports = self.export_results.len();
        self.statistics.successful_exports = self.export_results.iter()
            .filter(|r| r.is_success())
            .count();
        self.statistics.failed_exports = self.export_results.iter()
            .filter(|r| !r.is_success())
            .count();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            projects: self.projects.values().cloned().collect(),
            tracks: self.tracks.values().cloned().collect(),
            clips: self.clips.values().cloned().collect(),
            effects: self.effects.values().cloned().collect(),
            effect_chains: self.effect_chains.values().cloned().collect(),
            mixers: self.mixers.values().cloned().collect(),
            export_results: self.export_results.clone(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.projects.clear();
        for project in export.projects {
            self.projects.insert(project.id.clone(), project);
        }
        
        self.tracks.clear();
        for track in export.tracks {
            self.tracks.insert(track.id.clone(), track);
        }
        
        self.clips.clear();
        for clip in export.clips {
            self.clips.insert(clip.id.clone(), clip);
        }
        
        self.effects.clear();
        for effect in export.effects {
            self.effects.insert(effect.id.clone(), effect);
        }
        
        self.effect_chains.clear();
        for chain in export.effect_chains {
            self.effect_chains.insert(chain.id.clone(), chain);
        }
        
        self.mixers.clear();
        for mixer in export.mixers {
            self.mixers.insert(mixer.id.clone(), mixer);
        }
        
        self.export_results = export.export_results;
        self.update_statistics();
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Projects
    pub projects: Vec<Project>,
    /// Tracks
    pub tracks: Vec<Track>,
    /// Clips
    pub clips: Vec<Clip>,
    /// Effects
    pub effects: Vec<Effect>,
    /// Effect chains
    pub effect_chains: Vec<EffectChain>,
    /// Mixers
    pub mixers: Vec<Mixer>,
    /// Export results
    pub export_results: Vec<ExportResult>,
}
