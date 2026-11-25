//! How a printed plot's grid is ruled.
//!
//! A plot that declares its gridlines gets those, captions and all: a
//! logarithmic sweep is ruled at its decades, with the eight interior mantissas
//! of each, because that is what its axis has. A plot that declares none keeps
//! the frame's ten even divisions, which is what a linear sweep deserves — and
//! what every plot used to get, including the frequency responses that have no
//! even divisions to rule.

use super::*;

/// How far a value label's baseline stands below its tick.
const VALUE_DROP_UM: u64 = 700;
/// How far a value label's capitals rise above its tick.
const VALUE_ASCENT_UM: u64 = 950;

impl SemanticSceneCompiler<'_> {
    /// Rule the plot's grid.
    ///
    /// A plot that declares its gridlines gets those, captions and all: a
    /// logarithmic sweep is ruled at its decades, with the eight interior
    /// mantissas of each, because that is what its axis has. A plot that declares
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
        // A plane ruled by its own guides, as a Smith chart is, has no even
        // divisions to rule.
        if plot.axis_ticks.is_empty() && plot.guides.is_empty() {
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
        for guide in &plot.guides {
            let stroke = if guide.major {
                StrokeStyle::try_new(
                    SemanticColor::Secondary,
                    Length::from_micrometres(220),
                    StrokePattern::Solid,
                    None,
                )?
            } else if guide.second {
                // The sheet inks a second family in its seventh trace hue at
                // a third of its strength, as a printed Z-Y chart inks its
                // admittance grid apart from its impedance grid.
                let [red, green, blue] = crate::sources::TRACE_PRINT_INKS[6];
                let faint = |ink: u8| (255.0 - (255.0 - f64::from(ink)) * 0.32).round() as u8;
                StrokeStyle::try_new(
                    SemanticColor::Exact(Rgb8::new(faint(red), faint(green), faint(blue))),
                    Length::from_micrometres(120),
                    StrokePattern::Solid,
                    None,
                )?
            } else {
                StrokeStyle::try_new(
                    SemanticColor::Grid,
                    Length::from_micrometres(120),
                    StrokePattern::Solid,
                    None,
                )?
            };
            for path in guide.paths.iter().filter(|path| path.len() >= 2) {
                self.primitives.push(ScenePrimitive::Polyline {
                    points: path
                        .iter()
                        .map(|point| self.semantic_point(*point))
                        .collect::<Result<_, _>>()?,
                    closed: false,
                    stroke,
                    fill: None,
                });
            }
        }
        let strips = plot
            .frames
            .iter()
            .map(|frame| {
                Ok((
                    self.semantic_point(frame.minimum)?,
                    self.semantic_point(frame.maximum)?,
                ))
            })
            .collect::<Result<Vec<_>, HardcopyRenderError>>()?;
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
            let (caption, anchor) = match tick.axis {
                crate::sources::SemanticAxisKind::Horizontal => {
                    let foot = if from.y.micrometres() > to.y.micrometres() {
                        from
                    } else {
                        to
                    };
                    // A value at either end of the axis stands inside it, as
                    // the sheet nudges it, clear of the abscissa's caption.
                    let (left, x) = (frame.x.micrometres(), foot.x.micrometres());
                    let anchor = if x <= left + 1 {
                        TextAnchor::Start
                    } else if x + 1 >= left + frame.width.micrometres() {
                        TextAnchor::End
                    } else {
                        TextAnchor::Middle
                    };
                    (self.offset_scene_point(foot, 0, 2_600)?, anchor)
                }
                crate::sources::SemanticAxisKind::Vertical => {
                    let foot = if from.x.micrometres() < to.x.micrometres() {
                        from
                    } else {
                        to
                    };
                    // A value at either end of its strip stands inside it,
                    // as the sheet's axis painter nudges it, rather than
                    // half over the frame.
                    let y = foot.y.micrometres();
                    let strip = strips.iter().find(|(top, bottom)| {
                        (top.y.micrometres()..=bottom.y.micrometres()).contains(&y)
                    });
                    let nudge = strip.map_or(0, |(top, bottom)| {
                        let (top, bottom) = (top.y.micrometres(), bottom.y.micrometres());
                        let inside = y.clamp(
                            top.saturating_add(VALUE_ASCENT_UM),
                            bottom.saturating_sub(VALUE_DROP_UM).max(top),
                        );
                        i64::try_from(inside).unwrap_or(0) - i64::try_from(y).unwrap_or(0)
                    });
                    (
                        self.offset_scene_point(foot, -1_000, VALUE_DROP_UM as i64 + nudge)?,
                        TextAnchor::End,
                    )
                }
            };
            let color = SemanticColor::Secondary;
            self.add_text_placed(
                caption,
                &tick.label,
                SceneFont::Monospace,
                2_300,
                color,
                (anchor, SceneTextRotation::Upright),
            )?;
        }
        Ok(())
    }
}
