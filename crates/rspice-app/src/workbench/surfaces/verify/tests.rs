//! Tests for regression export and tolerance contracts.
//!
//! The cases are fail-closed: an empty matrix still exports a blocked case,
//! XML control characters are refused rather than escaped away, and a receipt
//! whose tolerance digest does not match its contract is rejected.

use super::raster::sealed_run;
use super::tuning::{revert_tuning_session, tuning_commit_block_reason, tuning_is_dirty};
use super::*;
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use crate::state::{AnalysisResult, AnalysisType, SimulationRun};

fn attributed(analysis: AnalysisResult) -> AnalysisResult {
    attributed_with_id(analysis, AnalysisInstanceId::new())
}

fn attributed_with_id(
    analysis: AnalysisResult,
    source_instance_id: AnalysisInstanceId,
) -> AnalysisResult {
    analysis.with_provenance(
        crate::state::AnalysisResultProvenance::new(
            source_instance_id,
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0x5a; 32]),
            Vec::new(),
        )
        .expect("test provenance is internally valid"),
    )
}

#[test]
fn golden_regression_authority_is_scoped_to_the_active_plan() {
    let owner = crate::product::SimulationPlanId::new();
    let other = crate::product::SimulationPlanId::new();
    let run = sealed_run(
        1,
        owner,
        AnalysisInstanceId::new(),
        2,
        AnalysisResult::new(1, AnalysisType::Ac, "AC"),
    );

    assert!(validate_regression_run_for_plan(&run, owner).is_ok());
    let error = validate_regression_run_for_plan(&run, other).unwrap_err();
    assert!(error.contains(&owner.to_string()));
    assert!(error.contains(&other.to_string()));
}

#[test]
fn selecting_a_golden_regression_baseline_pins_its_dataset_atomically() {
    let mut app = RSpiceApp::test_instance();
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let source_id = AnalysisInstanceId::new();
    let baseline = sealed_run(
        1,
        plan_id,
        source_id,
        2,
        AnalysisResult::new(1, AnalysisType::Ac, "baseline")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)]),
    );
    let baseline_id = baseline.run_id;
    let candidate = sealed_run(
        2,
        plan_id,
        source_id,
        2,
        AnalysisResult::new(1, AnalysisType::Ac, "candidate")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)]),
    );
    app.state.simulation.retained.runs = vec![candidate, baseline].into();
    assert!(app.state.simulation.select_run(0));

    commit_regression_baseline(&mut app, baseline_id).expect("baseline commits");

    assert!(
        app.state
            .simulation
            .retained
            .run_by_stable_id(baseline_id)
            .expect("baseline remains retained")
            .retention()
            .is_pinned()
    );
    assert_eq!(
        app.state
            .workspace
            .content
            .plan_data(plan_id)
            .and_then(|payload| payload.regression_baseline_run),
        Some(baseline_id)
    );
}

fn add_tuning_variable(app: &mut RSpiceApp) -> crate::product::DesignVariableId {
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let variable = crate::state::DesignVariable::new(
        "RGAIN",
        "499 ohm",
        crate::state::DesignVariableQuantity::Resistance,
        crate::state::DesignVariableScope::Project,
        "Closed-loop gain-setting resistor",
        Some(crate::state::DesignVariableRange {
            minimum: "350 ohm".to_owned(),
            maximum: "750 ohm".to_owned(),
        }),
        crate::state::DesignVariableSweepEligibility::NestedSweepAndOptimization,
        crate::state::DesignVariableOverridePolicy::ExplicitTestLocalOverride,
    )
    .expect("valid tuning variable");
    let id = variable.id;
    app.state
        .workspace
        .content
        .add_design_variable(plan_id, variable)
        .expect("variable enters active plan payload");
    id
}

fn stage_literal_tuning_binding(
    app: &mut RSpiceApp,
    component_id: u64,
    variable_name: &str,
) -> crate::product::DesignVariableId {
    sync_tuning_session(app);
    let component = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .expect("component exists");
    let source_view = app.state.workspace.content.active_schematic_reference();
    let typed_expression = format!("{} ohm", component.value.trim());
    let variable = crate::state::DesignVariable::new(
        variable_name,
        &typed_expression,
        crate::state::DesignVariableQuantity::Resistance,
        crate::state::DesignVariableScope::Project,
        format!("Value of {}", component.name),
        None,
        crate::state::DesignVariableSweepEligibility::NestedSweepAndOptimization,
        crate::state::DesignVariableOverridePolicy::ExplicitTestLocalOverride,
    )
    .expect("test literal is a valid resistance");
    let variable_id = variable.id;
    app.state.workbench.verification.tuning_variables.push(
        crate::workbench::state::TuningVariableDraft {
            variable_id,
            baseline_expression: typed_expression.clone(),
            candidate_expression: typed_expression,
            validation_error: None,
            proposed: true,
        },
    );
    app.state.workbench.verification.tuning_instance_binding =
        Some(crate::workbench::state::TuningInstanceBindingDraft {
            component_id,
            component_name: component.name,
            source_view,
            source_topology_version: app.state.schematic.topology_version(),
            source_value: component.value,
            binding_expression: format!("{{{variable_name}}}"),
            variable,
            creates_variable: true,
        });
    variable_id
}

fn retain_preflight_report(app: &mut RSpiceApp) {
    let (plan_id, plan_revision) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| (plan.id(), plan.revision()))
        .expect("default plan");
    let (topology_root, topology_revision, topology_closure) =
        app.state.configured_topology_revision();
    app.state.workbench.preflight = crate::workbench::state::PreflightDialogState {
        open: true,
        report: Some(crate::workbench::state::PreflightReport {
            project_revision: app.state.workspace.content.project.revision().get(),
            topology_root,
            topology_revision,
            topology_closure,
            simulation_plan_id: Some(plan_id),
            simulation_plan_revision: Some(plan_revision),
            blockers: Vec::new(),
            advisories: Vec::new(),
            prepared: None,
        }),
        pending_toast: Some(crate::workbench::state::PreflightToast {
            message: "retained preflight".to_owned(),
            warning: false,
        }),
        ..Default::default()
    };
}

#[test]
fn verify_plan_insertion_invalidates_retained_preflight_evidence() {
    let mut app = RSpiceApp::test_instance();
    retain_preflight_report(&mut app);
    let source_revision = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .revision();

    open_analysis_configuration(&mut app, crate::simulation::plan::AnalysisKind::Noise)
        .expect("missing analysis inserts and opens");

    assert_eq!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("plan remains available")
            .revision(),
        source_revision.next().expect("revision advances")
    );
    assert!(!app.state.workbench.preflight.open);
    assert!(app.state.workbench.preflight.report.is_none());
    assert!(app.state.workbench.preflight.pending_toast.is_none());
}

#[test]
fn tuning_session_discovers_real_variables_and_revert_is_non_destructive() {
    let mut app = RSpiceApp::test_instance();
    let variable_id = add_tuning_variable(&mut app);
    let plan_id = app.state.sim_setup.stable_analysis_plan().unwrap().id();
    sync_tuning_session(&mut app);

    assert_eq!(app.state.workbench.verification.tuning_variables.len(), 1);
    let draft = &mut app.state.workbench.verification.tuning_variables[0];
    assert_eq!(draft.variable_id, variable_id);
    draft.candidate_expression = "620 ohm".to_owned();
    assert!(tuning_is_dirty(&app));

    revert_tuning_session(&mut app);

    assert!(!tuning_is_dirty(&app));
    assert_eq!(
        app.state
            .workspace
            .content
            .plan_data(plan_id)
            .unwrap()
            .design_variables[0]
            .expression,
        "499 ohm"
    );
}

#[test]
fn tuning_commit_is_one_plan_revision_and_queues_the_required_run() {
    let mut app = RSpiceApp::test_instance();
    app.state.provision_test_project_technology_contract();
    app.state.workspace.content.project_sources = Default::default();
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    let variable_id = add_tuning_variable(&mut app);
    let plan_id = app.state.sim_setup.stable_analysis_plan().unwrap().id();
    let source_plan_revision = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .unwrap()
        .revision();
    sync_tuning_session(&mut app);
    app.state.workbench.verification.tuning_variables[0].candidate_expression =
        "620 ohm".to_owned();

    tuning::commit_tuning_and_run(&mut app).expect("valid candidate commits and dispatches");

    let plan = app.state.sim_setup.stable_analysis_plan().unwrap();
    assert_eq!(plan.revision(), source_plan_revision.next().unwrap());
    let variable = app
        .state
        .workspace
        .content
        .plan_data(plan_id)
        .unwrap()
        .design_variables
        .iter()
        .find(|variable| variable.id == variable_id)
        .unwrap();
    assert_eq!(variable.expression, "620 ohm");
    assert_eq!(variable.revision, ObjectRevision::INITIAL.next().unwrap());
    assert!(app.state.simulation.execution.trigger_simulation);
    assert!(!tuning_is_dirty(&app));
}

#[test]
fn reverting_a_literal_value_proposal_discards_variable_and_binding_only() {
    let mut app = RSpiceApp::test_instance();
    let component_id = app.state.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::origin(),
    );
    app.state.schematic.document_mut_for_test().components[0].value = "10k".to_owned();
    let plan_id = app.state.sim_setup.stable_analysis_plan().unwrap().id();
    let workspace_before = serde_json::to_vec(&app.state.workspace).unwrap();
    let value_before = app.state.schematic.document().components[0].value.clone();
    stage_literal_tuning_binding(&mut app, component_id, "R1_VALUE");

    assert!(tuning_is_dirty(&app));
    revert_tuning_session(&mut app);

    assert!(!tuning_is_dirty(&app));
    assert!(
        app.state
            .workbench
            .verification
            .tuning_instance_binding
            .is_none()
    );
    assert!(
        app.state
            .workbench
            .verification
            .tuning_variables
            .iter()
            .all(|draft| !draft.proposed)
    );
    assert_eq!(
        app.state.schematic.document().components[0].value,
        value_before
    );
    assert_eq!(
        serde_json::to_vec(&app.state.workspace).unwrap(),
        workspace_before
    );
    assert!(
        app.state
            .workspace
            .content
            .plan_data(plan_id)
            .unwrap()
            .design_variables
            .is_empty()
    );
}

