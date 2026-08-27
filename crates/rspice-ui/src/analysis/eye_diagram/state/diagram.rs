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
use super::{EyeMask, EyeTimebaseProvenance};

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
    /// What the loaded data was folded at, when the controller said.
    ///
    /// The sheet quotes this beside the eye: an eye whose bit period was
    /// recovered from six edges and one whose rate the reader stated are not
    /// the same claim, and only one of them is the reader's own.
    timebase_provenance: Option<EyeTimebaseProvenance>,
    /// The bit-period editor, while it is open.
    ///
    /// A half-typed rate belongs to the viewer, not to the project: it lives
    /// here beside the eye it is about rather than in the Results document's
    /// persisted presentation state, which a draft has no business entering.
    pub rate_editor: Option<EyeRateEditor>,
    /// Bumped on every load so display caches can key on it rather than on
    /// data identity.
    data_revision: u64,
}

/// The open bit-period editor: what the reader has typed, and why it was
/// refused if it was.
#[derive(Debug, Clone, Default)]
pub struct EyeRateEditor {
    pub text: String,
    pub error: Option<String>,
    /// Set for the frame the editor opens, so the field takes focus once
    /// instead of taking it back every frame and never letting go.
    pub needs_focus: bool,
}

impl EyeDiagramState {
    /// Monotonic revision of the loaded data, for display caches.
    pub fn data_revision(&self) -> u64 {
        self.data_revision
    }

    /// What the eye on screen was folded at, when it is known.
    pub fn timebase_provenance(&self) -> Option<&EyeTimebaseProvenance> {
        self.timebase_provenance.as_ref()
    }

    /// Load eye data and recalculate measurements
    pub fn load_data(&mut self, data: EyeData) {
        self.load_data_with_timebase(data, None);
    }

    /// Load eye data together with the provenance of the fold.
    pub fn load_data_with_timebase(
        &mut self,
        data: EyeData,
        provenance: Option<EyeTimebaseProvenance>,
    ) {
        self.data_revision = self.data_revision.wrapping_add(1);
        self.data = data;
        self.timebase_provenance = provenance;
        self.recalculate_measurements();
        if self.show_mask {
            self.mask.enabled = true;
        }
        self.run_mask_test();
    }

    /// Show or hide the compliance mask.
    ///
    /// Enabling the mask runs the test against the acquisitions that are
    /// loaded now. The verdict used to be latched at load time, so a reader
    /// who turned the mask on after the eye arrived read `0 / 0` violations
    /// — a pass — over a mask that had never been tested against anything.
    pub fn set_show_mask(&mut self, show: bool) {
        self.show_mask = show;
        self.mask.enabled = show;
        self.run_mask_test();
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
        self.mask.violation_count = 0;
        self.mask.total_samples = 0;
        self.mask.margin = None;
        if !self.mask.enabled {
            return;
        }

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
        self.mask.margin = super::mask::geometric_margin(&inner, &self.data.traces);
    }

    /// Number of traces in the loaded data
    pub fn trace_count(&self) -> usize {
        self.data.trace_count()
    }
}
