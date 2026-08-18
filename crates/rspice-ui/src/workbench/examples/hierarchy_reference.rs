//! The one hierarchical project every hierarchy fixture is built from.
//!
//! Golden decks, navigator journeys, and project-format migrations all need a
//! design that is hierarchical in every sense the product supports at once:
//! two libraries, a cell instantiated twice, the same cell name in two
//! libraries, a multi-sheet master joined by an explicit cross-sheet port, an
//! authored symbol beside its schematic, a typed bus with a scalar tap, a
//! global supply label, an active configuration set, and a saved output. A
//! suite that invents its own fixture answers only for the shape it invented,
//! so every later wave builds from this one instead.
//!
//! Three rules keep it usable as a reference:
//!
//! - Nothing here is hand-assembled state. Every object is created through the
//!   API the application itself calls, so this fixture cannot encode a
//!   combination the product has no way to reach.
//! - Both cross-sheet anchors are wire points. A component-terminal anchor
//!   resolves through `SchematicState::connections`, which is runtime state
//!   cleared on save, so such an anchor would materialize in this session and
//!   fail in a reloaded one.
//! - `amp` declares `VDD` as an interface pin its own body does not load. A
//!   supply pin present in the interface and absent from the emitted body is
//!   exactly what a `.subckt` port list has to carry, so the reference states
//!   that case rather than avoiding it.

use crate::product::SimulationPlanId;
use crate::state::{
    BusDeclaration, BusNotation, BusSlice, BusTapOrientation, Cell, CellViewRef, ComponentType,
    ConfigurationBlackBoxPolicy, ConfigurationModelProfile, ConfigurationPlatform,
    ConfigurationSetDefinition, ConfigurationSetId, ConfigurationSetOverride, CrossSheetDiscipline,
    CrossSheetPortAnchor, CrossSheetPortDefinition, CrossSheetPortDirection,
    CrossSheetPortEndpoint, CrossSheetSignalType, Library, LibraryCellInstance,
    MoveBoundaryResolution, MoveSelectionRequest, PendingPortPlacement, Point, PortDirectionType,
    PortDiscipline, PortSpec, SavedOutput, SavedOutputCompatibility, SavedOutputKind,
    SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming, SchematicState, SheetDefinition,
    SheetId, SheetPortPolicy, SheetTemplate, SymbolDocument, SymbolEditorMetadata,
    SymbolTextObject, UnresolvedBindingPolicy, View, ViewType,
};
use crate::workbench::app_state::AppState;

/// The project library the bootstrapped workspace already owns.
pub(crate) const USER_LIBRARY: &str = "user";
/// A second writable library. A read-only vendor tier is a different fixture
/// with different failure modes; this one isolates the name collision.
pub(crate) const VENDOR_LIBRARY: &str = "vendor";
pub(crate) const TOP_CELL: &str = "top";
pub(crate) const AMP_CELL: &str = "amp";
/// Deliberately present in both libraries: an unqualified cell name is not an
/// identity, and every consumer of this fixture has to prove it treats it as
/// one only when a library qualifies it.
pub(crate) const FILTER_CELL: &str = "filter";
pub(crate) const SCHEMATIC_VIEW: &str = "schematic";
pub(crate) const SYMBOL_VIEW: &str = "symbol";

/// Instance names, in the order `top` places them.
pub(crate) const AMP_INSTANCES: [&str; 2] = ["X1", "X2"];
pub(crate) const FILTER_INSTANCES: [&str; 2] = ["XF1", "XF2"];

/// The net that crosses `amp`'s sheet boundary, and the sheets it joins.
pub(crate) const CROSS_SHEET_NET: &str = "mid";
pub(crate) const AMP_SHEET_NAMES: [&str; 2] = ["Input stage", "Output stage"];

/// `amp`'s interface, in `.subckt` port order.
pub(crate) const AMP_PORTS: [&str; 4] = ["IN", "OUT", "VDD", "GND"];
/// Both `filter` cells present the same interface, which is what makes the
/// name collision worth testing.
pub(crate) const FILTER_PORTS: [&str; 2] = ["A", "B"];

