//! Bode availability state used by run lifecycle and viewer-capability checks.
//! Inspectors consume the selected retained response through the sibling `inspector` module.

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
