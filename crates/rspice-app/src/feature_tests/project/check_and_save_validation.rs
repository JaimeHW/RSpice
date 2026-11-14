//! The Check and save report decided over the whole state: what blocks a
//! save, what only advises, what each finding names, and when the report
//! goes stale.

use crate::workbench::app::project_transactions::WholeStateValidation;
use rspice_design::drc::{DrcLocation, DrcSeverity};

use crate::state::{ComponentType, Point};
use crate::workbench::app::dialogs::check_and_save_validation::{
    CheckAndSaveFindingLevel, CheckAndSaveValidationReport,
};
use crate::workbench::app::project_transactions::run_design_checks_in;
use crate::workbench::app_state::AppState;
use crate::workbench::lifecycle::project_lifecycle::SaveScope;

#[test]
fn hierarchical_instances_validate_without_an_open_property_dialog() {
    let state = crate::workbench::examples::hierarchy_reference::build().state;
    assert!(
        state
            .design
            .property_registry
            .get(ComponentType::CellInstance)
            .is_none()
    );
    let report = CheckAndSaveValidationReport::validate(&state).unwrap();
    let parameters = report
        .blockers()
        .iter()
        .filter(|finding| finding.source == "parameters")
        .collect::<Vec<_>>();
    assert!(parameters.is_empty(), "{parameters:?}");
}

#[test]
fn omitted_instances_do_not_reappear_in_validated_save_hierarchy_findings() {
    let mut state = crate::workbench::examples::hierarchy_reference::build().state;
    let baseline = CheckAndSaveValidationReport::validate(&state).unwrap();
    assert!(
        baseline
            .blockers()
            .iter()
            .all(|finding| finding.source != "hierarchy")
    );
    crate::workbench::examples::hierarchy_reference::omit_missing_instance(&mut state);
    let authored = serde_json::to_value(&state.model.schematic).unwrap();
    let report = CheckAndSaveValidationReport::validate(&state).unwrap();
    assert!(
        report
            .blockers()
            .iter()
            .all(|finding| finding.source != "hierarchy"),
        "{:?}",
        report.blockers()
    );
    assert_eq!(
        serde_json::to_value(&state.model.schematic).unwrap(),
        authored
    );
}

#[test]
fn validation_is_deterministic_and_stales_after_design_change() {
    let mut state = AppState::default();
    run_design_checks_in(&mut state);
    let first = CheckAndSaveValidationReport::collect_in(&state).expect("first report");
    // As the commit re-derives it: the checks still describe the design.
    run_design_checks_in(&mut state);
    let second = CheckAndSaveValidationReport::collect_in(&state).expect("second report");
    assert_eq!(first.receipt_digest(), second.receipt_digest());

    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(10, 10));
    let unchecked = CheckAndSaveValidationReport::collect_in(&state).expect("unchecked report");
    assert!(
        unchecked
            .blockers()
            .iter()
            .any(|finding| finding.source == "checks"
                && finding.message.contains("configured simulation root")),
        "a design the checks no longer describe is not saved on their word"
    );
    run_design_checks_in(&mut state);
    let changed = CheckAndSaveValidationReport::collect_in(&state).expect("changed report");
    assert_ne!(first.receipt_digest(), changed.receipt_digest());
}

