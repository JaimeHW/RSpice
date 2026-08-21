//! What the preflight report is allowed to claim, and where each finding
//! sends the reader.
//!
//! Split from `preflight.rs`: the workflow and the evidence that it holds are
//! separate concerns, and the file was at its line budget.

use super::*;

fn blocker_report(observed: &str) -> PreflightReport {
    PreflightReport {
        project_revision: 7,
        topology_root: "user/top/schematic".to_owned(),
        topology_revision: 11,
        topology_closure: vec![("user/top/schematic".to_owned(), 11)],
        simulation_plan_id: None,
        simulation_plan_revision: None,
        blockers: vec![PreflightIssue {
            check: "Source and netlist currentness".to_owned(),
            observed: observed.to_owned(),
            required: "A current validated input with an exact source closure".to_owned(),
            remediation: PreflightRemediation::DesignChecks,
        }],
        advisories: Vec::new(),
        prepared: None,
    }
}

fn action_bounds(width: f32) -> egui::accesskit::Rect {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(width, 900.0))),
        ..egui::RawInput::default()
    };
    let report = blocker_report(
        "Generated input revision differs from the validated dependency closure and must wrap safely.",
    );
    let output = ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default()
            .frame(Frame::new())
            .show(ctx, |ui| {
                let mut requested_fix = None;
                blocker_list(ui, &report, &mut requested_fix);
            });
    });
    output
        .platform_output
        .accesskit_update
        .expect("AccessKit update")
        .nodes
        .iter()
        .find_map(|(_, node)| {
            (node.role() == egui::accesskit::Role::Button
                && node
                    .label()
                    .is_some_and(|label| label.starts_with("Run source checks for blocker:")))
            .then(|| node.bounds())
            .flatten()
        })
        .expect("preflight remediation button bounds")
}

fn prepared_contract() -> PreparedPreflightContract {
    PreparedPreflightContract {
        snapshot_digest: crate::product::ContentDigest::from_bytes([1; 32]),
        source_digest: crate::product::ContentDigest::from_bytes([2; 32]),
        receipt_digest: crate::product::ContentDigest::from_bytes([3; 32]),
        receipt_label: "receipt",
        analysis_ids: vec![crate::product::ContentDigest::from_bytes([4; 32])],
        task_count: 1,
        saved_output_contract_count: 0,
        pvt_point_count: 1,
        target: "Desktop background thread",
        save_policy: "Retain engine-produced results",
        model_identity_count: 1,
    }
}

fn disable_global_process_axis(state: &mut AppState) {
    for dimension in &mut state.sim_setup.run_set.dimensions {
        if dimension.kind == crate::simulation::run_set::RunSetDimensionKind::ProcessSection {
            dimension.enabled = false;
        }
    }
}

#[test]
fn run_command_remains_actionable_and_surfaces_preflight_blockers() {
    let mut app = RSpiceApp::test_instance();
    app.state.schematic = crate::state::SchematicState::default();

    assert!(Command::RunSimulation.is_enabled(&app));
    Command::RunSimulation.execute(&mut app);

    // Run records the request rather than calling the workflow, so that
    // the command vocabulary does not have to name a module above it.
    // `frame` serves it in the same pass; this is that hop.
    assert!(
        app.state.workbench.preflight.take_run_and_queue_request(),
        "Run must record a preflight-and-queue request for the frame loop"
    );
    run_and_queue(&mut app);

    assert!(!app.state.simulation.trigger_simulation);
    let report = app
        .state
        .workbench
        .preflight
        .report
        .as_ref()
        .expect("Run retains the same blocked report as explicit preflight");
    assert!(!report.blockers.is_empty());
    assert!(app.state.workbench.preflight.open);
    let toast = app
        .state
        .workbench
        .preflight
        .pending_toast
        .as_ref()
        .expect("blocked preflight retains its notification");
    assert!(
        toast.message.contains("design revision"),
        "preflight must distinguish the input-design revision from the simulation-plan revision: {}",
        toast.message
    );
}

