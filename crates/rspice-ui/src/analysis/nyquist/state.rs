//! Nyquist viewer state: the loaded curves and which one is selected.
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
    /// Nyquist data curves
    pub curves: Vec<NyquistData>,
    /// Selected curve index
    pub selected: usize,
}

impl NyquistState {
    /// Replace the contents with a single curve.
    pub fn load_data(&mut self, data: NyquistData) {
        self.curves = vec![data];
        self.selected = 0;
    }

    /// Append a curve alongside the ones already loaded.
    pub fn add_curve(&mut self, data: NyquistData) {
        self.curves.push(data);
    }

    /// Drop every curve.
    pub fn clear(&mut self) {
        self.curves.clear();
        self.selected = 0;
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.curves.is_empty()
    }
}
