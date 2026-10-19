//! A two-sheet cell view at production scale, and the frame-cost probe that
//! times the canvas on it.
//!
//! The probe is the reproduction for every per-frame cost on the canvas. It
//! runs whole frames through [`super::render_schematic_view`], so whatever a
//! frame recomputes is in the number, not just the function someone suspected.
//! Run it in release; debug numbers are an order of magnitude off:
//!
//! `cargo test -p rspice-app --lib --release -- --ignored --exact
//! schematic::view::frame_cost::canvas_frame_cost_at_scale --nocapture`
//!
//! The browser build is held to the same budgets. Its shipping profile is
//! `web-release`, and the probe runs there under Node through
//! `wasm-bindgen-test-runner`:
//!
//! `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner
//! cargo test -p rspice-app --lib --profile web-release
//! --target wasm32-unknown-unknown -- schematic::view::frame_cost
//! --include-ignored --nocapture`

use std::collections::HashMap;

use crate::state::{
    AnalysisResult, AnalysisType, Cell, CellViewRef, Component, ComponentType, DcOpResult,
    HierarchyMapRow, Junction, Library, LibraryCellInstance, OperatingPointValue, Point,
    PortDirection, PortSpec, SchematicAnnotationVisibility, SimulationRun, SymbolDocument,
    SymbolPin, SymbolShape, Tool, View, ViewType, Wire,
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
    /// Each sheet's routed (three- or four-point) wires.
    pub(super) routed_wires: [Vec<u64>; 2],
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
    state.simulation.source.cross_probe.install(
        state.workspace.content.active_view.clone(),
        sheets,
        state.schematic.topology_version(),
    );
}

/// The smallest drawing on which a run's geometry can land on the wrong
/// sheet: one wire on each of two sheets at the same drawn coordinates, the
/// first on net `VOUT` and the second on net `BIAS`, and a solved operating
/// point naming both. `second_sheet_active` opens Sheet 2.
pub(super) fn two_sheet_operating_point(second_sheet_active: bool) -> AppState {
    let mut state = super::sheet_visibility::two_sheet_state(&[1], &[2]);
    let (start, end) = (Point::new(100, 0), Point::new(200, 0));
    let document = state.schematic.document_mut_for_test();
    document.wires.push(Wire::segment(1, start, end));
    document.wires.push(Wire::segment(2, start, end));
    state.schematic.bump_topology_version();
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
    state.simulation.retained.runs.insert(0, run);
    state.simulation.view.active_run_idx = Some(0);
    state.simulation.view.active_analysis_idx = Some(0);
    state.ui.schematic_visibility.annotations = SchematicAnnotationVisibility::OperatingPoint;
    if second_sheet_active {
        let key = state.workspace.content.active_schematic_reference().key();
        let catalog = state
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
    state.library_manager.add_library(master_library());

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
    let routed_wires = [
        sheets[0].routed_wires.clone(),
        sheets[1].routed_wires.clone(),
    ];

    let document = state.schematic.document_mut_for_test();
    for content in sheets {
        document.components.extend(content.components);
        document.wires.extend(content.wires);
        document.junctions.extend(content.junctions);
    }
    state.schematic.recalculate_runtime_state();
    state.schematic.bump_topology_version();
    state.sync_active_schematic_to_workspace();

    install_projected_run_geometry(&mut state, nets);
    state
        .simulation
        .retained
        .runs
        .insert(0, operating_point_run(&instances));
    state.simulation.view.active_run_idx = Some(0);
    state.simulation.view.active_analysis_idx = Some(0);

    ScaleFixture {
        state,
        components,
        routed_wires,
        objects: [first.len(), second.len()],
    }
}

#[test]
fn the_scale_fixture_holds_the_stated_population() {
    let fixture = scale_fixture();
    let document = fixture.state.schematic.document();
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
    let analysis = fixture.state.simulation.active_analysis().expect("OP");
    assert_eq!(
        analysis.dc_op.as_ref().expect("dc op").node_voltages.len(),
        OPERATING_POINT_NODES
    );
}

// ---------------------------------------------------------------------------
// The probe
// ---------------------------------------------------------------------------

const SCREEN: egui::Vec2 = egui::vec2(1600.0, 1000.0);
const WARM_UP_FRAMES: usize = 2;
const TIMED_FRAMES: usize = 10;

/// One timed scenario's frames, in milliseconds.
struct FrameTimes {
    name: &'static str,
    budget_ms: f64,
    frames_ms: Vec<f64>,
    /// Whether every frame is held to the budget rather than the median: a
    /// gesture's frames each make the user wait.
    every_frame: bool,
}

impl FrameTimes {
    fn median(&self) -> f64 {
        let mut sorted = self.frames_ms.clone();
        sorted.sort_by(f64::total_cmp);
        sorted[sorted.len() / 2]
    }

    fn max(&self) -> f64 {
        self.frames_ms.iter().copied().fold(0.0, f64::max)
    }

    /// The quietest frame: on a machine that is also compiling, the closest
    /// reading of what the frame itself costs.
    fn min(&self) -> f64 {
        self.frames_ms.iter().copied().fold(f64::INFINITY, f64::min)
    }
}

/// Run one whole canvas frame with the pointer at `pointer`, returning the
/// frame's wall time and how many shapes it painted.
fn run_frame(
    ctx: &egui::Context,
    state: &mut AppState,
    symbols: Option<&super::super::symbols::SymbolLibrary>,
    pointer: egui::Pos2,
) -> (f64, usize) {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN)),
        events: vec![egui::Event::PointerMoved(pointer)],
        ..Default::default()
    };
    let started = crate::time_compat::Instant::now();
    let output = ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| super::render_schematic_view(ui, state, symbols));
    });
    let elapsed = started.elapsed().as_secs_f64() * 1_000.0;
    (elapsed, std::hint::black_box(output.shapes.len()))
}

