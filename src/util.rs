//! audacity model — audio editor/recorder.

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub tracks: Vec<String>,
    pub duration: f64,
    pub sample_rate: u32,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub projects: Vec<Project>,
}

impl Default for Model {
    fn default() -> Self { Self { projects: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, name: &str, sample_rate: u32) -> usize {
        let id = self.projects.len();
        self.projects.push(Project { name: name.into(), tracks: vec![], duration: 0.0, sample_rate });
        id
    }

    pub fn add_track(&mut self, project_id: usize, track_name: &str) -> bool {
        if let Some(p) = self.projects.get_mut(project_id) {
            p.tracks.push(track_name.into());
            true
        } else { false }
    }

    pub fn record(&mut self, project_id: usize, duration: f64) -> bool {
        if let Some(p) = self.projects.get_mut(project_id) {
            p.duration += duration;
            true
        } else { false }
    }

    pub fn list(&self) -> &[Project] { &self.projects }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_track_record() {
        let mut a = Model::new();
        let id = a.create("song1", 44100);
        assert!(a.add_track(id, "vocals"));
        assert!(a.record(id, 30.0));
        assert_eq!(a.projects[id].duration, 30.0);
    }
}
