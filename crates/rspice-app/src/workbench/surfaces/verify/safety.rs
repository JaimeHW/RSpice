//! Electrical safe operating area: every device check one run evaluated.
//!
//! One row per evaluated check, worst first — violations, then checks near
//! their limit, then the rest by how little room they have. A row's margin is
//! its verdict cell; clicking a row selects that device on the schematic. The
//! table reads only the SOA evidence the run retained and validated: a check
//! the run did not evaluate is not a row.

use super::table::{Cell, Column, DataTable, Verdict};
use super::toolbar::{PageToolbar, Summary, ToolbarAction, empty_line};
use super::*;

/// One evaluated check, as the table states it.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct SoaRow {
    pub(super) device: String,
    pub(super) check: &'static str,
    pub(super) observed: f64,
    pub(super) limit: f64,
    pub(super) unit: String,
    pub(super) when_s: f64,
    pub(super) verdict: Verdict,
}

impl SoaRow {
    /// Room left before the limit, as a fraction of it. Negative once broken.
    pub(super) fn margin(&self) -> Option<f64> {
        (self.limit != 0.0).then(|| (self.limit - self.observed) / self.limit.abs())
    }
}

/// The run's evaluated checks, worst first.
pub(super) fn soa_rows(evaluations: &[crate::state::SoaEvaluationEvidence]) -> Vec<SoaRow> {
    let mut rows = evaluations
        .iter()
        .map(|evaluation| SoaRow {
            device: evaluation.device_id.clone(),
            check: soa_parameter_display(evaluation.parameter),
            observed: evaluation.worst_actual_value,
            limit: evaluation.limit_value,
            unit: evaluation.unit.clone(),
            when_s: evaluation.worst_time_s,
            verdict: soa_rule_verdict(evaluation.verdict),
        })
        .collect::<Vec<_>>();
    let severity = |verdict: Verdict| match verdict {
        Verdict::Fail => 0,
        Verdict::Near => 1,
        Verdict::Pass => 2,
    };
    rows.sort_by(|left, right| {
        severity(left.verdict)
            .cmp(&severity(right.verdict))
            .then_with(|| {
                left.margin()
                    .unwrap_or(f64::INFINITY)
                    .total_cmp(&right.margin().unwrap_or(f64::INFINITY))
            })
    });
    rows
}

/// The toolbar's line: the violations and the devices they are on, or the
/// checks near their limit. A clean run says nothing.
pub(super) fn soa_summary(rows: &[SoaRow]) -> Option<Summary> {
    let violations = rows
        .iter()
        .filter(|row| row.verdict == Verdict::Fail)
        .collect::<Vec<_>>();
    if !violations.is_empty() {
        let mut devices = violations
            .iter()
            .map(|row| row.device.to_ascii_uppercase())
            .collect::<Vec<_>>();
        devices.sort();
        devices.dedup();
        return Some(Summary::failing(format!(
            "{} violation{} on {} device{}",
            violations.len(),
            if violations.len() == 1 { "" } else { "s" },
            devices.len(),
            if devices.len() == 1 { "" } else { "s" },
        )));
    }
    let near = rows
        .iter()
        .filter(|row| row.verdict == Verdict::Near)
        .count();
    (near > 0).then(|| {
        Summary::plain(format!(
            "{near} check{} near limit",
            if near == 1 { "" } else { "s" }
        ))
    })
}

/// What the SOA toolbar can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SoaAction {
    Run,
    Edit,
}

