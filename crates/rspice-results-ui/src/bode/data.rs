//! Number of retained frequency responses used by viewer availability checks.

/// Collection of frequency responses for Bode plot
#[derive(Debug, Clone, Default)]
pub struct BodeData {
    responses: usize,
}

impl BodeData {
    /// Create new empty Bode data
    pub fn new() -> Self {
        Self::default()
    }

    /// Record that one more frequency response was produced.
    pub fn add_response(&mut self) {
        self.responses += 1;
    }

    /// Number of responses
    pub fn response_count(&self) -> usize {
        self.responses
    }
}
