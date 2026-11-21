//! The Configuration page as a reader meets it: what it states on Default
//! and on a configured hierarchy, and how often it resolves the design.

use egui::Color32;

use crate::workbench::RSpiceApp;
use crate::workbench::examples::hierarchy_reference;
use crate::workbench::features;
use rspice_app_project::workbench::project::configuration_state::ConfigurationSelection;
use rspice_app_project::workbench::surfaces::project::configuration::configuration;

fn reference_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state = hierarchy_reference::build().state;
    app
}

/// `levels` cells under the project's top, each placing the next thirty-two
/// times and the last a resistor: four levels expand past a million
/// instances.
pub(super) fn nested_app(levels: usize) -> RSpiceApp {
    use rspice_app_state::state::{
        Cell, CellViewRef, ComponentType, LibraryCellInstance, PendingPortPlacement, Point,
        PortDirectionType, PortDiscipline, SchematicState, View, ViewType,
    };
    fn port(schematic: &mut SchematicState, x: i32, name: &str, direction: PortDirectionType) {
        let pending = PendingPortPlacement::new(
            name,
            direction,
            PortDiscipline::Electrical,
            schematic.topology_version(),
            schematic.next_interface_order(),
        );
        schematic
            .place_pending_port(Point::new(x, 0), pending)
            .expect("the port places");
    }

    let mut state = crate::workbench::app_state::AppState::default();
    let project = &state.model.workspace.content.project;
    let (library, top) = (project.root_library.clone(), project.top_cell.clone());
    let cells = (1..=levels)
        .map(|level| format!("l{level}"))
        .collect::<Vec<_>>();
    {
        let mut user = state
            .model
            .library_manager
            .edit_library(&library)
            .expect("the project library");
        for name in &cells {
            let mut cell = Cell::new(name);
            cell.add_view(View::new("schematic", ViewType::Schematic));
            user.add_cell(cell);
        }
    }
    let mut below: Option<&str> = None;
    for name in cells.iter().map(String::as_str).rev().chain([top.as_str()]) {
        state.open_workspace_view(CellViewRef::new(&library, name, "schematic"));
        let ports = below.map(|child| {
            state
                .model
                .workspace
                .content
                .schematic_buffers
                .get(&CellViewRef::new(&library, child, "schematic").key())
                .expect("the cell below is saved")
                .document()
                .interface_ports()
        });
        let schematic = &mut state.model.schematic;
        if name != top {
            port(schematic, 10, "a", PortDirectionType::InputAnalog);
            port(schematic, 130, "b", PortDirectionType::OutputAnalog);
        }
        match (below, ports) {
            (Some(child), Some(ports)) => {
                for at in 0..32 {
                    let mut binding = LibraryCellInstance::new(&library, child, "schematic");
                    binding.bind_interface(&ports);
                    schematic.add_library_cell_component(Point::new(300 + 200 * at, 200), binding);
                }
            }
            _ => {
                schematic
                    .add_wire(vec![Point::new(0, 0), Point::new(40, 0)])
                    .expect("a wire commits");
                schematic.add_component(ComponentType::Resistor, Point::new(60, 0));
                schematic
                    .add_wire(vec![Point::new(80, 0), Point::new(120, 0)])
                    .expect("a wire commits");
            }
        }
        state.sync_active_schematic_to_workspace();
        below = Some(name);
    }
    let mut app = RSpiceApp::test_instance();
    app.state = state;
    app
}

/// One frame of the page: every accessible name and value, and every string
/// it painted with the colour it was painted in.
struct Frame {
    announced: Vec<String>,
    painted: Vec<(String, Color32)>,
}

fn frame(app: &mut RSpiceApp, width: f32) -> Frame {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width, 900.0),
            )),
            ..Default::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| configuration(ui, &mut features::cx(app)));
        },
    );
    let announced = output
        .platform_output
        .accesskit_update
        .expect("an accessibility update")
        .nodes
        .iter()
        .flat_map(|(_, node)| {
            [node.label(), node.value(), node.description()]
                .into_iter()
                .flatten()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .collect();
    fn collect(shape: &egui::epaint::Shape, out: &mut Vec<(String, Color32)>) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push((
                text.galley.job.text.clone(),
                text.galley
                    .job
                    .sections
                    .first()
                    .map_or(text.fallback_color, |section| section.format.color),
            )),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, out);
                }
            }
            _ => {}
        }
    }
    let mut painted = Vec::new();
    for shape in &output.shapes {
        collect(&shape.shape, &mut painted);
    }
    Frame { announced, painted }
}

