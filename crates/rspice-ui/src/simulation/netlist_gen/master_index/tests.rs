//! What the deck has to be true of: one body per master, one name per master,
//! and one typed defect per thing that went wrong.

use super::*;
use crate::simulation::netlist_gen::{generate_netlist, generate_netlist_hierarchical};
use crate::state::{
    Cell, ConfigurationSetCatalog, ConfigurationSetDefinition, ConfigurationSetOverride, Library,
    LibraryManager, NetLabel, PendingPortPlacement, Point, PortDirection, PortDirectionType,
    PortDiscipline, PortSpec, ProjectWorkspace, SymbolDocument, SymbolPin, UnresolvedBindingPolicy,
    View, ViewType, Wire,
};

fn instance_path(text: &str) -> InstancePath {
    InstancePath::parse(text).expect("fixture instance path")
}

/// The `.subckt` name a plan says one occurrence instantiates.
fn master_name<'a>(
    plan: &'a ConfigurationExecutionPlan,
    occurrence: &InstancePath,
) -> Option<&'a str> {
    plan.master(plan.occurrence_master(occurrence)?)
        .map(crate::state::workspace::MasterRecord::name)
}

fn place_port(state: &mut SchematicState, name: &str, pos: Point) {
    let id = state.add_component(ComponentType::Port, pos);
    let component = state
        .components
        .iter_mut()
        .find(|c| c.id == id)
        .expect("port exists");
    component.value = name.to_string();
}

fn binding(cell: &str, terminals: &[&str]) -> LibraryCellInstance {
    let mut binding = LibraryCellInstance::new("work", cell, "schematic");
    binding.terminal_order = terminals.iter().map(|t| t.to_string()).collect();
    binding
}

fn binding_in(library: &str, cell: &str, terminals: &[&str]) -> LibraryCellInstance {
    let mut binding = LibraryCellInstance::new(library, cell, "schematic");
    binding.terminal_order = terminals.iter().map(|t| t.to_string()).collect();
    binding
}

fn binding_with_interface(cell: &str, ports: &[PortSpec]) -> LibraryCellInstance {
    let mut binding = LibraryCellInstance::new("work", cell, "schematic");
    binding.bind_interface(ports);
    binding
}

/// `div`: resistor between ports a and b. Port terminals coincide with the
/// resistor terminals, so connectivity needs no wires.
fn div_master() -> SchematicState {
    let mut master = SchematicState::default();
    place_port(&mut master, "a", Point::new(20, 0));
    master.add_component(ComponentType::Resistor, Point::new(30, 0));
    place_port(&mut master, "b", Point::new(60, 0));
    master
}

/// A two-port master whose body is one resistor of the given value.
fn two_port_master(value: &str) -> SchematicState {
    let mut master = div_master();
    let resistor = master
        .components
        .iter_mut()
        .find(|component| component.kind == ComponentType::Resistor)
        .expect("the fixture master has a resistor");
    resistor.value = value.to_owned();
    master
}

/// Testbench instantiating `work/div`: V source into pin 1, pin 2 to ground.
fn top_with_instance(terminals: &[&str]) -> SchematicState {
    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 0), binding("div", terminals));
    top.add_component(ComponentType::VoltageSource, Point::new(40, 40));
    top.add_component(ComponentType::Ground, Point::new(130, 20));
    top.add_component(ComponentType::Ground, Point::new(40, 70));
    top.wires.push(Wire::new(
        1,
        vec![Point::new(40, 20), Point::new(40, 0), Point::new(70, 0)],
    ));
    top.wires
        .push(Wire::new(2, vec![Point::new(130, 0), Point::new(130, 10)]));
    top
}

fn subckt_headers(netlist: &str) -> Vec<String> {
    netlist
        .lines()
        .map(str::trim)
        .filter(|line| line.to_ascii_lowercase().starts_with(".subckt "))
        .map(str::to_owned)
        .collect()
}

/// The deck's own instance line for one reference.
///
/// A master body names its instances in the same space the deck does, so
/// `X1` inside a `.subckt` is a different instance from `X1` at the top. The
/// search therefore skips every definition block and answers about the deck.
fn instance_line<'a>(netlist: &'a str, reference: &str) -> &'a str {
    let needle = format!("{} ", reference.to_ascii_lowercase());
    let mut depth = 0_usize;
    netlist
        .lines()
        .find(|line| {
            let folded = line.trim_start().to_ascii_lowercase();
            if folded.starts_with(".subckt ") {
                depth += 1;
                return false;
            }
            if folded.starts_with(".ends") {
                depth = depth.saturating_sub(1);
                return false;
            }
            depth == 0 && folded.starts_with(&needle)
        })
        .unwrap_or_else(|| panic!("no top-level instance line for {reference} in:\n{netlist}"))
}