#[test]
fn a_current_retained_report_queues_without_reauthoring_preflight() {
    let mut app = RSpiceApp::test_instance();
    let (topology_root, topology_revision, topology_closure) =
        app.state.configured_topology_revision();
    let current_plan = app.state.active_plan_revision();
    app.state.workbench.preflight.report = Some(PreflightReport {
        project_revision: app.state.workspace.project.revision().get(),
        topology_root,
        topology_revision,
        topology_closure,
        simulation_plan_id: current_plan.map(|(id, _)| id),
        simulation_plan_revision: current_plan.map(|(_, revision)| revision),
        blockers: Vec::new(),
        advisories: Vec::new(),
        prepared: Some(prepared_contract()),
    });

    assert!(queue_retained_run(&mut app));
    assert!(app.state.simulation.trigger_simulation);
    assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
}

#[test]
fn preflight_uses_the_mockup_workflow_geometry_and_local_breakpoints() {
    assert_eq!(PREFLIGHT_DIALOG_SIZE, DialogSize::SimulationWorkflow);
    assert_eq!(
        PreflightBodyLayout::resolve(760.0, 1_440.0),
        PreflightBodyLayout {
            issues: IssueLayout::Table,
            context: ContextLayout::Split,
        }
    );
    assert_eq!(
        PreflightBodyLayout::resolve(760.0, 760.0),
        PreflightBodyLayout {
            issues: IssueLayout::Table,
            context: ContextLayout::Stacked,
        }
    );
    assert_eq!(
        PreflightBodyLayout::resolve(390.0, 390.0),
        PreflightBodyLayout {
            issues: IssueLayout::Records,
            context: ContextLayout::Stacked,
        }
    );
}

#[test]
fn advisories_header_is_full_bleed_while_only_its_body_is_inset() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(400.0, 160.0))),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(Frame::NONE)
                .show(ctx, |ui| {
                    context_panel(ui, "Advisories", |ui| {
                        ui.label("Body content");
                    });
                });
        },
    );
    let header = output
        .platform_output
        .accesskit_update
        .expect("accessibility update")
        .nodes
        .iter()
        .find_map(|(_, node)| {
            (node.role() == egui::accesskit::Role::Label && node.label() == Some("Advisories"))
                .then(|| node.bounds())
                .flatten()
        })
        .expect("advisories header bounds");

    assert!(header.x0 <= 0.5, "header left edge was inset: {header:?}");
    assert!(
        header.x1 >= 399.5,
        "header right edge was inset: {header:?}"
    );
}

#[test]
fn issue_table_regions_are_contiguous_and_non_overlapping_at_the_boundary() {
    let geometry = IssueTableGeometry::resolve(680.0, 104.0);
    assert_eq!(geometry.column_edges[0], 0.0);
    assert!((geometry.column_edges[5] - 680.0).abs() < f32::EPSILON);
    assert!(
        geometry
            .column_edges
            .windows(2)
            .all(|edges| edges[0] < edges[1])
    );
    for cells in geometry.column_edges.windows(3) {
        assert!(cells[1] >= cells[0] && cells[2] >= cells[1]);
    }
}

#[test]
fn remediation_action_stays_inside_wide_table_and_phone_record_surfaces() {
    for width in [680.0_f32, 390.0] {
        let bounds = action_bounds(width);
        assert!(
            bounds.x0 >= 0.0,
            "button started outside {width}: {bounds:?}"
        );
        assert!(
            bounds.x1 <= f64::from(width) + 0.5,
            "button overflowed {width}: {bounds:?}"
        );
        assert!(bounds.x1 > bounds.x0 && bounds.y1 > bounds.y0);
    }
}

#[test]
fn report_collects_all_independent_blocker_classes() {
    let mut state = AppState::default();
    disable_global_process_axis(&mut state);
    state.schematic = crate::state::SchematicState::default();
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("current project owns a stable plan");
    let transient_id = plan.instances()[0].id();
    plan.set_enabled(transient_id, false)
        .expect("the sole analysis disables");
    crate::workbench::menu_bar::run_design_rule_check(&mut state);

    let report = collect_report(&state);
    let (topology_root, topology_revision, topology_closure) = state.configured_topology_revision();

    assert!(!report.is_runnable_for(
        state.workspace.project.revision().get(),
        &topology_root,
        topology_revision,
        &topology_closure,
        state.active_plan_revision(),
    ));
    assert!(
        report
            .blockers
            .iter()
            .any(|issue| issue.check == "Design topology")
    );
    assert!(report.blockers.iter().any(|issue| {
        issue.check == "Analysis-instance graph"
            && issue.observed == "Enable at least one analysis instance."
            && issue.remediation == PreflightRemediation::SimulationPlan
    }));
    assert!(
        !report
            .blockers
            .iter()
            .any(|issue| issue.check == "Project technology contract"),
        "a typical plan demands no technology: {:?}",
        report.blockers
    );
    assert!(
        report
            .advisories
            .iter()
            .any(|advisory| advisory.message == TECHNOLOGY_NOT_REQUIRED_ADVISORY),
        "{:?}",
        report.advisories
    );
}