/// Time `TIMED_FRAMES` hover frames over a fitted sheet, after warming up.
fn time_scenario(
    name: &'static str,
    budget_ms: f64,
    state: AppState,
    verify: impl FnOnce(&AppState),
) -> FrameTimes {
    time_scenario_in(name, budget_ms, state, None, verify)
}

/// [`time_scenario`], over the sheet fitted to the screen or, given a zoom
/// and a world point, at that zoom centred on that point.
fn time_scenario_in(
    name: &'static str,
    budget_ms: f64,
    mut state: AppState,
    view: Option<(f64, Point)>,
    verify: impl FnOnce(&AppState),
) -> FrameTimes {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let symbols = super::super::symbols::SymbolLibrary::load_embedded().ok();
    let editor = &mut state.schematic.session.editor;
    match view {
        None => editor.needs_fit = true,
        Some((zoom, centre)) => {
            editor.needs_fit = false;
            editor.zoom = zoom;
            editor.pan = (
                f64::from(SCREEN.x) * 0.5 - f64::from(centre.x) * zoom,
                f64::from(SCREEN.y) * 0.5 - f64::from(centre.y) * zoom,
            );
        }
    }
    let centre = egui::pos2(SCREEN.x * 0.5, SCREEN.y * 0.5);
    let mut painted = 0;
    for frame in 0..WARM_UP_FRAMES {
        let nudge = if frame % 2 == 0 { 0.0 } else { 1.0 };
        let (_, shapes) = run_frame(
            &ctx,
            &mut state,
            symbols.as_ref(),
            centre + egui::vec2(nudge, 0.0),
        );
        painted += shapes;
    }
    let mut frames_ms = Vec::with_capacity(TIMED_FRAMES);
    for frame in 0..TIMED_FRAMES {
        let nudge = if frame % 2 == 0 { 1.0 } else { 0.0 };
        let (elapsed, shapes) = run_frame(
            &ctx,
            &mut state,
            symbols.as_ref(),
            centre + egui::vec2(nudge, 0.0),
        );
        painted += shapes;
        frames_ms.push(elapsed);
    }
    assert!(painted > 0, "{name}: the frames painted nothing");
    verify(&state);
    FrameTimes {
        name,
        budget_ms,
        frames_ms,
        every_frame: false,
    }
}

/// The fixture with its operating-point overlay hidden, so each editing
/// scenario is timed on its own; the overlay is a scenario of its own.
fn fixture_without_overlay() -> ScaleFixture {
    let mut fixture = scale_fixture();
    fixture.state.ui.schematic_visibility.annotations = SchematicAnnotationVisibility::Hidden;
    fixture
}

fn idle_hover() -> FrameTimes {
    time_scenario(
        "idle hover (Select)",
        4.0,
        fixture_without_overlay().state,
        |_| {},
    )
}

