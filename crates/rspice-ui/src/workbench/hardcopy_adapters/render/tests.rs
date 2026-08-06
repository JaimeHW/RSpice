//! Cross-backend qualification tests for the hardcopy renderer.
//!
//! These cases pin semantic-scene compilation, pagination, archival output,
//! worker transfer, and exact output parity across supported formats.

use lopdf::Document as ParsedPdf;
use uuid::Uuid;

use super::*;
use crate::hardcopy::sources::{
    HardcopySourceIdentity, HardcopySourceSet, HardcopySourceSetMember,
};
use crate::hardcopy::{
    ActiveHardcopySource, DecorationSetup, DuplexMode, FontPolicy, HardcopyContentSection,
    HardcopyDocumentKind, HardcopyScope, HardcopySetup, Orientation, OutsideSheetContentPolicy,
    PageMargins, PaperSize, PhysicalPageSetup, PrintMappingEntry, PrintMappingSaveScope,
    PrintMappingTable, PrintObjectIdentity, PrinterJobSettings, PrinterMediaSource, RenderSetup,
    ScaleMode, SchematicHardcopyExtent, SchematicHardcopySetup, StandardPaper, TilingMode,
    TilingSetup,
};
use crate::state::{
    DrawingSheetManagedLogo, DrawingSheetManagedLogoPoint, DrawingSheetManagedLogoPrimitive,
    DrawingSheetManagedTemplateSnapshot, DrawingSheetStandard, DrawingSheetTitleBlockRotation,
    DrawingSheetTitleBlockTemplate, DrawingSheetTitleFieldId, DrawingSheetZoneEdges,
    DrawingSheetZoneLabels, SchematicPageOrientation, SchematicSheetFormat, SchematicState,
    SheetCatalog, SheetDefinition, SheetPageNumbering, SheetPortPolicy, SheetTemplate,
};
use crate::workbench::hardcopy_adapters::sources::{
    SchematicHardcopySource, SymbolHardcopySource, resolve_blank_schematic_sheet_with_format,
    resolve_hardcopy_source_set_with, resolve_schematic_source, resolve_symbol_source,
    schematic_sheet_identity,
};

fn digest(byte: u8) -> ContentDigest {
    ContentDigest::from_bytes([byte; 32])
}

fn reseal_preview_worker_manifest(manifest: &mut PreviewWorkerManifest) -> Vec<u8> {
    let material = PreviewWorkerManifestMaterial {
        schema_version: manifest.schema_version,
        plan_id: manifest.plan_id,
        plan_digest: manifest.plan_digest,
        source_document_id: manifest.source_document_id,
        source_revision: manifest.source_revision,
        source_digest: manifest.source_digest,
        zero_based_page: manifest.zero_based_page,
        page_number: manifest.page_number,
        coordinate: &manifest.coordinate,
        width: manifest.width,
        height: manifest.height,
        dpi: manifest.dpi,
        soft_proof_applied: manifest.soft_proof_applied,
        rgba_byte_length: manifest.rgba_byte_length,
        rgba_digest: manifest.rgba_digest,
        preview_digest: manifest.preview_digest,
    };
    manifest.transport_digest = preview_worker_material_digest(&material).unwrap();
    serde_json::to_vec(manifest).unwrap()
}

fn reseal_publication_worker_manifest(manifest: &mut PublicationWorkerManifest) -> Vec<u8> {
    let material = PublicationWorkerManifestMaterial {
        schema_version: manifest.schema_version,
        plan_digest: manifest.plan_digest,
        source_digest: manifest.source_digest,
        publication_digest: manifest.publication_digest,
        format: manifest.format,
        page_count: manifest.page_count,
        pdf_conformance: manifest.pdf_conformance,
        parts: &manifest.parts,
    };
    manifest.transport_digest = publication_worker_manifest_digest(&material).unwrap();
    serde_json::to_vec(manifest).unwrap()
}

fn extent(width_um: u64, height_um: u64) -> ContentExtent {
    ContentExtent::try_new(
        Length::from_micrometres(width_um),
        Length::from_micrometres(height_um),
    )
    .unwrap()
}

fn source() -> ActiveHardcopySource {
    ActiveHardcopySource::try_new(
        HardcopyDocumentId::try_from_uuid(Uuid::from_u128(1)).unwrap(),
        ObjectRevision::INITIAL,
        digest(0x33),
        "top · schematic",
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::CurrentSheet,
    )
    .unwrap()
}

fn setup(format: OutputFormat, tiled: bool) -> HardcopySetup {
    let target = match format {
        OutputFormat::NativePrinter => RenderTarget::SystemPrinter {
            printer_id: "test-printer".to_owned(),
            job: PrinterJobSettings::try_new(
                digest(0x44),
                "paper-letter",
                crate::hardcopy::PrinterRasterGeometry::try_new(792, 612, 0, 0, 792, 612).unwrap(),
                PrinterMediaSource::AutomaticCompatibleTray,
                72,
                DuplexMode::Off,
                1,
                false,
            )
            .unwrap(),
        },
        OutputFormat::BrowserPrintDocument => RenderTarget::BrowserPrintDialog,
        _ => RenderTarget::ExportArtifact,
    };
    let fonts = FontPolicy::new(format.is_vector(), format.is_vector());
    HardcopySetup::try_new(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(StandardPaper::Letter),
            PageMargins::uniform(Length::from_micrometres(10_000)),
            Bleed::None,
            Orientation::Landscape,
        )
        .unwrap(),
        if tiled {
            ScaleMode::EngineeringOneToOne
        } else {
            ScaleMode::FitPrintableArea
        },
        TilingSetup::try_new(TilingMode::Automatic, Length::ZERO, true).unwrap(),
        RenderSetup::try_new(
            target,
            format,
            ColorMapping::PrintSafeEngineeringPalette,
            BackgroundMode::White,
            fonts,
            true,
        )
        .unwrap(),
        DecorationSetup::try_new(true, true, true, Watermark::Draft).unwrap(),
        PrintMappingTable::try_new(
            PrintMappingSaveScope::Document,
            vec![
                PrintMappingEntry::try_new(
                    PrintObjectIdentity::try_new(
                        PrintObjectKind::Trace,
                        "trace:test",
                        "V(out)",
                        "blue solid",
                    )
                    .unwrap(),
                    PrintColor::Black,
                    PrintRedundancy::SolidLine {
                        width: Length::from_micrometres(300),
                    },
                    true,
                )
                .unwrap(),
            ],
        )
        .unwrap(),
    )
    .unwrap()
}

fn plan(format: OutputFormat, content: ContentExtent, tiled: bool) -> HardcopyPlan {
    HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(Uuid::from_u128(2)).unwrap(),
        source(),
        setup(format, tiled),
        content,
    )
    .unwrap()
}

fn aggregate_plan_and_scene(format: OutputFormat) -> (HardcopyPlan, HardcopyScene) {
    let target = match format {
        OutputFormat::NativePrinter => RenderTarget::SystemPrinter {
            printer_id: "test-printer".to_owned(),
            job: PrinterJobSettings::try_new(
                digest(0x44),
                "paper-letter",
                crate::hardcopy::PrinterRasterGeometry::try_new(792, 612, 0, 0, 792, 612).unwrap(),
                PrinterMediaSource::AutomaticCompatibleTray,
                72,
                DuplexMode::Off,
                1,
                false,
            )
            .unwrap(),
        },
        _ => RenderTarget::ExportArtifact,
    };
    let setup = HardcopySetup::try_new(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(StandardPaper::Letter),
            PageMargins::uniform(Length::from_micrometres(10_000)),
            Bleed::None,
            Orientation::AutomaticPerPage,
        )
        .unwrap(),
        ScaleMode::EngineeringOneToOne,
        TilingSetup::try_new(TilingMode::Automatic, Length::ZERO, true).unwrap(),
        RenderSetup::try_new(
            target,
            format,
            ColorMapping::PrintSafeEngineeringPalette,
            BackgroundMode::White,
            FontPolicy::new(format.is_vector(), format.is_vector()),
            true,
        )
        .unwrap(),
        DecorationSetup::try_new(false, false, false, Watermark::None).unwrap(),
        PrintMappingTable::default(),
    )
    .unwrap();
    let portrait_extent = extent(100_000, 200_000);
    let landscape_extent = extent(200_000, 100_000);
    let aggregate_extent = extent(200_000, 305_000);
    let sections = vec![
        HardcopyContentSection::try_new(
            0,
            digest(0x10),
            Length::ZERO,
            Length::ZERO,
            portrait_extent,
            false,
        )
        .unwrap(),
        HardcopyContentSection::try_new(
            1,
            digest(0x11),
            Length::ZERO,
            Length::from_micrometres(205_000),
            landscape_extent,
            true,
        )
        .unwrap(),
    ];
    let plan = HardcopyPlan::compile_with_id_and_sections(
        HardcopyPlanId::try_from_uuid(Uuid::from_u128(0x29)).unwrap(),
        source(),
        setup,
        aggregate_extent,
        sections,
    )
    .unwrap();
    let text = |origin, value: &str| ScenePrimitive::Text {
        origin,
        text: value.to_owned(),
        font: SceneFont::Sans,
        size: Length::from_micrometres(4_000),
        color: SemanticColor::Foreground,
        anchor: TextAnchor::Start,
        rotation: SceneTextRotation::Upright,
    };
    let scene = HardcopyScene {
        extent: aggregate_extent,
        metadata: HardcopySceneMetadata::try_new("Mixed pages", "RSpice tests").unwrap(),
        primitives: vec![
            text(
                ScenePoint::new(
                    Length::from_micrometres(5_000),
                    Length::from_micrometres(10_000),
                ),
                "PORTRAIT_ONLY",
            ),
            text(
                ScenePoint::new(
                    Length::from_micrometres(5_000),
                    Length::from_micrometres(215_000),
                ),
                "LANDSCAPE_ONLY",
            ),
        ],
        legend: Vec::new(),
        aggregate_sections: vec![
            AggregateSection {
                ordinal: 0,
                source_key: "child:portrait".to_owned(),
                display_name: "Portrait child".to_owned(),
                content_digest: digest(0x10),
                origin: ScenePoint::new(Length::ZERO, Length::ZERO),
                extent: portrait_extent,
                page_break_before: false,
                primitive_start: 0,
                primitive_end: 1,
            },
            AggregateSection {
                ordinal: 1,
                source_key: "child:landscape".to_owned(),
                display_name: "Landscape child".to_owned(),
                content_digest: digest(0x11),
                origin: ScenePoint::new(Length::ZERO, Length::from_micrometres(205_000)),
                extent: landscape_extent,
                page_break_before: true,
                primitive_start: 1,
                primitive_end: 2,
            },
        ],
    };
    scene.validate().unwrap();
    (plan, scene)
}

