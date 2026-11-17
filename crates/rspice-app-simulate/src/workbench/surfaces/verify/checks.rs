//! Checks: every check the run Verify reads evaluated, one row each, worst
//! first — its safe-operating-area rules, its `.ASSERT` cards, and its HDL
//! `$warning`, `$error` and `$fatal` records.
//!
//! The rows are the run's shared verification checks, so this view cannot
//! call a run clean that its checks failed. A row states where the check is,
//! what it observed against what limit, when it broke and for how long; the
//! inspector states the rest of the selected one. Only the status is
//! coloured, and only when it is not a pass.

use rspice_core::ModelFinishPoint;
use rspice_core::config::{HdlPoint, HdlSeverity};
use rspice_core::netlist::AssertAnalysis;
use rspice_results::verification::{AssertionOutcome, Check, CheckRow, CheckStatus, Checks};

use super::toolbar::{PageToolbar, RunChoice, Summary, ToolbarAction, action_menu, empty_line};
use super::verdict::{Verdict, VerdictCell};
use super::*;
use crate::ui::widgets::table::{Cell, Column, ContextRequest, DataTable, RowKey, SortKey};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::requests::{RevealExecutedDeck, SoaCheckRef};
use crate::workbench::state::CheckRowOrder;

/// The toolbar's one action.
const VIOLATIONS_ONLY: &str = "Violations only";

/// Where a row's menu or a double-click takes the reader.
#[derive(Debug, Clone, PartialEq)]
enum Leave {
    /// The device, selected on the schematic.
    Schematic(String),
    /// The deck the run executed.
    Deck,
    /// The check's evidence in Results, by its position in the run's checks.
    Results(usize),
}

pub(super) fn checks_view(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let checks = cx.model.simulation.verification_checks();
    let revision = cx.model.simulation.verification_revision();
    let session = &mut cx.state.verification;
    order_rows(
        &mut session.checks_rows,
        checks,
        revision,
        session.checks_violations_only,
    );
    let total = checks.rows.len();
    let actions = [ToolbarAction::new(VIOLATIONS_ONLY).toggled(session.checks_violations_only)];
    let output = PageToolbar {
        title: VerificationPage::Checks.label(),
        runs: &RunChoice::default(),
        summary: (total > 0).then(|| checks_summary(checks)),
        actions: if total > 0 { &actions } else { &[] },
    }
    .show(ui);
    if output.action == Some(0) {
        session.checks_violations_only = !session.checks_violations_only;
        order_rows(
            &mut session.checks_rows,
            checks,
            revision,
            session.checks_violations_only,
        );
    }
    action_problem_line(ui, cx.model, cx.state);
    let session = &mut cx.state.verification;

    if total == 0 {
        let text = if verification_run(cx.model).is_some() {
            "This run evaluated no checks."
        } else {
            "No run yet. Run the plan to evaluate its checks."
        };
        if empty_line(ui, text, Some("Edit SOA checks")) {
            let result =
                open_analysis_configuration(cx, crate::simulation::plan::AnalysisKind::Soa);
            record_action(&mut cx.state.verification, result);
        }
        return;
    }
    if session.checks_rows.rows.is_empty() {
        let _ = empty_line(ui, "No check was violated.", None);
        return;
    }

    let several_points = checks.points.len() > 1;
    let columns = check_columns(several_points);
    let order = &session.checks_rows;
    let response = DataTable::new("verify.checks", &columns, order.rows.len())
        .state(&mut session.checks_table)
        .keys(&order.keys)
        .freeze(1)
        .pin_when_narrow(columns.len() - 1)
        .max_height(ui.available_height())
        .show(ui, |index| {
            row_cells(checks, &checks.rows[order.rows[index]], several_points)
        });

    let mut leave = response
        .double_clicked
        .map(|cell| Leave::Results(order.rows[cell.row]));
    let menu = |row: usize| {
        order
            .rows
            .get(row)
            .map_or_else(Vec::new, |index| menu_for(&checks.rows[*index], *index))
    };
    if let Some((row, choice)) = row_menu(ui, "verify.checks", response.context_menu, menu) {
        leave = menu(row).into_iter().nth(choice).map(|(_, leave)| leave);
    }
    if let Some(leave) = leave {
        let result = match leave {
            Leave::Schematic(device) => cross_probe_soa_device(cx, &device),
            Leave::Deck => show_in_deck(cx),
            Leave::Results(index) => open_in_results(cx, index),
        };
        record_action(&mut cx.state.verification, result);
    }
}

