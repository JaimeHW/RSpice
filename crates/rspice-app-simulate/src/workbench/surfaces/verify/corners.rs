//! Corners: every specification at every point the run solved.
//!
//! One row per specification and one column per point, the nominal point
//! first, as the ADE Assembler detail view lays a corner run out. The head
//! names each point without a legend: one band row for each coordinate that
//! varies across the run but the last, a run of equal neighbours under one
//! span, and the head row itself stating the last coordinate. Transposed, the
//! points are the rows, their coordinates the leading columns, and every
//! specification a column to sort by.
//!
//! Nothing here judges a value. Every cell is the verification sheet's, and
//! its colour is the sheet's status for it: a failure, an error and a value
//! near its limit are marked, a pass is neutral, a point outside the
//! specification's scope reads "n/a", and a point that gave nothing a dash.
//! A run-set sweep, a corner family and a sweep family are all points to the
//! sheet, so all of them fill this view.

use rspice_results::verification::{
    Cell as SheetCell, CellStatus, Point, PointAxis, PointSource, Row, RowStatus, Sheet,
};

use super::toolbar::{PageToolbar, RunChoice, Summary, ToolbarAction, empty_line};
use super::verdict::{Verdict, VerdictCell};
use super::*;
use crate::state::FamilyMemberId;
use crate::ui::widgets::table::{
    Align, Cell, Column, DataTable, RowKey, Selection, SortKey, Span, TableState,
};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::requests::SetFamilyFilter;
use crate::workbench::simulate::commands::SimulateCommand;

/// The plain arrangement's leading columns: Specification, Limit, Min, Max.
/// All four stay put while the points scroll, as ADE Assembler's Detail view
/// reads a specification's spread across the run before its corners: the
/// worst case is the row's answer, so it never scrolls out of view.
pub(super) const LEADING: usize = 4;
/// Least width of a point or specification column.
const VALUE_COLUMN_MIN_W: f32 = 88.0;
/// The word the nominal point's head starts with.
const NOMINAL: &str = "Nominal";

/// A coordinate as few digits as it needs, with a true minus sign.
fn coordinate(value: f64) -> String {
    let rounded = (value * 1.0e6).round() / 1.0e6;
    format!("{rounded}").replace('-', "\u{2212}")
}

/// What a point says on one axis, as a head states it: `SS`, `1.62 V`,
/// `125 °C`, `rload = 2k`.
pub(super) fn axis_value(point: &Point, axis: &PointAxis) -> Option<String> {
    match axis {
        PointAxis::Process => point.process.clone(),
        PointAxis::Supply => point
            .supply_voltage
            .map(|volts| format!("{} V", coordinate(volts))),
        PointAxis::Temperature => point
            .temperature_celsius
            .map(|celsius| format!("{} \u{00b0}C", coordinate(celsius))),
        PointAxis::Swept { axis, name } => point
            .swept_values
            .iter()
            .find(|value| value.axis == *axis && value.name == *name)
            .map(|value| format!("{} = {}", value.name, value.value)),
    }
}

/// The number an axis value sorts by, where it has one.
fn axis_number(point: &Point, axis: &PointAxis) -> Option<f64> {
    match axis {
        PointAxis::Supply => point.supply_voltage,
        PointAxis::Temperature => point.temperature_celsius,
        PointAxis::Process | PointAxis::Swept { .. } => None,
    }
}

/// An axis as a column head names it.
fn axis_name(axis: &PointAxis) -> String {
    match axis {
        PointAxis::Process => "Process".to_owned(),
        PointAxis::Supply => "Supply".to_owned(),
        PointAxis::Temperature => "Temperature".to_owned(),
        PointAxis::Swept { name, .. } => name.clone(),
    }
}

/// `text` with the nominal mark in front when `point` is the nominal one: the
/// one way the nominal point is told apart, in either arrangement.
fn marked(point: &Point, text: String) -> String {
    match (point.nominal, text.is_empty()) {
        (false, _) => text,
        (true, true) => NOMINAL.to_owned(),
        (true, false) => format!("{NOMINAL} {text}"),
    }
}

/// What a point states on the last axis that varies, or its own name when no
/// axis does: the head row of the plain arrangement, and the last coordinate
/// of a transposed row.
fn last_coordinate(point: &Point, axes: &[PointAxis]) -> String {
    let value = axes
        .last()
        .and_then(|axis| axis_value(point, axis))
        .unwrap_or_else(|| point.label.clone());
    marked(point, value)
}

