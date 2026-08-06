//! Tests for hardcopy outcome truthfulness.
//!
//! A receipt must describe what actually happened - the right page count and
//! format, a spool acceptance bound to the device that accepted it, and
//! cancellation and failure as explicit validated outcomes rather than
//! silence.

use super::*;

fn digest(byte: u8) -> ContentDigest {
    ContentDigest::from_bytes([byte; 32])
}

fn printer_geometry() -> PrinterRasterGeometry {
    PrinterRasterGeometry::try_new(10_200, 13_200, 0, 0, 10_200, 13_200).unwrap()
}

fn id(value: u128) -> HardcopyDocumentId {
    HardcopyDocumentId::try_from_uuid(Uuid::from_u128(value)).expect("document id")
}

fn source_with(
    document_id: HardcopyDocumentId,
    kind: HardcopyDocumentKind,
    scope: HardcopyScope,
) -> ActiveHardcopySource {
    ActiveHardcopySource::try_new(
        document_id,
        ObjectRevision::INITIAL,
        digest(0x42),
        "top · schematic",
        kind,
        scope,
    )
    .expect("source")
}

fn source() -> ActiveHardcopySource {
    source_with(
        id(1),
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::CurrentSheet,
    )
}

fn one_to_one_setup(tiling: TilingMode, overlap: Length) -> HardcopySetup {
    HardcopySetup::try_new(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(StandardPaper::Letter),
            PageMargins::uniform(Length::from_micrometres(10_000)),
            Bleed::None,
            Orientation::Landscape,
        )
        .unwrap(),
        ScaleMode::EngineeringOneToOne,
        TilingSetup::try_new(tiling, overlap, true).unwrap(),
        RenderSetup::try_new(
            RenderTarget::ExportArtifact,
            OutputFormat::PdfVector,
            ColorMapping::PrintSafeEngineeringPalette,
            BackgroundMode::White,
            FontPolicy::new(true, true),
            true,
        )
        .unwrap(),
        DecorationSetup::try_new(false, false, false, Watermark::None).unwrap(),
        PrintMappingTable::default(),
    )
    .unwrap()
}

fn printer_job() -> PrinterJobSettings {
    PrinterJobSettings::try_new(
        digest(0x35),
        "paper-1",
        printer_geometry(),
        PrinterMediaSource::AutomaticCompatibleTray,
        1_200,
        DuplexMode::Off,
        1,
        false,
    )
    .unwrap()
}

#[test]
fn decimal_lengths_are_exact_and_unit_typed() {
    assert_eq!(
        Length::parse_decimal("0.25", LengthUnit::Inches).unwrap(),
        Length::from_micrometres(6_350)
    );
    assert_eq!(
        Length::parse_decimal("210", LengthUnit::Millimetres).unwrap(),
        Length::from_micrometres(210_000)
    );
    assert!(Length::parse_decimal("1 in", LengthUnit::Inches).is_err());
    assert!(Length::parse_decimal("-1", LengthUnit::Inches).is_err());
}

#[test]
fn scope_is_checked_against_document_kind() {
    assert!(
        ActiveHardcopySource::try_new(
            id(2),
            ObjectRevision::INITIAL,
            digest(1),
            "waveform",
            HardcopyDocumentKind::PlotOrWorksheet,
            HardcopyScope::ActivePlotDocument,
        )
        .is_ok()
    );
    assert!(matches!(
        ActiveHardcopySource::try_new(
            id(3),
            ObjectRevision::INITIAL,
            digest(1),
            "waveform",
            HardcopyDocumentKind::PlotOrWorksheet,
            HardcopyScope::CurrentSheet,
        ),
        Err(HardcopyError::IncompatibleScope { .. })
    ));
}

#[test]
fn custom_paper_and_bleed_fail_closed() {
    let paper = CustomPaper::try_new(
        "Engineering custom 11×17",
        Length::parse_decimal("17", LengthUnit::Inches).unwrap(),
        Length::parse_decimal("11", LengthUnit::Inches).unwrap(),
        LengthUnit::Inches,
    )
    .unwrap();
    assert_eq!(paper.display_unit(), LengthUnit::Inches);
    assert!(
        PhysicalPageSetup::try_new(
            PaperSize::Custom(paper),
            PageMargins::uniform(Length::from_micrometres(5_000)),
            Bleed::Uniform(Length::from_micrometres(6_000)),
            Orientation::Landscape,
        )
        .is_err()
    );
}