fn scene(content: ContentExtent) -> HardcopyScene {
    let mut metadata =
        HardcopySceneMetadata::try_new("Precision sensor & verification", "RSpice hardcopy tests")
            .unwrap();
    metadata.set_publication_timestamp(
        HardcopyPublicationTimestamp::try_new(2026, 7, 22, 12, 30, 0).unwrap(),
    );
    metadata
        .set_provenance_lines(vec!["retained run 17 · model digest 9f00".to_owned()])
        .unwrap();
    let trace = StrokeStyle::try_new(
        SemanticColor::Trace(1),
        Length::from_micrometres(350),
        StrokePattern::Solid,
        Some(1),
    )
    .unwrap();
    HardcopyScene::try_new(
        content,
        metadata,
        vec![
            ScenePrimitive::Line {
                from: ScenePoint::new(
                    Length::from_micrometres(5_000),
                    Length::from_micrometres(5_000),
                ),
                to: ScenePoint::new(
                    Length::from_micrometres(content.width().micrometres() - 5_000),
                    Length::from_micrometres(content.height().micrometres() - 5_000),
                ),
                stroke: trace,
            },
            ScenePrimitive::Text {
                origin: ScenePoint::new(
                    Length::from_micrometres(10_000),
                    Length::from_micrometres(20_000),
                ),
                text: "V(out) < 1.2 V".to_owned(),
                font: SceneFont::Monospace,
                size: Length::from_micrometres(4_000),
                color: SemanticColor::Foreground,
                anchor: TextAnchor::Start,
                rotation: SceneTextRotation::Upright,
            },
        ],
        vec![LegendEntry::try_new("V(out)", trace).unwrap()],
    )
    .unwrap()
}

fn resolved_symbol() -> ResolvedHardcopyDocument {
    let document = SymbolDocument {
        pins: Vec::new(),
        body: vec![SymbolShape::Polyline {
            points: vec![
                SchematicPoint::new(-20, -15),
                SchematicPoint::new(20, -15),
                SchematicPoint::new(20, 15),
                SchematicPoint::new(-20, 15),
            ],
            closed: true,
        }],
        origin: SchematicPoint::origin(),
        name_anchor: SchematicPoint::new(-20, -25),
        value_anchor: SchematicPoint::new(-20, 25),
    };
    resolve_symbol_source(SymbolHardcopySource {
        identity: HardcopySourceIdentity::try_new(
            "test-symbol",
            HardcopyDocumentId::try_from_uuid(Uuid::from_u128(77)).unwrap(),
            ObjectRevision::INITIAL,
            "Comparator symbol",
        )
        .unwrap(),
        document: &document,
        selection: None,
        scope: HardcopyScope::ActiveDocument,
    })
    .unwrap()
}

fn resolved_wide_symbol() -> ResolvedHardcopyDocument {
    let document = SymbolDocument {
        pins: Vec::new(),
        body: vec![SymbolShape::Polyline {
            points: vec![
                SchematicPoint::new(0, 0),
                SchematicPoint::new(2_000, 0),
                SchematicPoint::new(2_000, 100),
                SchematicPoint::new(0, 100),
            ],
            closed: true,
        }],
        origin: SchematicPoint::origin(),
        name_anchor: SchematicPoint::origin(),
        value_anchor: SchematicPoint::origin(),
    };
    resolve_symbol_source(SymbolHardcopySource {
        identity: HardcopySourceIdentity::try_new(
            "test-wide-symbol",
            HardcopyDocumentId::try_from_uuid(Uuid::from_u128(79)).unwrap(),
            ObjectRevision::INITIAL,
            "Wide comparator symbol",
        )
        .unwrap(),
        document: &document,
        selection: None,
        scope: HardcopyScope::ActiveDocument,
    })
    .unwrap()
}

fn resolved_blank_schematic(format: &SchematicSheetFormat) -> ResolvedHardcopyDocument {
    resolved_blank_schematic_with_identity(
        format,
        "test-schematic",
        0x5343_4845_4d41,
        "Test schematic",
    )
}

fn resolved_blank_schematic_with_identity(
    format: &SchematicSheetFormat,
    source_key: &str,
    document_id: u128,
    display_name: &str,
) -> ResolvedHardcopyDocument {
    resolve_blank_schematic_sheet_with_format(
        HardcopySourceIdentity::try_new(
            source_key,
            HardcopyDocumentId::try_from_uuid(Uuid::from_u128(document_id)).unwrap(),
            ObjectRevision::INITIAL,
            display_name,
        )
        .unwrap(),
        HardcopyScope::CurrentSheet,
        Some(format),
    )
    .unwrap()
}

fn setup_with_schematic_output(
    format: OutputFormat,
    schematic: SchematicHardcopySetup,
) -> HardcopySetup {
    let base = setup(format, false);
    HardcopySetup::try_new_with_schematic(
        base.physical_page().clone(),
        base.scale(),
        base.tiling(),
        base.render().clone(),
        base.decorations().clone(),
        schematic,
        base.print_mapping().clone(),
    )
    .unwrap()
}

fn plan_for_resolved(source: &ResolvedHardcopyDocument, format: OutputFormat) -> HardcopyPlan {
    HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(Uuid::from_u128(78)).unwrap(),
        source.authority().clone(),
        setup(format, false),
        source.content_extent(),
    )
    .unwrap()
}

fn resolved_metadata() -> HardcopySceneMetadata {
    HardcopySceneMetadata::try_new("Comparator symbol", "RSpice hardcopy tests").unwrap()
}

#[test]
fn scene_validation_rejects_geometry_outside_declared_extent() {
    let content = extent(100_000, 60_000);
    let metadata = HardcopySceneMetadata::try_new("test", "RSpice").unwrap();
    assert!(matches!(
        HardcopyScene::try_new(
            content,
            metadata,
            vec![ScenePrimitive::Line {
                from: ScenePoint::new(Length::ZERO, Length::ZERO),
                to: ScenePoint::new(
                    Length::from_micrometres(100_001),
                    Length::from_micrometres(1)
                ),
                stroke: StrokeStyle::default(),
            }],
            Vec::new(),
        ),
        Err(HardcopyRenderError::PrimitiveOutsideExtent)
    ));
}

#[test]
fn embedded_fonts_cover_engineering_symbols_and_reject_notdef() {
    let coverage = FontCoverage::load().unwrap();
    let glyphs = "Ω µ μ Δ ≥ ≤ ± ° × · √";
    coverage
        .validate_text(SceneFont::Sans, glyphs, "engineering glyph")
        .unwrap();
    for font in [SceneFont::Monospace] {
        for glyph in glyphs
            .chars()
            .filter(|character| !character.is_whitespace())
        {
            match coverage.validate_text(font, &glyph.to_string(), "engineering glyph") {
                Ok(()) => {}
                Err(HardcopyRenderError::UnsupportedGlyph { codepoint, context }) => {
                    assert_eq!(codepoint, glyph as u32);
                    assert_eq!(context, "engineering glyph");
                }
                Err(error) => panic!("unexpected font validation failure: {error}"),
            }
        }
    }
    for font in [SceneFont::Sans, SceneFont::Monospace] {
        assert!(matches!(
            coverage.validate_text(font, "∠", "engineering angle"),
            Err(HardcopyRenderError::UnsupportedGlyph {
                codepoint: 0x2220,
                context: "engineering angle"
            })
        ));
    }
    assert!(matches!(
        coverage.validate_text(SceneFont::Sans, "\u{10ffff}", "test"),
        Err(HardcopyRenderError::UnsupportedGlyph {
            codepoint: 0x10ffff,
            context: "test"
        })
    ));
}

#[test]
fn exact_cross_hatch_is_shared_by_svg_pdf_and_raster_paths() {
    let content = extent(100_000, 60_000);
    let metadata = HardcopySceneMetadata::try_new("hatch test", "RSpice").unwrap();
    let fill = SceneFill::CrossHatch {
        color: SemanticColor::Foreground,
        line_width: Length::from_micrometres(275),
        spacing: Length::from_micrometres(2_750),
    };
    let scene = HardcopyScene::try_new(
        content,
        metadata,
        vec![ScenePrimitive::Rect {
            rect: SceneRect::try_new(
                Length::from_micrometres(5_000),
                Length::from_micrometres(5_000),
                Length::from_micrometres(30_000),
                Length::from_micrometres(20_000),
            )
            .unwrap(),
            stroke: Some(StrokeStyle::default()),
            fill: Some(fill),
        }],
        Vec::new(),
    )
    .unwrap();
    let svg_plan = plan(OutputFormat::SvgVector, content, false);
    let svg_publication = HardcopyRenderer::render(&svg_plan, &scene).unwrap();
    let svg = std::str::from_utf8(svg_publication.single_part().unwrap().bytes()).unwrap();
    assert!(svg.contains("stroke-width=\"275\""));
    assert!(svg.contains("-275-2750"));
    assert!(svg.contains("fill=\"url(#hatch-"));

    let pdf_plan = plan(OutputFormat::PdfVector, content, false);
    let pdf = HardcopyRenderer::render(&pdf_plan, &scene).unwrap();
    ParsedPdf::load_mem(pdf.single_part().unwrap().bytes()).unwrap();

    let png_plan = plan(OutputFormat::Png { dpi: 72 }, content, false);
    let png = HardcopyRenderer::render(&png_plan, &scene).unwrap();
    let decoder = png::Decoder::new(Cursor::new(png.single_part().unwrap().bytes()));
    decoder.read_info().unwrap();
}

