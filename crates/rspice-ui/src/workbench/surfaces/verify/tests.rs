//! Tests for regression export and tolerance contracts.
//!
//! The cases are fail-closed: an empty matrix still exports a blocked case,
//! XML control characters are refused rather than escaped away, and a receipt
//! whose tolerance digest does not match its contract is rejected.

use super::*;
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SimulationState};

#[test]
fn verification_split_panes_share_one_top_aligned_row() {
    let row = egui::Rect::from_min_size(egui::pos2(7.0, 11.0), egui::vec2(1_420.0, 336.0));
    let (left, right) = verification_split_rects(row, 645.0);
    assert_eq!(left.top(), row.top());
    assert_eq!(right.top(), row.top());
    assert_eq!(left.bottom(), row.bottom());
    assert_eq!(right.bottom(), row.bottom());
    assert_eq!(right.left() - left.right(), 1.0);
    assert_eq!(right.right(), row.right());
}

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

fn sealed_run(
    run_number: u64,
    plan_id: crate::product::SimulationPlanId,
    source_instance_id: AnalysisInstanceId,
    analysis_kind_tag: u8,
    analysis: AnalysisResult,
) -> SimulationRun {
    let snapshot = ContentDigest::from_bytes([0x5a; 32]);
    let task = crate::state::PreparedRunTaskReceipt::new(
        source_instance_id,
        ObjectRevision::INITIAL,
        Vec::new(),
        analysis_kind_tag,
        ContentDigest::from_bytes([analysis_kind_tag; 32]),
    )
    .unwrap();
    let receipt = crate::state::PreparedRunReceipt::new(
        crate::state::AnalysisResultSourceDomain::SimulationPlan,
        Some(plan_id),
        ObjectRevision::INITIAL,
        snapshot,
        ContentDigest::from_bytes([0x41; 32]),
        crate::state::PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes(
            [0x42; 32],
        )),
        vec![task],
    )
    .unwrap();
    let mut run = SimulationRun::new_prepared(run_number, receipt);
    run.add_analysis(attributed_with_id(analysis, source_instance_id));
    run
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
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .expect("component exists");
    let source_view = app.state.workspace.active_schematic_reference();
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
        crate::workbench::preflight::configured_topology_revision(&app.state);
    app.state.workbench.preflight = crate::workbench::state::PreflightDialogState {
        open: true,
        report: Some(crate::workbench::state::PreflightReport {
            project_revision: app.state.workspace.project.revision().get(),
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
            .active_plan_data(plan_id)
            .unwrap()
            .design_variables[0]
            .expression,
        "499 ohm"
    );
}

#[test]
fn tuning_commit_is_one_plan_revision_and_queues_the_required_run() {
    let mut app = RSpiceApp::test_instance();
    app.state.workspace.project_sources = Default::default();
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
        .active_plan_data(plan_id)
        .unwrap()
        .design_variables
        .iter()
        .find(|variable| variable.id == variable_id)
        .unwrap();
    assert_eq!(variable.expression, "620 ohm");
    assert_eq!(variable.revision, ObjectRevision::INITIAL.next().unwrap());
    assert!(app.state.simulation.trigger_simulation);
    assert!(!tuning_is_dirty(&app));
}

#[test]
fn reverting_a_literal_value_proposal_discards_variable_and_binding_only() {
    let mut app = RSpiceApp::test_instance();
    let component_id = app.state.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::origin(),
    );
    app.state.schematic.components[0].value = "10k".to_owned();
    let plan_id = app.state.sim_setup.stable_analysis_plan().unwrap().id();
    let workspace_before = serde_json::to_vec(&app.state.workspace).unwrap();
    let value_before = app.state.schematic.components[0].value.clone();
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
    assert_eq!(app.state.schematic.components[0].value, value_before);
    assert_eq!(
        serde_json::to_vec(&app.state.workspace).unwrap(),
        workspace_before
    );
    assert!(
        app.state
            .workspace
            .active_plan_data(plan_id)
            .unwrap()
            .design_variables
            .is_empty()
    );
}