/// The report reads the checks the page lists, under the waivers the page
/// states them under: a waived error is listed and blocks nothing.
#[test]
fn a_save_decides_from_the_checks_the_page_lists_and_waivers_count() {
    use crate::workbench::app_state::diagnostic_store::DiagnosticInputs;
    use crate::workbench::app_state::waivers::WaiverAction;

    let mut state = AppState::default();
    state.open_project_session();
    // An open pin is an error under this deck, and one a waiver may cover.
    state
        .model
        .workspace
        .content
        .design_rules
        .set_severity(
            rspice_design::drc::DrcViolationType::UnconnectedPin,
            DrcSeverity::Error,
        )
        .expect("an open pin may be promoted");
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(10, 10));
    run_design_checks_in(&mut state);
    let report = CheckAndSaveValidationReport::collect_in(&state).expect("report");
    let root = state.model.workspace.content.simulation_root_reference();
    let published = state
        .model
        .diagnostics
        .batch(&DiagnosticInputs::DesignCheck { subject: root })
        .expect("the checks the report read are on the page")
        .records
        .clone();
    let rule_blockers = |report: &CheckAndSaveValidationReport| {
        report
            .blockers()
            .iter()
            .filter(|finding| {
                finding.source == "configured-root checks" || finding.source == "design checks"
            })
            .count()
    };
    let errors = published
        .iter()
        .filter(|record| record.level.is_error())
        .collect::<Vec<_>>();
    assert_eq!(
        rule_blockers(&report),
        errors.len(),
        "{:?}",
        report.blockers()
    );

    let targets = errors
        .iter()
        .filter_map(|record| match &record.waiver_action {
            Some(WaiverAction::Waive(target)) => Some(target.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(!targets.is_empty(), "{errors:?}");
    for target in &targets {
        state
            .waive_finding(target, "accepted for this design")
            .expect("the waiver is recorded");
    }
    run_design_checks_in(&mut state);
    let waived = CheckAndSaveValidationReport::collect_in(&state).expect("waived report");
    assert_eq!(rule_blockers(&waived), errors.len() - targets.len());
    assert_eq!(
        waived
            .advisories()
            .iter()
            .filter(|finding| finding.message.ends_with("(waived)"))
            .count(),
        targets.len(),
        "{:?}",
        waived.advisories()
    );
}

#[test]
fn missing_ground_blocks_validated_save() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(10, 10));
    let report = CheckAndSaveValidationReport::validate(&state).expect("report");
    assert!(!report.can_save());
    assert!(
        report
            .blockers()
            .iter()
            .any(|finding| { finding.message.to_ascii_lowercase().contains("ground") })
    );
}

/// A finding about the active document names the object it is about.
#[test]
fn a_located_finding_names_the_object_it_is_about() {
    let mut state = AppState::default();
    let resistor = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(40, 40));
    let placed = state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == resistor)
        .expect("the placed resistor is retained");
    placed.name = "Q1".to_owned();

    let report = CheckAndSaveValidationReport::validate(&state).expect("report");
    // One refused designator is one finding. The netlister writes this
    // resistor as `RQ1` and the run starts, so it advises rather than
    // blocks.
    let naming = report
        .blockers()
        .iter()
        .chain(report.advisories())
        .filter(|finding| finding.message.contains("'Q1'"))
        .collect::<Vec<_>>();
    assert_eq!(naming.len(), 1, "{naming:?}");
    let located = naming[0];
    assert_eq!(located.level, CheckAndSaveFindingLevel::Advisory);
    assert!(located.message.contains("RQ1"), "{}", located.message);
    assert!(
        matches!(
            &located.location,
            Some(DrcLocation::Component { id, .. }) if *id == resistor
        ),
        "a designator refused on the active document names its instance: {located:?}"
    );
}

/// A first save checks every document, and every cellview exactly once.
/// A leaf with one open resistor, placed twice by a cell the root places
/// twice, used to be stated by the root at four occurrences, by the middle
/// cell at two and by the leaf itself — fourteen rows for two open legs.
#[test]
fn a_first_save_states_each_cellviews_findings_once() {
    let mut state = AppState::default();
    let mut work = crate::state::Library::new("work");
    for name in ["leaf", "mid"] {
        let mut cell = crate::state::Cell::new(name);
        cell.add_view(crate::state::View::new(
            "schematic",
            crate::state::ViewType::Schematic,
        ));
        work.add_cell(cell);
    }
    state.model.library_manager.add_library(work);
    let placed = |cell: &str| {
        let mut binding = crate::state::LibraryCellInstance::new("work", cell, "schematic");
        binding.bind_interface(&[]);
        binding
    };

    let mut leaf = crate::state::SchematicState::default();
    leaf.add_component(ComponentType::Resistor, Point::new(30, 0));
    state
        .model
        .workspace
        .insert_schematic_editor("work/leaf/schematic".to_owned(), leaf);
    let mut mid = crate::state::SchematicState::default();
    mid.add_library_cell_component(Point::new(100, 0), placed("leaf"));
    mid.add_library_cell_component(Point::new(300, 0), placed("leaf"));
    state
        .model
        .workspace
        .insert_schematic_editor("work/mid/schematic".to_owned(), mid);
    state
        .model
        .schematic
        .add_library_cell_component(Point::new(100, 0), placed("mid"));
    state
        .model
        .schematic
        .add_library_cell_component(Point::new(300, 0), placed("mid"));
    state.sync_active_schematic_to_workspace();

    let report = CheckAndSaveValidationReport::validate(&state).expect("report");
    assert_eq!(report.effective_scope, SaveScope::AllDocuments);

    let open_legs = report
        .blockers()
        .iter()
        .chain(report.advisories())
        .filter(|finding| finding.message.contains("is not connected"))
        .collect::<Vec<_>>();
    assert_eq!(open_legs.len(), 2, "{open_legs:?}");
    assert!(
        open_legs
            .iter()
            .all(|finding| finding.message.starts_with("work/leaf/schematic: ")),
        "each is stated by the leaf that holds it: {open_legs:?}"
    );
    // No cell is asked for a ground of its own: it takes its reference
    // from whatever places it.
    assert!(
        report.blockers().iter().all(|finding| !finding
            .message
            .starts_with("work/leaf/schematic: Circuit has no ground")
            && !finding
                .message
                .starts_with("work/mid/schematic: Circuit has no ground")),
        "{:?}",
        report.blockers()
    );
}