#[test]
fn printer_working_set_accounts_for_retained_rgba_and_spool_conversion() {
    let letter_1200_dpi_pixels = 13_200_u64 * 10_200;
    assert!(matches!(
        validate_printer_raster_working_set(letter_1200_dpi_pixels, 1),
        Err(HardcopyRenderError::ResourceLimit {
            scope: "raster working-set bytes",
            ..
        })
    ));
}

#[test]
fn aggregate_mapping_namespaces_colliding_child_object_identities() {
    let bounds = SemanticBounds::try_new(
        SemanticPoint::new(0, 0),
        SemanticPoint::new(100_000, 100_000),
    )
    .unwrap();
    let content = extent(100_000, 100_000);
    let empty = PrintMappingTable::default();
    let mut first_identity =
        SemanticSceneCompiler::new(bounds, content, &empty, SchematicHardcopySetup::default());
    first_identity.mapping_ordinal = Some(0);
    let mut second_identity =
        SemanticSceneCompiler::new(bounds, content, &empty, SchematicHardcopySetup::default());
    second_identity.mapping_ordinal = Some(1);
    let first_id = first_identity.mapping_stable_id("trace:shared");
    let second_id = second_identity.mapping_stable_id("trace:shared");
    assert_ne!(first_id, second_id);

    let mapping = PrintMappingTable::try_new(
        PrintMappingSaveScope::Document,
        vec![
            PrintMappingEntry::try_new(
                PrintObjectIdentity::try_new(
                    PrintObjectKind::Trace,
                    first_id,
                    "Child 1 shared trace",
                    "red",
                )
                .unwrap(),
                PrintColor::Rgb {
                    red: 220,
                    green: 20,
                    blue: 20,
                },
                PrintRedundancy::SolidLine {
                    width: Length::from_micrometres(300),
                },
                true,
            )
            .unwrap(),
            PrintMappingEntry::try_new(
                PrintObjectIdentity::try_new(
                    PrintObjectKind::Trace,
                    second_id,
                    "Child 2 shared trace",
                    "blue",
                )
                .unwrap(),
                PrintColor::Rgb {
                    red: 20,
                    green: 20,
                    blue: 220,
                },
                PrintRedundancy::SolidLine {
                    width: Length::from_micrometres(300),
                },
                true,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let fallback = StrokeStyle::default();
    let mut first =
        SemanticSceneCompiler::new(bounds, content, &mapping, SchematicHardcopySetup::default());
    first.mapping_ordinal = Some(0);
    let mut second =
        SemanticSceneCompiler::new(bounds, content, &mapping, SchematicHardcopySetup::default());
    second.mapping_ordinal = Some(1);
    assert_eq!(
        first
            .mapped_stroke(PrintObjectKind::Trace, "trace:shared", fallback)
            .color,
        SemanticColor::Exact(Rgb8::new(220, 20, 20))
    );
    assert_eq!(
        second
            .mapped_stroke(PrintObjectKind::Trace, "trace:shared", fallback)
            .color,
        SemanticColor::Exact(Rgb8::new(20, 20, 220))
    );
}

#[test]
fn unix_timestamp_conversion_is_checked_and_gregorian() {
    assert_eq!(
        HardcopyPublicationTimestamp::from_unix_seconds(0).unwrap(),
        HardcopyPublicationTimestamp::try_new(1970, 1, 1, 0, 0, 0).unwrap()
    );
    assert_eq!(
        HardcopyPublicationTimestamp::from_unix_seconds(951_782_400).unwrap(),
        HardcopyPublicationTimestamp::try_new(2000, 2, 29, 0, 0, 0).unwrap()
    );
    assert_eq!(
        HardcopyPublicationTimestamp::from_unix_seconds(253_402_300_799).unwrap(),
        HardcopyPublicationTimestamp::try_new(9999, 12, 31, 23, 59, 59).unwrap()
    );
    assert!(matches!(
        HardcopyPublicationTimestamp::from_unix_seconds(253_402_300_800),
        Err(HardcopyRenderError::InvalidTimestamp)
    ));
}

#[test]
fn svg_is_deterministic_searchable_embedded_and_xml_escaped() {
    let content = extent(100_000, 60_000);
    let plan = plan(OutputFormat::SvgVector, content, false);
    let scene = scene(content);
    let first = HardcopyRenderer::render(&plan, &scene).unwrap();
    let second = HardcopyRenderer::render(&plan, &scene).unwrap();
    assert_eq!(first, second);
    let part = first.single_part().unwrap();
    let svg = std::str::from_utf8(part.bytes()).unwrap();
    assert!(svg.contains("@font-face"));
    assert!(svg.contains("Precision sensor &amp; verification"));
    assert!(svg.contains("V(out) &lt; 1.2 V"));
    assert_eq!(part.media_type(), "image/svg+xml");
    assert_eq!(first.identity().unwrap().page_count(), 1);
}

#[test]
fn tiled_svg_is_a_complete_numbered_multi_part_publication() {
    let content = extent(500_000, 100_000);
    let plan = plan(OutputFormat::SvgVector, content, true);
    assert!(plan.pagination().pages().len() > 1);
    let publication = HardcopyRenderer::render(&plan, &scene(content)).unwrap();
    assert_eq!(publication.parts().len(), plan.pagination().pages().len());
    assert_eq!(publication.page_count(), publication.parts().len() as u32);
    for (index, part) in publication.parts().iter().enumerate() {
        assert_eq!(part.first_page(), index as u32 + 1);
        assert!(
            part.suggested_filename()
                .starts_with(&format!("page-{:04}", index + 1))
        );
        assert_eq!(part.page_count(), 1);
    }
}

#[test]
fn browser_print_document_is_self_contained_paginated_and_digest_bound() {
    let content = extent(500_000, 100_000);
    let plan = plan(OutputFormat::BrowserPrintDocument, content, true);
    let publication = HardcopyRenderer::render(&plan, &scene(content)).unwrap();
    let part = publication.single_part().unwrap();
    assert_eq!(part.media_type(), "text/html");
    assert_eq!(part.filename_extension(), "html");
    let html = std::str::from_utf8(part.bytes()).unwrap();
    assert!(html.contains("@page rspice-page-1{size:279.400mm 215.900mm;margin:0;}"));
    assert!(html.contains(
        ".rspice-print-page[data-page=\"1\"]{page:rspice-page-1;width:279.400mm;height:215.900mm}"
    ));
    assert!(html.contains(&format!(
        "name=\"rspice-plan-digest\" content=\"{}\"",
        plan.content_digest()
    )));
    assert!(html.contains(&format!(
        "name=\"rspice-source-digest\" content=\"{}\"",
        plan.source().content_digest()
    )));
    assert_eq!(
        html.matches("class=\"rspice-print-page\"").count(),
        plan.pagination().pages().len()
    );
    assert!(html.contains("@font-face"));
    assert!(html.contains("V(out) &lt; 1.2 V"));
    assert!(!html.contains("script-src"));
    assert!(!html.contains("<script"));
}

#[test]
fn aggregate_pages_render_independently_with_mixed_orientations() {
    let (plan, scene) = aggregate_plan_and_scene(OutputFormat::SvgVector);
    let publication = HardcopyRenderer::render(&plan, &scene).unwrap();
    assert_eq!(publication.parts().len(), 2);
    let portrait = std::str::from_utf8(publication.parts()[0].bytes()).unwrap();
    let landscape = std::str::from_utf8(publication.parts()[1].bytes()).unwrap();
    assert!(portrait.contains("width=\"215.900mm\" height=\"279.400mm\""));
    assert!(portrait.contains("PORTRAIT_ONLY"));
    assert!(!portrait.contains("LANDSCAPE_ONLY"));
    assert!(landscape.contains("width=\"279.400mm\" height=\"215.900mm\""));
    assert!(landscape.contains("LANDSCAPE_ONLY"));
    assert!(!landscape.contains("PORTRAIT_ONLY"));

    let (pdf_plan, pdf_scene) = aggregate_plan_and_scene(OutputFormat::PdfVector);
    let pdf = HardcopyRenderer::render(&pdf_plan, &pdf_scene).unwrap();
    assert_eq!(
        ParsedPdf::load_mem(pdf.single_part().unwrap().bytes())
            .unwrap()
            .get_pages()
            .len(),
        2
    );
}

#[test]
fn native_printer_resolves_automatic_orientation_before_rendering() {
    let (plan, scene) = aggregate_plan_and_scene(OutputFormat::NativePrinter);
    assert!(
        plan.pagination()
            .pages()
            .iter()
            .all(|page| { page.geometry().orientation() == ResolvedOrientation::Landscape })
    );
    let publication = HardcopyRenderer::render_printer_pages(&plan, &scene, 72).unwrap();
    assert_eq!(publication.pages().len(), plan.pagination().pages().len());
}

#[test]
fn png_uses_planned_physical_geometry_and_records_dpi() {
    let content = extent(100_000, 60_000);
    let plan = plan(OutputFormat::Png { dpi: 72 }, content, false);
    let publication = HardcopyRenderer::render(&plan, &scene(content)).unwrap();
    let decoder = png::Decoder::new(Cursor::new(publication.single_part().unwrap().bytes()));
    let reader = decoder.read_info().unwrap();
    let info = reader.info();
    assert_eq!((info.width, info.height), (792, 612));
    let dimensions = info.pixel_dims.unwrap();
    assert_eq!(dimensions.unit, png::Unit::Meter);
    assert!((2_830..=2_836).contains(&dimensions.xppu));
    assert_eq!(dimensions.xppu, dimensions.yppu);
}

#[test]
fn vector_pdf_is_deterministic_multi_page_and_searchable() {
    let content = extent(500_000, 100_000);
    let plan = plan(OutputFormat::PdfVector, content, true);
    let scene = scene(content);
    let first = HardcopyRenderer::render(&plan, &scene).unwrap();
    let second = HardcopyRenderer::render(&plan, &scene).unwrap();
    assert_eq!(first, second);
    let part = first.single_part().unwrap();
    let parsed = ParsedPdf::load_mem(part.bytes()).unwrap();
    assert_eq!(parsed.get_pages().len(), plan.pagination().pages().len());
    let pages = parsed.get_pages().keys().copied().collect::<Vec<_>>();
    let text = parsed.extract_text(&pages).unwrap();
    assert!(text.contains("V(out) < 1.2 V"));
    assert!(String::from_utf8_lossy(part.bytes()).contains("/FontFile2"));
}

#[test]
fn tiff_contains_every_planned_page_at_the_requested_resolution() {
    let content = extent(500_000, 100_000);
    let plan = plan(OutputFormat::Tiff { dpi: 72 }, content, true);
    let publication = HardcopyRenderer::render(&plan, &scene(content)).unwrap();
    let mut decoder =
        tiff::decoder::Decoder::new(Cursor::new(publication.single_part().unwrap().bytes()))
            .unwrap();
    let mut image_count = 1_usize;
    assert_eq!(decoder.dimensions().unwrap(), (792, 612));
    while decoder.more_images() {
        decoder.next_image().unwrap();
        assert_eq!(decoder.dimensions().unwrap(), (792, 612));
        image_count += 1;
    }
    assert_eq!(image_count, plan.pagination().pages().len());
}

#[test]
fn pdfa_is_validator_accepted_and_contains_archival_identification() {
    let content = extent(100_000, 60_000);
    let plan = plan(OutputFormat::PdfA, content, false);
    let publication = HardcopyRenderer::render(&plan, &scene(content)).unwrap();
    assert_eq!(
        publication.pdf_conformance(),
        Some(PdfConformance::PdfA2bValidated)
    );
    let raw = String::from_utf8_lossy(publication.single_part().unwrap().bytes());
    assert!(raw.contains("<pdfaid:part>2</pdfaid:part>"));
    assert!(raw.contains("<pdfaid:conformance>B</pdfaid:conformance>"));
    assert!(raw.contains("2026-07-22T12:30:00+00:00"));
}

#[test]
fn pdfa_fails_closed_without_publication_timestamp() {
    let content = extent(100_000, 60_000);
    let plan = plan(OutputFormat::PdfA, content, false);
    let metadata = HardcopySceneMetadata::try_new("test", "RSpice").unwrap();
    let scene = HardcopyScene::try_new(content, metadata, Vec::new(), Vec::new()).unwrap();
    assert!(matches!(
        HardcopyRenderer::render(&plan, &scene),
        Err(HardcopyRenderError::PdfARequiresPublicationTimestamp)
    ));
}

#[test]
fn native_printer_pages_are_opaque_and_share_canonical_pagination() {
    let content = extent(100_000, 60_000);
    let plan = plan(OutputFormat::NativePrinter, content, false);
    let rendered = HardcopyRenderer::render_printer_pages(&plan, &scene(content), 72).unwrap();
    assert_eq!(rendered.pages().len(), plan.pagination().pages().len());
    let page = &rendered.pages()[0];
    assert_eq!((page.width(), page.height()), (792, 612));
    assert_eq!(page.page_number(), 1);
    assert_eq!(page.dpi(), 72);
    assert!(page.rgba().chunks_exact(4).all(|pixel| pixel[3] == 255));
}

#[test]
fn resolved_source_compiles_to_semantic_scene_and_deterministic_preview() {
    let source = resolved_symbol();
    let plan = plan_for_resolved(&source, OutputFormat::SvgVector);
    let scene = scene_from_resolved(
        &source,
        plan.setup().print_mapping(),
        plan.setup().schematic(),
        resolved_metadata(),
    )
    .unwrap();
    assert!(!scene.primitives().is_empty());
    let first =
        HardcopyRenderer::render_preview_page_resolved(&plan, &source, resolved_metadata(), 0, 72)
            .unwrap();
    let second =
        HardcopyRenderer::render_preview_page_resolved(&plan, &source, resolved_metadata(), 0, 72)
            .unwrap();
    assert_eq!(first, second);
    assert_eq!((first.width(), first.height()), (792, 612));
    assert_eq!(first.page_number(), 1);
    assert_eq!(first.dpi(), 72);
    assert_eq!(first.rgba().len(), 792 * 612 * 4);
}

#[test]
fn schematic_inclusion_contract_gates_real_scene_primitives() {
    let source = resolved_blank_schematic(&SchematicSheetFormat::default());
    let excluded = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::Ask,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    let excluded_scene = scene_from_resolved(
        &source,
        source.default_print_mapping(),
        excluded,
        resolved_metadata(),
    )
    .unwrap();
    assert!(excluded_scene.primitives().is_empty());

    let grid_only = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::Ask,
        false,
        false,
        false,
        false,
        false,
        true,
    );
    let grid_scene = scene_from_resolved(
        &source,
        source.default_print_mapping(),
        grid_only,
        resolved_metadata(),
    )
    .unwrap();
    assert!(!grid_scene.primitives().is_empty());
    assert!(
        grid_scene
            .primitives()
            .iter()
            .all(|primitive| matches!(primitive, ScenePrimitive::Line { .. }))
    );
}

#[test]
fn schematic_clipping_policy_compiles_an_authored_sheet() {
    let source = resolved_blank_schematic(&SchematicSheetFormat::default());
    let clipping = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::ClipToAuthoredSheet,
        true,
        true,
        true,
        true,
        true,
        false,
    );

    let scene = scene_from_resolved(
        &source,
        source.default_print_mapping(),
        clipping,
        resolved_metadata(),
    )
    .unwrap();

    assert_eq!(scene.extent, source.content_extent());
}

#[test]
fn authored_sheet_bleed_compiles_for_full_and_clipped_output() {
    let format = SchematicSheetFormat::default()
        .try_update(|draft| draft.bleed_um = 5_000)
        .unwrap();
    let source = resolved_blank_schematic(&format);
    let (paper_width_um, paper_height_um) = format.oriented_dimensions_um();
    assert_eq!(
        source.content_extent().width().micrometres(),
        paper_width_um + 10_000
    );
    assert_eq!(
        source.content_extent().height().micrometres(),
        paper_height_um + 10_000
    );

    let full = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::Ask,
        true,
        true,
        true,
        true,
        true,
        false,
    );
    let full_scene = scene_from_resolved(
        &source,
        source.default_print_mapping(),
        full,
        resolved_metadata(),
    )
    .unwrap();
    assert_eq!(full_scene.extent, source.content_extent());

    let clipped = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::ClipToAuthoredSheet,
        true,
        true,
        true,
        true,
        true,
        false,
    );
    let clipped_scene = scene_from_resolved(
        &source,
        source.default_print_mapping(),
        clipped,
        resolved_metadata(),
    )
    .unwrap();
    assert_eq!(clipped_scene.extent.width().micrometres(), paper_width_um);
    assert_eq!(clipped_scene.extent.height().micrometres(), paper_height_um);
}

