//! A printed legend keeps every label inside its own column and inside its
//! band, in every writer and on every paper a long legend can be printed on.

use std::collections::{BTreeMap, BTreeSet};

use lopdf::Object as PdfObject;

use super::*;

/// The swatch sits this far inside the column it starts.
const COLUMN_INSET_UM: f64 = 2_000.0;
const POINT_UM: f64 = 25_400.0 / 72.0;

/// Twelve names of the length an aggregate print maps: a cell view, a sheet
/// and the object's own name. No two share a source, so none can be shortened
/// by taking a common prefix off.
fn long_labels() -> Vec<String> {
    let cells = [
        "analog_lib/amplifier_core/schematic",
        "analog_lib/bias_generator/schematic",
        "analog_lib/comparator_latch/schematic",
        "analog_lib/output_driver/schematic",
    ];
    let objects = [
        "Drawing sheet title block",
        "Wires and junctions",
        "Drawing sheet paper",
    ];
    cells
        .iter()
        .flat_map(|cell| {
            objects
                .iter()
                .map(move |object| format!("{cell} · Second sheet · {object}"))
        })
        .collect()
}

fn legend_plan_and_scene(
    format: OutputFormat,
    paper: StandardPaper,
    orientation: Orientation,
    labels: &[String],
) -> (HardcopyPlan, HardcopyScene) {
    let base = setup_with_watermark(format, false, Watermark::None);
    let entries = labels
        .iter()
        .enumerate()
        .map(|(index, label)| {
            PrintMappingEntry::try_new(
                PrintObjectIdentity::try_new(
                    PrintObjectKind::Trace,
                    format!("trace:{index}"),
                    label.as_str(),
                    "solid",
                )
                .unwrap(),
                PrintColor::Black,
                PrintRedundancy::SolidLine {
                    width: Length::from_micrometres(300),
                },
                true,
            )
            .unwrap()
        })
        .collect();
    let setup = HardcopySetup::try_new(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(paper),
            PageMargins::uniform(Length::from_micrometres(10_000)),
            Bleed::None,
            orientation,
        )
        .unwrap(),
        base.scale(),
        base.tiling(),
        base.render().clone(),
        base.decorations().clone(),
        PrintMappingTable::try_new(PrintMappingSaveScope::Document, entries).unwrap(),
    )
    .unwrap();
    let plan = plan_from_setup(setup, extent(100_000, 100_000));
    let mut scene = scene(plan.content_extent());
    let stroke = StrokeStyle::try_new(
        SemanticColor::Trace(1),
        Length::from_micrometres(350),
        StrokePattern::Solid,
        Some(1),
    )
    .unwrap();
    scene.legend = labels
        .iter()
        .enumerate()
        .map(|(index, label)| {
            // Every third entry is a filled swatch, the way a mapped area prints.
            let fill = (index % 3 == 2).then_some(SceneFill::Solid {
                color: SemanticColor::Trace(1),
            });
            LegendEntry::try_new_with_fill(label.clone(), stroke, fill).unwrap()
        })
        .collect();
    scene.validate().unwrap();
    (plan, scene)
}

/// The band the plan reserves on the first page, as left, top, right, bottom
/// in micrometres from the page's top-left corner.
fn legend_band(plan: &HardcopyPlan) -> (f64, f64, f64, f64) {
    let geometry = plan.pagination().pages()[0].geometry();
    let printable = geometry.printable_rect();
    let content = geometry.content_rect();
    let top = (content.y.micrometres() + content.height.micrometres()) as f64;
    (
        printable.x.micrometres() as f64,
        top,
        (printable.x.micrometres() + printable.width.micrometres()) as f64,
        top + geometry.legend_band().micrometres() as f64,
    )
}

/// One printed legend entry: where its swatch starts and where its label's
/// measured ink lies.
#[derive(Debug)]
struct PrintedEntry {
    swatch_left: f64,
    baseline: f64,
    label_left: f64,
    label_right: f64,
    label_top: f64,
    label_bottom: f64,
}

