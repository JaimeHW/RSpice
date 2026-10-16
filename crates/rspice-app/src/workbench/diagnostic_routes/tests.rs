//! Where following a finding lands.
//!
//! The destinations a dispatch finding names are covered beside the report
//! that produces them, in `preflight::tests`. What is here is the hop itself:
//! a request recorded by a page is served once, by the frame boundary, and
//! the two routes whose destination has to be resolved rather than named — a
//! netlist record, and a source document in whichever editor owns its bundle.

use super::*;
use crate::workbench::app_state::diagnostic_store::DiagnosticRoute;

fn context() -> Context {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

/// A request is served once. Serving it twice would re-open a destination the
/// reader has since navigated away from, on every frame after the click.
#[test]
fn a_requested_route_is_applied_once() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    app.state.diagnostics.request_route(DiagnosticRoute::Repair(
        PreflightRemediation::SimulationPlan { analysis: None },
    ));

    apply_pending(&context(), &mut app);
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Simulate
    );

    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    apply_pending(&context(), &mut app);
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Results,
        "a served request must not be served again",
    );
}

/// A source route resolves its own destination from the bundle's owner.
///
/// A bundle the project no longer holds says so rather than selecting a
/// document in whatever editor happened to be open — which is what carrying
/// only the language would have done, since the Verilog-A bundle in front of
/// the reader is whichever cell view is active.
#[test]
fn a_source_route_to_a_departed_bundle_states_its_refusal() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    let before = app.state.workbench.workspace;
    app.state
        .diagnostics
        .request_route(DiagnosticRoute::CodeDocument {
            bundle: crate::state::ProjectSourceId::new(),
            logical_path: std::sync::Arc::from("amp.va"),
            line: Some(3),
        });

    apply_pending(&context(), &mut app);

    assert_eq!(
        app.state.workbench.workspace, before,
        "a route that cannot resolve must not move the reader"
    );
    let refusal = app
        .state
        .log_buffer
        .entries()
        .last()
        .expect("the refusal is stated")
        .message
        .clone();
    assert!(
        refusal.contains("amp.va") && refusal.contains("no longer part of this project"),
        "{refusal}"
    );
}

/// A source route opens the editor that owns the bundle and selects the
/// document in it. The automation bundle is the project's, so the route lands
/// on the Netlist workspace's Automation page with the reported file chosen.
#[test]
fn a_source_route_opens_its_document_in_the_editor_that_owns_it() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app.state.workbench.activate(Workspace::Results);
    let owner = crate::state::ProjectSourceOwner::code_workspace(
        crate::state::ProjectSourceLanguage::RSpiceAutomation,
    );
    let bundle = app
        .state
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .expect("a project holds its automation bundle");
    let bundle_id = bundle.id();
    let logical_path = bundle
        .paths_for_role(crate::state::ProjectSourceRole::AutomationPermissionManifest)
        .next()
        .expect("the automation bundle has a permission manifest")
        .to_owned();
    app.state
        .diagnostics
        .request_route(DiagnosticRoute::CodeDocument {
            bundle: bundle_id,
            logical_path: std::sync::Arc::from(logical_path.as_str()),
            line: Some(2),
        });

    apply_pending(&context(), &mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
    assert_eq!(
        app.state.ui.code_workspace.page,
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation
    );
    assert_eq!(
        app.state
            .ui
            .code_workspace
            .automation
            .selected_file
            .as_deref(),
        Some(logical_path.as_str())
    );
}

/// A netlist record is opened by its canonical identity, and an identity the
/// collection no longer holds is refused in the editor's own words.
#[test]
fn a_netlist_route_to_a_retired_record_states_its_refusal() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app.state
        .diagnostics
        .request_route(DiagnosticRoute::NetlistDiagnostic(uuid::Uuid::nil()));

    apply_pending(&context(), &mut app);

    // The editor's own refusal, not any warning that happened to be logged:
    // the session logs warnings of its own, and a test that accepted one of
    // those would pass with the route deleted.
    assert!(
        app.state.log_buffer.entries().any(|entry| {
            entry.severity == crate::diagnostics::LogSeverity::Warning
                && entry.message == "The selected diagnostic is no longer available."
        }),
        "an unresolvable record must say so rather than opening nothing silently"
    );
}