#[test]
fn authored_sheet_clip_projects_crossing_geometry_to_the_exact_sheet_edge() {
    let clip = ClipRect {
        x: 50,
        y: 50,
        width: 100,
        height: 100,
    };
    let stroke = StrokeStyle::default();
    let primitives = clip_scene_primitive(
        &ScenePrimitive::Line {
            from: ScenePoint::new(Length::from_micrometres(0), Length::from_micrometres(100)),
            to: ScenePoint::new(Length::from_micrometres(200), Length::from_micrometres(100)),
            stroke,
        },
        clip,
    )
    .unwrap();
    assert!(matches!(
        primitives.as_slice(),
        [ScenePrimitive::Line { from, to, .. }]
            if from.x.micrometres() == 0
                && from.y.micrometres() == 50
                && to.x.micrometres() == 100
                && to.y.micrometres() == 50
    ));
}

#[test]
fn authored_sheet_clip_preserves_partially_intersecting_non_linear_primitives() {
    let source_bounds =
        SemanticBounds::try_new(SemanticPoint::new(0, 0), SemanticPoint::new(200, 200)).unwrap();
    let sheet_bounds =
        SemanticBounds::try_new(SemanticPoint::new(50, 50), SemanticPoint::new(150, 150)).unwrap();
    let crossing = vec![
        ScenePrimitive::Circle {
            center: ScenePoint::new(Length::from_micrometres(40), Length::from_micrometres(90)),
            radius: Length::from_micrometres(20),
            stroke: Some(StrokeStyle::default()),
            fill: None,
        },
        ScenePrimitive::RasterImage {
            rect: SceneRect::try_new(
                Length::from_micrometres(40),
                Length::from_micrometres(60),
                Length::from_micrometres(30),
                Length::from_micrometres(30),
            )
            .unwrap(),
            png: vec![1, 2, 3],
            content_digest: ContentDigest::from_bytes([7; 32]),
            alternative_text: "crossing image".to_owned(),
        },
        ScenePrimitive::Text {
            origin: ScenePoint::new(Length::from_micrometres(40), Length::from_micrometres(80)),
            text: "crossing text".to_owned(),
            font: SceneFont::Sans,
            size: Length::from_micrometres(20),
            color: SemanticColor::Foreground,
            anchor: TextAnchor::Start,
            rotation: SceneTextRotation::Upright,
        },
    ];
    let clipped =
        clip_primitives_to_authored_sheet(&crossing, source_bounds, sheet_bounds).unwrap();
    let [
        ScenePrimitive::ClippedGroup {
            source_origin,
            destination_origin,
            clip_extent,
            source_extent,
            primitives,
        },
    ] = clipped.as_slice()
    else {
        panic!("authored clipping must retain one exact renderer-owned clip group");
    };
    assert_eq!(source_origin.x.micrometres(), 50);
    assert_eq!(source_origin.y.micrometres(), 50);
    assert_eq!(
        *destination_origin,
        ScenePoint::new(Length::ZERO, Length::ZERO)
    );
    assert_eq!(clip_extent.width().micrometres(), 100);
    assert_eq!(clip_extent.height().micrometres(), 100);
    assert_eq!(source_extent.width().micrometres(), 200);
    assert_eq!(source_extent.height().micrometres(), 200);
    assert_eq!(primitives, &crossing);
}

