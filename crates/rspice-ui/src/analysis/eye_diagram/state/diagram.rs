//! Eye diagram viewer state.
//!
//! Only what `result_document::eye` reads: the loaded data, its measurements,
//! the compliance mask and whether to draw it, and a revision counter the
//! viewer's density-texture cache keys on.
//!
//! The persistence and colour-map settings, grid and measurement toggles,
//! trace selection, manual h/v scales, the pan/zoom view range, cursors,
//! markers, the persistence cache, and the measurement-pane width hints were a
//! controls row and an interaction model that were never built. The viewer
//! rasterises density itself and derives its axes from `data.ui_count`.

use super::super::EyeData;
use super::super::EyeMeasurements;
use super::EyeMask;

/// Complete eye diagram viewer state
#[derive(Debug, Clone, Default)]
pub struct EyeDiagramState {
    /// Eye data
    pub data: EyeData,
    /// Measurements derived from the data
    pub measurements: EyeMeasurements,
    /// Compliance mask
    pub mask: EyeMask,
    /// Draw the mask
    pub show_mask: bool,
    /// Bumped on every load so display caches can key on it rather than on
    /// data identity.
    data_revision: u64,
}

impl EyeDiagramState {
    /// Monotonic revision of the loaded data, for display caches.
    pub fn data_revision(&self) -> u64 {
        self.data_revision
    }

    /// Load eye data and recalculate measurements
    pub fn load_data(&mut self, data: EyeData) {
        self.data_revision = self.data_revision.wrapping_add(1);
        self.data = data;
        self.recalculate_measurements();
        if self.show_mask {
            self.mask.enabled = true;
            self.run_mask_test();
        }
    }

    /// Recalculate measurements from current data
    pub fn recalculate_measurements(&mut self) {
        self.measurements = super::super::calculate_eye_measurements(&self.data);
    }

    /// Count how many samples fall inside the compliance mask.
    ///
    /// Trace samples are already display-space (unit intervals, volts) — the
    /// same space the absolute mask maps into. The polygon is mapped once
    /// rather than per sample; its times scale by the UI ratio while its
    /// voltages stay absolute (see [`EyeMask::inner_in_ui_volts`]).
    pub fn run_mask_test(&mut self) {
        if !self.mask.enabled {
            return;
        }

        self.mask.violation_count = 0;
        self.mask.total_samples = 0;

        let inner = self.mask.inner_in_ui_volts();
        for trace in &self.data.traces {
            let n = trace.time.len().min(trace.amplitude.len());
            for i in 0..n {
                if inner.contains(trace.time[i], trace.amplitude[i]) {
                    self.mask.violation_count += 1;
                }
                self.mask.total_samples += 1;
            }
        }
    }

    /// Number of traces in the loaded data
    pub fn trace_count(&self) -> usize {
        self.data.trace_count()
    }
}
