//! A two-sheet cell view at production scale: the fixture the canvas's scene,
//! drag and raster tests build on.

use std::collections::HashMap;

use crate::state::{
    AnalysisResult, AnalysisType, Cell, CellViewRef, Component, ComponentType, DcOpResult,
    HierarchyMapRow, Junction, Library, LibraryCellInstance, OperatingPointValue, Point,
    PortDirection, PortSpec, SchematicAnnotationVisibility, SimulationRun, SymbolDocument,
    SymbolPin, SymbolShape, View, ViewType, Wire,
};

use super::AppState;

/// Components per sheet, laid out as a lattice of this many columns.
pub(super) const COLUMNS: usize = 40;
pub(super) const ROWS: usize = 25;
/// Lattice pitch in world units.
const PITCH: i32 = 200;
const ORIGIN: i32 = 400;
/// Every tenth component is a hierarchical instance.
const INSTANCE_STRIDE: usize = 10;
const MASTERS: [&str; 3] = ["amp", "buf", "bias"];
const OPERATING_POINT_NODES: usize = 300;

/// What the scale fixture holds, so a probe or a test can address it.
pub(super) struct ScaleFixture {
    pub(super) state: AppState,
    /// Object ids of each sheet's components, in lattice order.
    pub(super) components: [Vec<u64>; 2],
    /// How many objects each sheet holds: components, wires and junctions.
    pub(super) objects: [usize; 2],
}

/// Everything one sheet draws, before the sheet catalog assigns it.
#[derive(Default)]
struct SheetContent {
    components: Vec<Component>,
    wires: Vec<Wire>,
    junctions: Vec<Junction>,
    routed_wires: Vec<u64>,
}

impl SheetContent {
    fn ids(&self) -> impl Iterator<Item = u64> + '_ {
        self.components
            .iter()
            .map(|component| component.id)
            .chain(self.wires.iter().map(|wire| wire.id))
            .chain(self.junctions.iter().map(|junction| junction.id))
    }
}

fn port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
    }
}

pub(super) fn master_ports() -> [PortSpec; 3] {
    [
        port("IN", PortDirection::In),
        port("OUT", PortDirection::Out),
        port("BIAS", PortDirection::In),
    ]
}

/// One library with the three masters every instance binds to, each with an
/// authored symbol the canvas resolves: `amp`, `buf` and `bias`, with `IN`
/// at `(-w, 0)`, `OUT` at `(w, 0)` and `BIAS` at `(0, 40)`, where `w` is 40,
/// 50 and 60.
pub(super) fn master_library() -> Library {
    let mut library = Library::new("work");
    for (ordinal, master) in MASTERS.iter().enumerate() {
        let mut cell = Cell::new(*master);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        let width = 40 + 10 * i32::try_from(ordinal).expect("three masters");
        SymbolDocument {
            pins: vec![
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-width, 0))),
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(width, 0))),
                SymbolPin::new("BIAS", PortDirection::In, Some(Point::new(0, 40))),
            ],
            body: vec![
                SymbolShape::Polyline {
                    points: vec![
                        Point::new(-width + 10, -30),
                        Point::new(width - 10, -30),
                        Point::new(width - 10, 30),
                        Point::new(-width + 10, 30),
                    ],
                    closed: true,
                },
                SymbolShape::Polyline {
                    points: vec![Point::new(-width, 0), Point::new(-width + 10, 0)],
                    closed: false,
                },
                SymbolShape::Polyline {
                    points: vec![Point::new(width - 10, 0), Point::new(width, 0)],
                    closed: false,
                },
                SymbolShape::Polyline {
                    points: vec![Point::new(0, 30), Point::new(0, 40)],
                    closed: false,
                },
            ],
            ..SymbolDocument::default()
        }
        .store_in_view(&mut symbol_view)
        .expect("master symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
    }
    library
}

