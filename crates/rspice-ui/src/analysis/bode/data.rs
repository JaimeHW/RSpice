//! Bode data: how many frequency responses an AC run produced.
//!
//! Held so the workspace can tell whether a Bode result exists. The Bode
//! viewer draws from the AC Bode summary in `state::simulation::ac_bode` and
//! computes its own margins, so nothing downstream reads the samples.
//!
//! What used to live here was a frequency-response analysis library: DC gain,
//! 3 dB bandwidth, phase and magnitude ranges, interpolation at a frequency,
//! angular frequency, and a `StabilityMargins` type with its own gain/phase
//! margin search and formatters. All of it was unreachable, and the margin
//! search in particular was a second implementation of what
//! `result_document::bode` does against the summary. The per-point
//! `FrequencyResponse` went the same way once it was clear the points were
//! collected and never read — what survives is the count that answers
//! "is there a Bode result?".

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