/// A rule finding is about an object in one drawing. Checking `top`, opening
/// `aux` and following a `top` finding used to centre and select in `aux` —
/// on whatever object there happened to carry the same id.
#[test]
fn following_a_rule_finding_opens_its_cell_view_before_selecting() {
    use crate::state::{Cell, CellViewRef, ComponentType, Library, Point, View, ViewType};
    use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    let mut library = Library::new("work");
    for name in ["top", "aux"] {
        let mut cell = Cell::new(name);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(cell);
    }
    app.state.library_manager.add_library(library);
    let top = CellViewRef::new("work", "top", "schematic");
    let aux = CellViewRef::new("work", "aux", "schematic");

    app.state.open_workspace_view(top.clone());
    let resistor = app
        .state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(40, 40));
    app.state.sync_active_schematic_to_workspace();
    let mut result = DrcResult::new();
    result.add_violation(DrcViolation::new(
        1,
        DrcViolationType::MissingParameter,
        "R1 has no resistance",
        DrcLocation::Component {
            id: resistor,
            name: "R1".to_owned(),
        },
    ));
    result.completed = true;
    app.state
        .publish_active_design_check_result(result)
        .expect("the checks publish against top");
    let route = app
        .state
        .diagnostics
        .batches()
        .flat_map(|batch| batch.records.iter())
        .find_map(|record| record.route.clone())
        .expect("a finding about a drawn object has somewhere to go");

    // The other drawing has an object of its own, which ids are allocated per
    // drawing and so may well share the resistor's.
    app.state.open_workspace_view(aux.clone());
    app.state
        .schematic
        .add_component(ComponentType::Capacitor, Point::new(-200, -200));
    assert_eq!(
        app.state.workspace.content.active_schematic_reference(),
        aux
    );

    app.state.diagnostics.request_route(route);
    apply_pending(&context(), &mut app);

    assert_eq!(
        app.state.workspace.content.active_schematic_reference(),
        top,
        "the finding's own view opens"
    );
    assert!(
        app.state
            .schematic
            .session
            .editor
            .selection
            .components
            .contains(&resistor),
        "the object is selected in the view the finding is about"
    );
    assert!(
        app.state
            .schematic
            .document()
            .components
            .iter()
            .any(|component| component.id == resistor && component.kind == ComponentType::Resistor),
        "and that view is the one holding the resistor"
    );
}

/// A rule finding about a view that is not open still says where it is: the
/// route is resolved in that view's own drawing rather than left out.
#[test]
fn a_rule_finding_about_a_view_that_is_not_open_keeps_its_route() {
    use crate::state::{CellViewRef, ComponentType, Point, SchematicState};
    use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    let root = app.state.workspace.content.simulation_root_reference();
    let resistor = app
        .state
        .workspace
        .schematic_editor_mut(&root.key())
        .expect("default configured root buffer")
        .editor
        .add_component(ComponentType::Resistor, Point::new(40, 40));
    app.state.workspace.content.active_view =
        CellViewRef::new("user", "unrelated_editor", "schematic");
    app.state.schematic = SchematicState::default();

    let mut result = DrcResult::new();
    result.add_violation(DrcViolation::new(
        1,
        DrcViolationType::MissingParameter,
        "R1 has no resistance",
        DrcLocation::Component {
            id: resistor,
            name: "R1".to_owned(),
        },
    ));
    result.completed = true;
    app.state
        .publish_configured_root_design_check_result(result)
        .expect("the root's findings publish");

    let route = app
        .state
        .diagnostics
        .batches()
        .flat_map(|batch| batch.records.iter())
        .find_map(|record| record.route.clone());
    assert!(
        matches!(
            route,
            Some(DiagnosticRoute::Anchor(crate::diagnostics::LogAnchor::Schematic {
                ref cell_view,
                component: Some(component),
                ..
            })) if *cell_view == root && component == resistor
        ),
        "{route:?}"
    );
}