#[test]
fn literal_value_commit_adds_variable_binds_once_and_dispatches_prepared_run() {
    let mut app = RSpiceApp::test_instance();
    app.state.provision_test_project_technology_contract();
    app.state.workspace.content.project_sources = Default::default();
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    let component_id = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.kind == crate::state::ComponentType::Resistor)
        .map(|component| component.id)
        .expect("voltage divider resistor");
    let component_index = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .position(|component| component.id == component_id)
        .unwrap();
    app.state.schematic.document_mut_for_test().components[component_index].name =
        "RLOAD".to_owned();
    app.state.schematic.document_mut_for_test().components[component_index].value =
        "10k".to_owned();
    app.state.schematic.bump_topology_version();
    let plan_id = app.state.sim_setup.stable_analysis_plan().unwrap().id();
    let source_plan_revision = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .unwrap()
        .revision();
    let variable_id = stage_literal_tuning_binding(&mut app, component_id, "RLOAD_VALUE");

    tuning::commit_tuning_and_run(&mut app)
        .expect("the exact prepared candidate commits and dispatches");

    let variable = app
        .state
        .workspace
        .content
        .plan_data(plan_id)
        .unwrap()
        .design_variables
        .iter()
        .find(|variable| variable.id == variable_id)
        .expect("proposed variable became authoritative");
    assert_eq!(variable.name, "RLOAD_VALUE");
    assert_eq!(variable.expression, "10k ohm");
    assert_eq!(
        app.state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == component_id)
            .unwrap()
            .value,
        "{RLOAD_VALUE}"
    );
    assert_eq!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .unwrap()
            .revision(),
        source_plan_revision.next().unwrap()
    );
    assert!(app.state.schematic.can_undo());
    assert_eq!(
        app.state.schematic.undo_description(),
        Some("bind RLOAD to RLOAD_VALUE")
    );
    assert!(app.state.simulation.execution.trigger_simulation);
    assert!(
        app.state
            .workbench
            .verification
            .tuning_instance_binding
            .is_none()
    );
}

#[test]
fn tuning_commit_rechecks_live_schematic_authority_after_staging() {
    let mut app = RSpiceApp::test_instance();
    app.state.workspace.content.project_sources = Default::default();
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    let component_id = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.kind == crate::state::ComponentType::Resistor)
        .map(|component| component.id)
        .expect("voltage divider resistor");
    let component_index = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .position(|component| component.id == component_id)
        .unwrap();
    app.state.schematic.document_mut_for_test().components[component_index].name =
        "RLOCKED".to_owned();
    app.state.schematic.document_mut_for_test().components[component_index].value =
        "10k".to_owned();
    app.state.schematic.bump_topology_version();
    stage_literal_tuning_binding(&mut app, component_id, "RLOCKED_VALUE");

    let workspace_before = serde_json::to_vec(&app.state.workspace).unwrap();
    let plan_revision_before = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .unwrap()
        .revision();
    let topology_before = app.state.schematic.topology_version();
    let value_before = app.state.schematic.document().components[component_index]
        .value
        .clone();
    let can_undo_before = app.state.schematic.can_undo();

    app.state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..Default::default()
        },
        String::new(),
    );
    assert!(app.state.schematic_edit_read_only());

    let reason = tuning_commit_block_reason(&app);
    assert_eq!(
        reason,
        "The staged instance Value binding cannot be committed because the active schematic is read-only."
    );
    let error = tuning::commit_tuning_and_run(&mut app)
        .expect_err("authority revoked after staging must prevent every authoritative write");

    assert_eq!(error, reason);
    assert_eq!(
        serde_json::to_vec(&app.state.workspace).unwrap(),
        workspace_before
    );
    assert_eq!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .unwrap()
            .revision(),
        plan_revision_before
    );
    assert_eq!(app.state.schematic.topology_version(), topology_before);
    assert_eq!(
        app.state.schematic.document().components[component_index].value,
        value_before
    );
    assert_eq!(app.state.schematic.can_undo(), can_undo_before);
    assert!(tuning_is_dirty(&app));
    assert!(!app.state.simulation.execution.trigger_simulation);
}

#[test]
fn failed_literal_value_run_preparation_rolls_back_plan_and_schematic() {
    let mut app = RSpiceApp::test_instance();
    app.state.provision_test_project_technology_contract();
    let component_id = app.state.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::origin(),
    );
    app.state.schematic.document_mut_for_test().components[0].name = "RFAIL".to_owned();
    app.state.schematic.document_mut_for_test().components[0].value = "10k".to_owned();
    let plan_id = app.state.sim_setup.stable_analysis_plan().unwrap().id();
    let source_plan_revision = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .unwrap()
        .revision();
    let source_topology = app.state.schematic.topology_version();
    let source_can_undo = app.state.schematic.can_undo();
    let workspace_before = serde_json::to_vec(&app.state.workspace).unwrap();
    retain_preflight_report(&mut app);
    stage_literal_tuning_binding(&mut app, component_id, "RFAIL_VALUE");

    let error = tuning::commit_tuning_and_run(&mut app)
        .expect_err("missing ground and open pins fail exact run preparation");

    assert!(error.contains("exact run preparation failed"), "{error}");
    assert_eq!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .unwrap()
            .revision(),
        source_plan_revision
    );
    assert_eq!(app.state.schematic.topology_version(), source_topology);
    assert_eq!(app.state.schematic.document().components[0].value, "10k");
    assert_eq!(app.state.schematic.can_undo(), source_can_undo);
    assert_eq!(
        serde_json::to_vec(&app.state.workspace).unwrap(),
        workspace_before
    );
    assert!(
        app.state
            .workspace
            .content
            .plan_data(plan_id)
            .unwrap()
            .design_variables
            .is_empty()
    );
    assert!(app.state.workbench.preflight.open);
    assert!(tuning_is_dirty(&app));
    assert!(!app.state.simulation.execution.trigger_simulation);
}

#[test]
fn invalid_tuning_candidate_never_mutates_authoritative_plan_data() {
    let mut app = RSpiceApp::test_instance();
    add_tuning_variable(&mut app);
    let plan_id = app.state.sim_setup.stable_analysis_plan().unwrap().id();
    let before = serde_json::to_vec(&app.state.workspace).unwrap();
    sync_tuning_session(&mut app);
    let draft = &mut app.state.workbench.verification.tuning_variables[0];
    draft.candidate_expression = "2 kohm".to_owned();
    let mut candidate = app
        .state
        .workspace
        .content
        .plan_data(plan_id)
        .unwrap()
        .design_variables[0]
        .clone();
    candidate.expression.clone_from(&draft.candidate_expression);
    draft.validation_error = candidate.validate().err();

    assert!(tuning::commit_tuning_and_run(&mut app).is_err());
    assert_eq!(serde_json::to_vec(&app.state.workspace).unwrap(), before);
    assert!(!app.state.simulation.execution.trigger_simulation);
}

#[test]
fn blocked_tuning_run_rolls_back_plan_workspace_and_preflight_state() {
    let mut app = RSpiceApp::test_instance();
    add_tuning_variable(&mut app);
    let plan_id = app.state.sim_setup.stable_analysis_plan().unwrap().id();
    let source_plan_revision = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .unwrap()
        .revision();
    app.state.workbench.preflight.open = true;
    app.state.workbench.preflight.pending_toast = Some(crate::workbench::state::PreflightToast {
        message: "Retain this preflight state".to_owned(),
        warning: true,
    });
    sync_tuning_session(&mut app);
    app.state.workbench.verification.tuning_variables[0].candidate_expression =
        "620 ohm".to_owned();

    let error = tuning::commit_tuning_and_run(&mut app)
        .expect_err("an empty schematic must block the required run");

    assert!(error.contains("required run is blocked"));
    assert_eq!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .unwrap()
            .revision(),
        source_plan_revision
    );
    assert_eq!(
        app.state
            .workspace
            .content
            .plan_data(plan_id)
            .unwrap()
            .design_variables[0]
            .expression,
        "499 ohm"
    );
    assert!(app.state.workbench.preflight.open);
    assert_eq!(
        app.state
            .workbench
            .preflight
            .pending_toast
            .as_ref()
            .map(|toast| toast.message.as_str()),
        Some("Retain this preflight state")
    );
    assert!(tuning_is_dirty(&app));
    assert!(!app.state.simulation.execution.trigger_simulation);
}

#[test]
fn soa_cross_probe_selects_the_exact_schematic_device() {
    let mut app = RSpiceApp::test_instance();
    app.state.schematic.document_mut_for_test().components.push(
        crate::state::Component::new(
            42,
            crate::state::ComponentType::Nmos,
            crate::state::Point::new(10, 20),
        )
        .with_name_value("M1", "NMOS"),
    );

    cross_probe_soa_device(&mut app, "m1")
        .expect("device identity cross-probes case-insensitively");

    assert!(
        app.state
            .schematic
            .session
            .editor
            .selection
            .has_component(42)
    );
    assert_eq!(
        app.state.workbench.workspace,
        super::super::super::state::Workspace::Design
    );
    assert!(cross_probe_soa_device(&mut app, "M404").is_err());
}

#[test]
fn normalized_margin_is_positive_inside_and_negative_outside_bounds() {
    let spec = SpecEntry {
        measurement: "gain".to_owned(),
        expression: String::new(),
        min: Some(10.0),
        max: Some(20.0),
        unit: "dB".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };

    assert_eq!(verification_margin(&spec, 15.0), 5.0);
    assert_eq!(verification_margin(&spec, 8.0), -2.0);
    assert_eq!(verification_margin(&spec, 23.0), -3.0);
    assert_eq!(normalized_margin(&spec, 15.0), Some(0.5));
    assert_eq!(normalized_margin(&spec, 8.0), Some(-0.2));
}