#[test]
fn literal_value_commit_adds_variable_binds_once_and_dispatches_prepared_run() {
    let mut app = RSpiceApp::test_instance();
    app.state.workspace.project_sources = Default::default();
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    let component_id = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.kind == crate::state::ComponentType::Resistor)
        .map(|component| component.id)
        .expect("voltage divider resistor");
    let component_index = app
        .state
        .schematic
        .components
        .iter()
        .position(|component| component.id == component_id)
        .unwrap();
    app.state.schematic.components[component_index].name = "RLOAD".to_owned();
    app.state.schematic.components[component_index].value = "10k".to_owned();
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
        .active_plan_data(plan_id)
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
    assert!(app.state.simulation.trigger_simulation);
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
    app.state.workspace.project_sources = Default::default();
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    let component_id = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.kind == crate::state::ComponentType::Resistor)
        .map(|component| component.id)
        .expect("voltage divider resistor");
    let component_index = app
        .state
        .schematic
        .components
        .iter()
        .position(|component| component.id == component_id)
        .unwrap();
    app.state.schematic.components[component_index].name = "RLOCKED".to_owned();
    app.state.schematic.components[component_index].value = "10k".to_owned();
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
    let value_before = app.state.schematic.components[component_index]
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
        app.state.schematic.components[component_index].value,
        value_before
    );
    assert_eq!(app.state.schematic.can_undo(), can_undo_before);
    assert!(tuning_is_dirty(&app));
    assert!(!app.state.simulation.trigger_simulation);
}

#[test]
fn failed_literal_value_run_preparation_rolls_back_plan_and_schematic() {
    let mut app = RSpiceApp::test_instance();
    let component_id = app.state.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::origin(),
    );
    app.state.schematic.components[0].name = "RFAIL".to_owned();
    app.state.schematic.components[0].value = "10k".to_owned();
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
    assert_eq!(app.state.schematic.components[0].value, "10k");
    assert_eq!(app.state.schematic.can_undo(), source_can_undo);
    assert_eq!(
        serde_json::to_vec(&app.state.workspace).unwrap(),
        workspace_before
    );
    assert!(
        app.state
            .workspace
            .active_plan_data(plan_id)
            .unwrap()
            .design_variables
            .is_empty()
    );
    assert!(app.state.workbench.preflight.open);
    assert!(tuning_is_dirty(&app));
    assert!(!app.state.simulation.trigger_simulation);
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
        .active_plan_data(plan_id)
        .unwrap()
        .design_variables[0]
        .clone();
    candidate.expression.clone_from(&draft.candidate_expression);
    draft.validation_error = candidate.validate().err();

    assert!(tuning::commit_tuning_and_run(&mut app).is_err());
    assert_eq!(serde_json::to_vec(&app.state.workspace).unwrap(), before);
    assert!(!app.state.simulation.trigger_simulation);
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
            .active_plan_data(plan_id)
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
    assert!(!app.state.simulation.trigger_simulation);
}

#[test]
fn verify_layout_matches_desktop_tablet_and_phone_compositions() {
    let desktop = VerifyLayout::resolve(1_440.0, 820.0, 620.0, 5, 37.0, 28.0);
    assert!(desktop.split);
    assert!((desktop.left_width + 1.0 + desktop.right_width - 820.0).abs() < f32::EPSILON);
    assert_eq!(desktop.first_row_height, VERIFY_FIRST_ROW_MAX_HEIGHT);

    let compact_desktop = VerifyLayout::resolve(1_024.0, 744.0, 600.0, 5, 37.0, 28.0);
    assert!(!compact_desktop.split);

    let tablet = VerifyLayout::resolve(820.0, 768.0, 600.0, 5, 44.0, 31.0);
    assert!(!tablet.split);
    assert_eq!(tablet.left_width, 768.0);

    let phone = VerifyLayout::resolve(390.0, 390.0, 540.0, 5, 44.0, 36.0);
    assert!(!phone.split);
    assert_eq!(phone.right_width, 390.0);
}

