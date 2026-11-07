//! audacity model — audio editor.

#[derive(Debug, Clone)]
pub struct Track {
    pub name: String,
    pub type_: String,
    pub samples: u64,
    pub muted: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub tracks: Vec<Track>,
    pub selection_start: u64,
    pub selection_end: u64,
}

impl Default for Model {
    fn default() -> Self { Self { tracks: vec![], selection_start: 0, selection_end: 0 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_track(&mut self, track: Track) { self.tracks.push(track); }

    pub fn select(&mut self, start: u64, end: u64) {
        self.selection_start = start;
        self.selection_end = end;
    }

    pub fn export(&self, format: &str) -> bool {
        !self.tracks.is_empty()
    }

    pub fn list_tracks(&self) -> &[Track] { &self.tracks }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn track_select_export() {
        let mut a = Model::new();
        a.add_track(Track { name: "vox".into(), type_: "audio".into(), samples: 44100, muted: false });
        a.select(0, 22050);
        assert!(a.export("wav"));
        assert_eq!(a.list_tracks().len(), 1);
    }
}