#[test]
fn decoration_bands_are_reserved_from_printable_geometry() {
    let plan = HardcopyPlan::compile(
        source(),
        HardcopySetup::default(),
        ContentExtent::try_new(
            Length::from_micrometres(400_000),
            Length::from_micrometres(200_000),
        )
        .unwrap(),
    )
    .unwrap();
    let geometry = plan.pagination().geometry();
    assert_eq!(geometry.orientation(), ResolvedOrientation::Landscape);
    assert_eq!(
        geometry.header_band(),
        Length::from_micrometres(HEADER_BAND_UM)
    );
    assert_eq!(
        geometry.provenance_band(),
        Length::from_micrometres(PROVENANCE_BAND_UM)
    );
    assert!(geometry.content_rect().height < geometry.printable_rect().height);
}

#[test]
fn automatic_pagination_is_row_major_with_engineering_coordinates() {
    let setup = one_to_one_setup(TilingMode::Automatic, Length::from_micrometres(10_000));
    let plan = HardcopyPlan::compile(
        source(),
        setup,
        ContentExtent::try_new(
            Length::from_micrometres(400_000),
            Length::from_micrometres(100_000),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(plan.pagination().columns(), 2);
    assert_eq!(plan.pagination().rows(), 1);
    assert_eq!(plan.pagination().pages()[0].coordinate(), "A1");
    assert_eq!(plan.pagination().pages()[1].coordinate(), "B1");
    assert_eq!(
        plan.pagination().pages()[1].scaled_content_window().x,
        Length::from_micrometres(249_400)
    );
}

#[test]
fn single_page_and_manual_tiling_reject_uncovered_content() {
    let content = ContentExtent::try_new(
        Length::from_micrometres(400_000),
        Length::from_micrometres(100_000),
    )
    .unwrap();
    assert!(matches!(
        HardcopyPlan::compile(
            source(),
            one_to_one_setup(TilingMode::SinglePage, Length::ZERO),
            content,
        ),
        Err(HardcopyError::SinglePageOverflow {
            required_columns: 2,
            ..
        })
    ));
    assert!(matches!(
        HardcopyPlan::compile(
            source(),
            one_to_one_setup(
                TilingMode::Manual {
                    columns: 3,
                    rows: 1
                },
                Length::from_micrometres(10_000),
            ),
            content,
        ),
        Err(HardcopyError::ManualTilingDoesNotCover {
            columns: 3,
            required_columns: 2,
            ..
        })
    ));
}

#[test]
fn automatic_orientation_follows_selected_content_aspect() {
    let mut setup = one_to_one_setup(TilingMode::Automatic, Length::ZERO);
    setup.physical_page.orientation = Orientation::AutomaticPerPage;
    let portrait = HardcopyPlan::compile(
        source(),
        setup.clone(),
        ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(200_000),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        portrait.pagination().geometry().orientation(),
        ResolvedOrientation::Portrait
    );
    let landscape = HardcopyPlan::compile(
        source(),
        setup,
        ContentExtent::try_new(
            Length::from_micrometres(200_000),
            Length::from_micrometres(100_000),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        landscape.pagination().geometry().orientation(),
        ResolvedOrientation::Landscape
    );
}

#[test]
fn aggregate_sections_preserve_mixed_automatic_page_orientations() {
    let mut setup = one_to_one_setup(TilingMode::Automatic, Length::ZERO);
    setup.physical_page.orientation = Orientation::AutomaticPerPage;
    let portrait_extent = ContentExtent::try_new(
        Length::from_micrometres(100_000),
        Length::from_micrometres(200_000),
    )
    .unwrap();
    let landscape_extent = ContentExtent::try_new(
        Length::from_micrometres(200_000),
        Length::from_micrometres(100_000),
    )
    .unwrap();
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
    let plan = HardcopyPlan::compile_with_sections(
        source(),
        setup,
        ContentExtent::try_new(
            Length::from_micrometres(200_000),
            Length::from_micrometres(305_000),
        )
        .unwrap(),
        sections,
    )
    .unwrap();
    assert_eq!(plan.pagination().pages().len(), 2);
    assert_eq!(
        plan.pagination().pages()[0].geometry().orientation(),
        ResolvedOrientation::Portrait
    );
    assert_eq!(
        plan.pagination().pages()[1].geometry().orientation(),
        ResolvedOrientation::Landscape
    );
    assert_eq!(plan.pagination().pages()[0].coordinate(), "S1-A1");
    assert_eq!(plan.pagination().pages()[1].coordinate(), "S2-A1");
    assert_eq!(
        plan.pagination().pages()[1].scaled_content_window().y,
        Length::from_micrometres(205_000)
    );
}

#[test]
fn heterogeneous_aggregate_grids_publish_a_truthful_linear_page_summary() {
    let setup = one_to_one_setup(TilingMode::Automatic, Length::ZERO);
    let wide_extent = ContentExtent::try_new(
        Length::from_micrometres(400_000),
        Length::from_micrometres(100_000),
    )
    .unwrap();
    let compact_extent = ContentExtent::try_new(
        Length::from_micrometres(100_000),
        Length::from_micrometres(100_000),
    )
    .unwrap();
    let sections = vec![
        HardcopyContentSection::try_new(
            0,
            digest(0x20),
            Length::ZERO,
            Length::ZERO,
            wide_extent,
            false,
        )
        .unwrap(),
        HardcopyContentSection::try_new(
            1,
            digest(0x21),
            Length::ZERO,
            Length::from_micrometres(105_000),
            compact_extent,
            true,
        )
        .unwrap(),
    ];
    let plan = HardcopyPlan::compile_with_sections(
        source(),
        setup,
        ContentExtent::try_new(
            Length::from_micrometres(400_000),
            Length::from_micrometres(205_000),
        )
        .unwrap(),
        sections,
    )
    .unwrap();
    assert_eq!(plan.pagination().pages().len(), 3);
    assert_eq!(plan.pagination().columns(), 1);
    assert_eq!(plan.pagination().rows(), 3);
    assert_eq!(
        usize::from(plan.pagination().columns()) * usize::from(plan.pagination().rows()),
        plan.pagination().pages().len()
    );
    assert_eq!(
        plan.pagination()
            .pages()
            .iter()
            .map(PreviewPage::coordinate)
            .collect::<Vec<_>>(),
        vec!["S1-A1", "S1-B1", "S2-A1"]
    );
    assert_eq!(
        plan.pagination()
            .pages()
            .iter()
            .map(PreviewPage::section_ordinal)
            .collect::<Vec<_>>(),
        vec![0, 0, 1]
    );
}

#[test]
fn native_automatic_orientation_resolves_from_the_sealed_driver_geometry() {
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
            RenderTarget::SystemPrinter {
                printer_id: "landscape-driver".to_owned(),
                job: PrinterJobSettings::try_new(
                    digest(0x31),
                    "letter",
                    PrinterRasterGeometry::try_new(792, 612, 0, 0, 792, 612).unwrap(),
                    PrinterMediaSource::AutomaticCompatibleTray,
                    72,
                    DuplexMode::Off,
                    1,
                    false,
                )
                .unwrap(),
            },
            OutputFormat::NativePrinter,
            ColorMapping::PrintSafeEngineeringPalette,
            BackgroundMode::White,
            FontPolicy::new(true, false),
            true,
        )
        .unwrap(),
        DecorationSetup::try_new(false, false, false, Watermark::None).unwrap(),
        PrintMappingTable::default(),
    )
    .unwrap();
    let sections = vec![
        HardcopyContentSection::try_new(
            0,
            digest(0x32),
            Length::ZERO,
            Length::ZERO,
            ContentExtent::try_new(
                Length::from_micrometres(100_000),
                Length::from_micrometres(200_000),
            )
            .unwrap(),
            false,
        )
        .unwrap(),
        HardcopyContentSection::try_new(
            1,
            digest(0x33),
            Length::ZERO,
            Length::from_micrometres(205_000),
            ContentExtent::try_new(
                Length::from_micrometres(200_000),
                Length::from_micrometres(100_000),
            )
            .unwrap(),
            true,
        )
        .unwrap(),
    ];
    let plan = HardcopyPlan::compile_with_sections(
        source(),
        setup,
        ContentExtent::try_new(
            Length::from_micrometres(200_000),
            Length::from_micrometres(305_000),
        )
        .unwrap(),
        sections,
    )
    .unwrap();
    assert!(
        plan.pagination()
            .pages()
            .iter()
            .all(|page| { page.geometry().orientation() == ResolvedOrientation::Landscape })
    );
}

#[test]
fn render_target_background_and_font_combinations_are_validated() {
    assert!(matches!(
        RenderSetup::try_new(
            RenderTarget::SystemPrinter {
                printer_id: "office-a".to_owned(),
                job: printer_job(),
            },
            OutputFormat::PdfVector,
            ColorMapping::Monochrome,
            BackgroundMode::White,
            FontPolicy::new(true, true),
            true,
        ),
        Err(HardcopyError::IncompatibleRenderTarget)
    ));
    assert!(matches!(
        RenderSetup::try_new(
            RenderTarget::ExportArtifact,
            OutputFormat::Png { dpi: 600 },
            ColorMapping::ScreenColors,
            BackgroundMode::Transparent,
            FontPolicy::new(true, false),
            false,
        ),
        Err(HardcopyError::TransparentBackgroundRequiresVectorExport)
    ));
    assert!(matches!(
        RenderSetup::try_new(
            RenderTarget::ExportArtifact,
            OutputFormat::Png { dpi: 600 },
            ColorMapping::ScreenColors,
            BackgroundMode::White,
            FontPolicy::new(true, true),
            false,
        ),
        Err(HardcopyError::SearchableTextRequiresVectorOutput)
    ));
}

#[test]
fn printer_capability_selection_is_validated_and_plan_authenticated() {
    assert!(matches!(
        PrinterJobSettings::try_new(
            digest(0x35),
            "paper-1",
            printer_geometry(),
            PrinterMediaSource::ManualFeed,
            1_200,
            DuplexMode::LongEdge,
            0,
            false,
        ),
        Err(HardcopyError::InvalidCopyCount(0))
    ));
    assert!(matches!(
        PrinterJobSettings::try_new(
            digest(0x35),
            "paper-1",
            printer_geometry(),
            PrinterMediaSource::Roll {
                width: Length::from_micrometres(25_000),
            },
            1_200,
            DuplexMode::Off,
            1,
            false,
        ),
        Err(HardcopyError::InvalidPrinterRollWidth)
    ));

    let plan_id = HardcopyPlanId::try_from_uuid(Uuid::from_u128(61)).unwrap();
    let extent = ContentExtent::try_new(
        Length::from_micrometres(100_000),
        Length::from_micrometres(100_000),
    )
    .unwrap();
    let native_setup = |resolution_dpi| {
        let mut setup = HardcopySetup::default();
        setup.render = RenderSetup::try_new(
            RenderTarget::SystemPrinter {
                printer_id: "engineering-printer-04".to_owned(),
                job: PrinterJobSettings::try_new(
                    digest(0x35),
                    "paper-1",
                    printer_geometry(),
                    PrinterMediaSource::NamedTray("Letter / A4 tray".to_owned()),
                    resolution_dpi,
                    DuplexMode::ShortEdge,
                    2,
                    true,
                )
                .unwrap(),
            },
            OutputFormat::NativePrinter,
            ColorMapping::PrintSafeEngineeringPalette,
            BackgroundMode::White,
            FontPolicy::new(true, false),
            true,
        )
        .unwrap();
        setup
    };
    let high_resolution =
        HardcopyPlan::compile_with_id(plan_id, source(), native_setup(1_200), extent).unwrap();
    let standard_resolution =
        HardcopyPlan::compile_with_id(plan_id, source(), native_setup(600), extent).unwrap();
    assert_ne!(
        high_resolution.content_digest(),
        standard_resolution.content_digest()
    );
}

#[test]
fn per_object_print_mapping_covers_mockup_style_and_save_scope() {
    let trace = PrintMappingEntry::try_new(
        PrintObjectIdentity::try_new(
            PrintObjectKind::Trace,
            "trace:v(afe_out)",
            "V(afe_out)",
            "cyan solid · 2 px",
        )
        .unwrap(),
        PrintColor::Black,
        PrintRedundancy::SolidLine {
            width: Length::from_micrometres(350),
        },
        true,
    )
    .unwrap();
    let layer = PrintMappingEntry::try_new(
        PrintObjectIdentity::try_new(
            PrintObjectKind::Layer,
            "layer:m4:drawing",
            "M4 drawing",
            "yellow fill",
        )
        .unwrap(),
        PrintColor::GrayPercent(40),
        PrintRedundancy::CrossHatch {
            line_width: Length::from_micrometres(200),
            spacing: Length::from_micrometres(1_500),
        },
        true,
    )
    .unwrap();
    let table = PrintMappingTable::try_new(
        PrintMappingSaveScope::ProjectPrintSet("Release review".to_owned()),
        vec![trace, layer],
    )
    .unwrap();
    assert_eq!(table.entries().len(), 2);
    assert!(matches!(
        table.save_scope(),
        PrintMappingSaveScope::ProjectPrintSet(name) if name == "Release review"
    ));
    assert!(
        table
            .entries()
            .iter()
            .all(PrintMappingEntry::include_in_legend)
    );
}

#[test]
fn print_mapping_rejects_duplicate_semantic_objects_and_invalid_physical_styles() {
    let object = PrintObjectIdentity::try_new(
        PrintObjectKind::Trace,
        "trace:v(sensor_p)",
        "V(sensor_p)",
        "green solid · 2 px",
    )
    .unwrap();
    let entry = PrintMappingEntry::try_new(
        object,
        PrintColor::GrayPercent(70),
        PrintRedundancy::DashedLine {
            width: Length::from_micrometres(300),
            dash: Length::from_micrometres(2_000),
            gap: Length::from_micrometres(1_000),
        },
        true,
    )
    .unwrap();
    assert!(matches!(
        PrintMappingTable::try_new(
            PrintMappingSaveScope::PortablePersonalPreset("Lab printer".to_owned()),
            vec![entry.clone(), entry],
        ),
        Err(HardcopyError::DuplicatePrintObjectIdentity { .. })
    ));
    assert!(matches!(
        PrintMappingEntry::try_new(
            PrintObjectIdentity::try_new(
                PrintObjectKind::DrcMarker,
                "drc:blocking",
                "DRC blocking",
                "red marker",
            )
            .unwrap(),
            PrintColor::Black,
            PrintRedundancy::TriangleWithId { size: Length::ZERO },
            true,
        ),
        Err(HardcopyError::InvalidPrintFeatureSize { .. })
    ));
}

#[test]
fn fixed_identity_produces_deterministic_plan_digest() {
    let plan_id = HardcopyPlanId::try_from_uuid(Uuid::from_u128(50)).unwrap();
    let extent = ContentExtent::try_new(
        Length::from_micrometres(400_000),
        Length::from_micrometres(100_000),
    )
    .unwrap();
    let first = HardcopyPlan::compile_with_id(
        plan_id,
        source(),
        one_to_one_setup(TilingMode::Automatic, Length::ZERO),
        extent,
    )
    .unwrap();
    let second = HardcopyPlan::compile_with_id(
        plan_id,
        source(),
        one_to_one_setup(TilingMode::Automatic, Length::ZERO),
        extent,
    )
    .unwrap();
    assert_eq!(first.content_digest(), second.content_digest());
    assert_eq!(first.pagination(), second.pagination());
}

#[test]
fn source_content_identity_is_bound_into_plan_digest() {
    let plan_id = HardcopyPlanId::try_from_uuid(Uuid::from_u128(51)).unwrap();
    let extent = ContentExtent::try_new(
        Length::from_micrometres(100_000),
        Length::from_micrometres(100_000),
    )
    .unwrap();
    let first =
        HardcopyPlan::compile_with_id(plan_id, source(), HardcopySetup::default(), extent).unwrap();
    let mut changed = source();
    changed.content_digest = digest(0x99);
    let second =
        HardcopyPlan::compile_with_id(plan_id, changed, HardcopySetup::default(), extent).unwrap();
    assert_ne!(first.content_digest(), second.content_digest());
}

#[test]
fn setup_store_is_per_document_and_revisioned_only_on_change() {
    let mut store = HardcopySetupStore::default();
    let source = source();
    let inserted = store.save(&source, HardcopySetup::default()).unwrap();
    assert_eq!(inserted.disposition(), SetupSaveDisposition::Inserted);
    assert_eq!(inserted.saved().revision(), ObjectRevision::INITIAL);

    let unchanged = store.save(&source, HardcopySetup::default()).unwrap();
    assert_eq!(unchanged.disposition(), SetupSaveDisposition::Unchanged);
    assert_eq!(unchanged.saved().revision(), ObjectRevision::INITIAL);

    let changed = one_to_one_setup(TilingMode::Automatic, Length::ZERO);
    let updated = store.save(&source, changed).unwrap();
    assert_eq!(updated.disposition(), SetupSaveDisposition::Updated);
    assert_eq!(updated.saved().revision().get(), 2);
    assert_eq!(store.len(), 1);
    assert_eq!(store.setup_for(&source).unwrap(), Some(updated.saved()));
}

#[test]
fn setup_store_round_trips_and_rejects_digest_tampering() {
    let mut store = HardcopySetupStore::default();
    store.save(&source(), HardcopySetup::default()).unwrap();
    let encoded = serde_json::to_value(&store).unwrap();
    let restored: HardcopySetupStore = serde_json::from_value(encoded.clone()).unwrap();
    assert_eq!(store, restored);

    let mut tampered = encoded;
    let key = source().document_id().to_string();
    tampered["documents"][&key]["content_digest"] =
        serde_json::Value::String(digest(0xee).to_string());
    assert!(serde_json::from_value::<HardcopySetupStore>(tampered).is_err());
}

#[test]
fn setup_store_rejects_document_kind_reuse() {
    let mut store = HardcopySetupStore::default();
    let source = source();
    store.save(&source, HardcopySetup::default()).unwrap();
    let changed_kind = source_with(
        source.document_id(),
        HardcopyDocumentKind::PlotOrWorksheet,
        HardcopyScope::ActivePlotDocument,
    );
    assert!(matches!(
        store.save(&changed_kind, HardcopySetup::default()),
        Err(HardcopyError::PersistedDocumentKindChanged { .. })
    ));
}

#[test]
fn success_receipt_binds_exact_plan_artifact_and_digest() {
    let plan = HardcopyPlan::compile(
        source(),
        HardcopySetup::default(),
        ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(100_000),
        )
        .unwrap(),
    )
    .unwrap();
    let artifact = HardcopyArtifactIdentity::try_new(
        digest(0x77),
        12_345,
        plan.pagination().pages().len() as u32,
        OutputFormat::PdfVector,
    )
    .unwrap();
    let receipt_id = HardcopyReceiptId::try_from_uuid(Uuid::from_u128(70)).unwrap();
    let first = HardcopyReceipt::record_with_id(
        receipt_id,
        &plan,
        HardcopyOutcome::ArtifactExported {
            artifact: artifact.clone(),
        },
    )
    .unwrap();
    let second = HardcopyReceipt::record_with_id(
        receipt_id,
        &plan,
        HardcopyOutcome::ArtifactExported { artifact },
    )
    .unwrap();
    assert_eq!(first.content_digest(), second.content_digest());
    assert_eq!(first.plan_id(), plan.id());
    assert_eq!(first.plan_content_digest(), plan.content_digest());
    assert_eq!(
        first.source_content_digest(),
        plan.source().content_digest()
    );
}