/// A point as a select or a fact names it: its label, or the nominal mark
/// for a point the run's coordinates do not tell apart.
pub(super) fn point_name(point: &Point) -> String {
    if point.label.is_empty() {
        marked(point, String::new())
    } else {
        marked(point, point.label.clone())
    }
}

/// Every coordinate a point has, nominal or not, for a tooltip and a file.
fn point_detail(point: &Point) -> String {
    if point.nominal {
        format!("{} ({})", point.detail, NOMINAL.to_lowercase())
    } else {
        point.detail.clone()
    }
}

/// The plain arrangement's head over the point columns: a band row for each
/// varying axis but the last, and the head row's words for each point.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct PointHeads {
    pub(super) bands: Vec<Vec<Span>>,
    pub(super) heads: Vec<String>,
}

/// The head that names every point without a legend.
///
/// A band's span runs over equal neighbours, and never across a span of the
/// band above it: two columns that share a supply under two processes are two
/// spans, so every rule in a band row lines up with one in the row above.
pub(super) fn point_heads(sheet: &Sheet) -> PointHeads {
    let banded = sheet.axes.len().saturating_sub(1);
    let bands = (0..banded)
        .map(|band| {
            let keys = sheet
                .points
                .iter()
                .map(|point| {
                    sheet.axes[..=band]
                        .iter()
                        .map(|axis| axis_value(point, axis).unwrap_or_default())
                        .collect::<Vec<_>>()
                        .join("\u{1f}")
                })
                .collect::<Vec<_>>();
            Span::runs(LEADING, &keys)
                .into_iter()
                .map(|span| {
                    let label = span.label.rsplit('\u{1f}').next().unwrap_or_default();
                    Span::new(label, span.first, span.last)
                })
                .collect()
        })
        .collect();
    PointHeads {
        bands,
        heads: sheet
            .points
            .iter()
            .map(|point| last_coordinate(point, &sheet.axes))
            .collect(),
    }
}

const fn is_failing(status: CellStatus) -> bool {
    matches!(status, CellStatus::Fail | CellStatus::Error)
}

/// The specifications the plain arrangement lists, by their place in the
/// sheet: every one, or only those with a failing cell.
pub(super) fn shown_specifications(sheet: &Sheet, failing_only: bool) -> Vec<usize> {
    (0..sheet.rows.len())
        .filter(|row| {
            !failing_only
                || sheet.rows[*row]
                    .cells
                    .iter()
                    .any(|cell| is_failing(cell.status))
        })
        .collect()
}

/// The points the transposed arrangement lists: every one, or only those
/// where some specification fails.
pub(super) fn shown_points(sheet: &Sheet, failing_only: bool) -> Vec<usize> {
    (0..sheet.points.len())
        .filter(|point| {
            !failing_only
                || sheet.rows.iter().any(|row| {
                    row.cells
                        .get(*point)
                        .is_some_and(|cell| is_failing(cell.status))
                })
        })
        .collect()
}

/// How bad a cell's status is, worst highest; `None` for a cell that judged
/// nothing.
const fn severity(status: CellStatus) -> Option<u8> {
    match status {
        CellStatus::Pass => Some(0),
        CellStatus::Near => Some(1),
        CellStatus::Fail => Some(3),
        CellStatus::Error => Some(4),
        CellStatus::Excluded | CellStatus::Absent => None,
    }
}

/// A point's status: the worst of its cells.
pub(super) fn point_status(sheet: &Sheet, point: usize) -> Option<CellStatus> {
    sheet
        .rows
        .iter()
        .filter_map(|row| row.cells.get(point))
        .map(|cell| cell.status)
        .filter(|status| severity(*status).is_some())
        .max_by_key(|status| severity(*status))
}

/// A limit as its row states it: `≥ 40 dB`, `≤ 1.2 mA`, `10 dB … 20 dB`.
pub(crate) fn limit_text(row: &Row) -> String {
    rspice_results::verification::limit_text(&row.comparison, &row.unit).unwrap_or_default()
}

/// A limit's cell: its words in mono, the guard band on hover when it has
/// one, nothing for a specification that only tracks its value.
fn limit_cell(row: &Row) -> Cell {
    let text = limit_text(row);
    if text.is_empty() {
        return Cell::empty().hint("Tracked: recorded, not judged.");
    }
    let cell = Cell::number(text);
    if row.guard_band > 0.0 {
        cell.hint(format!(
            "Judged against the limit tightened by its guard band of {}.",
            format_bound(row.guard_band, &row.unit)
        ))
    } else {
        cell
    }
}

