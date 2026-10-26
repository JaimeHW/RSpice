//! Tests for the context toolbar: run currentness, the mockup's command rows
//! and glyphs, and the responsive widths every breakpoint reserves.

use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
use crate::state::{
    AnalysisResultSourceDomain, ComponentType, PreparedRunReceipt, PreparedRunTaskReceipt,
    PreparedSourceCheckReceipt, SimulationRun, SimulationRunLifecycle, SimulationRunProvenance,
    Tool,
};
use crate::ui::panels::WorkbenchIcon;
use crate::workbench::RSpiceApp;
use crate::workbench::chrome::toolbar::design::{
    DESIGN_DIRECT_TOOLBAR_COMMANDS, design_toolbar_command_selected,
};
use crate::workbench::chrome::toolbar::results::{
    results_document_is_historical, run_has_current_success_authority,
};
use crate::workbench::chrome::toolbar::simulate::{
    commit_reference_pvt, pvt_menu_height_for_viewport,
};
use crate::workbench::chrome::toolbar::*;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::{WorkbenchSession, Workspace};
use egui::Vec2;

fn digest(byte: u8) -> ContentDigest {
    ContentDigest::from_bytes([byte; 32])
}

fn completed_prepared_run(
    project_revision: ObjectRevision,
    source_digest: ContentDigest,
) -> SimulationRun {
    let task = PreparedRunTaskReceipt::new(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        Vec::new(),
        5,
        digest(0x64),
    )
    .expect("task receipt");
    let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(SimulationPlanId::new()),
        project_revision,
        prepared_snapshot_digest: digest(0x61),
        source_content_digest: source_digest,
        source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x63)),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![task],
    })
    .expect("run receipt");
    let mut run = SimulationRun::new(1);
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
        .expect("run provenance");
    run.mark_running().expect("running lifecycle");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("completed lifecycle");
    run
}

#[test]
fn toolbar_currentness_rejects_stale_failed_and_nonterminal_runs() {
    let revision = ObjectRevision::INITIAL;
    let source = digest(0x72);
    let mut run = completed_prepared_run(revision, source);

    assert!(run_has_current_success_authority(
        &run,
        revision,
        Some(source)
    ));
    assert!(!run_has_current_success_authority(
        &run,
        ObjectRevision::new(revision.get() + 1).expect("next revision"),
        Some(source)
    ));
    assert!(!run_has_current_success_authority(
        &run,
        revision,
        Some(digest(0x73))
    ));
    assert!(!run_has_current_success_authority(&run, revision, None));

    run.success = false;
    assert!(!run_has_current_success_authority(
        &run,
        revision,
        Some(source)
    ));
    run.success = true;
    run.lifecycle = SimulationRunLifecycle::Running;
    assert!(!run_has_current_success_authority(
        &run,
        revision,
        Some(source)
    ));
}

#[test]
fn running_dataset_is_never_presented_as_current() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Results;
    let source = "R1 1 0 1k\n";
    let source_digest = crate::state::content_digest(source);
    let generation_input = digest(0x81);
    app.state.model.simulation.source.netlist_content = source.to_owned();
    app.state.model.netlist_artifacts.generated_input_digest = Some(generation_input);
    app.state
        .model
        .netlist_artifacts
        .current_generation_input_digest = Some(generation_input);
    let revision = app.state.model.workspace.content.project.revision();
    let run = completed_prepared_run(revision, source_digest);
    let dataset_id = run.dataset_id;
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.workbench.documents.activate(
        crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id),
    );

    assert!(!results_document_is_historical(app.state.results_reads()));
    app.state.model.simulation.retained.runs[0].lifecycle = SimulationRunLifecycle::Running;
    assert!(results_document_is_historical(app.state.results_reads()));
}

#[test]
fn missing_current_generated_source_never_authenticates_a_retained_dataset() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Results;
    let revision = app.state.model.workspace.content.project.revision();
    let run = completed_prepared_run(revision, digest(0x83));
    let dataset_id = run.dataset_id;
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.workbench.documents.activate(
        crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id),
    );

    assert!(results_document_is_historical(app.state.results_reads()));
}