#[test]
fn coordinate_zone_mode_keeps_rules_but_suppresses_every_edge_label() {
    let mut format = SchematicSheetFormat::default();
    format.zones.labels = DrawingSheetZoneLabels::Coordinates;
    let geometry = format.geometry().unwrap();
    let source = resolved_blank_schematic(&format);
    let schematic = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::Ask,
        false,
        false,
        false,
        false,
        true,
        false,
    );
    let scene = scene_from_resolved(
        &source,
        source.default_print_mapping(),
        schematic,
        resolved_metadata(),
    )
    .unwrap();
    assert!(
        scene
            .primitives()
            .iter()
            .all(|primitive| !matches!(primitive, ScenePrimitive::Text { .. }))
    );
    assert!(
        scene
            .primitives()
            .iter()
            .all(|primitive| matches!(primitive, ScenePrimitive::Line { .. })),
        "zones-only output must own its ruled band without borrowing paper, border, or title primitives"
    );
    assert!(
        scene
            .primitives()
            .iter()
            .any(|primitive| matches!(primitive, ScenePrimitive::Line { .. }))
    );
    assert!(
        scene.primitives().iter().any(|primitive| {
            let ScenePrimitive::Line { from, to, .. } = primitive else {
                return false;
            };
            from.x.micrometres().abs_diff(to.x.micrometres()) == geometry.border_band_um
                || from.y.micrometres().abs_diff(to.y.micrometres()) == geometry.border_band_um
        }),
        "the zones-only layer must span the complete authored band even without the border layer"
    );
}

#[test]
fn hardcopy_zone_labels_follow_the_selected_output_edges() {
    let bottom_right = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft.zones.edges = DrawingSheetZoneEdges::BottomAndRight;
        })
        .unwrap();
    let geometry = bottom_right.geometry().unwrap();
    let source = resolved_blank_schematic(&bottom_right);
    let schematic = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::Ask,
        false,
        false,
        false,
        false,
        true,
        false,
    );
    let scene = scene_from_resolved(
        &source,
        source.default_print_mapping(),
        schematic,
        resolved_metadata(),
    )
    .unwrap();
    let labels = scene
        .primitives()
        .iter()
        .filter_map(|primitive| {
            let ScenePrimitive::Text { origin, .. } = primitive else {
                return None;
            };
            Some(origin)
        })
        .collect::<Vec<_>>();
    let zones = geometry.zones.unwrap();
    assert_eq!(labels.len(), usize::from(zones.columns + zones.rows));
    let drawing_right =
        u64::try_from(geometry.drawing_area.x_um + geometry.drawing_area.width_um as i64).unwrap();
    let drawing_bottom =
        u64::try_from(geometry.drawing_area.y_um + geometry.drawing_area.height_um as i64).unwrap();
    assert_eq!(
        labels
            .iter()
            .filter(|origin| origin.y.micrometres() > drawing_bottom)
            .count(),
        usize::from(zones.columns),
        "one column label must be emitted on the selected bottom edge"
    );
    assert_eq!(
        labels
            .iter()
            .filter(|origin| origin.x.micrometres() > drawing_right)
            .count(),
        usize::from(zones.rows),
        "one row label must be emitted on the selected right edge"
    );

    let all_edges = bottom_right
        .try_update(|draft| draft.zones.edges = DrawingSheetZoneEdges::All)
        .unwrap();
    let source = resolved_blank_schematic(&all_edges);
    let scene = scene_from_resolved(
        &source,
        source.default_print_mapping(),
        schematic,
        resolved_metadata(),
    )
    .unwrap();
    assert_eq!(
        scene
            .primitives()
            .iter()
            .filter(|primitive| matches!(primitive, ScenePrimitive::Text { .. }))
            .count(),
        usize::from((zones.columns + zones.rows) * 2),
        "all-edge output must publish the label set on both opposing edges"
    );
}

#[test]
fn structured_scale_authority_projects_even_when_legacy_field_storage_is_empty() {
    let format = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::Scale)
                .unwrap()
                .value
                .clear();
        })
        .unwrap();
    let source = resolved_blank_schematic(&format);
    let schematic = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::Ask,
        false,
        false,
        false,
        true,
        false,
        false,
    );
    let scene = scene_from_resolved(
        &source,
        source.default_print_mapping(),
        schematic,
        resolved_metadata(),
    )
    .unwrap();
    let text = scene
        .primitives()
        .iter()
        .filter_map(|primitive| {
            let ScenePrimitive::Text { text, .. } = primitive else {
                return None;
            };
            Some(text.as_str())
        })
        .collect::<Vec<_>>();
    assert!(text.contains(&"\u{2022} Scale"));
    assert!(text.contains(&"1:1"));
    assert!(
        !text.iter().any(|line| line.starts_with("SCALE: NTS")),
        "the structured ratio must not be replaced by an unrelated NTS value"
    );
}

#[test]
fn hardcopy_zone_alphabet_matches_engineering_drawing_overflow_rules() {
    assert_eq!(compiler::zone_alpha_label(0), "A");
    assert_eq!(compiler::zone_alpha_label(8), "J");
    assert_eq!(compiler::zone_alpha_label(21), "Y");
    assert_eq!(compiler::zone_alpha_label(22), "23");
}

#[test]
fn clipped_sheet_set_reflows_each_child_and_compiles_with_matching_sections() {
    let first = resolved_blank_schematic(&SchematicSheetFormat::default());
    let mut second_format = SchematicSheetFormat::default();
    second_format.orientation = crate::state::SchematicPageOrientation::Landscape;
    let second = resolved_blank_schematic_with_identity(
        &second_format,
        "test-schematic-second-sheet",
        0x5343_4845_4d42,
        "Test schematic · second sheet",
    );
    let members = [&first, &second]
        .into_iter()
        .map(|document| {
            HardcopySourceSetMember::try_new(
                document.source_key(),
                document.authority().display_name(),
                document.authority().document_id(),
                document.authority().revision(),
                document.authority().content_digest(),
                HardcopyScope::CurrentSheet,
            )
            .unwrap()
        })
        .collect();
    let set = HardcopySourceSet::try_new(
        HardcopyDocumentId::try_from_uuid(Uuid::from_u128(0x5151)).unwrap(),
        ObjectRevision::INITIAL,
        "Test sheet set",
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::AllSheetsOrPanes,
        members,
    )
    .unwrap();
    let mut retained = vec![first, second].into_iter();
    let source = resolve_hardcopy_source_set_with(&set, |_| Ok(retained.next().unwrap())).unwrap();
    let clipping = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::ClipToAuthoredSheet,
        false,
        true,
        true,
        true,
        true,
        false,
    );
    let extent = source.content_extent_for_setup(clipping).unwrap();
    let sections = source.hardcopy_sections_for_setup(clipping).unwrap();
    assert_eq!(sections.len(), 2);
    let setup = setup_with_schematic_output(OutputFormat::SvgVector, clipping);
    let plan =
        HardcopyPlan::compile_with_sections(source.authority().clone(), setup, extent, sections)
            .unwrap();
    let scene = scene_from_resolved(
        &source,
        plan.setup().print_mapping(),
        clipping,
        resolved_metadata(),
    )
    .unwrap();
    assert_eq!(scene.extent, plan.content_extent());
    HardcopyRenderer::render(&plan, &scene).unwrap();
}

#[test]
fn rotated_title_block_rotates_grid_text_and_published_svg() {
    let mut format = SchematicSheetFormat::default();
    format.title_block.rotation = DrawingSheetTitleBlockRotation::Clockwise90;
    let source = resolved_blank_schematic(&format);
    let schematic = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::Ask,
        false,
        false,
        false,
        true,
        false,
        false,
    );
    let setup = setup_with_schematic_output(OutputFormat::SvgVector, schematic);
    let plan =
        HardcopyPlan::compile(source.authority().clone(), setup, source.content_extent()).unwrap();
    let scene = scene_from_resolved(
        &source,
        plan.setup().print_mapping(),
        plan.setup().schematic(),
        resolved_metadata(),
    )
    .unwrap();
    let rotated_text = scene
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Text { rotation, .. } => Some(*rotation),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(!rotated_text.is_empty());
    assert!(
        rotated_text
            .iter()
            .all(|rotation| *rotation == SceneTextRotation::Clockwise90)
    );

    let publication =
        HardcopyRenderer::render_resolved(&plan, &source, resolved_metadata()).unwrap();
    let svg = String::from_utf8_lossy(publication.single_part().unwrap().bytes());
    assert!(svg.contains("transform=\"rotate(90 "));
}

