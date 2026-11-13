//! Cursor values, branch readings and retained-sample measurements.

use super::cursor_values::{slope_text, x_delta, x_reciprocal};
use super::pane::SwatchStyle;
use super::{
    NOISE_DENSITY_UNIT, ReadoutPolicy, StripModel, StripTrace, TraceKind, cursor_interpolation,
    trace_key,
};
use crate::derived::{DerivedSeries, WindowStats};
use egui::Ui;
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::sample::{
    SweepClass, SweepShape, XOrientation, nearest_sample, sample_at_with_shape,
    sample_branches_into,
};
use rspice_ui_kit::plot::{
    CursorId, CursorPair, SampleInterpolation, TraceLook, XScale, paint_trace_swatch,
};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

pub const READOUT_ROW_H: f32 = 20.0;
pub const READOUT_PAD_X: f32 = 10.0;
const READOUT_SWATCH_W: f32 = 22.0;

/// How many branches of one trace the readout will name separately.
///
/// Two is the hysteresis loop this exists for and six covers a retrace with
/// a few passes. Past that the rows stop being a measurement and become a
/// listing, so the readout says how many branches there are instead of
/// pretending to report each one.
pub const MAX_READOUT_BRANCHES: usize = 6;

/// How one branch of a multi-branch sweep names itself.
///
/// Two branches are the ordinary loop and read as the direction they travel.
/// Past two the direction alone no longer identifies a branch, so its ordinal
/// leads. Plain ASCII throughout: the bundled faces carry no arrow glyph, and
/// a tofu box beside a measured value is worse than a word.
pub fn branch_tag(shape: &SweepShape, run: usize) -> String {
    let direction = match shape.runs().get(run).map(|run| run.orientation) {
        Some(XOrientation::Descending) => "rev",
        _ => "fwd",
    };
    if shape.branch_count() <= 2 {
        direction.to_owned()
    } else {
        format!("r{} {direction}", run + 1)
    }
}