#[test]
fn success_receipt_rejects_wrong_page_count_or_format() {
    let plan = HardcopyPlan::compile(
        source(),
        HardcopySetup::default(),
        ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(100_000),
        )
        .unwrap(),
    )
    .unwrap();
    let wrong_pages =
        HardcopyArtifactIdentity::try_new(digest(1), 100, 99, OutputFormat::PdfVector).unwrap();
    assert!(matches!(
        HardcopyReceipt::record(
            &plan,
            HardcopyOutcome::ArtifactExported {
                artifact: wrong_pages
            }
        ),
        Err(HardcopyError::ArtifactPageCountMismatch { .. })
    ));
    let wrong_format = HardcopyArtifactIdentity::try_new(
        digest(1),
        100,
        plan.pagination().pages().len() as u32,
        OutputFormat::SvgVector,
    )
    .unwrap();
    assert!(matches!(
        HardcopyReceipt::record(
            &plan,
            HardcopyOutcome::ArtifactExported {
                artifact: wrong_format
            }
        ),
        Err(HardcopyError::ArtifactFormatMismatch)
    ));
}

#[test]
fn native_spool_acceptance_is_truthful_and_device_bound() {
    let mut setup = HardcopySetup::default();
    setup.render = RenderSetup::try_new(
        RenderTarget::SystemPrinter {
            printer_id: "engineering-printer-04".to_owned(),
            job: printer_job(),
        },
        OutputFormat::NativePrinter,
        ColorMapping::PrintSafeEngineeringPalette,
        BackgroundMode::White,
        FontPolicy::new(true, false),
        true,
    )
    .unwrap();
    let plan = HardcopyPlan::compile(
        source(),
        setup,
        ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(100_000),
        )
        .unwrap(),
    )
    .unwrap();
    let pages = plan.pagination().pages().len() as u32;
    let receipt = HardcopyReceipt::record(
        &plan,
        HardcopyOutcome::SpoolAccepted {
            device_id: "engineering-printer-04".to_owned(),
            job_id: "spool-job-813".to_owned(),
            pages_accepted: pages,
            source_artifact_digest: digest(0x81),
        },
    )
    .unwrap();
    assert!(matches!(
        receipt.outcome(),
        HardcopyOutcome::SpoolAccepted { job_id, .. } if job_id == "spool-job-813"
    ));
    assert!(matches!(
        HardcopyReceipt::record(
            &plan,
            HardcopyOutcome::SpoolAccepted {
                device_id: "different-printer".to_owned(),
                job_id: "spool-job-814".to_owned(),
                pages_accepted: pages,
                source_artifact_digest: digest(0x82),
            },
        ),
        Err(HardcopyError::SpoolDeviceMismatch)
    ));
}

