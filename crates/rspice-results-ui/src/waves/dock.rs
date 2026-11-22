//! The cursor readout: what the cursors and markers currently say.
//!
//! Every number here is read from the sample the cursor is actually on, and a
//! value that would require interpolating between samples is reported under
//! the strip's declared interpolation policy rather than silently computed.
//! Deltas and slopes are shown only when both cursors sit on the same strip,
//! because a difference across two domains is not a measurement.

use super::cursor_table::{self, CursorTable};
use super::cursor_values::x_separation;
use super::readout::readout_rows;
use super::{ReadoutPolicy, StripModel};
use crate::session::{HorizontalCursors, MarkerSelector, MarkerView, ResultViewerState};
use egui::Ui;
use rspice_results::result_presentation::{
    AnalysisPresentationKey, MarkerKind, WavePanePresentationKey,
};
use rspice_ui_kit::plot::CursorPair;
use rspice_ui_kit::tokens::Tokens;
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

/// Borrowed projections selected and validated by the host for this readout.
#[derive(Clone, Copy)]
pub struct ReadoutInput<'a> {
    pub models: &'a std::sync::Arc<Vec<StripModel>>,
    /// Which build of the strip models `models` is: what is measured of them
    /// is kept until it changes.
    pub models_generation: u64,
    pub on_screen_strips: &'a [AnalysisPresentationKey],
    pub presentation: ReadoutPolicy,
    pub quantity_policy: QuantityPresentationPolicy,
    /// How the host shows each row's name.
    pub display_name: super::DisplayName<'a>,
}

/// Source-bound marker requests. The host applies removal before opening an edit.
#[derive(Default)]
pub struct MarkerActions {
    pub remove: Option<MarkerSelector>,
    pub edit: Option<MarkerSelector>,
    /// A marker moved to a typed or snapped X.
    pub moved: Option<MarkerMove>,
}

/// A marker moved to `x`; a saved view, which keeps a marker on a retained
/// sample, takes the nearest one of the trace it rides.
#[derive(Clone, Copy, Debug)]
pub struct MarkerMove {
    pub selector: MarkerSelector,
    pub x: f64,
    pub nearest_sample: Option<f64>,
}

/// Rows the readout strip will report for the cursor's strip.
///
/// The count comes from the projection the strip actually draws, not from the
/// retained waveform list: a run overlay, a real/imaginary split, a corner
/// family and a sweep that turns around each put more rows on the table than
/// there are retained waveforms, and the band was sized for the smaller
/// number and then scrolled. The same walk answers the opposite case — a
/// viewer whose projection drops the cursor's strip entirely reserved rows
/// nothing could fill.
pub fn readout_row_count(state: &mut ResultViewerState, source: ReadoutInput<'_>) -> usize {
    let Some(index) = state.cursor_strip else {
        return 0;
    };
    let presentation = source.presentation;
    let quantity_policy = source.quantity_policy;
    let cursors = state.cursors;
    let models = source.models;
    models
        .iter()
        .find(|model| model.analysis_index == index)
        .map_or(0, |model| {
            readout_rows(
                model,
                cursors,
                presentation,
                quantity_policy,
                source.display_name,
            )
            .len()
                + strip_lane_rows(state, model, cursors, presentation, source.display_name).len()
        })
}

/// The cursor table's rows for the strip's logic, bus and `E()` lanes, read
/// from their exact event histories.
pub(crate) fn strip_lane_rows(
    state: &mut ResultViewerState,
    model: &StripModel,
    cursors: CursorPair,
    presentation: ReadoutPolicy,
    display_name: super::DisplayName<'_>,
) -> Vec<super::readout::ReadoutRow> {
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let Some(lanes) = super::lanes::lanes_at_cursors(model, cursors) else {
        return Vec::new();
    };
    let specs = super::lanes::lane_specs(model, state.event_lanes.get(&model.analysis_key));
    let user = super::lanes::user_bus_lanes(
        &mut state.derived,
        model,
        &lanes,
        &specs,
        super::lanes::BusReader::Cursors,
    );
    super::lanes::lane_readout_rows(model, &lanes, &specs, &user, cursors, digits, display_name)
}