// =============================================================================
// One master, one body, one name
// =============================================================================

#[test]
fn project_cell_emits_subckt_definition() {
    let master = div_master();
    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "div", &master);

    let top = top_with_instance(&["a", "b"]);
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    let text = result.netlist.to_lowercase();
    let def = text.find(".subckt div a b").expect("definition present");
    assert!(text.contains(".ends div"), "netlist:\n{}", result.netlist);
    let body_resistor = text.find("r1 a b").expect("body uses port names");
    let instance = text.find("\nx1 ").expect("X line present");
    assert!(def < instance, "definition must precede use");
    assert!(def < body_resistor && body_resistor < instance);
    assert!(
        instance_line(&result.netlist, "X1")
            .to_lowercase()
            .ends_with(" div")
    );
}

#[test]
fn two_instances_of_one_cellview_share_one_master_body() {
    let master = div_master();
    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "div", &master);

    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 0), binding("div", &["a", "b"]));
    top.add_library_cell_component(Point::new(300, 0), binding("div", &["a", "b"]));
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert_eq!(
        subckt_headers(&result.netlist),
        vec![".subckt div a b".to_owned()],
        "two occurrences of one resolved cellview are one master:\n{}",
        result.netlist
    );
    for reference in ["X1", "X2"] {
        assert!(
            instance_line(&result.netlist, reference).ends_with(" div"),
            "{reference} must bind the shared master:\n{}",
            result.netlist
        );
    }
}

#[test]
fn cross_library_cell_name_collision_qualifies_the_second_master() {
    let work_amp = two_port_master("1k");
    let analog_amp = two_port_master("2k");
    let mut buffers = HashMap::new();
    buffers.insert(CellViewRef::new("work", "amp", "schematic").key(), work_amp);
    buffers.insert(
        CellViewRef::new("analog", "amp", "schematic").key(),
        analog_amp,
    );
    let hierarchy = HierarchySource::from_buffers(&buffers);

    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 0), binding_in("work", "amp", &["a", "b"]));
    top.add_library_cell_component(Point::new(300, 0), binding_in("analog", "amp", &["a", "b"]));
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert_eq!(
        subckt_headers(&result.netlist),
        vec![
            ".subckt amp a b".to_owned(),
            ".subckt amp__analog a b".to_owned()
        ],
        "an unqualified cell name is not an identity:\n{}",
        result.netlist
    );
    assert!(instance_line(&result.netlist, "X1").ends_with(" amp"));
    assert!(instance_line(&result.netlist, "X2").ends_with(" amp__analog"));
}

#[test]
fn nested_cells_emit_leaf_first() {
    let inner = div_master();

    let mut outer = SchematicState::default();
    place_port(&mut outer, "p", Point::new(20, 0));
    place_port(&mut outer, "q", Point::new(60, 0));
    outer.add_library_cell_component(Point::new(120, 0), binding("inner", &["a", "b"]));

    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "inner", &inner);
    hierarchy.insert("work", "outer", &outer);

    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 0), binding("outer", &["p", "q"]));
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    let text = result.netlist.to_lowercase();
    let inner_def = text.find(".subckt inner").expect("inner def");
    let outer_def = text.find(".subckt outer").expect("outer def");
    assert!(inner_def < outer_def, "leaf definitions come first");
}

#[test]
fn typed_port_order_drives_subckt_header_after_storage_reordering() {
    let mut master = SchematicState::default();
    for name in ["IN", "OUT"] {
        let pending = PendingPortPlacement::new(
            name,
            if name == "IN" {
                PortDirectionType::InputAnalog
            } else {
                PortDirectionType::OutputAnalog
            },
            PortDiscipline::Electrical,
            master.topology_version(),
            master.next_interface_order(),
        );
        master
            .place_pending_port(Point::origin(), pending)
            .expect("typed port places");
    }
    master.components.reverse();
    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "ordered", &master);
    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 100), binding("ordered", &["IN", "OUT"]));

    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    assert!(
        result.netlist.contains(".subckt ordered IN OUT"),
        "netlist:\n{}",
        result.netlist
    );
}