/// Paint the selected strip's cursor table, including its accessible reading.
///
/// The A and B column heads say which cursor the arrow keys and the jumps
/// move, in that cursor's colour; a click on the other head hands them to
/// it, and is returned for the host to apply.
pub fn cursor_readout_table(
    ui: &mut Ui,
    model: &StripModel,
    cursors: CursorPair,
    stepping: CursorId,
    presentation: ReadoutPolicy,
    quantity_policy: QuantityPresentationPolicy,
    lane_rows: Vec<ReadoutRow>,
) -> Option<CursorId> {
    let rect = ui.available_rect_before_wrap();
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let a = cursors.a?;
    // Fixed columns keep focus/hover/value-length changes from shifting
    // neighboring fields. At compact widths the table scrolls horizontally.
    let name_column = (rect.width() * 0.24).clamp(150.0, 220.0);
    let value_column = ((rect.width() - name_column - READOUT_PAD_X * 2.0) / 4.0).max(108.0);
    let columns = [
        rect.left() + READOUT_PAD_X,
        rect.left() + READOUT_PAD_X + name_column,
        rect.left() + READOUT_PAD_X + name_column + value_column,
        rect.left() + READOUT_PAD_X + name_column + value_column * 2.0,
        rect.left() + READOUT_PAD_X + name_column + value_column * 3.0,
    ];
    // A trace row leads with its own line swatch, so a reader ties the row to
    // a curve without matching names; the header and X rows keep the bare
    // column so the table still reads as one grid.
    let draw_row = |row_index: usize,
                    values: [&str; 5],
                    colors: [egui::Color32; 5],
                    font: egui::FontId,
                    swatch: Option<(egui::Color32, Option<SwatchStyle>)>| {
        let top = rect.top() + row_index as f32 * READOUT_ROW_H;
        let row = egui::Rect::from_min_max(
            egui::pos2(rect.left(), top),
            egui::pos2(rect.right(), top + READOUT_ROW_H),
        );
        let painter = ui.painter().with_clip_rect(row);
        if let Some((color, style)) = swatch {
            let rect = egui::Rect::from_x_y_ranges(
                columns[0]..=columns[0] + READOUT_SWATCH_W - 4.0,
                row.y_range(),
            );
            let look = style.map_or(
                TraceLook {
                    dash: None,
                    marker: None,
                },
                |style| style.look(t.color_safe_traces),
            );
            paint_trace_swatch(&painter, rect, color, look, c.bg_panel);
        }
        for (index, ((text, x), color)) in values.into_iter().zip(columns).zip(colors).enumerate() {
            let x = if index == 0 && swatch.is_some() {
                x + READOUT_SWATCH_W
            } else {
                x
            };
            painter.text(
                egui::pos2(x, row.center().y),
                egui::Align2::LEFT_CENTER,
                text,
                font.clone(),
                color,
            );
        }
    };

    let head = |id: CursorId, color| if stepping == id { color } else { c.text_faint };
    draw_row(
        0,
        ["TRACE", "A", "B", "\u{0394}", "SLOPE"],
        [
            c.text_faint,
            head(CursorId::A, c.canvas_cursor_a),
            head(CursorId::B, c.canvas_cursor_b),
            c.text_faint,
            c.text_faint,
        ],
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        None,
    );
    let a_x = model.format_x(a, significant_digits, quantity_policy);
    let b_x = cursors
        .b
        .map(|b| model.format_x(b, significant_digits, quantity_policy))
        .unwrap_or_else(|| "Place cursor B".to_owned());
    let delta_x = cursors
        .b
        .map(|b| x_delta(model, a, b, significant_digits, quantity_policy))
        .unwrap_or_default();
    // The X row's last cell carries 1/Δx — the frequency of a period
    // measured between the cursors — and states the sweep's own shape: a
    // reader looking at two rows per signal is owed the reason there are two.
    let reciprocal = cursors
        .b
        .and_then(|b| x_reciprocal(model, a, b, significant_digits, quantity_policy))
        .map(|reciprocal| format!("1/\u{0394} {reciprocal}"));
    let x_note = [reciprocal.clone(), readout_branch_note(model)]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" \u{00b7} ");
    draw_row(
        1,
        [model.x_label(), &a_x, &b_x, &delta_x, &x_note],
        [c.text_dim, c.accent, c.text, c.text, c.text_dim],
        theme::mono(tokens::FS_0, FontWeight::Regular),
        None,
    );

    // Per-trace values at A and B, their difference, and their own slope;
    // then what each event lane reads at A and B.
    let mut rows = readout_rows(model, cursors, presentation, quantity_policy);
    rows.extend(lane_rows);
    for (index, row) in rows.iter().enumerate() {
        draw_row(
            index + 2,
            [&row.name, &row.a, &row.b, &row.delta, &row.slope],
            [c.text_dim, c.text, c.text, c.text_dim, c.text_dim],
            theme::mono(tokens::FS_0, FontWeight::Regular),
            Some((row.swatch, row.swatch_style)),
        );
    }

    // The table is painted, not built from widgets, so nothing about it
    // reaches a screen reader on its own. The marker half of the strip
    // already publishes its rows; this half stated the same numbers to a
    // sighted reader and nothing at all to anyone else.
    let response = ui.interact(rect, ui.id().with("cursor-readout"), egui::Sense::hover());
    let summary = readout_accessibility_text(
        model,
        cursors,
        &a_x,
        &b_x,
        &delta_x,
        reciprocal.as_deref(),
        &rows,
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, summary.clone()));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_label(summary.clone());
    });

    // The A and B heads hand the arrow keys and the jumps to their cursor.
    let mut chosen = None;
    for (id, left, right) in [
        (CursorId::A, columns[1], columns[2]),
        (CursorId::B, columns[2], columns[3]),
    ] {
        let head = egui::Rect::from_x_y_ranges(
            left..=right - 4.0,
            rect.top()..=rect.top() + READOUT_ROW_H,
        );
        let response = ui
            .interact(
                head,
                ui.id().with(("cursor-readout-head", id.letter())),
                egui::Sense::click(),
            )
            .on_hover_text(if stepping == id {
                format!(
                    "The arrow keys and the cursor jumps move cursor {}: arrows by sample, \
                     Shift by peak, Alt by crossing",
                    id.letter()
                )
            } else {
                format!(
                    "Click to move cursor {} with the arrow keys and the cursor jumps",
                    id.letter()
                )
            });
        if response.clicked() {
            chosen = Some(id);
        }
    }
    chosen
}