/// The typed bus and the single scalar member ripped out of it. A free-form
/// label on a tapped net is a blocking netlist error unless it names the exact
/// member, so the labelled net carries the member name itself.
pub(crate) const DATA_BUS: &str = "DATA";
pub(crate) const DATA_BUS_MEMBER: &str = "DATA<0>";
/// The trailing `!` is what promotes a label to a global under the default
/// connectivity contract.
pub(crate) const GLOBAL_SUPPLY_NET: &str = "VDD!";
pub(crate) const TOP_OUTPUT_NET: &str = "out";
pub(crate) const SAVED_OUTPUT_EXPRESSION: &str = "V(out)";

/// Length of the wire stub drawn outward from an instance terminal, in grid
/// units. Two pins of a generated block sit one pin pitch apart, so a stub is
/// exactly long enough to carry its label clear of its own pin and no longer.
const STUB_LENGTH: i32 = 20;

/// The built project and the identities a test addresses it by.
pub(crate) struct HierarchyReference {
    pub(crate) state: AppState,
    pub(crate) top: CellViewRef,
    pub(crate) amp_schematic: CellViewRef,
    pub(crate) amp_symbol: CellViewRef,
    pub(crate) user_filter: CellViewRef,
    pub(crate) vendor_filter: CellViewRef,
    /// `amp`'s sheets in page order: the resistor is on the first, the
    /// capacitor on the second.
    pub(crate) amp_sheets: [SheetId; 2],
    pub(crate) configuration: ConfigurationSetId,
    pub(crate) plan: SimulationPlanId,
}

impl HierarchyReference {
    /// Absolute execution path of one instance placed in `top`.
    pub(crate) fn instance_path(instance: &str) -> String {
        format!("/{TOP_CELL}/{instance}")
    }
}

/// Build the canonical hierarchical project.
pub(crate) fn build() -> HierarchyReference {
    let mut state = AppState::default();
    let top = CellViewRef::new(USER_LIBRARY, TOP_CELL, SCHEMATIC_VIEW);
    let amp_schematic = CellViewRef::new(USER_LIBRARY, AMP_CELL, SCHEMATIC_VIEW);
    let amp_symbol = CellViewRef::new(USER_LIBRARY, AMP_CELL, SYMBOL_VIEW);
    let user_filter = CellViewRef::new(USER_LIBRARY, FILTER_CELL, SCHEMATIC_VIEW);
    let vendor_filter = CellViewRef::new(VENDOR_LIBRARY, FILTER_CELL, SCHEMATIC_VIEW);

    provision_libraries(&mut state);

    let boundary = author_amp(&mut state, &amp_schematic);
    let amp_sheets = split_amp_across_sheets(&mut state, &amp_schematic, &boundary);
    author_amp_symbol(&mut state, &amp_schematic, &amp_symbol);

    author_filter(&mut state, &user_filter, "1k", "1n");
    author_filter(&mut state, &vendor_filter, "2k", "2n");

    author_top(&mut state, &top);

    let configuration = install_configuration(&mut state, &top);
    let plan = install_saved_output(&mut state);

    HierarchyReference {
        state,
        top,
        amp_schematic,
        amp_symbol,
        user_filter,
        vendor_filter,
        amp_sheets,
        configuration,
        plan,
    }
}

/// Create the cells and views the design refers to. `user` already exists as
/// the bootstrapped project library; `vendor` is added beside it.
///
/// `amp`'s symbol view is created empty on purpose. An existing symbol view
/// without the `generated` marker is one the author owns, so the schematic
/// synchronization leaves it alone and the authored document below is the only
/// thing that ever fills it.
fn provision_libraries(state: &mut AppState) {
    let mut amp = Cell::new(AMP_CELL);
    amp.description = "Two-sheet RC stage with an authored symbol".to_owned();
    amp.add_view(View::new(SCHEMATIC_VIEW, ViewType::Schematic));
    amp.add_view(View::new(SYMBOL_VIEW, ViewType::Symbol));

    let mut user_filter = Cell::new(FILTER_CELL);
    user_filter.description = "Project RC filter".to_owned();
    user_filter.add_view(View::new(SCHEMATIC_VIEW, ViewType::Schematic));

    let user = state
        .library_manager
        .get_library_mut(USER_LIBRARY)
        .expect("the bootstrapped workspace owns the project library");
    user.add_cell(amp);
    user.add_cell(user_filter);

    let mut vendor_filter = Cell::new(FILTER_CELL);
    vendor_filter.description = "Vendor RC filter sharing the project cell name".to_owned();
    vendor_filter.add_view(View::new(SCHEMATIC_VIEW, ViewType::Schematic));
    let mut vendor = Library::new(VENDOR_LIBRARY);
    vendor
        .metadata
        .insert("role".to_owned(), "vendor".to_owned());
    vendor.add_cell(vendor_filter);
    state.library_manager.add_library(vendor);
}

