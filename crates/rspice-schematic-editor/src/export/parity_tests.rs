//! The exported sheet against the canvas (C-10, D1 section 11): one fixture
//! sheet, painted by the canvas's scene painter and written as SVG, carries
//! the same labels, the same dots and the same anchors.

use egui::{Context, Id, LayerId, Order, Painter, Pos2, Rect, Shape, Vec2};
use rspice_design::schematic::{
    component::Component,
    component_display::SchematicParameterLabelVisibility,
    component_type::ComponentType,
    document::SchematicDocument,
    net_label::{Junction, NetLabel},
    probe::SchematicProbe,
    rotation::Rotation,
    wire::Wire,
};
use rspice_design_model::{
    Point,
    design_management::{SheetCatalog, SheetDefinition, SheetPortPolicy, SheetTemplate},
};

use super::{SvgDesignContext, SvgExportConfig, export_to_svg_with_resolved_symbol_entries};
use crate::session::{
    EditorSession,
    selection::SchematicSelectionFilter,
    visibility::{SchematicNetHighlighting, SchematicReviewMarkerVisibility},
};
use crate::symbols::SymbolLibrary;
use crate::view::{
    design_view::DesignView,
    drawing::ProbeVisualStatus,
    scene::{SceneOverlays, SceneSymbols, SceneView, draw_content},
    symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};

/// A T-junction with a net label on it, a resistor upright and one turned a
/// quarter, and a probe.
fn fixture_document() -> SchematicDocument {
    let mut upright = Component::new(1, ComponentType::Resistor, Point::new(0, 100));
    upright.name = "R1".to_owned();
    upright.value = "10k".to_owned();
    let mut turned = Component::new(2, ComponentType::Resistor, Point::new(120, 60));
    turned.name = "R2".to_owned();
    turned.value = "4.7k".to_owned();
    turned.rotation = Rotation::R90;
    SchematicDocument {
        components: vec![upright, turned],
        wires: vec![
            Wire::segment(3, Point::new(-60, 0), Point::new(120, 0)),
            Wire::segment(4, Point::new(0, 0), Point::new(0, 60)),
        ],
        junctions: vec![Junction::new(5, Point::new(0, 0))],
        net_labels: vec![NetLabel::new(6, Point::new(-40, 0), "VOUT")],
        probes: vec![SchematicProbe::new(7, Point::new(60, 0), "V(OUT)", None).expect("probe")],
        ..SchematicDocument::default()
    }
}

fn export(document: &SchematicDocument, context: SvgDesignContext<'_>) -> String {
    export_to_svg_with_resolved_symbol_entries(document, &SvgExportConfig::default(), &[], context)
}

/// D1's parity zoom: section 1's policy is the identity there for wires,
/// symbols, dots and text.
const ZOOM: f32 = 2.0;
const ORIGIN: Vec2 = Vec2::new(400.0, 400.0);

/// A canvas point in world units.
fn world(point: Pos2) -> Pos2 {
    ((point.to_vec2() - ORIGIN) / ZOOM).to_pos2()
}

/// Everything the scene painter drew for the fixture, flattened.
fn canvas_shapes(document: &SchematicDocument) -> Vec<Shape> {
    let ctx = Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    let _ = ctx.run_ui(egui::RawInput::default(), |_| {});
    let bounds = Rect::from_min_size(Pos2::ZERO, Vec2::splat(1200.0));
    let painter = Painter::new(
        ctx,
        LayerId::new(Order::Foreground, Id::new("svg-parity")),
        bounds,
    );
    let library = SymbolLibrary::load_embedded().expect("symbol library");
    let context = SchematicSymbolContext::default();
    let editor = EditorSession::default();
    draw_content(
        &painter,
        bounds,
        &Viewport {
            offset: ORIGIN.to_pos2(),
            zoom: ZOOM,
            bounds,
        },
        &SceneView {
            design: DesignView {
                document,
                canvas_cache: None,
                sheet_catalog: None,
                review_markers: SchematicReviewMarkerVisibility::All,
            },
            editor: &editor,
            net_highlighting: SchematicNetHighlighting::Off,
            parameter_labels: SchematicParameterLabelVisibility::NamesAndValues,
            hover_wire_vertex: None,
            selection_filter: SchematicSelectionFilter::default(),
            hover: None,
        },
        SceneSymbols {
            library: Some(&library),
            context: &context,
        },
        SceneOverlays {
            net_class_colors: std::collections::HashMap::new(),
            operating_point: Vec::new(),
            operating_point_layout: None,
            probe_status: &|_| ProbeVisualStatus::Materialized,
            selected_trace_expression: &|| None,
            open_pins: &std::collections::HashMap::new(),
        },
        String::new,
    );
    let mut shapes = Vec::new();
    painter.for_each_shape(|clipped| flatten(&clipped.shape, &mut shapes));
    shapes
}

