//! Eye diagram viewer state.
//!
//! Loaded data, its measurements and the reader's settings for it, consumed
//! by `eye_diagram::view`. The data revision keys the
//! view's density texture.

use super::super::{EyeData, EyeMeasureOptions, EyeMeasurements, calculate_eye_measurements_with};
use super::{EyeSettings, EyeTimebaseProvenance, MaskFrame};

/// Complete eye diagram viewer state
#[derive(Debug, Clone, Default)]
pub struct EyeDiagramState {
    /// Eye data
    pub data: EyeData,
    /// Measurements derived from the data
    pub measurements: EyeMeasurements,
    /// What the reader set the sheet to for the result shown: the levels
    /// the eye is measured with, the error rate it is read at and the mask.
    /// The host keeps them per result and hands them back each frame.
    settings: EyeSettings,
    /// What the loaded data was folded at, when the controller said.
    ///
    /// The sheet quotes this beside the eye: an eye whose bit period was
    /// recovered from six edges and one whose rate the reader stated are not
    /// the same claim, and only one of them is the reader's own.
    timebase_provenance: Option<EyeTimebaseProvenance>,
    /// Bumped on every load so display caches can key on it rather than on
    /// data identity.
    data_revision: u64,
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

    /// The settings the eye is measured and masked with.
    pub fn settings(&self) -> &EyeSettings {
        &self.settings
    }

    /// Take in the settings the host keeps for the result shown: a change
    /// of levels measures the eye again, a change of mask tests it again.
    pub fn adopt_settings(&mut self, settings: &EyeSettings) {
        // The last test's verdict is the sheet's, not the reader's setting.
        let mut incoming = settings.clone();
        incoming.mask.clear_test();
        let mut held = self.settings.clone();
        held.mask.clear_test();
        if incoming == held {
            return;
        }
        let remeasure = incoming.levels != held.levels;
        self.settings = incoming;
        if remeasure {
            self.recalculate_measurements();
        }
        self.run_mask_test();
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
        self.run_mask_test();
    }

    /// Show or hide the compliance mask, testing it against the
    /// acquisitions loaded now.
    pub fn set_show_mask(&mut self, show: bool) {
        self.settings.show_mask = show;
        self.run_mask_test();
    }

    /// Recalculate measurements from current data
    pub fn recalculate_measurements(&mut self) {
        self.measurements = calculate_eye_measurements_with(
            &self.data,
            &EyeMeasureOptions {
                levels: self.settings.levels,
                ..EyeMeasureOptions::default()
            },
        );
    }

    /// Where the mask is centred on the loaded eye: the window's centre and
    /// the threshold of its middle opening, with the eye's amplitude.
    pub fn mask_frame(&self) -> MaskFrame {
        let openings = &self.measurements.openings;
        let threshold = if openings.len() > 1 {
            openings[(openings.len() - 1) / 2].threshold
        } else {
            self.data.v_cross
        };
        MaskFrame {
            centre_ui: f64::from(self.data.ui_count) / 2.0,
            threshold,
            amplitude: self.measurements.amplitude.unwrap_or(self.data.swing),
        }
    }

    /// Update the mask verdict for the currently loaded acquisitions.
    pub fn run_mask_test(&mut self) {
        if !self.settings.show_mask {
            self.settings.mask.clear_test();
            return;
        }
        let frame = self.mask_frame();
        self.settings.mask.test_traces(
            frame,
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