/// One sheet cell as the table shows it, inked by its status and nothing
/// else.
pub(super) fn value_cell(cell: &SheetCell, unit: &str) -> Cell {
    let number =
        |value: f64| Cell::number(format_value(value, unit)).sort_by(SortKey::Number(value));
    let margin = |shown: Cell| match cell.value.zip(cell.signed_margin) {
        Some((value, margin)) => {
            shown.hint(format!("Margin {}", difference_text(margin, value, unit)))
        }
        None => shown,
    };
    match (cell.status, cell.value) {
        (CellStatus::Pass, Some(value)) => number(value),
        (CellStatus::Near, Some(value)) => margin(number(value).judged(Verdict::Near)),
        (CellStatus::Fail, Some(value)) => margin(number(value).judged(Verdict::Fail)),
        (CellStatus::Fail, None) => Cell::verdict(Verdict::Fail),
        (CellStatus::Error, value) => Cell::problem(
            value.map_or_else(|| "Error".to_owned(), |value| format_value(value, unit)),
            Problem::Failure,
        )
        .hint("The measurement failed at this point.")
        .sort_by(SortKey::Missing),
        (CellStatus::Excluded, _) => {
            let mut cell = Cell::absent().hint("Outside this specification's scope.");
            cell.text = "n/a".to_owned();
            cell
        }
        (CellStatus::Pass | CellStatus::Near | CellStatus::Absent, _) => Cell::absent(),
    }
}

/// A point's status as a word, coloured as its worst cell is.
fn status_cell(status: Option<CellStatus>) -> Cell {
    match status {
        Some(CellStatus::Pass) => Cell::verdict(Verdict::Pass),
        Some(CellStatus::Near) => Cell::verdict(Verdict::Near),
        Some(CellStatus::Fail) => Cell::verdict(Verdict::Fail),
        Some(CellStatus::Error) => {
            Cell::problem("Error", Problem::Failure).sort_by(SortKey::Status(4))
        }
        Some(CellStatus::Excluded | CellStatus::Absent) | None => Cell::empty(),
    }
}

/// The least and greatest value a row's points gave.
fn extremes(row: &Row) -> Option<(f64, f64)> {
    row.cells
        .iter()
        .filter_map(|cell| cell.value)
        .filter(|value| value.is_finite())
        .fold(None, |held, value| match held {
            None => Some((value, value)),
            Some((low, high)) => Some((low.min(value), high.max(value))),
        })
}

/// The toolbar's one line: how many points, and what fails where.
pub(super) fn corners_summary(sheet: &Sheet) -> Summary {
    let points = format!("{} points", sheet.points.len());
    let failing_rows = shown_specifications(sheet, true).len();
    if failing_rows > 0 {
        let at = shown_points(sheet, true);
        let place = match at.as_slice() {
            [point] => point_name(&sheet.points[*point]),
            several => format!("{} points", several.len()),
        };
        let verb = if failing_rows == 1 { "fails" } else { "fail" };
        return Summary::failing(format!(
            "{points} \u{00b7} {failing_rows} {verb} at {place}"
        ));
    }
    let near = sheet
        .rows
        .iter()
        .filter(|row| row.cells.iter().any(|cell| cell.status == CellStatus::Near))
        .count();
    if near > 0 {
        return Summary::plain(format!("{points} \u{00b7} {near} near the limit"));
    }
    let judged = sheet
        .rows
        .iter()
        .filter(|row| {
            row.status
                .is_some_and(|status| status != RowStatus::Tracked)
        })
        .count();
    if judged > 0 {
        Summary::plain(format!(
            "{points} \u{00b7} {judged} specification{} pass",
            if judged == 1 { "" } else { "s" }
        ))
    } else {
        Summary::plain(points)
    }
}

/// The transposed arrangement's leading columns: one per varying axis, or
/// the point's name when nothing varies, then its status.
pub(super) fn transposed_leading(sheet: &Sheet) -> usize {
    sheet.axes.len().max(1) + 1
}

/// The cell the reader selected in Corners: its point, and its
/// specification when the selection names one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SelectedPoint {
    pub(crate) point: usize,
    pub(crate) row: Option<usize>,
}

