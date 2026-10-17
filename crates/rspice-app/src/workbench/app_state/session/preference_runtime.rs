//! Runtime consumers for user-level engineering preferences.
//!
//! Preference pages never own execution semantics. This module applies the
//! resolved user defaults only at the runtime boundaries that can safely
//! consume them without mutating stored project data.

use crate::time_compat::Instant;
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
        self.refresh_live_checks_at(ctx, Instant::now());
    }

    /// [`Self::refresh_live_checks`], with the frame's clock read as `now`.
    ///
    /// The canvas and the Problems page are drawn before this runs, so a
    /// frame that publishes or withdraws findings asks for the next frame at
    /// once: otherwise the markers and rows it produced would wait for the
    /// reader's next input to be drawn.
    fn refresh_live_checks_at(&mut self, ctx: &egui::Context, now: Instant) {
        let published = self.state.diagnostics.generation();
        if let Some(wait) = self.state.refresh_live_checks(now) {
            ctx.request_repaint_after(wait);
        }
        if self.state.diagnostics.generation() != published {
            ctx.request_repaint();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point};
    use std::time::Duration;

    /// How long the frame that ran at `now` asks the next one to wait.
    fn frame(app: &mut RSpiceApp, ctx: &egui::Context, now: Instant) -> Duration {
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            app.refresh_live_checks_at(ui.ctx(), now);
        });
        output
            .viewport_output
            .get(&egui::ViewportId::ROOT)
            .map_or(Duration::MAX, |viewport| viewport.repaint_delay)
    }

    /// A live check runs in the frame that finds the drawing settled, after
    /// that frame has drawn the canvas and the page. That frame asks for the
    /// next at once, so the findings it published are drawn without the
    /// reader moving the mouse; a frame that publishes nothing asks for
    /// nothing.
    #[test]
    fn a_frame_that_publishes_live_findings_asks_for_the_next_at_once() {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.authority.open_session();
        let ctx = egui::Context::default();
        let start = Instant::now();
        for _ in 0..3 {
            frame(&mut app, &ctx, start);
        }
        assert_ne!(frame(&mut app, &ctx, start), Duration::ZERO, "settled");

        app.state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(400, 400));
        let waiting = frame(&mut app, &ctx, start);
        assert!(
            waiting > Duration::ZERO && waiting < Duration::MAX,
            "an edit asks for the frame that ends its wait: {waiting:?}"
        );

        let later = start + Duration::from_secs(1);
        assert_eq!(
            frame(&mut app, &ctx, later),
            Duration::ZERO,
            "the frame that published the findings asks for the next"
        );
        assert_ne!(
            crate::workbench::app_state::diagnostic_store::listing::listing(&app.state).len(),
            0,
            "the live pass was published"
        );
        // egui answers a repaint request with two frames, so the frame after
        // the publishing one is egui's own; the one after that is quiet.
        frame(&mut app, &ctx, later);
        assert_ne!(
            frame(&mut app, &ctx, later),
            Duration::ZERO,
            "a frame that publishes nothing asks for nothing"
        );
    }
}