/// The A/B/Δ/slope table, spoken.
///
/// One string rather than a node per cell: the table is a single painted
/// grid, so there is no per-cell rect to attach a node to, and a reader who
/// asks what the cursors say wants the reading, not a walk of the widget.
fn readout_accessibility_text(
    model: &StripModel,
    cursors: CursorPair,
    a_x: &str,
    b_x: &str,
    delta_x: &str,
    reciprocal: Option<&str>,
    rows: &[ReadoutRow],
) -> String {
    let mut text = format!("Cursor readout. {} A {a_x}", model.x_label());
    if cursors.b.is_some() {
        text.push_str(&format!(", B {b_x}, \u{0394} {delta_x}"));
        if let Some(reciprocal) = reciprocal {
            text.push_str(&format!(", 1/\u{0394} {reciprocal}"));
        }
    } else {
        text.push_str(", cursor B not placed");
    }
    if let Some(note) = readout_branch_note(model) {
        text.push_str(&format!(", {note}"));
    }
    for row in rows {
        text.push_str(&format!(
            ". {}: A {}, B {}, \u{0394} {}, slope {}",
            row.name, row.a, row.b, row.delta, row.slope
        ));
    }
    text
}

/// Paint retained-sample measurements for the selected strip and cursor span.
pub fn measurement_panel(
    ui: &mut Ui,
    derived: &mut DerivedSeries,
    model: &StripModel,
    window: Option<(f64, f64)>,
    significant_digits: usize,
    quantity_policy: QuantityPresentationPolicy,
) {
    let title = if window.is_some() {
        "Measurements · A–B"
    } else {
        "Measurements"
    };
    // Linear interpolation between retained samples is how every measurement
    // in the product is taken; stating it on the header of every strip is a
    // constant, not a qualification.
    section_header(ui, title, None);
    measurement_rows(
        ui,
        derived,
        model,
        window,
        significant_digits,
        quantity_policy,
    );
}
/// The traces the readout lists, in row order.
///
/// Every per-row producer reads this one predicate, so the value, delta,
/// slope and swatch columns cannot fall out of step with each other.
pub fn readout_traces(model: &StripModel) -> impl Iterator<Item = &StripTrace> {
    model.traces.iter().filter(|trace| trace.visible)
}

/// Nothing was measured here.
pub const READOUT_ABSENT: &str = "—";

/// One row of the cursor readout: what a trace — or one branch of it — says
/// at A, at B, and between them.
///
/// A row is a branch rather than a trace because a sweep that turns around
/// has two answers at the same abscissa, and one composite cell holding both
/// is not a column of measurements any more. The overlay run a row belongs to
/// is named on the row for the same reason: two runs of the same signal
/// otherwise arrive as two rows with one name.
pub struct ReadoutRow {
    pub name: String,
    pub swatch: egui::Color32,
    /// The trace's dash and marker, for a swatch that is its curve; `None`
    /// for an event lane, whose swatch is its colour.
    pub swatch_style: Option<SwatchStyle>,
    pub a: String,
    pub b: String,
    pub delta: String,
    pub slope: String,
}

/// Every row the readout will draw for one strip, in draw order.
///
/// One producer, because the value, difference and slope columns were three
/// independent walks of the same trace list and only stayed aligned by
/// producing equal-length vectors — which stopped being true the moment one
/// trace owed the reader more than one row.
pub fn readout_rows(
    model: &StripModel,
    cursors: CursorPair,
    presentation: ReadoutPolicy,
    quantity_policy: QuantityPresentationPolicy,
) -> Vec<ReadoutRow> {
    readout_traces(model)
        .flat_map(|trace| trace_readout_rows(model, trace, cursors, presentation, quantity_policy))
        .collect()
}

/// The suffix that says which run a row's numbers came from.
///
/// The active run owns the signal's name, so only an overlay adds one. Two
/// runs of the same signal are otherwise two rows spelled identically, with
/// nothing in the table to say which is being compared against which.
fn run_tag(model: &StripModel, trace: &StripTrace) -> Option<String> {
    trace
        .overlay
        .then(|| format!("run {}", run_ordinal(model, trace)))
}

