//! The Cursors page's table.
//!
//! A kit table with one column per vertical cursor or marker, named in its
//! head; the first row reads each one's X, typed in place to move it, and a
//! right-click in a column offers its menu. A difference column follows
//! for each other column against the base, and a pair adds its slope. Every
//! strip on screen that shares the columns' X has a band of rows, one per
//! trace with its swatch. A marker that rides a trace also reads that
//! trace's Y in a row of its own, so the difference of two markers on any
//! traces reads dx, dy, the slope and 1/dx, as a delta marker does in ADS.
//! Cursors and markers on strips with another X make a table of their own
//! below, as a difference across two domains is not a measurement.
//!
//! The horizontal cursors follow in a table of their own: a group per pane
//! naming its levels, and where each of its traces crosses them.

use super::cursor_values::{marker_x, slope_text, x_delta, x_reciprocal};
use super::dock::{MarkerActions, MarkerMove, ReadoutInput, marker_color, visible_markers};
use super::readout::{
    READOUT_ABSENT, ReadingLayout, TableRow, reading_resolution, readout_branch_note,
    readout_traces, settled, trace_table_rows,
};
use super::{StripModel, StripTrace, anchor_key};
use crate::session::{HorizontalCursors, MarkerSelector, ReadingColumnId, ResultViewerState};
use egui::Ui;
use rspice_results::result_presentation::{
    AnalysisPresentationKey, MarkerKind, WavePanePresentationKey, WaveformPresentationKey,
};
use rspice_results::waveform::SharedWaveformValues;
use rspice_ui_kit::plot::{CursorId, CursorPair, XScale};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::table::{
    CELL_PAD, Cell, CellEditor, Column, DataTable, EditOutcome, Group, TableState,
};

/// The narrowest a value column is drawn.
const VALUE_COLUMN_MIN_W: f32 = 72.0;
/// The tab of a trace's colour leading its row.
const SWATCH_W: f32 = 10.0;
/// The widest a crossings cell is drawn; its count leads, so what the clip
/// hides is still counted.
const CROSSINGS_MAX_W: f32 = 420.0;

/// One vertical reading of the table: a cursor or a marker, and where it
/// stands.
pub struct ReadingColumn {
    pub id: ReadingColumnId,
    /// What its head says: A, B, or the marker's id.
    pub name: String,
    /// What its head says on hover.
    pub hover: String,
    pub color: egui::Color32,
    /// The strip it was placed on, by its place in the source's models.
    pub model: usize,
    /// Where it stands, or why it stands nowhere: a search that finds
    /// nothing there.
    pub x: Result<f64, String>,
    /// The trace it rides, whose samples it snaps to.
    pub anchor: Option<WaveformPresentationKey>,
    /// Whether it is a point on that trace, which reads the trace's Y where
    /// it stands; a cursor or a vertical marker reads every trace instead.
    pub point: bool,
}

/// One strip's rows, under the band that names the strip.
pub struct TableBand {
    pub title: String,
    pub rows: Vec<TableRow>,
}

/// The cursors and markers that read one X, and every strip on screen
/// that has it.
pub struct CursorTable {
    /// The X quantity, as the strip's axis names it.
    pub x_label: String,
    pub columns: Vec<ReadingColumn>,
    /// The column every difference is taken from.
    pub base: usize,
    /// The columns read against the base.
    pub deltas: Vec<usize>,
    /// Exactly two columns: their difference has a slope, and their
    /// separation a reciprocal.
    pub pair: bool,
    /// The head's second row: each column's X, each difference's Δx, then
    /// the pair's 1/Δx over its slope.
    pub x_cells: Vec<String>,
    pub bands: Vec<TableBand>,
}

impl CursorTable {
    /// The head's first row: each column's name, each difference named by
    /// the two columns it subtracts, then the pair's slope.
    pub fn heads(&self) -> Vec<String> {
        let base = &self.columns[self.base].name;
        let mut heads: Vec<String> = self
            .columns
            .iter()
            .map(|column| column.name.clone())
            .collect();
        heads.extend(
            self.deltas
                .iter()
                .map(|&column| format!("{}-{base}", self.columns[column].name)),
        );
        if self.pair {
            heads.push("Slope".to_owned());
        }
        heads
    }

    /// The table as tab-separated text, as it pastes into a spreadsheet:
    /// the heads, the X row, then each band's title and rows.
    pub fn tsv(&self) -> String {
        let mut lines = vec![
            std::iter::once("Trace".to_owned())
                .chain(self.heads())
                .collect::<Vec<_>>()
                .join("\t"),
            std::iter::once(self.x_label.clone())
                .chain(self.x_cells.iter().cloned())
                .collect::<Vec<_>>()
                .join("\t"),
        ];
        for band in &self.bands {
            lines.push(band.title.clone());
            for row in &band.rows {
                lines.push(
                    std::iter::once(row.name.clone())
                        .chain(row.cells.iter().cloned())
                        .collect::<Vec<_>>()
                        .join("\t"),
                );
            }
        }
        lines.join("\n")
    }
}

/// Which X a strip reads: its quantity, unit and scale.
fn domain(model: &StripModel) -> (String, String, bool) {
    (
        model.x_label().to_owned(),
        model.x_unit.clone(),
        matches!(model.x_scale, XScale::Log10),
    )
}

/// The table's columns: A and B on the strip the cursors stand on, then
/// every marker on screen, in placement order.
pub fn reading_columns(
    state: &ResultViewerState,
    source: ReadoutInput<'_>,
    t: &Tokens,
) -> Vec<ReadingColumn> {
    let models = source.models;
    let mut columns = Vec::new();
    let cursor_model = state
        .cursor_strip
        .filter(|_| state.cursor_readout_active())
        .and_then(|index| {
            models
                .iter()
                .position(|model| model.analysis_index == index)
        });
    if let Some(model) = cursor_model {
        for (id, color, anchor) in [
            (
                CursorId::A,
                t.color.canvas_cursor_a,
                state.cursor_a_anchor.clone(),
            ),
            (CursorId::B, t.color.canvas_cursor_b, None),
        ] {
            let Some(x) = state.cursors.get(id) else {
                continue;
            };
            columns.push(ReadingColumn {
                id: ReadingColumnId::Cursor(id),
                name: id.letter().to_string(),
                hover: format!(
                    "Cursor {}. Click to move it with the arrow keys; right-click to make it \
                     the base, snap or delete it. Click its X to type where it stands.",
                    id.letter()
                ),
                color,
                model,
                x: Ok(x),
                anchor,
                point: false,
            });
        }
    }
    for marker in visible_markers(state, source) {
        let Some(model) = models
            .iter()
            .position(|model| model.analysis_key == marker.analysis())
        else {
            continue;
        };
        let kind = marker.kind();
        let x = marker_x(&models[model], state, marker)
            .map_err(|_| format!("no {} here", kind.label()));
        let note = marker.note().trim();
        let mut hover = format!("{} · {}", marker.display_id(), kind.label());
        if kind.rides_a_trace() {
            hover.push_str(&format!(" on {}", marker.trace_name()));
        }
        if !note.is_empty() {
            hover.push_str(&format!(" · {note}"));
        }
        hover.push_str(
            ". Right-click to make it the base, snap, rename or delete it. Click its X to type \
             where it stands.",
        );
        columns.push(ReadingColumn {
            id: ReadingColumnId::Marker(marker.selector()),
            name: marker.display_id(),
            hover,
            color: marker_color(kind, t),
            model,
            x,
            anchor: Some(marker.anchor().clone()),
            point: kind.rides_a_trace(),
        });
    }
    columns
}