#[test]
fn empty_binding_resolves_master_ports() {
    let master = div_master();
    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "div", &master);

    // Legacy placement: terminal_order never populated. The master has two
    // ports and the generic instance has two pins — resolvable.
    let top = top_with_instance(&[]);
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert!(result.netlist.to_lowercase().contains(".subckt div a b"));
}

#[test]
fn explicit_zero_port_project_cell_is_netlistable() {
    let mut master = SchematicState::default();
    master.add_component(ComponentType::Resistor, Point::new(30, 0));

    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "isolated", &master);

    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 0), binding_with_interface("isolated", &[]));
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert!(result.netlist.contains(".subckt isolated"));
    assert!(result.netlist.contains("X1  isolated"));
}

#[test]
fn authored_symbol_pin_positions_define_cell_instance_connectivity() {
    let ports = vec![
        PortSpec {
            name: "IN".to_owned(),
            direction: PortDirection::In,
        },
        PortSpec {
            name: "OUT".to_owned(),
            direction: PortDirection::Out,
        },
    ];

    let mut master = SchematicState::default();
    place_port(&mut master, "IN", Point::new(0, 0));
    place_port(&mut master, "OUT", Point::new(40, 0));

    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
        ],
        ..SymbolDocument::default()
    };
    let mut libraries = LibraryManager::new();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    document
        .store_in_view(&mut symbol_view)
        .expect("symbol stores");
    cell.add_view(symbol_view);
    library.add_cell(cell);
    libraries.add_library(library);

    let mut buffers = HashMap::new();
    buffers.insert(CellViewRef::new("work", "amp", "schematic").key(), master);

    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 50), binding_with_interface("amp", &ports));
    top.net_labels
        .push(NetLabel::new(1, Point::new(60, 40), "vin"));
    top.net_labels
        .push(NetLabel::new(2, Point::new(170, 70), "vout"));

    let hierarchy = HierarchySource::from_workspace(&libraries, &buffers);
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert!(
        instance_line(&result.netlist, "X1")
            .split_whitespace()
            .eq(["X1", "vin", "vout", "amp"]),
        "unexpected instance line:\n{}",
        result.netlist
    );
}

#[test]
fn flat_generation_is_unchanged_without_hierarchy() {
    let top = top_with_instance(&["a", "b"]);
    let result = generate_netlist(&top);
    // No hierarchy source: the project master is unresolvable, which is an
    // error — but no .SUBCKT block is invented.
    assert!(!result.netlist.to_lowercase().contains(".subckt"));
    assert!(!result.errors.is_empty());
}

// =============================================================================
// Typed defects
// =============================================================================

#[test]
fn recursion_by_lib_cell_view_is_a_typed_defect() {
    let mut looper = SchematicState::default();
    place_port(&mut looper, "a", Point::new(20, 0));
    place_port(&mut looper, "b", Point::new(60, 0));
    looper.add_library_cell_component(Point::new(120, 0), binding("loop", &["a", "b"]));

    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "loop", &looper);

    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 0), binding("loop", &["a", "b"]));
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    let chain = result
        .defects
        .iter()
        .find_map(|defect| match defect {
            NetlistDefect::RecursiveHierarchy { chain } => Some(chain.clone()),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no recursion defect in {:?}", result.defects));
    assert!(
        chain
            .iter()
            .all(|reference| reference.key().eq_ignore_ascii_case("work/loop/schematic")),
        "{chain:?}"
    );
    assert!(
        result
            .errors
            .iter()
            .any(|error| error.contains("Recursive cell hierarchy")),
        "errors: {:?}",
        result.errors
    );
}

#[test]
fn missing_master_is_an_error() {
    let hierarchy = HierarchySource::empty();
    let top = top_with_instance(&["a", "b"]);
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);
    assert!(
        result
            .defects
            .iter()
            .any(|defect| matches!(defect, NetlistDefect::MissingMaster { .. })),
        "defects: {:?}",
        result.defects
    );
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.contains("master not found in project")),
        "errors: {:?}",
        result.errors
    );
}