fn flatten(shape: &Shape, out: &mut Vec<Shape>) {
    match shape {
        Shape::Vec(shapes) => {
            for shape in shapes {
                flatten(shape, out);
            }
        }
        other => out.push(other.clone()),
    }
}

/// One `<text>` run on the page: its class, string, anchor and baseline in
/// world units.
#[derive(Debug)]
struct PageText {
    class: String,
    text: String,
    anchor: String,
    x: f32,
    baseline: f32,
}

fn attribute<'a>(line: &'a str, name: &str) -> Option<&'a str> {
    let start = line.find(&format!(" {name}=\""))? + name.len() + 3;
    let end = line[start..].find('"')? + start;
    Some(&line[start..end])
}

fn number(line: &str, name: &str) -> f32 {
    attribute(line, name)
        .and_then(|value| value.parse().ok())
        .unwrap_or_else(|| panic!("no numeric {name} in {line}"))
}

fn page_texts(svg: &str, unit: f32) -> Vec<PageText> {
    svg.lines()
        .filter(|line| line.starts_with("<text ") && line.ends_with("</text>"))
        .map(|line| {
            let open = line.find('>').expect("text tag");
            PageText {
                class: attribute(line, "class").unwrap_or_default().to_owned(),
                text: line[open + 1..line.len() - "</text>".len()].to_owned(),
                anchor: attribute(line, "text-anchor").unwrap_or("start").to_owned(),
                x: number(line, "x") / unit,
                baseline: number(line, "y") / unit,
            }
        })
        .collect()
}

/// The size, in world units, a page class sets its type in.
fn class_size(class: &str) -> f32 {
    match class {
        "name" => 10.0,
        "value" | "probe-label" => 9.0,
        "net-name" => 11.0,
        other => panic!("unexpected text class {other}"),
    }
}

/// Every label the canvas lettered is on the page, under the same string,
/// hung off the same anchor: the same start, middle or end, and the same
/// top of its line box.
#[test]
fn the_page_letters_every_canvas_label_at_the_canvas_anchor() {
    let document = fixture_document();
    let unit = SvgExportConfig::default().units_per_world as f32;
    let page = page_texts(&export(&document, SvgDesignContext::default()), unit);
    let canvas: Vec<(String, Rect)> = canvas_shapes(&document)
        .into_iter()
        .filter_map(|shape| match shape {
            Shape::Text(text) => Some((
                text.galley.text().to_owned(),
                Rect::from_min_max(world(text.pos), world(text.pos + text.galley.rect.size())),
            )),
            _ => None,
        })
        .collect();

    let mut canvas_strings: Vec<&str> = canvas.iter().map(|(text, _)| text.as_str()).collect();
    let mut page_strings: Vec<&str> = page.iter().map(|text| text.text.as_str()).collect();
    canvas_strings.sort_unstable();
    page_strings.sort_unstable();
    assert_eq!(
        page_strings, canvas_strings,
        "the page and the canvas letter the same labels"
    );
    for expected in ["R1", "10k", "R2", "4.7k", "VOUT", "V(OUT)"] {
        assert!(page_strings.contains(&expected), "{expected} is missing");
    }

    for text in &page {
        let (_, rect) = canvas
            .iter()
            .find(|(string, _)| *string == text.text)
            .expect("matched above");
        let canvas_x = match text.anchor.as_str() {
            "start" => rect.left(),
            "middle" => rect.center().x,
            "end" => rect.right(),
            other => panic!("unexpected anchor {other}"),
        };
        let top = text.baseline - 1.025 * class_size(&text.class);
        assert!(
            (canvas_x - text.x).abs() <= 0.5,
            "{} anchors at x {} on the page and {canvas_x} on the canvas",
            text.text,
            text.x
        );
        assert!(
            (rect.top() - top).abs() <= 0.5,
            "{}'s line box tops at {top} on the page and {} on the canvas",
            text.text,
            rect.top()
        );
    }
}