/// Which overlay run of the strip this trace belongs to, counting from one in
/// the order the strip drew them.
fn run_ordinal(model: &StripModel, trace: &StripTrace) -> usize {
    model
        .traces
        .iter()
        .filter(|candidate| candidate.overlay)
        .map(|candidate| candidate.run_id)
        .collect::<std::collections::BTreeSet<_>>()
        .iter()
        .position(|run_id| *run_id == trace.run_id)
        .map_or(1, |index| index + 1)
}

fn trace_readout_rows(
    model: &StripModel,
    trace: &StripTrace,
    cursors: CursorPair,
    presentation: ReadoutPolicy,
    quantity_policy: QuantityPresentationPolicy,
) -> Vec<ReadoutRow> {
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let shape = trace.shape.as_ref();
    let branches = shape.branch_count();
    let qualifiers = |branch: Option<usize>| -> String {
        let mut name = trace.name.clone();
        if let Some(branch) = branch {
            name.push(' ');
            name.push_str(&branch_tag(shape, branch));
        }
        if let Some(tag) = run_tag(model, trace) {
            name.push_str(" · ");
            name.push_str(&tag);
        }
        name
    };
    let format =
        |value: f64| model.format_trace_value(trace, value, significant_digits, quantity_policy);

    // Past the branch budget — or with no branch model at all — the honest
    // answer is the nearest retained sample, and the row says so rather than
    // presenting an interpolation of a curve that has no single value here.
    if branches > MAX_READOUT_BRANCHES || shape.class() == SweepClass::NonSweep {
        let approximate = |x: Option<f64>| {
            x.map_or_else(
                || READOUT_ABSENT.to_owned(),
                |x| format!("\u{2248} {}", format(nearest_sample(&trace.x, &trace.y, x))),
            )
        };
        return vec![ReadoutRow {
            name: qualifiers(None),
            swatch: trace.color,
            swatch_style: Some(SwatchStyle::of(trace)),
            a: approximate(cursors.a),
            b: approximate(cursors.b),
            delta: READOUT_ABSENT.to_owned(),
            slope: READOUT_ABSENT.to_owned(),
        }];
    }

    let sample = |x: f64, branch: Option<usize>| -> Option<f64> {
        match branch {
            None => Some(sample_at_with_shape(
                &trace.x,
                &trace.y,
                shape,
                x,
                interpolation,
            )),
            Some(branch) => {
                let mut out = Vec::new();
                sample_branches_into(&trace.x, &trace.y, shape, x, interpolation, &mut out);
                out.iter()
                    .find(|found| found.run == branch)
                    .map(|found| found.value)
            }
        }
    };
    let denominator = match (cursors.a, cursors.b) {
        (Some(a), Some(b)) => match model.x_scale {
            XScale::Log10 if a > 0.0 && b > 0.0 => b.log10() - a.log10(),
            XScale::Log10 => f64::NAN,
            XScale::Linear => b - a,
        },
        _ => f64::NAN,
    };

    let branch_list: Vec<Option<usize>> = if shape.is_monotone() || branches == 0 {
        vec![None]
    } else {
        (0..branches).map(Some).collect()
    };
    branch_list
        .into_iter()
        .map(|branch| {
            let at = |cursor: Option<f64>| cursor.and_then(|x| sample(x, branch));
            let (a_value, b_value) = (at(cursors.a), at(cursors.b));
            let difference = match (a_value, b_value) {
                (Some(a), Some(b)) => Some(b - a),
                _ => None,
            };
            let slope = difference
                .filter(|_| denominator.is_finite() && denominator != 0.0)
                .map(|difference| difference / denominator)
                .filter(|slope| slope.is_finite());
            ReadoutRow {
                name: qualifiers(branch),
                swatch: trace.color,
                swatch_style: Some(SwatchStyle::of(trace)),
                a: a_value.map_or_else(|| READOUT_ABSENT.to_owned(), &format),
                b: b_value.map_or_else(|| READOUT_ABSENT.to_owned(), &format),
                delta: difference.map_or_else(|| READOUT_ABSENT.to_owned(), &format),
                slope: slope.map_or_else(
                    || READOUT_ABSENT.to_owned(),
                    |slope| {
                        // A trace names its slope in its own unit, never the
                        // strip's: a current sharing a sheet with voltages must
                        // not report mA/ms of rise as volts. `trace_unit` is the
                        // one owner of that mapping.
                        let x_unit = match model.x_scale {
                            XScale::Log10 => "dec",
                            XScale::Linear if model.x_unit.is_empty() => "x",
                            XScale::Linear => model.x_unit.as_str(),
                        };
                        slope_text(slope, model.trace_unit(trace), x_unit, significant_digits)
                    },
                ),
            }
        })
        .collect()
}