/// Every table the page draws: one per X the columns read, the cursors'
/// first.
pub fn cursor_tables(
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    t: &Tokens,
) -> Vec<CursorTable> {
    let models = source.models;
    let mut groups: Vec<((String, String, bool), Vec<ReadingColumn>)> = Vec::new();
    for column in reading_columns(state, source, t) {
        let key = domain(&models[column.model]);
        match groups.iter_mut().find(|(domain, _)| *domain == key) {
            Some((_, columns)) => columns.push(column),
            None => groups.push((key, vec![column])),
        }
    }
    groups
        .into_iter()
        .map(|(key, columns)| build_table(state, source, &key, columns))
        .collect()
}

fn build_table(
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    key: &(String, String, bool),
    columns: Vec<ReadingColumn>,
) -> CursorTable {
    let models = source.models;
    let presentation = source.presentation;
    let quantity_policy = source.quantity_policy;
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let base = state
        .readout_base
        .and_then(|base| columns.iter().position(|column| column.id == base))
        .unwrap_or(0);
    let deltas: Vec<usize> = (0..columns.len()).filter(|&index| index != base).collect();
    let pair = columns.len() == 2;
    let positions: Vec<Option<f64>> = columns.iter().map(|column| column.x.clone().ok()).collect();
    let domain_model = &models[columns[base].model];

    let mut x_cells: Vec<String> = columns
        .iter()
        .map(|column| match &column.x {
            Ok(x) => models[column.model].format_x(*x, digits, quantity_policy),
            Err(why) => why.clone(),
        })
        .collect();
    let base_x = positions[base];
    for &column in &deltas {
        x_cells.push(match (base_x, positions[column]) {
            (Some(from), Some(to)) => x_delta(domain_model, from, to, digits, quantity_policy),
            _ => READOUT_ABSENT.to_owned(),
        });
    }
    if pair {
        let reciprocal = match (base_x, positions[deltas[0]]) {
            (Some(from), Some(to)) => x_reciprocal(domain_model, from, to, digits, quantity_policy),
            _ => None,
        };
        x_cells.push(reciprocal.map_or_else(
            || READOUT_ABSENT.to_owned(),
            |reciprocal| format!("1/\u{0394}x {reciprocal}"),
        ));
    }

    let layout = ReadingLayout {
        positions: &positions,
        base,
        deltas: &deltas,
        pair,
    };
    let cursor_model = columns
        .iter()
        .find(|column| matches!(column.id, ReadingColumnId::Cursor(_)))
        .map(|column| column.model);
    let mut bands: Vec<TableBand> = point_row(state, source, &columns, &layout)
        .map(|row| TableBand {
            title: "Markers".to_owned(),
            rows: vec![row],
        })
        .into_iter()
        .collect();
    for (index, model) in models.iter().enumerate() {
        let shown =
            source.on_screen_strips.contains(&model.analysis_key) || cursor_model == Some(index);
        if !shown || domain(model) != *key {
            continue;
        }
        let mut rows = Vec::new();
        for trace in readout_traces(model) {
            let trace_index = model
                .traces
                .iter()
                .position(|candidate| std::ptr::eq(candidate, trace));
            let extent = trace_index.and_then(|trace_index| {
                super::stack::pane_trace_extremes(
                    &mut state.derived,
                    model,
                    &[trace_index],
                    None,
                    |_| {},
                )
                .range()
            });
            rows.extend(trace_table_rows(
                model,
                trace,
                &layout,
                (presentation, quantity_policy, extent),
                source.display_name,
            ));
        }
        rows.extend(lane_table_rows(state, source, model, &layout));
        let mut title = if model.subtitle.is_empty() || model.subtitle == model.kind_tag {
            model.kind_tag.clone()
        } else {
            format!("{} · {}", model.kind_tag, model.subtitle)
        };
        if let Some(note) = readout_branch_note(model) {
            title.push_str(&format!(" · {note}"));
        }
        bands.push(TableBand { title, rows });
    }
    CursorTable {
        x_label: key.0.clone(),
        columns,
        base,
        deltas,
        pair,
        x_cells,
        bands,
    }
}