#[test]
fn browser_print_handoff_is_distinct_from_artifact_export() {
    let mut setup = HardcopySetup::default();
    setup.render = RenderSetup::try_new(
        RenderTarget::BrowserPrintDialog,
        OutputFormat::BrowserPrintDocument,
        ColorMapping::PrintSafeEngineeringPalette,
        BackgroundMode::White,
        FontPolicy::new(true, true),
        true,
    )
    .unwrap();
    let plan = HardcopyPlan::compile(
        source(),
        setup,
        ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(100_000),
        )
        .unwrap(),
    )
    .unwrap();
    let pages = plan.pagination().pages().len() as u32;
    assert!(
        HardcopyReceipt::record(
            &plan,
            HardcopyOutcome::BrowserPrintNavigationAccepted {
                navigation_id: "browser-print-22".to_owned(),
                pages_accepted: pages,
                source_artifact_digest: digest(0x90),
            },
        )
        .is_ok()
    );
    let artifact = HardcopyArtifactIdentity::try_new(
        digest(0x91),
        1_000,
        pages,
        OutputFormat::BrowserPrintDocument,
    )
    .unwrap();
    assert!(matches!(
        HardcopyReceipt::record(&plan, HardcopyOutcome::ArtifactExported { artifact },),
        Err(HardcopyError::OutcomeRenderTargetMismatch)
    ));
    assert!(matches!(
        HardcopyReceipt::record(
            &plan,
            HardcopyOutcome::BrowserPrintNavigationAccepted {
                navigation_id: "browser-print-23".to_owned(),
                pages_accepted: pages + 1,
                source_artifact_digest: digest(0x92),
            },
        ),
        Err(HardcopyError::AcceptedPageCountMismatch { .. })
    ));
    assert!(
        HardcopyReceipt::record(
            &plan,
            HardcopyOutcome::DesktopPrintHandoffAccepted {
                handoff_id: "desktop-print-24".to_owned(),
                pages_accepted: pages,
                source_artifact_digest: digest(0x93),
            },
        )
        .is_ok()
    );
}

