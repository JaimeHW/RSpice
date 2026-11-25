//! A report figure, printed as the plot it keeps: ruled, valued and keyed
//! inside the box the report gives it.

use super::*;
use crate::sources::report_layout::set_width;
use crate::sources::{PLOT_INSET_UM, PlotArea, SemanticReportFigure, report_figure_plot};
use rspice_fonts::Role;
use rspice_results::report_document::FrozenReportArtifact;
use rspice_results::report_figure::ReportFigure;

/// Above a figure's frame: room for an offset axis's anchor.
const TOP_GUTTER_UM: u64 = 4_000;
/// Right of a figure's frame: its last value stands inside the frame, so
/// only air.
const RIGHT_GUTTER_UM: u64 = 2_000;
/// Under a figure's frame: the abscissa's values and caption, and its
/// anchor when it has one.
const FOOT_UM: u64 = 8_000;
/// The key's lettering, the pitch of its rows, the length of a swatch, and
/// the space after a swatch and after an entry.
const KEY_TEXT_UM: u64 = 2_600;
const KEY_ROW_UM: u64 = 4_000;
const KEY_SWATCH_UM: u64 = 6_000;
const KEY_LABEL_GAP_UM: u64 = 1_500;
const KEY_ENTRY_GAP_UM: u64 = 5_000;
/// The least a figure's frame can be either way and still be read.
const LEAST_FRAME_UM: u64 = 20_000;

impl SemanticSceneCompiler<'_> {
    /// `figure` printed in the box at `origin`, `size` across and down: its
    /// plot filling the box, and under the plot a key naming each trace
    /// beside the line it is drawn in. The figure is one primitive, so a
    /// tagged print describes all of it by its alternative text.
    pub(super) fn report_figure(
        &mut self,
        figure: &SemanticReportFigure,
        origin: ScenePoint,
        (width, height): (u64, u64),
    ) -> Result<(), HardcopyRenderError> {
        let kept = FrozenReportArtifact::new(figure.media_type.clone(), figure.payload.clone())
            .and_then(|artifact| ReportFigure::read(&artifact))
            .map_err(|error| {
                HardcopyRenderError::InvalidEmbeddedFigure(format!(
                    "report block {}: {error}",
                    figure.block_id
                ))
            })?;
        let labels = kept
            .traces
            .iter()
            .filter(|trace| !trace.paths.is_empty())
            .map(|trace| trace.label.as_str())
            .collect::<Vec<_>>();
        let inset = PLOT_INSET_UM.unsigned_abs();
        let plot_width = width.saturating_sub(inset + RIGHT_GUTTER_UM);
        let rows = key_rows(&labels, plot_width);
        let key = rows.len() as u64 * KEY_ROW_UM;
        let plot_height = height.saturating_sub(TOP_GUTTER_UM + FOOT_UM + key);
        if plot_width < LEAST_FRAME_UM || plot_height < LEAST_FRAME_UM {
            return Err(HardcopyRenderError::InvalidEmbeddedFigure(format!(
                "report block {}: its box has no room for its plot",
                figure.block_id
            )));
        }
        let signed = |value: u64| {
            i64::try_from(value).map_err(|_| conversion_error("report figure size overflow"))
        };
        let plot = report_figure_plot(
            &kept,
            PlotArea {
                plot_width: signed(plot_width)?,
                plot_height: signed(plot_height)?,
                foot: signed(FOOT_UM)?,
            },
        )
        .map_err(|error| conversion_error(error.to_string()))?;

        // The plot is laid out from its own top-left corner, with a page's
        // gutter above its frame; the box keeps only the gutter it needs.
        let top = PLOT_INSET_UM - signed(TOP_GUTTER_UM)?;
        let primitive_start = self.primitives.len();
        let legend_start = self.legend.len();
        let (bounds, offset) = (self.bounds, self.coordinate_offset);
        self.bounds = SemanticBounds::try_new(
            SemanticPoint::new(0, top),
            SemanticPoint::new(signed(width)?, top + signed(height)?),
        )
        .map_err(|error| conversion_error(error.to_string()))?;
        self.coordinate_offset = origin;
        let drawn = self.plot(&plot);
        self.bounds = bounds;
        self.coordinate_offset = offset;
        // The figure keys its own traces; the page's legend is not theirs.
        let entries = self.legend.split_off(legend_start);
        drawn?;

        let at = |x: u64, y: u64| {
            ScenePoint::new(Length::from_micrometres(x), Length::from_micrometres(y))
        };
        let left = origin.x.micrometres() + inset;
        let key_top = origin.y.micrometres() + height - key;
        for (row, placed) in rows.iter().enumerate() {
            let baseline = key_top + row as u64 * KEY_ROW_UM + KEY_TEXT_UM;
            // The swatch stands at the middle of the lettering's small letters.
            let swatch = baseline - KEY_TEXT_UM * 3 / 10;
            for &(index, offset) in placed {
                let Some(entry) = entries.iter().find(|entry| entry.label == labels[index]) else {
                    continue;
                };
                let x = left + offset;
                self.primitives.push(ScenePrimitive::Line {
                    from: at(x, swatch),
                    to: at(x + KEY_SWATCH_UM, swatch),
                    stroke: entry.stroke,
                });
                self.add_text(
                    at(x + KEY_SWATCH_UM + KEY_LABEL_GAP_UM, baseline),
                    labels[index],
                    SceneFont::Sans,
                    KEY_TEXT_UM,
                    SemanticColor::Foreground,
                )?;
            }
        }
        let primitives = self.primitives.split_off(primitive_start);
        self.primitives.push(ScenePrimitive::Figure {
            alternative_text: figure.alternative_text.clone(),
            primitives,
        });
        Ok(())
    }
}

/// The key's entries row by row: each label's index and its offset along
/// the row, packed from the left and wrapped at `measure`.
fn key_rows(labels: &[&str], measure: u64) -> Vec<Vec<(usize, u64)>> {
    let mut rows: Vec<Vec<(usize, u64)>> = Vec::new();
    let mut x = 0;
    for (index, label) in labels.iter().enumerate() {
        let width = KEY_SWATCH_UM + KEY_LABEL_GAP_UM + set_width(Role::Sans, label, KEY_TEXT_UM);
        if rows.is_empty() || (x > 0 && x + width > measure) {
            rows.push(Vec::new());
            x = 0;
        }
        if let Some(row) = rows.last_mut() {
            row.push((index, x));
        }
        x += width + KEY_ENTRY_GAP_UM;
    }
    rows
}