/// The cell Corners' selection names, read through either arrangement.
///
/// Rows are keyed by their place in the sheet, a specification's in the
/// plain arrangement and a point's transposed, so the selection is the same
/// cell however the rows are sorted or filtered.
pub(crate) fn selected_point(
    sheet: &Sheet,
    session: &VerificationSessionState,
) -> Option<SelectedPoint> {
    if sheet.points.len() < 2 {
        return None;
    }
    let selection = session.corners_table.selection.as_ref()?;
    let Some(RowKey::Number(key)) = selection.key else {
        return None;
    };
    let key = usize::try_from(key).ok()?;
    if session.corners_transposed {
        let row = selection
            .column
            .and_then(|column| column.checked_sub(transposed_leading(sheet)))
            .filter(|row| *row < sheet.rows.len());
        (key < sheet.points.len()).then_some(SelectedPoint { point: key, row })
    } else {
        let point = selection.column?.checked_sub(LEADING)?;
        (key < sheet.rows.len() && point < sheet.points.len()).then_some(SelectedPoint {
            point,
            row: Some(key),
        })
    }
}

/// The same cell selected in the other arrangement, once the reader turns the
/// table: a point with no specification has no cell in the plain one.
fn turned_selection(sheet: &Sheet, selected: SelectedPoint, transposed: bool) -> Option<Selection> {
    if transposed {
        Some(Selection {
            row: selected.point,
            key: Some(RowKey::Number(selected.point as u64)),
            column: selected.row.map(|row| transposed_leading(sheet) + row),
        })
    } else {
        let row = selected.row?;
        Some(Selection {
            row,
            key: Some(RowKey::Number(row as u64)),
            column: Some(LEADING + selected.point),
        })
    }
}

/// Arrange Corners `transposed` or plain, the selected cell carried across;
/// the sort and the sideways scroll belong to the arrangement left.
fn arrange(cx: &mut Cx<'_, SimulateFeature>, transposed: bool) {
    let turned = cx.model.simulation.verification_sheet().and_then(|sheet| {
        let selected = selected_point(sheet, &cx.state.verification)?;
        turned_selection(sheet, selected, transposed)
    });
    let session = &mut cx.state.verification;
    session.corners_transposed = transposed;
    session.corners_table.selection = turned;
    session.corners_table.sort = None;
    session.corners_table.scroll_x = 0.0;
}

/// Corners opens transposed on a phone-width window: a point a row and a
/// specification a column, so a sweep's many points scroll down the screen
/// and its few specifications across it.
pub(super) fn open_transposed(cx: &mut Cx<'_, SimulateFeature>) {
    if !cx.state.verification.corners_transposed {
        arrange(cx, true);
    }
}

/// Where in the run one cell's value came from: the analysis, and the member
/// of its family when the point is one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Evidence {
    pub(super) analysis: usize,
    pub(super) sample: Option<usize>,
}

/// The analysis behind one cell: of those solved at the point, the one that
/// produces the specification, else one that measured it, else the first.
pub(super) fn evidence_at(
    run: &crate::state::SimulationRun,
    sheet: &Sheet,
    row: usize,
    point: usize,
) -> Option<Evidence> {
    let row = sheet.rows.get(row)?;
    let point = sheet.points.get(point)?;
    let at = run
        .analyses
        .iter()
        .enumerate()
        .filter_map(|(index, analysis)| {
            let provenance = analysis.provenance()?;
            let sample = match &point.source {
                PointSource::Member(member) => Some(
                    analysis
                        .family_metadata
                        .as_ref()?
                        .member_measurements()
                        .iter()
                        .position(|measured| same_member(&measured.member, member))?,
                ),
                PointSource::Attributed(record) => {
                    (provenance.pvt_point() == Some(record)).then_some(None)?
                }
                PointSource::Deck => provenance.pvt_point().is_none().then_some(None)?,
            };
            Some((
                Evidence {
                    analysis: index,
                    sample,
                },
                analysis,
            ))
        })
        .collect::<Vec<_>>();
    let produces = |analysis: &crate::state::AnalysisResult| {
        row.producing_analysis.is_some()
            && analysis
                .provenance()
                .map(|provenance| provenance.authored_source_instance_id())
                == row.producing_analysis
    };
    let measured =
        |(evidence, analysis): &(Evidence, &crate::state::AnalysisResult)| match evidence.sample {
            Some(sample) => analysis.family_metadata.as_ref().is_some_and(|family| {
                family
                    .member_measurements()
                    .get(sample)
                    .is_some_and(|member| member.evidence_for(&row.measurement).is_some())
            }),
            None => analysis
                .measurements
                .iter()
                .any(|measurement| measurement.name.eq_ignore_ascii_case(&row.measurement)),
        };
    at.iter()
        .find(|(_, analysis)| produces(analysis))
        .or_else(|| at.iter().find(|entry| measured(entry)))
        .or(at.first())
        .map(|(evidence, _)| *evidence)
}