/// The check selected in the Checks view: none while the shown order is from
/// an earlier reading of the run's checks.
pub(crate) fn selected_check<'m>(
    model: &'m Model,
    session: &VerificationSessionState,
) -> Option<&'m CheckRow> {
    let order = &session.checks_rows;
    if order.basis.map(|(revision, _)| revision) != Some(model.simulation.verification_revision()) {
        return None;
    }
    let index = *order.rows.get(session.checks_table.selected_row()?)?;
    model.simulation.verification_checks().rows.get(index)
}

/// Select the SOA check `check` names in the Checks view, showing every check
/// again when the violations filter hides it. Whether the run Verify reads
/// holds that check; when it does not, the selection is left as it was.
pub(crate) fn select_soa_check(
    model: &Model,
    session: &mut VerificationSessionState,
    check: &SoaCheckRef,
) -> bool {
    let checks = model.simulation.verification_checks();
    let Some(index) = checks.rows.iter().position(|row| {
        row.analysis == check.analysis
            && matches!(&row.check, Check::Soa(soa)
                if soa.device == check.device && soa.parameter == check.parameter)
    }) else {
        return false;
    };
    if session.checks_violations_only && !checks.rows[index].status.is_violation() {
        session.checks_violations_only = false;
    }
    let revision = model.simulation.verification_revision();
    order_rows(
        &mut session.checks_rows,
        checks,
        revision,
        session.checks_violations_only,
    );
    let order = &session.checks_rows;
    let Some(position) = order.rows.iter().position(|row| *row == index) else {
        return false;
    };
    session
        .checks_table
        .select(position, order.keys.get(position).cloned());
    true
}

/// Build the shown order again when the checks or the filter changed since
/// it was built; otherwise leave it as it is.
pub(crate) fn order_rows(
    order: &mut CheckRowOrder,
    checks: &Checks,
    revision: u64,
    violations_only: bool,
) {
    if order.basis == Some((revision, violations_only)) {
        return;
    }
    let mut rows = (0..checks.rows.len())
        .filter(|index| !violations_only || checks.rows[*index].status.is_violation())
        .collect::<Vec<_>>();
    // Worst status first; within one status the SOA rule with the least room
    // first, and otherwise the run's own order.
    rows.sort_by(|left, right| {
        let (left, right) = (&checks.rows[*left], &checks.rows[*right]);
        right
            .status
            .cmp(&left.status)
            .then_with(|| room(left).total_cmp(&room(right)))
    });
    let mut seen = std::collections::HashMap::<String, usize>::new();
    let keys = rows
        .iter()
        .map(|index| {
            let identity = check_identity(&checks.rows[*index]);
            let repeat = seen.entry(identity.clone()).or_default();
            *repeat += 1;
            RowKey::Text(if *repeat == 1 {
                identity
            } else {
                format!("{identity} #{repeat}")
            })
        })
        .collect();
    *order = CheckRowOrder {
        basis: Some((revision, violations_only)),
        rows,
        keys,
    };
}

/// What a check is, in words that name the same check after the run's checks
/// are read again.
fn check_identity(row: &CheckRow) -> String {
    let analysis = row.analysis;
    match &row.check {
        Check::Soa(soa) => format!("{analysis} soa {} {:?}", soa.device, soa.parameter),
        Check::Assertion(assertion) => {
            let start = match &assertion.outcome {
                AssertionOutcome::Violated { start, .. } => format!("{start:?}"),
                AssertionOutcome::Held => "held".to_owned(),
                AssertionOutcome::NotEvaluated(_) => "not evaluated".to_owned(),
                AssertionOutcome::SamplesAfterStop { .. } => "after stop".to_owned(),
            };
            format!(
                "{analysis} assert {} {} {start}",
                assertion.name,
                assertion.instance.as_deref().unwrap_or_default()
            )
        }
        Check::Hdl { origin, at, .. } => {
            format!("{analysis} hdl {}:{} {at:?}", origin.file, origin.line)
        }
        Check::Finish(_) => format!("{analysis} finish"),
    }
}

