//! The canvas raster review set: three fixture sheets rendered through the
//! real paint path ([`super::render_schematic_view`]) at the zooms, themes
//! and pixel densities a customer meets, and the measurements a review reads
//! off them.
//!
//! The sheets are a dense analog stage, a hierarchy sheet with its
//! operating-point overlay on, and the frame-cost probe's production-scale
//! sheet. [`render`] drives a headless context through three whole frames and
//! hands back both the rasterized canvas (device pixels) and the shapes the
//! last frame painted (points), so an assertion can measure either.
//!
//! The PNGs are for a person: `render_the_canvas_raster_review_set` is
//! `#[ignore]`d and writes them, with a measurement table beside each, to
//! `RSPICE_RASTER_DIR`. No image or fingerprint is pinned: every claim a
//! review makes is pinned as a measured-geometry assertion instead.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use egui::{Color32, Rect, Shape, epaint::ClippedShape};
use rspice_design::schematic::{
    bus::{Bus, BusTap, BusTapOrientation},
    design_note::{DesignNote, DesignNoteKind},
    net_label::NetLabel,
    probe::SchematicProbe,
    rotation::Rotation,
};
use rspice_design_model::bus::{BusDeclaration, BusSlice};

use crate::state::{
    AnalysisResult, AnalysisType, Component, ComponentType, CrossSheetPortDirection, DcOpResult,
    Junction, LibraryCellInstance, OperatingPointValue, Point, SchematicAnnotationVisibility,
    SimulationRun, Wire,
};

use super::AppState;

/// The canvas, in points.
pub(super) const SCREEN: egui::Vec2 = egui::vec2(960.0, 640.0);
/// An ISO A1 sheet in world units (841 mm by 594 mm at 4 wu/mm).
const A1_WORLD: (f64, f64) = (3364.0, 2376.0);
/// The canvas of a desktop window, as the frame-cost probe sizes it.
const DESKTOP_CANVAS: egui::Vec2 = egui::vec2(1600.0, 1000.0);

/// Hands out object ids in order.
struct Ids(u64);

impl Ids {
    fn next(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }
}

