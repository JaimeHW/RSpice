//! Runtime consumers for user-level engineering preferences.
//!
//! Preference pages never own execution semantics. This module applies the
//! resolved user defaults only at the runtime boundaries that can safely
//! consume them without mutating stored project data.

use crate::workbench::app::RSpiceApp;

impl RSpiceApp {
    /// Check the drawing in front of the reader against the live rules once
    /// an edit to it settles, when live checking is on.
    ///
    /// The checks are the production rules a check of the cellview runs,
    /// restricted to the ones a drawing alone decides, so a live finding is
    /// the same finding the explicit check would state. A frame with no edit
    /// costs nothing; one that waits for an edit to settle asks for the frame
    /// that ends the wait.
    pub(in crate::workbench) fn refresh_live_checks(&mut self, ctx: &egui::Context) {
        if let Some(wait) = self
            .state
            .refresh_live_checks(crate::time_compat::Instant::now())
        {
            ctx.request_repaint_after(wait);
        }
    }
}