#[test]
fn verification_first_row_height_is_finite_and_capped() {
    assert_eq!(
        bounded_verification_first_row_height(10_000.0, 100.0),
        VERIFY_FIRST_ROW_MAX_HEIGHT
    );
    assert_eq!(
        bounded_verification_first_row_height(100.0, 200.0),
        VERIFY_FIRST_ROW_MIN_HEIGHT
    );
    assert_eq!(
        bounded_verification_first_row_height(f32::INFINITY, 100.0),
        VERIFY_FIRST_ROW_MIN_HEIGHT
    );
    assert_eq!(
        bounded_verification_first_row_height(f32::NAN, 100.0),
        VERIFY_FIRST_ROW_MIN_HEIGHT
    );
}

#[test]
fn visible_height_depends_only_on_consumed_content_not_scroll_offset() {
    assert_eq!(remaining_viewport_height(600.0, -40.0), 600.0);
    assert_eq!(remaining_viewport_height(600.0, 0.0), 600.0);
    assert_eq!(remaining_viewport_height(600.0, 160.0), 440.0);
    assert_eq!(remaining_viewport_height(600.0, 640.0), 0.0);
}

#[test]
fn responsive_verify_geometry_matches_mockup_breakpoints() {
    assert_eq!(verification_status_columns(1_261.0), 4);
    assert_eq!(verification_status_columns(1_260.0), 2);
    assert_eq!(verification_status_columns(821.0), 2);
    assert_eq!(verification_status_columns(820.0), 2);
    assert_eq!(verification_status_columns(390.0), 2);

    assert_eq!(verification_stacked_chart_height(561.0), 250.0);
    assert_eq!(verification_stacked_chart_height(560.0), 230.0);

    assert_eq!(verification_table_row_height(1_440.0), 28.0);
    assert_eq!(verification_table_row_height(820.0), 31.0);
    assert_eq!(verification_table_row_height(560.0), 36.0);

    assert_eq!(verification_table_width(1_280.0, 718.0, None), 718.0);
    assert_eq!(verification_table_width(560.0, 390.0, None), 540.0);
    assert_eq!(
        verification_table_width(1_280.0, 718.0, Some(1_450.0)),
        1_450.0
    );
}

#[test]
fn reliability_projection_rows_preserve_numeric_axis_and_every_retained_metric() {
    let device = crate::state::ReliabilityDeviceEvidence {
        device_id: "M1".to_owned(),
        stress: crate::state::ReliabilityStressEvidence {
            average_gate_stress_v: 1.2,
            average_drain_stress_v: 1.8,
            average_temperature_k: 350.0,
            duration_s: 3_600.0,
        },
        checkpoints: vec![
            crate::state::ReliabilityCheckpointEvidence {
                years: 1.0,
                shift: crate::state::ReliabilityShiftEvidence {
                    threshold_voltage_shift_v: 0.01,
                    mobility_shift: -0.001,
                    drain_source_resistance_shift: 0.0005,
                },
            },
            crate::state::ReliabilityCheckpointEvidence {
                years: 10.0,
                shift: crate::state::ReliabilityShiftEvidence {
                    threshold_voltage_shift_v: 0.03,
                    mobility_shift: -0.004,
                    drain_source_resistance_shift: 0.0015,
                },
            },
        ],
    };

    let duration = reliability_projection_row(&device, 3);
    assert!(duration[0].text.contains("duration"));
    assert!(duration[1].text.contains("3600"));
    let vth = reliability_projection_row(&device, 4);
    assert!(vth[2].text.contains("0.010000"));
    assert!(vth[3].text.contains("0.030000"));
    let mobility = reliability_projection_row(&device, 5);
    assert_eq!(mobility[2].text, "-0.1000%");
    assert_eq!(mobility[3].text, "-0.4000%");
    let rds = reliability_projection_row(&device, 6);
    assert_eq!(rds[2].text, "+0.0500%");
    assert_eq!(rds[3].text, "+0.1500%");
}

#[test]
fn soa_cross_probe_selects_the_exact_schematic_device() {
    let mut app = RSpiceApp::test_instance();
    app.state.schematic.components.push(
        crate::state::Component::new(
            42,
            crate::state::ComponentType::Nmos,
            crate::state::Point::new(10, 20),
        )
        .with_name_value("M1", "NMOS"),
    );

    cross_probe_reliability_device(&mut app, "m1")
        .expect("device identity cross-probes case-insensitively");

    assert!(app.state.schematic.selection.has_component(42));
    assert_eq!(
        app.state.workbench.workspace,
        super::super::super::state::Workspace::Design
    );
    assert!(cross_probe_reliability_device(&mut app, "M404").is_err());
}