#[test]
fn joint_sample_summary_requires_aligned_trails_and_ands_every_spec() {
    let make_result = |target: &str, trail: Vec<bool>| {
        let pass_count = trail.iter().filter(|passes| **passes).count();
        rspice_results::yield_analysis::YieldResult {
            spec: rspice_results::yield_analysis::YieldSpec::lower(target, 0.0, ""),
            total_runs: trail.len(),
            pass_count,
            fail_count: trail.len() - pass_count,
            yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
            stats: rspice_results::yield_analysis::DistributionStats::default(),
            samples: vec![1.0; trail.len()],
            trail,
        }
    };
    let aligned = [
        make_result("gain", vec![true, false, false]),
        make_result("bandwidth", vec![true, false, true]),
    ];
    let joint = joint_sample_summary(&aligned).expect("aligned joint trail");
    assert_eq!(joint.passing, 1);
    assert_eq!(joint.total, 3);
    assert_eq!(joint.specification_count, 2);
    assert!((joint.yield_percent() - 100.0 / 3.0).abs() < 1.0e-12);

    let misaligned = [
        make_result("gain", vec![true, true]),
        make_result("bandwidth", vec![true]),
    ];
    assert_eq!(joint_sample_summary(&misaligned), None);
}

#[test]
fn joint_headline_can_be_lower_than_every_individual_spec_yield() {
    let make_result = |target: &str, trail: Vec<bool>| {
        let pass_count = trail.iter().filter(|passes| **passes).count();
        rspice_results::yield_analysis::YieldResult {
            spec: rspice_results::yield_analysis::YieldSpec::lower(target, 0.0, ""),
            total_runs: trail.len(),
            pass_count,
            fail_count: trail.len() - pass_count,
            yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
            stats: rspice_results::yield_analysis::DistributionStats::default(),
            samples: vec![1.0; trail.len()],
            trail,
        }
    };
    let results = [
        make_result("gain", vec![true, true, false, true]),
        make_result("bandwidth", vec![true, false, true, true]),
    ];

    let joint = joint_sample_summary(&results).expect("aligned joint trail");
    assert_eq!((joint.passing, joint.total), (2, 4));
    assert_eq!(joint.yield_percent(), 50.0);
    assert!(
        results
            .iter()
            .all(|result| result.yield_percent > joint.yield_percent())
    );
}

#[test]
fn verification_requires_an_explicit_active_run_selection() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .simulation
        .retained
        .runs
        .push(SimulationRun::new(1));
    app.state.simulation.view.active_run_idx = None;

    assert_eq!(verification_run_index(&app), None);
    assert!(verification_run(&app).is_none());
}

#[test]
fn measurement_evidence_requires_success_finite_value_and_provenance() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "legacy")
            .with_measurements(vec![rspice_core::MeasureResult::success("legacy", 1.0)]),
    );
    run.add_analysis(attributed(
        AnalysisResult::failed(2, AnalysisType::Ac, "failed", "solver failed")
            .with_measurements(vec![rspice_core::MeasureResult::success("failed", 2.0)]),
    ));
    run.add_analysis(attributed(
        AnalysisResult::new(3, AnalysisType::Ac, "measure failed").with_measurements(vec![
            rspice_core::MeasureResult::failed("measure_failed", "crossing not found"),
        ]),
    ));
    run.add_analysis(attributed(
        AnalysisResult::new(4, AnalysisType::Ac, "valid")
            .with_measurements(vec![rspice_core::MeasureResult::success("valid", 4.0)]),
    ));

    let tracked = |name: &str| SpecEntry {
        measurement: name.to_owned(),
        expression: String::new(),
        min: None,
        max: None,
        unit: String::new(),
        scope: crate::state::SpecPointScope::AllPoints,
    };

    assert_eq!(worst_measurement_in_run(&run, &tracked("legacy")), None);
    assert_eq!(worst_measurement_in_run(&run, &tracked("failed")), None);
    assert_eq!(
        worst_measurement_in_run(&run, &tracked("measure_failed")),
        None
    );
    assert_eq!(
        worst_measurement_in_run(&run, &tracked("valid")),
        Some(4.0)
    );
}

#[test]
fn regression_evidence_uses_only_aligned_finite_measurements() {
    let source_instance_id = AnalysisInstanceId::new();
    let mut baseline = SimulationRun::new(38);
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Ac, "baseline").with_measurements(vec![
            rspice_core::MeasureResult::success("gain", 40.0),
            rspice_core::MeasureResult::success("bandwidth", 100.0),
            rspice_core::MeasureResult::success("baseline_only", 1.0),
        ]),
        source_instance_id,
    ));
    let mut current = SimulationRun::new(41);
    current.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Ac, "candidate").with_measurements(vec![
            rspice_core::MeasureResult::success("GAIN", 40.4),
            rspice_core::MeasureResult::success("bandwidth", 103.0),
            rspice_core::MeasureResult::success("candidate_only", 2.0),
        ]),
        source_instance_id,
    ));

    let checks = derive_regression_checks(&baseline, &current);
    assert_eq!(checks.len(), 2);
    assert_eq!(checks[0].name, "bandwidth");
    assert!(checks[0].changed());
    assert_eq!(checks[1].name, "gain");
    assert!(checks[1].changed());
}

#[test]
fn regression_evidence_retains_finite_goal_misses() {
    let source_instance_id = AnalysisInstanceId::new();
    let mut baseline_measurement = rspice_core::MeasureResult::success("gain", 40.0);
    baseline_measurement.expected = Some(41.0);
    baseline_measurement.tolerance = Some(0.1);
    baseline_measurement.passed = false;
    baseline_measurement.error = Some("value misses GOAL".to_owned());
    let mut current_measurement = rspice_core::MeasureResult::success("gain", 40.2);
    current_measurement.expected = Some(41.0);
    current_measurement.tolerance = Some(0.1);
    current_measurement.passed = false;
    current_measurement.error = Some("value misses GOAL".to_owned());

    let mut baseline = SimulationRun::new(38);
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Ac, "baseline")
            .with_measurements(vec![baseline_measurement]),
        source_instance_id,
    ));
    let mut current = SimulationRun::new(41);
    current.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Ac, "candidate")
            .with_measurements(vec![current_measurement]),
        source_instance_id,
    ));

    let checks = derive_regression_checks(&baseline, &current);
    assert_eq!(checks.len(), 1);
    assert_eq!(checks[0].baseline, 40.0);
    assert_eq!(checks[0].current, 40.2);
}

#[test]
fn regression_waveforms_align_by_stable_source_and_signal_identity() {
    let source_instance_id = AnalysisInstanceId::new();
    let mut baseline = SimulationRun::new(38);
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(9, AnalysisType::Transient, "unrelated transient").with_waveforms(
            vec![crate::state::WaveformData::new(
                "v(OUT)",
                vec![0.0, 2.0, 4.0],
                vec![0.0, 2.0, 4.0],
                "#808080",
            )],
        ),
        AnalysisInstanceId::new(),
    ));
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Transient, "baseline").with_waveforms(vec![
            crate::state::WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 1.0, 2.0],
                "#ffffff",
            ),
        ]),
        source_instance_id,
    ));
    let mut current = SimulationRun::new(41);
    current.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Transient, "candidate").with_waveforms(vec![
            crate::state::WaveformData::new(
                "v(OUT)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 1.1, 2.1],
                "#ffbf00",
            ),
            crate::state::WaveformData::new(
                "V(other)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 0.5, 1.0],
                "#00ff00",
            ),
        ]),
        source_instance_id,
    ));

    let aligned = regression_waveform_pairs(&baseline, &current);
    assert_eq!(aligned.len(), 1);
    assert_eq!(aligned[0].baseline.name, "V(out)");

    current.analyses[0].waveforms[0].x = std::sync::Arc::new(vec![0.0, 1.0, 2.5]);
    assert_eq!(regression_waveform_pairs(&baseline, &current).len(), 1);
}

fn tolerance_rule(
    target: crate::state::RegressionTargetSelector,
    method: crate::state::RegressionComparisonMethod,
) -> crate::state::RegressionToleranceRule {
    crate::state::RegressionToleranceRule {
        target,
        method,
        absolute_tolerance: 0.0,
        relative_tolerance: 0.0,
        time_skew_allowance: 0.0,
        comparison_window: None,
    }
}

#[test]
fn scalar_regression_verdict_uses_persisted_absolute_and_relative_contract() {
    let source_instance_id = AnalysisInstanceId::new();
    let mut baseline = SimulationRun::new(1);
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Ac, "baseline")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 100.0)]),
        source_instance_id,
    ));
    let mut candidate = SimulationRun::new(2);
    candidate.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Ac, "candidate")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 101.5)]),
        source_instance_id,
    ));
    let check = derive_regression_checks(&baseline, &candidate)
        .into_iter()
        .next()
        .unwrap();
    let mut rule = tolerance_rule(
        check.target.clone(),
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope,
    );
    rule.absolute_tolerance = 0.5;
    rule.relative_tolerance = 0.01;
    assert!(evaluate_regression_check(&check, Some(&rule)).passed());

    rule.relative_tolerance = 0.005;
    assert!(evaluate_regression_check(&check, Some(&rule)).failed());

    rule.method = crate::state::RegressionComparisonMethod::PointwiseRelative;
    rule.absolute_tolerance = 2.0;
    rule.relative_tolerance = 0.01;
    assert!(evaluate_regression_check(&check, Some(&rule)).passed());

    rule.absolute_tolerance = 0.4;
    rule.relative_tolerance = 0.005;
    assert!(evaluate_regression_check(&check, Some(&rule)).failed());
    assert_eq!(
        evaluate_regression_check(&check, None),
        RegressionVerdict::NotConfigured
    );
}