/// Every label ends before the next column in its row starts, the last one
/// in a row before the band's edge, and every label lies inside the band.
fn assert_columns_clear(context: &str, band: (f64, f64, f64, f64), entries: &[PrintedEntry]) {
    let (left, top, right, bottom) = band;
    let mut rows: BTreeMap<i64, Vec<&PrintedEntry>> = BTreeMap::new();
    for entry in entries {
        rows.entry(entry.baseline.round() as i64)
            .or_default()
            .push(entry);
        assert!(
            entry.label_left >= left && entry.label_top >= top && entry.label_bottom <= bottom,
            "{context}: a label leaves the legend band: {entry:?} in {band:?}"
        );
    }
    assert!(
        rows.len() > 1,
        "{context}: the fixture must fill several rows"
    );
    assert!(
        rows.values().any(|row| row.len() > 1),
        "{context}: the fixture must fill several columns"
    );
    for row in rows.values_mut() {
        row.sort_by(|a, b| a.swatch_left.total_cmp(&b.swatch_left));
        for (index, entry) in row.iter().enumerate() {
            let limit = row
                .get(index + 1)
                .map_or(right, |next| next.swatch_left - COLUMN_INSET_UM);
            assert!(
                entry.label_right <= limit,
                "{context}: a label runs {:.0} um past the next column: {entry:?}, column ends at {limit}",
                entry.label_right - limit
            );
        }
    }
}

/// A shortened label keeps the object's own name and the end of what it
/// names; no two labels on a page read the same.
fn assert_labels_identify_their_entries(context: &str, printed: &[String], labels: &[String]) {
    assert_eq!(
        printed.len(),
        labels.len(),
        "{context}: an entry was dropped"
    );
    let unique: BTreeSet<_> = printed.iter().collect();
    assert_eq!(
        unique.len(),
        printed.len(),
        "{context}: two labels read the same: {printed:?}"
    );
    for (printed, label) in printed.iter().zip(labels) {
        let own_name = label.rsplit(" · ").next().unwrap();
        assert!(
            printed.ends_with(own_name),
            "{context}: {printed:?} lost the object's own name {own_name:?}"
        );
        match printed.split_once('…') {
            None => assert_eq!(printed, label, "{context}"),
            Some((head, tail)) => assert!(
                label.starts_with(head) && label.ends_with(tail),
                "{context}: {printed:?} does not shorten {label:?}"
            ),
        }
    }
}

fn papers() -> [(StandardPaper, Orientation); 4] {
    [
        (StandardPaper::A4, Orientation::Landscape),
        (StandardPaper::A4, Orientation::Portrait),
        (StandardPaper::A3, Orientation::Landscape),
        (StandardPaper::A3, Orientation::Portrait),
    ]
}

/// The SVG writer is also the raster path: PNG, TIFF, the printer and the
/// preview rasterise this page. Labels are shaped with the raster engine, so
/// the check measures the ink a reader sees, not the layout's own arithmetic.
#[test]
fn svg_legend_labels_stay_inside_their_columns_on_a4_and_a3() {
    let labels = long_labels();
    for (paper, orientation) in papers() {
        let context = format!("SVG {paper:?} {orientation:?}");
        let (plan, scene) =
            legend_plan_and_scene(OutputFormat::SvgVector, paper, orientation, &labels);
        let publication = HardcopyRenderer::render(&plan, &scene).unwrap();
        let svg = std::str::from_utf8(publication.single_part().unwrap().bytes()).unwrap();
        let xml = usvg::roxmltree::Document::parse(svg).unwrap();
        let legend = xml
            .descendants()
            .find(|node| node.attribute("data-rspice-decoration") == Some("legend"))
            .expect("the page prints its legend");
        let (width, height) = plan.pagination().pages()[0].geometry().physical_size();
        let mut options = usvg::Options::default();
        options
            .fontdb_mut()
            .load_font_data(IBM_PLEX_SANS_REGULAR.to_vec());
        let number = |node: usvg::roxmltree::Node<'_, '_>, name: &str| {
            node.attribute(name).unwrap().parse::<f64>().unwrap()
        };
        let mut swatch_left = None;
        let mut entries = Vec::new();
        let mut printed = Vec::new();
        for node in legend.children().filter(|node| node.is_element()).skip(1) {
            match node.tag_name().name() {
                "line" => swatch_left = Some(number(node, "x1")),
                "rect" => swatch_left = Some(number(node, "x")),
                "text" => {
                    let fragment = format!(
                        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">{}</svg>",
                        width.micrometres(),
                        height.micrometres(),
                        svg[node.range()]
                            .replace(node.attribute("font-family").unwrap(), "IBM Plex Sans"),
                    );
                    let tree = usvg::Tree::from_str(&fragment, &options).unwrap();
                    let ink = tree.root().abs_bounding_box();
                    entries.push(PrintedEntry {
                        swatch_left: swatch_left.take().expect("a swatch precedes its label"),
                        baseline: number(node, "y"),
                        label_left: f64::from(ink.left()),
                        label_right: f64::from(ink.right()),
                        label_top: f64::from(ink.top()),
                        label_bottom: f64::from(ink.bottom()),
                    });
                    printed.push(node.text().unwrap_or_default().to_owned());
                }
                other => panic!("{context}: unexpected legend element {other}"),
            }
        }
        assert_labels_identify_their_entries(&context, &printed, &labels);
        assert_columns_clear(&context, legend_band(&plan), &entries);
    }
}

