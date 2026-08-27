//! How a printed plot's grid is ruled.
//!
//! A plot that declares its gridlines gets those, captions and all: a
//! logarithmic sweep is ruled at its decades, with the nine minor lines inside
//! each, because that is what its axis has. A plot that declares none keeps
//! the frame's ten even divisions, which is what a linear sweep deserves — and
//! what every plot used to get, including the frequency responses that have no
//! even divisions to rule.

use super::*;

impl SemanticSceneCompiler<'_> {
    /// Rule the plot's grid.
    ///
    /// A plot that declares its gridlines gets those, captions and all: a
    /// logarithmic sweep is ruled at its decades, with the nine minor lines
    /// inside each, because that is what its axis has. A plot that declares
    /// none keeps the frame's ten even divisions, which is what a linear
    /// sweep deserves and what every plot used to get — including the
    /// frequency responses that have no even divisions to rule.
    pub(super) fn plot_grid(
        &mut self,
        plot: &SemanticPlot,
        frame: SceneRect,
    ) -> Result<(), HardcopyRenderError> {
        let rule = |weight| {
            StrokeStyle::try_new(
                SemanticColor::Grid,
                Length::from_micrometres(weight),
                StrokePattern::Dotted,
                None,
            )
        };
        if plot.axis_ticks.is_empty() {
            let bottom =
                Length::from_micrometres(frame.y.micrometres() + frame.height.micrometres());
            for division in 1..10_u64 {
                let x = Length::from_micrometres(self.extent.width().micrometres() * division / 10);
                self.primitives.push(ScenePrimitive::Line {
                    from: ScenePoint::new(x, frame.y),
                    to: ScenePoint::new(x, bottom),
                    stroke: rule(120)?,
                });
            }
            return Ok(());
        }
        for tick in &plot.axis_ticks {
            let (from, to) = (
                self.semantic_point(tick.start)?,
                self.semantic_point(tick.end)?,
            );
            self.primitives.push(ScenePrimitive::Line {
                from,
                to,
                stroke: rule(if tick.major { 160 } else { 90 })?,
            });
            if tick.label.is_empty() {
                continue;
            }
            let foot = if from.y.micrometres() > to.y.micrometres() {
                from
            } else {
                to
            };
            let caption = self.offset_scene_point(foot, 600, 2_600)?;
            let color = SemanticColor::Secondary;
            self.add_text(caption, &tick.label, SceneFont::Monospace, 2_300, color)?;
        }
        Ok(())
    }
}