#[test]
fn waveform_envelope_interpolates_within_skew_and_window() {
    let source_instance_id = AnalysisInstanceId::new();
    let mut baseline = SimulationRun::new(1);
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Transient, "baseline").with_waveforms(vec![
            crate::state::WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 1.0, 0.0],
                "#fff",
            ),
        ]),
        source_instance_id,
    ));
    let mut candidate = SimulationRun::new(2);
    candidate.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Transient, "candidate").with_waveforms(vec![
            crate::state::WaveformData::new(
                "V(out)",
                vec![0.1, 1.1, 2.1],
                vec![0.0, 1.0, 0.0],
                "#ff0",
            ),
        ]),
        source_instance_id,
    ));
    let pairs = regression_waveform_pairs(&baseline, &candidate);
    let pair = &pairs[0];
    let mut rule = tolerance_rule(
        pair.target.clone(),
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope,
    );
    rule.time_skew_allowance = 0.100_000_1;
    rule.comparison_window = Some(crate::state::RegressionComparisonWindow {
        start: 0.5,
        end: 1.5,
    });
    assert!(evaluate_regression_waveform(pair, Some(&rule)).passed());

    rule.time_skew_allowance = 0.01;
    assert!(evaluate_regression_waveform(pair, Some(&rule)).failed());
}

#[test]
fn symmetric_waveform_envelope_rejects_candidate_only_spike_and_missing_coverage() {
    let target = crate::state::RegressionTargetSelector {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        source_instance_id: AnalysisInstanceId::new(),
        kind: crate::state::RegressionTargetKind::Waveform,
        name: "v(out)".to_owned(),
        occurrence: 0,
    };
    let baseline =
        crate::state::WaveformData::new("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 0.0], "#fff");
    let spike = crate::state::WaveformData::new(
        "V(out)",
        vec![0.0, 0.5, 1.0, 1.5, 2.0],
        vec![0.0, 0.0, 10.0, 0.0, 0.0],
        "#ff0",
    );
    let mut rule = tolerance_rule(
        target.clone(),
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope,
    );
    rule.absolute_tolerance = 0.1;
    rule.time_skew_allowance = 0.6;
    assert!(
        evaluate_regression_waveform(
            &RegressionWaveformPair {
                target: target.clone(),
                baseline: &baseline,
                current: &spike,
            },
            Some(&rule),
        )
        .failed()
    );

    let truncated =
        crate::state::WaveformData::new("V(out)", vec![0.5, 1.0, 1.5], vec![0.0, 0.0, 0.0], "#ff0");
    rule.time_skew_allowance = 0.1;
    assert!(
        evaluate_regression_waveform(
            &RegressionWaveformPair {
                target,
                baseline: &baseline,
                current: &truncated,
            },
            Some(&rule),
        )
        .failed()
    );
}

#[test]
fn waveform_window_evaluates_interpolated_inclusive_boundaries() {
    let target = crate::state::RegressionTargetSelector {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        source_instance_id: AnalysisInstanceId::new(),
        kind: crate::state::RegressionTargetKind::Waveform,
        name: "v(out)".to_owned(),
        occurrence: 0,
    };
    let baseline =
        crate::state::WaveformData::new("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 0.0], "#fff");
    let candidate =
        crate::state::WaveformData::new("V(out)", vec![0.0, 1.0, 2.0], vec![2.0, 0.0, 2.0], "#ff0");
    let mut rule = tolerance_rule(
        target.clone(),
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope,
    );
    rule.comparison_window = Some(crate::state::RegressionComparisonWindow {
        start: 0.5,
        end: 1.5,
    });
    assert!(
        evaluate_regression_waveform(
            &RegressionWaveformPair {
                target,
                baseline: &baseline,
                current: &candidate,
            },
            Some(&rule),
        )
        .failed(),
        "equal stored interior knots must not hide different interpolated window boundaries"
    );
}

#[test]
fn coverage_union_blocks_dropped_measurement_waveform_analysis_and_orphan_rule() {
    let source_id = AnalysisInstanceId::new();
    let dropped_analysis_id = AnalysisInstanceId::new();
    let orphan_id = AnalysisInstanceId::new();
    let mut baseline = SimulationRun::new(1);
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Transient, "baseline")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 10.0)])
            .with_waveforms(vec![crate::state::WaveformData::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![0.0, 1.0],
                "#fff",
            )]),
        source_id,
    ));
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(2, AnalysisType::DcOp, "dropped empty analysis"),
        dropped_analysis_id,
    ));
    let mut candidate = SimulationRun::new(2);
    candidate.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Transient, "candidate"),
        source_id,
    ));
    let orphan_target = crate::state::RegressionTargetSelector {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        source_instance_id: orphan_id,
        kind: crate::state::RegressionTargetKind::Measurement,
        name: "orphan".to_owned(),
        occurrence: 0,
    };
    let rules = vec![tolerance_rule(
        orphan_target.clone(),
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope,
    )];

    let issues = regression_coverage_issues(&baseline, &candidate, &rules);
    assert_eq!(issues.len(), 4);
    assert!(
        issues.iter().any(|issue| issue.label.contains("gain")
            && issue.detail.contains("missing from the candidate"))
    );
    assert!(issues.iter().any(|issue| issue.label.contains("v(out)")
        && issue.detail.contains("missing from the candidate")));
    assert!(issues.iter().any(
        |issue| issue.label.contains(&dropped_analysis_id.to_string())
            && issue.detail.contains("analysis is missing")
    ));
    assert!(
        issues.iter().any(
            |issue| issue.label.contains("orphan") && issue.detail.contains("absent from both")
        )
    );
    assert_eq!(orphaned_regression_targets(&issues), vec![orphan_target]);

    let limit_only = regression_coverage_issues_for_policy(
        &baseline,
        &candidate,
        &rules,
        crate::state::RegressionSpecificationPolicy::LimitOnly,
    );
    assert_eq!(limit_only.len(), 3);
    assert!(
        limit_only
            .iter()
            .all(|issue| !issue.label.contains("v(out)"))
    );
    assert!(!regression_requires_waveforms(
        crate::state::RegressionSpecificationPolicy::LimitOnly
    ));
    assert!(regression_requires_waveforms(
        crate::state::RegressionSpecificationPolicy::LimitAndWaveform
    ));
}

#[test]
fn every_supported_waveform_method_rejects_nonfinite_and_nonmonotonic_samples() {
    assert_eq!(crate::state::RegressionComparisonMethod::ALL.len(), 2);
    assert!(
        serde_json::from_str::<crate::state::RegressionComparisonMethod>("\"feature_landmarks\"")
            .is_err()
    );
    let target = crate::state::RegressionTargetSelector {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        source_instance_id: AnalysisInstanceId::new(),
        kind: crate::state::RegressionTargetKind::Waveform,
        name: "v(out)".to_owned(),
        occurrence: 0,
    };
    let baseline =
        crate::state::WaveformData::new("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 0.0], "#fff");
    let invalid = crate::state::WaveformData::new(
        "V(out)",
        vec![0.0, 2.0, 1.0],
        vec![0.0, f64::NAN, 0.0],
        "#ff0",
    );
    let pair = RegressionWaveformPair {
        target: target.clone(),
        baseline: &baseline,
        current: &invalid,
    };
    for method in crate::state::RegressionComparisonMethod::ALL {
        let rule = tolerance_rule(target.clone(), method);
        assert!(matches!(
            evaluate_regression_waveform(&pair, Some(&rule)),
            RegressionVerdict::NotEvaluated(_)
        ));
    }
}

#[test]
fn legacy_and_incomplete_runs_are_not_eligible_regression_datasets() {
    let source_id = AnalysisInstanceId::new();
    let mut legacy = SimulationRun::new(1);
    legacy.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "legacy").with_provenance(
            crate::state::AnalysisResultProvenance::new_with_source_domain(
                crate::state::AnalysisResultSourceDomain::LegacyUnclassified,
                source_id,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x31; 32]),
                Vec::new(),
            )
            .unwrap(),
        ),
    );
    legacy
        .restore_provenance(crate::state::SimulationRunProvenance::LegacyPreparedUnclassified)
        .unwrap();
    assert!(regression_run_seal(&legacy).is_err());

    let plan_id = crate::product::SimulationPlanId::new();
    let complete = sealed_run(
        2,
        plan_id,
        AnalysisInstanceId::new(),
        2,
        AnalysisResult::new(1, AnalysisType::Ac, "complete"),
    );
    let mut incomplete = complete.clone();
    incomplete.analyses.clear();
    assert!(
        regression_run_seal(&incomplete)
            .unwrap_err()
            .contains("incomplete")
    );
}

#[test]
fn ci_documents_are_deterministic_receipt_bound_and_cover_the_full_matrix() {
    let digest = |byte| ContentDigest::from_bytes([byte; 32]);
    let receipt = super::super::super::state::RegressionComparisonReceipt {
        plan_id: crate::product::SimulationPlanId::new(),
        plan_revision: ObjectRevision::INITIAL,
        tolerance_digest: digest(1),
        baseline_run: crate::product::RunId::new(),
        candidate_run: crate::product::RunId::new(),
        baseline_dataset: crate::product::DatasetId::new(),
        candidate_dataset: crate::product::DatasetId::new(),
        baseline_content_digest: digest(2),
        candidate_content_digest: digest(3),
        baseline_authority_digest: digest(4),
        candidate_authority_digest: digest(5),
        aligned_checks: 1,
        aligned_waveforms: 1,
        changed_checks: 1,
        passed_checks: 1,
        failed_checks: 0,
        passed_waveforms: 0,
        failed_waveforms: 1,
        unconfigured_targets: 0,
        unevaluated_targets: 1,
    };
    let cases = vec![
        RegressionExportCase {
            name: "measurement::gain".to_owned(),
            detail: "worst_delta=0".to_owned(),
            disposition: RegressionExportDisposition::Pass,
        },
        RegressionExportCase {
            name: "waveform::v(out)".to_owned(),
            detail: "envelope exceeded <limit>".to_owned(),
            disposition: RegressionExportDisposition::Failure,
        },
        RegressionExportCase {
            name: "coverage::analysis".to_owned(),
            detail: "candidate missing".to_owned(),
            disposition: RegressionExportDisposition::Blocked,
        },
    ];
    let first = regression_ci_documents(&receipt, &cases).unwrap();
    let second = regression_ci_documents(&receipt, &cases).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.0.matches("<testcase ").count(), 3);
    assert!(first.0.contains("tests=\"3\" failures=\"1\" errors=\"1\""));
    assert!(
        first
            .0
            .contains(&receipt.candidate_content_digest.to_string())
    );
    assert!(first.0.contains("envelope exceeded &lt;limit&gt;"));
    assert!(first.1.contains("1..3"));
    assert!(first.1.contains("not ok 2 - waveform::v(out)"));
    assert!(
        first
            .1
            .contains(&receipt.baseline_authority_digest.to_string())
    );
}