/// An SOA rule's room before its limit, so the nearest breaks sort first;
/// every other check sorts after it, in the run's order.
fn room(row: &CheckRow) -> f64 {
    match &row.check {
        Check::Soa(soa) => soa.margin.unwrap_or(f64::INFINITY),
        _ => f64::INFINITY,
    }
}

/// The toolbar's line: how many checks were violated and how many warned, out
/// of how many, behind a failure's mark when one was violated and a warning's
/// when one only warned. A warning is not a violation, so it is counted apart.
pub(crate) fn checks_summary(checks: &Checks) -> Summary {
    let counts = checks.counts();
    let plural =
        |count: usize, word: &str| format!("{count} {word}{}", if count == 1 { "" } else { "s" });
    let warnings = (counts.warnings > 0).then(|| plural(counts.warnings, "warning"));
    let text = if counts.violations == 0 {
        let clean = format!("{}, none violated", plural(counts.total, "check"));
        match warnings {
            Some(warnings) => format!("{clean} \u{00b7} {warnings}"),
            None => clean,
        }
    } else {
        [Some(plural(counts.violations, "violation")), warnings]
            .into_iter()
            .flatten()
            .chain([plural(counts.total, "check")])
            .collect::<Vec<_>>()
            .join(" \u{00b7} ")
    };
    if counts.violations > 0 {
        Summary::failing(text)
    } else if counts.warnings > 0 {
        Summary::warning(text)
    } else {
        Summary::plain(text)
    }
}

fn check_columns(several_points: bool) -> Vec<Column> {
    let mut columns = vec![
        Column::name("Target", 90.0),
        Column::name("Check", 110.0),
        Column::number("Observed", 88.0),
        Column::number("Limit", 88.0),
        Column::number("Margin", 72.0),
        Column::number("When", 80.0),
        Column::number("Duration", 80.0),
    ];
    if several_points {
        columns.push(Column::name("Point", 100.0));
    }
    columns.push(Column::words("Status", 72.0));
    columns
}

/// The word a status column states.
pub(crate) fn status_word(status: CheckStatus) -> &'static str {
    match status {
        CheckStatus::Pass => "Pass",
        CheckStatus::Info => "Info",
        CheckStatus::Warning => "Warning",
        CheckStatus::Error => "Error",
        CheckStatus::Fail => "Fail",
    }
}

/// A status cell: a failure or an error in the error tone, a warning in the
/// warning tone, a pass and a notice neutral. A sort puts the worst first.
fn status_cell(status: CheckStatus) -> Cell {
    let cell = Cell::text(status_word(status));
    let cell = match status {
        CheckStatus::Pass | CheckStatus::Info => cell,
        CheckStatus::Warning => cell.judged(Verdict::Near),
        CheckStatus::Error | CheckStatus::Fail => cell.judged(Verdict::Fail),
    };
    cell.sort_by(SortKey::Status(status as u8))
}

/// A time, or the swept value a DC check broke at.
fn axis_cell(analysis: AssertAnalysis, value: Option<f64>) -> Cell {
    match (analysis, value) {
        (AssertAnalysis::Tran, Some(time)) => seconds_cell(time),
        (AssertAnalysis::Dc, Some(value)) => {
            Cell::number(format_scalar(value)).sort_by(SortKey::Number(value))
        }
        _ => Cell::absent(),
    }
}

fn seconds_cell(seconds: f64) -> Cell {
    Cell::number(format_value(seconds, "s")).sort_by(SortKey::Number(seconds))
}

/// When an HDL record was reported.
fn hdl_when(at: HdlPoint) -> Cell {
    match at {
        HdlPoint::Transient { time } => seconds_cell(time),
        HdlPoint::DcSweep { value } => {
            Cell::number(format_scalar(value)).sort_by(SortKey::Number(value))
        }
        HdlPoint::OperatingPoint => Cell::text("OP"),
        HdlPoint::Initialization => Cell::text("Initial"),
    }
}