/// The row that reads each point marker's Y on the trace it rides, with
/// each difference against the base taken across traces and, for a pair,
/// its slope: what a delta marker reads. `None` when no column is a point.
/// Each reading and difference is settled as the trace's own row settles
/// it, so the two rows read one point alike.
fn point_row(
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    columns: &[ReadingColumn],
    layout: &ReadingLayout<'_>,
) -> Option<TableRow> {
    let models = source.models;
    let digits = usize::from(source.presentation.displayed_significant_digits().get());
    let interpolation = super::cursor_interpolation(source.presentation.cursor_interpolation());
    // Each point's model, trace, settled reading and the trace's resolution.
    let points: Vec<Option<(&StripModel, &StripTrace, f64, f64)>> = columns
        .iter()
        .zip(layout.positions)
        .map(|(column, x)| {
            let anchor = column.anchor.as_ref().filter(|_| column.point)?;
            let x = (*x)?;
            let model = &models[column.model];
            let index = model
                .traces
                .iter()
                .position(|trace| !trace.overlay && anchor_key(model, trace) == *anchor)?;
            let trace = &model.traces[index];
            let resolution = super::stack::pane_trace_extremes(
                &mut state.derived,
                model,
                &[index],
                None,
                |_| {},
            )
            .range()
            .map_or(0.0, |extent| reading_resolution(extent, digits));
            let value = settled(model.trace_value(trace, x, interpolation), resolution);
            Some((model, trace, value, resolution))
        })
        .collect();
    if points.iter().all(Option::is_none) {
        return None;
    }
    let absent = || READOUT_ABSENT.to_owned();
    let format = |(model, trace, value, _): (&StripModel, &StripTrace, f64, f64)| {
        model.format_trace_value(trace, value, digits, source.quantity_policy)
    };
    let mut cells: Vec<String> = points
        .iter()
        .map(|point| point.map_or_else(absent, format))
        .collect();
    let base = points[layout.base];
    let mut first = None;
    for (order, &column) in layout.deltas.iter().enumerate() {
        // Volts less amps is not a difference.
        let difference = match (base, points[column]) {
            (Some(from), Some(to)) if from.0.trace_unit(from.1) == to.0.trace_unit(to.1) => {
                let resolution = from.3.max(to.3);
                Some((
                    from.0,
                    from.1,
                    settled(to.2 - from.2, resolution),
                    resolution,
                ))
            }
            _ => None,
        };
        if order == 0 {
            first = difference;
        }
        cells.push(difference.map_or_else(absent, format));
    }
    if layout.pair {
        let run = match (
            layout.positions[layout.base],
            layout.positions[layout.deltas[0]],
        ) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        };
        let slope = first.zip(run).and_then(|((model, trace, dy, _), (a, b))| {
            let (dx, x_unit) = match model.x_scale {
                XScale::Log10 if a > 0.0 && b > 0.0 => (b.log10() - a.log10(), "dec"),
                XScale::Log10 => return None,
                XScale::Linear if model.x_unit.is_empty() => (b - a, "x"),
                XScale::Linear => (b - a, model.x_unit.as_str()),
            };
            let slope = dy / dx;
            slope
                .is_finite()
                .then(|| slope_text(slope, model.trace_unit(trace), x_unit, digits))
        });
        cells.push(slope.unwrap_or_else(absent));
    }
    Some(TableRow {
        name: "Y on its trace".to_owned(),
        swatch: egui::Color32::TRANSPARENT,
        swatch_style: None,
        cells,
    })
}

/// The table's rows for a strip's logic, bus and `E()` lanes: what each
/// reads at every column, and the pair's difference and slope as the lanes
/// state them between two cursors.
fn lane_table_rows(
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    model: &StripModel,
    layout: &ReadingLayout<'_>,
) -> Vec<TableRow> {
    let read = |state: &mut ResultViewerState, cursors: CursorPair| {
        super::dock::strip_lane_rows(
            state,
            model,
            cursors,
            source.presentation,
            source.display_name,
        )
    };
    let at: Vec<_> = layout
        .positions
        .iter()
        .map(|&x| read(state, CursorPair { a: x, b: None }))
        .collect();
    let between = layout.pair.then(|| {
        read(
            state,
            CursorPair {
                a: layout.positions[layout.base],
                b: layout.positions[layout.deltas[0]],
            },
        )
    });
    let Some(first) = at.first() else {
        return Vec::new();
    };
    (0..first.len())
        .map(|row| {
            let mut cells: Vec<String> = at
                .iter()
                .map(|rows| {
                    rows.get(row)
                        .map_or_else(|| READOUT_ABSENT.to_owned(), |found| found.a.clone())
                })
                .collect();
            match between.as_ref().and_then(|rows| rows.get(row)) {
                Some(found) => cells.extend([found.delta.clone(), found.slope.clone()]),
                None => cells.extend(
                    std::iter::repeat_with(|| READOUT_ABSENT.to_owned())
                        .take(layout.deltas.len() + usize::from(layout.pair)),
                ),
            }
            TableRow {
                name: first[row].name.clone(),
                swatch: first[row].swatch,
                swatch_style: first[row].swatch_style,
                cells,
            }
        })
        .collect()
}

/// Read a typed X: a number with an SI prefix, as the table prints it
/// ("10 ns", "1.5 MHz", "2.2u"), the axis' unit allowed after it. `M` is
/// mega and `m` milli, as printed; `meg` is mega too.
pub fn parse_typed_x(text: &str, unit: &str) -> Option<f64> {
    let text: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    let split =
        text.char_indices()
            .find(|&(index, c)| {
                !(c.is_ascii_digit()
                    || c == '.'
                    || ((c == '-' || c == '+')
                        && (index == 0 || matches!(text[..index].chars().last(), Some('e' | 'E')))))
                    && !((c == 'e' || c == 'E')
                        && text[index + 1..].chars().next().is_some_and(|next| {
                            next.is_ascii_digit() || next == '-' || next == '+'
                        }))
            })
            .map_or(text.len(), |(index, _)| index);
    let number: f64 = text[..split].parse().ok()?;
    let suffix = &text[split..];
    let scale = if suffix.is_empty() || suffix == unit {
        1.0
    } else if suffix.to_ascii_lowercase().starts_with("meg") {
        1e6
    } else {
        match suffix.chars().next()? {
            'f' => 1e-15,
            'p' => 1e-12,
            'n' => 1e-9,
            'u' | '\u{00b5}' | '\u{03bc}' => 1e-6,
            'm' => 1e-3,
            'k' | 'K' => 1e3,
            'M' => 1e6,
            'G' => 1e9,
            'T' => 1e12,
            _ if suffix.chars().all(char::is_alphabetic) => 1.0,
            _ => return None,
        }
    };
    let value = number * scale;
    value.is_finite().then_some(value)
}

/// The retained sample of `model`'s trace `anchor` nearest `x`, or of its
/// first visible trace when the column rides none.
pub fn nearest_sample_x(
    model: &StripModel,
    anchor: Option<&WaveformPresentationKey>,
    x: f64,
) -> Option<f64> {
    let trace = anchor
        .and_then(|anchor| {
            model
                .traces
                .iter()
                .find(|trace| !trace.overlay && anchor_key(model, trace) == *anchor)
        })
        .or_else(|| readout_traces(model).next())?;
    let distance = |left: &f64, right: &f64| (left - x).abs().total_cmp(&(right - x).abs());
    let Some((xs, _)) = trace.held() else {
        return nearest_sample_around(&trace.x, &trace.shape, x, distance);
    };
    if trace.shape.is_single_ascending() {
        let at = xs.partition_point(|sample| *sample < x);
        return [at.checked_sub(1), Some(at)]
            .into_iter()
            .flatten()
            .filter_map(|index| xs.get(index).copied())
            .filter(|sample| sample.is_finite())
            .min_by(distance);
    }
    xs.iter()
        .copied()
        .filter(|sample| sample.is_finite())
        .min_by(distance)
}