/// Glyph advances of each Type0 font on a page, by resource name.
fn pdf_font_widths(
    pdf: &ParsedPdf,
    page: lopdf::ObjectId,
) -> BTreeMap<Vec<u8>, (f64, BTreeMap<u32, f64>)> {
    let mut fonts = BTreeMap::new();
    for (name, font) in pdf.get_page_fonts(page).unwrap() {
        assert_eq!(
            font.get(b"Subtype").unwrap().as_name().unwrap(),
            b"Type0",
            "legend text is set in a composite font"
        );
        let descendants = pdf
            .dereference(font.get(b"DescendantFonts").unwrap())
            .unwrap()
            .1
            .as_array()
            .unwrap();
        let cid_font = pdf
            .dereference(&descendants[0])
            .unwrap()
            .1
            .as_dict()
            .unwrap();
        let default_width = cid_font
            .get(b"DW")
            .map_or(1_000.0, |width| f64::from(width.as_float().unwrap()));
        let mut widths = BTreeMap::new();
        if let Ok(table) = cid_font.get(b"W") {
            let table = pdf.dereference(table).unwrap().1.as_array().unwrap();
            let mut index = 0;
            while index < table.len() {
                let first = table[index].as_i64().unwrap() as u32;
                match pdf.dereference(&table[index + 1]).unwrap().1 {
                    PdfObject::Array(run) => {
                        for (offset, width) in run.iter().enumerate() {
                            widths.insert(
                                first + offset as u32,
                                f64::from(width.as_float().unwrap()),
                            );
                        }
                        index += 2;
                    }
                    last => {
                        let last = last.as_i64().unwrap() as u32;
                        let width = f64::from(table[index + 2].as_float().unwrap());
                        for cid in first..=last {
                            widths.insert(cid, width);
                        }
                        index += 3;
                    }
                }
            }
        }
        fonts.insert(name, (default_width, widths));
    }
    fonts
}

