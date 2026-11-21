//! Canvas text read off fixture sheets rendered through the real paint path
//! ([`rspice_app_design::schematic::view::render_schematic_view`]).
//!
//! The sheets are a hierarchy sheet with its operating-point overlay on and a
//! shelf of functional parts. [`render`] drives a headless context through
//! three whole frames and hands back the shapes the last frame painted
//! (points), so an assertion can read them.

use egui::{Color32, Rect, Shape, epaint::ClippedShape};
use rspice_design::schematic::{net_label::NetLabel, rotation::Rotation};

use rspice_app_state::state::{
    AnalysisResult, AnalysisType, Component, ComponentType, CrossSheetPortDirection, DcOpResult,
    Junction, LibraryCellInstance, OperatingPointValue, Point, SchematicAnnotationVisibility,
    SimulationRun, Wire,
};

use crate::workbench::app_state::AppState;

/// The canvas, in points.
pub(super) const SCREEN: egui::Vec2 = egui::vec2(960.0, 640.0);

/// Hands out object ids in order.
struct Ids(u64);

impl Ids {
    fn next(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }
}

fn wire(ids: &mut Ids, points: &[(i32, i32)]) -> Wire {
    Wire::new(
        ids.next(),
        points.iter().map(|(x, y)| Point::new(*x, *y)).collect(),
    )
}

fn junction(ids: &mut Ids, at: (i32, i32)) -> Junction {
    Junction::new(ids.next(), Point::new(at.0, at.1))
}

/// Finish a fixture the way a load does: runtime state, topology, and the
/// workspace's copy of the active schematic.
fn settle(state: &mut AppState) {
    state.model.schematic.recalculate_runtime_state();
    state.model.schematic.bump_topology_version();
    state.sync_active_schematic_to_workspace();
}

/// Where the hierarchy sheet is measured from: its middle, and the two
/// connectors between its instances with their badges.
pub(super) const HIERARCHY_CENTRE: Point = Point::new(470, 330);

/// Three instances of the probe's masters wired in a chain, the cell's
/// ports in each direction, an off-sheet connector of each direction, and a
/// solved operating point over five named nets with the overlay on. `X2` is
/// selected, so its device block shows.
///
/// Spans world x 140 to 800 and y 160 to 500.
pub(super) fn hierarchy_sheet() -> AppState {
    let mut ids = Ids(0);
    let mut components = Vec::new();
    let instance = |ids: &mut Ids, master: &str, at: (i32, i32), name: &str| {
        let mut binding = LibraryCellInstance::new("work", master, "schematic");
        binding.bind_interface(&rspice_app_design::schematic::view::frame_cost::master_ports());
        let mut component = Component::new(
            ids.next(),
            ComponentType::CellInstance,
            Point::new(at.0, at.1),
        );
        component.name = name.to_owned();
        component.library_cell = Some(binding);
        component
    };
    // amp (w 40), buf (w 50), bias (w 60): IN (-w, 0), OUT (w, 0), BIAS (0, 40).
    components.push(instance(&mut ids, "amp", (300, 200), "X1"));
    let selected = instance(&mut ids, "buf", (600, 200), "X2");
    let selected_id = selected.id;
    components.push(selected);
    components.push(instance(&mut ids, "bias", (450, 380), "X3"));
    let port = |ids: &mut Ids, at: (i32, i32), name: &str, direction: &str, rotation| {
        let mut component = Component::new(ids.next(), ComponentType::Port, Point::new(at.0, at.1))
            .with_rotation(rotation);
        component.value = name.to_owned();
        component.params = format!("dir={direction}");
        component
    };
    components.push(port(&mut ids, (150, 200), "IN", "in", Rotation::R180));
    components.push(port(&mut ids, (770, 200), "OUT", "out", Rotation::R0));
    components.push(port(&mut ids, (150, 380), "VREF", "inout", Rotation::R180));
    components.push(port(&mut ids, (460, 480), "VDD", "supply", Rotation::R0));

    let wires = vec![
        wire(&mut ids, &[(160, 200), (260, 200)]),
        wire(&mut ids, &[(340, 200), (550, 200)]),
        wire(&mut ids, &[(650, 200), (760, 200)]),
        wire(&mut ids, &[(510, 380), (600, 380), (600, 240)]),
        wire(&mut ids, &[(300, 240), (300, 300), (600, 300)]),
        wire(&mut ids, &[(160, 380), (390, 380)]),
        wire(&mut ids, &[(450, 420), (450, 480)]),
    ];
    let junctions = vec![junction(&mut ids, (600, 300))];
    let labels = vec![
        NetLabel::off_sheet(
            ids.next(),
            Point::new(450, 200),
            "MID",
            CrossSheetPortDirection::Output,
        ),
        NetLabel::off_sheet(
            ids.next(),
            Point::new(400, 300),
            "NBIAS",
            CrossSheetPortDirection::Input,
        ),
        NetLabel::off_sheet(
            ids.next(),
            Point::new(260, 380),
            "VREF",
            CrossSheetPortDirection::InOut,
        ),
        NetLabel::off_sheet(
            ids.next(),
            Point::new(450, 450),
            "VDD",
            CrossSheetPortDirection::Supply,
        ),
    ];
    let nets = [
        ("IN", &wires[0]),
        ("MID", &wires[1]),
        ("OUT", &wires[2]),
        ("NBIAS", &wires[3]),
        ("VREF", &wires[5]),
        ("VDD", &wires[6]),
    ]
    .into_iter()
    .map(|(net, wire)| (net.to_owned(), wire.points.clone()))
    .chain(std::iter::once((
        "NBIAS".to_owned(),
        wires[4].points.clone(),
    )))
    .fold(
        std::collections::HashMap::<String, Vec<Point>>::new(),
        |mut nets, (net, points)| {
            nets.entry(net).or_default().extend(points);
            nets
        },
    );

    let mut state = AppState::default();
    state
        .model
        .library_manager
        .add_library(rspice_app_design::schematic::view::frame_cost::master_library());
    let document = state.model.schematic.document_mut_for_test();
    document.components = components;
    document.wires = wires;
    document.junctions = junctions;
    document.net_labels = labels;
    settle(&mut state);

    rspice_app_design::schematic::view::frame_cost::install_projected_run_geometry(
        &mut state.model,
        nets,
    );
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_dc_op(DcOpResult {
            node_voltages: [
                ("V(in)", 0.9),
                ("V(mid)", 1.2134),
                ("V(out)", 0.91243),
                ("V(nbias)", 0.6512),
                ("V(vref)", 0.6),
                ("V(vdd)", 1.8),
            ]
            .into_iter()
            .map(|(name, value)| OperatingPointValue {
                name: name.to_owned(),
                value,
                unit: "V".to_owned(),
            })
            .collect(),
            branch_currents: vec![OperatingPointValue {
                name: "I(X2)".to_owned(),
                value: 21.4e-6,
                unit: "A".to_owned(),
            }],
            power_dissipation: Vec::new(),
        }),
    );
    state.model.simulation.retained.runs.insert(0, run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state.design.schematic_visibility.annotations = SchematicAnnotationVisibility::OperatingPoint;
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(selected_id);
    state
}