#[test]
fn stale_interface_is_one_typed_defect_at_every_site() {
    let master = div_master(); // master order: a, b
    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "div", &master);

    for placed in [
        vec!["a", "b", "c"], // count changed
        vec!["b", "a"],      // same count, wrong order
    ] {
        let top = top_with_instance(&placed);
        let result = generate_netlist_hierarchical(&top, &[], &hierarchy);
        let stale = result
            .defects
            .iter()
            .filter(|defect| matches!(defect, NetlistDefect::StaleInterface { .. }))
            .collect::<Vec<_>>();
        assert_eq!(
            stale.len(),
            1,
            "{placed:?} must report the stale interface exactly once: {:?}",
            result.defects
        );
        if let NetlistDefect::StaleInterface {
            occurrence,
            expected,
            actual,
            ..
        } = stale[0]
        {
            assert_eq!(occurrence, &instance_path("/X1"));
            assert_eq!(expected.as_slice(), ["a".to_owned(), "b".to_owned()]);
            assert_eq!(
                actual.as_slice(),
                placed
                    .iter()
                    .map(|name| (*name).to_owned())
                    .collect::<Vec<_>>()
                    .as_slice()
            );
        } else {
            panic!("the filter kept only stale interfaces: {:?}", stale[0]);
        }
        assert_eq!(
            result
                .errors
                .iter()
                .filter(|error| error.contains("interface no longer matches"))
                .count(),
            1,
            "errors: {:?}",
            result.errors
        );
        assert!(
            result.errors.iter().any(|error| error.contains("is stale")),
            "errors: {:?}",
            result.errors
        );
    }
}

#[test]
fn legacy_instance_cannot_bind_a_zero_port_master() {
    let mut master = SchematicState::default();
    master.add_component(ComponentType::Resistor, Point::new(30, 0));

    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "div", &master);

    let top = top_with_instance(&["a", "b"]);
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.contains("interface no longer matches")),
        "errors: {:?}",
        result.errors
    );
}

#[test]
fn port_tied_to_ground_is_an_error() {
    let mut master = div_master();
    // Ground terminal coincides with port a's net.
    master.add_component(ComponentType::Ground, Point::new(10, 10));

    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "div", &master);

    let top = top_with_instance(&["a", "b"]);
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);
    assert!(
        result
            .defects
            .iter()
            .any(|defect| matches!(defect, NetlistDefect::PortTiedToGround { .. })),
        "defects: {:?}",
        result.defects
    );
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.contains("tied to ground inside")),
        "errors: {:?}",
        result.errors
    );
}

// =============================================================================
// Declared parameters
// =============================================================================

fn libraries_with_parameter_contract(
    contract: &str,
) -> (LibraryManager, HashMap<String, SchematicState>) {
    let mut libraries = LibraryManager::new();
    let mut library = Library::new("work");
    let mut cell = Cell::new("div");
    let mut view = View::new("schematic", ViewType::Schematic);
    view.metadata
        .insert("netlist.parameter_contract".to_owned(), contract.to_owned());
    cell.add_view(view);
    library.add_cell(cell);
    libraries.add_library(library);

    let mut buffers = HashMap::new();
    buffers.insert(
        CellViewRef::new("work", "div", "schematic").key(),
        two_port_master("1k"),
    );
    (libraries, buffers)
}

#[test]
fn declared_parameters_reach_the_subckt_header_and_the_deck_parses() {
    let (libraries, buffers) = libraries_with_parameter_contract(
        r#"[{"name":"rload","default":"1k"},{"name":"gain","default":"2"}]"#,
    );
    let hierarchy = HierarchySource::from_workspace(&libraries, &buffers);

    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 0), binding("div", &["a", "b"]));
    top.components
        .iter_mut()
        .find(|component| component.kind == ComponentType::CellInstance)
        .expect("the placed instance is retained")
        .params = "rload=4k".to_owned();
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert_eq!(
        subckt_headers(&result.netlist),
        vec![".subckt div a b params: rload=1k gain=2".to_owned()],
        "netlist:\n{}",
        result.netlist
    );
    assert!(
        instance_line(&result.netlist, "X1").contains("rload=4k"),
        "netlist:\n{}",
        result.netlist
    );
    rspice_core::Netlist::parse(&result.netlist)
        .expect("a deck declaring formals and overriding one parses");
}