/// The PDF writer sets each label as glyphs of an embedded font. The check
/// reads the page back: each label's start from its text matrix and its end
/// from the embedded font's own advances, independent of the layout.
#[test]
fn pdf_legend_labels_stay_inside_their_columns_on_a4_and_a3() {
    let labels = long_labels();
    for (paper, orientation) in papers() {
        let context = format!("PDF {paper:?} {orientation:?}");
        let (plan, scene) =
            legend_plan_and_scene(OutputFormat::PdfVector, paper, orientation, &labels);
        let publication = HardcopyRenderer::render(&plan, &scene).unwrap();
        let pdf = ParsedPdf::load_mem(publication.single_part().unwrap().bytes()).unwrap();
        let page = pdf.get_pages()[&1];
        let fonts = pdf_font_widths(&pdf, page);
        let (_, page_height) = plan.pagination().pages()[0].geometry().physical_size();
        let page_height = page_height.micrometres() as f64;
        let band = legend_band(&plan);
        let (band_left, band_top, _, band_bottom) = band;
        // Page space is points from the bottom-left corner; the band is
        // micrometres from the top-left one.
        let to_page = |ctm: [f64; 6], x: f64, y: f64| {
            let px = ctm[0] * x + ctm[2] * y + ctm[4];
            let py = ctm[1] * x + ctm[3] * y + ctm[5];
            (px * POINT_UM, page_height - py * POINT_UM)
        };
        let float = |object: &PdfObject| f64::from(object.as_float().unwrap());
        let mut ctm = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        let mut saved = Vec::new();
        let mut font = (1_000.0, BTreeMap::new());
        let mut size = 0.0;
        let mut text_begin = None;
        let mut text_origin = (0.0, 0.0);
        let mut advance = 0.0;
        let mut swatch_left = None;
        let mut entries = Vec::new();
        for operation in pdf.get_and_decode_page_content(page).unwrap().operations {
            let operands = &operation.operands;
            match operation.operator.as_str() {
                "q" => saved.push(ctm),
                "Q" => ctm = saved.pop().unwrap(),
                "cm" => {
                    let m: Vec<f64> = operands.iter().map(float).collect();
                    ctm = [
                        m[0] * ctm[0] + m[1] * ctm[2],
                        m[0] * ctm[1] + m[1] * ctm[3],
                        m[2] * ctm[0] + m[3] * ctm[2],
                        m[2] * ctm[1] + m[3] * ctm[3],
                        m[4] * ctm[0] + m[5] * ctm[2] + ctm[4],
                        m[4] * ctm[1] + m[5] * ctm[3] + ctm[5],
                    ];
                }
                "m" | "re" => {
                    let (x, y) = to_page(ctm, float(&operands[0]), float(&operands[1]));
                    let at_band_corner =
                        (x - band_left).abs() < 100.0 && (y - band_top).abs() < 100.0;
                    if y > band_top && y < band_bottom && !at_band_corner {
                        swatch_left = Some(x);
                    }
                }
                "Tf" => {
                    font = fonts[operands[0].as_name().unwrap()].clone();
                    size = float(&operands[1]);
                }
                "BT" => text_begin = None,
                // A label split into several runs restarts the matrix at
                // each run; its first run's origin is where it begins.
                "Tm" => {
                    text_origin = (float(&operands[4]), float(&operands[5]));
                    text_begin.get_or_insert(text_origin);
                    advance = 0.0;
                }
                "Tj" | "TJ" => {
                    let items = match &operands[0] {
                        PdfObject::Array(items) => items.clone(),
                        single => vec![single.clone()],
                    };
                    for item in items {
                        match item {
                            PdfObject::String(bytes, _) => {
                                for cid in bytes.chunks(2) {
                                    let cid = u32::from(cid[0]) << 8 | u32::from(cid[1]);
                                    let width = font.1.get(&cid).copied().unwrap_or(font.0);
                                    advance += width / 1_000.0 * size;
                                }
                            }
                            adjustment => advance -= float(&adjustment) / 1_000.0 * size,
                        }
                    }
                }
                "ET" => {
                    let begin = text_begin.unwrap_or(text_origin);
                    let (start, baseline) = to_page(ctm, begin.0, begin.1);
                    let (end, _) = to_page(ctm, text_origin.0 + advance, text_origin.1);
                    if baseline > band_top && baseline < band_bottom {
                        let ascent = size * POINT_UM;
                        entries.push(PrintedEntry {
                            swatch_left: swatch_left.take().expect("a swatch precedes its label"),
                            baseline,
                            label_left: start.min(end),
                            label_right: start.max(end),
                            label_top: baseline - ascent * 0.75,
                            label_bottom: baseline + ascent * 0.25,
                        });
                    }
                }
                _ => {}
            }
        }
        assert_eq!(
            entries.len(),
            labels.len(),
            "{context}: an entry was dropped"
        );
        assert_columns_clear(&context, band, &entries);
    }
}
