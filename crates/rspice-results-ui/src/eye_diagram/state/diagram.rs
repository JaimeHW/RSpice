//! Eye diagram viewer state.
//!
//! Loaded data, measurements, compliance mask and local controls consumed by
//! `eye_diagram::view`. The data revision keys the view's density texture.

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

    /// Update the mask verdict for the currently loaded acquisitions.
    pub fn run_mask_test(&mut self) {
        self.mask.test_traces(
            self.data
                .traces
                .iter()
                .map(|trace| (trace.time.as_slice(), trace.amplitude.as_slice())),
        );
    }

    /// Number of traces in the loaded data
    pub fn trace_count(&self) -> usize {
        self.data.trace_count()
    }
}