/// The sample of `xs` nearest `x` from the rows around it alone: the two
/// either side of it in each monotone run of `shape`, found by bisection, so
/// a stored axis whose whole column is not held reads a few rows a run.
fn nearest_sample_around(
    xs: &SharedWaveformValues,
    shape: &rspice_ui_kit::plot::sample::SweepShape,
    x: f64,
    distance: impl Fn(&f64, &f64) -> std::cmp::Ordering,
) -> Option<f64> {
    use rspice_results::sampling::XOrientation;
    let runs: Vec<(std::ops::Range<usize>, bool)> = if shape.is_single_ascending() {
        vec![(0..xs.len(), true)]
    } else {
        shape
            .runs()
            .iter()
            .map(|run| {
                (
                    run.start..run.end,
                    run.orientation == XOrientation::Ascending,
                )
            })
            .collect()
    };
    let sample = |index: usize| xs.sample(index).filter(|sample| sample.is_finite());
    runs.into_iter()
        .flat_map(|(rows, ascending)| {
            let before = |sample: f64| if ascending { sample < x } else { sample > x };
            // The first row of the run past `x`.
            let (mut low, mut high) = (rows.start, rows.end);
            while low < high {
                let middle = low + (high - low) / 2;
                if xs.sample(middle).is_some_and(before) {
                    low = middle + 1;
                } else {
                    high = middle;
                }
            }
            [
                low.checked_sub(1).filter(|row| *row >= rows.start),
                Some(low),
            ]
            .into_iter()
            .flatten()
            .filter(move |row| *row < rows.end)
        })
        .filter_map(sample)
        .min_by(distance)
}

/// `x`, or the retained sample nearest it while the page snaps.
pub(super) fn snapped(
    state: &ResultViewerState,
    model: &StripModel,
    anchor: Option<&WaveformPresentationKey>,
    x: f64,
) -> f64 {
    if state.snap_to_samples {
        nearest_sample_x(model, anchor, x).unwrap_or(x)
    } else {
        x
    }
}

/// Put A and B on the nearest retained sample of the trace A rides, while
/// the page snaps; says whether either moved.
pub fn snap_cursors(state: &mut ResultViewerState, models: &[StripModel]) -> bool {
    if !state.snap_to_samples {
        return false;
    }
    let Some(model) = state
        .cursor_strip
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))
    else {
        return false;
    };
    let anchor = state.cursor_a_anchor.clone();
    let mut moved = false;
    for id in [CursorId::A, CursorId::B] {
        if let Some(x) = state.cursors.get(id)
            && let Some(sample) = nearest_sample_x(model, anchor.as_ref(), x)
            && sample.to_bits() != x.to_bits()
        {
            state.cursors.set(id, sample);
            moved = true;
        }
    }
    moved
}

/// The strip a reading added from the page goes on: the one the cursors
/// stand on, else the first on screen.
pub(super) fn followed_model(state: &ResultViewerState, source: ReadoutInput<'_>) -> Option<usize> {
    let models = source.models;
    state
        .cursor_strip
        .filter(|_| state.cursor_readout_active())
        .and_then(|index| {
            models
                .iter()
                .position(|model| model.analysis_index == index)
        })
        .or_else(|| {
            models
                .iter()
                .position(|model| source.on_screen_strips.contains(&model.analysis_key))
        })
}

/// The X window a strip shows: the reader's zoom, or its whole record.
pub(super) fn shown_x_window(state: &ResultViewerState, model: &StripModel) -> Option<(f64, f64)> {
    super::stack::shared_x_view(state, model.analysis_key).or_else(|| {
        let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);
        for trace in readout_traces(model) {
            if let Some((xs, _)) = trace.held()
                && let (Some(first), Some(last)) = (xs.first(), xs.last())
            {
                low = low.min(first.min(*last));
                high = high.max(first.max(*last));
            }
        }
        (low < high).then_some((low, high))
    })
}

/// The point `fraction` of the way across `window`, evenly on the strip's
/// own scale.
pub(super) fn window_point(model: &StripModel, (low, high): (f64, f64), fraction: f64) -> f64 {
    match model.x_scale {
        XScale::Log10 if low > 0.0 && high > 0.0 => {
            10f64.powf(low.log10() + (high.log10() - low.log10()) * fraction)
        }
        _ => low + (high - low) * fraction,
    }
}

/// What the page's Add offers, in its order.
pub const ADD_CHOICES: [&str; 4] = [
    "Delta pair",
    "Vertical marker",
    "Horizontal cursor",
    "Point marker",
];

/// Place A a third and B two thirds across the followed strip's window,
/// arming the cursor tool; says whether there was a strip to place them on.
pub fn add_delta_pair(state: &mut ResultViewerState, source: ReadoutInput<'_>) -> bool {
    let Some(model) = followed_model(state, source).map(|index| &source.models[index]) else {
        return false;
    };
    let Some(window) = shown_x_window(state, model) else {
        return false;
    };
    state.arm_cursor_tool();
    state.cursors = CursorPair::default();
    state.cursor_strip = Some(model.analysis_index);
    state.cursor_a_anchor = None;
    let a = snapped(state, model, None, window_point(model, window, 1.0 / 3.0));
    let b = snapped(state, model, None, window_point(model, window, 2.0 / 3.0));
    state.cursors.a = Some(a);
    state.cursors.b = Some(b);
    state.stepping_cursor = CursorId::A;
    true
}

/// Place a horizontal cursor on the followed strip's active pane: H1 at
/// three quarters of what its traces reach, H2 at a quarter. Says whether
/// one was placed; a pane with both placed takes no third.
pub fn add_horizontal(state: &mut ResultViewerState, source: ReadoutInput<'_>) -> bool {
    let Some(model) = followed_model(state, source).map(|index| &source.models[index]) else {
        return false;
    };
    let panes = model.unit_panes();
    let Some(pane) = state
        .active_wave_pane
        .as_ref()
        .filter(|active| active.analysis == model.analysis_key)
        .and_then(|active| panes.iter().find(|pane| pane.unit == active.unit))
        .or_else(|| panes.first())
    else {
        return false;
    };
    let key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    };
    let Some(slot) = state
        .pane_horizontal_cursors(&key)
        .iter()
        .position(Option::is_none)
    else {
        return false;
    };
    let Some((low, high)) =
        super::stack::pane_trace_extremes(&mut state.derived, model, &pane.traces, None, |_| {})
            .range()
    else {
        return false;
    };
    let fraction = if slot == 0 { 0.75 } else { 0.25 };
    state.set_horizontal_cursor(key, slot, low + (high - low) * fraction);
    true
}