#[test]
fn empty_regression_matrix_exports_one_blocked_case_and_xml_controls_fail_closed() {
    let cases = regression_export_cases(&[], &[], &[], &[], &[]);
    assert_eq!(cases.len(), 1);
    assert_eq!(cases[0].name, "coverage::no_comparable_targets");
    assert_eq!(cases[0].disposition, RegressionExportDisposition::Blocked);
    let digest = |byte| ContentDigest::from_bytes([byte; 32]);
    let receipt = super::super::super::state::RegressionComparisonReceipt {
        plan_id: crate::product::SimulationPlanId::new(),
        plan_revision: ObjectRevision::INITIAL,
        tolerance_digest: digest(1),
        baseline_run: crate::product::RunId::new(),
        candidate_run: crate::product::RunId::new(),
        baseline_dataset: crate::product::DatasetId::new(),
        candidate_dataset: crate::product::DatasetId::new(),
        baseline_content_digest: digest(2),
        candidate_content_digest: digest(3),
        baseline_authority_digest: digest(4),
        candidate_authority_digest: digest(5),
        aligned_checks: 0,
        aligned_waveforms: 0,
        changed_checks: 0,
        passed_checks: 0,
        failed_checks: 0,
        passed_waveforms: 0,
        failed_waveforms: 0,
        unconfigured_targets: 0,
        unevaluated_targets: 1,
    };
    let (junit, tap) = regression_ci_documents(&receipt, &cases).unwrap();
    assert!(junit.contains("tests=\"1\" failures=\"0\" errors=\"1\""));
    assert!(tap.contains("not ok 1 - coverage::no_comparable_targets"));
    assert!(xml_escape("illegal\u{1}control").is_err());
    assert_eq!(xml_escape("safe < value").unwrap(), "safe &lt; value");

    let source = AnalysisInstanceId::new();
    let checks = [RegressionCheck {
        target: crate::state::RegressionTargetSelector {
            source_domain: crate::state::AnalysisResultSourceDomain::ManualDeck,
            source_instance_id: source,
            kind: crate::state::RegressionTargetKind::Measurement,
            name: "gain".to_owned(),
            occurrence: 2,
        },
        name: "gain".to_owned(),
        source_identity: source.to_string(),
        baseline: 1.0,
        current: 1.0,
    }];
    let cases = regression_export_cases(
        &checks,
        &[RegressionVerdict::Pass {
            worst_delta: 0.0,
            allowed_delta: 0.0,
        }],
        &[],
        &[],
        &[],
    );
    assert_eq!(
        cases[0].name,
        format!("measurement::manual_deck::{source}::gain[2]")
    );
}

#[test]
fn comparison_window_parser_is_unit_safe_and_fail_closed() {
    assert_eq!(
        parse_regression_window("1m … 20m").unwrap(),
        Some(crate::state::RegressionComparisonWindow {
            start: 1e-3,
            end: 20e-3,
        })
    );
    assert_eq!(parse_regression_window("full domain").unwrap(), None);
    assert!(parse_regression_window("20m … 1m").is_err());
    assert!(parse_regression_window("not a window").is_err());
}

#[test]
fn tolerance_digest_is_order_independent_and_receipts_fail_on_contract_mismatch() {
    let target = |name: &str| crate::state::RegressionTargetSelector {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        source_instance_id: AnalysisInstanceId::from_namespace(
            uuid::Uuid::NAMESPACE_OID,
            name.as_bytes(),
        ),
        kind: crate::state::RegressionTargetKind::Measurement,
        name: name.to_owned(),
        occurrence: 0,
    };
    let mut first = tolerance_rule(
        target("gain"),
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope,
    );
    first.absolute_tolerance = 0.1;
    let mut second = tolerance_rule(
        target("settling"),
        crate::state::RegressionComparisonMethod::PointwiseRelative,
    );
    second.relative_tolerance = 0.02;
    let digest = regression_tolerance_digest(&[first.clone(), second.clone()]);
    assert_eq!(
        digest,
        regression_tolerance_digest(&[second.clone(), first.clone()])
    );
    second.relative_tolerance = 0.03;
    let changed_digest = regression_tolerance_digest(&[first, second]);
    assert_ne!(digest, changed_digest);

    let plan_id = crate::product::SimulationPlanId::new();
    let source_id = AnalysisInstanceId::new();
    let baseline = sealed_run(
        1,
        plan_id,
        source_id,
        2,
        AnalysisResult::new(1, AnalysisType::Ac, "baseline")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)]),
    );
    let mut candidate = sealed_run(
        2,
        plan_id,
        source_id,
        2,
        AnalysisResult::new(1, AnalysisType::Ac, "candidate")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)]),
    );
    let baseline_seal = regression_run_seal(&baseline).unwrap();
    let candidate_seal = regression_run_seal(&candidate).unwrap();
    let receipt = super::super::super::state::RegressionComparisonReceipt {
        plan_id,
        plan_revision: ObjectRevision::INITIAL,
        tolerance_digest: digest,
        baseline_run: baseline.run_id,
        candidate_run: candidate.run_id,
        baseline_dataset: baseline.dataset_id,
        candidate_dataset: candidate.dataset_id,
        baseline_content_digest: baseline_seal.content_digest,
        candidate_content_digest: candidate_seal.content_digest,
        baseline_authority_digest: baseline_seal.authority_digest,
        candidate_authority_digest: candidate_seal.authority_digest,
        aligned_checks: 2,
        aligned_waveforms: 0,
        changed_checks: 1,
        passed_checks: 2,
        failed_checks: 0,
        passed_waveforms: 0,
        failed_waveforms: 0,
        unconfigured_targets: 0,
        unevaluated_targets: 0,
    };
    assert!(regression_receipt_matches_contract(
        &receipt,
        plan_id,
        ObjectRevision::INITIAL,
        digest,
        &baseline,
        &candidate,
    ));
    assert!(!regression_receipt_matches_contract(
        &receipt,
        crate::product::SimulationPlanId::new(),
        ObjectRevision::INITIAL,
        digest,
        &baseline,
        &candidate,
    ));
    assert!(!regression_receipt_matches_contract(
        &receipt,
        plan_id,
        ObjectRevision::new(2).unwrap(),
        digest,
        &baseline,
        &candidate,
    ));
    assert!(!regression_receipt_matches_contract(
        &receipt,
        plan_id,
        ObjectRevision::INITIAL,
        changed_digest,
        &baseline,
        &candidate,
    ));

    candidate.analyses[0].measurements[0].value = Some(1.25);
    assert!(!regression_receipt_matches_contract(
        &receipt,
        plan_id,
        ObjectRevision::INITIAL,
        digest,
        &baseline,
        &candidate,
    ));

    let mut session = super::super::super::state::VerificationSessionState::default();
    session.regression_comparison = Some(receipt);
    let serialized = serde_json::to_value(session).unwrap();
    assert!(serialized.get("regression_comparison").is_none());
}

/// A limit answered on both surfaces must be answered with the same number.
///
/// Verify's cockpit and the studio's requirements page read the same limits
/// against the same selected dataset. Verify used to take the first
/// name-matching measurement it found; the studio takes the worst of every
/// attributed measurement of that name. On a dataset holding several — a corner
/// set, or a family whose members measured — that is not a display difference:
/// one surface reports a specification passing while a retained point fails it.
#[test]
fn verify_and_the_studio_report_the_same_worst_value_for_one_limit() {
    use crate::state::{
        AnalysisResultFamilyMetadata, FamilyMeasurementEvidence, FamilyMemberId,
        FamilyMemberMeasurements,
    };

    let spec = SpecEntry {
        measurement: "gain".to_owned(),
        expression: "param=gain".to_owned(),
        min: Some(10.0),
        max: None,
        unit: "dB".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };

    let members = [(0, 25.0, 12.0), (1, 75.0, 7.25), (2, 125.0, 11.0)]
        .into_iter()
        .map(|(index, coordinate, value)| {
            FamilyMemberMeasurements::new(
                FamilyMemberId::SweepPoint {
                    index,
                    value: coordinate,
                },
                vec![FamilyMeasurementEvidence {
                    unit: None,
                    name: "gain".to_owned(),
                    value: Some(value),
                    passed: true,
                    error: None,
                }],
            )
        })
        .collect();

    let mut run = SimulationRun::new(7);
    run.add_analysis(attributed(
        AnalysisResult::new(1, AnalysisType::Parametric, "sweep").with_family_metadata(
            AnalysisResultFamilyMetadata::Parametric {
                target: "TEMP".to_owned(),
                sweep_values: vec![25.0, 75.0, 125.0],
                failed_points: 0,
                member_measurements: members,
            },
        ),
    ));

    let verify_value = worst_measurement_in_run(&run, &spec);
    let studio_value =
        crate::workbench::surfaces::simulate::measurement_in_output_dataset_for_test(&run, &spec);

    assert_eq!(
        verify_value,
        Some(7.25),
        "Verify must report the worst retained point, not the first"
    );
    assert_eq!(
        verify_value, studio_value,
        "Verify and the studio must not disagree about which number answers a limit"
    );
}