/// A port named `name` declaring `direction`.
fn add_port(schematic: &mut crate::state::SchematicState, name: &str, direction: &str) {
    let port = schematic.add_component(ComponentType::Port, Point::new(20, 0));
    let placed = schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port)
        .expect("the placed port is retained");
    placed.value = name.to_owned();
    placed.params = format!("dir={direction}");
}

/// Place `cell`'s schematic in `schematic` as `name`, presenting `ports`.
fn place(
    schematic: &mut crate::state::SchematicState,
    cell: &str,
    name: &str,
    ports: &[crate::state::PortSpec],
    at: Point,
) {
    let mut binding = crate::state::LibraryCellInstance::new("work", cell, "schematic");
    binding.bind_interface(ports);
    let instance = schematic.add_library_cell_component(at, binding);
    schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == instance)
        .expect("the placed instance is retained")
        .name = name.to_owned();
}

/// A library `work` holding `cells`, each with the listed schematic views.
fn add_cells(state: &mut AppState, cells: &[(&str, &[&str])]) {
    let mut work = crate::state::Library::new("work");
    for &(name, views) in cells {
        let mut cell = crate::state::Cell::new(name);
        for &view in views {
            cell.add_view(crate::state::View::new(
                view,
                crate::state::ViewType::Schematic,
            ));
        }
        work.add_cell(cell);
    }
    state.model.library_manager.add_library(work);
}

/// The configuration binds `mid`'s instance of `leaf` to a view whose pin
/// is an input, where the placed view declares it bidirectional. `mid`'s
/// own check, reading the placed binding, finds an open pin; the root's
/// walk finds an undriven input. They are two findings, and the second is
/// the one the run refuses, so the save must state it too.
#[test]
fn a_finding_only_the_configuration_makes_is_stated_beside_the_cells_own() {
    let mut state = AppState::default();
    add_cells(
        &mut state,
        &[
            ("leaf", &["schematic", "schematic_v2"]),
            ("mid", &["schematic"]),
        ],
    );
    for (view, direction) in [("schematic", "inout"), ("schematic_v2", "in")] {
        let mut leaf = crate::state::SchematicState::default();
        add_port(&mut leaf, "a", direction);
        state
            .model
            .workspace
            .insert_schematic_editor(format!("work/leaf/{view}"), leaf);
    }
    let mut mid = crate::state::SchematicState::default();
    place(
        &mut mid,
        "leaf",
        "XA",
        &[crate::state::PortSpec {
            name: "a".to_owned(),
            direction: crate::state::PortDirection::InOut,
        }],
        Point::new(100, 0),
    );
    state
        .model
        .workspace
        .insert_schematic_editor("work/mid/schematic".to_owned(), mid);
    place(
        &mut state.model.schematic,
        "mid",
        "XM",
        &[],
        Point::new(100, 0),
    );
    state.sync_active_schematic_to_workspace();
    let root = state.model.workspace.content.active_view.clone();
    state
        .model
        .workspace
        .content
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Second leaf".to_owned(),
            root,
            dut_path: "/XM".to_owned(),
            executable_view_policy: vec!["schematic_v2".to_owned(), "schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            cell_bindings: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "check and save test".to_owned(),
        })
        .expect("the fixture configuration is well formed");

    let report = CheckAndSaveValidationReport::validate(&state).expect("report");

    assert!(
        report
            .advisories()
            .iter()
            .any(|finding| finding.message == "work/mid/schematic: Pin XA.a is not connected"),
        "{:?}",
        report.advisories()
    );
    assert!(
        report
            .blockers()
            .iter()
            .any(|finding| finding.message == "Configured root, /XM: Input pin XA.a has no driver"),
        "{:?}",
        report.blockers()
    );
}