#[test]
fn a_non_typical_reference_process_is_the_only_row_that_owns_the_missing_section() {
    let mut state = AppState::default();
    disable_global_process_axis(&mut state);
    state
        .sim_setup
        .set_reference_pvt(crate::product::ProcessCorner::SS, 27.0)
        .expect("the reference point is valid");

    let report = collect_report(&state);

    let technology: Vec<&PreflightIssue> = report
        .blockers
        .iter()
        .filter(|issue| issue.check == "Project technology contract")
        .collect();
    assert_eq!(technology.len(), 1, "{technology:?}");
    assert!(technology[0].observed.contains("SS"), "{technology:?}");
    assert_eq!(
        technology[0].remediation,
        PreflightRemediation::ProjectTechnology
    );
    assert!(
        !report
            .blockers
            .iter()
            .any(|issue| issue.check == "Reference model binding"),
        "the demand row already owns the missing reference section: {:?}",
        report.blockers
    );
}

/// A run outside a corner's qualified range warns; it does not refuse.
///
/// It is the foundry that declines to vouch for the point, not the tool
/// that can prove it wrong — so the report says so, offers the page where
/// the range is authored, and lets the run proceed.
#[test]
fn a_run_outside_a_corner_s_qualified_range_advises_rather_than_blocks() {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    let mut library = crate::state::model_library::ModelLibrary::new("pdk");
    let mut corner = crate::state::model_library::ProcessCorner::new("hot");
    corner.minimum_temperature_c = Some(-40.0);
    corner.maximum_temperature_c = Some(125.0);
    library.corners.clear();
    library.corners.insert("hot".to_owned(), corner);
    library.selected_corner = Some("hot".to_owned());
    state.model_library_manager.add_library(library);

    // The reference point is inside the range, so nothing is said.
    state.sim_setup.reference_pvt.temperature_celsius = 27.0;
    assert!(
        temperature_validity_advisories(&state).is_empty(),
        "a corner that covers the run set is silent"
    );

    state.sim_setup.reference_pvt.temperature_celsius = 150.0;
    let advisories = temperature_validity_advisories(&state);
    assert_eq!(advisories.len(), 1);
    assert!(
        advisories[0].message.contains("hot")
            && advisories[0].message.contains("-40.000 to 125.000 °C")
            && advisories[0].message.contains("150 °C"),
        "the advisory names the corner, its range and the request: {}",
        advisories[0].message
    );
    assert_eq!(
        advisories[0].remediation,
        Some(PreflightRemediation::model_corner(
            "pdk",
            Some("hot".to_owned())
        )),
        "and routes to the exact corner it named, in the library that holds it"
    );

    // The endpoint itself is qualified, which is the rule the corner owns.
    state.sim_setup.reference_pvt.temperature_celsius = 125.0;
    assert!(temperature_validity_advisories(&state).is_empty());

    // And it is an advisory: the report it lands in blocks on nothing.
    state.sim_setup.reference_pvt.temperature_celsius = 150.0;
    let report = collect_report(&state);
    assert!(
        report
            .advisories
            .iter()
            .any(|advisory| advisory.message.contains("qualified")),
        "{:?}",
        report.advisories
    );
    assert!(
        !report
            .blockers
            .iter()
            .any(|blocker| blocker.observed.contains("qualified")),
        "temperature validity never blocks a run"
    );
}

