//! What a headless pass actually painted, for surfaces that must not clip.
//!
//! A layout test that asserts over state proves the state; it does not prove
//! that the reader can see it. This walks the shapes one pass emitted, records
//! every text run with the rect it was clipped to, and lets a test say the one
//! thing a screenshot review says: nothing ran off its pane or off the card.
//!
//! It is the reusable form of the walk `stimulus_link` wrote for its own list.
//! The difference is the assertion: a scrolling list is judged horizontally
//! only, because a row below the fold is a row the reader can reach, while the
//! command forms this helper serves are scroll-free by construction, so a run
//! below the fold is a run nobody will ever see.

use egui::{Rect, Vec2};

use crate::ui::tokens::Mode;

/// Every text run one pass painted, with the surface it was painted on.
pub(crate) struct PaintedRuns {
    /// `(text, run rect, clip rect)`, in paint order.
    pub(crate) runs: Vec<(String, Rect, Rect)>,
    /// The largest opaque card painted inside the viewport: the dialog surface.
    pub(crate) surface: Rect,
}

impl PaintedRuns {
    /// Single-line editors scroll their content within a fixed clip. Require
    /// those explicitly named runs to retain a visible horizontal slice and
    /// keep the clip inside the card; all other text must fit in full.
    pub(crate) fn assert_inside_clip_and_surface_with_horizontal_scroll(
        &self,
        label: &str,
        scroll_values: &[&str],
    ) {
        assert!(
            self.surface.is_finite() && self.surface.width() > 1.0,
            "{label}: no dialog surface was painted"
        );
        for (text, rect, clip) in &self.runs {
            if scroll_values.contains(&text.as_str()) {
                assert!(
                    self.surface.expand(1.0).contains_rect(*clip),
                    "{label}: editor clip {clip:?} leaves the card {:?}",
                    self.surface
                );
                assert!(
                    rect.top() >= clip.top() - 1.0 && rect.bottom() <= clip.bottom() + 1.0,
                    "{label}: editor {text:?} is vertically clipped"
                );
                assert!(
                    rect.intersect(*clip).width() > 1.0,
                    "{label}: editor {text:?} has no visible content"
                );
                continue;
            }
            let past_surface = (self.surface.left() - rect.left())
                .max(rect.right() - self.surface.right())
                .max(self.surface.top() - rect.top())
                .max(rect.bottom() - self.surface.bottom());
            assert!(
                past_surface <= 1.0,
                "{label}: {text:?} lands {past_surface} points outside the surface \
                 (run {rect:?}, surface {:?})",
                self.surface
            );
            let past_clip = (clip.left() - rect.left())
                .max(rect.right() - clip.right())
                .max(clip.top() - rect.top())
                .max(rect.bottom() - clip.bottom());
            assert!(
                past_clip <= 1.0,
                "{label}: {text:?} runs {past_clip} points past its pane \
                 (run {rect:?}, pane {clip:?})"
            );
        }
    }

    /// Where a run with exactly this text was painted.
    pub(crate) fn rect_of(&self, text: &str) -> Option<Rect> {
        self.runs
            .iter()
            .find(|(painted, _, _)| painted == text)
            .map(|(_, rect, _)| *rect)
    }
}

/// Lay `pass` out `passes` times at `screen` in `mode`, and read back the last
/// pass.
///
/// More than one pass is the norm rather than a precaution: the first builds
/// the font set and measures a content-height surface, and only a pass laid out
/// against both measurements paints where the reader will see it.
pub(crate) fn painted_runs(
    screen: Vec2,
    mode: Mode,
    passes: usize,
    mut pass: impl FnMut(&egui::Context),
) -> PaintedRuns {
    let ctx = egui::Context::default();
    crate::ui::Theme {
        mode,
        ..crate::ui::Theme::default()
    }
    .apply(&ctx);
    let input = || egui::RawInput {
        screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, screen)),
        ..egui::RawInput::default()
    };
    let mut output = ctx.run_ui(input(), |ui| pass(ui));
    for _ in 1..passes.max(1) {
        output = ctx.run_ui(input(), |ui| pass(ui));
    }

    let mut painted = PaintedRuns {
        runs: Vec::new(),
        surface: Rect::ZERO,
    };
    for clipped in &output.shapes {
        walk(&clipped.shape, clipped.clip_rect, &mut painted, screen);
    }
    painted
}

fn walk(shape: &egui::epaint::Shape, clip: Rect, into: &mut PaintedRuns, screen: Vec2) {
    match shape {
        egui::epaint::Shape::Text(painted) => {
            let text = painted.galley.job.text.clone();
            into.runs.push((
                text,
                Rect::from_min_size(painted.pos, painted.galley.size()),
                clip,
            ));
        }
        egui::epaint::Shape::Rect(painted) => {
            // The scrim covers the viewport and the shadow is a blurred rect
            // wider than the card it sits under; neither is the surface. Of
            // what is left, the surface is the largest, because every strip
            // inside it is a part of it.
            let rect = painted.rect;
            if painted.blur_width == 0.0
                && rect.width() < screen.x - 1.0
                && rect.area() > into.surface.area()
            {
                into.surface = rect;
            }
        }
        egui::epaint::Shape::Vec(shapes) => {
            for shape in shapes {
                walk(shape, clip, into, screen);
            }
        }
        _ => {}
    }
}