/// Where a marker added from the page goes: on the trace A rides, else the
/// followed strip's first, at the middle of the strip's window.
pub struct MarkerSpot {
    pub analysis: AnalysisPresentationKey,
    pub anchor: WaveformPresentationKey,
    pub trace_name: String,
    pub x: f64,
    /// The trace's X samples, for a saved view to land the marker on one.
    pub samples: SharedWaveformValues,
}

pub fn marker_spot(state: &ResultViewerState, source: ReadoutInput<'_>) -> Option<MarkerSpot> {
    let model = &source.models[followed_model(state, source)?];
    let trace = state
        .cursor_a_anchor
        .as_ref()
        .and_then(|anchor| {
            readout_traces(model)
                .find(|trace| !trace.overlay && anchor_key(model, trace) == *anchor)
        })
        .or_else(|| readout_traces(model).find(|trace| !trace.overlay && !trace.is_expression()))?;
    let anchor = anchor_key(model, trace);
    let x = window_point(model, shown_x_window(state, model)?, 0.5);
    Some(MarkerSpot {
        analysis: model.analysis_key,
        x: snapped(state, model, Some(&anchor), x),
        anchor,
        trace_name: trace.name.clone(),
        samples: trace.x.clone(),
    })
}

/// Paint the page's tables, then its horizontal cursors, and take what the
/// reader asks of them. Every table shares one name column, so their
/// readings start on one edge.
pub fn paint_page(
    ui: &mut Ui,
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    tables: &[CursorTable],
) -> MarkerActions {
    let mut requests = MarkerActions::default();
    let horizontal =
        horizontal_sections(source, &super::dock::horizontal_cursor_rows(state, source));
    let name_w = name_column_width(ui, tables, &horizontal);
    for (index, table) in tables.iter().enumerate() {
        paint_table(ui, state, source, (index, table), name_w, &mut requests);
    }
    if !horizontal.is_empty() {
        paint_horizontal(ui, state, &horizontal, name_w);
    }
    requests
}

/// The words a table cell prints in, for sizing a column to them.
fn cell_font() -> egui::FontId {
    theme::mono(tokens::FS_1, FontWeight::Regular)
}

/// The name column every table of the page draws: as wide as its widest
/// name, within bounds.
fn name_column_width(ui: &Ui, tables: &[CursorTable], horizontal: &[HorizontalSection]) -> f32 {
    let font = cell_font();
    let mut width = text_width(ui, "Trace", &font);
    for table in tables {
        width = width.max(text_width(ui, &table.x_label, &font));
        for row in table.bands.iter().flat_map(|band| &band.rows) {
            width = width.max(text_width(ui, &row.name, &font));
        }
    }
    for row in horizontal.iter().flat_map(|section| &section.rows) {
        width = width.max(text_width(ui, &row.name, &font));
    }
    width.clamp(96.0, 320.0) + CELL_PAD * 2.0
}

fn text_width(ui: &Ui, text: &str, font: &egui::FontId) -> f32 {
    ui.painter()
        .layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
}

/// A trace's row leads with a tab of its colour, so a reader ties the row
/// to its curve without matching names.
fn swatch_cell(color: egui::Color32) -> Cell {
    Cell::empty().wash(color)
}

/// A printed reading as a cell: a number, or the mark for nothing read.
fn reading_cell(text: &str) -> Cell {
    if text == READOUT_ABSENT {
        Cell::absent()
    } else {
        Cell::quantity(text)
    }
}

/// A column's X as the reader types it in the table's X row, and as it
/// read when they began.
#[derive(Clone)]
struct XDraft {
    table: usize,
    /// The table's column, past its swatch and name.
    column: usize,
    text: String,
    seed: String,
    focus: bool,
}

fn x_draft_id() -> egui::Id {
    egui::Id::new("rspice.results.cursor-table.x-draft")
}

/// The column menu a right-click in a table asked for: the table, its
/// column and where.
fn column_menu_id() -> egui::Id {
    egui::Id::new("rspice.results.cursor-table.column-menu")
}

/// Columns before the readings: the swatch and the name.
const LEAD_COLUMNS: usize = 2;

