//! Command-line options for audacity.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Default mode
    Default,
    /// New project mode
    New(String),
    /// Open project mode
    Open(String),
    /// Export mode
    Export(String, String),
    /// Record mode
    Record,
    /// Play mode
    Play,
    /// Stop mode
    Stop,
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Config file path
    pub config_path: String,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            config_path: String::new(),
            verbose: false,
            quiet: false,
        }
    }
}
