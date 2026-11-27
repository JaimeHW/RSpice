//! How a printed plot's grid is ruled.
//!
//! A plot is ruled at the gridlines it declares, captions and all: a
//! logarithmic sweep at its decades, with the eight interior mantissas of
//! each, because that is what its axis has, and a linear one at the values
//! its axis names. A plane is ruled by its own guides instead.

use super::*;
use crate::sources::report_layout::set_width;
use rspice_fonts::Role;

/// A value's lettering.
const VALUE_TEXT_UM: u64 = 2_300;
/// The room left of a frame a value can take: the inset, less the space
/// before the frame and a little air at the edge.
const VALUE_GUTTER_UM: u64 = crate::sources::PLOT_INSET_UM.unsigned_abs() - 1_700;
/// How far a unit set on its own line stands above its value's baseline.
const UNIT_RISE_UM: i64 = 3_100;
/// How far a value label's baseline stands below its tick.
const VALUE_DROP_UM: u64 = 700;
/// How far a value label's capitals rise above its tick.
const VALUE_ASCENT_UM: u64 = 950;

impl SemanticSceneCompiler<'_> {
    /// Rule the plot's grid at the gridlines and guides it declares.
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
        for guide in &plot.guides {
            let stroke = if guide.major {
                StrokeStyle::try_new(
                    SemanticColor::Secondary,
                    Length::from_micrometres(220),
                    StrokePattern::Solid,
                    None,
                )?
            } else if guide.second {
                // The sheet inks a second family in its orange trace hue at
                // a third of its strength, as a printed Z-Y chart inks its
                // admittance grid apart from its impedance grid.
                let [red, green, blue] = crate::sources::TRACE_PRINT_INKS[5];
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
            // A value whose unit makes it wider than the gutter keeps its
            // number on its rule and stands the unit on the line above,
            // rather than running past the edge of the page.
            let lines = match tick.label.split_once(' ') {
                Some((value, unit))
                    if tick.axis == crate::sources::SemanticAxisKind::Vertical
                        && set_width(Role::Mono, &tick.label, VALUE_TEXT_UM) > VALUE_GUTTER_UM =>
                {
                    vec![(value, 0), (unit, UNIT_RISE_UM)]
                }
                _ => vec![(tick.label.as_str(), 0)],
            };
            for (text, rise) in lines {
                let at = self.offset_scene_point(caption, 0, -rise)?;
                self.add_text_placed(
                    at,
                    text,
                    SceneFont::Monospace,
                    VALUE_TEXT_UM,
                    color,
                    (anchor, SceneTextRotation::Upright),
                )?;
            }
        }
        Ok(())
    }
}