/// The object identities `amp`'s cross-sheet contract is anchored to, and the
/// selection that moves to the second sheet.
struct AmpBoundary {
    first_wire: u64,
    first_point: Point,
    second_wire: u64,
    second_point: Point,
    moved_objects: Vec<u64>,
}

/// Draw `amp` as `IN -> R1 -> mid -> C1 -> OUT`, with `VDD` and `GND`
/// declared.
///
/// Both halves are drawn in one document. A sheet is a membership decision
/// over these object identities, not a second document, which is why the split
/// below can happen after the drawing is complete.
fn author_amp(state: &mut AppState, reference: &CellViewRef) -> AmpBoundary {
    state.open_workspace_view(reference.clone());
    let schematic = &mut state.schematic;

    place_port(
        schematic,
        Point::new(10, 0),
        AMP_PORTS[0],
        PortDirectionType::InputAnalog,
    );
    add_wire(schematic, Point::new(0, 0), Point::new(40, 0));
    let resistor = schematic.add_component(ComponentType::Resistor, Point::new(60, 0));
    set_value(schematic, resistor, "1k");
    let first_wire = add_wire(schematic, Point::new(80, 0), Point::new(140, 0));

    let second_wire = add_wire(schematic, Point::new(0, 200), Point::new(40, 200));
    let capacitor = schematic.add_component(ComponentType::Capacitor, Point::new(60, 200));
    set_value(schematic, capacitor, "1n");
    let output_wire = add_wire(schematic, Point::new(80, 200), Point::new(120, 200));
    let output_port = place_port(
        schematic,
        Point::new(130, 200),
        AMP_PORTS[1],
        PortDirectionType::OutputAnalog,
    );

    place_port(
        schematic,
        Point::new(10, 300),
        AMP_PORTS[2],
        PortDirectionType::InOutPower,
    );
    place_port(
        schematic,
        Point::new(10, 360),
        AMP_PORTS[3],
        PortDirectionType::InOutPower,
    );

    state.sync_active_schematic_to_workspace();
    AmpBoundary {
        first_wire,
        first_point: Point::new(140, 0),
        second_wire,
        second_point: Point::new(0, 200),
        moved_objects: vec![second_wire, capacitor, output_wire, output_port],
    }
}