/// One sheet's lattice of `columns` by `rows` components, the conductors
/// routed between them, and a junction wherever a stub tees into a rail.
fn sheet_content(next_id: &mut u64, sheet: usize, columns: usize, rows: usize) -> SheetContent {
    let mut allocate = || {
        let id = *next_id;
        *next_id += 1;
        id
    };
    let mut content = SheetContent::default();
    let kinds = [
        ComponentType::Resistor,
        ComponentType::Capacitor,
        ComponentType::Nmos,
        ComponentType::Pmos,
    ];
    let lattice = |column: usize, row: usize| {
        Point::new(
            ORIGIN + PITCH * i32::try_from(column).expect("lattice column"),
            ORIGIN + PITCH * i32::try_from(row).expect("lattice row"),
        )
    };
    // The second sheet is drawn a little lower than the first. Two sheets may
    // share coordinates, but loading a document folds junctions that share a
    // position, so a fixture that overlapped them exactly would lose half.
    let sheet_shift = Point::new(0, 70 * i32::try_from(sheet).expect("two sheets"));
    for row in 0..rows {
        for column in 0..columns {
            let ordinal = row * columns + column;
            let id = allocate();
            let at = lattice(column, row);
            let position = Point::new(at.x + sheet_shift.x, at.y + sheet_shift.y);
            let component = if ordinal % INSTANCE_STRIDE == INSTANCE_STRIDE - 1 {
                let master = MASTERS[(ordinal / INSTANCE_STRIDE) % MASTERS.len()];
                let mut binding = LibraryCellInstance::new("work", master, "schematic");
                binding.bind_interface(&master_ports());
                let mut component = Component::new(id, ComponentType::CellInstance, position);
                component.name = format!("X{}_{ordinal}", sheet + 1);
                component.library_cell = Some(binding);
                component
            } else {
                let kind = kinds[ordinal % kinds.len()];
                let mut component = Component::new(id, kind, position);
                component.name = format!("{}{}_{ordinal}", kind.spice_prefix(), sheet + 1);
                component
            };
            content.components.push(component);
        }
    }

    let terminals = content
        .components
        .iter()
        .map(|component| {
            component
                .terminal_positions()
                .into_iter()
                .map(|(_, point)| point)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for row in 0..rows {
        let rail_y = lattice(0, row).y + sheet_shift.y + 100;
        for column in 0..columns {
            let ordinal = row * columns + column;
            let pins = &terminals[ordinal];
            let last = *pins.last().expect("every component has a terminal");
            let second = pins.get(1).copied().unwrap_or(last);
            // A routed conductor from this part to its right-hand neighbour.
            if column + 1 < columns {
                let next = terminals[ordinal + 1][0];
                let middle = (second.x + next.x) / 2 / 10 * 10;
                let points = if second.y == next.y {
                    vec![second, Point::new(middle, second.y), next]
                } else {
                    vec![
                        second,
                        Point::new(middle, second.y),
                        Point::new(middle, next.y),
                        next,
                    ]
                };
                let id = allocate();
                content.routed_wires.push(id);
                content.wires.push(Wire::new(id, points));
            }
            // A stub from the part's last terminal down into its row's rail,
            // with the junction where it tees in.
            let tee = Point::new(last.x, rail_y);
            content.wires.push(Wire::new(allocate(), vec![last, tee]));
            content.junctions.push(Junction::new(allocate(), tee));
        }
        // Rails under each pair of columns, and one return rail per row.
        for pair in 0..columns / 2 {
            let left = lattice(pair * 2, row).x - 100;
            let right = lattice(pair * 2 + 1, row).x + 100;
            content.wires.push(Wire::new(
                allocate(),
                vec![Point::new(left, rail_y), Point::new(right, rail_y)],
            ));
        }
        let return_y = rail_y + 30;
        content.wires.push(Wire::new(
            allocate(),
            vec![
                Point::new(lattice(0, row).x - 100, return_y),
                Point::new(lattice(columns - 1, row).x + 100, return_y),
            ],
        ));
    }
    content
}

/// The run's cross-probe map as the netlister hands it over today: one net
/// per routed conductor group, every point read off the projected document,
/// which lays the sheet at catalog index `k` at `k * 1_000_000` in x.
fn cross_probe_nets(sheets: &[SheetContent; 2]) -> HashMap<String, Vec<Point>> {
    let mut nets: HashMap<String, Vec<Point>> = HashMap::new();
    let mut ordinal = 0_usize;
    for (sheet, content) in sheets.iter().enumerate() {
        let offset = 1_000_000 * i32::try_from(sheet).expect("two sheets");
        for wire in &content.wires {
            if !content.routed_wires.contains(&wire.id) {
                continue;
            }
            let net = format!("N{}", ordinal % OPERATING_POINT_NODES);
            ordinal += 1;
            nets.entry(net).or_default().extend(
                wire.points
                    .iter()
                    .map(|point| Point::new(point.x + offset, point.y)),
            );
        }
    }
    nets
}

/// Install a run's net geometry the way a run hands it over: `nets` holds
/// every point as the netlister read it off the projected document, where the
/// sheet at catalog index `k` lies `k * 1_000_000` along x, and the snapshot
/// takes each point home to its sheet.
pub(super) fn install_projected_run_geometry(
    state: &mut AppState,
    nets: HashMap<String, Vec<Point>>,
) {
    let point_to_net = nets
        .iter()
        .flat_map(|(net, points)| points.iter().map(move |point| (*point, net.clone())))
        .collect();
    let sheets = rspice_simulation::netlist_gen::net_geometry_by_sheet(
        super::sheet_visibility::active_sheet_catalog(state),
        point_to_net,
        nets,
        HashMap::new(),
    );
    state.model.simulation.source.cross_probe.install(
        state.model.workspace.content.active_view.clone(),
        sheets,
        state.model.schematic.topology_version(),
    );
}

/// The smallest drawing on which a run's geometry can land on the wrong
/// sheet: one wire on each of two sheets at the same drawn coordinates, the
/// first on net `VOUT` and the second on net `BIAS`, and a solved operating
/// point naming both. `second_sheet_active` opens Sheet 2.
pub(super) fn two_sheet_operating_point(second_sheet_active: bool) -> AppState {
    let mut state = super::sheet_visibility::two_sheet_state(&[1], &[2]);
    let (start, end) = (Point::new(100, 0), Point::new(200, 0));
    let document = state.model.schematic.document_mut_for_test();
    document.wires.push(Wire::segment(1, start, end));
    document.wires.push(Wire::segment(2, start, end));
    state.model.schematic.bump_topology_version();
    let projected = |point: Point| Point::new(point.x + 1_000_000, point.y);
    install_projected_run_geometry(
        &mut state,
        HashMap::from([
            ("VOUT".to_owned(), vec![start, end]),
            ("BIAS".to_owned(), vec![projected(start), projected(end)]),
        ]),
    );
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_dc_op(DcOpResult {
            node_voltages: [("V(vout)", 2.5), ("V(bias)", 0.7)]
                .into_iter()
                .map(|(name, value)| OperatingPointValue {
                    name: name.to_owned(),
                    value,
                    unit: "V".to_owned(),
                })
                .collect(),
            branch_currents: Vec::new(),
            power_dissipation: Vec::new(),
        }),
    );
    state.model.simulation.retained.runs.insert(0, run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state.design.schematic_visibility.annotations = SchematicAnnotationVisibility::OperatingPoint;
    if second_sheet_active {
        let key = state
            .model
            .workspace
            .content
            .active_schematic_reference()
            .key();
        let catalog = state
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog_mut(&key)
            .expect("two sheets");
        let second = catalog.sheets()[1].id();
        catalog.set_active(second).expect("sheet 2 opens");
    }
    state
}

/// A prepared run that emitted every hierarchical instance, with a solved
/// operating point of [`OPERATING_POINT_NODES`] nodes.
fn operating_point_run(instances: &[(String, &'static str)]) -> SimulationRun {
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
    use crate::state::{
        AnalysisResultSourceDomain, PreparedRunReceipt, PreparedRunReceiptInput,
        PreparedRunTaskReceipt, PreparedSourceCheckReceipt,
    };

    let digest = |byte: u8| ContentDigest::from_bytes([byte; 32]);
    let receipt = PreparedRunReceipt::new(PreparedRunReceiptInput {
        source_domain: AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(SimulationPlanId::new()),
        project_revision: ObjectRevision::INITIAL,
        prepared_snapshot_digest: digest(0x31),
        source_content_digest: digest(0x32),
        source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x33)),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![
            PreparedRunTaskReceipt::new(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                Vec::new(),
                0,
                digest(0x34),
            )
            .expect("valid task receipt"),
        ],
    })
    .expect("valid plan receipt")
    .with_hierarchy_map(
        instances
            .iter()
            .map(|(name, master)| {
                HierarchyMapRow::new(
                    &format!("/{name}"),
                    *master,
                    name.clone(),
                    CellViewRef::new("work", *master, "schematic"),
                )
                .expect("hierarchy row")
            })
            .collect(),
    )
    .expect("distinct occurrences seal");

    let mut run = SimulationRun::new_prepared(1, receipt);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_dc_op(DcOpResult {
            node_voltages: (0..OPERATING_POINT_NODES)
                .map(|node| OperatingPointValue {
                    name: format!("V(n{node})"),
                    value: 0.01 * node as f64,
                    unit: "V".to_owned(),
                })
                .collect(),
            branch_currents: Vec::new(),
            power_dissipation: Vec::new(),
        }),
    );
    run
}

/// A two-sheet cell view: 2,000 components (R, C, NMOS, PMOS and 200
/// instances of three masters), 5,000 wires, 2,000 junctions, and a prepared
/// operating point of 300 nodes. Sheet 1 is active. Deterministic: the same
/// ids and coordinates every call.
pub(super) fn scale_fixture() -> ScaleFixture {
    scale_fixture_of(COLUMNS, ROWS)
}

/// [`scale_fixture`] with each sheet's lattice `columns` by `rows`
/// components rather than 40 by 25.
pub(super) fn scale_fixture_of(columns: usize, rows: usize) -> ScaleFixture {
    let mut next_id = 1_u64;
    let sheets = [
        sheet_content(&mut next_id, 0, columns, rows),
        sheet_content(&mut next_id, 1, columns, rows),
    ];
    let first = sheets[0].ids().collect::<Vec<_>>();
    let second = sheets[1].ids().collect::<Vec<_>>();
    let mut state = super::sheet_visibility::two_sheet_state(&first, &second);
    state.model.library_manager.add_library(master_library());

    let nets = cross_probe_nets(&sheets);
    let instances = sheets
        .iter()
        .flat_map(|content| &content.components)
        .filter_map(|component| {
            let master = component.library_cell.as_ref()?;
            let master = MASTERS
                .iter()
                .copied()
                .find(|candidate| *candidate == master.cell)?;
            Some((component.name.clone(), master))
        })
        .collect::<Vec<_>>();
    let components = [
        sheets[0].components.iter().map(|c| c.id).collect(),
        sheets[1].components.iter().map(|c| c.id).collect(),
    ];

    let document = state.model.schematic.document_mut_for_test();
    for content in sheets {
        document.components.extend(content.components);
        document.wires.extend(content.wires);
        document.junctions.extend(content.junctions);
    }
    state.model.schematic.recalculate_runtime_state();
    state.model.schematic.bump_topology_version();
    state.sync_active_schematic_to_workspace();

    install_projected_run_geometry(&mut state, nets);
    state
        .model
        .simulation
        .retained
        .runs
        .insert(0, operating_point_run(&instances));
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);

    ScaleFixture {
        state,
        components,
        objects: [first.len(), second.len()],
    }
}

