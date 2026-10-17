//! A printed schematic letters every name and value where the canvas does,
//! clear of the part's body for every turn and mirror, and draws the buses,
//! taps, net-label marks, connector tabs and port arrows that say which nets
//! are joined (U-4).

use super::*;
use crate::sources::{SemanticComponent, SemanticSymbolSource, schematic_bounds};
use rspice_design::schematic::{
    bus::{Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation},
    bus_route,
    component_type::ComponentType,
    net_label::{Junction, NetLabel},
    port_overlay,
    rotation::Rotation,
    wire::Wire,
};
use rspice_design_model::{design_management::CrossSheetPortDirection, port::PortDirection};

const UM: i64 = SCHEMATIC_UNIT_UM;

fn part(id: u64, kind: ComponentType, at: (i32, i32), name: &str, value: &str) -> Component {
    Component::new(id, kind, SchematicPoint::new(at.0, at.1)).with_name_value(name, value)
}

/// An authored cell symbol: a 60 x 40 box, its name two grid steps above
/// its top-left corner and its value two below its bottom-left one, where a
/// generated symbol puts them.
fn cell_symbol() -> SymbolDocument {
    SymbolDocument {
        pins: Vec::new(),
        body: vec![SymbolShape::Polyline {
            points: vec![
                SchematicPoint::new(-30, -20),
                SchematicPoint::new(30, -20),
                SchematicPoint::new(30, 20),
                SchematicPoint::new(-30, 20),
            ],
            closed: true,
        }],
        origin: SchematicPoint::origin(),
        name_anchor: SchematicPoint::new(-30, -40),
        value_anchor: SchematicPoint::new(-30, 40),
        ..SymbolDocument::default()
    }
}

fn semantic(component: Component) -> SemanticComponent {
    let resolved_symbol = (component.kind == ComponentType::CellInstance).then(cell_symbol);
    SemanticComponent {
        symbol_source: resolved_symbol
            .as_ref()
            .map(|_| SemanticSymbolSource::Authored),
        component,
        resolved_symbol,
    }
}

fn sheet(components: Vec<Component>) -> SemanticSchematic {
    SemanticSchematic {
        view_path: "test-schematic".to_owned(),
        drawing_sheet: None,
        drawing_sheet_title_values: BTreeMap::new(),
        drawing_sheet_page_numbering: None,
        grid_pitch_units: 10,
        components: components.into_iter().map(semantic).collect(),
        wires: Vec::new(),
        buses: Vec::new(),
        bus_taps: Vec::new(),
        junctions: Vec::new(),
        net_labels: Vec::new(),
        design_notes: Vec::new(),
        documentation_shapes: Vec::new(),
    }
}

/// The review sheet: turned and mirrored parts, a bus with two taps, a net
/// label, two off-sheet connectors and two ports.
pub(super) fn review_sheet() -> SemanticSchematic {
    let mut components = vec![
        part(1, ComponentType::Resistor, (0, 0), "R1", "10k"),
        part(2, ComponentType::Resistor, (80, 0), "R2", "4.7k")
            .with_rotation(Rotation::R90)
            .with_mirror_h(true),
        part(3, ComponentType::Nmos, (160, 0), "M1", "nch"),
        part(4, ComponentType::Pmos, (260, 0), "M2", "pch")
            .with_rotation(Rotation::R90)
            .with_mirror_v(true),
        part(5, ComponentType::NpnBjt, (360, 0), "Q1", "bc547")
            .with_rotation(Rotation::R270)
            .with_mirror_h(true),
        part(6, ComponentType::CellInstance, (480, 0), "X1", "amp").with_rotation(Rotation::R180),
        part(7, ComponentType::Port, (0, 120), "P1", "IN"),
        part(8, ComponentType::Port, (480, 120), "P2", "OUT").with_rotation(Rotation::R180),
        part(9, ComponentType::VoltageSource, (40, 200), "V1", "5"),
    ];
    components[6].params = "dir=in".to_owned();
    components[7].params = "dir=out".to_owned();
    let mut sheet = sheet(components);
    let bus = Bus::segment(
        20,
        SchematicPoint::new(120, 160),
        SchematicPoint::new(400, 160),
        Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
    )
    .unwrap();
    let tap = |id, x, bit: &str| {
        BusTap::new(
            id,
            &bus,
            SchematicPoint::new(x, 160),
            SchematicPoint::new(x + 20, 200),
            BusSlice::parse(bit).unwrap(),
            BusTapOrientation::Automatic,
        )
        .unwrap()
    };
    sheet.bus_taps = vec![tap(21, 180, "DATA[3]"), tap(22, 280, "DATA[4]")];
    sheet.buses = vec![bus];
    sheet.wires = vec![
        Wire::segment(30, SchematicPoint::new(20, 0), SchematicPoint::new(60, 0)),
        Wire::segment(31, SchematicPoint::new(40, 0), SchematicPoint::new(40, 60)),
    ];
    sheet.junctions = vec![Junction::new(32, SchematicPoint::new(40, 0))];
    sheet.net_labels = vec![
        NetLabel::new(40, SchematicPoint::new(40, 60), "VOUT"),
        NetLabel::off_sheet(
            41,
            SchematicPoint::new(200, 240),
            "CLK",
            CrossSheetPortDirection::Output,
        ),
        NetLabel::off_sheet(
            42,
            SchematicPoint::new(320, 240),
            "RST",
            CrossSheetPortDirection::Input,
        ),
    ];
    sheet
}