/// Put every `amp` object on one governed sheet, add a second, then move the
/// output half across with the explicit boundary contract that keeps `mid`
/// electrically whole.
fn split_amp_across_sheets(
    state: &mut AppState,
    reference: &CellViewRef,
    boundary: &AmpBoundary,
) -> [SheetId; 2] {
    let key = reference.key();
    let objects = live_object_ids(
        state
            .workspace
            .schematic_buffers
            .get(&key)
            .expect("the amp schematic was saved to the workspace"),
    );

    let mut candidate = state.workspace.design_management.clone();
    let first = candidate
        .bootstrap_for_cell_view(&key, AMP_SHEET_NAMES[0], objects)
        .expect("a fresh cell view accepts its first governed sheet");
    let catalog = candidate
        .sheet_catalog_mut(&key)
        .expect("the sheet catalog was just created");
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: AMP_SHEET_NAMES[1].to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .expect("a second governed sheet inserts after the first");
    catalog
        .move_selection(MoveSelectionRequest {
            expected_catalog_revision: catalog.revision(),
            object_ids: boundary.moved_objects.clone(),
            destination_sheet_id: second,
            boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                ports: vec![CrossSheetPortDefinition {
                    net_name: CROSS_SHEET_NET.to_owned(),
                    first: CrossSheetPortEndpoint {
                        sheet_id: first,
                        anchor: CrossSheetPortAnchor::WirePoint {
                            wire_id: boundary.first_wire,
                            point: boundary.first_point,
                        },
                    },
                    second: CrossSheetPortEndpoint {
                        sheet_id: second,
                        anchor: CrossSheetPortAnchor::WirePoint {
                            wire_id: boundary.second_wire,
                            point: boundary.second_point,
                        },
                    },
                    direction: CrossSheetPortDirection::Output,
                    signal_type: CrossSheetSignalType::Analog,
                    discipline: CrossSheetDiscipline::Electrical,
                }],
            },
        })
        .expect("the reviewed boundary contract moves the output half");
    state
        .workspace
        .replace_design_management(candidate)
        .expect("the reviewed sheet catalog publishes");
    [first, second]
}

/// Author `amp`'s symbol from its own interface and give it a text object, so
/// the reference carries a hand-authored symbol rather than the generated view
/// schematic synchronization would otherwise maintain.
fn author_amp_symbol(state: &mut AppState, schematic: &CellViewRef, symbol: &CellViewRef) {
    let ports = state
        .workspace
        .schematic_buffers
        .get(&schematic.key())
        .expect("the amp schematic was saved to the workspace")
        .interface_ports();
    let document = SymbolDocument::generated_from_ports(&ports);
    let mut metadata = SymbolEditorMetadata::for_document(&document);
    metadata.texts.push(SymbolTextObject {
        id: metadata.next_text_id,
        text: AMP_CELL.to_ascii_uppercase(),
        position: Point::origin(),
        shown: true,
    });
    metadata.next_text_id = metadata.next_text_id.saturating_add(1);

    state.open_workspace_view(symbol.clone());
    state
        .store_active_symbol_editor_bundle(&document, &metadata)
        .expect("the authored amp symbol publishes");
}

/// Draw a trivial RC between ports `A` and `B`, returning the capacitor to the
/// cell's own ground reference.
fn author_filter(
    state: &mut AppState,
    reference: &CellViewRef,
    resistance: &str,
    capacitance: &str,
) {
    state.open_workspace_view(reference.clone());
    let schematic = &mut state.schematic;

    place_port(
        schematic,
        Point::new(10, 0),
        FILTER_PORTS[0],
        PortDirectionType::InputAnalog,
    );
    add_wire(schematic, Point::new(0, 0), Point::new(40, 0));
    let resistor = schematic.add_component(ComponentType::Resistor, Point::new(60, 0));
    set_value(schematic, resistor, resistance);
    add_wire(schematic, Point::new(80, 0), Point::new(120, 0));
    place_port(
        schematic,
        Point::new(130, 0),
        FILTER_PORTS[1],
        PortDirectionType::OutputAnalog,
    );

    let capacitor = schematic.add_component(ComponentType::Capacitor, Point::new(100, 60));
    set_value(schematic, capacitor, capacitance);
    add_wire(schematic, Point::new(80, 60), Point::new(80, 0));
    schematic.add_component(ComponentType::Ground, Point::new(130, 70));
    add_wire(schematic, Point::new(120, 60), Point::new(130, 60));

    state.sync_active_schematic_to_workspace();
}

