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
    let refusal = app.state.prompt_text().expect("the refusal is stated");
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

    assert_eq!(
        app.state.prompt_text(),
        Some("The selected diagnostic is no longer available."),
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

/// A loop is every source in it. Its record names each source as the drawing
/// does and says what it is to the loop, and following each one from another
/// view opens the loop's view and selects that source there.
#[test]
fn every_source_in_a_loop_is_named_and_followed_into_its_view() {
    use crate::schematic::view::violations::tests::draw_source_loop;

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    let (top, aux, sources) = draw_source_loop(&mut app.state);
    let code = rspice_design::drc::DrcViolationType::SourceToSource.rule_code();
    let places = app
        .state
        .diagnostics
        .batches()
        .flat_map(|batch| batch.records.iter())
        .find(|record| record.canonical.code.as_ref() == code)
        .map(|record| record.related_places.clone())
        .expect("the loop is published");

    let mut labels: Vec<&str> = places.iter().map(|place| place.label.as_str()).collect();
    labels.sort_unstable();
    let mut expected: Vec<String> = sources
        .iter()
        .map(|source| format!("{} \u{00b7} source in the loop", source.name))
        .collect();
    expected.sort_unstable();
    assert_eq!(
        labels, expected,
        "each source, by the name it is drawn with"
    );

    for source in &sources {
        let place = places
            .iter()
            .find(|place| place.label.starts_with(&format!("{} ", source.name)))
            .expect("every source is a place");
        assert_eq!(place.cell_view, top, "drawn in the loop's own view");
        let route = place
            .route
            .clone()
            .expect("a drawn source has somewhere to go");

        app.state.open_workspace_view(aux.clone());
        app.state.diagnostics.request_route(route);
        apply_pending(&context(), &mut app);

        assert_eq!(
            app.state.workspace.content.active_schematic_reference(),
            top,
            "{}: the loop's view opens",
            source.name
        );
        assert_eq!(
            app.state
                .schematic
                .session
                .editor
                .selection
                .components
                .iter()
                .copied()
                .collect::<Vec<_>>(),
            [source.id],
            "{}: that source alone is selected",
            source.name
        );
    }
}

/// Two instances carrying one name on two sheets of one cellview: the finding
/// is stated at one and names the other. Each is drawn on its own sheet, and
/// following either brings that sheet forward before selecting it, which is
/// what neither the finding's own route nor a related place did while either
/// stopped at the sheet in view.
#[test]
fn a_duplicate_name_on_another_sheet_is_named_and_followed_onto_its_sheet() {
    use crate::state::{ComponentType, Point, SheetDefinition, SheetPortPolicy, SheetTemplate};
    use rspice_design::drc::DrcViolationType;

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    let subject = app.state.workspace.content.active_schematic_reference();
    let ids = [Point::new(0, 0), Point::new(400, 0)].map(|at| {
        app.state
            .schematic
            .add_component(ComponentType::Resistor, at)
    });
    for component in app
        .state
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .filter(|component| ids.contains(&component.id))
    {
        component.name = "R1".to_owned();
    }
    app.state.sync_active_schematic_to_workspace();
    let key = subject.key();
    let first = app
        .state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Page 1", [ids[0]])
        .expect("a fresh cell view accepts its first governed sheet");
    let catalog = app
        .state
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("the sheet catalog was just created");
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: "Page 2".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .expect("a second governed sheet inserts after the first");
    catalog
        .assign_objects(catalog.revision(), second, [ids[1]])
        .expect("the second instance belongs to the second page");
    catalog
        .set_active(first)
        .expect("the first page is in view");

    app.state
        .check_active_cellview()
        .expect("the checks run on the drawing");
    let code = DrcViolationType::DuplicateName.rule_code();
    let record = app
        .state
        .diagnostics
        .batches()
        .flat_map(|batch| batch.records.iter())
        .find(|record| record.canonical.code.as_ref() == code)
        .cloned()
        .expect("the duplicated name is published");
    let [place] = record.related_places.as_slice() else {
        panic!(
            "the finding names the other instance: {:?}",
            record.related_places
        );
    };
    assert_eq!(place.label, "R1 \u{00b7} also named R1");
    let (stated, other) = match place.location {
        rspice_design::drc::DrcLocation::Component { id, .. } if id == ids[0] => (ids[1], ids[0]),
        rspice_design::drc::DrcLocation::Component { id, .. } if id == ids[1] => (ids[0], ids[1]),
        ref location => panic!("the other instance, by where it is: {location:?}"),
    };
    let sheet_of = |id: u64| if id == ids[0] { first } else { second };

    for (route, id) in [
        (place.route.clone(), other),
        (record.route.clone(), stated),
        (place.route.clone(), other),
    ] {
        let route = route.expect("each instance has somewhere to go, on whichever sheet");
        app.state.diagnostics.request_route(route);
        apply_pending(&context(), &mut app);

        assert_eq!(
            crate::workbench::app::sheets::active_sheet_id(&app.state),
            Some(sheet_of(id)),
            "the sheet that draws #{id} is brought forward"
        );
        assert_eq!(
            app.state
                .schematic
                .session
                .editor
                .selection
                .components
                .iter()
                .copied()
                .collect::<Vec<_>>(),
            [id],
            "and #{id} is selected on it"
        );
    }
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