/// When a model ended the run.
fn finish_when(point: ModelFinishPoint) -> Cell {
    match point {
        ModelFinishPoint::Transient { time } => seconds_cell(time),
        ModelFinishPoint::DcSweep { value } => {
            Cell::number(format_scalar(value)).sort_by(SortKey::Number(value))
        }
        ModelFinishPoint::Frequency { frequency } => {
            Cell::number(format_value(frequency, "Hz")).sort_by(SortKey::Number(frequency))
        }
        ModelFinishPoint::OperatingPoint => Cell::text("OP"),
        ModelFinishPoint::Initialization => Cell::text("Initial"),
    }
}

/// The task an HDL record was reported by, as it is written.
pub(crate) fn severity_task(severity: HdlSeverity) -> &'static str {
    match severity {
        HdlSeverity::Info => "$info",
        HdlSeverity::Warning => "$warning",
        HdlSeverity::Error => "$error",
        HdlSeverity::Fatal => "$fatal",
    }
}

/// Words a cell names, or nothing when there are none.
fn name_cell(name: &str) -> Cell {
    if name.trim().is_empty() {
        Cell::absent()
    } else {
        Cell::text(name)
    }
}

/// One row's cells, in the columns' order.
fn row_cells(checks: &Checks, row: &CheckRow, several_points: bool) -> Vec<Cell> {
    let [target, check, observed, limit, margin, when, duration] = match &row.check {
        Check::Soa(soa) => [
            name_cell(&soa.device),
            Cell::text(soa_parameter_display(soa.parameter)),
            Cell::number(format_value(soa.observed, &soa.unit))
                .sort_by(SortKey::Number(soa.observed)),
            Cell::number(format_value(soa.limit, &soa.unit)).sort_by(SortKey::Number(soa.limit)),
            soa.margin.map_or_else(Cell::absent, |margin| {
                Cell::number(format!("{:+.1} %", margin * 100.0)).sort_by(SortKey::Number(margin))
            }),
            seconds_cell(soa.worst_time_s),
            soa.duration
                .filter(|duration| duration.total_exceedance_s > 0.0)
                .map_or_else(Cell::absent, |duration| {
                    seconds_cell(duration.total_exceedance_s)
                }),
        ],
        Check::Assertion(assertion) => {
            let target = name_cell(
                assertion
                    .instance
                    .as_deref()
                    .unwrap_or(assertion.target.as_str()),
            );
            let check = Cell::text(&assertion.name);
            match &assertion.outcome {
                AssertionOutcome::Violated {
                    start,
                    duration,
                    extreme,
                    ..
                } => [
                    target,
                    check,
                    extreme.map_or_else(Cell::absent, |extreme| {
                        Cell::number(format_scalar(extreme)).sort_by(SortKey::Number(extreme))
                    }),
                    Cell::absent(),
                    Cell::absent(),
                    axis_cell(assertion.analysis, *start),
                    duration.map_or_else(Cell::absent, seconds_cell),
                ],
                AssertionOutcome::NotEvaluated(reason) => [
                    target,
                    check.hint(reason.clone()),
                    Cell::absent(),
                    Cell::absent(),
                    Cell::absent(),
                    Cell::absent(),
                    Cell::absent(),
                ],
                AssertionOutcome::SamplesAfterStop { count, first_time } => [
                    target,
                    check.hint(format!(
                        "{count} expected output{} after the stop time",
                        if *count == 1 { "" } else { "s" }
                    )),
                    Cell::absent(),
                    Cell::absent(),
                    Cell::absent(),
                    seconds_cell(*first_time),
                    Cell::absent(),
                ],
                AssertionOutcome::Held => [
                    target,
                    check,
                    Cell::absent(),
                    Cell::absent(),
                    Cell::absent(),
                    Cell::absent(),
                    Cell::absent(),
                ],
            }
        }
        Check::Hdl {
            severity,
            text,
            at,
            origin,
        } => [
            name_cell(&origin.scope),
            Cell::text(severity_task(*severity)).hint(text.trim()),
            Cell::absent(),
            Cell::absent(),
            Cell::absent(),
            hdl_when(*at),
            Cell::absent(),
        ],
        Check::Finish(finish) => {
            let check = Cell::text(finish.task.name());
            [
                name_cell(&finish.instance),
                match &finish.message {
                    Some(message) => check.hint(message.trim()),
                    None => check,
                },
                Cell::absent(),
                Cell::absent(),
                Cell::absent(),
                finish_when(finish.point),
                Cell::absent(),
            ]
        }
    };
    let mut cells = vec![target, check, observed, limit, margin, when, duration];
    if several_points {
        cells.push(
            row.point
                .and_then(|point| checks.points.get(point))
                .map_or_else(Cell::absent, |point| Cell::text(&point.label)),
        );
    }
    cells.push(status_cell(row.status));
    cells
}

