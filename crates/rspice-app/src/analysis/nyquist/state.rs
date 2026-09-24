//! Nyquist viewer state: the loop-gain locus currently loaded.
//!
//! One locus, because a result carries one loop gain: a stability run measures
//! the return ratio at one probe and retains one contour for it. The list of
//! curves and the selected index that used to live here were filled by handing
//! every complex AC waveform to the viewer as though each were a loop gain,
//! and nothing ever moved the selection.
//!
//! The Nyquist viewer draws its own axes, critical point, and annotations, so
//! this holds no display policy. The overlay enum, the grid/critical-point/
//! equal-axes toggles, the manual axis ranges, and the encirclement and
//! stability queries that used to live here were a controls row and a
//! stability readout that were never built.

use super::data::NyquistData;

/// Complete Nyquist plot viewer state
#[derive(Debug, Clone, Default)]
pub struct NyquistState {
    /// The retained loop-gain locus.
    curve: Option<NyquistData>,
}

impl NyquistState {
    /// Replace the contents with a single locus.
    pub fn load_data(&mut self, data: NyquistData) {
        self.curve = Some(data);
    }

    /// The loaded locus, if there is one.
    pub fn curve(&self) -> Option<&NyquistData> {
        self.curve.as_ref()
    }

    /// Drop the locus.
    pub fn clear(&mut self) {
        self.curve = None;
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.curve.as_ref().is_none_or(NyquistData::is_empty)
    }
}
