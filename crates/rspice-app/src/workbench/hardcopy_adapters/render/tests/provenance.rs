//! Publication identities must remain complete and inside the physical footer.

use super::*;

fn publication_plan(format: OutputFormat, portrait: bool) -> HardcopyPlan {
    let base = setup_with_watermark(format, false, Watermark::None);
    let settings = HardcopySetup::try_new(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(StandardPaper::Letter),
            PageMargins::uniform(Length::from_micrometres(10_000)),
            Bleed::None,
            if portrait {
                Orientation::Portrait
            } else {
                Orientation::Landscape
            },
        )
        .unwrap(),
        base.scale(),
        base.tiling(),
        base.render().clone(),
        base.decorations().clone(),
        base.print_mapping().clone(),
    )
    .unwrap();
    plan_from_setup(settings, extent(100_000, 100_000))
}

fn identity() -> String {
    format!(
        "source project:{}:result-dataset:{} · document {} · revision 42 · digest {}",
        Uuid::from_u128(12),
        Uuid::from_u128(13),
        Uuid::from_u128(14),
        digest(0x53),
    )
}

/// Shape the emitted SVG with the raster backend's engine. Checking actual
/// glyph geometry catches overflow independently of the wrapping algorithm.
fn check_footer(plan: &HardcopyPlan, svg: &str, expected: &str) {
    let xml = usvg::roxmltree::Document::parse(svg).unwrap();
    let page = &plan.pagination().pages()[0];
    let geometry = page.geometry();
    let printable = geometry.printable_rect();
    let bottom = printable.y.micrometres() + printable.height.micrometres();
    let top = bottom - geometry.provenance_band().micrometres();
    let rows: Vec<_> = xml
        .descendants()
        .filter(|node| {
            node.has_tag_name("text")
                && node
                    .attribute("y")
                    .and_then(|y| y.parse::<f64>().ok())
                    .is_some_and(|y| y >= top as f64)
        })
        .collect();
    assert_eq!(
        rows.iter().filter_map(|row| row.text()).collect::<String>(),
        expected
    );
    assert_eq!(
        rows.len(),
        2,
        "long identities must wrap into the reserved band"
    );
    let (width, height) = geometry.physical_size();
    let fragments = rows.iter().map(|row| &svg[row.range()]).collect::<String>();
    let fragment = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">{}</svg>",
        width.micrometres(),
        height.micrometres(),
        fragments.replace(rows[0].attribute("font-family").unwrap(), "IBM Plex Mono"),
    );
    let mut options = usvg::Options::default();
    options
        .fontdb_mut()
        .load_font_data(IBM_PLEX_MONO_REGULAR.to_vec());
    let tree = usvg::Tree::from_str(&fragment, &options).unwrap();
    assert!(
        !tree.root().children().is_empty(),
        "font shaping must produce ink"
    );
    let bounds = tree.root().abs_bounding_box();
    assert!(bounds.left() >= printable.x.micrometres() as f32);
    assert!(bounds.right() <= (printable.x.micrometres() + printable.width.micrometres()) as f32);
    assert!(bounds.top() >= top as f32);
    assert!(bounds.bottom() <= bottom as f32);
}

#[test]
fn long_provenance_wraps_losslessly_inside_portrait_and_landscape_bands() {
    for portrait in [false, true] {
        let plan = publication_plan(OutputFormat::SvgVector, portrait);
        for text in [identity(), "éµ".repeat(110), "e\u{301}".repeat(210)] {
            let mut scene = scene(plan.content_extent());
            scene
                .metadata
                .set_provenance_lines(vec![text.clone()])
                .unwrap();
            let publication = HardcopyRenderer::render(&plan, &scene).unwrap();
            let svg = std::str::from_utf8(publication.single_part().unwrap().bytes()).unwrap();
            check_footer(&plan, svg, &text);
        }
    }
}

#[test]
fn provenance_publication_is_complete_in_every_artifact_format() {
    for format in [
        OutputFormat::SvgVector,
        OutputFormat::PdfVector,
        OutputFormat::PdfA,
        OutputFormat::Png { dpi: 96 },
        OutputFormat::Tiff { dpi: 96 },
        OutputFormat::BrowserPrintDocument,
    ] {
        let plan = publication_plan(format, true);
        let mut scene = scene(plan.content_extent());
        scene
            .metadata
            .set_provenance_lines(vec![identity()])
            .unwrap();
        let publication = HardcopyRenderer::render(&plan, &scene).unwrap();
        let bytes = publication.single_part().unwrap().bytes();
        if matches!(format, OutputFormat::PdfVector | OutputFormat::PdfA) {
            let pdf = ParsedPdf::load_mem(bytes).unwrap();
            let text = pdf.extract_text(&[1]).unwrap();
            for field in ["source", "revision 42", &digest(0x53).to_string()] {
                assert!(text.contains(field), "{format:?} lost {field}");
            }
        }
        if format == OutputFormat::BrowserPrintDocument {
            let html = std::str::from_utf8(bytes).unwrap();
            assert_eq!(
                html.matches("data-rspice-decoration=\"provenance\"")
                    .count(),
                2
            );
            assert!(html.contains(&digest(0x53).to_string()));
        }
        scene
            .metadata
            .set_provenance_lines(vec!["x".repeat(4_000)])
            .unwrap();
        assert!(
            matches!(
                HardcopyRenderer::render(&plan, &scene),
                Err(HardcopyRenderError::DecorationOverflow {
                    decoration: "provenance rows",
                    ..
                })
            ),
            "{format:?} must refuse an identity too large for its band"
        );
    }
}

#[test]
fn provenance_preserves_authored_rows_and_rejects_excess_rows() {
    let plan = publication_plan(OutputFormat::SvgVector, false);
    let mut scene = scene(plan.content_extent());
    scene
        .metadata
        .set_provenance_lines(vec!["first  line".to_owned(), "second line".to_owned()])
        .unwrap();
    let publication = HardcopyRenderer::render(&plan, &scene).unwrap();
    let svg = std::str::from_utf8(publication.single_part().unwrap().bytes()).unwrap();
    check_footer(&plan, svg, "first  linesecond line");
    scene
        .metadata
        .set_provenance_lines(vec!["a".to_owned(); 3])
        .unwrap();
    assert!(matches!(
        HardcopyRenderer::render(&plan, &scene),
        Err(HardcopyRenderError::DecorationOverflow { .. })
    ));
}
