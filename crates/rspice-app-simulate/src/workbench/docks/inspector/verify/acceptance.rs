//! The Specifications view's inspector with no specification selected: how
//! the plan accepts a run against its specifications, and the run it judges.
//!
//! The four acceptance settings are the plan's, frozen into every run it
//! prepares: changing one is a plan change, and a verdict a run already holds
//! keeps the policy it was judged under. The evidence rows say which run the
//! view judges and whether the limits, or anything else in the project, moved
//! since it ran.

use egui::Ui;
use rspice_results::verification::LimitsSinceRun;
use rspice_ui_kit::panels::inspector::muted_inspector_copy;
use rspice_ui_kit::panels::{
    property_row, property_row_combo, property_row_wrapped, section_header,
};

use crate::state::{
    MissingMeasurementPolicy, MonteCarloSpecificationGate, NominalFailurePolicy,
    RegressionSpecificationPolicy, SimulationRunLifecycle, SpecificationPolicy,
};
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::SimulateFeature;
use crate::workbench::surfaces::verify::specification_edit::{edit_policy, monte_carlo_gate_label};

use super::compact_readout;

const NOMINAL: [(NominalFailurePolicy, &str); 2] = [
    (NominalFailurePolicy::Block, "Block plan acceptance"),
    (
        NominalFailurePolicy::RecordDisposition,
        "Record disposition for review",
    ),
];

const MISSING: [(MissingMeasurementPolicy, &str); 2] = [
    (MissingMeasurementPolicy::FailClosed, "Fail closed"),
    (
        MissingMeasurementPolicy::ReportUnmapped,
        "Report unmapped for review",
    ),
];

const REGRESSION: [(RegressionSpecificationPolicy, &str); 2] = [
    (
        RegressionSpecificationPolicy::LimitAndWaveform,
        "Limits and waveforms",
    ),
    (RegressionSpecificationPolicy::LimitOnly, "Limits only"),
];

/// The yield gates offered, as percentages; `None` gates nothing.
const MONTE_CARLO: [Option<f64>; 4] = [None, Some(95.0), Some(99.0), Some(99.9)];

pub(super) fn acceptance(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    section_header(ui, "Acceptance", None);
    let Ok(plan_id) = cx
        .model
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.id())
    else {
        muted_inspector_copy(ui, "The simulation plan is not ready.");
        evidence(ui, cx.model);
        return;
    };
    let policy = cx
        .model
        .workspace
        .content
        .plan_data(plan_id)
        .map(|payload| payload.specification_policy.clone())
        .unwrap_or_default();
    let mut edited = policy.clone();
    choice(ui, "Nominal failure", &NOMINAL, &mut edited.nominal_failure);
    choice(
        ui,
        "Missing measurement",
        &MISSING,
        &mut edited.missing_measurement,
    );
    monte_carlo_choice(ui, &mut edited.monte_carlo);
    choice(ui, "Regression check", &REGRESSION, &mut edited.regression);
    if !edited.bitwise_eq(&policy) {
        commit(cx, edited);
    }
    evidence(ui, cx.model);
}

fn commit(cx: &mut Cx<'_, SimulateFeature>, policy: SpecificationPolicy) {
    cx.state.verification.action_problem = edit_policy(cx, policy)
        .err()
        .map(|problem| format!("The acceptance policy was not changed: {problem}."));
}

/// One setting as a choice between its options, written back when another is
/// picked.
fn choice<T: Copy + PartialEq>(ui: &mut Ui, label: &str, options: &[(T, &str)], value: &mut T) {
    let mut selected = options
        .iter()
        .find(|(option, _)| *option == *value)
        .map(|(_, words)| (*words).to_owned())
        .unwrap_or_default();
    let shown = options
        .iter()
        .map(|(_, words)| ((*words).to_owned(), (*words).to_owned()))
        .collect::<Vec<_>>();
    if property_row_combo(
        ui,
        label,
        ("verify.acceptance", label),
        &mut selected,
        &shown,
        true,
    ) && let Some((option, _)) = options.iter().find(|(_, words)| *words == selected)
    {
        *value = *option;
    }
}

/// The Monte Carlo gate, as one of the offered yields — or the plan's own,
/// when it holds one the list does not offer.
fn monte_carlo_choice(ui: &mut Ui, gate: &mut MonteCarloSpecificationGate) {
    let gates = MONTE_CARLO.map(|percent| {
        percent.map_or(MonteCarloSpecificationGate::NotGated, |percent| {
            MonteCarloSpecificationGate::YieldAtLeast { percent }
        })
    });
    let mut options = gates
        .iter()
        .map(|gate| {
            let words = monte_carlo_gate_label(gate);
            (words.clone(), words)
        })
        .collect::<Vec<_>>();
    let mut selected = monte_carlo_gate_label(gate);
    if !options.iter().any(|(words, _)| *words == selected) {
        options.push((selected.clone(), selected.clone()));
    }
    if property_row_combo(
        ui,
        "Monte Carlo gate",
        ("verify.acceptance", "Monte Carlo gate"),
        &mut selected,
        &options,
        true,
    ) && let Some(picked) = gates
        .iter()
        .find(|candidate| monte_carlo_gate_label(candidate) == selected)
    {
        *gate = picked.clone();
    }
}

/// The run the view judges, and what moved since it ran.
fn evidence(ui: &mut Ui, model: &Model) {
    section_header(ui, "Evidence", None);
    let Some(run) = model.simulation.active_run() else {
        muted_inspector_copy(ui, "No run is selected.");
        return;
    };
    property_row(ui, "Run", &run.label);
    let finished = match run.lifecycle {
        SimulationRunLifecycle::Completed => {
            format!("Completed in {}", compact_readout(run.elapsed_time, "s"))
        }
        SimulationRunLifecycle::Failed => "Failed".to_owned(),
        SimulationRunLifecycle::Aborted => "Cancelled".to_owned(),
        SimulationRunLifecycle::Interrupted => "Interrupted".to_owned(),
        SimulationRunLifecycle::Preparing
        | SimulationRunLifecycle::Running
        | SimulationRunLifecycle::Cancelling => "Running".to_owned(),
        SimulationRunLifecycle::LegacyUnknown => "Not recorded".to_owned(),
    };
    property_row_wrapped(ui, "Finished", &finished);
    let limits = match model
        .simulation
        .verification_sheet()
        .map(|sheet| sheet.summary.limits)
    {
        Some(LimitsSinceRun::Edited) => "Edited since this run",
        Some(LimitsSinceRun::Unchanged) => "As this run was judged",
        Some(LimitsSinceRun::NotSealed) | None => "Not sealed with this run",
    };
    property_row_wrapped(ui, "Limits", limits);
    let project = run
        .prepared_receipt()
        .map_or("Not recorded with this run", |receipt| {
            if receipt.project_revision() == model.workspace.content.project.revision() {
                "Unchanged since this run"
            } else {
                "Changed since this run"
            }
        });
    property_row_wrapped(ui, "Plan and design", project);
}
