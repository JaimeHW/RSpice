//! Pole-zero viewer state: which datasets are loaded.
//!
//! The plot is built by `result_document::pz`, which assembles its own
//! [`PoleZeroData`] from the run and reads `roots` and `name` off it directly.
//! This type exists so the workspace can ask whether a pole-zero result is
//! present and drop it when the run changes.
//!
//! It used to also carry the domain (s vs z), manual and auto axis ranges, and
//! grid / unit-circle / stability-region / dominant-pole / annotation toggles
//! with a setter apiece — a controls row that was never built, over ranges the
//! viewer computes for itself.

use super::data::PoleZeroData;

/// Pole-zero viewer state
#[derive(Debug, Clone, Default)]
pub struct PoleZeroState {
    /// Pole-zero data sets
    pub datasets: Vec<PoleZeroData>,
    /// Selected dataset index
    pub selected: usize,
}

impl PoleZeroState {
    /// Replace the contents with a single dataset.
    #[cfg(test)]
    pub fn load_data(&mut self, data: PoleZeroData) {
        self.datasets = vec![data];
        self.selected = 0;
    }

    /// Drop every dataset.
    pub fn clear(&mut self) {
        self.datasets.clear();
        self.selected = 0;
    }

    /// Is empty?
    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.datasets.is_empty()
    }
}