/// A document the configured root does not place is checked through
/// what it places, which no other check reaches; and where it is a design
/// of its own rather than a cell, it is asked for a ground.
#[test]
fn an_unplaced_document_is_checked_through_its_hierarchy() {
    let mut state = AppState::default();
    add_cells(
        &mut state,
        &[
            ("supply", &["schematic"]),
            ("bench", &["schematic"]),
            ("stray", &["schematic"]),
            ("buffer", &["schematic"]),
        ],
    );
    // One source holds vdd! against ground. Placed twice, the two copies
    // are a loop, which neither copy's cell can see.
    let mut supply = crate::state::SchematicState::default();
    let source = supply.add_component(ComponentType::VoltageSource, Point::new(30, 0));
    let terminals = supply
        .document()
        .components
        .iter()
        .find(|component| component.id == source)
        .map(|component| component.terminal_positions())
        .expect("the source is placed");
    for (id, (pin, node)) in (900..).zip([("+", "vdd!"), ("-", "0")]) {
        let point = terminals
            .iter()
            .find_map(|(name, point)| (*name == pin).then_some(*point))
            .expect("a voltage source has both terminals");
        supply
            .document_mut_for_test()
            .net_labels
            .push(crate::state::NetLabel::new(id, point, node));
    }
    state
        .model
        .workspace
        .insert_schematic_editor("work/supply/schematic".to_owned(), supply);
    let mut bench = crate::state::SchematicState::default();
    place(&mut bench, "supply", "X1", &[], Point::new(100, 0));
    place(&mut bench, "supply", "X2", &[], Point::new(300, 0));
    state
        .model
        .workspace
        .insert_schematic_editor("work/bench/schematic".to_owned(), bench);

    // A resistor on its own, with no ground anywhere: a design that cannot
    // solve. The same resistor behind a port is a cell, which takes its
    // reference from whatever places it.
    let mut stray = crate::state::SchematicState::default();
    stray.add_component(ComponentType::Resistor, Point::new(30, 0));
    let mut buffer = stray.clone();
    add_port(&mut buffer, "a", "inout");
    state
        .model
        .workspace
        .insert_schematic_editor("work/stray/schematic".to_owned(), stray);
    state
        .model
        .workspace
        .insert_schematic_editor("work/buffer/schematic".to_owned(), buffer);
    state.sync_active_schematic_to_workspace();

    let report = CheckAndSaveValidationReport::validate(&state).expect("report");
    assert_eq!(report.effective_scope, SaveScope::AllDocuments);

    let loops = report
        .blockers()
        .iter()
        .filter(|finding| finding.message.contains("form a closed loop"))
        .collect::<Vec<_>>();
    assert_eq!(loops.len(), 1, "{:?}", report.blockers());
    assert!(
        loops[0]
            .message
            .starts_with("work/bench/schematic, /X1: 2 voltage sources"),
        "{}",
        loops[0].message
    );

    let unground = |key: &str| {
        report.blockers().iter().any(|finding| {
            finding
                .message
                .starts_with(&format!("{key}: Circuit has no ground"))
        })
    };
    assert!(unground("work/stray/schematic"), "{:?}", report.blockers());
    assert!(
        !unground("work/buffer/schematic"),
        "{:?}",
        report.blockers()
    );
    assert!(!unground("work/bench/schematic"), "{:?}", report.blockers());
}

#[test]
fn first_save_discloses_all_document_scope() {
    let state = AppState::default();
    let report = CheckAndSaveValidationReport::validate(&state).expect("report");
    assert_eq!(report.effective_scope, SaveScope::AllDocuments);
    assert!(
        report
            .document_scope_label()
            .starts_with("All project documents")
    );
}