#[test]
fn managed_title_logo_is_digest_bound_and_rendered_as_search_safe_vector_geometry() {
    let logo = DrawingSheetManagedLogo::try_new(
        "Example Labs mark",
        40_000,
        vec![
            DrawingSheetManagedLogoPrimitive::try_new(
                vec![
                    DrawingSheetManagedLogoPoint::try_new(1_000, 9_000).unwrap(),
                    DrawingSheetManagedLogoPoint::try_new(5_000, 1_000).unwrap(),
                    DrawingSheetManagedLogoPoint::try_new(9_000, 9_000).unwrap(),
                ],
                true,
                true,
            )
            .unwrap(),
            DrawingSheetManagedLogoPrimitive::try_new(
                vec![
                    DrawingSheetManagedLogoPoint::try_new(2_500, 7_000).unwrap(),
                    DrawingSheetManagedLogoPoint::try_new(7_500, 7_000).unwrap(),
                ],
                false,
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let snapshot = DrawingSheetManagedTemplateSnapshot::try_new_with_logo(
        "example.release-title-block",
        "2026.08",
        180_000,
        50_000,
        5,
        DrawingSheetTitleFieldId::ALL.to_vec(),
        vec![DrawingSheetTitleFieldId::Classification],
        Some(logo),
    )
    .unwrap();
    let format = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
            draft.title_block.managed_template = Some(snapshot);
        })
        .unwrap();
    let source = resolved_blank_schematic(&format);
    let schematic = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::Ask,
        true,
        true,
        true,
        true,
        true,
        false,
    );
    let setup = setup_with_schematic_output(OutputFormat::SvgVector, schematic);
    let plan =
        HardcopyPlan::compile(source.authority().clone(), setup, source.content_extent()).unwrap();
    let scene = scene_from_resolved(
        &source,
        plan.setup().print_mapping(),
        plan.setup().schematic(),
        resolved_metadata(),
    )
    .unwrap();
    assert!(scene.primitives().iter().any(|primitive| matches!(
        primitive,
        ScenePrimitive::Polyline {
            closed: true,
            fill: Some(SceneFill::Solid { .. }),
            ..
        }
    )));
    assert!(scene.primitives().iter().any(|primitive| matches!(
        primitive,
        ScenePrimitive::Polyline {
            closed: false,
            fill: None,
            ..
        }
    )));

    let publication = HardcopyRenderer::render(&plan, &scene).unwrap();
    let svg = String::from_utf8_lossy(publication.single_part().unwrap().bytes());
    assert!(svg.contains("<polygon points=\""));
    assert!(svg.contains("<polyline points=\""));
    assert!(!svg.contains("<script"));
    assert!(!svg.contains("href="));

    let pdf_source = resolved_blank_schematic(&format);
    let pdf_setup = setup_with_schematic_output(OutputFormat::PdfVector, schematic);
    let pdf_plan = HardcopyPlan::compile(
        pdf_source.authority().clone(),
        pdf_setup,
        pdf_source.content_extent(),
    )
    .unwrap();
    let pdf =
        HardcopyRenderer::render_resolved(&pdf_plan, &pdf_source, resolved_metadata()).unwrap();
    assert_eq!(
        ParsedPdf::load_mem(pdf.single_part().unwrap().bytes())
            .unwrap()
            .get_pages()
            .len(),
        1
    );

    let png_source = resolved_blank_schematic(&format);
    let png_setup = setup_with_schematic_output(OutputFormat::Png { dpi: 150 }, schematic);
    let png_plan = HardcopyPlan::compile(
        png_source.authority().clone(),
        png_setup,
        png_source.content_extent(),
    )
    .unwrap();
    let png =
        HardcopyRenderer::render_resolved(&png_plan, &png_source, resolved_metadata()).unwrap();
    let png_bytes = png.single_part().unwrap().bytes();
    let decoder = png::Decoder::new(Cursor::new(png_bytes));
    let mut reader = decoder.read_info().unwrap();
    let mut rgba = vec![0; reader.output_buffer_size().unwrap()];
    let info = reader.next_frame(&mut rgba).unwrap();
    assert_eq!(info.color_type, png::ColorType::Rgba);
    let pixels = &rgba[..info.buffer_size()];
    let dark_pixels = pixels
        .chunks_exact(4)
        .filter(|pixel| pixel[..3].iter().any(|channel| *channel < 200))
        .count();
    let pixel_count = usize::try_from(info.width).unwrap() * usize::try_from(info.height).unwrap();
    assert!(dark_pixels > pixel_count / 1_000);
    assert!(dark_pixels < pixel_count / 4);
    let minimum_filled_run = usize::try_from(info.width).unwrap() / 50;
    let mut consecutive_filled_rows = 0usize;
    let mut maximum_filled_rows = 0usize;
    for row in pixels.chunks_exact(usize::try_from(info.width).unwrap() * 4) {
        let mut run = 0usize;
        let mut longest_run = 0usize;
        for pixel in row.chunks_exact(4) {
            if pixel[..3].iter().any(|channel| *channel < 200) {
                run += 1;
                longest_run = longest_run.max(run);
            } else {
                run = 0;
            }
        }
        if longest_run >= minimum_filled_run {
            consecutive_filled_rows += 1;
            maximum_filled_rows = maximum_filled_rows.max(consecutive_filled_rows);
        } else {
            consecutive_filled_rows = 0;
        }
    }
    assert!(maximum_filled_rows > usize::try_from(info.height).unwrap() / 100);
    for (x_range, y_range) in [
        (0..info.width / 2, 0..info.height / 2),
        (info.width / 2..info.width, 0..info.height / 2),
        (0..info.width / 2, info.height / 2..info.height),
        (info.width / 2..info.width, info.height / 2..info.height),
    ] {
        let mut quadrant_has_ink = false;
        'rows: for y in y_range {
            for x in x_range.clone() {
                let index = (usize::try_from(y).unwrap() * usize::try_from(info.width).unwrap()
                    + usize::try_from(x).unwrap())
                    * 4;
                if pixels[index..index + 3]
                    .iter()
                    .any(|channel| *channel < 200)
                {
                    quadrant_has_ink = true;
                    break 'rows;
                }
            }
        }
        assert!(quadrant_has_ink);
    }
    if let Some(path) = std::env::var_os("RSPICE_SHEET_QUALIFICATION_PNG") {
        std::fs::write(path, png_bytes).unwrap();
    }
}

#[test]
fn drawing_sheet_publication_matrix_covers_every_standard_orientation_and_title_contract() {
    let schematic = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::Ask,
        true,
        true,
        true,
        true,
        true,
        true,
    );
    let title_templates = [
        DrawingSheetTitleBlockTemplate::Compact,
        DrawingSheetTitleBlockTemplate::Standard,
        DrawingSheetTitleBlockTemplate::Wide,
        DrawingSheetTitleBlockTemplate::OrganizationManaged,
        DrawingSheetTitleBlockTemplate::None,
    ];
    let rotations = [
        DrawingSheetTitleBlockRotation::Upright,
        DrawingSheetTitleBlockRotation::Clockwise90,
        DrawingSheetTitleBlockRotation::CounterClockwise90,
    ];
    let orientations = [
        SchematicPageOrientation::Portrait,
        SchematicPageOrientation::Landscape,
    ];
    let zone_labels = [
        DrawingSheetZoneLabels::AlphaNumeric,
        DrawingSheetZoneLabels::NumericAlpha,
        DrawingSheetZoneLabels::Coordinates,
    ];
    let zone_edges = [
        DrawingSheetZoneEdges::All,
        DrawingSheetZoneEdges::TopAndLeft,
        DrawingSheetZoneEdges::BottomAndRight,
    ];
    let mut cases = 0usize;

    for standard in DrawingSheetStandard::ALL {
        for orientation in orientations {
            for template in title_templates {
                for rotation in rotations {
                    let format = SchematicSheetFormat::from_standard(standard, orientation)
                            .try_update(|draft| {
                                draft.title_block.template = template;
                                draft.title_block.rotation = rotation;
                                draft.zones.labels = zone_labels[cases % zone_labels.len()];
                                draft.zones.edges = zone_edges[cases % zone_edges.len()];
                                draft
                                    .title_block
                                    .fields
                                    .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
                                    .expect("canonical title field")
                                    .value = format!("Qualification {}", cases + 1);
                            })
                            .unwrap_or_else(|error| {
                                panic!(
                                    "invalid matrix format for {} {} {template:?} {rotation:?}: {error}",
                                    standard.label(),
                                    orientation.label()
                                )
                            });
                    let geometry = format.geometry().unwrap_or_else(|error| {
                        panic!(
                            "invalid matrix geometry for {} {} {template:?} {rotation:?}: {error}",
                            standard.label(),
                            orientation.label()
                        )
                    });
                    assert!(geometry.paper.width_um > 0);
                    assert!(geometry.paper.height_um > 0);
                    assert!(geometry.drawing_area.width_um > 0);
                    assert!(geometry.drawing_area.height_um > 0);

                    let source = resolved_blank_schematic(&format);
                    let setup = setup_with_schematic_output(OutputFormat::SvgVector, schematic);
                    let plan = HardcopyPlan::compile(
                        source.authority().clone(),
                        setup,
                        source.content_extent(),
                    )
                    .unwrap_or_else(|error| {
                        panic!(
                            "matrix plan failed for {} {} {template:?} {rotation:?}: {error}",
                            standard.label(),
                            orientation.label()
                        )
                    });
                    let scene = scene_from_resolved(
                        &source,
                        plan.setup().print_mapping(),
                        plan.setup().schematic(),
                        resolved_metadata(),
                    )
                    .unwrap_or_else(|error| {
                        panic!(
                            "matrix scene failed for {} {} {template:?} {rotation:?}: {error}",
                            standard.label(),
                            orientation.label()
                        )
                    });
                    assert_eq!(scene.extent, plan.content_extent());
                    HardcopyRenderer::render(&plan, &scene).unwrap_or_else(|error| {
                            panic!(
                                "matrix publication failed for {} {} {template:?} {rotation:?}: {error}",
                                standard.label(),
                                orientation.label()
                            )
                        });
                    cases += 1;
                }
            }
        }
    }

    assert_eq!(cases, 600);
}

#[test]
fn preview_worker_transfer_round_trips_without_pixel_json() {
    let source = resolved_symbol();
    let plan = plan_for_resolved(&source, OutputFormat::SvgVector);
    let preview =
        HardcopyRenderer::render_preview_page_resolved(&plan, &source, resolved_metadata(), 0, 72)
            .unwrap();
    let transfer = preview
        .clone()
        .into_worker_transfer(&plan, &source, 0)
        .unwrap();
    let (manifest_json, rgba) = transfer.into_parts();
    assert_eq!(rgba, preview.rgba());
    assert!(manifest_json.len() < 4_096);
    let manifest_value: serde_json::Value = serde_json::from_slice(&manifest_json).unwrap();
    assert!(manifest_value.get("rgba").is_none());
    assert!(manifest_value.get("rgba_base64").is_none());
    assert_eq!(
        HardcopyPreviewPage::from_worker_transfer(&plan, &source, 0, 72, &manifest_json, rgba,)
            .unwrap(),
        preview
    );
}