/// The sections an enabled corner analysis will materialize are the plan's, so
/// preflight states them once — as the plan's demand, not the instance's.
#[test]
fn an_enabled_corner_analysis_demands_the_plans_non_typical_sections() {
    let mut state = AppState::default();
    for dimension in &mut state.sim_setup.run_set.dimensions {
        match dimension.kind {
            crate::simulation::run_set::RunSetDimensionKind::ProcessSection => {
                dimension.enabled = true;
            }
            crate::simulation::run_set::RunSetDimensionKind::Supply => {
                dimension.source = format!(
                    "{}VDD",
                    crate::simulation::run_set::NETLIST_SUPPLY_SOURCE_PREFIX
                );
            }
            _ => {}
        }
    }
    let plan = state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("a default state owns a stable plan");
    let position = plan.instances().len();
    plan.insert_draft_with_id(
        crate::product::AnalysisInstanceId::new(),
        crate::simulation::plan::AnalysisDraft::Corner(
            crate::simulation::dialog::corner::CornerDialogState::default(),
        ),
        true,
        position,
    )
    .expect("a corner analysis has no prerequisites");

    let report = collect_report(&state);

    let technology = report
        .blockers
        .iter()
        .find(|issue| issue.check == "Project technology contract")
        .expect("the declared space demands its non-typical sections");
    assert!(technology.observed.contains("SS, FF"), "{technology:?}");
    assert_eq!(
        technology.remediation,
        PreflightRemediation::ProjectTechnology
    );
}

#[test]
fn an_unaudited_technology_binding_blocks_on_reattachment_without_faking_drift() {
    let mut state = AppState::default();
    state.provision_test_project_technology_contract();
    // A project copy retains the exact binding and starts a fresh audit
    // history, which is precisely the binding-without-receipts state.
    state.workspace.project = state
        .workspace
        .project
        .fork_copy_at(std::path::PathBuf::from("preflight_copy.rspiceproj"));
    assert!(state.workspace.project.technology_binding().is_some());
    assert!(state.workspace.project.technology_change_audit().is_empty());

    let report = collect_report(&state);

    let technology = report
        .blockers
        .iter()
        .find(|issue| issue.check == "Project technology contract")
        .expect("a binding without receipts blocks preflight");
    assert_eq!(
        technology.observed,
        "The attached technology binding predates checkpoint-backed authority receipts"
    );
    assert_eq!(
        technology.required,
        "Reattach the technology to record an audited change receipt"
    );
    assert_eq!(
        technology.remediation,
        PreflightRemediation::ProjectTechnology
    );
    assert!(
        !report.blockers.iter().any(|issue| {
            issue.check == "Project model technology" || issue.check == "Project signed PDK"
        }),
        "the retained binding still resolves exactly: {:?}",
        report.blockers
    );
}

#[test]
fn a_valid_attached_technology_reports_no_technology_row_at_all() {
    let mut state = AppState::default();
    state.provision_test_project_technology_contract();

    let report = collect_report(&state);

    assert!(
        !report.blockers.iter().any(|issue| {
            issue.check == "Project technology contract"
                || issue.check == "Project model technology"
                || issue.check == "Project signed PDK"
        }),
        "a valid contract is silent: {:?}",
        report.blockers
    );
    assert!(
        !report
            .advisories
            .iter()
            .any(|advisory| advisory.message == TECHNOLOGY_NOT_REQUIRED_ADVISORY),
        "{:?}",
        report.advisories
    );
}

#[test]
fn report_is_bound_to_the_exact_project_topology_and_plan_revision() {
    let mut state = AppState::default();
    crate::workbench::menu_bar::run_design_rule_check(&mut state);

    let report = collect_report(&state);

    assert_eq!(
        report.project_revision,
        state.workspace.project.revision().get()
    );
    let (plan_id, plan_revision) = state.active_plan_revision().expect("active plan");
    let (topology_root, topology_revision, topology_closure) = state.configured_topology_revision();
    assert_eq!(report.topology_root, topology_root);
    assert_eq!(report.topology_revision, topology_revision);
    assert_eq!(report.topology_closure, topology_closure);
    assert_eq!(report.simulation_plan_id, Some(plan_id));
    assert_eq!(report.simulation_plan_revision, Some(plan_revision));
    assert!(report.is_current_for(
        state.workspace.project.revision().get(),
        &topology_root,
        topology_revision,
        &topology_closure,
        Some((plan_id, plan_revision)),
    ));

    let transient_id = state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .instances()[0]
        .id();
    state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("stable plan")
        .edit(transient_id, |_| ())
        .expect("analysis edit advances the plan revision");
    assert!(!report.is_current_for(
        state.workspace.project.revision().get(),
        &topology_root,
        topology_revision,
        &topology_closure,
        state.active_plan_revision(),
    ));
}

