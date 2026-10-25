//! FFT view settings: the window function and the displayed spectrum's
//! revision.

use super::super::window::WindowFunction;
use super::*;

impl FftState {
    /// Set window function
    pub fn set_window(&mut self, window: WindowFunction) -> Result<(), FftFailure> {
        if self.window == window && self.has_data() {
            return Ok(());
        }
        self.window = window;
        self.recompute_from_source()
    }

    /// Monotonic revision of the displayed spectrum; bumped on every
    /// recompute. Display caches key on this rather than data identity.
    pub fn spectrum_revision(&self) -> u64 {
        self.spectrum_revision
    }

    pub(super) fn mark_spectrum_changed(&mut self) {
        self.spectrum_revision = self.spectrum_revision.wrapping_add(1);
    }
}