#[test]
fn preview_worker_transfer_rejects_manifest_payload_and_preview_tampering() {
    let source = resolved_symbol();
    let plan = plan_for_resolved(&source, OutputFormat::SvgVector);
    let preview =
        HardcopyRenderer::render_preview_page_resolved(&plan, &source, resolved_metadata(), 0, 72)
            .unwrap();
    let (manifest_json, rgba) = preview
        .into_worker_transfer(&plan, &source, 0)
        .unwrap()
        .into_parts();

    let mut unknown_field: serde_json::Value = serde_json::from_slice(&manifest_json).unwrap();
    unknown_field["unexpected"] = serde_json::Value::Bool(true);
    assert!(matches!(
        HardcopyPreviewPage::from_worker_transfer(
            &plan,
            &source,
            0,
            72,
            &serde_json::to_vec(&unknown_field).unwrap(),
            rgba.clone(),
        ),
        Err(HardcopyRenderError::WorkerSnapshot(_))
    ));

    let mut changed_pixels = rgba.clone();
    changed_pixels[0] ^= 0xff;
    let mut pixel_manifest: PreviewWorkerManifest = serde_json::from_slice(&manifest_json).unwrap();
    pixel_manifest.rgba_digest = ContentDigest::from_bytes(Sha256::digest(&changed_pixels).into());
    let pixel_manifest_json = reseal_preview_worker_manifest(&mut pixel_manifest);
    assert!(matches!(
        HardcopyPreviewPage::from_worker_transfer(
            &plan,
            &source,
            0,
            72,
            &pixel_manifest_json,
            changed_pixels,
        ),
        Err(HardcopyRenderError::WorkerSnapshot(_))
    ));

    let mut digest_manifest: PreviewWorkerManifest =
        serde_json::from_slice(&manifest_json).unwrap();
    digest_manifest.preview_digest = digest(0xa5);
    let digest_manifest_json = reseal_preview_worker_manifest(&mut digest_manifest);
    assert!(matches!(
        HardcopyPreviewPage::from_worker_transfer(
            &plan,
            &source,
            0,
            72,
            &digest_manifest_json,
            rgba,
        ),
        Err(HardcopyRenderError::WorkerSnapshot(_))
    ));
}

#[test]
fn preview_worker_transfer_strictly_checks_caller_authority_geometry_and_budgets() {
    let source = resolved_symbol();
    let plan = plan_for_resolved(&source, OutputFormat::SvgVector);
    let preview =
        HardcopyRenderer::render_preview_page_resolved(&plan, &source, resolved_metadata(), 0, 72)
            .unwrap();
    let (manifest_json, rgba) = preview
        .into_worker_transfer(&plan, &source, 0)
        .unwrap()
        .into_parts();
    let original_manifest: PreviewWorkerManifest = serde_json::from_slice(&manifest_json).unwrap();

    let mutations: [fn(&mut PreviewWorkerManifest); 15] = [
        |manifest| manifest.schema_version += 1,
        |manifest: &mut PreviewWorkerManifest| manifest.plan_id = HardcopyPlanId::new(),
        |manifest| manifest.plan_digest = digest(0x91),
        |manifest| manifest.source_document_id = HardcopyDocumentId::new(),
        |manifest| manifest.source_revision = ObjectRevision::new(2).unwrap(),
        |manifest| manifest.source_digest = digest(0x92),
        |manifest: &mut PreviewWorkerManifest| manifest.zero_based_page = 1,
        |manifest: &mut PreviewWorkerManifest| manifest.page_number = 2,
        |manifest| manifest.coordinate.push('x'),
        |manifest: &mut PreviewWorkerManifest| manifest.width += 1,
        |manifest| manifest.height += 1,
        |manifest: &mut PreviewWorkerManifest| manifest.dpi = 73,
        |manifest| manifest.soft_proof_applied = !manifest.soft_proof_applied,
        |manifest| manifest.rgba_byte_length += 4,
        |manifest| manifest.rgba_digest = digest(0x93),
    ];
    for mutate in mutations {
        let mut candidate = original_manifest.clone();
        mutate(&mut candidate);
        let candidate_json = reseal_preview_worker_manifest(&mut candidate);
        assert!(
            HardcopyPreviewPage::from_worker_transfer(
                &plan,
                &source,
                0,
                72,
                &candidate_json,
                rgba.clone(),
            )
            .is_err()
        );
    }
    let mut transport_tamper = original_manifest;
    transport_tamper.transport_digest = digest(0x94);
    assert!(
        HardcopyPreviewPage::from_worker_transfer(
            &plan,
            &source,
            0,
            72,
            &serde_json::to_vec(&transport_tamper).unwrap(),
            rgba.clone(),
        )
        .is_err()
    );
    assert!(
        HardcopyPreviewPage::from_worker_transfer(
            &plan,
            &source,
            0,
            73,
            &manifest_json,
            rgba.clone(),
        )
        .is_err()
    );
    assert!(matches!(
        HardcopyPreviewPage::from_worker_transfer(&plan, &source, 0, 72, &[], rgba),
        Err(HardcopyRenderError::WorkerSnapshot(_))
    ));
    assert!(matches!(
        validate_preview_worker_transfer_budget(MAX_PREVIEW_WORKER_MANIFEST_BYTES + 1, 0),
        Err(HardcopyRenderError::WorkerSnapshotTooLarge)
    ));
    assert!(matches!(
        validate_preview_worker_transfer_budget(0, MAX_PREVIEW_WORKER_RGBA_BYTES + 1),
        Err(HardcopyRenderError::WorkerSnapshotTooLarge)
    ));
}

#[test]
fn publication_worker_transfer_round_trips_without_payload_json() {
    let source = resolved_wide_symbol();
    let plan = HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(Uuid::from_u128(81)).unwrap(),
        source.authority().clone(),
        setup(OutputFormat::SvgVector, true),
        source.content_extent(),
    )
    .unwrap();
    let publication = HardcopyRenderer::render_resolved(
        &plan,
        &source,
        HardcopySceneMetadata::try_new("Wide comparator symbol", "RSpice tests").unwrap(),
    )
    .unwrap();
    assert!(publication.parts().len() >= 2);
    let transfer = publication
        .clone()
        .into_worker_transfer(&plan, &source)
        .unwrap();
    let (manifest, payloads) = transfer.into_parts();
    assert!(manifest.len() < payloads.iter().map(Vec::len).sum::<usize>());
    for payload in &payloads {
        assert!(payload.len() >= 16);
        assert!(!manifest.windows(16).any(|window| window == &payload[..16]));
    }
    let restored =
        RenderedHardcopyPublication::from_worker_transfer(&plan, &source, &manifest, payloads)
            .unwrap();
    assert_eq!(restored, publication);
}

#[test]
fn publication_worker_transfer_rejects_payload_manifest_and_plan_tampering() {
    let source = resolved_symbol();
    let resolved_plan = plan_for_resolved(&source, OutputFormat::PdfVector);
    let publication =
        HardcopyRenderer::render_resolved(&resolved_plan, &source, resolved_metadata()).unwrap();
    let (manifest, mut payloads) = publication
        .clone()
        .into_worker_transfer(&resolved_plan, &source)
        .unwrap()
        .into_parts();
    payloads[0][0] ^= 0x01;
    assert!(matches!(
        RenderedHardcopyPublication::from_worker_transfer(
            &resolved_plan,
            &source,
            &manifest,
            payloads
        ),
        Err(HardcopyRenderError::PublicationWorkerTransfer(_))
    ));

    let (mut manifest, payloads) = publication
        .clone()
        .into_worker_transfer(&resolved_plan, &source)
        .unwrap()
        .into_parts();
    let marker = manifest
        .windows(b"publication_digest".len())
        .position(|window| window == b"publication_digest")
        .unwrap();
    manifest[marker] = b'P';
    assert!(matches!(
        RenderedHardcopyPublication::from_worker_transfer(
            &resolved_plan,
            &source,
            &manifest,
            payloads
        ),
        Err(HardcopyRenderError::PublicationWorkerTransfer(_))
    ));

    let unrelated = plan(OutputFormat::PdfVector, source.content_extent(), false);
    let (manifest, payloads) = publication
        .into_worker_transfer(&resolved_plan, &source)
        .unwrap()
        .into_parts();
    assert!(matches!(
        RenderedHardcopyPublication::from_worker_transfer(&unrelated, &source, &manifest, payloads),
        Err(HardcopyRenderError::SourceAuthorityMismatch)
    ));
}

#[test]
fn publication_worker_transfer_strictly_validates_every_authority_and_part_field() {
    let source = resolved_symbol();
    let plan = plan_for_resolved(&source, OutputFormat::PdfVector);
    let publication =
        HardcopyRenderer::render_resolved(&plan, &source, resolved_metadata()).unwrap();
    let (manifest_json, payloads) = publication
        .into_worker_transfer(&plan, &source)
        .unwrap()
        .into_parts();
    let manifest: PublicationWorkerManifest = serde_json::from_slice(&manifest_json).unwrap();
    assert!(matches!(
        RenderedHardcopyPublication::from_worker_transfer(&plan, &source, &[], payloads.clone()),
        Err(HardcopyRenderError::PublicationWorkerTransfer(_))
    ));
    let rejects = |mut candidate: PublicationWorkerManifest| {
        let encoded = reseal_publication_worker_manifest(&mut candidate);
        matches!(
            RenderedHardcopyPublication::from_worker_transfer(
                &plan,
                &source,
                &encoded,
                payloads.clone()
            ),
            Err(HardcopyRenderError::PublicationWorkerTransfer(_))
        )
    };

    let mut changed = manifest.clone();
    changed.plan_digest = digest(0x90);
    assert!(rejects(changed));

    let mut changed = manifest.clone();
    changed.source_digest = digest(0x91);
    assert!(rejects(changed));

    let mut changed = manifest.clone();
    changed.publication_digest = digest(0x92);
    assert!(rejects(changed));

    let mut changed = manifest.clone();
    changed.format = OutputFormat::PdfA;
    changed.pdf_conformance = Some(PdfConformance::PdfA2bValidated);
    assert!(rejects(changed));

    let mut changed = manifest.clone();
    changed.page_count += 1;
    assert!(rejects(changed));

    let mut changed = manifest.clone();
    changed.parts[0].byte_length += 1;
    assert!(rejects(changed));

    let mut changed = manifest.clone();
    changed.parts[0].digest = digest(0x93);
    assert!(rejects(changed));

    let mut changed = manifest.clone();
    changed.parts[0].first_page += 1;
    assert!(rejects(changed));

    let mut changed = manifest.clone();
    changed.parts[0].page_count += 1;
    assert!(rejects(changed));

    let mut unknown_field: serde_json::Value = serde_json::from_slice(&manifest_json).unwrap();
    unknown_field["unexpected"] = serde_json::Value::Bool(true);
    assert!(matches!(
        RenderedHardcopyPublication::from_worker_transfer(
            &plan,
            &source,
            &serde_json::to_vec(&unknown_field).unwrap(),
            payloads
        ),
        Err(HardcopyRenderError::PublicationWorkerTransfer(_))
    ));
}