fn move_preview() -> FrameTimes {
    let mut fixture = fixture_without_overlay();
    let state = &mut fixture.state;
    for id in fixture.components[0].iter().take(40) {
        state
            .schematic
            .session
            .editor
            .selection
            .select_component(*id);
    }
    crate::workbench::app::arm_move_selection(state);
    assert!(state.dialogs.move_selection.armed, "the move arms");
    state.dialogs.move_selection.canvas.preview_delta = Point::new(40, 0);
    time_scenario("move preview, delta (40, 0)", 8.0, fixture.state, |state| {
        assert!(state.dialogs.move_selection.armed);
        assert_eq!(state.dialogs.move_selection.canvas.preview_error, None);
    })
}

fn junction_hover() -> FrameTimes {
    let mut state = fixture_without_overlay().state;
    state.schematic.session.editor.tool = Tool::Junction;
    time_scenario("junction tool hover", 8.0, state, |state| {
        assert_eq!(state.schematic.session.editor.tool, Tool::Junction);
    })
}

fn stretch_preview() -> FrameTimes {
    let mut fixture = fixture_without_overlay();
    let state = &mut fixture.state;
    let wire = fixture.routed_wires[0]
        .iter()
        .copied()
        .find(|id| {
            state
                .schematic
                .document()
                .wires
                .iter()
                .any(|wire| wire.id == *id && wire.points.len() == 4)
        })
        .expect("a four-point routed wire");
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_wire_segment(wire, 1);
    crate::workbench::app::arm_stretch_selection(state);
    assert!(state.dialogs.stretch_selection.armed, "the stretch arms");
    state.dialogs.stretch_selection.canvas.gesture.preview_delta = Point::new(20, 0);
    time_scenario(
        "stretch preview, delta (20, 0)",
        8.0,
        fixture.state,
        |state| {
            assert!(state.dialogs.stretch_selection.armed);
            assert_eq!(
                state.dialogs.stretch_selection.canvas.gesture.preview_error,
                None
            );
        },
    )
}

fn operating_point_overlay() -> FrameTimes {
    let mut state = scale_fixture().state;
    state.ui.schematic_visibility.annotations = SchematicAnnotationVisibility::OperatingPoint;
    time_scenario("operating-point overlay", 8.0, state, |state| {
        assert!(
            !super::scene::operating_point_annotations(state).is_empty(),
            "the overlay annotates the active sheet"
        );
    })
}

/// The overlay at 100 %, where the badges are legible and so are placed:
/// fitted, the whole sheet is too small for them and they hide.
fn operating_point_overlay_legible() -> FrameTimes {
    let mut state = scale_fixture().state;
    state.ui.schematic_visibility.annotations = SchematicAnnotationVisibility::OperatingPoint;
    time_scenario_in(
        "operating-point overlay, 100 %",
        8.0,
        state,
        Some((1.0, Point::new(ORIGIN + PITCH * 4, ORIGIN + PITCH * 3))),
        |state| {
            assert!(
                !super::scene::operating_point_annotations(state).is_empty(),
                "the overlay annotates the active sheet"
            );
            assert!(
                (state.schematic.session.editor.zoom - 1.0).abs() < f64::EPSILON,
                "the view stays at 100 %"
            );
        },
    )
}