#[test]
fn the_scale_fixture_holds_the_stated_population() {
    let fixture = scale_fixture();
    let document = fixture.state.model.schematic.document();
    assert_eq!(document.components.len(), 2_000);
    assert_eq!(document.wires.len(), 5_000);
    assert_eq!(document.junctions.len(), 2_000);
    assert_eq!(
        document
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
            .count(),
        200
    );
    let symbols = super::schematic_symbol_context(&fixture.state);
    assert!(
        document
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
            .all(|component| symbols.resolved_symbol(component).is_some()),
        "every instance resolves its master's authored symbol"
    );
    let catalog = super::sheet_visibility::multi_sheet_catalog(&fixture.state)
        .expect("the fixture has two sheets");
    assert_eq!(catalog.sheets().len(), 2);
    assert!(
        fixture.components[0]
            .iter()
            .all(|id| super::sheet_visibility::object_is_on_active_sheet(&fixture.state, *id))
    );
    assert!(
        fixture.components[1]
            .iter()
            .all(|id| !super::sheet_visibility::object_is_on_active_sheet(&fixture.state, *id))
    );
    let analysis = fixture
        .state
        .model
        .simulation
        .active_analysis()
        .expect("OP");
    assert_eq!(
        analysis.dc_op.as_ref().expect("dc op").node_voltages.len(),
        OPERATING_POINT_NODES
    );
}
