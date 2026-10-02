//! Bode viewer state: the loaded frequency responses.
//!
//! `result_document::bode` builds its display model from the AC Bode summary
//! in `state::simulation::ac_bode`, not from here — it owns the axis ranges,
//! the cursor, and the margin annotations. What this type provides is whether
//! a Bode result is present at all, which is what the viewer-capability check
//! and the run lifecycle ask.
//!
//! The display-mode and phase-wrap enums, the grid / margin / cursor toggles,
//! the manual and auto magnitude and phase ranges, trace selection, and a
//! `margins()` accessor lived here for a controls row that was never built.

use super::data::BodeData;

/// Bode plot viewer state
#[derive(Debug, Clone, Default)]
pub struct BodePlotState {
    /// Frequency response data
    pub data: BodeData,
}

impl BodePlotState {
    /// Replace the loaded data.
    pub fn load_data(&mut self, data: BodeData) {
        self.data = data;
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.data.response_count() == 0
    }
}