fn part(ids: &mut Ids, kind: ComponentType, at: (i32, i32), name: &str, value: &str) -> Component {
    let mut component = Component::new(ids.next(), kind, Point::new(at.0, at.1));
    component.name = name.to_owned();
    component.value = value.to_owned();
    component
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

fn label(ids: &mut Ids, at: (i32, i32), name: &str) -> NetLabel {
    NetLabel::new(ids.next(), Point::new(at.0, at.1), name)
}

/// Finish a fixture the way a load does: runtime state, topology, and the
/// workspace's copy of the active schematic.
fn settle(state: &mut AppState) {
    state.schematic.recalculate_runtime_state();
    state.schematic.bump_topology_version();
    state.sync_active_schematic_to_workspace();
}

/// Where the analog sheet is reviewed from: its middle, and the diode tie of
/// its mirror.
pub(super) const ANALOG_CENTRE: Point = Point::new(510, 300);
const ANALOG_DETAIL: Point = Point::new(330, 220);
/// Where the hierarchy sheet is reviewed from: its middle, and the two
/// connectors between its instances with their badges.
pub(super) const HIERARCHY_CENTRE: Point = Point::new(470, 330);
const HIERARCHY_DETAIL: Point = Point::new(450, 250);

/// A five-transistor OTA with its supplies, compensation, load and a probe;
/// a four-bit bus with its declaration and four taps into a resistor array;
/// T-junctions, a crossing with a dot and two crossings without; local net
/// labels; a note. One PMOS load (M3) and the bias conductor are selected,
/// and the tail device's bulk is left open. M2, M3 and M5 are mirrored to
/// face left.
///
/// Spans world x 100 to 920 and y 40 to 520.
pub(super) fn analog_sheet() -> AppState {
    use ComponentType::{Capacitor, CurrentSource, Ground, Nmos, Pmos, Resistor, VoltageSource};

    let mut ids = Ids(0);
    let mut components = Vec::new();
    let mut wires = Vec::new();
    let mut junctions = Vec::new();
    let mut labels = Vec::new();

    // The stage. MOS pins sit at D (20, -40), G (-20, 0), S (20, 40) and
    // B (20, 0) before mirroring.
    let m3 = part(&mut ids, Pmos, (300, 180), "M3", "pch W=4u L=1u")
        .with_mirror_h(true)
        .with_mirror_v(true);
    let selected_part = m3.id;
    components.push(m3);
    components.push(part(&mut ids, Pmos, (500, 180), "M4", "pch W=4u L=1u").with_mirror_v(true));
    components.push(part(&mut ids, Nmos, (260, 300), "M1", "nch W=8u L=0.5u"));
    components.push(part(&mut ids, Nmos, (540, 300), "M2", "nch W=8u L=0.5u").with_mirror_h(true));
    components.push(part(&mut ids, Nmos, (420, 400), "M5", "nch W=4u L=2u").with_mirror_h(true));
    // Supplies and stimulus.
    components.push(part(&mut ids, VoltageSource, (160, 180), "VDD", "1.8"));
    components.push(part(&mut ids, Ground, (160, 230), "", ""));
    components.push(part(&mut ids, VoltageSource, (140, 340), "VIP", "0.9"));
    components.push(part(&mut ids, Ground, (140, 380), "", ""));
    components.push(part(&mut ids, VoltageSource, (140, 460), "VIN", "0.9"));
    components.push(part(&mut ids, Ground, (140, 510), "", ""));
    components.push(part(&mut ids, CurrentSource, (660, 440), "IB", "20u"));
    components.push(part(&mut ids, Ground, (660, 480), "", ""));
    components.push(part(&mut ids, Ground, (400, 470), "", ""));
    // Compensation and load.
    components.push(part(&mut ids, Capacitor, (600, 300), "CC", "1p").with_rotation(Rotation::R90));
    components.push(part(&mut ids, Ground, (600, 340), "", ""));
    components
        .push(part(&mut ids, Resistor, (700, 300), "RL", "100k").with_rotation(Rotation::R90));
    components.push(part(&mut ids, Ground, (700, 340), "", ""));
    // The bus's resistor array.
    for (bit, y) in [160, 240, 320, 400].into_iter().enumerate() {
        components.push(part(
            &mut ids,
            Resistor,
            (860, y),
            &format!("RD{bit}"),
            "10k",
        ));
    }
    components.push(part(&mut ids, Ground, (880, 450), "", ""));

    // VDD rail and the loads' sources and bulks.
    wires.push(wire(&mut ids, &[(160, 160), (160, 100), (680, 100)]));
    wires.push(wire(&mut ids, &[(280, 140), (280, 100)]));
    wires.push(wire(&mut ids, &[(520, 140), (520, 100)]));
    wires.push(wire(&mut ids, &[(280, 180), (240, 180), (240, 100)]));
    wires.push(wire(&mut ids, &[(520, 180), (560, 180), (560, 100)]));
    for at in [(240, 100), (280, 100), (520, 100), (560, 100)] {
        junctions.push(junction(&mut ids, at));
    }
    labels.push(label(&mut ids, (640, 100), "VDD"));
    wires.push(wire(&mut ids, &[(160, 200), (160, 220)]));
    // The mirror: gates tied, M3 diode-connected.
    wires.push(wire(&mut ids, &[(320, 180), (480, 180)]));
    wires.push(wire(&mut ids, &[(360, 180), (360, 240), (280, 240)]));
    wires.push(wire(&mut ids, &[(280, 220), (280, 260)]));
    junctions.push(junction(&mut ids, (360, 180)));
    junctions.push(junction(&mut ids, (280, 240)));
    // The output: M4's drain to the load, with a crossing that carries a dot
    // where the compensation leg passes through it.
    wires.push(wire(&mut ids, &[(520, 220), (520, 260)]));
    wires.push(wire(&mut ids, &[(520, 240), (700, 240), (700, 280)]));
    wires.push(wire(&mut ids, &[(600, 200), (600, 280)]));
    junctions.push(junction(&mut ids, (520, 240)));
    junctions.push(junction(&mut ids, (600, 240)));
    labels.push(label(&mut ids, (660, 240), "OUT"));
    wires.push(wire(&mut ids, &[(600, 320), (600, 330)]));
    wires.push(wire(&mut ids, &[(700, 320), (700, 330)]));
    // The tail and the pair's bulks.
    wires.push(wire(
        &mut ids,
        &[(280, 340), (280, 360), (520, 360), (520, 340)],
    ));
    wires.push(wire(&mut ids, &[(280, 300), (300, 300), (300, 360)]));
    wires.push(wire(&mut ids, &[(520, 300), (500, 300), (500, 360)]));
    junctions.push(junction(&mut ids, (300, 360)));
    junctions.push(junction(&mut ids, (400, 360)));
    junctions.push(junction(&mut ids, (500, 360)));
    wires.push(wire(&mut ids, &[(400, 440), (400, 460)]));
    // Inputs. INN runs down past the bias line, which crosses it with no dot.
    wires.push(wire(&mut ids, &[(240, 300), (140, 300), (140, 320)]));
    labels.push(label(&mut ids, (200, 300), "INP"));
    wires.push(wire(&mut ids, &[(140, 360), (140, 370)]));
    wires.push(wire(&mut ids, &[(560, 300), (580, 300), (580, 520)]));
    labels.push(label(&mut ids, (580, 500), "INN"));
    wires.push(wire(&mut ids, &[(140, 440), (140, 420), (220, 420)]));
    labels.push(label(&mut ids, (200, 420), "INN"));
    wires.push(wire(&mut ids, &[(140, 480), (140, 500)]));
    // The bias line from M5's gate to the bias source.
    let bias = wire(&mut ids, &[(440, 400), (660, 400), (660, 420)]);
    let selected_wire = bias.id;
    wires.push(bias);
    labels.push(label(&mut ids, (620, 400), "VB"));
    wires.push(wire(&mut ids, &[(660, 460), (660, 470)]));

    // The bus, its taps and the array they feed.
    let declaration = BusDeclaration::parse("D[3:0]").expect("bus declaration");
    let bus = Bus::new(
        ids.next(),
        vec![Point::new(780, 120), Point::new(780, 440)],
        Some(declaration),
    )
    .expect("bus");
    let mut taps = Vec::new();
    for (bit, y) in [160, 240, 320, 400].into_iter().enumerate() {
        taps.push(
            BusTap::new(
                ids.next(),
                &bus,
                Point::new(780, y),
                Point::new(800, y),
                BusSlice::parse(&format!("D[{bit}]")).expect("tap slice"),
                BusTapOrientation::Right,
            )
            .expect("bus tap"),
        );
        wires.push(wire(&mut ids, &[(800, y), (840, y)]));
    }
    wires.push(wire(&mut ids, &[(880, 160), (880, 440)]));
    for y in [240, 320, 400] {
        junctions.push(junction(&mut ids, (880, y)));
    }

    let probe =
        SchematicProbe::new(ids.next(), Point::new(700, 260), "V(OUT)", None).expect("probe");
    let note = DesignNote::new(
        ids.next(),
        Point::new(120, 40),
        DesignNoteKind::PlainText,
        "Five-transistor OTA, gm/Id = 12 at the input pair",
    )
    .expect("note");

    let mut state = AppState::default();
    let document = state.schematic.document_mut_for_test();
    document.components = components;
    document.wires = wires;
    document.junctions = junctions;
    document.net_labels = labels;
    document.buses = vec![bus];
    document.bus_taps = taps;
    document.probes = vec![probe];
    document.design_notes = vec![note];
    settle(&mut state);
    state.ui.schematic_visibility.annotations = SchematicAnnotationVisibility::Hidden;
    let selection = &mut state.schematic.session.editor.selection;
    selection.select_component(selected_part);
    selection.select_wire(selected_wire);
    state
}

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
        binding.bind_interface(&super::frame_cost::master_ports());
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
        .library_manager
        .add_library(super::frame_cost::master_library());
    let document = state.schematic.document_mut_for_test();
    document.components = components;
    document.wires = wires;
    document.junctions = junctions;
    document.net_labels = labels;
    settle(&mut state);

    super::frame_cost::install_projected_run_geometry(&mut state, nets);
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
    state.simulation.retained.runs.insert(0, run);
    state.simulation.view.active_run_idx = Some(0);
    state.simulation.view.active_analysis_idx = Some(0);
    state.ui.schematic_visibility.annotations = SchematicAnnotationVisibility::OperatingPoint;
    state
        .schematic
        .session
        .editor
        .selection
        .select_component(selected_id);
    state
}