#[test]
fn required_parameter_without_default_is_a_defect() {
    let (libraries, buffers) =
        libraries_with_parameter_contract(r#"[{"name":"rload","required":true}]"#);
    let hierarchy = HierarchySource::from_workspace(&libraries, &buffers);

    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 0), binding("div", &["a", "b"]));
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    assert!(
        result.defects.iter().any(|defect| matches!(
            defect,
            NetlistDefect::ParameterWithoutDefault { parameter, .. } if parameter == "rload"
        )),
        "defects: {:?}",
        result.defects
    );
    assert!(
        !result.netlist.contains("params:"),
        "a parameter with no value cannot be declared:\n{}",
        result.netlist
    );
}

// =============================================================================
// Emission map
// =============================================================================

#[test]
fn emission_map_covers_every_occurrence() {
    let inner = div_master();
    let mut outer = SchematicState::default();
    place_port(&mut outer, "p", Point::new(20, 0));
    place_port(&mut outer, "q", Point::new(60, 0));
    outer.add_library_cell_component(Point::new(120, 0), binding("inner", &["a", "b"]));

    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "inner", &inner);
    hierarchy.insert("work", "outer", &outer);

    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 0), binding("outer", &["p", "q"]));
    top.add_library_cell_component(Point::new(400, 0), binding("outer", &["p", "q"]));
    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    let mut rows = result
        .emission_map
        .iter()
        .map(|row| {
            (
                row.occurrence.to_string(),
                row.master.clone(),
                row.engine_prefix.clone(),
            )
        })
        .collect::<Vec<_>>();
    rows.sort();
    assert_eq!(
        rows,
        vec![
            ("/X1".to_owned(), "outer".to_owned(), "X1".to_owned()),
            ("/X1/X1".to_owned(), "inner".to_owned(), "X1.X1".to_owned()),
            ("/X2".to_owned(), "outer".to_owned(), "X2".to_owned()),
            ("/X2/X1".to_owned(), "inner".to_owned(), "X2.X1".to_owned()),
        ],
        "every occurrence names the master it was emitted against"
    );
    assert_eq!(
        result
            .emission_map
            .iter()
            .map(|row| row.master_reference.key())
            .collect::<std::collections::BTreeSet<_>>(),
        [
            "work/inner/schematic".to_owned(),
            "work/outer/schematic".to_owned()
        ]
        .into_iter()
        .collect()
    );
}