/// Verify must not judge a dataset the studio refuses.
///
/// The run's own measured values stay on the page — it did measure them — but
/// a run another plan owns answers none of this plan's limits, so no limit
/// reaches the table and nothing in it is judged.
#[test]
fn verify_refuses_the_datasets_the_studio_refuses_and_names_why() {
    use crate::state::EvidenceDomain;

    assert!(EvidenceDomain::ThisPlan.answers_a_plan_limit());
    assert!(EvidenceDomain::Legacy.answers_a_plan_limit());
    for refused in [
        EvidenceDomain::AnotherPlan,
        EvidenceDomain::ManualDeck,
        EvidenceDomain::NoDataset,
    ] {
        assert!(
            !refused.answers_a_plan_limit(),
            "{refused:?} must not answer a plan's limit"
        );
        assert!(
            refused.refusal().is_some(),
            "{refused:?} must state why it cannot, or the surface goes silently blank"
        );
    }

    let mut app = super::raster::corner_app();
    assert_eq!(
        verification_evidence_domain(&app),
        EvidenceDomain::Legacy,
        "the corner fixture's run predates receipts, which Verify reads"
    );
    assert!(
        !verification_specs(&app).is_empty(),
        "and its four limits are this plan's"
    );

    // The same run, now owned by a plan that is not the one being read.
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let other = crate::product::SimulationPlanId::new();
    let analysis = app.state.simulation.retained.runs[0].analyses[0].clone();
    let source = analysis
        .provenance()
        .expect("the fixture attributes its analysis")
        .source_instance_id();
    app.state.simulation.retained.runs =
        vec![super::raster::sealed_run(3, other, source, 19, analysis)].into();
    app.state.simulation.view.active_run_idx = Some(0);

    assert_ne!(other, plan_id);
    assert_eq!(
        verification_evidence_domain(&app),
        EvidenceDomain::AnotherPlan
    );
    assert!(
        verification_specs(&app).is_empty(),
        "another plan's dataset answers none of this plan's limits"
    );
    let run = verification_run(&app).expect("the run is still the one Verify reads");
    let family =
        latest_analysis(&app, AnalysisType::Corner).expect("and still holds its corner run");
    let matrix = corner_matrix(run, family, None, &verification_specs(&app), None);
    assert!(
        !matrix.rows.is_empty(),
        "the values the run did measure stay on the page"
    );
    assert!(
        matrix.rows.iter().all(|row| row.verdict().is_none()),
        "and nothing in it is judged"
    );
}

// ---------------------------------------------------------------------------
// The SOA evidence gate
// ---------------------------------------------------------------------------

/// Retained samples in the fixture SOA stress histories. Large enough that a
/// per-frame revalidation is real work, small enough to build in a test.
const EVIDENCE_SAMPLES: usize = 512;
/// Evaluated rules, and so retained stress waveforms, in the fixture.
const EVIDENCE_RULES: usize = 8;
/// Frames measured after the first, matching the Results idle-frame gate: ten
/// is enough that a per-frame walk cannot hide behind an alternating schedule.
const EVIDENCE_IDLE_FRAMES: usize = 10;

/// An SOA analysis whose retained evidence validates, built in the shape the
/// validator accepts: one exact worst-point event per warned rule.
fn soa_evidence_analysis() -> AnalysisResult {
    use crate::state::{
        SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
        SoaViolationSeverityEvidence,
    };

    let time: Vec<f64> = (0..EVIDENCE_SAMPLES)
        .map(|index| index as f64 * 1.0e-9)
        .collect();
    let mut waveforms = Vec::with_capacity(EVIDENCE_RULES);
    let mut evaluations = Vec::with_capacity(EVIDENCE_RULES);
    let mut violations = Vec::with_capacity(EVIDENCE_RULES);
    for rule in 0..EVIDENCE_RULES {
        let device_id = format!("M{rule:03}");
        let y: Vec<f64> = (0..EVIDENCE_SAMPLES)
            .map(|index| 3.0 * (index as f64 + 1.0) / EVIDENCE_SAMPLES as f64)
            .collect();
        let worst_actual_value = y[EVIDENCE_SAMPLES - 1];
        let worst_time_s = time[EVIDENCE_SAMPLES - 1];
        waveforms.push(crate::state::WaveformData::new(
            crate::results::safety::soa_stress_waveform_name(
                &device_id,
                crate::results::safety::SoAParameter::Vds,
            ),
            time.clone(),
            y,
            "#00aaff",
        ));
        evaluations.push(SoaEvaluationEvidence {
            duration: None,
            thresholds: Default::default(),
            envelope: None,
            derating: None,
            device_id: device_id.clone(),
            parameter: SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 3.3,
            worst_actual_value,
            worst_time_s,
            sample_count: EVIDENCE_SAMPLES as u64,
            unit: "V".to_owned(),
            description: "Maximum drain-source voltage".to_owned(),
            verdict: SoaRuleVerdictEvidence::Warning,
        });
        violations.push(SoaViolationEvidence {
            device_id,
            parameter: SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 3.3,
            actual_value: worst_actual_value,
            time_s: worst_time_s,
            severity: SoaViolationSeverityEvidence::Warning,
        });
    }
    AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::Soa { time })
        .with_waveforms(waveforms)
        .with_result_payload(crate::state::AnalysisResultPayload::Soa {
            source_history: None,
            evaluations,
            violations,
        })
}

fn app_retaining_soa_evidence() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let run = sealed_run(
        1,
        plan_id,
        AnalysisInstanceId::new(),
        9,
        soa_evidence_analysis(),
    );
    app.state.simulation.retained.runs = vec![run].into();
    assert!(app.state.simulation.select_run(0));
    app
}

#[test]
fn the_soa_pane_validate_retained_evidence_once_per_dataset_generation() {
    use crate::workbench::documents::result_document::frame_work::{DatasetWalk, WorkCounts};

    let mut app = app_retaining_soa_evidence();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    // One frame of the SOA tab against unchanged state and identical
    // input, drawn through the pane's own entry point.
    let mut frame = || {
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_680.0, 1_020.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| device_safety(ui, &mut app));
            },
        );
    };

    // The first frame is allowed to build the memo the rest read — but it must
    // build it through the counted owner. A pane that validates on its own is
    // uncounted, so this half of the gate is what keeps the other half from
    // passing by measuring nothing.
    let opening = WorkCounts::reset();
    frame();
    assert_eq!(
        opening.since().get(DatasetWalk::EvidenceValidation),
        1,
        "the SOA gate must ask the memoized owner once on the \
         opening frame; a gate that walks the dataset itself is not counted here \
         and would leave the idle-frame assertion below measuring nothing"
    );

    let baseline = WorkCounts::reset();
    for _ in 0..EVIDENCE_IDLE_FRAMES {
        frame();
    }
    let work = baseline.since();

    assert_eq!(
        work.get(DatasetWalk::EvidenceValidation),
        0,
        "the SOA pane revalidated the retained dataset on an idle frame; counted work was {:?}",
        work.nonzero()
    );
}

#[test]
fn the_soa_evidence_gate_rejects_source_edits_before_a_version_bump() {
    let mut app = app_retaining_soa_evidence();
    let version = app.state.simulation.view.data_version;
    assert!(
        latest_validated_analysis(&app, AnalysisType::Soa).is_some(),
        "the fixture retains validated SOA evidence"
    );

    // A retained waveform with more coordinates than values is exactly what
    // `validate_retained_evidence` exists to refuse.
    let analysis = app.state.simulation.retained.runs[0]
        .analyses
        .iter_mut()
        .find(|analysis| analysis.analysis_type == AnalysisType::Soa)
        .expect("the fixture retains an SOA analysis");
    let mut shortened = analysis.waveforms[0].y.as_ref().clone();
    shortened.pop();
    analysis.waveforms[0].y = std::sync::Arc::new(shortened);

    assert!(
        latest_validated_analysis(&app, AnalysisType::Soa).is_none(),
        "the gate kept a valid verdict after its retained source was corrupted"
    );
    assert_eq!(app.state.simulation.view.data_version, version);

    app.state.simulation.view.data_version = app.state.simulation.view.data_version.wrapping_add(1);

    assert!(
        latest_validated_analysis(&app, AnalysisType::Soa).is_none(),
        "the gate served a verdict from the previous dataset generation"
    );
}

// ---------------------------------------------------------------------------
// The page grammar: corners, SOA, optimization
// ---------------------------------------------------------------------------

use super::table::Verdict;

#[test]
fn corner_labels_read_back_into_their_point() {
    assert_eq!(
        parse_corner_label("SS_1.620000V_-40.000000C"),
        Some(CornerPoint {
            process: "SS".to_owned(),
            voltage: Some(1.62),
            temperature: -40.0,
        })
    );
    assert_eq!(
        parse_corner_label("TT_MC_1.00V_27C").map(|point| point.process),
        Some("TT_MC".to_owned())
    );
    assert_eq!(parse_corner_label("Point 3"), None);
    assert_eq!(parse_corner_label("TT_1.8_27C"), None);
}

fn fixture_corner_matrix() -> CornerMatrix {
    let app = super::raster::corner_app();
    let run = verification_run(&app).expect("the fixture selects its run");
    let family =
        latest_analysis(&app, AnalysisType::Corner).expect("the fixture holds a corner run");
    corner_matrix(
        run,
        family,
        None,
        &verification_specs(&app),
        Some(&ReferenceCorner {
            process: "TT".to_owned(),
            temperature: 27.0,
        }),
    )
}