/// How a render frames its sheet.
#[derive(Clone, Copy)]
pub(super) enum Zoom {
    /// Points per world unit.
    Fixed(f64),
    /// The zoom at which an A1 sheet fits the canvas.
    FitA1,
}

#[derive(Clone, Copy)]
pub(super) struct Framing {
    pub(super) name: &'static str,
    pub(super) zoom: Zoom,
    /// The world point at the middle of the canvas.
    pub(super) centre: Point,
}

/// The zoom at which an A1 sheet fits a desktop canvas, as the canvas's own
/// framing computes it: about 0.4. (On the smaller review canvas it would be
/// 0.25 again.)
pub(super) fn a1_fit_zoom() -> f64 {
    let mut probe = crate::state::SchematicState::default();
    probe.zoom_to_fit_world_rect(
        (0.0, 0.0, A1_WORLD.0, A1_WORLD.1),
        f64::from(DESKTOP_CANVAS.x),
        f64::from(DESKTOP_CANVAS.y),
        rspice_schematic_editor::view::drawing_sheet::FIT_SCREEN_INSET,
        0.0,
    );
    probe.session.editor.zoom
}

/// One rendered frame.
pub(super) struct Render {
    pub(super) canvas: crate::ui::raster::Canvas,
    /// What the last frame painted, in points.
    pub(super) shapes: Vec<ClippedShape>,
    pub(super) pixels_per_point: f32,
    pub(super) zoom: f64,
    pub(super) tokens: std::sync::Arc<crate::ui::tokens::Tokens>,
}