#[test]
fn long_toolbar_labels_remain_bounded() {
    assert_eq!(explicit_label_width("Run"), 76.0);
    assert!(explicit_label_width("Run schematic checks (Ctrl+E)") > 162.0);
    assert!(explicit_label_width("A very long engineering command label") <= 224.0);
}

#[test]
fn run_control_label_tracks_stable_execution_and_cancellation_lifecycle() {
    let mut app = RSpiceApp::test_instance();
    let desktop = LayoutSpec::resolve(1_280.0, 900.0, &WorkbenchSession::default());
    assert_eq!(run_control_label(&app, desktop), "Run plan");

    let identity = app
        .state
        .model
        .simulation
        .start_run()
        .execution_identity()
        .expect("current run has execution identity");
    app.state.model.simulation.execution.active_execution = Some(identity);
    app.state.model.simulation.execution.is_running = false;
    assert_eq!(run_control_label(&app, desktop), "Stop");

    app.state
        .model
        .simulation
        .request_abort_active_run()
        .unwrap();
    assert_eq!(run_control_label(&app, desktop), "Stopping…");
}

#[test]
fn desktop_design_toolbar_matches_the_upgraded_mockup() {
    assert_eq!(
        DESIGN_DIRECT_TOOLBAR_COMMANDS.map(|(command, _, _)| command),
        [
            Command::SelectTool,
            Command::PlaceInstance,
            Command::PlaceWire,
            Command::PlaceBus,
            Command::PlaceBusTap,
            Command::PlaceJunction,
            Command::PlaceLabel,
            Command::PlaceOffSheetConnector,
            Command::PlacePin,
            Command::PlaceProbe,
            Command::PlaceText,
        ]
    );
}

/// The direct tools are a single unlabelled row, so a glyph is the only
/// thing distinguishing one arm from another while the pointer is
/// elsewhere. `DescendHierarchyDirect` sits in the same row, which is why
/// its glyph is checked here too.
#[test]
fn no_two_armed_schematic_tools_paint_the_same_glyph() {
    let icons = DESIGN_DIRECT_TOOLBAR_COMMANDS.map(|(_, icon, _)| icon);
    for (index, icon) in icons.iter().enumerate() {
        assert!(
            !icons[index + 1..].contains(icon),
            "{icon:?} is painted twice in the direct tool row"
        );
        assert_ne!(
            *icon,
            WorkbenchIcon::Layers,
            "Layers is the descend-hierarchy glyph of this same toolbar"
        );
    }
}

#[test]
fn schematic_tooltips_use_the_upgraded_mockup_copy() {
    assert_eq!(
        DESIGN_DIRECT_TOOLBAR_COMMANDS.map(|(_, _, label)| label),
        [
            "Select (Esc)",
            "Place instance",
            "Draw wire",
            "Draw bus",
            "Place bus tap",
            "Place junction",
            "Net label",
            "Off-sheet connector",
            "Create pin",
            "Probe signal",
            "Place text or note",
        ]
    );
}

#[test]
fn direct_schematic_toolbar_selection_tracks_every_armed_tool() {
    assert!(design_toolbar_command_selected(
        Tool::Place(ComponentType::Resistor),
        Command::PlaceInstance,
    ));
    assert!(!design_toolbar_command_selected(
        Tool::Place(ComponentType::Port),
        Command::PlaceInstance,
    ));
    assert!(design_toolbar_command_selected(
        Tool::Place(ComponentType::Port),
        Command::PlacePin,
    ));
    assert!(design_toolbar_command_selected(
        Tool::BusTap,
        Command::PlaceBusTap,
    ));
    assert!(design_toolbar_command_selected(
        Tool::Junction,
        Command::PlaceJunction,
    ));
    assert!(design_toolbar_command_selected(
        Tool::DesignNote,
        Command::PlaceText,
    ));
}