/// How a render frames its sheet.
#[derive(Clone, Copy)]
pub(super) struct Framing {
    /// Points per world unit.
    pub(super) zoom: f64,
    /// The world point at the middle of the canvas.
    pub(super) centre: Point,
}

/// One rendered frame.
pub(super) struct Render {
    /// What the last frame painted, in points.
    pub(super) shapes: Vec<ClippedShape>,
    pub(super) tokens: std::sync::Arc<rspice_app_workbench::ui::tokens::Tokens>,
}

/// Render `state` through the real canvas at `framing`, under `theme`, on a
/// display of `pixels_per_point`.
pub(super) fn render(
    state: &mut AppState,
    theme: rspice_app_workbench::ui::Theme,
    framing: Framing,
    pixels_per_point: f32,
) -> Render {
    let zoom = framing.zoom;
    let ctx = egui::Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    theme.apply(&ctx);
    let symbols = rspice_app_design::schematic::symbols::SymbolLibrary::load_embedded().ok();
    let mut output = None;
    for _ in 0..3 {
        let editor = &mut state.model.schematic.session.editor;
        editor.needs_fit = false;
        editor.needs_drawing_sheet_fit = false;
        editor.zoom = zoom;
        editor.pan = (
            f64::from(SCREEN.x) * 0.5 - f64::from(framing.centre.x) * zoom,
            f64::from(SCREEN.y) * 0.5 - f64::from(framing.centre.y) * zoom,
        );
        let input = rspice_app_workbench::ui::raster::display_input(SCREEN, pixels_per_point);
        output = Some(ctx.run_ui(input, |ui| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ui, |ui| {
                    rspice_app_design::schematic::view::render_schematic_view(
                        ui,
                        &mut state.canvas_frame(),
                        symbols.as_ref(),
                    );
                });
        }));
    }
    let output = output.expect("three frames");
    assert!(
        (ctx.pixels_per_point() - pixels_per_point).abs() < f32::EPSILON,
        "the context renders at {} px/pt, not {pixels_per_point}",
        ctx.pixels_per_point()
    );
    Render {
        shapes: output.shapes,
        tokens: rspice_app_workbench::ui::tokens::Tokens::get(&ctx),
    }
}