/// The dark and the light canvas.
pub(super) fn themes() -> [(&'static str, crate::ui::Theme); 2] {
    [
        ("dark", crate::ui::Theme::default()),
        (
            "light",
            crate::ui::Theme {
                mode: rspice_ui_kit::Mode::Light,
                canvas_theme: rspice_ui_kit::EngineeringCanvasTheme::Light,
                ..crate::ui::Theme::default()
            },
        ),
    ]
}

/// Render `state` through the real canvas at `framing`, under `theme`, on a
/// display of `pixels_per_point`.
pub(super) fn render(
    state: &mut AppState,
    theme: crate::ui::Theme,
    framing: Framing,
    pixels_per_point: f32,
) -> Render {
    render_pointing(state, theme, framing, pixels_per_point, None)
}

/// [`render`] with the pointer resting at `pointer` (in points), so the
/// canvas draws whatever it pre-highlights there.
pub(super) fn render_pointing(
    state: &mut AppState,
    theme: crate::ui::Theme,
    framing: Framing,
    pixels_per_point: f32,
    pointer: Option<egui::Pos2>,
) -> Render {
    let zoom = match framing.zoom {
        Zoom::Fixed(zoom) => zoom,
        Zoom::FitA1 => a1_fit_zoom(),
    };
    let ctx = egui::Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    theme.apply(&ctx);
    let symbols = super::super::symbols::SymbolLibrary::load_embedded().ok();
    let mut output = None;
    for _ in 0..3 {
        let editor = &mut state.schematic.session.editor;
        editor.needs_fit = false;
        editor.needs_drawing_sheet_fit = false;
        editor.zoom = zoom;
        editor.pan = (
            f64::from(SCREEN.x) * 0.5 - f64::from(framing.centre.x) * zoom,
            f64::from(SCREEN.y) * 0.5 - f64::from(framing.centre.y) * zoom,
        );
        let mut input = crate::ui::raster::display_input(SCREEN, pixels_per_point);
        input.events.extend(pointer.map(egui::Event::PointerMoved));
        output = Some(ctx.run_ui(input, |ui| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ui, |ui| {
                    super::render_schematic_view(ui, state, symbols.as_ref());
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
        canvas: crate::ui::raster::rasterize(&ctx, SCREEN, &output),
        shapes: output.shapes,
        pixels_per_point,
        zoom,
        tokens: crate::ui::tokens::Tokens::get(&ctx),
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

/// One painted run of text: its colour, size, family and layout box.
pub(super) struct PaintedText {
    pub(super) text: String,
    pub(super) colour: Color32,
    pub(super) size: f32,
    pub(super) family: egui::FontFamily,
    pub(super) rect: Rect,
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
                size: section.format.font_id.size,
                family: section.format.font_id.family.clone(),
                rect,
            })
        })
        .collect()
}

fn hex(colour: Color32) -> String {
    let [r, g, b, a] = colour.to_srgba_unmultiplied();
    if a == 255 {
        format!("#{r:02X}{g:02X}{b:02X}")
    } else {
        format!("#{r:02X}{g:02X}{b:02X}@{a}")
    }
}

fn relative_luminance(colour: Color32) -> f64 {
    let linear = |channel: u8| {
        let channel = f64::from(channel) / 255.0;
        if channel <= 0.040_45 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * linear(colour.r()) + 0.7152 * linear(colour.g()) + 0.0722 * linear(colour.b())
}

/// WCAG 2.x contrast of `ink` composited over an opaque `ground`.
pub(super) fn contrast(ink: Color32, ground: Color32) -> f64 {
    let alpha = f32::from(ink.a()) / 255.0;
    // `Color32` is premultiplied: over an opaque ground, src + dst * (1 - a).
    let over = |src: u8, dst: u8| {
        (f32::from(src) + f32::from(dst) * (1.0 - alpha))
            .round()
            .clamp(0.0, 255.0) as u8
    };
    let composed = Color32::from_rgb(
        over(ink.r(), ground.r()),
        over(ink.g(), ground.g()),
        over(ink.b(), ground.b()),
    );
    let (a, b) = (relative_luminance(composed), relative_luminance(ground));
    let (bright, dark) = if a > b { (a, b) } else { (b, a) };
    (bright + 0.05) / (dark + 0.05)
}

/// The measurement table a review reads: stroke widths and circle radii by
/// colour, and text sizes by colour with their contrast on the canvas.
pub(super) fn measurement_table(render: &Render) -> String {
    let canvas_bg = render.tokens.color.canvas_bg;
    let mut strokes: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut circles: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let add = |table: &mut BTreeMap<String, BTreeMap<String, usize>>, key, value| {
        *table.entry(key).or_default().entry(value).or_default() += 1;
    };
    let rgb = |colour: Color32| {
        let [r, g, b, _] = colour.to_srgba_unmultiplied();
        [r, g, b]
    };
    let named = |colour: Color32| {
        let c = &render.tokens.color;
        let tokens = [
            ("wire", c.wire),
            ("symbol", c.symbol),
            ("net_label", c.net_label),
            ("text", c.text),
            ("text_dim", c.text_dim),
            ("text_faint", c.text_faint),
            ("canvas_select", c.canvas_select),
            ("canvas_grid", c.canvas_grid),
            ("canvas_bg", c.canvas_bg),
            ("info", c.info),
            ("accent", c.accent),
            ("border", c.border),
        ];
        tokens
            .iter()
            .find(|(_, token)| colour.a() > 0 && rgb(*token) == rgb(colour))
            .map_or_else(
                || hex(colour),
                |(name, _)| {
                    if colour.a() == 255 {
                        (*name).to_owned()
                    } else {
                        format!("{name}@{}", colour.a())
                    }
                },
            )
    };
    for (_, shape) in leaf_shapes(&render.shapes) {
        match shape {
            Shape::LineSegment { stroke, .. } => add(
                &mut strokes,
                format!("line {}", named(stroke.color)),
                format!("{:.2}", stroke.width),
            ),
            Shape::Path(path) => {
                if path.stroke.width > 0.0
                    && let egui::epaint::ColorMode::Solid(colour) = path.stroke.color
                {
                    add(
                        &mut strokes,
                        format!(
                            "path{} {}",
                            if path.closed { " closed" } else { "" },
                            named(colour)
                        ),
                        format!("{:.2}", path.stroke.width),
                    );
                }
            }
            Shape::Circle(circle) => add(
                &mut circles,
                format!(
                    "circle fill {} stroke {}",
                    named(circle.fill),
                    named(circle.stroke.color)
                ),
                format!("r {:.2} w {:.2}", circle.radius, circle.stroke.width),
            ),
            Shape::Rect(rect) => add(
                &mut strokes,
                format!(
                    "rect fill {} stroke {}",
                    named(rect.fill),
                    named(rect.stroke.color)
                ),
                format!(
                    "{:.1}x{:.1} w {:.2}",
                    rect.rect.width(),
                    rect.rect.height(),
                    rect.stroke.width
                ),
            ),
            _ => {}
        }
    }
    let mut texts: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    for text in painted_text(&render.shapes) {
        add(
            &mut texts,
            format!(
                "text {} ({:.2}:1) {:?}",
                named(text.colour),
                contrast(text.colour, canvas_bg),
                text.family
            ),
            format!("{:.2} pt", text.size),
        );
    }
    let mut out = format!(
        "zoom {:.4} pt/wu, {} px/pt, canvas {}\n",
        render.zoom,
        render.pixels_per_point,
        hex(canvas_bg)
    );
    for (title, table) in [("strokes", strokes), ("marks", circles), ("text", texts)] {
        let _ = writeln!(out, "-- {title}");
        for (key, values) in table {
            let values = values
                .iter()
                .map(|(value, count)| format!("{value} x{count}"))
                .collect::<Vec<_>>()
                .join(", ");
            let _ = writeln!(out, "{key}: {values}");
        }
    }
    out
}

/// The review set: each sheet at zoom 0.25, at the zoom that fits A1, at 1
/// and at 4; dark and light; at 1x and 2x. Writes `<sheet>-<zoom>-<theme>-
/// <density>.png` and a `.txt` measurement table beside each into
/// `RSPICE_RASTER_DIR`.
#[test]
#[ignore = "writes PNGs and measurement tables for a person to review; run with --ignored"]
fn render_the_canvas_raster_review_set() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let only = std::env::var("RSPICE_REVIEW_ONLY").ok();
    let stderr = std::io::stderr();
    let mut report = stderr.lock();

    let framings = |whole: Point, detail: Point| {
        [
            Framing {
                name: "z0.25",
                zoom: Zoom::Fixed(0.25),
                centre: whole,
            },
            Framing {
                name: "fitA1",
                zoom: Zoom::FitA1,
                centre: whole,
            },
            Framing {
                name: "z1",
                zoom: Zoom::Fixed(1.0),
                centre: whole,
            },
            Framing {
                name: "z4",
                zoom: Zoom::Fixed(4.0),
                centre: detail,
            },
        ]
    };
    type Sheet = (&'static str, fn() -> AppState, [Framing; 4]);
    let sheets: [Sheet; 3] = [
        (
            "analog",
            analog_sheet,
            framings(ANALOG_CENTRE, ANALOG_DETAIL),
        ),
        (
            "hierarchy",
            hierarchy_sheet,
            framings(HIERARCHY_CENTRE, HIERARCHY_DETAIL),
        ),
        (
            "scale",
            || super::frame_cost::scale_fixture().state,
            framings(Point::new(1400, 1100), Point::new(1210, 1060)),
        ),
    ];
    for (sheet, build, framings) in sheets {
        if only.as_deref().is_some_and(|only| !only.contains(sheet)) {
            continue;
        }
        for (theme_name, theme) in themes() {
            for pixels_per_point in [1.0_f32, 2.0] {
                let mut state = build();
                for framing in framings {
                    let render = render(&mut state, theme, framing, pixels_per_point);
                    let name = format!(
                        "{sheet}-{}-{theme_name}-{}x",
                        framing.name, pixels_per_point as u32
                    );
                    let png = directory.join(format!("{name}.png"));
                    std::fs::write(&png, render.canvas.png(render.canvas.height()))
                        .expect("write the render");
                    std::fs::write(
                        directory.join(format!("{name}.txt")),
                        measurement_table(&render),
                    )
                    .expect("write the measurements");
                    writeln!(report, "{}", png.display()).ok();
                }
            }
        }
    }
}

/// Where `world` lands on a render framed at `centre` and `zoom`.
fn on_screen(centre: Point, zoom: f64, world: Point) -> egui::Pos2 {
    egui::pos2(
        SCREEN.x * 0.5 + ((f64::from(world.x) - f64::from(centre.x)) * zoom) as f32,
        SCREEN.y * 0.5 + ((f64::from(world.y) - f64::from(centre.y)) * zoom) as f32,
    )
}

fn at_zoom(zoom: f64, centre: Point) -> Framing {
    Framing {
        name: "test",
        zoom: Zoom::Fixed(zoom),
        centre,
    }
}

/// A transistor letters its value on the side of its body away from its
/// gate. A differential pair is always drawn with one half mirrored, and the
/// mirrored half's value stood on its gate side, across the gate lead and the
/// wire leaving it: on the analog sheet M2's value ran over its input wire
/// and the compensation capacitor's labels, M3's over the mirror's gate tie
/// and M5's over the bias line.
#[test]
fn a_transistor_letters_its_value_away_from_its_gate() {
    let mut state = analog_sheet();
    let framing = at_zoom(1.0, ANALOG_CENTRE);
    let render = render(&mut state, themes()[0].1, framing, 1.0);
    let values = painted_text(&render.shapes)
        .into_iter()
        .filter(|text| text.text.contains("W="))
        .collect::<Vec<_>>();
    let transistors = state
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| matches!(component.kind, ComponentType::Nmos | ComponentType::Pmos))
        .collect::<Vec<_>>();
    assert_eq!(transistors.len(), 5);
    assert_eq!(values.len(), 5, "every transistor letters its value");
    for transistor in transistors {
        let body = on_screen(framing.centre, render.zoom, transistor.pos);
        let gate = transistor
            .terminal_positions()
            .into_iter()
            .find(|(name, _)| *name == "G")
            .map(|(_, at)| on_screen(framing.centre, render.zoom, at))
            .expect("a gate");
        let value = values
            .iter()
            .min_by(|a, b| {
                a.rect
                    .center()
                    .distance(body)
                    .total_cmp(&b.rect.center().distance(body))
            })
            .expect("a value");
        let away = if gate.x > body.x {
            value.rect.max.x <= body.x
        } else {
            value.rect.min.x >= body.x
        };
        assert!(
            away,
            "{}: the value {:?} at {:?} stands on the gate's side (gate at {gate:?}, body at {body:?})",
            transistor.name, value.text, value.rect
        );
    }
}

/// An operating-point badge reads like an annotation: four significant
/// digits, the SI prefix joined to its unit and micro spelled `µ` (D1
/// section 8), so `912.4 mV`, not the property-editor spelling `912.430m V`.
#[test]
fn an_operating_point_badge_reads_four_digits_and_a_prefixed_unit() {
    let mut state = hierarchy_sheet();
    let render = render(
        &mut state,
        themes()[0].1,
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
        "in 900 mV",
        "out 912.4 mV",
    ]
    .map(str::to_owned)
    .into_iter()
    .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(badges, expected);
}

/// An off-sheet connector's tab is stroked in whole device pixels, like the
/// symbol beside it. It was stroked in proportion to its text's height, which
/// at 100 % on a 1x display is 1.19 px: a soft two-pixel smear beside crisp
/// one-pixel wires.
#[test]
fn an_off_sheet_connector_tab_is_stroked_in_whole_device_pixels() {
    for (zoom, pixels_per_point) in [(1.0, 1.0_f32), (4.0, 2.0)] {
        let mut state = hierarchy_sheet();
        let render = render(
            &mut state,
            themes()[0].1,
            at_zoom(zoom, HIERARCHY_DETAIL),
            pixels_per_point,
        );
        let net_label = render.tokens.color.net_label;
        let widths = leaf_shapes(&render.shapes)
            .into_iter()
            .filter_map(|(_, shape)| match shape {
                Shape::Rect(rect) if rect.stroke.color == net_label => Some(rect.stroke.width),
                Shape::LineSegment { stroke, .. } if stroke.color == net_label => {
                    Some(stroke.width)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(!widths.is_empty(), "zoom {zoom}: no connector tab painted");
        for width in widths {
            let device = width * pixels_per_point;
            assert!(
                device >= 1.0 && (device - device.round()).abs() < 1e-3,
                "zoom {zoom} at {pixels_per_point}x: a tab stroke is {device} device pixels"
            );
        }
    }
}

/// A named net's badge stands on the middle of its label's name (D1 section
/// 8: right of the label's text rect, vertically centred on it). It was
/// centred on the label's whole bounds, which reach down to the wire the
/// label hangs from, so it sat 3 wu low: 12 pt at 400 %, level with the
/// name's baseline rather than its middle.
#[test]
fn a_named_net_badge_is_centred_on_its_label_name() {
    for zoom in [1.0, 4.0] {
        let mut state = hierarchy_sheet();
        let render = render(
            &mut state,
            themes()[0].1,
            at_zoom(zoom, HIERARCHY_DETAIL),
            1.0,
        );
        let texts = painted_text(&render.shapes);
        let find = |text: &str| {
            texts
                .iter()
                .find(|painted| painted.text == text)
                .unwrap_or_else(|| panic!("zoom {zoom}: {text:?} is not painted"))
                .rect
        };
        for (name, badge) in [("MID", "1.213 V"), ("NBIAS", "651.2 mV")] {
            let (name_rect, badge_rect) = (find(name), find(badge));
            assert!(
                badge_rect.min.x > name_rect.max.x,
                "zoom {zoom}: the {name} badge stands right of its name"
            );
            let offset = badge_rect.center().y - name_rect.center().y;
            assert!(
                offset.abs() <= 0.5,
                "zoom {zoom}: the {name} badge's middle is {offset} pt below its name's"
            );
        }
    }
}

/// The hierarchy sheet's ports: each one's name and position.
fn ports(state: &AppState) -> Vec<(String, Point)> {
    state
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::Port)
        .map(|component| (component.value.clone(), component.pos))
        .collect()
}

/// A port's name is an electrical name (D1 section 1): lettered as an
/// off-sheet connector's name is, in Plex Mono Medium at 11 wu in the
/// net-label ink, and greeked to a bar below the legibility floor rather
/// than vanishing. It was lettered as an instance value, in 9-wu Plex Sans
/// and the dim ink, so at the A1 fit every port's name disappeared.
#[test]
fn a_port_name_letters_and_greeks_as_a_net_name() {
    let mut state = hierarchy_sheet();
    let ports = ports(&state);
    assert_eq!(ports.len(), 4);

    let framing = at_zoom(1.0, HIERARCHY_CENTRE);
    let close = render(&mut state, themes()[0].1, framing, 1.0);
    let texts = painted_text(&close.shapes);
    let connector = texts
        .iter()
        .find(|text| text.text == "MID")
        .expect("the MID connector's name is painted");
    for (name, at) in &ports {
        let anchor = on_screen(framing.centre, close.zoom, *at);
        let painted = texts
            .iter()
            .filter(|text| text.text == *name)
            .min_by(|a, b| {
                a.rect
                    .center()
                    .distance(anchor)
                    .total_cmp(&b.rect.center().distance(anchor))
            })
            .unwrap_or_else(|| panic!("{name}: the port's name is painted"));
        assert!(
            painted.rect.center().distance(anchor) < 30.0,
            "{name}: the nearest {name:?} at {:?} is not the port's",
            painted.rect
        );
        assert_eq!(
            (painted.colour, painted.size, &painted.family),
            (connector.colour, connector.size, &connector.family),
            "{name}: the port's name is lettered as a net name"
        );
    }

    let framing = Framing {
        name: "test",
        zoom: Zoom::FitA1,
        centre: HIERARCHY_CENTRE,
    };
    let fit = render(&mut state, themes()[0].1, framing, 1.0);
    let greeked = fit
        .tokens
        .color
        .net_label
        .gamma_multiply(rspice_schematic_editor::view::lod::GREEK_ALPHA);
    let bars = leaf_shapes(&fit.shapes)
        .into_iter()
        .filter_map(|(_, shape)| match shape {
            Shape::Rect(rect) if rect.fill == greeked => Some(rect.rect),
            _ => None,
        })
        .collect::<Vec<_>>();
    for (name, at) in &ports {
        let anchor = on_screen(framing.centre, fit.zoom, *at);
        let near = bars
            .iter()
            .filter(|bar| bar.center().distance(anchor) < 30.0 * fit.zoom as f32)
            .count();
        assert_eq!(
            near, 1,
            "zoom {:.3}: {name}'s name greeks to one bar beside the port (bars: {bars:?})",
            fit.zoom
        );
    }
}

/// Every run of text the canvas paints is legible (D1 section 1: at least
/// 6 device pixels and 3.5 pt), at the zoom that fits A1 and at 0.25, on 1x
/// and 2x. A design note kept its own 4-pt floor, compared in points: at the
/// A1 fit on a 1x display it painted its text 4.75 px tall.
#[test]
fn no_canvas_text_is_painted_below_the_legibility_floor() {
    for (zoom, pixels_per_point) in [
        (Zoom::FitA1, 1.0_f32),
        (Zoom::FitA1, 2.0),
        (Zoom::Fixed(0.25), 2.0),
    ] {
        let mut state = analog_sheet();
        let framing = Framing {
            name: "test",
            zoom,
            centre: ANALOG_CENTRE,
        };
        let render = render(&mut state, themes()[0].1, framing, pixels_per_point);
        for text in painted_text(&render.shapes) {
            assert!(
                text.size * pixels_per_point >= 6.0 - 1e-3 && text.size >= 3.5 - 1e-3,
                "zoom {:.3} at {pixels_per_point}x: {:?} is painted at {} pt",
                render.zoom,
                text.text,
                text.size
            );
        }
    }
}

/// A selected design note keeps its ink and takes the canvas selection's
/// rect, as a selected net label does (D1 section 2: text never takes the
/// selection colour, and canvas states use the canvas tokens, not the
/// interface accent). Its text turned the accent, inside an accent rect.
#[test]
fn a_selected_note_keeps_its_ink_inside_the_canvas_selection() {
    let mut state = analog_sheet();
    let note = state.schematic.document().design_notes[0].id;
    let selection = &mut state.schematic.session.editor.selection;
    selection.clear();
    selection.design_notes.insert(note);
    let render = render(&mut state, themes()[0].1, at_zoom(1.0, ANALOG_CENTRE), 1.0);
    let colour = &render.tokens.color;
    let text = painted_text(&render.shapes)
        .into_iter()
        .find(|text| text.text.starts_with("Five-transistor OTA"))
        .expect("the note is painted");
    assert_eq!(text.colour, colour.text_dim, "the note keeps its ink");
    let framed = leaf_shapes(&render.shapes)
        .into_iter()
        .any(|(_, shape)| match shape {
            Shape::Rect(rect) => {
                rect.stroke.color == colour.canvas_select && rect.rect.contains_rect(text.rect)
            }
            _ => false,
        });
    assert!(framed, "the note stands in a canvas-selection rect");
}

/// A pin name stands clear of its lead, as a generated symbol letters it:
/// inside the body, past the lead's inner end. A symbol that draws its leads
/// as artwork (an imported LTspice or SVG symbol, and the hierarchy sheet's
/// masters, whose IN, OUT and BIAS leads are 10-wu body lines from the
/// terminal in) lettered each name from the terminal instead, on the lead,
/// which struck through it.
#[test]
fn a_pin_name_stands_clear_of_an_artwork_lead() {
    const MASTER_WIDTHS: [(&str, i32); 3] = [("amp", 40), ("buf", 50), ("bias", 60)];
    let mut state = hierarchy_sheet();
    let framing = at_zoom(2.0, HIERARCHY_CENTRE);
    let render = render(&mut state, themes()[0].1, framing, 1.0);
    let mut leads = Vec::new();
    for instance in &state.schematic.document().components {
        let Some(binding) = &instance.library_cell else {
            continue;
        };
        let width = MASTER_WIDTHS
            .iter()
            .find(|(master, _)| *master == binding.cell)
            .map(|(_, width)| *width)
            .expect("a master of the hierarchy sheet");
        let at = |(x, y): (i32, i32)| {
            on_screen(
                framing.centre,
                render.zoom,
                Point::new(instance.pos.x + x, instance.pos.y + y),
            )
        };
        for (pin, terminal, inner) in [
            ("IN", (-width, 0), (-width + 10, 0)),
            ("OUT", (width, 0), (width - 10, 0)),
            ("BIAS", (0, 40), (0, 30)),
        ] {
            leads.push((
                instance.name.clone(),
                pin,
                Rect::from_two_pos(at(terminal), at(inner)).expand(1.0),
            ));
        }
    }
    let pin_size = rspice_schematic_editor::view::lod::quantize_font_size(
        rspice_design::symbol_generation::GENERATED_PIN_LABEL_SIZE * render.zoom as f32,
    );
    let names = painted_text(&render.shapes)
        .into_iter()
        .filter(|text| text.size == pin_size && ["IN", "OUT", "BIAS"].contains(&text.text.as_str()))
        .collect::<Vec<_>>();
    assert_eq!(names.len(), 9, "every instance letters its three pin names");
    for name in &names {
        for (instance, pin, lead) in &leads {
            assert!(
                !name.rect.intersects(*lead),
                "the pin name {:?} at {:?} stands on {instance}'s {pin} lead at {lead:?}",
                name.text,
                name.rect
            );
        }
    }
}

/// A transistor's value stands clear of every pin's line, mirrored or not:
/// off the axis its gate and bulk leads run along, and outside the columns
/// its drain and source leads run up and down. An unmirrored device's value
/// stood on its bulk's axis, so the bulk wire ran under the text (M1's on the
/// analog sheet), and a mirrored device's on the same axis on its other side
/// (M3's). Hung below that axis, it still sat across the input pair's bulk
/// ties, which turn down toward the tail.
#[test]
fn a_transistor_value_stands_clear_of_every_pin_line() {
    let mut state = analog_sheet();
    let framing = at_zoom(1.0, ANALOG_CENTRE);
    let render = render(&mut state, themes()[0].1, framing, 1.0);
    let values = painted_text(&render.shapes)
        .into_iter()
        .filter(|text| text.text.contains("W="))
        .collect::<Vec<_>>();
    let transistors = state
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| matches!(component.kind, ComponentType::Nmos | ComponentType::Pmos))
        .collect::<Vec<_>>();
    assert_eq!(transistors.len(), 5);
    for transistor in transistors {
        let body = on_screen(framing.centre, render.zoom, transistor.pos);
        let value = values
            .iter()
            .min_by(|a, b| {
                a.rect
                    .center()
                    .distance(body)
                    .total_cmp(&b.rect.center().distance(body))
            })
            .expect("a value");
        for (pin, at) in transistor.terminal_positions() {
            let at = on_screen(framing.centre, render.zoom, at);
            // The gate and bulk leads run across, the drain and source leads
            // up and down.
            let crosses = match pin {
                "G" | "B" => value.rect.min.y < at.y && at.y < value.rect.max.y,
                _ => value.rect.min.x < at.x && at.x < value.rect.max.x,
            };
            assert!(
                !crosses,
                "{}: the value {:?} at {:?} stands on the line of its {pin} pin at {at:?}",
                transistor.name, value.text, value.rect
            );
        }
        // Nor does a wire leaving one of its pins run under it: the pair's
        // bulk ties turn down toward the tail 20 wu out from the body.
        let terminals = transistor
            .terminal_positions()
            .into_iter()
            .map(|(_, at)| at)
            .collect::<Vec<_>>();
        for wire in &state.schematic.document().wires {
            if !wire.points.iter().any(|point| terminals.contains(point)) {
                continue;
            }
            for pair in wire.points.windows(2) {
                let segment = Rect::from_two_pos(
                    on_screen(framing.centre, render.zoom, pair[0]),
                    on_screen(framing.centre, render.zoom, pair[1]),
                )
                .expand(0.5);
                assert!(
                    !segment.intersects(value.rect),
                    "{}: the value {:?} at {:?} stands on the wire from {:?} to {:?}",
                    transistor.name,
                    value.text,
                    value.rect,
                    pair[0],
                    pair[1]
                );
            }
        }
    }
}

/// A hovered junction's ring is stroked in whole device pixels at every
/// pixel density, as crisp as the wires it sits on. D1 section 6 gives it
/// 1.5 pt, which on a 1x display antialiased into a soft two-pixel ring.
#[test]
fn a_hovered_junction_ring_is_stroked_in_whole_device_pixels() {
    // The dot where the compensation leg crosses the output, clear of any
    // part's pins.
    let junction = Point::new(600, 240);
    for pixels_per_point in [1.0_f32, 1.5, 2.0] {
        let mut state = analog_sheet();
        let framing = at_zoom(1.0, ANALOG_CENTRE);
        let pointer = on_screen(framing.centre, 1.0, junction);
        let render = render_pointing(
            &mut state,
            themes()[0].1,
            framing,
            pixels_per_point,
            Some(pointer),
        );
        let select = render.tokens.color.canvas_select;
        let rings = leaf_shapes(&render.shapes)
            .into_iter()
            .filter_map(|(_, shape)| match shape {
                Shape::Circle(circle)
                    if circle.stroke.color == select
                        && circle.stroke.width > 0.0
                        && circle.center.distance(pointer) < 0.5 =>
                {
                    Some(circle.stroke.width)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            rings.len(),
            1,
            "{pixels_per_point}x: the hovered junction draws one ring"
        );
        let device = rings[0] * pixels_per_point;
        assert!(
            device >= 1.0 && (device - device.round()).abs() < 1e-3,
            "{pixels_per_point}x: the hover ring is {device} device pixels"
        );
    }
}