impl Frame {
    fn says(&self, text: &str) -> bool {
        self.announced.iter().any(|announced| announced == text)
            || self.painted.iter().any(|(painted, _)| painted == text)
    }

    fn every_word(&self) -> impl Iterator<Item = &str> {
        self.announced
            .iter()
            .map(String::as_str)
            .chain(self.painted.iter().map(|(text, _)| text.as_str()))
    }
}

#[test]
fn the_default_states_the_built_in_lists() {
    let mut app = RSpiceApp::test_instance();
    let project = &app.state.model.workspace.content.project;
    let top = format!("{}/{}/schematic", project.root_library, project.top_cell);
    let shown = frame(&mut app, 1440.0);
    for fact in [
        "Default",
        "Used for runs",
        "New configuration\u{2026}",
        top.as_str(),
        "\u{2014}",
        "the instance's own view, then extracted, then spice",
        "Stop list 1: spice",
        "Stop list 2: verilog",
        "Stop list 3: veriloga",
        "Stop list 4: verilogams",
        "Stop list 5: extracted",
        "Stop the netlist",
    ] {
        assert!(shown.says(fact), "Default must state {fact:?}");
    }
    assert!(
        !shown.says("Apply") && !shown.says("Revert"),
        "Default is read-only"
    );
}

/// The band counts the circuit the variant populates, not every instance
/// the schematic draws.
#[test]
fn the_band_counts_only_the_populated_variant() {
    let mut app = reference_app();
    hierarchy_reference::omit_missing_instance(&mut app.state);
    let expected = app
        .state
        .model
        .workspace
        .design_projection(
            &app.state.model.library_manager,
            &app.state.model.workspace.content.active_view,
            &app.state.model.schematic,
        )
        .unwrap()
        .hierarchy_resolution()
        .clone();
    let authored = app.state.model.workspace.resolve_hierarchy_with_active(
        &app.state.model.library_manager,
        &app.state.model.workspace.content.active_view,
        &app.state.model.schematic,
    );
    assert_ne!(authored.total_instances, expected.total_instances);
    assert!(expected.is_valid());
    let total = expected.total_instances;
    for width in [640.0, 1440.0] {
        let shown = frame(&mut app, width);
        assert!(
            shown.says(&format!("{total} of {total} instances bound")),
            "the {width}px band must count {total} instances"
        );
    }
}

/// The candidate hierarchy is resolved when what it depends on changes, and
/// a frame that changes nothing resolves nothing.
#[test]
fn the_design_is_resolved_once_until_an_edit_commits() {
    let mut app = reference_app();
    frame(&mut app, 1440.0);
    assert_eq!(app.state.project.configuration.resolution_builds(), 1);
    rspice_design::projection::reset_materialization_count();
    frame(&mut app, 1440.0);
    frame(&mut app, 1440.0);
    assert_eq!(app.state.project.configuration.resolution_builds(), 1);
    assert_eq!(rspice_design::projection::materialization_count(), 0);

    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    let id = catalog.active_configuration_id().unwrap();
    app.state
        .project
        .configuration
        .edit(&catalog, id, |definition| {
            definition.executable_view_policy.push("spice".to_owned());
        });
    frame(&mut app, 1440.0);
    assert_eq!(app.state.project.configuration.resolution_builds(), 2);
    frame(&mut app, 1440.0);
    assert_eq!(app.state.project.configuration.resolution_builds(), 2);

    app.state
        .project
        .configuration
        .select(ConfigurationSelection::Default);
    frame(&mut app, 1440.0);
    assert_eq!(app.state.project.configuration.resolution_builds(), 3);
}

/// A drawing edit that leaves the hierarchy as it resolved, a wire drawn,
/// keeps the resolution the page shows; one that places an instance
/// resolves the design again.
#[test]
fn a_drawing_edit_that_places_no_instance_keeps_the_resolution() {
    use rspice_app_state::state::{LibraryCellInstance, Point};
    let mut app = reference_app();
    frame(&mut app, 1440.0);
    assert_eq!(app.state.project.configuration.resolution_builds(), 1);

    rspice_design::projection::reset_materialization_count();
    app.state.model.schematic.with_undo("Wire", |schematic| {
        schematic.add_wire(vec![Point::new(-800, -800), Point::new(-600, -800)]);
    });
    frame(&mut app, 1440.0);
    assert!(
        rspice_design::projection::materialization_count() > 0,
        "the wire moved the design"
    );
    assert_eq!(
        app.state.project.configuration.resolution_builds(),
        1,
        "a wire places no instance: the resolution holds"
    );

    app.state.model.schematic.with_undo("Place", |schematic| {
        schematic.add_library_cell_component(
            Point::new(-800, -1200),
            LibraryCellInstance::new("missing_library", "omitted_master", "schematic"),
        );
    });
    frame(&mut app, 1440.0);
    assert_eq!(
        app.state.project.configuration.resolution_builds(),
        2,
        "a placed instance resolves the design again"
    );
}