#[test]
fn report_currentness_tracks_the_configured_root_not_an_unrelated_active_editor() {
    let mut state = AppState::default();
    let configured_root = state.workspace.simulation_root_reference();
    let configured_root_key = configured_root.key();
    let configured_schematic = state
        .workspace
        .schematic_buffers
        .get_mut(&configured_root_key)
        .expect("default configured root buffer");
    configured_schematic.add_component(
        crate::state::ComponentType::Ground,
        crate::state::Point::new(40, 40),
    );
    let configured_revision = configured_schematic.topology_version();

    state.workspace.active_view =
        crate::state::CellViewRef::new("user", "unrelated_editor", "schematic");
    state.schematic = crate::state::SchematicState::default();
    state.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::new(80, 80),
    );

    let report = collect_report(&state);
    assert_eq!(report.topology_root, configured_root_key);
    assert_eq!(report.topology_revision, configured_revision);

    state.schematic.add_component(
        crate::state::ComponentType::Capacitor,
        crate::state::Point::new(120, 80),
    );
    let (live_root, live_revision, live_closure) = state.configured_topology_revision();
    assert!(report.is_current_for(
        state.workspace.project.revision().get(),
        &live_root,
        live_revision,
        &live_closure,
        state.active_plan_revision(),
    ));
}

#[test]
fn report_currentness_expires_when_a_referenced_child_topology_changes() {
    let mut state = AppState::default();
    let child = crate::state::CellViewRef::new("user", "child", "schematic");
    state.schematic.add_library_cell_component(
        crate::state::Point::new(40, 40),
        crate::state::LibraryCellInstance::new("user", "child", "schematic"),
    );
    let mut child_schematic = crate::state::SchematicState::default();
    child_schematic.add_component(
        crate::state::ComponentType::Ground,
        crate::state::Point::new(20, 20),
    );
    state
        .workspace
        .schematic_buffers
        .insert(child.key(), child_schematic.clone());

    let report = collect_report(&state);
    let root_revision = report.topology_revision;
    assert!(
        report
            .topology_closure
            .iter()
            .any(|(key, _)| key.eq_ignore_ascii_case(&child.key()))
    );

    // A real editor transition persists the departing active schematic
    // before activating the child view. Mirror that contract explicitly;
    // otherwise this synthetic state would replace the edited root with
    // the stale default buffer and test a transition the UI cannot make.
    state.workspace.schematic_buffers.insert(
        crate::state::CellViewRef::default_top().key(),
        state.schematic.clone(),
    );
    state.workspace.active_view = child;
    state.schematic = child_schematic;
    state.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::new(80, 20),
    );
    let (live_root, live_revision, live_closure) = state.configured_topology_revision();
    assert_eq!(live_revision, root_revision);
    assert!(!report.is_current_for(
        state.workspace.project.revision().get(),
        &live_root,
        live_revision,
        &live_closure,
        state.active_plan_revision(),
    ));
}

#[test]
fn unresolved_hierarchy_is_an_ordered_preflight_blocker() {
    let mut state = AppState::default();
    state.schematic.add_library_cell_component(
        crate::state::Point::new(20, 20),
        crate::state::LibraryCellInstance::new("missing_library", "missing_master", "schematic"),
    );
    assert!(
        state
            .workspace
            .schematic_buffers
            .get(&crate::state::CellViewRef::default_top().key())
            .expect("persisted root schematic")
            .components
            .is_empty(),
        "fixture must remain unsynchronized to exercise the live overlay"
    );
    crate::workbench::menu_bar::run_design_rule_check(&mut state);

    let report = collect_report(&state);

    let hierarchy = report
        .blockers
        .iter()
        .find(|issue| issue.check == "Hierarchy binding")
        .expect("unbound master blocks preflight");
    assert!(
        hierarchy
            .observed
            .contains("missing_library/missing_master")
    );
    assert_eq!(hierarchy.remediation, PreflightRemediation::DesignChecks);
}