/// What a row's context menu offers, in order: the place the check is, then
/// its evidence in Results.
fn menu_for(row: &CheckRow, index: usize) -> Vec<(&'static str, Leave)> {
    let place = match &row.check {
        Check::Soa(soa) => ("Show in schematic", Leave::Schematic(soa.device.clone())),
        Check::Assertion(_) | Check::Hdl { .. } | Check::Finish(_) => ("Show in deck", Leave::Deck),
    };
    vec![place, ("Open in Results", Leave::Results(index))]
}

/// The context menu of a table row: opened at the place the reader asked for
/// it and kept open until an item is taken or the reader clicks elsewhere.
/// Returns the row it is open on and the index of the item taken.
pub(super) fn row_menu<T>(
    ui: &Ui,
    table: &str,
    request: Option<ContextRequest>,
    items: impl FnOnce(usize) -> Vec<(&'static str, T)>,
) -> Option<(usize, usize)> {
    let id = ui.id().with((table, "row menu"));
    let at = id.with("at");
    if let Some(request) = request {
        ui.ctx().data_mut(|data| {
            data.insert_temp(at, (request.cell.row, request.position));
        });
    }
    let (row, position) = ui
        .ctx()
        .data(|data| data.get_temp::<(usize, egui::Pos2)>(at))?;
    if request.is_none() && !egui::Popup::is_id_open(ui.ctx(), id) {
        return None;
    }
    let actions = items(row)
        .into_iter()
        .map(|(label, _)| ToolbarAction::new(label))
        .collect::<Vec<_>>();
    action_menu(ui.ctx(), id, position, request.is_some(), &actions).map(|choice| (row, choice))
}

/// Open the deck the run executed, at the source the engine read.
fn show_in_deck(cx: &mut Cx<'_, SimulateFeature>) -> Result<(), String> {
    let run = verification_run(cx.model).ok_or("There is no run to read.")?;
    cx.wb.outbox.push(RevealExecutedDeck {
        seq: run.id,
        close_manager_on_success: false,
    });
    Ok(())
}

/// Open a check's evidence in Results: an SOA rule in the safe-operating-area
/// viewer, an assertion's checked waveform in the waveform viewer, and an HDL
/// record's analysis there too.
fn open_in_results(cx: &mut Cx<'_, SimulateFeature>, index: usize) -> Result<(), String> {
    let row = cx
        .model
        .simulation
        .verification_checks()
        .rows
        .get(index)
        .ok_or("That check is no longer in the run.")?;
    let analysis = row.analysis;
    let waveform = match &row.check {
        Check::Soa(_) => {
            if !cx.model.simulation.select_analysis(analysis) {
                return Err("The run no longer holds that analysis.".to_owned());
            }
            cx.execute(ids::RESULT_SAFE_OPERATING_AREA);
            return Ok(());
        }
        Check::Assertion(assertion) => cx
            .model
            .simulation
            .active_run()
            .and_then(|run| run.analyses.get(analysis))
            .and_then(|result| {
                result
                    .waveforms
                    .iter()
                    .position(|waveform| waveform.name.eq_ignore_ascii_case(&assertion.target))
            }),
        Check::Hdl { .. } | Check::Finish(_) => None,
    };
    match waveform {
        Some(waveform) => {
            crate::workbench::surfaces::simulate::page_outputs::open_materialized_traces(
                cx,
                analysis,
                &[waveform],
            );
        }
        None => {
            if !cx.model.simulation.select_analysis(analysis) {
                return Err("The run no longer holds that analysis.".to_owned());
            }
            cx.execute(ids::RESULT_WAVEFORMS);
        }
    }
    Ok(())
}

pub(super) fn cross_probe_soa_device(
    cx: &mut Cx<'_, SimulateFeature>,
    device_id: &str,
) -> Result<(), String> {
    let component_id = cx
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.name.eq_ignore_ascii_case(device_id))
        .map(|component| component.id)
        .ok_or_else(|| format!("{device_id} is not on the open schematic"))?;
    cx.model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(component_id);
    cx.wb.activate(crate::workbench::state::Workspace::Design);
    Ok(())
}