struct Compiled {
    primitives: Vec<ScenePrimitive>,
    minimum: SemanticPoint,
}

impl Compiled {
    /// A world point in this scene's micrometres.
    fn at(&self, x_wu: f64, y_wu: f64) -> (i64, i64) {
        (
            (x_wu * UM as f64).round() as i64 - self.minimum.x_um,
            (y_wu * UM as f64).round() as i64 - self.minimum.y_um,
        )
    }

    fn texts(&self) -> Vec<(&str, i64, i64, TextAnchor, u64)> {
        self.primitives
            .iter()
            .filter_map(|primitive| match primitive {
                ScenePrimitive::Text {
                    origin,
                    text,
                    anchor,
                    size,
                    ..
                } => Some((
                    text.as_str(),
                    origin.x.micrometres() as i64,
                    origin.y.micrometres() as i64,
                    *anchor,
                    size.micrometres(),
                )),
                _ => None,
            })
            .collect()
    }

    fn text(&self, wanted: &str) -> (i64, i64, TextAnchor, u64) {
        let found = self
            .texts()
            .into_iter()
            .filter(|(text, ..)| *text == wanted)
            .map(|(_, x, y, anchor, size)| (x, y, anchor, size))
            .collect::<Vec<_>>();
        assert_eq!(
            found.len(),
            1,
            "one run reads {wanted:?}: {:?}",
            self.texts()
        );
        found[0]
    }
}

fn compile(semantic: &SemanticSchematic) -> Compiled {
    let bounds = schematic_bounds(semantic).unwrap();
    let mapping = PrintMappingTable::try_new(PrintMappingSaveScope::Document, Vec::new()).unwrap();
    let mut compiler = SemanticSceneCompiler::new(
        bounds,
        bounds.content_extent().unwrap(),
        &mapping,
        SchematicHardcopySetup::default(),
    );
    compiler.schematic(semantic).unwrap();
    Compiled {
        primitives: compiler.primitives,
        minimum: bounds.minimum,
    }
}

/// A box as `(left, top, right, bottom)`, in scene micrometres.
type Box2 = (i64, i64, i64, i64);

fn union(boxes: impl IntoIterator<Item = Box2>) -> Option<Box2> {
    boxes
        .into_iter()
        .reduce(|a, b| (a.0.min(b.0), a.1.min(b.1), a.2.max(b.2), a.3.max(b.3)))
}

fn point_box(point: ScenePoint, reach: i64) -> Box2 {
    let (x, y) = (point.x.micrometres() as i64, point.y.micrometres() as i64);
    (x - reach, y - reach, x + reach, y + reach)
}