/// The Solver surface states that a node without a DC path is refused at
/// preflight. The report is where that statement has to become true, or
/// the author learns it from the engine after the run was queued.
#[test]
fn a_topology_the_engine_refuses_is_an_ordered_preflight_blocker() {
    use crate::state::{ComponentType, Point, Wire};

    let mut state = AppState::default();
    state
        .schematic
        .add_component(ComponentType::Ground, Point::new(0, 40));
    state
        .schematic
        .add_component(ComponentType::VoltageSource, Point::new(0, 10));
    state
        .schematic
        .add_component(ComponentType::Capacitor, Point::new(100, -10));
    state
        .schematic
        .wires
        .push(Wire::segment(1, Point::new(0, -10), Point::new(80, -10)));

    let report = collect_report(&state);

    let topology = report
        .blockers
        .iter()
        .find(|issue| issue.check == "Nodes without a DC path")
        .expect("the capacitor's far terminal reaches no reference");
    assert!(
        topology
            .observed
            .contains("No DC path to ground from node(s)"),
        "{topology:?}"
    );
    assert_eq!(topology.remediation, PreflightRemediation::DesignChecks);
}

#[test]
fn visible_mockup_labels_are_exact_and_do_not_expose_internal_contract_rows() {
    assert_eq!(
        ISSUE_TABLE_HEADERS,
        ["Order", "Check", "Observed", "Required", "Action"]
    );
    assert_eq!(CLEAN_CHECK, "No blocking issues.");
    assert_eq!(EMPTY_CELL, "—");
    assert_eq!(
        FROZEN_DISPATCH_ROWS,
        [
            "Revision",
            "Analysis identities",
            "PVT points",
            "Tasks",
            "Target"
        ]
    );
    assert_eq!(
        remediation_label(&PreflightRemediation::DesignChecks),
        "Run source checks"
    );
    assert_eq!(
        remediation_label(&PreflightRemediation::SimulationPlan),
        "Open plan"
    );
    assert_eq!(
        remediation_label(&PreflightRemediation::ProjectTechnology),
        "Attach technology"
    );
}

#[test]
fn summary_copy_and_blocker_action_mapping_match_the_mockup_contract() {
    let runnable = PreflightReport {
        project_revision: 7,
        topology_root: "user/top/schematic".to_owned(),
        topology_revision: 11,
        topology_closure: vec![("user/top/schematic".to_owned(), 11)],
        simulation_plan_id: None,
        simulation_plan_revision: None,
        blockers: Vec::new(),
        advisories: Vec::new(),
        prepared: Some(prepared_contract()),
    };
    assert_eq!(summary_heading(&runnable), RUNNABLE_HEADING);
    assert_eq!(
        RUNNABLE_SUMMARY,
        "Immutable inputs, target, task graph, and save policy are ready for dispatch."
    );
    assert_eq!(
        BLOCKED_SUMMARY,
        "The run was not queued. Resolve the ordered issues below, then rerun preflight."
    );

    let issue = |check: &str| PreflightIssue {
        check: check.to_owned(),
        observed: "blocked".to_owned(),
        required: "fixed".to_owned(),
        remediation: PreflightRemediation::DesignChecks,
    };
    let one = PreflightReport {
        blockers: vec![issue("Source closure")],
        prepared: None,
        ..runnable.clone()
    };
    assert_eq!(summary_heading(&one), "1 blocking issue");
    let two = PreflightReport {
        blockers: vec![issue("Model bindings"), issue("Executable netlist")],
        prepared: None,
        ..runnable
    };
    assert_eq!(summary_heading(&two), "2 blocking issues");

    use crate::simulation::execution::PreparationStage;
    for stage in [
        PreparationStage::DesignChecks,
        PreparationStage::SourceChecks,
    ] {
        assert_eq!(
            preparation_remediation(stage, None),
            PreflightRemediation::DesignChecks
        );
    }
    // A model binding is repaired in Models, not by the source checks.
    assert_eq!(
        preparation_remediation(PreparationStage::ModelBindings, None),
        PreflightRemediation::models_page(ModelsPage::Corners)
    );
    // And a netlist-stage failure is repaired in the deck.
    assert_eq!(
        preparation_remediation(PreparationStage::Netlist, None),
        PreflightRemediation::NetlistSource { line: None }
    );
    for stage in [
        PreparationStage::AnalysisPlan,
        PreparationStage::Authorization,
    ] {
        assert_eq!(
            preparation_remediation(stage, None),
            PreflightRemediation::SimulationPlan
        );
    }
}