/// Whether two family members are the same point: a corner by its label,
/// whichever place it had in its family, anything else by its identity.
fn same_member(left: &FamilyMemberId, right: &FamilyMemberId) -> bool {
    match (left, right) {
        (
            FamilyMemberId::Corner { label: left, .. },
            FamilyMemberId::Corner { label: right, .. },
        ) => left == right,
        _ => left == right,
    }
}

/// Open one analysis of the run Verify reads in Results, narrowed to one
/// member of its family when `sample` names one.
pub(super) fn open_in_results(cx: &mut Cx<'_, SimulateFeature>, evidence: Evidence) {
    if !cx.model.simulation.select_analysis(evidence.analysis) {
        record_action(
            &mut cx.state.verification,
            Err("Nothing was opened: the analysis is no longer in this run.".to_owned()),
        );
        return;
    }
    record_action(&mut cx.state.verification, Ok(()));
    cx.wb.activate(crate::workbench::state::Workspace::Results);
    if let Some(sample) = evidence.sample {
        // The family form seeds its filter when its command opens it, so the
        // member is narrowed to once that command has run.
        cx.wb
            .outbox
            .push(SetFamilyFilter(format!("sample = {}", sample + 1)));
        cx.execute(ids::FAMILY_SLICING);
    }
}

/// One section of what the inspector states about a selection.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FactSection {
    pub(crate) title: String,
    pub(crate) rows: Vec<(String, String)>,
}

/// A cell's status as a word.
pub(crate) const fn status_word(status: CellStatus) -> &'static str {
    match status {
        CellStatus::Pass => "Pass",
        CellStatus::Near => "Near the limit",
        CellStatus::Fail => "Fail",
        CellStatus::Error => "Measurement failed",
        CellStatus::Excluded => "Outside its scope",
        CellStatus::Absent => "No value",
    }
}

/// What the inspector states of the selected Corners cell: the point's
/// conditions, then what it gave the specification and where that came from.
pub(crate) fn point_facts(model: &Model, session: &VerificationSessionState) -> Vec<FactSection> {
    let Some(sheet) = model.simulation.verification_sheet() else {
        return Vec::new();
    };
    let Some(selected) = selected_point(sheet, session) else {
        return Vec::new();
    };
    let point = &sheet.points[selected.point];
    let mut conditions = [
        PointAxis::Process,
        PointAxis::Supply,
        PointAxis::Temperature,
    ]
    .iter()
    .filter_map(|axis| Some((axis_name(axis), axis_value(point, axis)?)))
    .collect::<Vec<_>>();
    conditions.extend(
        point
            .swept_values
            .iter()
            .map(|value| (value.name.clone(), value.value.clone())),
    );
    conditions.push((
        NOMINAL.to_owned(),
        if point.nominal { "Yes" } else { "No" }.to_owned(),
    ));
    let mut sections = vec![FactSection {
        title: point_name(point),
        rows: conditions,
    }];
    if let Some(row_index) = selected.row {
        let row = &sheet.rows[row_index];
        let cell = &row.cells[selected.point];
        let mut rows = vec![(
            "Value".to_owned(),
            cell.value.map_or_else(
                || status_word(cell.status).to_owned(),
                |value| format_value(value, &row.unit),
            ),
        )];
        if let Some((value, margin)) = cell.value.zip(cell.signed_margin) {
            rows.push((
                "Margin".to_owned(),
                difference_text(margin, value, &row.unit),
            ));
        }
        let limit = limit_text(row);
        if !limit.is_empty() {
            rows.push(("Limit".to_owned(), limit));
        }
        if sheet.is_judged() {
            rows.push(("Status".to_owned(), status_word(cell.status).to_owned()));
        }
        if let Some(analysis) = model.simulation.active_run().and_then(|run| {
            let evidence = evidence_at(run, sheet, row_index, selected.point)?;
            run.analyses.get(evidence.analysis)
        }) {
            rows.push(("Analysis".to_owned(), analysis.label.clone()));
        }
        sections.push(FactSection {
            title: row.name.clone(),
            rows,
        });
    }
    sections
}