#[test]
fn a_dirty_configuration_offers_apply_and_revert() {
    let mut app = reference_app();
    let shown = frame(&mut app, 1440.0);
    assert!(shown.says("Used for runs"));
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    let id = catalog.active_configuration_id().unwrap();
    app.state
        .project
        .configuration
        .edit(&catalog, id, |definition| {
            definition.executable_view_policy.push("spice".to_owned());
        });
    let shown = frame(&mut app, 1440.0);
    assert!(shown.says("View list 2: spice"));
    assert!(shown.says("Apply") && shown.says("Revert"));
}

/// Every instance of the expanded design is a row, under its parent, with
/// what it resolved to and the view it is bound to.
#[test]
fn the_instances_tree_states_what_every_instance_resolved_to() {
    let mut app = reference_app();
    let shown = frame(&mut app, 1440.0);
    for (name, cell, to_use, rule) in [
        ("X1", "user/amp", "Inherited", "View list"),
        ("X2", "user/amp", "schematic", "Instance binding"),
    ] {
        let row = format!(
            "Instance: {name}, Cell: {cell}, View found: schematic, View to use: {to_use}, \
             Bound by: {rule}, Model section: "
        );
        assert!(
            shown.announced.iter().any(|label| label.starts_with(&row)),
            "no row starts {row:?} in {:#?}",
            shown.announced
        );
    }
    assert!(shown.says(
        "Table columns: Instance, Cell, Cell to use, View found, View to use, Bound by, \
         Model section, Status"
    ));

    let collapsed = app.state.project.configuration.visible_instances().len();
    app.state.project.configuration.toggle_collapsed("/");
    assert_eq!(app.state.project.configuration.visible_instances().len(), 1);
    assert!(collapsed > 1);
}

/// A hierarchy past a million instances lists the nets a run can start from
/// once per design: the Solver page asks every frame and is handed what was
/// listed, and an instance and what it places are found by halving, not by
/// reading every instance.
#[test]
fn a_hierarchy_past_a_million_instances_lists_its_nets_once() {
    use rspice_app_state::state::InstancePath;
    use rspice_simulation::output_contract::selection::{occurrence_at, occurrences_inside};
    let app = nested_app(4);
    let expanded = 1 + 32 + 32 * 32 + 32 * 32 * 32 + 32 * 32 * 32 * 32;
    let listed = app
        .state
        .model
        .simulation_occurrence_nets()
        .expect("the design resolves");
    assert_eq!(listed.len(), expanded);
    let again = app
        .state
        .model
        .simulation_occurrence_nets()
        .expect("the design resolves");
    assert!(
        std::sync::Arc::ptr_eq(&listed, &again),
        "the next frame is handed the same list"
    );

    assert_eq!(occurrences_inside(&listed, 0).len(), 32);
    for (scope, path) in [
        ("x32.x32.x32.x32", "/X32/X32/X32/X32"),
        ("/X1/X17", "/X1/X17"),
    ] {
        let at = occurrence_at(&listed, &InstancePath::parse(scope).expect("a path"))
            .unwrap_or_else(|| panic!("{scope} is found"));
        assert_eq!(listed[at].occurrence().to_string(), path);
    }
    let deepest = occurrence_at(
        &listed,
        &InstancePath::parse("/X32/X32/X32/X32").expect("a path"),
    )
    .expect("the deepest instance is found");
    assert!(occurrences_inside(&listed, deepest).is_empty());
    let middle = occurrence_at(&listed, &InstancePath::parse("/X1/X17").expect("a path"))
        .expect("an instance two deep is found");
    assert_eq!(occurrences_inside(&listed, middle).len(), 32);
}