/// What the page draws of a part: every stroke and dot, not its text.
fn drawn_box(primitives: &[ScenePrimitive]) -> Box2 {
    union(primitives.iter().flat_map(|primitive| match primitive {
        ScenePrimitive::Polyline { points, .. } => {
            points.iter().map(|point| point_box(*point, 0)).collect()
        }
        ScenePrimitive::Line { from, to, .. } => vec![point_box(*from, 0), point_box(*to, 0)],
        ScenePrimitive::Circle { center, radius, .. } => {
            vec![point_box(*center, radius.micrometres() as i64)]
        }
        _ => Vec::new(),
    }))
    .expect("the part draws something")
}

/// The box a printed run takes: Plex's ascent above its baseline and its
/// descent below, and the shared layout's estimated advance across.
fn text_box(x: i64, baseline: i64, anchor: TextAnchor, size: u64, text: &str) -> Box2 {
    let size = size as f64;
    let width = (text.chars().count() as f64 * size * 0.62).round() as i64;
    let left = match anchor {
        TextAnchor::Start => x,
        TextAnchor::Middle => x - width / 2,
        TextAnchor::End => x - width,
    };
    (
        left,
        baseline - (size * 1.025).round() as i64,
        left + width,
        baseline + (size * 0.275).round() as i64,
    )
}

fn overlap(a: Box2, b: Box2) -> bool {
    a.0 < b.2 && b.0 < a.2 && a.1 < b.3 && b.1 < a.3
}