/// The sheet as Corners shows it, as comma-separated values: values in the
/// specification's unit at full precision, `n/a` outside a specification's
/// scope, `error` where its measurement failed, nothing where a point gave
/// nothing. A point is named by every coordinate it has.
pub(super) fn corners_csv(sheet: &Sheet, transposed: bool, failing_only: bool) -> String {
    use rspice_formats::table::escape_csv_field as field;
    let value = |cell: &SheetCell| match (cell.status, cell.value) {
        (CellStatus::Error, _) => "error".to_owned(),
        (CellStatus::Excluded, _) => "n/a".to_owned(),
        (_, Some(value)) => format!("{value}"),
        (_, None) => String::new(),
    };
    let line = |fields: Vec<String>| {
        fields
            .iter()
            .map(|text| field(text))
            .collect::<Vec<_>>()
            .join(",")
    };
    let mut lines = Vec::new();
    if transposed {
        let mut head = vec!["Point".to_owned()];
        head.extend(sheet.axes.iter().map(axis_name));
        head.push("Status".to_owned());
        head.extend(sheet.rows.iter().map(|row| {
            if row.unit.is_empty() {
                row.name.clone()
            } else {
                format!("{} ({})", row.name, row.unit)
            }
        }));
        lines.push(line(head));
        for point in shown_points(sheet, failing_only) {
            let at = &sheet.points[point];
            let mut fields = vec![point_detail(at)];
            fields.extend(
                sheet
                    .axes
                    .iter()
                    .map(|axis| axis_value(at, axis).unwrap_or_default()),
            );
            fields.push(
                point_status(sheet, point)
                    .map_or("", status_word)
                    .to_owned(),
            );
            fields.extend(sheet.rows.iter().map(|row| value(&row.cells[point])));
            lines.push(line(fields));
        }
    } else {
        let mut head = ["Specification", "Unit", "Limit", "Min", "Max"]
            .map(str::to_owned)
            .to_vec();
        head.extend(sheet.points.iter().map(point_detail));
        lines.push(line(head));
        for row in shown_specifications(sheet, failing_only) {
            let row = &sheet.rows[row];
            let (low, high) = extremes(row)
                .map_or((String::new(), String::new()), |(low, high)| {
                    (format!("{low}"), format!("{high}"))
                });
            let mut fields = vec![
                row.name.clone(),
                row.unit.clone(),
                limit_text(row),
                low,
                high,
            ];
            fields.extend(row.cells.iter().map(value));
            lines.push(line(fields));
        }
    }
    let mut text = lines.join("\r\n");
    text.push_str("\r\n");
    text
}

/// Write the sheet as Corners shows it to a file the reader picks.
fn export_corners(cx: &mut Cx<'_, SimulateFeature>, csv: &str) {
    use crate::workbench::workflows::export_workflow::SaveDialogConfig;

    let result: Result<(), String> = (|| {
        let Some(path) = cx.handles.export_io.show_save_dialog(SaveDialogConfig {
            title: "Export Corners",
            default_name: "corners.csv",
            filter_name: "Comma-separated values",
            filter_extensions: &["csv"],
        })?
        else {
            return Ok(());
        };
        let destination = cx.handles.export_io.observe_destination(&path)?;
        cx.handles
            .export_io
            .write_text_file_observed(&destination, csv)
    })();
    record_action(
        &mut cx.state.verification,
        result.map_err(|error| format!("The corners were not exported: {error}")),
    );
}

/// What the Corners toolbar can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CornerAction {
    Transpose,
    FailingOnly,
    Export,
}

const CORNER_ACTIONS: [CornerAction; 3] = [
    CornerAction::Transpose,
    CornerAction::FailingOnly,
    CornerAction::Export,
];

/// What the reader asked of the table this frame.
#[derive(Debug, Clone, Copy, Default)]
struct Asked {
    /// The cell double-clicked, as a specification and a point of the sheet.
    open: Option<(usize, usize)>,
}