/// The netlist blocker with a one-click remedy has to be findable as such,
/// or the dialog can only repeat the generator's prose at the author.
#[test]
fn a_stale_instance_interface_is_a_blocker_that_names_its_repair() {
    let mut state = AppState::default();
    let mut work = crate::state::Library::new("work");
    let mut div = crate::state::Cell::new("div");
    div.add_view(crate::state::View::new(
        "schematic",
        crate::state::ViewType::Schematic,
    ));
    work.add_cell(div);
    state.model.library_manager.add_library(work);

    let mut master = crate::state::SchematicState::default();
    let port = master.add_component(ComponentType::Port, Point::new(20, 0));
    master
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port)
        .expect("the placed port is retained")
        .value = "a".to_owned();
    state
        .model
        .workspace
        .insert_schematic_editor("work/div/schematic".to_owned(), master);

    let mut binding = crate::state::LibraryCellInstance::new("work", "div", "schematic");
    binding.bind_interface(&[crate::state::PortSpec {
        name: "old".to_owned(),
        direction: crate::state::PortDirection::InOut,
    }]);
    let instance = state
        .model
        .schematic
        .add_library_cell_component(Point::new(100, 0), binding);
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == instance)
        .expect("the placed instance is retained")
        .name = "X1".to_owned();

    let report = CheckAndSaveValidationReport::validate(&state).expect("report");

    assert_eq!(report.stale_interface_instances().len(), 1);
    assert_eq!(report.stale_interface_instances()[0], "X1");
    assert!(
        report.blockers().iter().any(|finding| {
            finding.source == "stale interface" && finding.message.contains("X1")
        }),
        "the repairable blocker states the instance: {:?}",
        report.blockers()
    );
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
    state.model.library_manager.add_library(work);

    let mut master = crate::state::SchematicState::default();
    master.add_component(ComponentType::Resistor, Point::new(30, 0));
    state
        .model
        .workspace
        .insert_schematic_editor("work/amp/schematic".to_owned(), master);

    let binding = crate::state::LibraryCellInstance::new("work", "amp", "schematic");
    let instance = state
        .model
        .schematic
        .add_library_cell_component(Point::new(100, 0), binding);
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == instance)
        .expect("the placed instance is retained")
        .name = "X1".to_owned();
    state.sync_active_schematic_to_workspace();

    let root = state.model.workspace.content.active_view.clone();
    state
        .model
        .workspace
        .content
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
            cell_bindings: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "projection consumer test".to_owned(),
        })
        .expect("the fixture configuration is well formed");
    state
}

#[test]
fn a_configuration_honoured_differently_than_it_reads_is_an_advisory() {
    let state = state_with_a_stop_view_warning();

    let report = CheckAndSaveValidationReport::validate(&state).expect("report");

    assert!(
        report.advisories().iter().any(|finding| {
            finding.source == "hierarchy" && finding.message.contains("stop view")
        }),
        "the reviewed save records the configuration warning: {:?}",
        report.advisories()
    );
}

#[test]
fn an_unresolved_configuration_is_stated_rather_than_checked_from_editor_buffers() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(10, 10));
    state.sync_active_schematic_to_workspace();
    let root = state.model.workspace.content.active_view.clone();
    state
        .model
        .workspace
        .content
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Unresolvable DUT".to_owned(),
            root,
            dut_path: "/XABSENT".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            cell_bindings: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "projection consumer test".to_owned(),
        })
        .expect("the fixture configuration is well formed");

    let report = CheckAndSaveValidationReport::validate(&state).expect("report");

    assert!(
        report
            .blockers()
            .iter()
            .any(|finding| finding.message.contains("XABSENT")),
        "the report states the projection's own reason: {:?}",
        report.blockers()
    );
    assert!(
        !report
            .blockers()
            .iter()
            .chain(report.advisories())
            .any(|finding| finding.source == "netlist" || finding.source == "design checks"),
        "no check may be derived from the editor buffers once the \
         configuration is refused: {:?}",
        report.blockers()
    );
}

/// Check and Save checks the design, then the open cellview as the
/// design places it. The second pass computes no loop the copies of the
/// cellview close through a global, so the loop Check Design found is
/// still listed when the report blocks on it.
#[test]
fn the_loop_check_and_save_blocks_on_is_the_one_the_page_lists() {
    use crate::workbench::examples::hierarchy_reference;
    use crate::workbench::examples::hierarchy_reference::check_scope_tests::{
        draw_supply_source, source_loop_errors,
    };

    let reference = hierarchy_reference::build();
    let mut state = reference.state;
    state.open_project_session();
    state.open_workspace_view(reference.amp_schematic);
    draw_supply_source(&mut state);

    run_design_checks_in(&mut state);
    let report = CheckAndSaveValidationReport::collect_in(&state).expect("report");
    assert!(
        report
            .blockers()
            .iter()
            .any(|finding| finding.message.contains("form a closed loop")),
        "{:?}",
        report.blockers()
    );
    assert!(
        source_loop_errors(&state) > 0,
        "the page lists the loop the report blocks on"
    );
}