/// The junction dot and the net label's diamond stand where the canvas
/// draws them, at the canvas's size.
#[test]
fn the_page_marks_connectivity_where_the_canvas_does() {
    let document = fixture_document();
    let unit = SvgExportConfig::default().units_per_world as f32;
    let svg = export(&document, SvgDesignContext::default());
    let shapes = canvas_shapes(&document);

    let junctions: Vec<&str> = svg
        .lines()
        .filter(|line| line.starts_with("<circle class=\"junction\""))
        .collect();
    assert_eq!(junctions.len(), 1, "{svg}");
    let (cx, cy, r) = (
        number(junctions[0], "cx") / unit,
        number(junctions[0], "cy") / unit,
        number(junctions[0], "r") / unit,
    );
    let dot = shapes
        .iter()
        .find_map(|shape| match shape {
            Shape::Circle(circle)
                if circle.fill != egui::Color32::TRANSPARENT
                    && world(circle.center).distance(Pos2::new(cx, cy)) <= 0.25
                    && circle.radius / ZOOM >= 1.0 =>
            {
                Some(circle.radius / ZOOM)
            }
            _ => None,
        })
        .expect("the canvas draws the junction dot the page does");
    assert!(
        (dot - r).abs() <= 0.1,
        "the dot is {r} wu on the page and {dot} on the canvas"
    );

    let mark = svg
        .lines()
        .find(|line| line.starts_with("<polygon class=\"net-mark\""))
        .expect("the page draws the label's diamond");
    let mut page_diamond: Vec<Pos2> = attribute(mark, "points")
        .expect("points")
        .split(' ')
        .map(|pair| {
            let (x, y) = pair.split_once(',').expect("x,y");
            Pos2::new(
                x.parse::<f32>().expect("x") / unit,
                y.parse::<f32>().expect("y") / unit,
            )
        })
        .collect();
    let mut canvas_diamond = shapes
        .iter()
        .find_map(|shape| match shape {
            Shape::Path(path)
                if path.closed
                    && path.points.len() == 4
                    && path
                        .points
                        .iter()
                        .all(|point| world(*point).distance(Pos2::new(-40.0, 0.0)) <= 3.0) =>
            {
                Some(
                    path.points
                        .iter()
                        .map(|point| world(*point))
                        .collect::<Vec<_>>(),
                )
            }
            _ => None,
        })
        .expect("the canvas draws the label's diamond");
    let order = |a: &Pos2, b: &Pos2| a.x.total_cmp(&b.x).then(a.y.total_cmp(&b.y));
    page_diamond.sort_by(order);
    canvas_diamond.sort_by(order);
    for (page, canvas) in page_diamond.iter().zip(&canvas_diamond) {
        assert!(
            page.distance(*canvas) <= 0.1,
            "diamond corner {page:?} on the page, {canvas:?} on the canvas"
        );
    }
}

/// Paper ink and line ends: conductors end square to their points, never a
/// half stroke past a pin, at section 1's nominal widths.
#[test]
fn conductors_end_butt_at_their_nominal_width() {
    let svg = export(&fixture_document(), SvgDesignContext::default());
    assert!(
        svg.contains(".wire { stroke: #4E3400; stroke-width: 11; fill: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 4; }"),
        "{svg}"
    );
    assert!(!svg.contains("square"), "no square caps anywhere: {svg}");
    assert!(
        svg.contains(".symbol { stroke: #2C3135; stroke-width: 10;"),
        "{svg}"
    );
    assert_eq!(
        svg.matches("<path class=\"wire\"").count(),
        2,
        "the through wire and the stub are two chains"
    );
}

/// Only the active sheet is on the canvas, so only it is on the page.
#[test]
fn the_page_exports_the_active_sheet_only() {
    let document = fixture_document();
    let mut catalog = SheetCatalog::default();
    let mut sheets = Vec::new();
    for page in [1, 2] {
        sheets.push(
            catalog
                .create_sheet(
                    SheetDefinition {
                        name: format!("Sheet {page}"),
                        template: SheetTemplate::AnalogSchematic,
                        port_policy: SheetPortPolicy::TypedOffSheetPorts,
                        explicit_page_number: Some(page),
                    },
                    sheets.last().copied(),
                )
                .expect("sheet"),
        );
    }
    // The turned resistor and the probe live on the second sheet.
    catalog
        .assign_objects(catalog.revision(), sheets[1], [2, 7])
        .expect("assign");
    catalog.set_active(sheets[0]).expect("activate");

    let whole = export(&document, SvgDesignContext::default());
    assert!(whole.contains(">R2</text>") && whole.contains(">V(OUT)</text>"));

    let active = export(
        &document,
        SvgDesignContext {
            sheet_catalog: Some(&catalog),
            ..SvgDesignContext::default()
        },
    );
    assert!(active.contains(">R1</text>") && active.contains(">VOUT</text>"));
    assert!(
        !active.contains(">R2</text>") && !active.contains(">V(OUT)</text>"),
        "another sheet's objects are not on this page: {active}"
    );
}

/// The fixture sheet as an SVG file, for a person to open and measure.
#[test]
#[ignore = "writes a review SVG to RSPICE_RASTER_DIR"]
fn render_svg_export_review() {
    let svg = export(&fixture_document(), SvgDesignContext::default());
    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("review output directory");
    let stage = std::env::var("RSPICE_REVIEW_STAGE").unwrap_or_else(|_| "now".to_owned());
    let path = directory.join(format!("svg-export-{stage}.svg"));
    std::fs::write(&path, svg).expect("write review svg");
    eprintln!("wrote {}", path.display());
}