pub(super) fn corners(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let session = &cx.state.verification;
    let (transposed, failing_only) = (session.corners_transposed, session.corners_failing_only);
    let sheet = cx.model.simulation.verification_sheet();
    let points = sheet.map_or(0, |sheet| sheet.points.len());
    let has_rows = sheet.is_some_and(|sheet| points > 1 && !sheet.rows.is_empty());
    let actions = if has_rows {
        vec![
            ToolbarAction::new("Transpose").toggled(transposed),
            ToolbarAction::new("Failing only").toggled(failing_only),
            ToolbarAction::new("Export CSV"),
        ]
    } else {
        Vec::new()
    };
    let summary = sheet.filter(|_| points > 1).map(|sheet| {
        if sheet.rows.is_empty() {
            Summary::plain(format!("{points} points"))
        } else {
            corners_summary(sheet)
        }
    });
    let output = PageToolbar {
        title: VerificationPage::Corners.label(),
        runs: &RunChoice::default(),
        summary,
        actions: &actions,
    }
    .show(ui);
    let action = output
        .action
        .and_then(|index| CORNER_ACTIONS.get(index))
        .copied();
    match action {
        Some(CornerAction::Transpose) => arrange(cx, !transposed),
        Some(CornerAction::FailingOnly) => {
            cx.state.verification.corners_failing_only = !failing_only;
        }
        Some(CornerAction::Export) => {
            if let Some(csv) = sheet.map(|sheet| corners_csv(sheet, transposed, failing_only)) {
                export_corners(cx, &csv);
            }
        }
        None => {}
    }
    action_problem_line(ui, cx.model, cx.state);
    // The table is drawn as the reader just left it arranged, so a turned
    // selection meets the rows it names.
    let session = &cx.state.verification;
    let (transposed, failing_only) = (session.corners_transposed, session.corners_failing_only);

    let Some(sheet) = cx
        .model
        .simulation
        .verification_sheet()
        .filter(|sheet| sheet.points.len() > 1)
    else {
        let text = if points == 1 {
            "This run solved one point. Add process, supply or temperature axes to the run set to \
             verify across corners."
        } else {
            "No run to verify across corners. Add process, supply or temperature axes to the run \
             set, then run the plan."
        };
        if empty_line(ui, text, Some("Open run set")) {
            cx.execute(SimulateCommand::Page(crate::state::SimulationPage::RunSet).key());
        }
        return;
    };
    if sheet.rows.is_empty() {
        let text = format!("No specification to verify across these {points} points.");
        if empty_line(ui, &text, Some("Open specifications")) {
            cx.execute(SimulateCommand::Verify(VerificationPage::Specifications).key());
        }
        return;
    }
    let shown = if transposed {
        shown_points(sheet, failing_only)
    } else {
        shown_specifications(sheet, failing_only)
    };
    if shown.is_empty() {
        let _ = empty_line(
            ui,
            &format!("Nothing fails across these {points} points."),
            None,
        );
        return;
    }
    let state = &mut cx.state.verification.corners_table;
    let asked = if transposed {
        transposed_table(ui, sheet, &shown, state)
    } else {
        plain_table(ui, sheet, &shown, state)
    };
    let opened = asked.open.and_then(|(row, point)| {
        let run = cx.model.simulation.active_run()?;
        let sheet = cx.model.simulation.verification_sheet()?;
        evidence_at(run, sheet, row, point)
    });
    if let Some(evidence) = opened {
        open_in_results(cx, evidence);
    }
}

/// The width a value column needs for its head.
fn value_column_width(ui: &Ui, head: &str) -> f32 {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let measured = ui
        .painter()
        .layout_no_wrap(head.to_owned(), font, t.color.text)
        .size()
        .x;
    (measured + 24.0).max(VALUE_COLUMN_MIN_W)
}

/// The width a body cell needs for `words`, set as the table sets values.
fn cell_text_width(ui: &Ui, words: &str) -> f32 {
    let t = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let measured = ui
        .painter()
        .layout_no_wrap(words.to_owned(), font, t.color.text)
        .size()
        .x;
    measured + 24.0
}

/// A right-aligned value column that shares the width the fixed columns
/// leave equally with its siblings, so two specifications turned into
/// columns stay as wide as each other instead of the last taking the rest.
fn shared_value_column(head: &str, width: f32) -> Column {
    Column::name(head, width).align(Align::Right)
}