/// A configuration whose stop view names a schematic.
///
/// A schematic is not a terminal implementation, so the resolver descends
/// into it and records a warning on the otherwise resolved binding: the
/// configuration is honoured differently than it reads.
fn state_with_a_stop_view_warning() -> AppState {
    let mut state = AppState::default();
    let mut work = crate::state::Library::new("work");
    let mut amp = crate::state::Cell::new("amp");
    amp.add_view(crate::state::View::new(
        "schematic",
        crate::state::ViewType::Schematic,
    ));
    work.add_cell(amp);
    state.library_manager.add_library(work);

    let mut master = crate::state::SchematicState::default();
    master.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::new(30, 0),
    );
    state
        .workspace
        .schematic_buffers
        .insert("work/amp/schematic".to_owned(), master);

    let binding = crate::state::LibraryCellInstance::new("work", "amp", "schematic");
    let instance = state
        .schematic
        .add_library_cell_component(crate::state::Point::new(100, 0), binding);
    state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == instance)
        .expect("the placed instance is retained")
        .name = "X1".to_owned();
    state.sync_active_schematic_to_workspace();

    let root = state.workspace.active_view.clone();
    state
        .workspace
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Stop at a schematic".to_owned(),
            root,
            dut_path: "/X1".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: vec!["schematic".to_owned()],
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "projection consumer test".to_owned(),
        })
        .expect("the fixture configuration is well formed");
    state
}

#[test]
fn a_configuration_honoured_differently_than_it_reads_is_a_preflight_advisory() {
    let state = state_with_a_stop_view_warning();

    let report = collect_report(&state);

    assert!(
        report
            .advisories
            .iter()
            .any(|advisory| advisory.message.contains("stop view")),
        "preflight states the configuration warning before the run: {:?}",
        report.advisories
    );
}

#[test]
fn a_model_binding_blocker_remediates_to_corners_and_sections() {
    use crate::simulation::execution::PreparationStage;

    // Both model-binding blockers filed `DesignChecks`, whose button says
    // "Run source checks" and opens Verify. Re-running the checks that
    // reported the failure cannot repair a binding; the bindings live in
    // Models → Corners & sections.
    let expected = PreflightRemediation::models_page(ModelsPage::Corners);
    assert_eq!(
        preparation_remediation(PreparationStage::ModelBindings, None),
        expected
    );
    assert_eq!(remediation_label(&expected), "Open Corners & sections");

    // A library whose selected corner it does not define fails to
    // materialize the reference process, which is the row the other site
    // files.
    let mut state = AppState::default();
    disable_global_process_axis(&mut state);
    let name = state
        .model_library_manager
        .load_library_bytes(
            "reference-binding.lib",
            b".lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n".to_vec(),
            None,
        )
        .expect("the fixture source parses");
    state
        .model_library_manager
        .get_library_mut(&name)
        .expect("the fixture library is retained")
        .selected_corner = Some("zz".to_owned());
    let report = collect_report(&state);
    let binding = report
        .blockers
        .iter()
        .find(|issue| issue.check == "Reference model binding")
        .unwrap_or_else(|| panic!("an unresolvable reference section is a blocker: {report:?}"));
    assert_eq!(binding.remediation, expected, "{binding:?}");

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.workbench.activate(Workspace::Verify);
    apply_remediation(&mut app, binding.remediation.clone());
    assert_eq!(app.state.workbench.workspace, Workspace::Models);
    assert_eq!(app.state.workbench.models_page, ModelsPage::Corners);
}