pub(super) fn device_safety(ui: &mut Ui, app: &mut RSpiceApp) {
    let runs = run_choice(app, |run| {
        run.analyses.iter().any(|analysis| {
            analysis.analysis_type == crate::state::AnalysisType::Soa && verified_analysis(analysis)
        })
    });
    let run_name = verification_run(app).map(|run| format!("Run {}", run.id));
    let rows =
        latest_validated_analysis(app, crate::state::AnalysisType::Soa).and_then(|analysis| {
            match &analysis.result_payload {
                Some(crate::state::AnalysisResultPayload::Soa { evaluations, .. }) => {
                    Some(soa_rows(evaluations))
                }
                _ => None,
            }
        });
    let actions = [SoaAction::Run, SoaAction::Edit];
    let toolbar_actions = [
        ToolbarAction::new("Run SOA checks"),
        ToolbarAction::new("Edit SOA checks"),
    ];
    let summary = rows.as_deref().and_then(soa_summary);
    let output = PageToolbar {
        title: "Electrical SOA",
        runs: &runs,
        summary,
        actions: &toolbar_actions,
    }
    .show(ui);
    match output.action.and_then(|index| actions.get(index)) {
        Some(SoaAction::Run) => {
            let result = request_analysis_run(app, &[crate::simulation::plan::AnalysisKind::Soa]);
            record_action(&mut app.state.workbench.verification, result);
        }
        Some(SoaAction::Edit) => {
            let result =
                open_analysis_configuration(app, crate::simulation::plan::AnalysisKind::Soa);
            record_action(&mut app.state.workbench.verification, result);
        }
        None => {}
    }
    if let Some(index) = output.run {
        select_verification_run(&mut app.state.simulation, index);
    }
    action_problem_line(ui, app);

    let Some(rows) = rows else {
        let text = run_name.map_or_else(
            || "No SOA results. Add SOA checks to the plan, then run them.".to_owned(),
            |run| format!("No SOA results in {run}. Add SOA checks to the plan, then run them."),
        );
        let _ = empty_line(ui, &text, None);
        return;
    };
    if rows.is_empty() {
        let _ = empty_line(ui, "This SOA run evaluated no checks.", None);
        return;
    }
    let columns = [
        Column::name("Device", 120.0),
        Column::words("Check", 150.0),
        Column::number("Observed", 100.0),
        Column::number("Limit", 100.0),
        Column::number("Margin", 84.0),
        Column::number("When", 96.0),
    ];
    let height = ui.available_height();
    let clicked = DataTable::new("verify.soa.checks", &columns, rows.len())
        .clickable()
        .max_height(height)
        .show(ui, |index| {
            let row = &rows[index];
            vec![
                Cell::text(&row.device),
                Cell::text(row.check),
                Cell::number(format_value(row.observed, &row.unit)),
                Cell::number(format_value(row.limit, &row.unit)),
                row.margin().map_or_else(Cell::absent, |margin| {
                    Cell::number(format!("{:+.1} %", margin * 100.0)).judged(row.verdict)
                }),
                Cell::number(format_value(row.when_s, "s")),
            ]
        })
        .clicked;
    if let Some(index) = clicked {
        let result = cross_probe_soa_device(app, &rows[index].device);
        record_action(&mut app.state.workbench.verification, result);
    }
}

pub(super) fn cross_probe_soa_device(app: &mut RSpiceApp, device_id: &str) -> Result<(), String> {
    let component_id = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.name.eq_ignore_ascii_case(device_id))
        .map(|component| component.id)
        .ok_or_else(|| format!("{device_id} is not on the open schematic"))?;
    app.state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(component_id);
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    Ok(())
}

pub(super) fn soa_parameter_display(parameter: crate::state::SoaParameterEvidence) -> &'static str {
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

/// The verdict an SOA check's margin cell is painted with: a near-limit check
/// in the warning colour, a violated one in the error colour, a pass neutral.
pub(super) fn soa_rule_verdict(verdict: crate::state::SoaRuleVerdictEvidence) -> Verdict {
    use crate::state::SoaRuleVerdictEvidence;
    match verdict {
        SoaRuleVerdictEvidence::Pass => Verdict::Pass,
        SoaRuleVerdictEvidence::Warning => Verdict::Near,
        SoaRuleVerdictEvidence::Violation | SoaRuleVerdictEvidence::Critical => Verdict::Fail,
    }
}
