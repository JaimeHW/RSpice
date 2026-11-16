//! What the plan's specifications ask of the run set, and the Verify row each
//! finding opens.

use super::super::session_of;
use super::*;
use rspice_app_runs::simulation::controller::prepared_run::fixtures::{definition, runnable_state};
use rspice_app_state::simulation::plan::AnalysisKind;
use rspice_app_state::state::{MonteCarloSpecificationGate, VerificationPage};

/// A run set with no enabled analysis of `kind`.
fn without(state: &mut AppState, kind: AnalysisKind) {
    let plan = state.model.sim_setup.analysis_plan.as_mut().unwrap();
    let ids = plan
        .instances()
        .iter()
        .filter(|instance| instance.kind() == kind && instance.enabled())
        .map(|instance| instance.id())
        .collect::<Vec<_>>();
    for id in ids {
        plan.set_enabled(id, false).unwrap();
    }
}

/// Follow `remediation` from Results, as a reader of the report would.
fn follow(remediation: PreflightRemediation, selected: Option<&str>) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Results);
    app.state.simulate.selected_specification = selected.map(str::to_owned);
    apply_remediation(&mut app, remediation);
    assert_eq!(app.state.workbench.workspace, Workspace::Verify);
    assert_eq!(
        app.state.simulate.verification_page,
        VerificationPage::Specifications
    );
    app
}

/// An authored measurement no enabled analysis takes is the run's blocker,
/// stated before preparation would stop on it, and it opens that
/// specification's row; enabling the analysis it needs clears it.
#[test]
fn a_specification_no_enabled_analysis_measures_blocks_on_its_row() {
    let mut state = session_of(runnable_state());
    without(&mut state, AnalysisKind::Ac);
    let plan_id = state.model.sim_setup.analysis_plan.as_ref().unwrap().id();
    state
        .model
        .workspace
        .content
        .replace_active_specification_definitions(
            plan_id,
            vec![definition("gain", ".MEAS AC gain FIND VM(out) AT=1k")],
        );

    let report = collect_report(&state);
    let blocker = report
        .blockers
        .iter()
        .find(|issue| issue.check == "Specification measurement")
        .unwrap_or_else(|| panic!("an unmeasurable specification blocks: {report:?}"));
    assert!(blocker.observed.contains("AC"), "{blocker:?}");
    assert_eq!(
        blocker.remediation,
        PreflightRemediation::Specification {
            measurement: Some("gain".to_owned())
        }
    );
    let app = follow(blocker.remediation.clone(), None);
    assert_eq!(
        app.state.simulate.selected_specification.as_deref(),
        Some("gain")
    );

    let plan = state.model.sim_setup.analysis_plan.as_mut().unwrap();
    plan.insert(AnalysisKind::Ac).unwrap();
    let report = collect_report(&state);
    assert!(
        !report
            .blockers
            .iter()
            .any(|issue| issue.check == "Specification measurement"),
        "{report:?}"
    );
}

/// A specification read from an analysis the run set leaves disabled runs,
/// is said, and opens its row.
#[test]
fn a_specification_read_from_a_disabled_analysis_is_advised_on_its_row() {
    let mut state = session_of(runnable_state());
    let plan = state.model.sim_setup.analysis_plan.as_mut().unwrap();
    let (ac, _) = plan.insert(AnalysisKind::Ac).unwrap();
    plan.set_enabled(ac, false).unwrap();
    let plan_id = plan.id();
    let mut reference = definition("gain", "");
    reference.define_measurement = false;
    reference.producing_analysis = Some(ac);
    state
        .model
        .workspace
        .content
        .replace_active_specification_definitions(plan_id, vec![reference]);

    let report = collect_report(&state);
    let advisory = report
        .advisories
        .iter()
        .find(|advisory| advisory.about.0 == "unmeasured-specification")
        .unwrap_or_else(|| panic!("a disabled producer is advised: {report:?}"));
    assert!(
        advisory.message.contains("'gain'") && advisory.message.contains("not measured"),
        "{advisory:?}"
    );
    assert!(
        !report
            .blockers
            .iter()
            .any(|issue| issue.check == "Specification measurement"),
        "the run is sound: {report:?}"
    );
    let app = follow(advisory.remediation.clone().unwrap(), None);
    assert_eq!(
        app.state.simulate.selected_specification.as_deref(),
        Some("gain")
    );
}

/// A Monte Carlo yield gate with no Monte Carlo analysis to judge is advised,
/// and opens the view with no row selected, which is where the gate is set.
#[test]
fn a_yield_gate_with_no_monte_carlo_is_advised_on_the_acceptance() {
    let mut state = session_of(runnable_state());
    without(&mut state, AnalysisKind::MonteCarlo);
    let plan_id = state.model.sim_setup.analysis_plan.as_ref().unwrap().id();
    state
        .model
        .workspace
        .content
        .ensure_active_plan_data(plan_id)
        .specification_policy
        .monte_carlo = MonteCarloSpecificationGate::YieldAtLeast { percent: 99.0 };

    let gate = |state: &AppState| {
        collect_report(state)
            .advisories
            .into_iter()
            .find(|advisory| advisory.about.0 == "monte-carlo-yield-gate")
    };
    let advisory = gate(&state).expect("a gate with nothing to judge is advised");
    assert!(advisory.message.contains("99%"), "{advisory:?}");
    assert_eq!(
        advisory.remediation,
        Some(PreflightRemediation::Specification { measurement: None })
    );
    let app = follow(advisory.remediation.unwrap(), Some("gain"));
    assert_eq!(app.state.simulate.selected_specification, None);

    let plan = state.model.sim_setup.analysis_plan.as_mut().unwrap();
    plan.insert(AnalysisKind::MonteCarlo).unwrap();
    assert_eq!(gate(&state), None);
}