fn paint_table(
    ui: &mut Ui,
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    (index, table): (usize, &CursorTable),
    name_w: f32,
    requests: &mut MarkerActions,
) {
    let font = cell_font();
    let heads = table.heads();
    let readings = table.columns.len();

    // Columns as wide as what they hold, so no reading is cut short; the
    // last column takes what is left, so the readings stand by the names.
    let mut columns = vec![
        Column::words("", SWATCH_W).unsortable(),
        Column::words("Trace", name_w).unsortable(),
    ];
    for (column, head) in heads.iter().enumerate() {
        let mut width = text_width(ui, head, &font);
        if let Some(x) = table.x_cells.get(column) {
            width = width.max(text_width(ui, x, &font));
        }
        for row in table.bands.iter().flat_map(|band| &band.rows) {
            if let Some(cell) = row.cells.get(column) {
                width = width.max(text_width(ui, cell, &font));
            }
        }
        let tooltip = match table.columns.get(column) {
            Some(reading) => reading.hover.clone(),
            None if column < readings + table.deltas.len() => {
                format!("{head}: each row's difference, and over it their separation")
            }
            None => "The pair's slope, and over it the reciprocal of their separation".to_owned(),
        };
        columns.push(
            Column::number(head, (width + CELL_PAD * 2.0).max(VALUE_COLUMN_MIN_W))
                .unsortable()
                .tooltip(tooltip),
        );
    }
    columns.push(Column::name("", 0.0).unsortable());

    // The X row first, its readings' X typed in place; then each strip's
    // rows under a group head that names it.
    let mut rows: Vec<Vec<Cell>> = Vec::new();
    let mut x_row = vec![Cell::empty(), Cell::text(&table.x_label)];
    for (column, x) in table.x_cells.iter().enumerate() {
        x_row.push(match table.columns.get(column).map(|reading| &reading.x) {
            Some(Ok(_)) => Cell::quantity(x)
                .editable()
                .hint("Click to type where it stands; right-click for its menu"),
            Some(Err(why)) => Cell::text(why),
            None => reading_cell(x),
        });
    }
    x_row.push(Cell::empty());
    rows.push(x_row);
    let mut groups = Vec::new();
    for band in &table.bands {
        groups.push(Group::new(&band.title, rows.len()));
        for row in &band.rows {
            let mut cells = vec![swatch_cell(row.swatch), Cell::text(&row.name)];
            cells.extend(row.cells.iter().map(|cell| reading_cell(cell)));
            cells.push(Cell::empty());
            rows.push(cells);
        }
    }

    let id = format!("rspice.results.cursor-table.{index}");
    let state_id = egui::Id::new(&id);
    let mut table_state = ui
        .data(|data| data.get_temp::<TableState>(state_id))
        .unwrap_or_default();
    let mut draft = ui
        .data(|data| data.get_temp::<XDraft>(x_draft_id()))
        .filter(|draft| draft.table == index);
    let editor = draft.as_mut().map(|draft| CellEditor {
        row: 0,
        column: draft.column + LEAD_COLUMNS,
        text: &mut draft.text,
        focus: std::mem::take(&mut draft.focus),
    });
    let response = DataTable::new(&id, &columns, rows.len())
        .state(&mut table_state)
        .groups(&groups)
        .freeze(LEAD_COLUMNS)
        .editing(editor)
        .show(ui, |row| rows[row].clone());
    ui.data_mut(|data| data.insert_temp(state_id, table_state));

    // A reading's X opens for entry; Enter moves it there.
    if let Some((0, column)) = response.edit
        && let Some(reading) = column
            .checked_sub(LEAD_COLUMNS)
            .and_then(|column| table.columns.get(column))
        && reading.x.is_ok()
    {
        let seed = table.x_cells[column - LEAD_COLUMNS].clone();
        draft = Some(XDraft {
            table: index,
            column: column - LEAD_COLUMNS,
            text: seed.clone(),
            seed,
            focus: true,
        });
    }
    match response.edited {
        Some(EditOutcome::Cancel) => draft = None,
        Some(EditOutcome::Commit) => {
            if let Some(open) = draft.take()
                && open.text.trim() != open.seed.trim()
                && let Some(reading) = table.columns.get(open.column)
            {
                let unit = &source.models[reading.model].x_unit;
                match parse_typed_x(&open.text, unit) {
                    Some(x) => move_column(state, source, reading, x, requests),
                    // An entry that does not read stays open to be put right.
                    None => {
                        draft = Some(XDraft {
                            focus: true,
                            ..open
                        });
                    }
                }
            }
        }
        None => {}
    }
    ui.data_mut(|data| match draft {
        Some(draft) => {
            data.insert_temp(x_draft_id(), draft);
        }
        None => {
            if data
                .get_temp::<XDraft>(x_draft_id())
                .is_some_and(|draft| draft.table == index)
            {
                data.remove::<XDraft>(x_draft_id());
            }
        }
    });

    // A right-click in a reading's column asks for its menu.
    let ctx = ui.ctx().clone();
    if let Some(asked) = response.context_menu
        && let Some(column) = asked
            .cell
            .column
            .and_then(|column| column.checked_sub(LEAD_COLUMNS))
            .filter(|column| *column < readings)
    {
        ctx.data_mut(|data| data.insert_temp(column_menu_id(), (index, column, asked.position)));
        egui::Popup::open_id(&ctx, column_menu_id());
    }
    if egui::Popup::is_id_open(&ctx, column_menu_id())
        && let Some((menu_table, column, position)) =
            ctx.data(|data| data.get_temp::<(usize, usize, egui::Pos2)>(column_menu_id()))
        && menu_table == index
        && column < readings
    {
        egui::Popup::new(
            column_menu_id(),
            ctx.clone(),
            egui::PopupAnchor::Position(position),
            egui::LayerId::new(egui::Order::Foreground, column_menu_id()),
        )
        .kind(egui::PopupKind::Menu)
        .open_memory(None)
        .show(|ui| column_menu(ui, state, source, table, column, requests));
    }
}

/// Move a column to `x`: a cursor at once, a marker through the view
/// that keeps it.
fn move_column(
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    reading: &ReadingColumn,
    x: f64,
    requests: &mut MarkerActions,
) {
    let x = snapped(
        state,
        &source.models[reading.model],
        reading.anchor.as_ref(),
        x,
    );
    match reading.id {
        ReadingColumnId::Cursor(id) => {
            state.cursors.set(id, x);
            if id == CursorId::A {
                state.cursor_a_anchor = reading.anchor.clone();
            }
            state.stepping_cursor = id;
        }
        ReadingColumnId::Marker(selector) => {
            requests.moved = Some(MarkerMove {
                selector,
                x,
                nearest_sample: nearest_sample_x(
                    &source.models[reading.model],
                    reading.anchor.as_ref(),
                    x,
                ),
            });
        }
    }
}

/// Where an offset from marker `selector` is put: where that marker stands,
/// on its trace. `None` for an offset, which another cannot be read from.
fn offset_spot(
    state: &ResultViewerState,
    source: ReadoutInput<'_>,
    reading: &ReadingColumn,
    selector: MarkerSelector,
) -> Option<MarkerSpot> {
    let marker = state.document_marker(selector.marker_id)?;
    if matches!(marker.kind, MarkerKind::Offset(_)) {
        return None;
    }
    let model = &source.models[reading.model];
    let trace = model
        .traces
        .iter()
        .find(|trace| !trace.overlay && anchor_key(model, trace) == marker.anchor)?;
    Some(MarkerSpot {
        analysis: model.analysis_key,
        anchor: marker.anchor.clone(),
        trace_name: marker.trace_name.clone(),
        x: *reading.x.as_ref().ok()?,
        samples: trace.x.clone(),
    })
}

fn column_menu(
    ui: &mut Ui,
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    table: &CursorTable,
    column: usize,
    requests: &mut MarkerActions,
) {
    let reading = &table.columns[column];
    if ui
        .add_enabled(column != table.base, egui::Button::new("Make base"))
        .on_hover_text("Read the other columns' differences against this one")
        .clicked()
    {
        state.readout_base = Some(reading.id);
        ui.close();
    }
    let snapped =
        reading.x.as_ref().ok().and_then(|&x| {
            nearest_sample_x(&source.models[reading.model], reading.anchor.as_ref(), x)
        });
    if ui
        .add_enabled(
            snapped.is_some(),
            egui::Button::new("Snap to nearest sample"),
        )
        .on_hover_text("Move it onto the nearest retained sample of the trace it rides")
        .clicked()
    {
        if let Some(x) = snapped {
            move_column(state, source, reading, x, requests);
        }
        ui.close();
    }
    if let ReadingColumnId::Marker(selector) = reading.id
        && ui
            .button("Edit marker\u{2026}")
            .on_hover_text("Edit the marker's label and kind")
            .clicked()
    {
        requests.edit = Some(selector);
        ui.close();
    }
    if let ReadingColumnId::Marker(selector) = reading.id
        && reading.point
        && let Some(spot) = offset_spot(state, source, reading, selector)
        && ui
            .button("Add offset marker\u{2026}")
            .on_hover_text(
                "A marker a typed amount from this one's level on its trace, as 3 dB below a \
                 peak finds a corner; it follows this one",
            )
            .clicked()
    {
        requests.offset = Some((spot, selector));
        ui.close();
    }
    ui.separator();
    if ui.button("Delete").clicked() {
        match reading.id {
            ReadingColumnId::Cursor(CursorId::A) => {
                // B takes A's place, as a pair always has an A.
                match state.cursors.b.take() {
                    Some(b) => {
                        state.cursors.a = Some(b);
                        state.cursor_a_anchor = None;
                        state.stepping_cursor = CursorId::A;
                    }
                    None => state.clear_cursors(),
                }
            }
            ReadingColumnId::Cursor(CursorId::B) => {
                state.cursors.b = None;
                state.stepping_cursor = CursorId::A;
            }
            ReadingColumnId::Marker(selector) => requests.remove = Some(selector),
        }
        ui.close();
    }
}