/// Kind owns the marker's colour: a spec limit reads as a bound to meet,
/// a peak as a called-out feature, a note as neutral annotation.
pub fn marker_color(kind: MarkerKind, t: &Tokens) -> egui::Color32 {
    match kind {
        MarkerKind::Note => t.color.text,
        MarkerKind::Peak | MarkerKind::Valley | MarkerKind::Crossing(_) => t.color.accent,
        MarkerKind::Spec => t.color.warn,
    }
}

/// Tag text: the id always, the note only when the user wrote one.
pub fn marker_label(marker: MarkerView<'_>) -> String {
    let id = marker.display_id();
    let note = marker.note().trim();
    if note.is_empty() {
        id
    } else {
        format!("{id} · {note}")
    }
}

/// Markers the strip will list, in placement order, across both stores.
///
/// A persistent pane's retained markers are listed here beside the dataset's
/// quick markers so the reader has exactly one marker list, not one per owner.
pub fn visible_markers<'a>(
    state: &'a ResultViewerState,
    source: ReadoutInput<'_>,
) -> Vec<MarkerView<'a>> {
    source
        .on_screen_strips
        .iter()
        .copied()
        .flat_map(|analysis| state.strip_markers(analysis))
        .collect()
}

/// Whether the cursor has a strip on this sheet to report about.
///
/// "On this sheet" is the operative half: the retained analysis surviving is
/// not enough, because switching viewers reprojects the stack and can leave
/// the cursor pointing at a strip the sheet no longer draws. The band was
/// still reserved for it, and the table it opened for drew nothing.
fn cursor_target_available(state: &mut ResultViewerState, source: ReadoutInput<'_>) -> bool {
    if !state.cursor_readout_active() {
        return false;
    }
    let Some(index) = state.cursor_strip else {
        return false;
    };
    let models = source.models;
    models.iter().any(|model| model.analysis_index == index)
}

/// What the dock's Cursors page draws, below its toolbar band: `tables`
/// (see [`cursor_tables`]), then the horizontal cursors, scrolled when
/// taller than the page. `None` when there is nothing to read, no cursor on
/// a strip of the sheet and no marker, and the page says how to place one
/// instead.
pub fn cursors_page(
    ui: &mut Ui,
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    tables: &[CursorTable],
) -> Option<MarkerActions> {
    if !page_has_readings(state, source) {
        return None;
    }
    let mut actions = MarkerActions::default();
    egui::ScrollArea::vertical()
        .id_salt("rspice.results.cursors")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            actions = cursor_table::paint_page(ui, state, source, tables);
        });
    Some(actions)
}

/// The Cursors page's tables: one per X its cursors and markers read.
pub fn cursor_tables(
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    t: &Tokens,
) -> Vec<CursorTable> {
    cursor_table::cursor_tables(state, source, t)
}

/// Whether the Cursors page has anything to read: a cursor on a strip of
/// the sheet, or a marker on a strip on screen.
pub fn page_has_readings(state: &mut ResultViewerState, source: ReadoutInput<'_>) -> bool {
    cursor_target_available(state, source)
        || !visible_markers(state, source).is_empty()
        || !horizontal_cursor_rows(state, source).is_empty()
}

/// The strip the Cursors page reads, as its header names it: the strip the
/// cursors stand on, else the first on screen with a marker.
pub fn followed_strip(state: &mut ResultViewerState, source: ReadoutInput<'_>) -> Option<String> {
    let models = source.models;
    let followed = state
        .cursor_strip
        .filter(|_| cursor_target_available(state, source))
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))
        .or_else(|| {
            models.iter().find(|model| {
                source.on_screen_strips.contains(&model.analysis_key)
                    && state.strip_markers(model.analysis_key).next().is_some()
            })
        })?;
    Some(
        if followed.subtitle.is_empty() || followed.subtitle == followed.kind_tag {
            followed.kind_tag.clone()
        } else {
            format!("{} · {}", followed.kind_tag, followed.subtitle)
        },
    )
}