/// A part dragged across the sheet at 100 %, one grid step a frame, each
/// frame timed: the press and every move, then the drop on its own. Each move
/// changes the drag's delta, so each one moves the part, rubber-bands its
/// wires and re-judges the open pins. The drop is the one frame that commits
/// the edit: it records the undo step, cleans the sheet's wires up and syncs
/// the workspace, so it is held to one display frame rather than to the move
/// budget.
fn part_drag() -> [FrameTimes; 2] {
    let fixture = fixture_without_overlay();
    let mut state = fixture.state;
    // A resistor in the middle of the open sheet's lattice.
    let part = fixture.components[0][COLUMNS * (ROWS / 2) + COLUMNS / 2];
    let at = state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == part)
        .map(|component| component.pos)
        .expect("the part is on the sheet");
    let grid = state.schematic.document().grid_size;
    let editor = &mut state.schematic.session.editor;
    editor.needs_fit = false;
    editor.zoom = 1.0;
    editor.pan = (
        f64::from(SCREEN.x) * 0.5 - f64::from(at.x),
        f64::from(SCREEN.y) * 0.5 - f64::from(at.y),
    );
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let symbols = super::super::symbols::SymbolLibrary::load_embedded().ok();
    let mut clock = 0.0;
    let mut frame = |state: &mut AppState, event: egui::Event| {
        clock += 1.0 / 60.0;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN)),
            events: vec![event],
            time: Some(clock),
            ..Default::default()
        };
        let started = crate::time_compat::Instant::now();
        let output = ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    super::render_schematic_view(ui, state, symbols.as_ref())
                });
        });
        let elapsed = started.elapsed().as_secs_f64() * 1_000.0;
        std::hint::black_box(output.shapes.len());
        elapsed
    };
    let button = |pos, pressed| egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };

    let mut pointer = egui::pos2(SCREEN.x * 0.5, SCREEN.y * 0.5);
    for _ in 0..WARM_UP_FRAMES {
        frame(&mut state, egui::Event::PointerMoved(pointer));
    }
    let mut moves_ms = vec![frame(&mut state, button(pointer, true))];
    for _ in 0..TIMED_FRAMES {
        pointer.x += grid as f32;
        moves_ms.push(frame(&mut state, egui::Event::PointerMoved(pointer)));
    }
    let drop_ms = frame(&mut state, button(pointer, false));

    let steps = i32::try_from(TIMED_FRAMES).expect("a few frames");
    let dropped = state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == part)
        .map(|component| component.pos);
    assert_eq!(
        dropped,
        Some(Point::new(at.x + steps * grid, at.y)),
        "the part follows the pointer and drops where it was released"
    );
    assert!(
        !state.schematic_drag_in_progress(),
        "the drop ends the drag"
    );
    [
        FrameTimes {
            name: "part drag at 100 %: each move",
            budget_ms: 8.0,
            frames_ms: moves_ms,
            every_frame: true,
        },
        FrameTimes {
            name: "part drag at 100 %: the drop",
            // One display frame at 60 Hz.
            budget_ms: 16.7,
            frames_ms: vec![drop_ms],
            every_frame: true,
        },
    ]
}

/// One suspected site's cost on its own: the median of seven calls, in
/// milliseconds. `work` returns something read off its result, so the call
/// cannot be optimized away.
fn site_ms(mut work: impl FnMut() -> usize) -> f64 {
    let mut samples = (0..7)
        .map(|_| {
            let started = crate::time_compat::Instant::now();
            std::hint::black_box(work());
            started.elapsed().as_secs_f64() * 1_000.0
        })
        .collect::<Vec<_>>();
    samples.sort_by(f64::total_cmp);
    samples[samples.len() / 2]
}