#[test]
fn a_printed_name_and_value_stand_clear_of_the_body_for_every_turn_and_mirror() {
    for kind in [
        ComponentType::Resistor,
        ComponentType::Nmos,
        ComponentType::CellInstance,
    ] {
        for rotation in [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270] {
            for (mirror_h, mirror_v) in [(false, false), (true, false), (false, true)] {
                let component = part(1, kind, (0, 0), "N1", "V1")
                    .with_rotation(rotation)
                    .with_mirror_h(mirror_h)
                    .with_mirror_v(mirror_v);
                let compiled = compile(&sheet(vec![component]));
                let body = drawn_box(&compiled.primitives);
                let texts = compiled.texts();
                let context = format!("{kind:?} {rotation:?} mirror ({mirror_h}, {mirror_v})");
                assert_eq!(
                    texts.iter().map(|text| text.0).collect::<Vec<_>>(),
                    ["N1", "V1"],
                    "{context}: the name and the value, once each"
                );
                for (text, x, y, anchor, size) in texts {
                    let label = text_box(x, y, anchor, size, text);
                    assert!(
                        !overlap(label, body),
                        "{context}: {text:?} at {label:?} overlaps the body at {body:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn a_printed_label_hangs_off_the_canvas_anchor_in_the_canvas_type() {
    let compiled = compile(&review_sheet());

    // Upright resistor: name centred 4 wu above the body, value 4 wu below.
    // A 10-wu name's baseline is 13 wu (one line box) up from its bottom
    // edge plus 10.25 (Plex's ascent) down; a 9-wu value's is 9.225 below
    // its top edge.
    let (x, y) = compiled.at(0.0, -14.0 - 13.0 + 10.25);
    assert_eq!(compiled.text("R1"), (x, y, TextAnchor::Middle, 2_500));
    let (x, y) = compiled.at(0.0, 14.0 + 9.225);
    assert_eq!(compiled.text("10k"), (x, y, TextAnchor::Middle, 2_250));

    // Turned a quarter: the name ends 4 wu left of the body, the value starts
    // 4 wu right of it, both centred on the part.
    let (x, y) = compiled.at(80.0 - 14.0, -6.5 + 10.25);
    assert_eq!(compiled.text("R2"), (x, y, TextAnchor::End, 2_500));
    let (x, y) = compiled.at(80.0 + 14.0, -5.85 + 9.225);
    assert_eq!(compiled.text("4.7k"), (x, y, TextAnchor::Start, 2_250));

    // A MOSFET: name above, value to the right, past the 40 x 80 body.
    let (x, y) = compiled.at(160.0, -44.0 - 13.0 + 10.25);
    assert_eq!(compiled.text("M1"), (x, y, TextAnchor::Middle, 2_500));
    let (x, y) = compiled.at(160.0 + 24.0, -5.85 + 9.225);
    assert_eq!(compiled.text("nch"), (x, y, TextAnchor::Start, 2_250));

    // The cell turned a half: its symbol's name anchor (-30, -40) lands at
    // (30, 40) and the run, which grew rightward, now ends there.
    let (x, y) = compiled.at(480.0 + 30.0, 40.0 - 6.5 + 10.25);
    assert_eq!(compiled.text("X1"), (x, y, TextAnchor::End, 2_500));
}

/// Every polyline drawn at `width`, as world-unit points relative to the
/// scene.
fn polylines_at(compiled: &Compiled, width: u64) -> Vec<Vec<(i64, i64)>> {
    compiled
        .primitives
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Polyline { points, stroke, .. }
                if stroke.width.micrometres() == width =>
            {
                Some(
                    points
                        .iter()
                        .map(|point| (point.x.micrometres() as i64, point.y.micrometres() as i64))
                        .collect(),
                )
            }
            _ => None,
        })
        .collect()
}

#[test]
fn a_printed_bus_reads_as_the_canvas_draws_it() {
    let sheet = review_sheet();
    let compiled = compile(&sheet);

    // Three 1.2-wu strands, 4 wu apart, along the bus.
    let mut strands = polylines_at(&compiled, 300);
    strands.sort();
    let strand = |y: f64| vec![compiled.at(120.0, y), compiled.at(400.0, y)];
    assert_eq!(strands, [strand(156.0), strand(160.0), strand(164.0)]);

    // The declaration stands 8 wu right of and 8 wu above the bus's end,
    // its bottom-left corner there: a 9-wu line box up, Plex's ascent down.
    let (x, y) = compiled.at(408.0, 152.0 - 11.7 + 9.225);
    assert_eq!(compiled.text("DATA[7:0]"), (x, y, TextAnchor::Start, 2_250));

    // Each tap takes the canvas's route at 2 wu, breakout and all, and
    // letters its bit 5 wu right of and 7 wu above its connection point.
    let taps = polylines_at(&compiled, 500);
    assert_eq!(taps.len(), 2);
    for (tap, bit) in sheet.bus_taps.iter().zip(["DATA[3]", "DATA[4]"]) {
        let route = bus_route::bus_tap_route_points(tap)
            .into_iter()
            .map(|point| compiled.at(f64::from(point.x), f64::from(point.y)))
            .collect::<Vec<_>>();
        assert!(route.len() >= 3, "{bit}: the route breaks out of the bus");
        assert!(taps.contains(&route), "{bit}: {route:?} in {taps:?}");
        let corner = tap.connection_point;
        let (x, y) = compiled.at(
            f64::from(corner.x) + 5.0,
            f64::from(corner.y) - 7.0 - 11.7 + 9.225,
        );
        assert_eq!(compiled.text(bit), (x, y, TextAnchor::Start, 2_250));
    }
}

/// Every straight line drawn at `width`, as scene-micrometre endpoints.
fn lines_at(compiled: &Compiled, width: u64) -> Vec<[(i64, i64); 2]> {
    let at = |point: &ScenePoint| (point.x.micrometres() as i64, point.y.micrometres() as i64);
    compiled
        .primitives
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Line { from, to, stroke } if stroke.width.micrometres() == width => {
                Some([at(from), at(to)])
            }
            _ => None,
        })
        .collect()
}

#[test]
fn a_printed_net_label_carries_its_mark_and_a_connector_its_tab() {
    let compiled = compile(&review_sheet());

    // VOUT's attachment diamond, 2.25 wu to each point, filled, on (40, 60).
    let diamond = vec![
        compiled.at(40.0, 57.75),
        compiled.at(42.25, 60.0),
        compiled.at(40.0, 62.25),
        compiled.at(37.75, 60.0),
    ];
    let filled = compiled
        .primitives
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Polyline {
                points,
                closed: true,
                fill: Some(_),
                ..
            } => Some(
                points
                    .iter()
                    .map(|point| (point.x.micrometres() as i64, point.y.micrometres() as i64))
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(filled.contains(&diamond), "{diamond:?} in {filled:?}");

    // Its name stands on the corner 7 wu right of and 5 wu above the mark,
    // in Plex Mono Medium 11 wu: one line box up, Plex's ascent down.
    let (x, y) = compiled.at(47.0, 55.0 - 14.3 + 11.275);
    assert_eq!(compiled.text("VOUT"), (x, y, TextAnchor::Start, 2_750));

    // CLK's tab: 3 wu after three 6.7-wu glyphs, 12 wu wide and one line
    // box tall, stroked at 0.085 of its height.
    let tab = vec![
        compiled.at(230.1, 220.7),
        compiled.at(242.1, 220.7),
        compiled.at(242.1, 235.0),
        compiled.at(230.1, 235.0),
    ];
    let outlines = polylines_at(&compiled, 304);
    assert!(outlines.contains(&tab), "{tab:?} in {outlines:?}");
    assert_eq!(outlines.len(), 2, "one tab per connector");

    // Each tab holds a stem and a two-stroke head, at the end the signal
    // travels toward: right for CLK's output, left for RST's input.
    let marks = lines_at(&compiled, 304);
    assert_eq!(marks.len(), 6);
    for (left, right, head_right) in [(230.1, 242.1, true), (350.1, 362.1, false)] {
        let (left, right) = (compiled.at(left, 0.0).0, compiled.at(right, 0.0).0);
        let inside = marks
            .iter()
            .filter(|[a, b]| (left..=right).contains(&a.0) && (left..=right).contains(&b.0))
            .collect::<Vec<_>>();
        assert_eq!(inside.len(), 3);
        let tip = inside[1][0];
        assert!(inside[1..].iter().all(|mark| mark[0] == tip), "{inside:?}");
        assert_eq!(tip.0 > (left + right) / 2, head_right, "{inside:?}");
    }
}

#[test]
fn a_printed_port_shows_which_way_its_signal_goes() {
    let sheet = review_sheet();
    let compiled = compile(&sheet);
    let marks = lines_at(&compiled, 250);

    // P1, an input, upright at (0, 120): the arrow points into the body.
    let p1 = [
        [compiled.at(-1.5, 120.0), compiled.at(5.0, 120.0)],
        [compiled.at(5.0, 120.0), compiled.at(2.0, 117.5)],
        [compiled.at(5.0, 120.0), compiled.at(2.0, 122.5)],
    ];
    for mark in &p1 {
        assert!(marks.contains(mark), "{mark:?} in {marks:?}");
    }

    // P2, an output turned a half, through the turn its body was drawn at.
    let library = SymbolLibrary::load_embedded().unwrap();
    let port = &sheet.components[7].component;
    let (_, rotation) = library
        .get_with_rotation_variant(port.kind, port.rotation.degrees(), None)
        .unwrap();
    let place = |point| {
        let point = port_overlay::transform_point(point, rotation, false, false);
        compiled.at(480.0 + f64::from(point.x), 120.0 + f64::from(point.y))
    };
    for segment in port_overlay::direction_segments(PortDirection::Out) {
        let mark = [place(segment.start), place(segment.end)];
        assert!(marks.contains(&mark), "{mark:?} in {marks:?}");
    }
    assert_eq!(marks.len(), 6, "three strokes for each port");
}

/// A port's name is an electrical name, printed in the type a connector's
/// name is: Plex Mono Medium, 11 wu, in the net ink (D1 section 1). It was
/// set as an instance value, in 9-wu Plex Sans and the value ink.
#[test]
fn a_printed_port_name_is_set_as_a_net_name() {
    let compiled = compile(&review_sheet());
    let set_in = |wanted: &str| {
        compiled
            .primitives
            .iter()
            .find_map(|primitive| match primitive {
                ScenePrimitive::Text {
                    text,
                    font,
                    size,
                    color,
                    ..
                } if text == wanted => Some((*font, size.micrometres(), *color)),
                _ => None,
            })
            .unwrap_or_else(|| panic!("{wanted:?} is printed"))
    };
    let connector = set_in("CLK");
    assert_eq!(
        connector,
        (
            SceneFont::MonospaceMedium,
            2_750,
            SemanticColor::Exact(Rgb8::new(0x7D, 0x52, 0x00))
        )
    );
    for port in ["IN", "OUT"] {
        assert_eq!(set_in(port), connector, "{port} is set as a net name");
    }
}

fn resolved_review_sheet() -> ResolvedHardcopyDocument {
    let semantic = review_sheet();
    let identity = HardcopySourceIdentity::try_new(
        "test-schematic",
        HardcopyDocumentId::try_from_uuid(Uuid::from_u128(0x5343_4845)).unwrap(),
        ObjectRevision::INITIAL,
        "Label review",
    )
    .unwrap();
    let digest =
        rspice_hardcopy_contract::sources::canonical_digest(b"label-review", &semantic).unwrap();
    let bounds = schematic_bounds(&semantic).unwrap();
    resolve_semantic_source(
        identity,
        digest,
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::ActiveDocument,
        HardcopySemanticDocument::Schematic(Box::new(semantic)),
        bounds,
    )
    .unwrap()
}

#[test]
fn a_printed_sheet_draws_in_the_export_inks_and_weights() {
    let source = resolved_review_sheet();
    let scene = scene_from_resolved(
        &source,
        source.default_print_mapping(),
        SchematicHardcopySetup::default(),
        resolved_metadata(),
    )
    .unwrap();
    let wire = SemanticColor::Exact(Rgb8::new(0x4E, 0x34, 0x00));
    let symbol = SemanticColor::Exact(Rgb8::new(0x2C, 0x31, 0x35));
    let strokes = scene
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Polyline { stroke, .. } | ScenePrimitive::Line { stroke, .. } => {
                Some((stroke.width.micrometres(), stroke.color))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    // D1: wires 1.1 wu, bus strands 1.2, taps 2.0, all in the wire ink;
    // symbols 1.0 wu in the symbol ink.
    for (width, ink, count) in [(275, wire, 2), (300, wire, 3), (500, wire, 2)] {
        let drawn = strokes
            .iter()
            .filter(|stroke| stroke.0 == width)
            .collect::<Vec<_>>();
        assert_eq!(drawn.len(), count, "{width} um strokes: {strokes:?}");
        assert!(drawn.iter().all(|stroke| stroke.1 == ink), "{drawn:?}");
    }
    let symbols = strokes
        .iter()
        .filter(|stroke| stroke.0 == 250)
        .collect::<Vec<_>>();
    assert!(!symbols.is_empty());
    assert!(
        symbols.iter().all(|stroke| stroke.1 == symbol),
        "{symbols:?}"
    );
    // A junction is a 1.5-wu dot in the wire ink.
    let dots = scene
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Circle {
                radius,
                fill: Some(fill),
                ..
            } => Some((radius.micrometres(), *fill)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(dots, [(375, SceneFill::solid(wire))]);
}

/// The review sheet as a PNG, for a person to look at, through the print
/// mapping a new hardcopy starts from.
#[test]
#[ignore = "writes a review PNG to RSPICE_RASTER_DIR"]
fn render_schematic_label_review() {
    let source = resolved_review_sheet();
    let base = setup(OutputFormat::Png { dpi: 200 }, false);
    let setup = HardcopySetup::try_new_with_schematic(
        base.physical_page().clone(),
        base.scale(),
        base.tiling(),
        base.render().clone(),
        base.decorations().clone(),
        SchematicHardcopySetup::default(),
        source.default_print_mapping().clone(),
    )
    .unwrap();
    let plan = HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(Uuid::from_u128(79)).unwrap(),
        source.authority().clone(),
        setup,
        source.content_extent(),
    )
    .unwrap();
    let publication =
        HardcopyRenderer::render_resolved(&plan, &source, resolved_metadata()).unwrap();
    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).unwrap();
    let stage = std::env::var("RSPICE_REVIEW_STAGE").unwrap_or_else(|_| "now".to_owned());
    let path = directory.join(format!("printed-schematic-{stage}.png"));
    std::fs::write(&path, publication.single_part().unwrap().bytes()).unwrap();
    eprintln!("wrote {}", path.display());
}