/// A hierarchy that expands past a million instances opens on the page,
/// which lists an instance's children when it is opened, and netlists as the
/// cells it is: each written once, however often it is placed.
#[test]
fn a_hierarchy_past_a_million_instances_opens_on_the_page_and_netlists() {
    let mut app = nested_app(4);
    let expanded = 1 + 32 + 32 * 32 + 32 * 32 * 32 + 32 * 32 * 32 * 32;
    assert!(expanded > 1_000_000);

    let shown = frame(&mut app, 1440.0);
    for fact in [
        format!("{expanded} of {expanded} instances bound"),
        format!("{expanded} instances"),
    ] {
        assert!(
            shown.says(&fact),
            "the page must state {fact:?}: {:#?}",
            shown.every_word().collect::<Vec<_>>()
        );
    }
    let page = &mut app.state.project.configuration;
    assert_eq!(page.visible_instances().len(), 1 + 32);
    let mut open = String::new();
    for depth in 1..=3 {
        open.push_str("/X1");
        page.toggle_collapsed(&open);
        assert_eq!(page.visible_instances().len(), 1 + 32 * (depth + 1));
    }
    assert!(
        page.visible_instances()
            .iter()
            .any(|row| row.path() == "/X1/X1/X1/X32"),
        "the opened instance shows what it places"
    );
    frame(&mut app, 1440.0);

    let state = &app.state;
    let projection = state
        .model
        .workspace
        .configuration_execution_projection(
            &state.model.library_manager,
            &state.model.workspace.content.active_view,
            &state.model.schematic,
        )
        .expect("the design resolves for a run");
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_execution_projection(
        state.model.library_manager.catalog(),
        &projection,
    );
    let generated = rspice_simulation::netlist_gen::generate_netlist_hierarchical(
        projection.root_schematic().expect("a materialized root"),
        &[],
        &hierarchy,
        &rspice_simulation::netlist_gen::NetlistSourceData::new(
            &rspice_app_state::simulation::table_route::SourceFiles,
        ),
    );
    assert!(generated.errors.is_empty(), "{:?}", generated.errors);
    let bodies = generated
        .netlist
        .lines()
        .filter(|line| {
            line.trim_start()
                .to_ascii_lowercase()
                .starts_with(".subckt")
        })
        .count();
    assert_eq!(bodies, 4, "one body per cell");
    assert_eq!(generated.emission_map.len(), expanded - 1);
}

/// The Cells tab lists each library/cell once, and keeps a cell binding the
/// design does not use, marked, so it can be removed. The list a cell
/// inherits is stated only on a row with a binding of its own: the grid
/// above states it once for every cell that inherits.
#[test]
fn the_cells_tab_lists_each_cell_once_with_its_binding() {
    let mut app = reference_app();
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    let id = catalog.active_configuration_id().unwrap();
    app.state
        .project
        .configuration
        .edit(&catalog, id, |definition| {
            rspice_app_project::workbench::project::configuration_state::set_cell_view(
                definition,
                "vendor",
                "filter",
                Some("schematic"),
            );
            rspice_app_project::workbench::project::configuration_state::set_cell_view(
                definition,
                "user",
                "retired_block",
                Some("schematic"),
            );
        });
    app.state.project.configuration.tab =
        rspice_app_project::workbench::project::configuration_state::ConfigurationTab::Cells;
    let shown = frame(&mut app, 1440.0);
    assert!(shown.says(
        "Table columns: Library, Cell, Cell to use, Instances, View found, View to use, \
         Inherited view list"
    ));
    for row in [
        "Library: user, Cell: amp, Instances: 2, View found: schematic, View to use: Inherited",
        "Library: vendor, Cell: filter, Instances: 1, View found: schematic, \
         View to use: schematic, Inherited view list: schematic",
        "Library: user, Cell: retired_block, Instances: 0, \
         View found: Warning: not in this design, View to use: schematic, \
         Inherited view list: schematic",
    ] {
        assert!(shown.says(row), "no row {row:?} in {:#?}", shown.announced);
    }
    let amp_rows = shown
        .announced
        .iter()
        .filter(|label| label.starts_with("Library: user, Cell: amp,"))
        .count();
    assert_eq!(amp_rows, 1, "a cell instantiated twice is one row");
    // The count is the design's cells; the binding beside them is counted
    // apart, so the header agrees with the rows under it.
    assert!(
        shown.says("4 cells \u{b7} 1 binding not in this design"),
        "{:#?}",
        shown.every_word().collect::<Vec<_>>()
    );
}