/// The instrument bar's inline `A · B · Δ`, while the dock's Cursors page
/// is not shown.
///
/// The host asks only then: the page is the register that owns these three
/// numbers, and the bar states them while it is away — never both at once,
/// which is the duplication the results de-duplication pass removed.
pub fn inline_cursor_readout(
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
) -> Option<String> {
    if !state.cursor_readout_active() {
        return None;
    }
    let presentation = source.presentation;
    let quantity_policy = source.quantity_policy;
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = source.models;
    let model = state
        .cursor_strip
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))?;
    let cursors = state.cursors;
    let a = cursors.a?;
    let a_text = model.format_x(a, significant_digits, quantity_policy);
    let Some(b) = cursors.b else {
        return Some(format!("A {a_text}"));
    };
    Some(format!(
        "A {a_text} · B {} · \u{0394} {}",
        model.format_x(b, significant_digits, quantity_policy),
        x_separation(model, a, b, significant_digits, quantity_policy),
    ))
}

/// The panes of the on-screen strips that have a horizontal cursor, with
/// their cursors, in strip and unit order.
pub fn horizontal_cursor_rows(
    state: &ResultViewerState,
    source: ReadoutInput<'_>,
) -> Vec<(WavePanePresentationKey, HorizontalCursors)> {
    let mut rows: Vec<_> = state
        .horizontal_cursors
        .iter()
        .filter(|(pane, cursors)| {
            cursors.iter().any(Option::is_some) && source.on_screen_strips.contains(&pane.analysis)
        })
        .map(|(pane, cursors)| (pane.clone(), *cursors))
        .collect();
    rows.sort_by(|(a, _), (b, _)| {
        let strip = |pane: &WavePanePresentationKey| {
            source
                .on_screen_strips
                .iter()
                .position(|analysis| *analysis == pane.analysis)
        };
        strip(a).cmp(&strip(b)).then_with(|| a.unit.cmp(&b.unit))
    });
    rows
}

/// How a horizontal cursor row names its pane: the unit, and the strip when
/// more than one strip is on the sheet.
pub(crate) fn horizontal_pane_name(
    models: &[StripModel],
    pane: &WavePanePresentationKey,
) -> String {
    let model = models
        .iter()
        .find(|model| model.analysis_key == pane.analysis);
    let unit = model.map_or_else(
        || pane.unit.clone(),
        |model| model.pane_axis_unit(&pane.unit),
    );
    let unit = if unit.is_empty() {
        crate::waves::pane::NO_UNIT
    } else {
        unit.as_str()
    };
    match model {
        Some(model) if models.len() > 1 => format!("{} {unit}", model.kind_tag),
        _ => format!("{unit} pane"),
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Window statistics over the cursor span.
///
/// The A/B/Δ readout itself lives in the stage's readout strip; repeating it
/// one panel away is what the results de-duplication pass removed.
pub fn right_panel(ui: &mut Ui, state: &mut ResultViewerState, source: ReadoutInput<'_>) {
    let presentation = source.presentation;
    let quantity_policy = source.quantity_policy;
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = source.models;
    let cursor_model = state
        .cursor_strip
        .and_then(|index| models.iter().position(|m| m.analysis_index == index));

    // Statistics over the cursor window (or the full range).
    let cursors = state.cursors;
    let measured_model = cursor_model.or_else(|| (!models.is_empty()).then_some(0));
    if let Some(measured) = measured_model {
        let window = match (cursors.a, cursors.b) {
            (Some(a), Some(b)) => Some((a.min(b), a.max(b))),
            _ => None,
        };
        super::readout::measurement_panel(
            ui,
            &mut state.derived,
            models,
            measured,
            source.models_generation,
            window,
            super::readout::MeasurementWording {
                significant_digits,
                quantity_policy,
                display_name: source.display_name,
            },
        );
    }
}

#[cfg(test)]
mod tests;