/// Where `ys` over `xs` crosses `level`, by linear interpolation between
/// the samples either side.
pub fn crossings(xs: &[f64], ys: &[f64], level: f64) -> Vec<f64> {
    let mut found = Vec::new();
    for index in 1..xs.len().min(ys.len()) {
        let (x0, x1, y0, y1) = (
            xs[index - 1],
            xs[index],
            ys[index - 1] - level,
            ys[index] - level,
        );
        if !(x0.is_finite() && x1.is_finite() && y0.is_finite() && y1.is_finite()) {
            continue;
        }
        if y0 == 0.0 {
            if found.last() != Some(&x0) {
                found.push(x0);
            }
        } else if y0.signum() != y1.signum() && y1 != 0.0 {
            found.push(x0 + (x1 - x0) * (y0 / (y0 - y1)));
        } else if y1 == 0.0 {
            found.push(x1);
        }
    }
    found
}

/// One pane's horizontal cursors: its levels, and where its traces cross
/// them.
struct HorizontalSection {
    pane: WavePanePresentationKey,
    /// The pane, its levels and their difference, as its group head reads.
    title: String,
    rows: Vec<CrossingRow>,
}

/// Where one trace crosses each level of its pane.
struct CrossingRow {
    name: String,
    swatch: egui::Color32,
    /// Per level, every crossing in X order, as printed.
    crossings: Vec<Vec<String>>,
}

fn horizontal_sections(
    source: ReadoutInput<'_>,
    panes: &[(WavePanePresentationKey, HorizontalCursors)],
) -> Vec<HorizontalSection> {
    let digits = usize::from(source.presentation.displayed_significant_digits().get());
    let quantity_policy = source.quantity_policy;
    let mut sections = Vec::new();
    for (pane, cursors) in panes {
        let model = source
            .models
            .iter()
            .find(|model| model.analysis_key == pane.analysis);
        let unit = model.map_or_else(
            || pane.unit.clone(),
            |model| model.pane_axis_unit(&pane.unit),
        );
        let format = |value: f64| rspice_ui_kit::plot::fmt_column_reading(value, &unit, digits);
        let levels: Vec<(usize, f64)> = cursors
            .iter()
            .enumerate()
            .filter_map(|(slot, level)| level.map(|level| (slot, level)))
            .collect();
        let mut title = vec![super::dock::horizontal_pane_name(source.models, pane)];
        title.extend(
            levels
                .iter()
                .map(|&(slot, level)| format!("H{} {}", slot + 1, format(level))),
        );
        if let [Some(h1), Some(h2)] = cursors {
            title.push(format!("\u{0394}Y {}", format(h2 - h1)));
        }
        let mut rows = Vec::new();
        if let Some(model) = model {
            let traces = model
                .unit_panes()
                .into_iter()
                .find(|unit_pane| unit_pane.unit == pane.unit)
                .map(|unit_pane| unit_pane.traces)
                .unwrap_or_default();
            for &index in &traces {
                let Some(trace) = model.traces.get(index).filter(|trace| trace.visible) else {
                    continue;
                };
                let Some((xs, ys)) = trace.held() else {
                    continue;
                };
                rows.push(CrossingRow {
                    name: (source.display_name)(&trace.name).into_owned(),
                    swatch: trace.color,
                    crossings: levels
                        .iter()
                        .map(|&(_, level)| {
                            crossings(&xs, &ys, level)
                                .into_iter()
                                .map(|x| model.format_x(x, digits, quantity_policy))
                                .collect()
                        })
                        .collect(),
                });
            }
        }
        sections.push(HorizontalSection {
            pane: pane.clone(),
            title: title.join("  \u{00b7}  "),
            rows,
        });
    }
    sections
}

/// A level's crossings as one cell: their count, then each in X order.
/// The table cuts it after a whole word, and shows it whole on hover.
fn crossings_text(crossings: &[String]) -> String {
    if crossings.is_empty() {
        "no crossing".to_owned()
    } else {
        format!("{} \u{00b7} {}", crossings.len(), crossings.join(", "))
    }
}