/// Draw the testbench: four instances across two libraries, the stimulus, the
/// typed bus, and the global supply.
fn author_top(state: &mut AppState, reference: &CellViewRef) {
    state.open_workspace_view(reference.clone());
    let amp_ports = interface_of(state, USER_LIBRARY, AMP_CELL);
    let user_filter_ports = interface_of(state, USER_LIBRARY, FILTER_CELL);
    let vendor_filter_ports = interface_of(state, VENDOR_LIBRARY, FILTER_CELL);
    let schematic = &mut state.schematic;

    let source = schematic.add_component(ComponentType::VoltageSource, Point::new(100, 200));
    set_value(schematic, source, "1");
    stub_with_label(schematic, Point::new(100, 200), Point::new(100, 180), "in");
    stub_with_label(schematic, Point::new(100, 200), Point::new(100, 220), "gnd");
    schematic.add_component(ComponentType::Ground, Point::new(100, 300));
    stub_with_label(schematic, Point::new(100, 300), Point::new(100, 290), "gnd");

    place_instance(
        schematic,
        AMP_INSTANCES[0],
        Point::new(300, 200),
        LibraryCellInstance::new(USER_LIBRARY, AMP_CELL, SCHEMATIC_VIEW),
        &amp_ports,
        &["in", "stage1", GLOBAL_SUPPLY_NET, "gnd"],
    );
    place_instance(
        schematic,
        AMP_INSTANCES[1],
        Point::new(300, 400),
        LibraryCellInstance::new(USER_LIBRARY, AMP_CELL, SCHEMATIC_VIEW),
        &amp_ports,
        &["stage1", "stage2", GLOBAL_SUPPLY_NET, "gnd"],
    );
    place_instance(
        schematic,
        FILTER_INSTANCES[0],
        Point::new(600, 200),
        LibraryCellInstance::new(USER_LIBRARY, FILTER_CELL, SCHEMATIC_VIEW),
        &user_filter_ports,
        &["stage2", "stage3"],
    );
    place_instance(
        schematic,
        FILTER_INSTANCES[1],
        Point::new(600, 400),
        LibraryCellInstance::new(VENDOR_LIBRARY, FILTER_CELL, SCHEMATIC_VIEW),
        &vendor_filter_ports,
        &["stage3", TOP_OUTPUT_NET],
    );

    author_data_bus(schematic);
    state.sync_active_schematic_to_workspace();
}

/// A declared four-bit bus with one scalar member ripped out to an ordinary
/// wire, and that wire named.
fn author_data_bus(schematic: &mut SchematicState) {
    let declaration = BusDeclaration::new(DATA_BUS, 3, 0, BusNotation::Angle)
        .expect("a four-bit descending bus declaration is valid");
    let bus = schematic
        .add_bus(
            vec![Point::new(900, 100), Point::new(900, 300)],
            Some(declaration),
        )
        .expect("a declared bus polyline commits");
    let member = BusSlice::new(DATA_BUS, 0, 0, BusNotation::Angle)
        .expect("a scalar member selector is valid");
    let tap_point = Point::new(940, 200);
    add_wire(schematic, tap_point, Point::new(980, 200));
    schematic
        .place_bus_tap(
            bus,
            Point::new(900, 200),
            tap_point,
            member,
            BusTapOrientation::Right,
        )
        .expect("a scalar tap terminating on a wire commits");
    schematic.add_net_label(Point::new(980, 200), DATA_BUS_MEMBER.to_owned());
}

/// Place one bound instance and wire every terminal to a named net.
fn place_instance(
    schematic: &mut SchematicState,
    name: &str,
    position: Point,
    mut binding: LibraryCellInstance,
    ports: &[PortSpec],
    nets: &[&str],
) {
    binding.bind_interface(ports);
    let id = schematic.add_library_cell_component(position, binding);
    let component = schematic
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("the placed instance is retained");
    component.name = name.to_owned();
    let terminals = component
        .terminal_positions()
        .into_iter()
        .map(|(_, point)| point)
        .collect::<Vec<_>>();
    assert_eq!(
        terminals.len(),
        nets.len(),
        "{name} presents {} terminals but {} nets were named",
        terminals.len(),
        nets.len()
    );
    for (terminal, net) in terminals.into_iter().zip(nets) {
        stub_with_label(schematic, position, terminal, net);
    }
}