#[test]
fn the_corner_matrix_reads_every_output_at_every_corner_from_the_run() {
    let matrix = fixture_corner_matrix();
    assert_eq!(
        matrix
            .columns
            .iter()
            .map(|column| column.head.as_str())
            .collect::<Vec<_>>(),
        [
            "TT 1.8 V 27 °C",
            "SS 1.62 V 125 °C",
            "SS 1.62 V −40 °C",
            "FF 1.98 V 125 °C",
            "FF 1.98 V −40 °C",
            "SF 1.8 V 27 °C",
            "FS 1.8 V 27 °C",
        ]
    );
    assert_eq!(matrix.columns[1].detail, "SS · 1.62 V · 125 °C");
    assert_eq!(
        matrix.nominal,
        Some(0),
        "TT at 27 °C is the reference corner"
    );
    assert_eq!(matrix.corner_indices(), [1, 2, 3, 4, 5, 6]);
    assert_eq!(
        matrix
            .rows
            .iter()
            .map(|row| row.output.as_str())
            .collect::<Vec<_>>(),
        ["gain", "bw", "pm", "idd", "V(out)", "V(bias)"],
        "limited outputs in specification order, then the corner waveforms"
    );
    let gain = &matrix.rows[0];
    assert_eq!(gain.values[1], Some(39.4));
    assert_eq!(gain.verdict_of(39.4), Some(Verdict::Fail));
    assert_eq!(gain.verdict(), Some(Verdict::Fail));
    assert_eq!(matrix.rows[2].verdict_of(47.0), Some(Verdict::Near));
    assert_eq!(matrix.rows[3].verdict(), Some(Verdict::Near));
    assert_eq!(matrix.rows[1].verdict(), Some(Verdict::Pass));
    assert_eq!(
        matrix.rows[4].verdict(),
        None,
        "a node voltage has no limit"
    );
    assert_eq!(matrix.rows[4].unit, "V");
    assert_eq!(
        matrix.summary(),
        super::toolbar::Summary::failing(
            "3 / 4 outputs pass across 7 corners · worst SS 1.62 V 125 °C"
        )
    );
}

#[test]
fn a_corner_run_without_its_reference_point_has_no_nominal_column() {
    let app = super::raster::corner_app();
    let run = verification_run(&app).expect("run");
    let family = latest_analysis(&app, AnalysisType::Corner).expect("corner run");
    let matrix = corner_matrix(
        run,
        family,
        None,
        &verification_specs(&app),
        Some(&ReferenceCorner {
            process: "TT".to_owned(),
            temperature: 85.0,
        }),
    );
    assert_eq!(matrix.nominal, None);
    assert_eq!(matrix.corner_indices().len(), 7);
}

#[test]
fn soa_rows_put_violations_first_and_the_summary_counts_devices() {
    let app = super::raster::soa_app();
    let analysis = latest_validated_analysis(&app, AnalysisType::Soa)
        .expect("the SOA fixture's evidence validates");
    let Some(crate::state::AnalysisResultPayload::Soa { evaluations, .. }) =
        &analysis.result_payload
    else {
        panic!("the fixture holds an SOA payload");
    };
    let rows = soa_rows(evaluations);
    assert_eq!(
        rows.iter()
            .map(|row| (row.device.as_str(), row.verdict))
            .collect::<Vec<_>>(),
        [
            ("M3", Verdict::Fail),
            ("M1", Verdict::Fail),
            ("M2", Verdict::Near),
            ("M1", Verdict::Pass),
            ("M4", Verdict::Pass),
        ]
    );
    assert_eq!(
        soa_summary(&rows),
        Some(super::toolbar::Summary::failing(
            "2 violations on 2 devices"
        ))
    );
    assert_eq!(soa_summary(&rows[3..]), None, "a clean run says nothing");
}

#[test]
fn optimization_iterations_put_the_cost_first_and_name_the_best() {
    let app = super::raster::optimization_app();
    let result = latest_analysis(&app, AnalysisType::Optimization).expect("optimization run");
    let table = IterationTable::from_result(result).expect("iterations");
    assert_eq!(
        table
            .traces
            .iter()
            .map(|(head, _)| head.as_str())
            .collect::<Vec<_>>(),
        ["Cost", "IBIAS", "W1"]
    );
    assert_eq!(table.best, Some(23));
    assert_eq!(
        table.summary().text,
        "converged · best cost 20.04 m at iteration 23 · 24 iterations"
    );
}

/// Every seeded page opens with a 36 px toolbar whose title starts on the
/// panel inset, then a table of 24 px head and rows directly under it, with
/// every right-aligned column ending on one edge and nothing past the insets.
#[test]
fn seeded_verify_pages_follow_the_table_grammar() {
    use super::raster::{
        PAGE_WIDTH, corner_app, monte_carlo_app, optimization_app, page_geometry, soa_app,
    };

    for (page, app) in [
        (VerificationPage::Yield, monte_carlo_app()),
        (VerificationPage::Corners, corner_app()),
        (VerificationPage::DeviceSafety, soa_app()),
        (VerificationPage::Optimization, optimization_app()),
    ] {
        let geometry = page_geometry(app, page);
        let toolbar = geometry.one("toolbar").expect("toolbar");
        assert_eq!(toolbar.height(), super::toolbar::TOOLBAR_H, "{page:?}");
        assert_eq!(toolbar.left(), 0.0, "{page:?}");
        assert_eq!(
            geometry.one("toolbar.title").expect("title").left(),
            10.0,
            "{page:?}"
        );
        let head = geometry.one("table.head").expect("table head");
        assert_eq!(head.height(), 24.0, "{page:?}");
        assert_eq!(
            head.top(),
            toolbar.bottom(),
            "{page:?}: nothing between toolbar and table"
        );
        let rows = geometry.all("table.row");
        assert!(!rows.is_empty(), "{page:?}");
        assert!(rows.iter().all(|row| row.height() == 24.0), "{page:?}");
        assert_eq!(rows[0].top(), head.bottom(), "{page:?}");
        for (column, edges) in geometry.right_edges() {
            assert_eq!(
                edges.len(),
                1,
                "{page:?} {column}: numbers end on one edge, {edges:?}"
            );
        }
        assert_eq!(
            geometry.outside_insets(PAGE_WIDTH),
            Vec::<String>::new(),
            "{page:?}"
        );
        assert!(
            geometry.one("toolbar.summary").is_some(),
            "{page:?} states a summary"
        );
    }
}

/// A page with nothing to show is its toolbar and one line saying what
/// produces data: no table, no tiles.
#[test]
fn empty_verify_pages_are_one_line_under_the_toolbar() {
    for page in VerificationPage::NAVIGATION {
        let geometry = super::raster::page_geometry(RSpiceApp::test_instance(), page);
        let toolbar = geometry.one("toolbar").expect("toolbar");
        let empty = geometry.one("empty").expect("empty line");
        assert_eq!(empty.top(), toolbar.bottom(), "{page:?}");
        assert_eq!(empty.height(), super::toolbar::EMPTY_LINE_H, "{page:?}");
        assert!(geometry.one("table.head").is_none(), "{page:?}");
        assert!(geometry.one("toolbar.summary").is_none(), "{page:?}");
    }
}

/// Tuning is its toolbar, the variable editors beside the response they
/// produce, and under both the limits judged against the baseline and the
/// candidate run.
#[test]
fn the_tuning_page_is_two_panes_over_the_limits_table() {
    use super::raster::{PAGE_WIDTH, page_geometry, tuning_app};

    let geometry = page_geometry(tuning_app(), VerificationPage::Tuning);
    let toolbar = geometry.one("toolbar").expect("toolbar");
    assert_eq!(toolbar.height(), super::toolbar::TOOLBAR_H);
    assert!(
        geometry.one("toolbar.summary").is_some(),
        "the toolbar names the candidate and the baseline"
    );
    let variables = geometry.one("tuning.variables").expect("variables pane");
    let response = geometry.one("tuning.response").expect("response pane");
    assert_eq!(variables.left(), 0.0);
    assert_eq!(variables.top(), toolbar.bottom());
    assert_eq!(response.top(), variables.top());
    assert_eq!(response.bottom(), variables.bottom());
    assert_eq!(response.left(), variables.right() + 1.0);
    assert_eq!(response.right(), PAGE_WIDTH);
    let head = geometry.one("table.head").expect("limits table");
    assert_eq!(head.top(), variables.bottom() + 1.0, "one hairline between");
    let rows = geometry.all("table.row");
    assert_eq!(rows.len(), 2, "one row per limit");
    assert!(rows.iter().all(|row| row.height() == 24.0));
    assert_eq!(rows[0].top(), head.bottom());
    for (column, edges) in geometry.right_edges() {
        assert_eq!(edges.len(), 1, "{column}: numbers end on one edge");
    }
    assert_eq!(geometry.outside_insets(PAGE_WIDTH), Vec::<String>::new());
}

/// Regression is one table of every compared target, and the selected
/// target's detail in a pane directly under it.
#[test]
fn the_regression_page_is_one_table_and_the_selected_targets_detail() {
    use super::raster::{PAGE_WIDTH, page_geometry, regression_app};

    let geometry = page_geometry(regression_app(), VerificationPage::Regression);
    let toolbar = geometry.one("toolbar").expect("toolbar");
    assert!(geometry.one("toolbar.summary").is_some());
    let head = geometry.one("table.head").expect("checks table");
    assert_eq!(head.top(), toolbar.bottom(), "nothing between toolbar and table");
    let rows = geometry.all("table.row");
    assert_eq!(rows.len(), 5, "four measurements and one waveform");
    assert!(rows.iter().all(|row| row.height() == 24.0));
    assert_eq!(rows[0].top(), head.bottom());
    let detail = geometry.one("split").expect("the selected waveform's detail");
    let last = rows.last().expect("rows");
    assert_eq!(detail.top(), last.bottom() + 1.0, "the detail sits under the table");
    assert_eq!(detail.left(), 0.0);
    assert_eq!(detail.right(), PAGE_WIDTH);
    for (column, edges) in geometry.right_edges() {
        assert_eq!(edges.len(), 1, "{column}: numbers end on one edge");
    }
    assert_eq!(geometry.outside_insets(PAGE_WIDTH), Vec::<String>::new());
}

// ---------------------------------------------------------------------------
// Monte Carlo yield and its histogram
// ---------------------------------------------------------------------------

fn fixture_yield_results(app: &RSpiceApp) -> Vec<rspice_results::yield_analysis::YieldResult> {
    verification_yield_results(app).to_vec()
}