/// A finding that names a library and a corner has to land on them.
///
/// Opening Corners & sections without adopting the selection left the
/// reader to find, in a page that shows one library's matrix at a time,
/// the exact object the sentence they had just read named.
#[test]
fn a_corner_finding_opens_corners_and_sections_on_the_object_it_named() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.model_library_manager.clear();
    let mut library = crate::state::model_library::ModelLibrary::new("pdk");
    let mut corner = crate::state::model_library::ProcessCorner::new("hot");
    corner.minimum_temperature_c = Some(-40.0);
    corner.maximum_temperature_c = Some(125.0);
    library.corners.clear();
    library.corners.insert("hot".to_owned(), corner);
    library.selected_corner = Some("hot".to_owned());
    app.state.model_library_manager.add_library(library);
    app.state
        .model_library_manager
        .add_library(crate::state::model_library::ModelLibrary::new("other"));
    app.state.model_library_manager.select_library("other");
    app.state.sim_setup.reference_pvt.temperature_celsius = 150.0;

    let advisories = temperature_validity_advisories(&app.state);
    let remediation = advisories[0]
        .remediation
        .clone()
        .expect("a corner advisory names where its range is authored");
    app.state.workbench.activate(Workspace::Verify);
    apply_remediation(&mut app, remediation);

    assert_eq!(app.state.workbench.workspace, Workspace::Models);
    assert_eq!(app.state.workbench.models_page, ModelsPage::Corners);
    assert_eq!(
        app.state.model_library_manager.selected_library.as_deref(),
        Some("pdk"),
        "the page must arrive on the library the advisory named"
    );
    assert_eq!(
        app.state.workbench.models_view.selected_corner.as_deref(),
        Some("pdk\u{1f}hot"),
        "and with the corner it named inspected"
    );
}

/// A netlist-stage preparation failure is a defect in the deck.
///
/// It used to file `DesignChecks`, whose button said "Run source checks"
/// and whose destination was the Verify workspace's Yield page — the
/// checks had already passed, which is how preparation reached the
/// netlist stage at all, and Yield names nothing about the deck.
#[test]
fn a_netlist_stage_failure_opens_the_deck_rather_than_re_running_design_checks() {
    use crate::simulation::execution::PreparationStage;

    let remediation = preparation_remediation(PreparationStage::Netlist, Some(42));
    assert_eq!(
        remediation,
        PreflightRemediation::NetlistSource { line: Some(42) }
    );
    assert_eq!(remediation_label(&remediation), "Open netlist source");
    assert!(
        remediation.blocks_executable_netlist(),
        "the preflight strip's Netlist cell still has to see it"
    );

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.workbench.activate(Workspace::Verify);
    app.state.ui.code_workspace.page =
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation;
    apply_remediation(&mut app, remediation);

    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
    assert_eq!(
        app.state.ui.code_workspace.page,
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
    );
    assert_eq!(
        app.state.ui.netlist.requested_line,
        Some(42),
        "a finding that named a line must land on it, not merely open the deck"
    );
}

/// A located finding is a jump; an unlocated one is a row.
///
/// The row-level click is offered only where the failure named a line,
/// because that is the only case where pressing the row means something more
/// than the Action button already does. Every row keeps that button, so
/// nothing is taken away from the findings that stay rows.
#[test]
fn only_a_blocker_naming_a_line_makes_its_row_a_jump() {
    use crate::simulation::execution::PreparationStage;

    assert_eq!(
        remediation_source_line(&preparation_remediation(PreparationStage::Netlist, Some(7))),
        Some(7)
    );
    assert_eq!(
        remediation_source_line(&preparation_remediation(PreparationStage::Netlist, None)),
        None,
        "a netlist failure that reported no line offers no jump"
    );
    for stage in [
        PreparationStage::DesignChecks,
        PreparationStage::SourceChecks,
        PreparationStage::ModelBindings,
        PreparationStage::AnalysisPlan,
        PreparationStage::Authorization,
    ] {
        assert_eq!(
            remediation_source_line(&preparation_remediation(stage, Some(7))),
            None,
            "{stage:?} names a workspace, not a line — a line offered to it \
             would be a coordinate in the wrong document"
        );
    }
}

/// The parser numbers a deck from 1; the buffer indexes it from 0. A
/// conversion missed here puts the cursor one line off the defect, which is
/// the kind of wrong that reads as right.
#[test]
fn a_named_line_survives_the_parser_to_buffer_conversion() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;

    apply_remediation(
        &mut app,
        PreflightRemediation::NetlistSource { line: Some(1) },
    );

    assert_eq!(app.state.ui.netlist.cursor_line, 0);
    assert_eq!(app.state.ui.netlist.requested_line, Some(1));
}