#[test]
fn bounded_preview_batch_resolves_once_and_cancels_between_pages() {
    let source = resolved_wide_symbol();
    let plan = HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(Uuid::from_u128(80)).unwrap(),
        source.authority().clone(),
        setup(OutputFormat::SvgVector, true),
        source.content_extent(),
    )
    .unwrap();
    assert!(plan.pagination().pages().len() >= 2);
    let batch = HardcopyRenderer::render_preview_pages_resolved(
        &plan,
        &source,
        resolved_metadata(),
        &[0, 1],
        72,
        || false,
    )
    .unwrap();
    assert_eq!(batch.len(), 2);
    assert_ne!(batch[0].digest(), batch[1].digest());
    let selected =
        HardcopyRenderer::render_preview_page_resolved(&plan, &source, resolved_metadata(), 0, 72)
            .unwrap();
    assert_eq!(batch[0], selected);

    let cancelled = HardcopyRenderer::render_preview_pages_resolved(
        &plan,
        &source,
        resolved_metadata(),
        &[0, 1],
        72,
        || true,
    )
    .unwrap();
    assert_eq!(cancelled, vec![selected]);
    assert!(matches!(
        HardcopyRenderer::render_preview_pages_resolved(
            &plan,
            &source,
            resolved_metadata(),
            &[0, 0],
            72,
            || false
        ),
        Err(HardcopyRenderError::InvalidPreviewPageBatch)
    ));
}

#[test]
fn resolved_preview_rejects_authority_index_and_resolution_mismatch() {
    let source = resolved_symbol();
    let resolved_plan = plan_for_resolved(&source, OutputFormat::PdfVector);
    let unrelated = plan(OutputFormat::PdfVector, source.content_extent(), false);
    assert!(matches!(
        HardcopyRenderer::render_preview_page_resolved(
            &unrelated,
            &source,
            resolved_metadata(),
            0,
            72
        ),
        Err(HardcopyRenderError::SourceAuthorityMismatch)
    ));
    assert!(matches!(
        HardcopyRenderer::render_preview_page_resolved(
            &resolved_plan,
            &source,
            resolved_metadata(),
            1,
            72
        ),
        Err(HardcopyRenderError::PreviewPageOutOfRange { .. })
    ));
    assert!(matches!(
        HardcopyRenderer::render_preview_page_resolved(
            &resolved_plan,
            &source,
            resolved_metadata(),
            0,
            12
        ),
        Err(HardcopyRenderError::InvalidPreviewDpi(12))
    ));
}

#[test]
fn resolved_native_printer_pages_succeed_and_reject_stale_authority() {
    let source = resolved_symbol();
    let resolved_plan = plan_for_resolved(&source, OutputFormat::NativePrinter);
    let pages = HardcopyRenderer::render_printer_pages_resolved(
        &resolved_plan,
        &source,
        resolved_metadata(),
        72,
    )
    .unwrap();
    assert_eq!(
        pages.pages().len(),
        resolved_plan.pagination().pages().len()
    );
    assert!(
        pages.pages()[0]
            .rgba()
            .chunks_exact(4)
            .all(|pixel| pixel[3] == 255)
    );
    let unrelated = plan(OutputFormat::NativePrinter, source.content_extent(), false);
    assert!(matches!(
        HardcopyRenderer::render_printer_pages_resolved(
            &unrelated,
            &source,
            resolved_metadata(),
            72
        ),
        Err(HardcopyRenderError::SourceAuthorityMismatch)
    ));
}

#[test]
fn grayscale_trace_policy_adds_redundant_dash_encoding() {
    let content = extent(100_000, 60_000);
    let base = setup(OutputFormat::SvgVector, false);
    let setup = HardcopySetup::try_new(
        base.physical_page().clone(),
        base.scale(),
        base.tiling(),
        RenderSetup::try_new(
            RenderTarget::ExportArtifact,
            OutputFormat::SvgVector,
            ColorMapping::GrayscaleWithDashMarkerRedundancy,
            BackgroundMode::White,
            FontPolicy::new(true, true),
            true,
        )
        .unwrap(),
        base.decorations().clone(),
        base.print_mapping().clone(),
    )
    .unwrap();
    let plan = HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(Uuid::from_u128(3)).unwrap(),
        source(),
        setup,
        content,
    )
    .unwrap();
    let publication = HardcopyRenderer::render(&plan, &scene(content)).unwrap();
    let svg = std::str::from_utf8(publication.single_part().unwrap().bytes()).unwrap();
    assert!(svg.contains("stroke-dasharray"));
}

#[test]
fn gray_percent_is_black_ink_coverage_not_reflected_channel_value() {
    assert_eq!(
        print_color_rgb(PrintColor::GrayPercent(70)),
        Rgb8::new(77, 77, 77)
    );
    assert_eq!(
        print_color_rgb(PrintColor::GrayPercent(40)),
        Rgb8::new(153, 153, 153)
    );
    assert_eq!(
        print_color_rgb(PrintColor::GrayPercent(60)),
        Rgb8::new(102, 102, 102)
    );
}

#[test]
fn named_print_set_projects_per_set_sheet_numbers_without_mutating_child_authority() {
    let sheet = |name: &str| SheetDefinition {
        name: name.to_owned(),
        template: SheetTemplate::AnalogSchematic,
        port_policy: SheetPortPolicy::TypedOffSheetPorts,
        explicit_page_number: None,
    };
    let schematic = SchematicState::default();
    let mut catalog = SheetCatalog::default();
    let first = catalog.create_sheet(sheet("First"), None).unwrap();
    let second = catalog.create_sheet(sheet("Second"), Some(first)).unwrap();
    let third = catalog.create_sheet(sheet("Third"), Some(second)).unwrap();
    let mut settings = catalog.settings().clone();
    settings.page_numbering = SheetPageNumbering::PerPrintSet;
    catalog.set_settings(catalog.revision(), settings).unwrap();
    let project_settings = crate::state::DrawingSheetProjectSettings::default();
    let base_identity = HardcopySourceIdentity::try_new(
        "named-set-schematic",
        HardcopyDocumentId::new(),
        ObjectRevision::INITIAL,
        "Active document",
    )
    .unwrap();
    let mut selected = [second, third]
        .into_iter()
        .map(|sheet_id| {
            resolve_schematic_source(SchematicHardcopySource {
                identity: schematic_sheet_identity(&base_identity, catalog.find(sheet_id).unwrap())
                    .unwrap(),
                schematic: &schematic,
                expected_topology_version: schematic.topology_version(),
                symbol_resolver: None,
                sheet_catalog: Some(&catalog),
                sheet_id: Some(sheet_id),
                project_default_drawing_sheet: Some(&project_settings.default_format),
                project_title_block_field_values: Some(&project_settings.title_block_field_values),
                scope: HardcopyScope::CurrentSheet,
            })
            .unwrap()
        })
        .collect::<Vec<_>>();
    for (resolved, expected) in selected.iter().zip(["2 of 3", "3 of 3"]) {
        let HardcopySemanticDocument::Schematic(schematic) = resolved.semantic_document() else {
            panic!("expected governed schematic sheet")
        };
        assert_eq!(
            schematic
                .drawing_sheet_title_values
                .get(&DrawingSheetTitleFieldId::Page)
                .map(String::as_str),
            Some(expected),
            "the retained child keeps its catalog-relative source semantics"
        );
    }
    let members = selected
        .iter()
        .map(HardcopySourceSetMember::from_resolved)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let source_set = HardcopySourceSet::try_new(
        HardcopyDocumentId::new(),
        ObjectRevision::INITIAL,
        "Release subset",
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::NamedPrintSet("Release subset".to_owned()),
        members,
    )
    .unwrap();
    let mut selected = selected.drain(..);
    let aggregate = resolve_hardcopy_source_set_with(&source_set, |_| {
        Ok(selected.next().expect("one exact retained set member"))
    })
    .unwrap();
    let HardcopySemanticDocument::Aggregate(semantic) = aggregate.semantic_document() else {
        panic!("expected named aggregate")
    };
    assert_eq!(
        semantic
            .children
            .iter()
            .map(|child| child.publication_page_label.as_deref())
            .collect::<Vec<_>>(),
        [Some("1 of 2"), Some("2 of 2")]
    );

    let scene = scene_from_resolved(
        &aggregate,
        aggregate.default_print_mapping(),
        SchematicHardcopySetup::default(),
        HardcopySceneMetadata::for_resolved_source(&aggregate, "RSpice test").unwrap(),
    )
    .unwrap();
    let printed_text = scene
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Text { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(printed_text.contains(&"1 of 2"));
    assert!(printed_text.contains(&"2 of 2"));
    assert!(!printed_text.contains(&"2 of 3"));
    assert!(!printed_text.contains(&"3 of 3"));
}