/// What the X row says about the shape of the sweep it is reporting, when
/// that shape is not one pass.
pub fn readout_branch_note(model: &StripModel) -> Option<String> {
    let mut branches = 1usize;
    for trace in readout_traces(model) {
        if trace.shape.class() == SweepClass::NonSweep
            || trace.shape.branch_count() > MAX_READOUT_BRANCHES
        {
            return Some("multi-branch sweep".to_owned());
        }
        branches = branches.max(trace.shape.branch_count());
    }
    (branches > 1).then(|| format!("{branches} branches"))
}

/// Interval statistics are cached by source, cursor window, and sweep branch.
pub fn trace_interval_statistics(
    derived: &mut DerivedSeries,
    model: &StripModel,
    trace: &StripTrace,
    window: Option<(f64, f64)>,
    branch: Option<usize>,
) -> WindowStats {
    use rspice_results::measurements::{MeasurementError, measure_interval};

    if window.is_some_and(|(a, b)| !a.is_finite() || !b.is_finite()) {
        return Err(MeasurementError::NonFiniteWindow);
    }
    let (a_bits, b_bits) = window.map_or((u64::MAX, u64::MAX), |(a, b)| {
        (a.min(b).to_bits(), a.max(b).to_bits())
    });
    let key = (
        trace_key(model, trace),
        a_bits,
        b_bits,
        branch.unwrap_or(usize::MAX),
    );
    derived.stats_or(key, || {
        if trace.x.len() != trace.y.len() {
            return Err(MeasurementError::LengthMismatch);
        }
        if trace.x.iter().any(|v| !v.is_finite()) {
            return Err(MeasurementError::NonFiniteAxis);
        }
        let range = match branch {
            Some(index) => {
                let run = trace
                    .shape
                    .runs()
                    .get(index)
                    .ok_or(MeasurementError::NonMonotoneAxis)?;
                run.start..run.end
            }
            None => 0..trace.x.len(),
        };
        measure_interval(&trace.x[range.clone()], &trace.y[range], window)
    })
}

pub fn measurement_values(
    derived: &mut DerivedSeries,
    model: &StripModel,
    window: Option<(f64, f64)>,
    significant_digits: usize,
    quantity_policy: QuantityPresentationPolicy,
) -> Vec<(String, String)> {
    let mut rows = Vec::new();
    for trace in readout_traces(model) {
        let branches: Vec<Option<usize>> = if trace.shape.class() == SweepClass::MultiBranch
            && trace.shape.branch_count() <= MAX_READOUT_BRANCHES
        {
            (0..trace.shape.branch_count()).map(Some).collect()
        } else {
            vec![None]
        };
        for branch in branches {
            let mut name = trace.name.clone();
            if let Some(branch) = branch {
                name.push(' ');
                name.push_str(&branch_tag(&trace.shape, branch));
            }
            if let Some(tag) = run_tag(model, trace) {
                name.push_str(" · ");
                name.push_str(&tag);
            }
            let stats = trace_interval_statistics(derived, model, trace, window, branch);
            let stats = match stats {
                Ok(stats) => stats,
                Err(error) => {
                    rows.push((name, format!("Unavailable: {error}")));
                    continue;
                }
            };
            let fmt = |v| model.format_trace_value(trace, v, significant_digits, quantity_policy);
            rows.push((format!("{name} min"), fmt(stats.min)));
            rows.push((format!("{name} max"), fmt(stats.max)));
            if matches!(
                trace.kind,
                TraceKind::Value | TraceKind::Real | TraceKind::Imaginary
            ) {
                rows.push((format!("{name} rms"), fmt(stats.rms)));
            }
        }
    }
    // An arbitrary cap the reader had to work around by hiding traces is not
    // a measurement, so the row that announced it goes.
    rows
}