// =============================================================================
// Configured hierarchies
// =============================================================================

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn configured_exact_paths_materialize_distinct_schematic_and_source_views() {
    let source_path =
        std::env::temp_dir().join(format!("rspice-config-view-{}.cir", uuid::Uuid::new_v4()));
    std::fs::write(
        &source_path,
        ".lib tt\n.subckt div_sp a b\nRsrc a b 9k\n.ends div_sp\n.endl tt\n",
    )
    .expect("source fixture writes");

    let mut libraries = LibraryManager::new();
    let mut user = Library::new("user");
    let mut top_cell = Cell::new("top");
    top_cell.add_view(View::new("schematic", ViewType::Schematic));
    user.add_cell(top_cell);
    libraries.add_library(user);

    let mut work = Library::new("work");
    let mut div = Cell::new("div");
    div.add_view(View::new("schematic", ViewType::Schematic));
    let mut spice = View::new("spice", ViewType::Spice).with_path(source_path.clone());
    spice
        .metadata
        .insert("netlist.ports".to_owned(), "a,b".to_owned());
    spice
        .metadata
        .insert("netlist.module".to_owned(), "div_sp".to_owned());
    div.add_view(spice);
    work.add_cell(div);
    libraries.add_library(work);

    let mut workspace = ProjectWorkspace::default();
    workspace.schematic_buffers.insert(
        CellViewRef::new("work", "div", "schematic").key(),
        div_master(),
    );
    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 0), binding("div", &["a", "b"]));
    top.add_library_cell_component(Point::new(240, 0), binding("div", &["a", "b"]));
    workspace
        .schematic_buffers
        .insert(workspace.active_view.key(), top.clone());

    let mut catalog = ConfigurationSetCatalog::default();
    catalog
        .create(ConfigurationSetDefinition {
            name: "Mixed implementation".to_owned(),
            root: workspace.active_view.clone(),
            dut_path: "/X1".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: vec![ConfigurationSetOverride {
                instance_path: "/X2".to_owned(),
                executable_views: vec!["spice".to_owned()],
                stop_view: Some("spice".to_owned()),
                model_section: Some("tt".to_owned()),
                eligible_platforms: vec![crate::state::ConfigurationPlatform::Desktop],
            }],
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "test".to_owned(),
        })
        .expect("configuration creates");
    workspace.configuration_sets = catalog;

    let active = workspace.active_view.clone();
    let projection = workspace
        .configuration_execution_projection(&libraries, &active, &top)
        .expect("configuration resolves into an execution plan");
    let plan = projection.plan();
    let x1 = plan.binding(&instance_path("/X1")).expect("X1 binding");
    let x2 = plan.binding(&instance_path("/X2")).expect("X2 binding");
    assert_eq!(x1.resolved_reference().view, "schematic");
    assert_eq!(x2.resolved_reference().view, "spice");
    assert_eq!(x2.model_section(), Some("tt"));
    assert_eq!(
        master_name(plan, &instance_path("/X1")),
        Some("div"),
        "the schematic occurrence takes the cell's own name"
    );
    assert_eq!(
        plan.occurrence_master(&instance_path("/X2")),
        None,
        "a source-backed view is an engine module, not a generated master"
    );

    let hierarchy = HierarchySource::from_execution_projection(&libraries, &projection);
    let result = generate_netlist_hierarchical(
        projection.root_schematic().expect("root schematic"),
        &[],
        &hierarchy,
    );
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    let source_literal = source_path.to_string_lossy();
    assert!(
        result
            .netlist
            .contains(&format!(".lib \"{}\" tt", source_literal))
    );
    assert!(
        !result.netlist.contains("__cfg_"),
        "the path-hashed alias is gone:\n{}",
        result.netlist
    );
    assert_eq!(
        subckt_headers(&result.netlist),
        vec![".subckt div a b".to_owned()],
        "netlist:\n{}",
        result.netlist
    );
    assert!(instance_line(&result.netlist, "X1").ends_with(" div"));
    assert!(instance_line(&result.netlist, "X2").ends_with(" div_sp"));

    let active_configuration = workspace
        .configuration_sets
        .active()
        .expect("active configuration")
        .clone();
    let mut desktop_incompatible = active_configuration.definition().clone();
    desktop_incompatible.overrides[0].eligible_platforms =
        vec![crate::state::ConfigurationPlatform::Browser];
    workspace
        .configuration_sets
        .update(
            active_configuration.id(),
            active_configuration.revision(),
            desktop_incompatible,
        )
        .expect("platform policy update");
    let platform_error = workspace
        .configuration_execution_projection(&libraries, &active, &top)
        .expect_err("desktop-incompatible binding blocks projection");
    assert!(
        platform_error
            .to_string()
            .contains("not supported by this execution target")
    );

    let _ = std::fs::remove_file(source_path);
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn two_instances_with_different_descendant_bindings_emit_v2() {
    let source_path =
        std::env::temp_dir().join(format!("rspice-master-v2-{}.cir", uuid::Uuid::new_v4()));
    std::fs::write(
        &source_path,
        ".subckt div_sp a b\nRsrc a b 9k\n.ends div_sp\n",
    )
    .expect("source fixture writes");

    let mut libraries = LibraryManager::new();
    let mut user = Library::new("user");
    let mut top_cell = Cell::new("top");
    top_cell.add_view(View::new("schematic", ViewType::Schematic));
    user.add_cell(top_cell);
    libraries.add_library(user);

    let mut work = Library::new("work");
    let mut div = Cell::new("div");
    div.add_view(View::new("schematic", ViewType::Schematic));
    let mut spice = View::new("spice", ViewType::Spice).with_path(source_path.clone());
    spice
        .metadata
        .insert("netlist.ports".to_owned(), "a,b".to_owned());
    spice
        .metadata
        .insert("netlist.module".to_owned(), "div_sp".to_owned());
    div.add_view(spice);
    work.add_cell(div);
    let mut wrap = Cell::new("wrap");
    wrap.add_view(View::new("schematic", ViewType::Schematic));
    work.add_cell(wrap);
    libraries.add_library(work);

    // `wrap` presents p/q and contains exactly one `div`, so the only thing
    // that can distinguish two `wrap` occurrences is what their child binds to.
    let mut wrap_master = SchematicState::default();
    place_port(&mut wrap_master, "p", Point::new(20, 0));
    place_port(&mut wrap_master, "q", Point::new(60, 0));
    wrap_master.add_library_cell_component(Point::new(120, 0), binding("div", &["a", "b"]));

    let mut workspace = ProjectWorkspace::default();
    workspace.schematic_buffers.insert(
        CellViewRef::new("work", "div", "schematic").key(),
        div_master(),
    );
    workspace.schematic_buffers.insert(
        CellViewRef::new("work", "wrap", "schematic").key(),
        wrap_master,
    );
    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 0), binding("wrap", &["p", "q"]));
    top.add_library_cell_component(Point::new(400, 0), binding("wrap", &["p", "q"]));
    workspace
        .schematic_buffers
        .insert(workspace.active_view.key(), top.clone());

    let mut catalog = ConfigurationSetCatalog::default();
    catalog
        .create(ConfigurationSetDefinition {
            name: "Split descendant".to_owned(),
            root: workspace.active_view.clone(),
            dut_path: "/X1".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: vec![ConfigurationSetOverride {
                instance_path: "/X2/X1".to_owned(),
                executable_views: vec!["spice".to_owned()],
                stop_view: Some("spice".to_owned()),
                model_section: None,
                eligible_platforms: vec![crate::state::ConfigurationPlatform::Desktop],
            }],
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "test".to_owned(),
        })
        .expect("configuration creates");
    workspace.configuration_sets = catalog;

    let active = workspace.active_view.clone();
    let projection = workspace
        .configuration_execution_projection(&libraries, &active, &top)
        .expect("configuration resolves into an execution plan");
    let hierarchy = HierarchySource::from_execution_projection(&libraries, &projection);
    let result = generate_netlist_hierarchical(
        projection.root_schematic().expect("root schematic"),
        &[],
        &hierarchy,
    );

    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert!(
        !result.netlist.contains("__cfg_"),
        "no path-hashed alias survives:\n{}",
        result.netlist
    );
    assert_eq!(
        subckt_headers(&result.netlist),
        vec![
            ".subckt div a b".to_owned(),
            ".subckt wrap p q".to_owned(),
            ".subckt wrap__v2 p q".to_owned(),
        ],
        "one cellview with two binding closures is two masters:\n{}",
        result.netlist
    );
    assert!(instance_line(&result.netlist, "X1").ends_with(" wrap"));
    assert!(instance_line(&result.netlist, "X2").ends_with(" wrap__v2"));

    let _ = std::fs::remove_file(source_path);
}