/// Specifications down, points across.
fn plain_table(ui: &mut Ui, sheet: &Sheet, rows: &[usize], state: &mut TableState) -> Asked {
    let heads = point_heads(sheet);
    let mut columns = vec![
        Column::name("Specification", 160.0),
        Column::number("Limit", 112.0),
        Column::number("Min", 96.0),
        Column::number("Max", 96.0),
    ];
    columns.extend(sheet.points.iter().zip(&heads.heads).map(|(point, head)| {
        Column::number(head, value_column_width(ui, head)).tooltip(point_detail(point))
    }));
    let keys = rows
        .iter()
        .map(|row| RowKey::Number(*row as u64))
        .collect::<Vec<_>>();
    let response = DataTable::new("verify.corners", &columns, rows.len())
        .state(state)
        .keys(&keys)
        .freeze(LEADING)
        .bands(&heads.bands)
        .cell_selection()
        .max_height(ui.available_height())
        .show(ui, |index| {
            let row = &sheet.rows[rows[index]];
            let number = |value: f64| {
                Cell::number(format_value(value, &row.unit)).sort_by(SortKey::Number(value))
            };
            let (low, high) = extremes(row)
                .map_or((Cell::absent(), Cell::absent()), |(low, high)| {
                    (number(low), number(high))
                });
            let name = Cell::text(&row.name);
            let mut cells = vec![
                if row.name == row.measurement {
                    name
                } else {
                    name.hint(&row.measurement)
                },
                limit_cell(row),
                low,
                high,
            ];
            cells.extend(row.cells.iter().map(|cell| value_cell(cell, &row.unit)));
            cells
        });
    Asked {
        open: response.double_clicked.and_then(|cell| {
            let point = cell.column?.checked_sub(LEADING)?;
            Some((*rows.get(cell.row)?, point))
        }),
    }
}

/// Points down, specifications across.
fn transposed_table(ui: &mut Ui, sheet: &Sheet, points: &[usize], state: &mut TableState) -> Asked {
    let leading = transposed_leading(sheet);
    let last_axis = sheet.axes.len().saturating_sub(1);
    // The last axis's column carries the nominal mark, so it is as wide as
    // the marked value.
    let nominal_words = sheet
        .points
        .iter()
        .find(|point| point.nominal)
        .and_then(|point| {
            let axis = sheet.axes.get(last_axis)?;
            Some(marked(point, axis_value(point, axis)?))
        });
    let mut columns = if sheet.axes.is_empty() {
        vec![Column::name("Point", 160.0)]
    } else {
        sheet
            .axes
            .iter()
            .enumerate()
            .map(|(position, axis)| {
                let head = axis_name(axis);
                let marked_w = nominal_words
                    .as_deref()
                    .filter(|_| position == last_axis)
                    .map_or(0.0, |words| cell_text_width(ui, words));
                match axis {
                    PointAxis::Process => Column::words(head, marked_w.max(96.0)),
                    _ => {
                        let width = value_column_width(ui, &head).max(marked_w);
                        Column::number(head, width)
                    }
                }
            })
            .collect()
    };
    columns.push(Column::words("Status", 72.0));
    columns.extend(sheet.rows.iter().map(|row| {
        let limit = limit_text(row);
        let column = shared_value_column(&row.name, value_column_width(ui, &row.name));
        if limit.is_empty() {
            column
        } else {
            column.tooltip(format!("{} \u{00b7} {limit}", row.name))
        }
    }));
    let keys = points
        .iter()
        .map(|point| RowKey::Number(*point as u64))
        .collect::<Vec<_>>();
    let response = DataTable::new("verify.corners.transposed", &columns, points.len())
        .state(state)
        .keys(&keys)
        .freeze(leading)
        .cell_selection()
        .max_height(ui.available_height())
        .show(ui, |index| {
            let point_index = points[index];
            let point = &sheet.points[point_index];
            let mut cells = if sheet.axes.is_empty() {
                vec![Cell::text(point_name(point)).hint(point_detail(point))]
            } else {
                sheet
                    .axes
                    .iter()
                    .enumerate()
                    .map(|(position, axis)| {
                        let words = axis_value(point, axis).unwrap_or_default();
                        let words = if position == last_axis {
                            marked(point, words)
                        } else {
                            words
                        };
                        let cell = match axis {
                            PointAxis::Process => Cell::text(words),
                            _ => Cell::number(words),
                        };
                        match axis_number(point, axis) {
                            Some(value) => cell.sort_by(SortKey::Number(value)),
                            None => cell,
                        }
                    })
                    .collect()
            };
            cells.push(status_cell(point_status(sheet, point_index)));
            cells.extend(
                sheet
                    .rows
                    .iter()
                    .map(|row| value_cell(&row.cells[point_index], &row.unit)),
            );
            cells
        });
    Asked {
        open: response.double_clicked.and_then(|cell| {
            let row = cell.column?.checked_sub(leading)?;
            (row < sheet.rows.len()).then_some((row, *points.get(cell.row)?))
        }),
    }
}

#[cfg(test)]
mod tests;