pub(crate) fn soa_parameter_display(parameter: crate::state::SoaParameterEvidence) -> &'static str {
    use crate::state::SoaParameterEvidence;
    match parameter {
        SoaParameterEvidence::GateSourceVoltage => "VGS max",
        SoaParameterEvidence::DrainSourceVoltage => "VDS max",
        SoaParameterEvidence::GateDrainVoltage => "VGD max",
        SoaParameterEvidence::BaseEmitterVoltage => "VBE max",
        SoaParameterEvidence::CollectorEmitterVoltage => "VCE max",
        SoaParameterEvidence::BaseCollectorVoltage => "VBC max",
        SoaParameterEvidence::DrainCurrent => "ID max",
        SoaParameterEvidence::CollectorCurrent => "IC max",
        SoaParameterEvidence::PowerDissipation => "Power max",
        SoaParameterEvidence::Temperature => "Temperature max",
        SoaParameterEvidence::GateSourceVoltagePositive => "Vgs + max",
        SoaParameterEvidence::GateSourceVoltageNegative => "Vgs − max",
        SoaParameterEvidence::DrainSourceVoltagePositive => "Vds + max",
        SoaParameterEvidence::DrainSourceVoltageNegative => "Vds − max",
        SoaParameterEvidence::GateDrainVoltagePositive => "Vgd + max",
        SoaParameterEvidence::GateDrainVoltageNegative => "Vgd − max",
        SoaParameterEvidence::BaseEmitterVoltagePositive => "Vbe + max",
        SoaParameterEvidence::BaseEmitterVoltageNegative => "Vbe − max",
        SoaParameterEvidence::CollectorEmitterVoltagePositive => "Vce + max",
        SoaParameterEvidence::CollectorEmitterVoltageNegative => "Vce − max",
        SoaParameterEvidence::BaseCollectorVoltagePositive => "Vbc + max",
        SoaParameterEvidence::BaseCollectorVoltageNegative => "Vbc − max",
        SoaParameterEvidence::DrainCurrentPositive => "Id + max",
        SoaParameterEvidence::DrainCurrentNegative => "Id − max",
        SoaParameterEvidence::CollectorCurrentPositive => "Ic + max",
        SoaParameterEvidence::CollectorCurrentNegative => "Ic − max",
        SoaParameterEvidence::GateCurrent => "Ig max",
        SoaParameterEvidence::GateCurrentPositive => "Ig + max",
        SoaParameterEvidence::GateCurrentNegative => "Ig − max",
        SoaParameterEvidence::SourceCurrent => "Is max",
        SoaParameterEvidence::SourceCurrentPositive => "Is + max",
        SoaParameterEvidence::SourceCurrentNegative => "Is − max",
        SoaParameterEvidence::BaseCurrent => "Ib max",
        SoaParameterEvidence::BaseCurrentPositive => "Ib + max",
        SoaParameterEvidence::BaseCurrentNegative => "Ib − max",
        SoaParameterEvidence::EmitterCurrent => "Ie max",
        SoaParameterEvidence::EmitterCurrentPositive => "Ie + max",
        SoaParameterEvidence::EmitterCurrentNegative => "Ie − max",
        SoaParameterEvidence::CollectorSubstrateVoltage => "Collector to substrate voltage max",
        SoaParameterEvidence::CollectorSubstrateVoltagePositive => {
            "Collector to substrate voltage positive max"
        }
        SoaParameterEvidence::CollectorSubstrateVoltageNegative => {
            "Collector to substrate voltage negative max"
        }
        SoaParameterEvidence::BaseSubstrateVoltage => "Base to substrate voltage max",
        SoaParameterEvidence::BaseSubstrateVoltagePositive => {
            "Base to substrate voltage positive max"
        }
        SoaParameterEvidence::BaseSubstrateVoltageNegative => {
            "Base to substrate voltage negative max"
        }
        SoaParameterEvidence::EmitterSubstrateVoltage => "Emitter to substrate voltage max",
        SoaParameterEvidence::EmitterSubstrateVoltagePositive => {
            "Emitter to substrate voltage positive max"
        }
        SoaParameterEvidence::EmitterSubstrateVoltageNegative => {
            "Emitter to substrate voltage negative max"
        }
        SoaParameterEvidence::SubstrateCurrent => "Substrate current max",
        SoaParameterEvidence::SubstrateCurrentPositive => "Substrate current positive max",
        SoaParameterEvidence::SubstrateCurrentNegative => "Substrate current negative max",
        SoaParameterEvidence::AnodeCathodeVoltage => "Diode anode to cathode voltage max",
        SoaParameterEvidence::AnodeCathodeVoltagePositive => {
            "Diode anode to cathode voltage positive max"
        }
        SoaParameterEvidence::AnodeCathodeVoltageNegative => {
            "Diode anode to cathode voltage negative max"
        }
        SoaParameterEvidence::AnodeCurrent => "Diode anode current max",
        SoaParameterEvidence::AnodeCurrentPositive => "Diode anode current positive max",
        SoaParameterEvidence::AnodeCurrentNegative => "Diode anode current negative max",

        SoaParameterEvidence::BodySourceVoltage => "Vbs max",
        SoaParameterEvidence::BodySourceVoltagePositive => "Vbs positive max",
        SoaParameterEvidence::BodySourceVoltageNegative => "Vbs negative max",
        SoaParameterEvidence::BodyDrainVoltage => "Vbd max",
        SoaParameterEvidence::BodyDrainVoltagePositive => "Vbd positive max",
        SoaParameterEvidence::BodyDrainVoltageNegative => "Vbd negative max",
        SoaParameterEvidence::GateBodyVoltage => "Vgb max",
        SoaParameterEvidence::GateBodyVoltagePositive => "Vgb positive max",
        SoaParameterEvidence::GateBodyVoltageNegative => "Vgb negative max",
        SoaParameterEvidence::BulkCurrent => "Bulk / body-contact current max",
        SoaParameterEvidence::BulkCurrentPositive => "Bulk / body-contact current positive max",
        SoaParameterEvidence::BulkCurrentNegative => "Bulk / body-contact current negative max",
        SoaParameterEvidence::BackgateSourceVoltage => "Ves (back gate to source) max",
        SoaParameterEvidence::BackgateSourceVoltagePositive => {
            "Ves (back gate to source) positive max"
        }
        SoaParameterEvidence::BackgateSourceVoltageNegative => {
            "Ves (back gate to source) negative max"
        }
        SoaParameterEvidence::BackgateDrainVoltage => "Ved (back gate to drain) max",
        SoaParameterEvidence::BackgateDrainVoltagePositive => {
            "Ved (back gate to drain) positive max"
        }
        SoaParameterEvidence::BackgateDrainVoltageNegative => {
            "Ved (back gate to drain) negative max"
        }
        SoaParameterEvidence::GateBackgateVoltage => "Vge (gate to back gate) max",
        SoaParameterEvidence::GateBackgateVoltagePositive => "Vge (gate to back gate) positive max",
        SoaParameterEvidence::GateBackgateVoltageNegative => "Vge (gate to back gate) negative max",
        SoaParameterEvidence::BackgateCurrent => "Back-gate current max",
        SoaParameterEvidence::BackgateCurrentPositive => "Back-gate current positive max",
        SoaParameterEvidence::BackgateCurrentNegative => "Back-gate current negative max",
        SoaParameterEvidence::BodyBackgateVoltage => "Body contact to back gate voltage max",
        SoaParameterEvidence::BodyBackgateVoltagePositive => {
            "Body contact to back gate voltage positive max"
        }
        SoaParameterEvidence::BodyBackgateVoltageNegative => {
            "Body contact to back gate voltage negative max"
        }
    }
}

#[cfg(test)]
mod tests;