/// Each per-frame site the canvas audit named, timed alone over the fixture,
/// so a site that costs nothing measurable at this scale is reported rather
/// than optimized: the table of their median costs.
fn site_cost_table() -> String {
    use std::fmt::Write as _;

    let mut fixture = fixture_without_overlay();
    for id in fixture.components[0].iter().take(40) {
        fixture
            .state
            .schematic
            .session
            .editor
            .selection
            .select_component(*id);
    }
    fixture.state.ui.schematic_visibility.annotations =
        SchematicAnnotationVisibility::OperatingPoint;
    fixture.state.schematic.ensure_canvas_cache();
    let state = &fixture.state;
    let symbols = super::schematic_symbol_context(state);
    let design = super::schematic_design_view(state);
    let sheet = super::drawing_sheet::resolve_active_drawing_sheet(state);
    let viewport = super::viewport::Viewport {
        offset: egui::Pos2::ZERO,
        zoom: 0.15,
        bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN),
    };
    let pointer = egui::pos2(SCREEN.x * 0.5, SCREEN.y * 0.5);
    let near = Point::new(ORIGIN + PITCH * 3, ORIGIN + 100);
    let mode = state.dialogs.move_selection.mode;
    // The badges at 100 %, each about a dozen mono characters on one line.
    let badges = super::scene::operating_point_annotations(state);
    let badge_sizes = vec![egui::vec2(66.0, 12.0); badges.len()];
    let placement = rspice_schematic_editor::view::scene::BadgePlacement {
        view: &design,
        symbols: &symbols,
        annotations: &badges,
        sizes: &badge_sizes,
        zoom: 1.0,
        world_view: egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN),
    };
    let held_layout = rspice_schematic_editor::view::scene::BadgeLayoutMemo::default();
    // 500 check findings on the active sheet's objects, half on instances
    // and half on wires, the far end of each list first.
    let findings: Vec<rspice_design::drc::DrcLocation> = fixture.components[0]
        .iter()
        .rev()
        .take(250)
        .map(|id| rspice_design::drc::DrcLocation::Component {
            id: *id,
            name: String::new(),
        })
        .chain(
            fixture.routed_wires[0]
                .iter()
                .rev()
                .take(250)
                .map(|id| rspice_design::drc::DrcLocation::Wire { id: *id }),
        )
        .collect();
    let (circles, tessellate_ms) = idle_frame_circles_and_tessellation();
    // What an idle Select frame does with the pointer over the sheet: pick
    // the object under it, and outline every object off the sheet.
    let pick_ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&pick_ctx);
    let pointer_view = rspice_schematic_editor::view::tool_input::ToolPointerView {
        design: super::schematic_design_view(state),
        snap_engine: &state.schematic.session.editor.snap_engine,
        can_edit: true,
    };
    let overflow = std::sync::Arc::new(super::drawing_sheet::drawing_sheet_overflow_summary(
        state, &symbols, &sheet,
    ));
    let overflow_painter = egui::Painter::new(
        pick_ctx.clone(),
        egui::LayerId::background(),
        viewport.bounds,
    );
    // Every wire the active sheet draws, in plain ink, as the canvas groups
    // them at fit.
    let chain_groups = vec![(
        rspice_schematic_editor::view::drawing::ConductorInk::Plain,
        state
            .schematic
            .document()
            .wires
            .iter()
            .enumerate()
            .filter(|(_, wire)| design.object_is_visible(wire.id))
            .map(|(index, _)| index)
            .collect::<Vec<_>>(),
    )];
    let sites = [
        (
            "symbol context, resolved afresh (C-6)",
            site_ms(|| {
                usize::from(symbols.revision() == super::schematic_symbol_context(state).revision())
            }),
        ),
        (
            "selection kept to the active sheet",
            site_ms(|| super::sheet_visibility::selection_on_active_sheet(state).count()),
        ),
        (
            "snap target under the pointer (C-14)",
            site_ms(|| {
                let target = rspice_schematic_editor::view::snap_resolution::resolve_target_pointer(
                    &design,
                    &state.schematic.session.editor.snap_engine,
                    &symbols,
                    &viewport,
                    pointer,
                );
                usize::try_from(target.snapped_position.x.unsigned_abs()).unwrap_or(0)
            }),
        ),
        (
            "junction candidate near a point (C-8)",
            site_ms(|| {
                design
                    .nearest_junction_candidate(near, state.schematic.document().grid_size)
                    .map_or(0, |point| point.x.unsigned_abs() as usize)
            }),
        ),
        (
            "overflow advisories (C-9)",
            site_ms(|| {
                super::drawing_sheet::drawing_sheet_overflow_summary(state, &symbols, &sheet)
                    .items
                    .len()
            }),
        ),
        (
            "overflow advisories drawn, held",
            site_ms(|| {
                rspice_schematic_editor::view::drawing_sheet::draw_overflow_advisories(
                    &overflow_painter,
                    viewport.bounds,
                    &viewport,
                    &overflow,
                );
                overflow.items.len()
            }),
        ),
        (
            "object under the pointer, Select hover",
            site_ms(|| {
                let query = pointer_view.selection_query(&viewport, pointer);
                usize::from(
                    rspice_schematic_editor::view::pointer_target::pointer_target(
                        &pointer_view.design,
                        query,
                        state.ui.schematic_selection_filter,
                        &symbols,
                        &pick_ctx,
                        &viewport,
                        || state.workspace.content.active_view.display_path(),
                    )
                    .is_some(),
                )
            }),
        ),
        (
            "move candidate, delta (40, 0) (C-5)",
            site_ms(|| {
                let (candidate, moved) = super::preview::move_preview_candidate(
                    state,
                    &symbols,
                    Point::new(40, 0),
                    mode,
                );
                candidate.document().wires.len() + usize::from(matches!(moved, Ok(true)))
            }),
        ),
        (
            "operating-point badges, read afresh (C-7)",
            site_ms(|| super::scene::operating_point_annotations(state).len()),
        ),
        (
            "operating-point badge layout, afresh (item 11)",
            site_ms(|| rspice_schematic_editor::view::scene::place_badges(&placement, None).len()),
        ),
        (
            "operating-point badge layout, held (item 11)",
            site_ms(|| {
                rspice_schematic_editor::view::scene::place_badges(&placement, Some(&held_layout))
                    .len()
            }),
        ),
        (
            "conductor chains, active sheet (C-21)",
            site_ms(|| {
                let wires =
                    design.objects_on_active_sheet(&state.schematic.document().wires, |w| w.id);
                rspice_schematic_editor::view::drawing::chain_conductors(
                    wires.iter().map(|wire| wire.points.as_slice()),
                )
                .len()
            }),
        ),
        (
            "conductor chains, held (C-21)",
            site_ms(|| {
                design
                    .canvas_cache
                    .expect("the fixture's canvas cache is built")
                    .conductor_chains(&state.schematic.document().wires, chain_groups.clone())
                    .len()
            }),
        ),
        (
            "violation anchors, 500 findings (C-21)",
            site_ms(|| {
                findings
                    .iter()
                    .filter_map(|location| {
                        rspice_schematic_editor::view::violations::anchor(&design, location)
                    })
                    .count()
            }),
        ),
        ("tessellating the idle frame at fit (C-21)", tessellate_ms),
    ];
    let mut table = String::from("site                                           median ms\n");
    for (site, milliseconds) in sites {
        let _ = writeln!(table, "{site:<46} {milliseconds:>10.3}");
    }
    let _ = writeln!(table, "circles painted in the idle frame at fit: {circles}");
    table
}

