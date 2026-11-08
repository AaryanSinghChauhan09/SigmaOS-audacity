//! Audio export definitions.

/// Export format
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
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

/// Export config
#[derive(Debug, Clone)]
pub struct ExportConfig {
    /// Export format
    pub format: ExportFormat,
    /// Sample rate
    pub sample_rate: u32,
    /// Bit depth
    pub bit_depth: u16,
    /// Channels
    pub channels: u16,
    /// Quality (0-100)
    pub quality: u8,
    /// Output path
    pub output_path: String,
    /// Normalize audio
    pub normalize: bool,
    /// Fade in duration
    pub fade_in: f64,
    /// Fade out duration
    pub fade_out: f64,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::WAV,
            sample_rate: 44100,
            bit_depth: 16,
            channels: 2,
            quality: 90,
            output_path: String::new(),
            normalize: false,
            fade_in: 0.0,
            fade_out: 0.0,
        }
    }
}

/// Export result
#[derive(Debug, Clone)]
pub struct ExportResult {
    /// Export success
    pub success: bool,
    /// Output file path
    pub output_path: String,
    /// Export size in bytes
    pub size_bytes: u64,
    /// Export duration in seconds
    pub duration: f64,
    /// Export error
    pub error: Option<String>,
    /// Export timestamp
    pub timestamp: u64,
}

impl ExportResult {
    /// Create successful export result
    pub fn success(output_path: String, size_bytes: u64, duration: f64) -> Self {
        Self {
            success: true,
            output_path,
            size_bytes,
            duration,
            error: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Create failed export result
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            output_path: String::new(),
            size_bytes: 0,
            duration: 0.0,
            error: Some(error),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Check if export was successful
    pub fn is_success(&self) -> bool {
        self.success
    }
    
    /// Get export age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.timestamp
    }
}