#[test]
fn responsive_toolbar_reserves_fixed_trailing_controls() {
    let tablet = LayoutSpec::resolve(834.0, 1112.0, &WorkbenchSession::default());
    let available_after_navigator = 780.0;
    assert_eq!(
        trailing_controls_width(tablet, 5.0, true, 117.0, 96.0),
        252.0
    );
    assert_eq!(
        context_tools_width(
            available_after_navigator,
            tablet,
            8.0,
            5.0,
            true,
            117.0,
            96.0,
        ),
        520.0
    );

    let phone = LayoutSpec::resolve(390.0, 844.0, &WorkbenchSession::default());
    assert_eq!(
        trailing_controls_width(phone, 2.0, true, 136.0, 106.0),
        185.0
    );
    assert_eq!(
        trailing_controls_width(phone, 2.0, false, 136.0, 106.0),
        46.0
    );
    assert!(context_tools_width(336.0, phone, 3.0, 2.0, true, 136.0, 106.0) > 0.0);
}

#[test]
fn toolbar_padding_and_group_gaps_match_every_mockup_breakpoint() {
    assert_eq!(toolbar_horizontal_padding(1_280.0), 8.0);
    assert_eq!(toolbar_horizontal_padding(820.0), 5.0);
    assert_eq!(toolbar_horizontal_padding(560.0), 3.0);
    assert_eq!(toolbar_outer_gap(1_280.0), 5.0);
    assert_eq!(toolbar_outer_gap(820.0), 5.0);
    assert_eq!(toolbar_outer_gap(560.0), 2.0);
}

#[test]
fn toolbar_icon_and_panel_toggle_boxes_match_the_mockup_classes() {
    let desktop = LayoutSpec::resolve(1_280.0, 900.0, &WorkbenchSession::default());
    assert_eq!(toolbar_icon_button_size(desktop), Vec2::new(30.0, 29.0));
    assert_eq!(panel_toggle_size(desktop), Vec2::new(28.0, 27.0));

    let phone = LayoutSpec::resolve(390.0, 844.0, &WorkbenchSession::default());
    assert_eq!(toolbar_icon_button_size(phone), Vec2::splat(44.0));
    assert_eq!(panel_toggle_size(phone), Vec2::splat(44.0));
}

#[test]
fn compact_toolbar_projects_the_mockup_button_limit_for_every_workspace() {
    let compact = LayoutSpec::resolve(390.0, 844.0, &WorkbenchSession::default());
    assert_eq!(compact.toolbar_tool_limit, Some(2));
}

#[test]
fn pvt_menu_height_is_bounded_for_short_and_touch_viewports() {
    assert_eq!(pvt_menu_height_for_viewport(900.0, 29.0), 484.0);
    assert_eq!(pvt_menu_height_for_viewport(600.0, 44.0), 484.0);
    assert_eq!(pvt_menu_height_for_viewport(140.0, 44.0), 132.0);
}

#[test]
fn reference_pvt_commit_advances_the_plan_and_invalidates_preflight() {
    let mut app = RSpiceApp::test_instance();
    let (plan_id, source_revision) = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| (plan.id(), plan.revision()))
        .expect("default plan");
    let (topology_root, topology_revision, topology_closure) =
        app.state.configured_topology_revision();
    app.state.runs.preflight.report = Some(crate::state::PreflightReport {
        project_revision: app.state.model.workspace.content.project.revision().get(),
        topology_root,
        topology_revision,
        topology_closure,
        simulation_plan_id: Some(plan_id),
        simulation_plan_revision: Some(source_revision),
        blockers: Vec::new(),
        advisories: Vec::new(),
        prepared: None,
    });

    assert!(
        commit_reference_pvt(&mut app, crate::product::ProcessCorner::FF, -40.0,)
            .expect("PVT selection commits")
    );

    assert_eq!(
        app.state
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("plan remains available")
            .revision(),
        source_revision.next().expect("revision advances")
    );
    assert!(app.state.runs.preflight.report.is_none());
}