/// Print one of the probe's tables.
///
/// The crate denies the print macros, because the desktop build detaches from
/// its console and the browser build has no stderr. This `#[ignore]`d probe's
/// whole output is its tables, read through `--nocapture`, the case the
/// crate's other measurement harnesses argue; every table goes through this
/// one function, so the probe holds one suppression.
#[allow(clippy::print_stdout)]
fn report(table: &str) {
    #[cfg(not(target_arch = "wasm32"))]
    println!("{table}");
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_test::console_log!("{table}");
}

/// One idle Select frame over the fitted sheet: how many circles it paints
/// (symbol corner dots among them) and how long tessellating all of its
/// shapes takes, which the timed frames leave to the host.
fn idle_frame_circles_and_tessellation() -> (usize, f64) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let symbols = super::super::symbols::SymbolLibrary::load_embedded().ok();
    let mut state = fixture_without_overlay().state;
    state.schematic.session.editor.needs_fit = true;
    let mut shapes = Vec::new();
    for _ in 0..WARM_UP_FRAMES {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN)),
            ..Default::default()
        };
        shapes = ctx
            .run_ui(input, |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        super::render_schematic_view(ui, &mut state, symbols.as_ref());
                    });
            })
            .shapes;
    }
    let circles = shapes
        .iter()
        .filter(|clipped| matches!(clipped.shape, egui::Shape::Circle(_)))
        .count();
    let tessellate_ms = site_ms(|| ctx.tessellate(shapes.clone(), 1.0).len());
    (circles, tessellate_ms)
}

/// Release budgets, on desktop and in the browser alike: idle hover under
/// 4 ms; the move preview, Stretch and the operating-point overlay under
/// 8 ms; a part drag's press and every one of its moves under 8 ms, and its
/// drop under one 60 Hz frame, each by its slowest frame and not only its
/// median. Every scenario is printed before any budget is asserted, so a
/// failing run still reports them all.
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[ignore = "frame-cost probe"]
fn canvas_frame_cost_at_scale() {
    use std::fmt::Write as _;

    report(&site_cost_table());
    let mut scenarios = vec![
        idle_hover(),
        move_preview(),
        junction_hover(),
        stretch_preview(),
        operating_point_overlay(),
        operating_point_overlay_legible(),
    ];
    scenarios.extend(part_drag());
    let mut table =
        String::from("scenario                            min ms  median ms     max ms   budget\n");
    for scenario in &scenarios {
        let _ = writeln!(
            table,
            "{:<32} {:>9.2} {:>10.2} {:>10.2} {:>8.1}",
            scenario.name,
            scenario.min(),
            scenario.median(),
            scenario.max(),
            scenario.budget_ms
        );
    }
    report(&table);
    for scenario in &scenarios {
        let (measured, which) = if scenario.every_frame {
            (scenario.max(), "its slowest frame")
        } else {
            (scenario.median(), "a frame")
        };
        assert!(
            measured < scenario.budget_ms,
            "{} takes {measured:.2} ms {which} against a {:.1} ms budget",
            scenario.name,
            scenario.budget_ms
        );
    }
}