#[test]
fn cancellation_and_failure_are_explicit_validated_outcomes() {
    let plan = HardcopyPlan::compile(
        source(),
        HardcopySetup::default(),
        ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(100_000),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(
        HardcopyReceipt::record(
            &plan,
            HardcopyOutcome::Cancelled {
                phase: CancellationPhase::Rendering,
                pages_completed: 0,
                reason: Some("Cancelled by user".to_owned()),
            },
        )
        .is_ok()
    );
    assert!(
        HardcopyReceipt::record(
            &plan,
            HardcopyOutcome::Failed {
                code: HardcopyFailureCode::DeviceUnavailable,
                message: "Selected printer is offline".to_owned(),
                pages_completed: 0,
                retryable: true,
            },
        )
        .is_ok()
    );
    assert!(matches!(
        HardcopyReceipt::record(
            &plan,
            HardcopyOutcome::Failed {
                code: HardcopyFailureCode::RenderFailure,
                message: "".to_owned(),
                pages_completed: 0,
                retryable: false,
            },
        ),
        Err(HardcopyError::InvalidText { .. })
    ));
}

#[test]
fn receipt_ledger_round_trips_and_rejects_tampered_entries() {
    let plan = HardcopyPlan::compile(
        source(),
        HardcopySetup::default(),
        ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(100_000),
        )
        .unwrap(),
    )
    .unwrap();
    let receipt = HardcopyReceipt::record(
        &plan,
        HardcopyOutcome::Cancelled {
            phase: CancellationPhase::Preparing,
            pages_completed: 0,
            reason: Some("operator cancelled preview".to_owned()),
        },
    )
    .unwrap();
    let receipt_id = receipt.id();
    let mut ledger = HardcopyReceiptLedger::default();
    ledger.append(receipt).unwrap();
    let bytes = serde_json::to_vec(&ledger).unwrap();
    let restored: HardcopyReceiptLedger = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(restored.latest().map(HardcopyReceipt::id), Some(receipt_id));

    let mut tampered: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    tampered["receipts"][0]["content_digest"] =
        serde_json::to_value(ContentDigest::from_bytes([0xFE; 32])).unwrap();
    assert!(
        serde_json::from_value::<HardcopyReceiptLedger>(tampered).is_err(),
        "persisted outcome history must revalidate every sealed receipt"
    );
}

