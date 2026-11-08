//! SigmaOS-audacity — comprehensive audio editor component.
//!
//! Models audio projects, tracks, clips, effects, mixing, and export
//! for the SigmaOS multimedia subsystem.

pub mod project;
pub mod track;
pub mod clip;
pub mod effect;
pub mod mix;
pub mod export;
pub mod manager;
pub mod options;

pub use project::{Project, ProjectInfo, ProjectState};
pub use track::{Track, TrackType, TrackState, TrackInfo};
pub use clip::{Clip, ClipType, ClipInfo, AudioFormat};
pub use effect::{Effect, EffectType, EffectParams, EffectChain};
pub use mix::{MixChannel, Mixer, MixState, MixLevel};
pub use export::{ExportFormat, ExportConfig, ExportResult};
pub use manager::{AudioManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_project_state() {
        let states = vec![
            ProjectState::New,
            ProjectState::Open,
            ProjectState::Modified,
            ProjectState::Saved,
            ProjectState::Exported,
        ];
        assert_eq!(states.len(), 5);
    }
    
    #[test]
    fn test_track_type() {
        let types = vec![
            TrackType::Mono,
            TrackType::Stereo,
            TrackType::MultiChannel,
        ];
        assert_eq!(types.len(), 3);
    }
    
    #[test]
    fn test_clip_type() {
        let types = vec![
            ClipType::Audio,
            ClipType::MIDI,
            ClipType::Label,
            ClipType::Silence,
        ];
        assert_eq!(types.len(), 4);
    }
    
    #[test]
    fn test_effect_type() {
        let types = vec![
            EffectType::Reverb,
            EffectType::Delay,
            EffectType::EQ,
            EffectType::Compression,
            EffectType::NoiseReduction,
            EffectType::Normalize,
            EffectType::Amplify,
            EffectType::Fade,
        ];
        assert_eq!(types.len(), 8);
    }
    
    #[test]
    fn test_export_format() {
        let formats = vec![
            ExportFormat::WAV,
            ExportFormat::MP3,
            ExportFormat::FLAC,
            ExportFormat::OGG,
            ExportFormat::AIFF,
        ];
        assert_eq!(formats.len(), 5);
    }
    
    #[test]
    fn test_project_creation() {
        let project = Project::new("My Song".into());
        assert_eq!(project.name, "My Song");
        assert_eq!(project.state, ProjectState::New);
    }
    
    #[test]
    fn test_track_creation() {
        let track = Track::new("Track 1".into(), TrackType::Stereo);
        assert_eq!(track.name, "Track 1");
        assert_eq!(track.track_type, TrackType::Stereo);
    }
    
    #[test]
    fn test_clip_creation() {
        let clip = Clip::new("clip-1".into(), ClipType::Audio, 44100, 2);
        assert_eq!(clip.id, "clip-1");
        assert_eq!(clip.clip_type, ClipType::Audio);
        assert_eq!(clip.info.sample_rate, 44100);
    }
    
    #[test]
    fn test_effect_creation() {
        let effect = Effect::new("reverb-1".into(), EffectType::Reverb);
        assert_eq!(effect.id, "reverb-1");
        assert_eq!(effect.effect_type, EffectType::Reverb);
        assert!(effect.enabled);
    }
    
    #[test]
    fn test_effect_chain() {
        let mut chain = EffectChain::new();
        let effect1 = Effect::new("e1".into(), EffectType::EQ);
        let effect2 = Effect::new("e2".into(), EffectType::Compression);
        
        chain.add_effect(effect1);
        chain.add_effect(effect2);
        
        assert_eq!(chain.effects.len(), 2);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = AudioManager::new();
        assert!(manager.list_projects().is_empty());
    }
    
    #[test]
    fn test_create_project() {
        let mut manager = AudioManager::new();
        let project_id = manager.create_project("Test Project".into());
        
        assert!(manager.get_project(&project_id).is_some());
        assert_eq!(manager.list_projects().len(), 1);
    }
    
    #[test]
    fn test_add_track_to_project() {
        let mut manager = AudioManager::new();
        let project_id = manager.create_project("Test".into());
        
        let track_id = manager.add_track(&project_id, "Track 1".into(), TrackType::Stereo);
        assert!(track_id.is_some());
        
        let project = manager.get_project(&project_id).unwrap();
        assert_eq!(project.tracks.len(), 1);
    }
    
    #[test]
    fn test_add_effect_to_track() {
        let mut manager = AudioManager::new();
        let project_id = manager.create_project("Test".into());
        let track_id = manager.add_track(&project_id, "Track 1".into(), TrackType::Stereo).unwrap();
        
        let effect_id = manager.add_effect(&project_id, &track_id, EffectType::Reverb);
        assert!(effect_id.is_some());
    }
    
    #[test]
    fn test_export_project() {
        let mut manager = AudioManager::new();
        let project_id = manager.create_project("Test".into());
        
        let result = manager.export_project(&project_id, ExportFormat::WAV, "/tmp/out.wav".into());
        assert!(result.is_some());
        assert!(result.unwrap().success);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = AudioManager::new();
        manager.create_project("Project1".into());
        
        let export = manager.export_data();
        
        let mut manager2 = AudioManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_projects().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = AudioManager::new();
        let project_id = manager.create_project("Test".into());
        manager.add_track(&project_id, "Track 1".into(), TrackType::Stereo);
        manager.add_track(&project_id, "Track 2".into(), TrackType::Mono);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_projects, 1);
        assert_eq!(stats.total_tracks, 2);
    }
}
