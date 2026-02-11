use super::*;

/// A simulation run containing multiple analysis results.
///
/// This represents a single invocation of the simulator, which may contain
/// multiple analyses (e.g., DC Op + Transient + AC in one run).
///
/// Follows Cadence Spectre PSF database conventions:
/// - Timestamped runs for identification
/// - Multiple analyses per run
/// - Retained in history for comparison
#[derive(Debug, Clone)]
pub struct SimulationRun {
    /// Unique run identifier (monotonically increasing)
    pub id: u64,
    /// Human-readable label with timestamp (e.g., "Run 3 (1:04:01 PM)")
    pub label: String,
    /// Unix timestamp when run started
    pub timestamp: f64,
    /// All analysis results from this run
    pub analyses: Vec<AnalysisResult>,
    /// Total simulation time in seconds
    pub elapsed_time: f64,
    /// Whether all analyses in this run succeeded
    pub success: bool,
}

impl SimulationRun {
    /// Create a new simulation run with auto-generated ID and timestamp
    pub fn new(run_number: u64) -> Self {
        let timestamp = Self::current_timestamp();
        let time_str = Self::format_time(timestamp);
        Self {
            id: run_number,
            label: format!("Run {} ({})", run_number, time_str),
            timestamp,
            analyses: Vec::new(),
            elapsed_time: 0.0,
            success: true,
        }
    }

    /// Add an analysis result to this run
    pub fn add_analysis(&mut self, analysis: AnalysisResult) {
        if !analysis.success {
            self.success = false;
        }
        self.analyses.push(analysis);
    }

    /// Set the total elapsed time for this run
    pub fn set_elapsed_time(&mut self, elapsed: f64) {
        self.elapsed_time = elapsed;
    }

    /// Get count of successful analyses
    pub fn successful_analyses(&self) -> usize {
        self.analyses.iter().filter(|a| a.success).count()
    }

    /// Find analysis by type (returns first match)
    pub fn find_analysis(&self, analysis_type: AnalysisType) -> Option<&AnalysisResult> {
        self.analyses
            .iter()
            .find(|a| a.analysis_type == analysis_type)
    }

    /// Get current timestamp as Unix epoch seconds
    fn current_timestamp() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs_f64())
            .unwrap_or(0.0)
    }

    /// Format timestamp as human-readable time string (e.g., "1:04:01 PM")
    fn format_time(timestamp: f64) -> String {
        use std::time::{Duration, UNIX_EPOCH};
        let _datetime = UNIX_EPOCH + Duration::from_secs_f64(timestamp);
        // Use chrono-free approach for portability
        let secs = timestamp as u64;
        let hours = (secs / 3600) % 24;
        let minutes = (secs / 60) % 60;
        let seconds = secs % 60;
        let (hour_12, am_pm) = if hours == 0 {
            (12, "AM")
        } else if hours < 12 {
            (hours, "AM")
        } else if hours == 12 {
            (12, "PM")
        } else {
            (hours - 12, "PM")
        };
        format!("{}:{:02}:{:02} {}", hour_12, minutes, seconds, am_pm)
    }
}