/// Run a stub outward from a terminal and name the net at its far end.
///
/// Outward is decided by the terminal's own offset from the symbol origin. A
/// stub that ran the other way would cross the body and could land on the
/// neighbouring pin, which is one pin pitch away.
fn stub_with_label(schematic: &mut SchematicState, origin: Point, terminal: Point, net: &str) {
    let offset_x = terminal.x - origin.x;
    let offset_y = terminal.y - origin.y;
    let end = if offset_x.abs() >= offset_y.abs() {
        Point::new(terminal.x + offset_x.signum() * STUB_LENGTH, terminal.y)
    } else {
        Point::new(terminal.x, terminal.y + offset_y.signum() * STUB_LENGTH)
    };
    add_wire(schematic, terminal, end);
    schematic.add_net_label(end, net.to_owned());
}

/// The active configuration: `top`'s schematic as the executable root, `X1` as
/// the device under test, and one path-scoped override on `X2`.
fn install_configuration(state: &mut AppState, top: &CellViewRef) -> ConfigurationSetId {
    let mut candidate = state.workspace.configuration_sets.clone();
    let configuration = candidate
        .create(ConfigurationSetDefinition {
            name: "Reference hierarchy".to_owned(),
            root: top.clone(),
            dut_path: HierarchyReference::instance_path(AMP_INSTANCES[0]),
            executable_view_policy: vec![SCHEMATIC_VIEW.to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: vec![ConfigurationSetOverride {
                instance_path: HierarchyReference::instance_path(AMP_INSTANCES[1]),
                executable_views: vec![SCHEMATIC_VIEW.to_owned()],
                stop_view: None,
                model_section: None,
                eligible_platforms: vec![
                    ConfigurationPlatform::Desktop,
                    ConfigurationPlatform::Browser,
                ],
            }],
            model_profile: ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Hierarchy reference".to_owned(),
        })
        .expect("the reference configuration is well formed");
    state
        .workspace
        .replace_configuration_sets(candidate)
        .expect("the reviewed configuration catalog publishes");
    configuration
}

/// One plan-owned saved output on the testbench's labelled output net.
fn install_saved_output(state: &mut AppState) -> SimulationPlanId {
    let plan = state
        .sim_setup
        .stable_analysis_plan()
        .expect("the bootstrapped session owns a stable plan")
        .id();
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        TOP_OUTPUT_NET,
        SAVED_OUTPUT_EXPRESSION,
        SavedOutputCompatibility::OpTranAc,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .expect("a raw node probe is a valid saved output");
    state
        .workspace
        .add_saved_output(plan, output)
        .expect("the plan accepts its first saved output");
    plan
}

fn place_port(
    schematic: &mut SchematicState,
    position: Point,
    name: &str,
    direction: PortDirectionType,
) -> u64 {
    let pending = PendingPortPlacement::new(
        name,
        direction,
        PortDiscipline::Electrical,
        schematic.topology_version(),
        schematic.next_interface_order(),
    );
    schematic
        .place_pending_port(position, pending)
        .unwrap_or_else(|error| panic!("interface port {name} places: {error}"))
}

fn add_wire(schematic: &mut SchematicState, start: Point, end: Point) -> u64 {
    schematic
        .add_wire(vec![start, end])
        .expect("a two-point wire commits")
}

fn set_value(schematic: &mut SchematicState, component: u64, value: &str) {
    schematic
        .components
        .iter_mut()
        .find(|candidate| candidate.id == component)
        .expect("the placed component is retained")
        .value = value.to_owned();
}

fn interface_of(state: &AppState, library: &str, cell: &str) -> Vec<PortSpec> {
    let reference = CellViewRef::new(library, cell, SCHEMATIC_VIEW);
    state
        .workspace
        .schematic_buffers
        .get(&reference.key())
        .expect("the master schematic was saved to the workspace")
        .interface_ports()
}

/// Every stable object identity a schematic owns, which is what governed sheet
/// membership is keyed on.
fn live_object_ids(schematic: &SchematicState) -> Vec<u64> {
    schematic
        .components
        .iter()
        .map(|object| object.id)
        .chain(schematic.wires.iter().map(|object| object.id))
        .chain(schematic.buses.iter().map(|object| object.id))
        .chain(schematic.bus_taps.iter().map(|object| object.id))
        .chain(schematic.junctions.iter().map(|object| object.id))
        .chain(schematic.net_labels.iter().map(|object| object.id))
        .collect()
}

mod tests;