#[test]
fn authored_media_resolves_exact_mixed_sheet_size_and_orientation_per_section() {
    let mut setup = one_to_one_setup(TilingMode::Automatic, Length::ZERO);
    setup.physical_page.paper = PaperSize::MatchAuthoredSheets(vec![
        AuthoredSheetMedia::try_new(
            0,
            "ISO A4 portrait",
            Length::from_micrometres(210_000),
            Length::from_micrometres(297_000),
        )
        .unwrap(),
        AuthoredSheetMedia::try_new(
            1,
            "ISO A3 landscape",
            Length::from_micrometres(420_000),
            Length::from_micrometres(297_000),
        )
        .unwrap(),
    ]);
    setup.physical_page.margins = PageMargins::uniform(Length::ZERO);
    setup.physical_page.orientation = Orientation::Portrait;
    let portrait_extent = ContentExtent::try_new(
        Length::from_micrometres(210_000),
        Length::from_micrometres(297_000),
    )
    .unwrap();
    let landscape_extent = ContentExtent::try_new(
        Length::from_micrometres(420_000),
        Length::from_micrometres(297_000),
    )
    .unwrap();
    let sections = vec![
        HardcopyContentSection::try_new(
            0,
            digest(0x12),
            Length::ZERO,
            Length::ZERO,
            portrait_extent,
            false,
        )
        .unwrap(),
        HardcopyContentSection::try_new(
            1,
            digest(0x13),
            Length::ZERO,
            Length::from_micrometres(302_000),
            landscape_extent,
            true,
        )
        .unwrap(),
    ];
    let plan = HardcopyPlan::compile_with_sections(
        source(),
        setup,
        ContentExtent::try_new(
            Length::from_micrometres(420_000),
            Length::from_micrometres(599_000),
        )
        .unwrap(),
        sections,
    )
    .unwrap();
    let pages = plan.pagination().pages();
    assert_eq!(pages.len(), 2);
    assert_eq!(
        pages[0].geometry().physical_size(),
        (
            Length::from_micrometres(210_000),
            Length::from_micrometres(297_000)
        )
    );
    assert_eq!(
        pages[1].geometry().physical_size(),
        (
            Length::from_micrometres(420_000),
            Length::from_micrometres(297_000)
        )
    );
    assert_eq!(
        pages[0].geometry().orientation(),
        ResolvedOrientation::Portrait
    );
    assert_eq!(
        pages[1].geometry().orientation(),
        ResolvedOrientation::Landscape
    );
}

#[test]
fn schematic_output_inclusion_contract_round_trips_and_legacy_setups_default() {
    let base = HardcopySetup::default();
    let schematic = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::CompleteSchematicContent,
        OutsideSheetContentPolicy::ExtendOutput,
        false,
        true,
        false,
        true,
        false,
        true,
    );
    let setup = HardcopySetup::try_new_with_schematic(
        base.physical_page().clone(),
        base.scale(),
        base.tiling(),
        base.render().clone(),
        base.decorations().clone(),
        schematic,
        base.print_mapping().clone(),
    )
    .unwrap();

    let mut encoded = serde_json::to_value(&setup).unwrap();
    assert_eq!(
        encoded["schematic"]["outside_content"],
        serde_json::json!("extend-output")
    );
    assert_eq!(
        serde_json::from_value::<HardcopySetup>(encoded.clone()).unwrap(),
        setup
    );

    encoded.as_object_mut().unwrap().remove("schematic");
    let restored_legacy = serde_json::from_value::<HardcopySetup>(encoded).unwrap();
    assert_eq!(
        restored_legacy.schematic(),
        SchematicHardcopySetup::default()
    );
}