/// Min/max/RMS use retained linear segments, including interpolated A/B
/// endpoints. Rendering interpolation and display decimation do not alter them.
fn measurement_rows(
    ui: &mut Ui,
    derived: &mut DerivedSeries,
    model: &StripModel,
    window: Option<(f64, f64)>,
    significant_digits: usize,
    quantity_policy: QuantityPresentationPolicy,
) {
    let rows = measurement_values(derived, model, window, significant_digits, quantity_policy);
    let refs: Vec<(&str, &str)> = rows.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    rspice_ui_kit::widgets::measurement_table(ui, &refs);
}

/// Append the exact cursor values using the explicit copied-value policy.
pub fn append_copied_cursor(
    target: &mut String,
    cursor: &str,
    x: f64,
    model: &StripModel,
    interpolation: SampleInterpolation,
    policy: QuantityPresentationPolicy,
) {
    use std::fmt::Write as _;

    let copied_x = if model.x_unit == "Hz" {
        policy.copy_frequency(x)
    } else {
        policy.copy_si_value(x, &model.x_unit)
    };
    let _ = writeln!(
        target,
        "{cursor} {} = {}",
        model.x_label(),
        copied_x.trim_end()
    );
    for trace in readout_traces(model) {
        // The copy has to say what the table says. Read unshaped, a loop's
        // line pasted a value off the far side of its turnaround while the
        // register on screen reported each branch.
        let value = sample_at_with_shape(&trace.x, &trace.y, &trace.shape, x, interpolation);
        let copied = copied_trace_value(policy, trace.kind, value, model.trace_unit(trace));
        let _ = writeln!(target, "{} = {}", trace.name, copied.trim_end());
    }
    while target.ends_with('\n') {
        target.pop();
    }
}

/// One trace's value as Copied values writes it. The unit is the trace's
/// own, not the strip's: copying a supply current off a sheet it shares
/// with node voltages must not paste milliamps as millivolts.
fn copied_trace_value(
    policy: QuantityPresentationPolicy,
    kind: TraceKind,
    value: f64,
    unit: &str,
) -> String {
    match kind {
        TraceKind::PhaseDeg => policy.copy_angle_degrees(value),
        TraceKind::NoiseDensity => policy.copy_scaled_unit_value(value, NOISE_DENSITY_UNIT),
        TraceKind::MagnitudeDb | TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => {
            policy.copy_si_value(value, unit)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_vocabulary::quantity::{AngleDisplay, CopiedValueFormat};

    /// A phase trace holds degrees. Its copy went through radians and back,
    /// which landed a unit in the last place off for about one degree value
    /// in eight, so the pasted phase was not the one on the trace.
    #[test]
    fn a_copied_phase_reads_back_as_the_degrees_on_the_trace() {
        let policy = QuantityPresentationPolicy {
            copied_value_format: CopiedValueFormat::EngineeringNotationWithUnit,
            angle_display: AngleDisplay::Degrees,
            ..QuantityPresentationPolicy::default()
        };
        let mut degrees = (-72_000..=72_000)
            .map(|step| f64::from(step) * 0.01)
            .collect::<Vec<_>>();
        degrees.extend([1.0 / 3.0, 0.1, 5e-324, f64::MIN_POSITIVE, 1e300]);
        for exponent in -300..=300 {
            for mantissa in [1.0, 1.234_567_890_123_456_7, 3.2, 9.999_999_999_999_998] {
                let value = mantissa * 10_f64.powi(exponent);
                degrees.extend([value, -value]);
            }
        }
        for value in degrees {
            let copied = copied_trace_value(policy, TraceKind::PhaseDeg, value, "°");
            let (number, unit) = copied.split_once(' ').expect("a value and its unit");
            assert_eq!(unit, "°");
            let read_back: f64 = number.parse().expect("the copy parses as a number");
            assert_eq!(
                read_back.to_bits(),
                value.to_bits(),
                "{value:e} copied as {copied}"
            );
        }
    }
}