#[test]
fn histogram_bins_preserve_every_exact_sample() {
    let samples = [0.0, 0.2, 0.4, 0.8, 1.0];
    let bins = histogram_bins(&samples, 4, 0.0, 1.0);

    assert_eq!(bins.iter().sum::<usize>(), samples.len());
    assert_eq!(bins, vec![2, 1, 0, 2]);
}

#[test]
fn signed_margin_is_positive_inside_and_negative_outside_bounds() {
    let spec = SpecEntry {
        measurement: "gain".to_owned(),
        min: Some(10.0),
        max: Some(20.0),
        unit: "dB".to_owned(),
    };

    assert_eq!(signed_margin(&spec, 15.0), 5.0);
    assert_eq!(signed_margin(&spec, 8.0), -2.0);
    assert_eq!(signed_margin(&spec, 23.0), -3.0);
    assert_eq!(normalized_margin(&spec, 15.0), Some(0.5));
    assert_eq!(normalized_margin(&spec, 8.0), Some(-0.2));
}

#[test]
fn joint_sample_summary_requires_aligned_trails_and_ands_every_spec() {
    let make_result = |target: &str, trail: Vec<bool>| {
        let pass_count = trail.iter().filter(|passes| **passes).count();
        crate::services::yield_manager::YieldResult {
            spec: crate::services::yield_manager::YieldSpec::lower(target, 0.0, ""),
            total_runs: trail.len(),
            pass_count,
            fail_count: trail.len() - pass_count,
            yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
            stats: crate::services::yield_manager::DistributionStats::default(),
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
    assert_eq!(
        worst_individual_yield_result(&aligned)
            .expect("worst individual detail")
            .spec
            .target,
        "gain"
    );

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
        crate::services::yield_manager::YieldResult {
            spec: crate::services::yield_manager::YieldSpec::lower(target, 0.0, ""),
            total_runs: trail.len(),
            pass_count,
            fail_count: trail.len() - pass_count,
            yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
            stats: crate::services::yield_manager::DistributionStats::default(),
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
fn specification_editor_never_falls_through_to_an_inactive_dataset() {
    let mut inactive = SimulationRun::new(1);
    inactive.add_analysis(attributed(
        AnalysisResult::new(1, AnalysisType::Ac, "inactive")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 99.0)]),
    ));
    let mut active = SimulationRun::new(2);
    active.add_analysis(attributed(
        AnalysisResult::new(1, AnalysisType::Ac, "active")
            .with_measurements(vec![rspice_core::MeasureResult::success("bandwidth", 42.0)]),
    ));
    let active_dataset = active.dataset_id;
    let mut simulation = SimulationState {
        runs: vec![inactive, active],
        active_run_idx: Some(1),
        ..SimulationState::default()
    };

    assert_eq!(
        simulation.active_run().map(|run| run.dataset_id),
        Some(active_dataset)
    );
    assert_eq!(
        active_dataset_measurement(&simulation, "bandwidth"),
        Some(42.0)
    );
    assert_eq!(active_dataset_measurement(&simulation, "gain"), None);

    simulation.active_run_idx = None;
    assert_eq!(active_dataset_measurement(&simulation, "bandwidth"), None);
    assert_eq!(active_dataset_measurement(&simulation, "gain"), None);
}

#[test]
fn verification_requires_an_explicit_active_run_selection() {
    let mut app = RSpiceApp::test_instance();
    app.state.simulation.runs.push(SimulationRun::new(1));
    app.state.simulation.active_run_idx = None;

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

    assert_eq!(measurement_in_run(&run, "legacy"), None);
    assert_eq!(measurement_in_run(&run, "failed"), None);
    assert_eq!(measurement_in_run(&run, "measure_failed"), None);
    assert_eq!(measurement_in_run(&run, "valid"), Some(4.0));
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
