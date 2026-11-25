use super::*;
use rspice_results::report_document::{
    DataTableBlock, EvidenceBlock, FrozenReportArtifact, ProseBlock, ProseStyle, ReportDocument,
    ReportEdit, ReportEntityRef, ReportPublicationProfile, ReportReferenceMode, RequirementEntry,
    RequirementsBlock, TableColumn,
};
use rspice_vocabulary::product::{ObjectRevision, VerificationEvidenceId};

fn frozen() -> ReportReferenceMode {
    frozen_from(ReportSourceId::ExternalRecord {
        namespace: "lab".to_owned(),
        key: "bench-7".to_owned(),
    })
}

fn frozen_from(source: ReportSourceId) -> ReportReferenceMode {
    ReportReferenceMode::Frozen {
        snapshot: ReportReferenceSnapshot::new(
            source,
            Some(ObjectRevision::INITIAL),
            ContentDigest::from_bytes([0x44; 32]),
            Vec::new(),
        )
        .unwrap(),
        artifact: FrozenReportArtifact::new("application/json", br#"{"ok":true}"#.to_vec())
            .unwrap(),
    }
}

/// A report on `page_size` holding `blocks` in one section of one page.
fn report(page_size: ReportPublicationPageSize, blocks: Vec<ReportBlockKind>) -> SemanticReport {
    let document = report_document(page_size, blocks);
    SemanticReport {
        pages: document.pages().to_vec(),
        authenticated_references: Vec::new(),
        figures: Vec::new(),
        page_size,
    }
}

fn report_document(
    page_size: ReportPublicationPageSize,
    blocks: Vec<ReportBlockKind>,
) -> ReportDocument {
    let mut document = ReportDocument::new("Report").unwrap();
    let publication_profile = ReportPublicationProfile {
        page_size,
        ..ReportPublicationProfile::default()
    };
    if publication_profile != ReportPublicationProfile::default() {
        document
            .transact(
                document.revision(),
                vec![ReportEdit::SetPublicationProfile {
                    publication_profile,
                }],
                1,
            )
            .unwrap();
    }
    let receipt = document
        .transact(
            document.revision(),
            vec![ReportEdit::AddPage {
                title: "Amplifier".to_owned(),
            }],
            2,
        )
        .unwrap();
    let ReportEntityRef::Page(page_id) = receipt.created[0] else {
        unreachable!()
    };
    let receipt = document
        .transact(
            document.revision(),
            vec![ReportEdit::AddSection {
                page_id,
                title: "Results".to_owned(),
            }],
            3,
        )
        .unwrap();
    let ReportEntityRef::Section(section_id) = receipt.created[0] else {
        unreachable!()
    };
    document
        .transact(
            document.revision(),
            blocks
                .into_iter()
                .map(|kind| ReportEdit::AddBlock { section_id, kind })
                .collect(),
            4,
        )
        .unwrap();
    document
}

fn prose(markdown: impl Into<String>) -> ReportBlockKind {
    ReportBlockKind::Prose(ProseBlock {
        style: ProseStyle::Body,
        markdown: markdown.into(),
    })
}

fn texts(page: &LaidPage) -> Vec<(u64, u64, u64, Role, &str)> {
    page.items
        .iter()
        .filter_map(|item| match item {
            Laid::Text {
                x,
                y,
                size,
                role,
                text,
            } => Some((*x, *y, *size, *role, text.as_str())),
            _ => None,
        })
        .collect()
}

fn all_text(layout: &ReportLayout) -> String {
    layout
        .pages
        .iter()
        .flat_map(|page| texts(page).into_iter().map(|(.., text)| text.to_owned()))
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn ordinary_prose_prints_as_written() {
    for text in [
        "|S21| stays < 1 dB",
        "a lone * and a lone ` mark",
        "x < y > z",
    ] {
        let runs = inline_runs(text, Role::Sans);
        assert_eq!(plain(&runs), text);
        assert!(runs.iter().all(|run| run.role == Role::Sans));
    }
    let layout = lay_out(&report(
        ReportPublicationPageSize::A4Portrait,
        vec![prose(
            "> The gain |S21| stays within 1 dB.\n\n|S21| < 1 dB from 1 to 2 GHz <br>",
        )],
    ));
    let printed = all_text(&layout);
    assert!(printed.contains("|S21|"), "{printed}");
    assert!(printed.contains("<"), "{printed}");
}

#[test]
fn bold_is_set_only_where_it_is_marked() {
    let runs = inline_runs("gain is **20 dB** at `f0`", Role::Sans);
    assert_eq!(
        runs,
        vec![
            Run::new("gain is ", Role::Sans),
            Run::new("20 dB", Role::SansSemibold),
            Run::new(" at ", Role::Sans),
            Run::new("f0", Role::Mono),
        ]
    );
}

#[test]
fn a_link_prints_its_text_and_address() {
    let runs = inline_runs(
        "see [the spec](https://example.com/s) and ![the plot](p.png)",
        Role::Sans,
    );
    assert_eq!(
        plain(&runs),
        "see the spec (https://example.com/s) and the plot"
    );
}

#[test]
fn lines_break_where_the_measured_width_runs_out() {
    let width = 150_000;
    let lines = wrap_runs(
        &inline_runs(&"WWWW iiii ".repeat(40), Role::Sans),
        BODY_UM,
        width,
    );
    assert!(lines.len() > 2);
    for line in &lines {
        let set: u64 = line
            .iter()
            .map(|run| set_width(run.role, &run.text, BODY_UM))
            .sum();
        assert!(set <= width, "{set} > {width}");
    }
    // A line of narrow letters holds more characters than one of wide.
    let narrow = wrap_runs(
        &inline_runs(&"iiii ".repeat(80), Role::Sans),
        BODY_UM,
        width,
    );
    let wide = wrap_runs(
        &inline_runs(&"WWWW ".repeat(80), Role::Sans),
        BODY_UM,
        width,
    );
    assert!(plain(&narrow[0]).len() > plain(&wide[0]).len());
}

/// Two hundred paragraphs run on over several pages, every line inside its
/// page, numbered, the body at 10 pt.
#[test]
fn a_long_report_flows_onto_further_pages() {
    let paragraph = "The loop gain crosses unity near 12 MHz with 61 degrees of phase \
                     margin, and the closed-loop response stays flat to within a tenth \
                     of a decibel across the band of interest.";
    let markdown = (1..=200)
        .map(|index| format!("Paragraph {index}. {paragraph}"))
        .collect::<Vec<_>>()
        .join("\n\n");
    let layout = lay_out(&report(
        ReportPublicationPageSize::UsLetterPortrait,
        vec![prose(markdown)],
    ));
    assert!(layout.pages.len() > 5, "{} pages", layout.pages.len());
    let count = layout.pages.len();
    for (index, page) in layout.pages.iter().enumerate() {
        for (x, y, size, _, text) in texts(page) {
            assert!(y <= layout.height - MARGIN_UM / 2, "{text} at {y}");
            assert!(
                x + set_width(Role::Sans, text, size) <= layout.width,
                "{text}"
            );
        }
        let foot = format!("Page {} of {count}", index + 1);
        assert!(texts(page).iter().any(|(.., text)| *text == foot));
    }
    let printed = all_text(&layout);
    assert!(printed.contains("Paragraph 200."));
    let body = texts(&layout.pages[1])
        .into_iter()
        .filter(|(.., text)| text.contains("loop"))
        .map(|(_, _, size, ..)| size)
        .collect::<Vec<_>>();
    assert!(!body.is_empty() && body.iter().all(|size| *size == BODY_UM));
    // 10 pt is 10/72 inch.
    assert_eq!(BODY_UM, (10.0 * 25_400.0 / 72.0_f64).round() as u64);
}

/// An eighty-row table runs on over pages, its header repeated on each, its
/// cells at 9 pt.
#[test]
fn a_long_table_repeats_its_header_on_each_page() {
    let table = ReportBlockKind::DataTable(DataTableBlock {
        title: "Corner sweep".to_owned(),
        columns: vec![
            TableColumn {
                key: "corner".to_owned(),
                heading: "Corner".to_owned(),
                unit: None,
            },
            TableColumn {
                key: "gain".to_owned(),
                heading: "Gain".to_owned(),
                unit: Some("dB".to_owned()),
            },
        ],
        rows: (0..80)
            .map(|index| {
                vec![
                    TableCell::Text(format!("corner {index}")),
                    TableCell::Number {
                        value: 20.0 + f64::from(index) / 10.0,
                        unit: Some("dB".to_owned()),
                    },
                ]
            })
            .collect(),
        reference: frozen(),
    });
    let layout = lay_out(&report(ReportPublicationPageSize::A4Portrait, vec![table]));
    assert!(layout.pages.len() >= 2);
    for page in &layout.pages {
        assert!(
            texts(page)
                .iter()
                .any(|(_, _, size, role, text)| *text == "Gain"
                    && *role == Role::SansSemibold
                    && *size == TABLE_UM),
            "every page heads the table"
        );
    }
    let printed = all_text(&layout);
    assert!(printed.contains("corner 79"));
    // The columns are as wide as their contents, not shares of the page.
    let gain = layout.pages[0]
        .items
        .iter()
        .find_map(|item| match item {
            Laid::Rule { width, .. } => Some(*width),
            _ => None,
        })
        .unwrap();
    assert!(gain < (layout.width - 2 * MARGIN_UM) / 2, "{gain}");
}

#[test]
fn headings_step_down_from_the_title_to_the_body() {
    let sizes = [
        TITLE_UM,
        SECTION_UM,
        HEADING_UM[0],
        HEADING_UM[1],
        HEADING_UM[2],
        HEADING_UM[3],
        BODY_UM,
        TABLE_UM,
    ];
    assert!(sizes.windows(2).all(|pair| pair[0] > pair[1]), "{sizes:?}");
    let layout = lay_out(&report(
        ReportPublicationPageSize::A4Portrait,
        vec![prose("# One\n\n## Two\n\n### Three\n\n#### Four\n\nBody.")],
    ));
    let size_of = |wanted: &str| {
        texts(&layout.pages[0])
            .into_iter()
            .find(|(.., text)| *text == wanted)
            .map(|(_, _, size, ..)| size)
            .unwrap()
    };
    assert!(size_of("One") > size_of("Two"));
    assert!(size_of("Two") > size_of("Three"));
    assert!(size_of("Three") > size_of("Four"));
    assert!(size_of("Four") > size_of("Body."));
}

#[test]
fn verdicts_and_evidence_print_as_words() {
    let layout = lay_out(&report(
        ReportPublicationPageSize::A4Portrait,
        vec![
            ReportBlockKind::Requirements(RequirementsBlock {
                title: "Requirements".to_owned(),
                entries: vec![RequirementEntry {
                    requirement_id: "R-1".to_owned(),
                    statement: "Gain above 20 dB".to_owned(),
                    disposition: RequirementDisposition::NotEvaluated,
                    evidence_label: None,
                }],
                reference: frozen(),
            }),
            ReportBlockKind::Evidence(EvidenceBlock {
                title: "Bench receipt".to_owned(),
                summary: "All retained checks passed.".to_owned(),
                reference: frozen_from(ReportSourceId::VerificationEvidence {
                    evidence_id: VerificationEvidenceId::new(),
                }),
            }),
        ],
    ));
    let printed = all_text(&layout);
    assert!(printed.contains("Not evaluated"), "{printed}");
    assert!(!printed.contains("NotEvaluated"), "{printed}");
    assert!(printed.contains("Bench receipt"), "{printed}");
    assert!(printed.contains("All retained checks passed."), "{printed}");
}

/// A4 is a page of its own size, not Letter scaled.
#[test]
fn a4_lays_out_on_an_a4_page() {
    let markdown = (0..120)
        .map(|index| format!("Line {index} of the method."))
        .collect::<Vec<_>>()
        .join("\n\n");
    let a4 = lay_out(&report(
        ReportPublicationPageSize::A4Portrait,
        vec![prose(markdown.clone())],
    ));
    let letter = lay_out(&report(
        ReportPublicationPageSize::UsLetterPortrait,
        vec![prose(markdown)],
    ));
    assert_eq!((a4.width, a4.height), (210_000, 297_000));
    assert_eq!((letter.width, letter.height), (215_900, 279_400));
    // The taller page holds more of the method on its first page.
    let lines = |layout: &ReportLayout| {
        texts(&layout.pages[0])
            .iter()
            .filter(|(.., text)| text.starts_with("Line"))
            .count()
    };
    assert!(lines(&a4) > lines(&letter));
}

/// A table with room on the page sets every cell, and every heading, on
/// one line: a column as wide as its contents holds them unbroken.
#[test]
fn a_table_with_room_sets_each_cell_on_one_line() {
    let column = |key: &str, heading: &str, unit: Option<&str>| TableColumn {
        key: key.to_owned(),
        heading: heading.to_owned(),
        unit: unit.map(str::to_owned),
    };
    let number = |value: f64, unit: &str| TableCell::Number {
        value,
        unit: Some(unit.to_owned()),
    };
    let table = ReportBlockKind::DataTable(DataTableBlock {
        title: "Measurements across corners".to_owned(),
        columns: vec![
            column("corner", "Corner", None),
            column("gain", "Gain", Some("dB")),
            column("margin", "Phase margin", Some("°")),
            column("thd", "THD", Some("%")),
        ],
        rows: [
            ("Slow, 125 °C", 38.7, 61.0, 1.9),
            ("Fast, −40 °C", 41.3, 52.4, 1.1),
        ]
        .into_iter()
        .map(|(corner, gain, margin, thd)| {
            vec![
                TableCell::Text(corner.to_owned()),
                number(gain, "dB"),
                number(margin, "°"),
                number(thd, "%"),
            ]
        })
        .collect(),
        reference: frozen(),
    });
    for page_size in [
        ReportPublicationPageSize::A4Portrait,
        ReportPublicationPageSize::UsLetterPortrait,
    ] {
        let layout = lay_out(&report(page_size, vec![table.clone()]));
        let printed = texts(&layout.pages[0])
            .into_iter()
            .map(|(_, _, _, _, text)| text)
            .collect::<Vec<_>>();
        for cell in [
            "Slow, 125 °C",
            "Fast, −40 °C",
            "Phase margin",
            "38.70 dB",
            "1.900 %",
        ] {
            assert!(printed.contains(&cell), "{cell} is broken: {printed:?}");
        }
    }
}

#[test]
fn columns_narrow_by_wrapping_where_the_page_is_too_narrow() {
    let widths = column_widths(&[50_000, 200_000], &[10_000, 20_000], 150_000, 3_000);
    assert!(widths.iter().sum::<u64>() <= 150_000, "{widths:?}");
    assert!(widths[0] >= 13_000 && widths[1] >= 23_000, "{widths:?}");
    assert_eq!(
        column_widths(&[10_000, 20_000], &[5_000, 5_000], 150_000, 3_000),
        vec![13_000, 23_000]
    );
}

/// A report exports a page to a sheet of its own paper, at 1:1, its body
/// measuring 10 pt in the PDF.
#[test]
fn a_report_exports_a_page_to_a_sheet_with_its_body_at_ten_points() {
    use crate::render::{HardcopyRenderer, HardcopySceneMetadata};
    use lopdf::Object;
    let markdown = (1..=120)
        .map(|index| format!("Paragraph {index} of the method."))
        .collect::<Vec<_>>()
        .join("\n\n");
    let document = report_document(ReportPublicationPageSize::A4Portrait, vec![prose(markdown)]);
    let resolved = super::super::resolve_report_source(super::super::ReportHardcopySource {
        source_key: "report".to_owned(),
        document: &document,
        reference_inventory: None,
        scope: HardcopyScope::CompleteReport,
    })
    .unwrap();
    let setup = resolved.report_output_setup().unwrap();
    let sections = resolved
        .hardcopy_sections_for_setup(setup.schematic())
        .unwrap();
    assert!(sections.len() > 1, "{} sections", sections.len());
    let plan = rspice_hardcopy_contract::HardcopyPlan::compile_with_sections(
        resolved.authority().clone(),
        setup,
        resolved.content_extent(),
        sections.clone(),
    )
    .unwrap();
    let publication = HardcopyRenderer::render_resolved(
        &plan,
        &resolved,
        HardcopySceneMetadata::try_new("Report", "RSpice tests").unwrap(),
    )
    .unwrap();
    let pdf = lopdf::Document::load_mem(publication.single_part().unwrap().bytes()).unwrap();
    let pages = pdf.get_pages();
    assert_eq!(pages.len(), sections.len());
    let number = |object: &Object| match object {
        Object::Real(value) => f64::from(*value),
        Object::Integer(value) => *value as f64,
        _ => f64::NAN,
    };
    let mut sizes = Vec::new();
    for page in pages.values() {
        let media = pdf
            .get_dictionary(*page)
            .unwrap()
            .get(b"MediaBox")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(number)
            .collect::<Vec<_>>();
        // A4 in points.
        assert!(
            (media[2] - media[0] - 595.28).abs() < 0.1
                && (media[3] - media[1] - 841.89).abs() < 0.1,
            "{media:?}"
        );
        let content = lopdf::content::Content::decode(&pdf.get_page_content(*page)).unwrap();
        sizes.extend(
            content
                .operations
                .iter()
                .filter(|operation| operation.operator == "Tf")
                .map(|operation| number(&operation.operands[1])),
        );
    }
    assert!(
        sizes.iter().any(|size| (size - 10.0).abs() < 0.01),
        "{sizes:?}"
    );
}

/// A kept open- and closed-loop gain, over five decades of frequency, with
/// a cursor at 10 kHz.
fn bode_figure() -> FrozenReportArtifact {
    use rspice_results::report_figure::{FigureAxis, FigureCursor, FigureTrace, ReportFigure};
    use rspice_results::visualization_document::AxisScale;
    let axis = |label: &str, unit: &str, scale| FigureAxis {
        label: label.to_owned(),
        unit: Some(unit.to_owned()),
        scale,
        pinned: None,
    };
    let decades = |gains: [f64; 6]| {
        vec![
            (0..6)
                .map(|decade| (10.0_f64.powi(decade + 1), gains[decade as usize]))
                .collect(),
        ]
    };
    ReportFigure {
        x_axis: axis("Frequency", "Hz", AxisScale::Logarithmic),
        y_axis: axis("Gain", "dB", AxisScale::Decibels),
        traces: vec![
            FigureTrace {
                label: "Open loop".to_owned(),
                paths: decades([80.0, 80.0, 60.0, 40.0, 20.0, 0.0]),
            },
            FigureTrace {
                label: "Closed loop".to_owned(),
                paths: decades([20.0, 20.0, 20.0, 20.0, 18.0, 0.0]),
            },
        ],
        cursors: vec![FigureCursor {
            label: "A".to_owned(),
            x: 1.0e4,
        }],
    }
    .artifact()
    .unwrap()
}

/// A figure set to fit the page fills it, and the PDF carries its
/// alternative text in its structure rather than printing it. The figure
/// prints as lines and words, ruled on its decades and keyed, not as a
/// picture.
#[test]
fn a_fit_page_figure_fills_the_page_and_its_alternative_text_is_structure() {
    use crate::render::{HardcopyRenderer, HardcopySceneMetadata};
    use rspice_results::report_document::PlotFigureBlock;
    use rspice_vocabulary::product::ResultDocumentId;
    let figure = ReportBlockKind::PlotFigure(PlotFigureBlock {
        caption: "Open-loop gain".to_owned(),
        alternative_text: "Gain falls from 80 dB at 10 Hz to 0 dB at 12 MHz".to_owned(),
        sizing: FigureSizing::FitPage,
        source_locator: None,
        reference: ReportReferenceMode::Frozen {
            snapshot: ReportReferenceSnapshot::new(
                ReportSourceId::VisualizationDocument {
                    document_id: ResultDocumentId::new(),
                },
                Some(ObjectRevision::INITIAL),
                ContentDigest::from_bytes([0x52; 32]),
                Vec::new(),
            )
            .unwrap(),
            artifact: bode_figure(),
        },
    });
    let document = report_document(
        ReportPublicationPageSize::UsLetterPortrait,
        vec![prose("The gain of the amplifier."), figure],
    );
    let resolved = super::super::resolve_report_source(super::super::ReportHardcopySource {
        source_key: "report".to_owned(),
        document: &document,
        reference_inventory: None,
        scope: HardcopyScope::CompleteReport,
    })
    .unwrap();
    let HardcopySemanticDocument::Report(report) = resolved.semantic_document() else {
        panic!("a report resolves to a report");
    };
    let layout = lay_out(report);
    let (index, width, height) = layout
        .pages
        .iter()
        .enumerate()
        .find_map(|(index, page)| {
            page.items.iter().find_map(|item| match item {
                Laid::Figure { width, height, .. } => Some((index, *width, *height)),
                _ => None,
            })
        })
        .unwrap();
    assert_eq!(index, 1, "the figure takes a page of its own");
    let measure = layout.width - 2 * MARGIN_UM;
    assert!(
        width + 2 >= measure || height * 10 >= (layout.height - 2 * MARGIN_UM) * 8,
        "{width} x {height} on a {measure} measure"
    );
    let printed = all_text(&layout);
    assert!(printed.contains("Open-loop gain"));
    assert!(!printed.contains("Gain falls"), "{printed}");
    let setup = resolved.report_output_setup().unwrap();
    let sections = resolved
        .hardcopy_sections_for_setup(setup.schematic())
        .unwrap();
    let scene = crate::render::scene_from_resolved(
        &resolved,
        resolved.default_print_mapping(),
        setup.schematic(),
        HardcopySceneMetadata::try_new("Report", "RSpice tests").unwrap(),
    )
    .unwrap();
    let (alternative_text, drawn) = scene
        .primitives()
        .iter()
        .find_map(|primitive| match primitive {
            crate::render::ScenePrimitive::Figure {
                alternative_text,
                primitives,
            } => Some((alternative_text, primitives)),
            _ => None,
        })
        .expect("the figure is one primitive");
    assert!(alternative_text.starts_with("Gain falls"));
    let words = drawn
        .iter()
        .filter_map(|primitive| match primitive {
            crate::render::ScenePrimitive::Text { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    for word in [
        "100",
        "1k",
        "10k",
        "100k",
        "Open loop",
        "Closed loop",
        "A",
        "FREQ \u{b7} Hz",
    ] {
        assert!(words.contains(&word), "{word:?} in {words:?}");
    }
    assert!(words.iter().any(|word| word.ends_with("dB")), "{words:?}");
    let lines = drawn
        .iter()
        .filter(|primitive| matches!(primitive, crate::render::ScenePrimitive::Polyline { .. }))
        .count();
    assert!(lines >= 2, "each trace draws as a line");
    assert!(
        !drawn.iter().any(|primitive| matches!(
            primitive,
            crate::render::ScenePrimitive::RasterImage { .. }
        )),
        "a figure is not a picture"
    );
    let plan = rspice_hardcopy_contract::HardcopyPlan::compile_with_sections(
        resolved.authority().clone(),
        setup,
        resolved.content_extent(),
        sections,
    )
    .unwrap();
    let publication = HardcopyRenderer::render_resolved(
        &plan,
        &resolved,
        HardcopySceneMetadata::try_new("Report", "RSpice tests").unwrap(),
    )
    .unwrap();
    let pdf = lopdf::Document::load_mem(publication.single_part().unwrap().bytes()).unwrap();
    let catalog = pdf.catalog().unwrap();
    assert!(catalog.get(b"StructTreeRoot").is_ok(), "the PDF is tagged");
    let alternative = pdf.objects.values().any(|object| {
        object.as_dict().is_ok_and(|dictionary| {
            dictionary.get(b"Alt").is_ok_and(|alt| {
                alt.as_str().is_ok_and(|text| {
                    String::from_utf8_lossy(text).contains("Gain falls from 80 dB")
                })
            })
        })
    });
    assert!(
        alternative,
        "the figure's alternative text is in the structure"
    );
}