/// Every leaf shape a frame painted, with its clip, flattened out of nested
/// `Shape::Vec`s.
pub(super) fn leaf_shapes(shapes: &[ClippedShape]) -> Vec<(Rect, &Shape)> {
    fn walk<'a>(clip: Rect, shape: &'a Shape, out: &mut Vec<(Rect, &'a Shape)>) {
        match shape {
            Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(clip, shape, out);
                }
            }
            shape => out.push((clip, shape)),
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(clipped.clip_rect, &clipped.shape, &mut out);
    }
    out
}

/// One painted run of text: its words and its colour.
pub(super) struct PaintedText {
    pub(super) text: String,
    pub(super) colour: Color32,
}

pub(super) fn painted_text(shapes: &[ClippedShape]) -> Vec<PaintedText> {
    leaf_shapes(shapes)
        .into_iter()
        .filter_map(|(clip, shape)| {
            let Shape::Text(text) = shape else {
                return None;
            };
            let section = text.galley.job.sections.first()?;
            let rect = Rect::from_min_size(text.pos, text.galley.size());
            if !clip.intersects(rect) || text.galley.text().trim().is_empty() {
                return None;
            }
            let colour = text
                .override_text_color
                .unwrap_or(if section.format.color == Color32::PLACEHOLDER {
                    text.fallback_color
                } else {
                    section.format.color
                })
                .gamma_multiply(text.opacity_factor);
            Some(PaintedText {
                text: text.galley.text().to_owned(),
                colour,
            })
        })
        .collect()
}

fn at_zoom(zoom: f64, centre: Point) -> Framing {
    Framing { zoom, centre }
}

/// An operating-point badge reads like an annotation: four significant
/// digits, the SI prefix joined to its unit and micro spelled `µ` (D1
/// section 8), so `912.4 mV`, not the property-editor spelling `912.430m V`.
/// An unlabelled net's badge names it as the drawing does: the ports are
/// drawn `IN` and `OUT`, whatever case the result spells them in.
#[test]
fn an_operating_point_badge_reads_four_digits_and_a_prefixed_unit() {
    let mut state = hierarchy_sheet();
    let render = render(
        &mut state,
        rspice_app_workbench::ui::Theme::default(),
        at_zoom(1.0, HIERARCHY_CENTRE),
        1.0,
    );
    let info = render.tokens.color.info;
    let badges = painted_text(&render.shapes)
        .into_iter()
        .filter(|text| text.colour == info)
        .map(|text| text.text)
        .collect::<std::collections::BTreeSet<_>>();
    let expected = [
        "1.213 V",
        "1.8 V",
        "600 mV",
        "651.2 mV",
        "I(X2) = 21.4 \u{00b5}A",
        "IN 900 mV",
        "OUT 912.4 mV",
    ]
    .map(str::to_owned)
    .into_iter()
    .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(badges, expected);
}

/// Where the functional sheet's first four rows are measured from: their
/// middle. The canvas holds four rows; each further four sit 560 lower.
const FUNCTIONAL_CENTRE: Point = Point::new(340, 300);

/// Every functional part on the shelf placed the way the shelf places it,
/// four to a row, 160 by 140 apart from (100, 90).
fn functional_sheet() -> AppState {
    let mut state = AppState::default();
    for (index, descriptor) in rspice_app_state::state::functional_parts()
        .iter()
        .enumerate()
    {
        let (column, row) = (index as i32 % 4, index as i32 / 4);
        state.model.schematic.add_library_cell_component(
            Point::new(100 + 160 * column, 90 + 140 * row),
            rspice_app_state::state::functional_library_binding(descriptor),
        );
    }
    settle(&mut state);
    state
}

/// A functional part placed from the shelf letters its part's name, never
/// the library subcircuit it netlists as.
#[test]
fn a_placed_functional_part_letters_its_name_not_its_subcircuit() {
    let mut state = functional_sheet();
    let rows = rspice_app_state::state::functional_parts()
        .len()
        .div_ceil(4);
    let mut painted = Vec::new();
    for band in 0..rows.div_ceil(4) {
        let centre = Point::new(FUNCTIONAL_CENTRE.x, FUNCTIONAL_CENTRE.y + 560 * band as i32);
        let render = render(
            &mut state,
            rspice_app_workbench::ui::Theme::default(),
            at_zoom(1.0, centre),
            1.0,
        );
        painted.extend(painted_text(&render.shapes));
    }
    for descriptor in rspice_app_state::state::functional_parts() {
        assert!(
            painted
                .iter()
                .any(|text| text.text == descriptor.display_name),
            "{} is not lettered",
            descriptor.display_name
        );
        assert!(
            painted
                .iter()
                .all(|text| !text.text.contains(descriptor.subcircuit)),
            "{} is lettered as {}",
            descriptor.display_name,
            descriptor.subcircuit
        );
    }
}