/// The horizontal cursors: a group per pane, its head naming the pane, its
/// levels and their difference, then where each of the pane's traces
/// crosses each level. A right-click on a row removes its pane's cursors.
fn paint_horizontal(
    ui: &mut Ui,
    state: &mut ResultViewerState,
    sections: &[HorizontalSection],
    name_w: f32,
) {
    let font = cell_font();
    let levels = sections
        .iter()
        .flat_map(|section| section.rows.iter().map(|row| row.crossings.len()))
        .max()
        .unwrap_or(0);
    let mut columns = vec![
        Column::words("", SWATCH_W).unsortable(),
        Column::words("Trace", name_w).unsortable(),
    ];
    for level in 0..levels {
        let width = sections
            .iter()
            .flat_map(|section| &section.rows)
            .filter_map(|row| row.crossings.get(level))
            .map(|listed| text_width(ui, &crossings_text(listed), &font) + CELL_PAD * 2.0)
            .fold(VALUE_COLUMN_MIN_W, f32::max)
            .min(CROSSINGS_MAX_W);
        columns.push(
            Column::words(format!("H{} crossings", level + 1), width)
                .unsortable()
                .tooltip("Where each trace crosses the level, counted, in X order"),
        );
    }
    columns.push(Column::name("", 0.0).unsortable());

    let mut rows: Vec<Vec<Cell>> = Vec::new();
    let mut owners = Vec::new();
    let mut groups = Vec::new();
    for (index, section) in sections.iter().enumerate() {
        groups.push(Group::new(&section.title, rows.len()));
        if section.rows.is_empty() {
            let mut cells = vec![Cell::empty(), Cell::text("no trace on this pane")];
            cells.resize_with(columns.len(), Cell::empty);
            rows.push(cells);
            owners.push(index);
        }
        for row in &section.rows {
            let mut cells = vec![swatch_cell(row.swatch), Cell::text(&row.name)];
            cells.extend(
                row.crossings
                    .iter()
                    .map(|listed| Cell::text(crossings_text(listed))),
            );
            cells.resize_with(columns.len(), Cell::empty);
            rows.push(cells);
            owners.push(index);
        }
    }

    let id = "rspice.results.horizontal-cursors";
    let state_id = egui::Id::new(id);
    let mut table_state = ui
        .data(|data| data.get_temp::<TableState>(state_id))
        .unwrap_or_default();
    let response = DataTable::new(id, &columns, rows.len())
        .state(&mut table_state)
        .groups(&groups)
        .freeze(LEAD_COLUMNS)
        .show(ui, |row| rows[row].clone());
    ui.data_mut(|data| data.insert_temp(state_id, table_state));

    let ctx = ui.ctx().clone();
    let menu = egui::Id::new("rspice.results.horizontal-cursors.menu");
    if let Some(asked) = response.context_menu
        && let Some(&owner) = owners.get(asked.cell.row)
    {
        ctx.data_mut(|data| data.insert_temp(menu, (owner, asked.position)));
        egui::Popup::open_id(&ctx, menu);
    }
    if egui::Popup::is_id_open(&ctx, menu)
        && let Some((owner, position)) = ctx.data(|data| data.get_temp::<(usize, egui::Pos2)>(menu))
        && let Some(section) = sections.get(owner)
    {
        egui::Popup::new(
            menu,
            ctx.clone(),
            egui::PopupAnchor::Position(position),
            egui::LayerId::new(egui::Order::Foreground, menu),
        )
        .kind(egui::PopupKind::Menu)
        .open_memory(None)
        .show(|ui| {
            if ui.button("Remove this pane's horizontal cursors").clicked() {
                state.horizontal_cursors.remove(&section.pane);
                ui.close();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_typed_x_reads_as_the_table_prints_it() {
        assert_eq!(parse_typed_x("10 ns", "s"), Some(10.0e-9));
        assert_eq!(parse_typed_x("1.5 MHz", "Hz"), Some(1.5e6));
        assert_eq!(parse_typed_x("2 mHz", "Hz"), Some(2.0e-3));
        assert_eq!(parse_typed_x("2.2u", "s"), Some(2.2e-6));
        assert_eq!(parse_typed_x("3 \u{00b5}s", "s"), Some(3.0e-6));
        assert_eq!(parse_typed_x("1meg", "Hz"), Some(1.0e6));
        assert_eq!(parse_typed_x("4 s", "s"), Some(4.0));
        assert_eq!(parse_typed_x("-1.25e-3", "V"), Some(-1.25e-3));
        assert_eq!(parse_typed_x("12", ""), Some(12.0));
        assert_eq!(parse_typed_x("ten", "s"), None);
        assert_eq!(parse_typed_x("", "s"), None);
    }

    /// A stored axis of `rows` rows, `value(row)` each, made as they are
    /// read, counting the rows read.
    struct Computed {
        rows: u64,
        value: fn(u64) -> f64,
        ascending: bool,
        read: std::sync::atomic::AtomicU64,
    }

    impl rspice_results::waveform::ColumnSource for Computed {
        fn name(&self) -> &str {
            "time"
        }

        fn store(&self) -> String {
            "run.rsr".to_owned()
        }

        fn rows(&self) -> u64 {
            self.rows
        }

        fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
            self.read
                .fetch_add(out.len() as u64, std::sync::atomic::Ordering::SeqCst);
            for (row, slot) in (first..).zip(out.iter_mut()) {
                *slot = (self.value)(row);
            }
            Ok(())
        }

        fn ascending(&self) -> bool {
            self.ascending
        }
    }

    fn computed(
        rows: u64,
        value: fn(u64) -> f64,
        ascending: bool,
    ) -> (std::sync::Arc<Computed>, SharedWaveformValues) {
        let column = std::sync::Arc::new(Computed {
            rows,
            value,
            ascending,
            read: Default::default(),
        });
        let window = rspice_results::waveform::StoredWindow {
            first: 0,
            rows,
            lead: rspice_results::waveform::WindowLead::None,
        };
        let values = SharedWaveformValues::stored(column.clone(), window).expect("it fits");
        (column, values)
    }

    /// Snap finds the sample nearest a cursor on a stored axis whose column
    /// is not held, from the rows around the cursor alone: on a run of two
    /// hundred million rows, a few dozen of them; on a sweep that goes up and
    /// back, the nearest of either way.
    #[test]
    fn snap_finds_the_nearest_sample_of_a_stored_axis_from_the_rows_around_it() {
        use rspice_ui_kit::plot::sample::SweepShape;
        let distance = |x: f64| move |a: &f64, b: &f64| (a - x).abs().total_cmp(&(b - x).abs());
        let (column, axis) = computed(1 << 28, |row| row as f64 * 1.0e-9, true);
        let shape = SweepShape::of_values(&axis);
        for (at, row) in [(1000.4e-9, 1000_u32), (1000.6e-9, 1001), (-1.0, 0)] {
            let nearest = f64::from(row) * 1.0e-9;
            let snapped = nearest_sample_around(&axis, &shape, at, distance(at));
            assert_eq!(
                snapped.map(f64::to_bits),
                Some(f64::to_bits(nearest)),
                "{at}"
            );
        }
        let read = column.read.load(std::sync::atomic::Ordering::SeqCst);
        assert!(read < 3 * 64, "{read} rows read");

        // Up from 0 to 5 in steps of 1, then back down in steps of 0.75.
        let (_, sweep) = computed(
            13,
            |row| {
                if row <= 5 {
                    row as f64
                } else {
                    5.0 - 0.75 * (row - 5) as f64
                }
            },
            false,
        );
        let shape = SweepShape::of_values(&sweep);
        assert_eq!(shape.runs().len(), 2);
        let samples: Vec<f64> = sweep.samples(..).collect();
        for at in [2.4, 2.7, 4.9, -0.2, 0.6] {
            let every = samples.iter().copied().min_by(distance(at));
            assert_eq!(
                nearest_sample_around(&sweep, &shape, at, distance(at)),
                every,
                "{at}"
            );
        }
    }

    #[test]
    fn crossings_are_found_between_samples_and_on_them() {
        let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
        let ys = [0.0, 1.0, 0.0, -1.0, 0.0];
        assert_eq!(crossings(&xs, &ys, 0.5), vec![0.5, 1.5]);
        assert_eq!(crossings(&xs, &ys, 0.0), vec![0.0, 2.0, 4.0]);
        assert!(crossings(&xs, &ys, 2.0).is_empty());
    }
}