/// A Spec cell and an axis tick state a quantity the way the measured values
/// beside them do: the SI prefix on the unit, one space in front of it.
///
/// The Spec column used to read `≥ 10M Hz` and `≤ 1.200m A` — the prefix split
/// off the unit it scales, and three zeros nobody authored — while the value
/// in the next cell read `11.90 MHz`.
#[test]
fn a_limit_and_a_tick_are_spelled_like_the_values_beside_them() {
    let bound = |min: Option<f64>, max: Option<f64>, unit: &str| {
        super::limit_cell(&SpecEntry {
            measurement: "m".to_owned(),
            expression: String::new(),
            min,
            max,
            unit: unit.to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        })
        .text
    };

    assert_eq!(bound(Some(10.0e6), None, "Hz"), "\u{2265} 10 MHz");
    assert_eq!(bound(None, Some(1.2e-3), "A"), "\u{2264} 1.2 mA");
    assert_eq!(bound(Some(40.0), None, "dB"), "\u{2265} 40 dB");
    assert_eq!(bound(Some(45.0), None, "deg"), "\u{2265} 45 deg");
    assert_eq!(bound(None, Some(2.0e-6), "s"), "\u{2264} 2 \u{b5}s");
    assert_eq!(bound(Some(1.62), Some(1.98), "V"), "1.62 V \u{2026} 1.98 V");
    assert_eq!(bound(None, None, "V"), "", "a bound of nothing says nothing");

    // One space before the unit, the same one the values carry.
    assert_eq!(super::histogram::tick_text(38.0, 0.5, "dB"), "38 dB");
    assert_eq!(super::histogram::tick_text(10.0e6, 2.0e6, "Hz"), "10 MHz");
    assert_eq!(super::format_value(38.9, "dB"), "38.90 dB");
}

/// The statistics come from the exact retained samples. The fixture's
/// summary block is all zeros, which is what the old histogram header printed
/// as "mean 0.000 dB" over data spanning 38.9 to 43.2.
#[test]
fn yield_rows_compute_their_statistics_from_the_samples() {
    let app = super::raster::monte_carlo_app();
    let results = fixture_yield_results(&app);
    assert_eq!(
        results[0].stats.mean, 0.0,
        "the fixture's own summary is empty"
    );
    let row = YieldRow::from_result(&results[0]);
    let samples = &results[0].samples;
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let stats = row.stats.expect("forty samples have statistics");
    assert!((stats.mean - mean).abs() < 1.0e-12);
    assert_eq!(stats.min, 38.9);
    assert_eq!(
        stats.max,
        samples.iter().copied().fold(f64::NEG_INFINITY, f64::max)
    );
    assert!((stats.max - 43.2).abs() < 1.0e-9);
    assert!(stats.std_dev > 0.5 && stats.std_dev < 2.0);
    let cpk = stats.cpk.expect("a lower limit has a one-sided Cpk");
    assert!((cpk - (stats.mean - 40.0) / (3.0 * stats.std_dev)).abs() < 1.0e-9);
    assert_eq!((row.passing, row.total), (37, 40));
    assert_eq!(row.yield_percent(), Some(92.5));
    assert_eq!(row.verdict(), Verdict::Fail);
    assert_eq!(row.limit.limit_text(), "≥ 40 dB");
    assert_eq!(
        yield_summary(&results),
        Some(super::toolbar::Summary::failing("yield 92.5 % (37 / 40)"))
    );
}

#[test]
fn the_shown_histogram_is_the_readers_pick_or_the_lowest_yield() {
    let row = |output: &str, passing: usize| YieldRow {
        output: output.to_owned(),
        limit: SpecEntry {
            measurement: output.to_owned(),
            expression: String::new(),
            min: Some(0.0),
            max: None,
            unit: String::new(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
        stats: None,
        passing,
        total: 10,
    };
    let rows = [row("gain", 9), row("bw", 7), row("pm", 10)];
    assert_eq!(shown_yield_row(&rows, None), Some(1));
    assert_eq!(shown_yield_row(&rows, Some("pm")), Some(2));
    assert_eq!(shown_yield_row(&rows, Some("gone")), Some(1));
    assert_eq!(shown_yield_row(&[], None), None);
}

/// The bins are laid so the limit is a bin edge: every sample in a failing
/// bar breaks the limit and every sample in a neutral bar meets it.
#[test]
fn histogram_bins_put_the_limit_on_an_edge_and_keep_every_sample() {
    let app = super::raster::monte_carlo_app();
    let results = fixture_yield_results(&app);
    let model = super::histogram::histogram_model(&results[0].samples, Some(40.0), None)
        .expect("forty samples bin");
    assert_eq!(model.counts.iter().sum::<usize>(), 40);
    assert!(
        model.edges.iter().any(|edge| (edge - 40.0).abs() < 1.0e-9),
        "the limit is an edge: {:?}",
        model.edges
    );
    let failing = model
        .counts
        .iter()
        .zip(&model.failing)
        .filter(|(_, failing)| **failing)
        .map(|(count, _)| count)
        .sum::<usize>();
    assert_eq!(failing, 3, "exactly the three samples under 40 dB");
    assert_eq!(model.limits, [40.0]);
    assert!(model.axis.0 <= model.mean - 3.0 * model.std_dev);
    assert!(model.axis.1 >= model.mean + 3.0 * model.std_dev);
    assert!((8..=48).contains(&model.counts.len()));
    assert!(model.ticks.len() >= 4);
    assert!(model.ticks.windows(2).all(|pair| pair[1] > pair[0]));
    assert_eq!(super::histogram::tick_text(39.5, 0.5, "dB"), "39.5 dB");
    assert!(super::histogram::histogram_model(&[f64::NAN], Some(1.0), None).is_none());
}

/// Bars are neutral and failing bars a muted error tone; nothing in the
/// histogram is painted in the accent.
#[test]
fn histogram_bars_are_neutral_and_failing_bars_muted_error() {
    let app = super::raster::monte_carlo_app();
    let results = fixture_yield_results(&app);
    let model = super::histogram::histogram_model(&results[0].samples, Some(40.0), None)
        .expect("forty samples bin");
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut fills = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 320.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    super::histogram::paint_histogram(ui, "gain", "dB", &model, 300.0);
                });
            },
        );
        fills = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(rect) => Some(rect.fill),
                _ => None,
            })
            .collect::<Vec<_>>();
    }
    let t = Tokens::get(&ctx);
    assert!(!fills.contains(&t.color.accent));
    assert!(fills.contains(&t.color.err.gamma_multiply(0.45)));
    assert!(fills.contains(&t.color.text_dim.gamma_multiply(0.38)));
}

/// The selected row's histogram sits in the pane under the table, not stacked
/// above it, and the table keeps the page's grammar.
#[test]
fn the_yield_page_draws_the_selected_histogram_under_the_table() {
    let geometry =
        super::raster::page_geometry(super::raster::monte_carlo_app(), VerificationPage::Yield);
    let rows = geometry.all("table.row");
    assert_eq!(rows.len(), 1);
    let split = geometry
        .one("split")
        .expect("the detail pane is split off the table");
    assert_eq!(split.top(), rows[0].bottom());
    let histogram = geometry
        .one("histogram")
        .expect("the selected row's histogram");
    assert_eq!(histogram.top(), split.bottom());
    assert!(
        (220.0..=340.0).contains(&histogram.height()),
        "{histogram:?}"
    );
}

// ---------------------------------------------------------------------------
// What the pages may not grow back
// ---------------------------------------------------------------------------

/// Every file the Verify pages are drawn from.
fn verify_slice() -> Vec<(&'static str, &'static str)> {
    vec![
        ("verify.rs", include_str!("../verify.rs")),
        ("verify/corners.rs", include_str!("corners.rs")),
        ("verify/histogram.rs", include_str!("histogram.rs")),
        ("verify/optimization.rs", include_str!("optimization.rs")),
        (
            "verify/regression_contract.rs",
            include_str!("regression_contract.rs"),
        ),
        (
            "verify/regression_panel.rs",
            include_str!("regression_panel.rs"),
        ),
        ("verify/safety.rs", include_str!("safety.rs")),
        ("verify/table.rs", include_str!("table.rs")),
        ("verify/toolbar.rs", include_str!("toolbar.rs")),
        ("verify/tuning.rs", include_str!("tuning.rs")),
        ("verify/yield_page.rs", include_str!("yield_page.rs")),
    ]
}

/// Colour on a Verify page is a problem, never a state.
///
/// A pass is the absence of one, so nothing here paints success: no status
/// dot, no `Success`/`Neutral` mark, no `ok` or `info` tone. A page that wants
/// to say a run is fine says it in the toolbar's words, in the text colour.
/// What colour is left is the failure and near-limit ink in a verdict cell,
/// the selected row's wash, and the one trace colour a candidate waveform is
/// drawn in.
#[test]
fn no_verify_page_paints_a_state_it_could_state_in_words() {
    for (path, source) in verify_slice() {
        let shipped = crate::source_guard::without_test_items(source);
        for banned in [
            "colored_label",
            "status_dot",
            "StatusMark::Success",
            "StatusMark::Neutral",
            "color.ok",
            "color.info",
        ] {
            assert!(
                !shipped.contains(banned),
                "{path} names `{banned}`: a Verify page colours a problem, not a state"
            );
        }
    }
}

/// The pages James called a generic dashboard do not come back.
///
/// Each page is its toolbar, one dense table and the detail of a selected
/// row. A tile of a big number with a subline under it, a lecturing subtitle
/// and a receipt reporting that an action worked are the shapes that made it
/// one, and the uppercase table head came with them.
#[test]
fn no_verify_page_grows_a_tile_strip_a_lecture_or_a_receipt_back() {
    for (path, source) in verify_slice() {
        let shipped = crate::source_guard::without_test_items(source);
        for (banned, why) in [
            ("KpiTile", "a tile of a big number is a dashboard, not a result"),
            ("kpi_strip", "a tile of a big number is a dashboard, not a result"),
            ("to_uppercase", "a table head is sentence case"),
            ("action_receipt", "an action that worked says nothing"),
            ("heading(ui", "a page's title is its toolbar's, and carries no subtitle"),
        ] {
            assert!(!shipped.contains(banned), "{path} names `{banned}`: {why}");
        }
    }
}