#[test]
fn configured_builtin_xspice_is_a_valid_executable_leaf() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    let descriptor = crate::state::engine_only_xspice_devices()
        .iter()
        .find(|descriptor| descriptor.model_type == "astate")
        .expect("astate catalog descriptor");
    let binding =
        crate::state::builtin_xspice_library_binding(descriptor).expect("astate placement binding");
    let top = workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top schematic");
    top.components.clear();
    top.add_library_cell_component(Point::new(100, 100), binding.clone());
    assert_eq!(top.components[0].name, "A1");

    workspace
        .configuration_sets
        .create(ConfigurationSetDefinition {
            name: "Built-in code model".to_owned(),
            root: CellViewRef::default_top(),
            dut_path: "/A1".to_owned(),
            executable_view_policy: vec!["schematic".to_owned(), "spice".to_owned()],
            stop_views: vec!["spice".to_owned()],
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Catalog test".to_owned(),
        })
        .expect("configuration creates");

    let active = workspace.active_view.clone();
    let top = workspace
        .schematic_buffers
        .get(&CellViewRef::default_top().key())
        .expect("top schematic")
        .clone();
    let projection = workspace
        .configuration_execution_projection(&libraries, &active, &top)
        .expect("built-in resolves under configuration");
    let execution = projection
        .plan()
        .binding(&instance_path("/A1"))
        .expect("exact built-in execution binding");
    assert_eq!(execution.resolved_view_type(), ViewType::Custom);
    assert_eq!(execution.materialized_binding(), Some(&binding));
    assert!(execution.stop_boundary());

    let hierarchy = HierarchySource::from_execution_projection(&libraries, &projection);
    let result = generate_netlist_hierarchical(
        projection.root_schematic().expect("root schematic"),
        &[],
        &hierarchy,
    );
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert!(result.netlist.contains(".MODEL a1_model astate"));
    assert!(result.netlist.lines().any(|line| line.starts_with("A1 ")));
    rspice_core::Netlist::parse(&result.netlist).expect("configured built-in deck parses");
}
