//! Tests of the print adapters: capability digests, exact printer modes and
//! paper identity, the spool transaction and its failure receipts, the
//! desktop print handoff, and the PDF the CUPS backend hands `lp`.

#[cfg(not(target_arch = "wasm32"))]
use std::cell::RefCell;
#[cfg(not(target_arch = "wasm32"))]
use std::rc::Rc;

use uuid::Uuid;

use super::*;
use crate::product::ObjectRevision;
#[cfg(not(target_arch = "wasm32"))]
use rspice_hardcopy::render::{HardcopyRenderer, HardcopySceneMetadata};
use rspice_hardcopy_contract::{
    ActiveHardcopySource, BackgroundMode, Bleed, ColorMapping, ContentExtent, DecorationSetup,
    FontPolicy, HardcopyDocumentId, HardcopyDocumentKind, HardcopyPlanId, HardcopyScope,
    HardcopySetup, Length, Orientation, PageMargins, PaperSize, PhysicalPageSetup,
    PrintMappingTable, PrinterRasterGeometry, RenderSetup, ScaleMode, StandardPaper, TilingMode,
    TilingSetup, Watermark,
};

fn digest(value: u8) -> ContentDigest {
    ContentDigest::from_bytes([value; 32])
}

fn capability_descriptor() -> PrinterCapabilityDescriptor {
    PrinterCapabilityDescriptor {
        device_id: "engineering-printer-04".to_owned(),
        display_name: "Engineering Printer 04".to_owned(),
        driver_name: "RSpice deterministic test driver".to_owned(),
        port_name: "TESTPORT:".to_owned(),
        driver_spec_version: 1,
        driver_version: 42,
        papers: vec![
            PrinterPaperCapability::try_new(5, "Legal", 215_900, 355_600).unwrap(),
            PrinterPaperCapability::try_new(1, "Letter", 215_900, 279_400).unwrap(),
            PrinterPaperCapability::try_new(
                8,
                "Letter-compatible engineering stock",
                215_900,
                279_400,
            )
            .unwrap(),
        ],
        trays: vec![
            PrinterTrayCapability::try_new(4, "Manual feed").unwrap(),
            PrinterTrayCapability::try_new(1, "Upper tray").unwrap(),
        ],
        resolutions: vec![
            PrinterResolutionCapability::try_new(600, 600).unwrap(),
            PrinterResolutionCapability::try_new(300, 300).unwrap(),
            PrinterResolutionCapability::try_new(72, 72).unwrap(),
        ],
        duplex_modes: vec![DuplexMode::ShortEdge, DuplexMode::Off, DuplexMode::LongEdge],
        maximum_copies: 99,
        supports_collation: true,
        supports_color: true,
    }
}

fn source() -> ActiveHardcopySource {
    ActiveHardcopySource::try_new(
        HardcopyDocumentId::try_from_uuid(Uuid::from_u128(1)).unwrap(),
        ObjectRevision::INITIAL,
        digest(0x11),
        "top · schematic",
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::CurrentSheet,
    )
    .unwrap()
}

fn raster_geometry(dpi: u16) -> PrinterRasterGeometry {
    let pixels = |micrometres: u64| {
        u32::try_from((u128::from(micrometres) * u128::from(dpi)).div_ceil(25_400)).unwrap()
    };
    let width = pixels(279_400);
    let height = pixels(215_900);
    PrinterRasterGeometry::try_new(width, height, 0, 0, width, height).unwrap()
}

fn native_plan(
    capabilities: &PrinterCapabilitySnapshot,
    media: PrinterMediaSource,
    dpi: u16,
    duplex: DuplexMode,
    copies: u16,
    collate: bool,
) -> HardcopyPlan {
    native_plan_with_layout(
        capabilities,
        media,
        dpi,
        duplex,
        copies,
        collate,
        ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(60_000),
        )
        .unwrap(),
        ScaleMode::FitPrintableArea,
        TilingMode::SinglePage,
    )
}

fn native_plan_with_layout(
    capabilities: &PrinterCapabilitySnapshot,
    media: PrinterMediaSource,
    dpi: u16,
    duplex: DuplexMode,
    copies: u16,
    collate: bool,
    content_extent: ContentExtent,
    scale: ScaleMode,
    tiling: TilingMode,
) -> HardcopyPlan {
    let setup = HardcopySetup::try_new(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(StandardPaper::Letter),
            PageMargins::uniform(Length::from_micrometres(10_000)),
            Bleed::None,
            Orientation::Landscape,
        )
        .unwrap(),
        scale,
        TilingSetup::try_new(tiling, Length::ZERO, false).unwrap(),
        RenderSetup::try_new(
            RenderTarget::SystemPrinter {
                printer_id: capabilities.device_id().to_owned(),
                job: PrinterJobSettings::try_new(
                    capabilities.content_digest(),
                    "1",
                    raster_geometry(dpi),
                    media,
                    dpi,
                    duplex,
                    copies,
                    collate,
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
    HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(Uuid::from_u128(2)).unwrap(),
        source(),
        setup,
        content_extent,
    )
    .unwrap()
}
#[cfg(not(target_arch = "wasm32"))]
fn resolved_source(plan: &HardcopyPlan) -> super::super::sources::ResolvedHardcopyDocument {
    use super::super::sources::{
        SemanticBounds, SemanticPoint, resolve_blank_schematic_sheet_with_format,
    };
    use rspice_hardcopy_contract::sources::HardcopySourceIdentity;
    let authority = plan.source();
    let identity = HardcopySourceIdentity::try_new(
        "printer-test",
        authority.document_id(),
        authority.revision(),
        authority.display_name(),
    )
    .unwrap();
    let blank = resolve_blank_schematic_sheet_with_format(
        identity.clone(),
        authority.scope().clone(),
        None,
    )
    .unwrap();
    rspice_hardcopy::sources::resolve_semantic_source(
        identity,
        authority.content_digest(),
        authority.document_kind(),
        authority.scope().clone(),
        blank.semantic_document().clone(),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(
                i64::try_from(plan.content_extent().width().micrometres()).unwrap(),
                i64::try_from(plan.content_extent().height().micrometres()).unwrap(),
            ),
        )
        .unwrap(),
    )
    .unwrap()
}
#[cfg(not(target_arch = "wasm32"))]
fn rendered_pages(plan: &HardcopyPlan, dpi: u16) -> RenderedPrinterPages {
    let metadata = HardcopySceneMetadata::try_new("test", "RSpice").unwrap();
    HardcopyRenderer::render_printer_pages_resolved(plan, &resolved_source(plan), metadata, dpi)
        .unwrap()
}
#[cfg(not(target_arch = "wasm32"))]
fn browser_print_plan_and_publication() -> (HardcopyPlan, RenderedHardcopyPublication) {
    let extent = ContentExtent::try_new(
        Length::from_micrometres(100_000),
        Length::from_micrometres(60_000),
    )
    .unwrap();
    let setup = HardcopySetup::try_new(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(StandardPaper::Letter),
            PageMargins::uniform(Length::from_micrometres(10_000)),
            Bleed::None,
            Orientation::Landscape,
        )
        .unwrap(),
        ScaleMode::FitPrintableArea,
        TilingSetup::try_new(TilingMode::SinglePage, Length::ZERO, false).unwrap(),
        RenderSetup::try_new(
            RenderTarget::BrowserPrintDialog,
            OutputFormat::BrowserPrintDocument,
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
    let plan = HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(Uuid::from_u128(3)).unwrap(),
        source(),
        setup,
        extent,
    )
    .unwrap();
    let metadata = HardcopySceneMetadata::try_new("test", "RSpice").unwrap();
    let publication =
        HardcopyRenderer::render_resolved(&plan, &resolved_source(&plan), metadata).unwrap();
    (plan, publication)
}

#[cfg(not(target_arch = "wasm32"))]
struct FakeDesktopLauncher {
    observed: Rc<RefCell<Vec<std::path::PathBuf>>>,
    fail: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl DesktopPrintLauncher for FakeDesktopLauncher {
    fn launch(&self, document: &std::path::Path) -> Result<(), HardcopyPrintError> {
        self.observed.borrow_mut().push(document.to_owned());
        if self.fail {
            Err(HardcopyPrintError::DesktopHandoff {
                operation: "testing the launcher",
                message: "rejected".to_owned(),
            })
        } else {
            Ok(())
        }
    }
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn desktop_print_handoff_is_authenticated_private_and_not_spool_acceptance() {
    let (plan, publication) = browser_print_plan_and_publication();
    let root = std::env::temp_dir().join(format!(
        "rspice-print-handoff-test-{}",
        Uuid::new_v4().simple()
    ));
    let observed = Rc::new(RefCell::new(Vec::new()));
    let outcome = handoff_desktop_print_document_with(
        &plan,
        &publication,
        &FakeDesktopLauncher {
            observed: observed.clone(),
            fail: false,
        },
        &root,
    )
    .unwrap();
    assert!(matches!(
        outcome,
        HardcopyOutcome::DesktopPrintHandoffAccepted {
            pages_accepted: 1,
            ..
        }
    ));
    let document = observed.borrow()[0].clone();
    let html = std::fs::read_to_string(&document).unwrap();
    assert!(html.contains(&plan.content_digest().to_string()));
    assert!(!html.to_ascii_lowercase().contains("<script"));

    let directory = document.parent().unwrap().to_owned();
    cleanup_handoff_directory(&directory);
    std::fs::remove_dir(&root).unwrap();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn rejected_desktop_handoff_removes_the_unclaimed_document() {
    let (plan, publication) = browser_print_plan_and_publication();
    let root = std::env::temp_dir().join(format!(
        "rspice-print-handoff-test-{}",
        Uuid::new_v4().simple()
    ));
    let observed = Rc::new(RefCell::new(Vec::new()));
    assert!(
        handoff_desktop_print_document_with(
            &plan,
            &publication,
            &FakeDesktopLauncher {
                observed: observed.clone(),
                fail: true,
            },
            &root,
        )
        .is_err()
    );
    assert!(!observed.borrow()[0].exists());
    assert_eq!(std::fs::read_dir(&root).unwrap().count(), 0);
    std::fs::remove_dir(&root).unwrap();
}

fn native_plan_for_exact_mode(
    capabilities: &PrinterCapabilitySnapshot,
    selected_paper_id: &str,
    geometry: PrinterRasterGeometry,
    dpi: u16,
) -> HardcopyPlan {
    let setup = HardcopySetup::try_new(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(StandardPaper::Letter),
            PageMargins::uniform(Length::from_micrometres(10_000)),
            Bleed::None,
            Orientation::Landscape,
        )
        .unwrap(),
        ScaleMode::FitPrintableArea,
        TilingSetup::try_new(TilingMode::SinglePage, Length::ZERO, false).unwrap(),
        RenderSetup::try_new(
            RenderTarget::SystemPrinter {
                printer_id: capabilities.device_id().to_owned(),
                job: PrinterJobSettings::try_new(
                    capabilities.content_digest(),
                    selected_paper_id,
                    geometry,
                    PrinterMediaSource::AutomaticCompatibleTray,
                    dpi,
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
    HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(Uuid::from_u128(3)).unwrap(),
        source(),
        setup,
        ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(60_000),
        )
        .unwrap(),
    )
    .unwrap()
}

#[test]
fn capability_digest_is_order_independent_and_changes_with_real_capabilities() {
    let first = PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
    let mut reordered = capability_descriptor();
    reordered.papers.reverse();
    reordered.trays.reverse();
    reordered.resolutions.reverse();
    reordered.duplex_modes.reverse();
    let second = PrinterCapabilitySnapshot::try_from_descriptor(reordered).unwrap();
    assert_eq!(first, second);

    let mut changed = capability_descriptor();
    changed.maximum_copies = 12;
    let changed = PrinterCapabilitySnapshot::try_from_descriptor(changed).unwrap();
    assert_ne!(first.content_digest(), changed.content_digest());
}

#[test]
fn capability_deserialization_revalidates_the_digest() {
    let snapshot = PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
    let mut encoded = serde_json::to_value(&snapshot).unwrap();
    encoded["maximum_copies"] = serde_json::json!(7);
    assert!(serde_json::from_value::<PrinterCapabilitySnapshot>(encoded).is_err());
}

#[test]
fn failure_outcomes_always_fit_the_receipt_contract() {
    let error = HardcopyPrintError::InvalidCapabilitySnapshot("é".repeat(2_000));
    let HardcopyOutcome::Failed { message, .. } = error.failure_outcome(0) else {
        panic!("failure adapter must produce a failed outcome");
    };
    assert!(message.len() <= MAX_RECEIPT_MESSAGE_BYTES);
    assert!(message.ends_with("..."));
}

#[test]
fn exact_plan_settings_resolve_without_substitution() {
    let capabilities =
        PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
    let plan = native_plan(
        &capabilities,
        PrinterMediaSource::NamedTray("Upper tray".to_owned()),
        600,
        DuplexMode::LongEdge,
        3,
        true,
    );
    let resolved = resolve_native_printer_job(&plan, &capabilities).unwrap();
    assert_eq!(resolved.paper_platform_id(), 1);
    assert_eq!(resolved.source_platform_id(), Some(1));
    assert_eq!(resolved.resolution_dpi(), 600);
    assert_eq!(resolved.orientation(), ResolvedOrientation::Landscape);
    assert_eq!(resolved.duplex(), DuplexMode::LongEdge);
    assert_eq!(resolved.copies(), 3);
    assert!(resolved.collate());
}

#[test]
fn paper_resolution_uses_the_selected_identity_not_same_size_fallback() {
    let capabilities =
        PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
    let exact = native_plan_for_exact_mode(&capabilities, "8", raster_geometry(600), 600);
    assert_eq!(
        resolve_native_printer_job(&exact, &capabilities)
            .unwrap()
            .paper_platform_id(),
        8
    );

    let unavailable = native_plan_for_exact_mode(&capabilities, "999", raster_geometry(600), 600);
    assert_eq!(
        resolve_native_printer_job(&unavailable, &capabilities),
        Err(HardcopyPrintError::UnsupportedPaperIdentity(
            "999".to_owned()
        ))
    );
}

/// The driver prints nothing outside its printable rectangle, so the
/// spool refuses pages whose ink reaches past it and says by how much.
/// The planned margin does not decide it: the same pages print on a
/// rectangle that stops exactly at their ink.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn device_printable_rectangle_must_contain_all_planned_ink() {
    let capabilities =
        PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
    let unbanded = native_plan_for_exact_mode(&capabilities, "1", raster_geometry(72), 72);
    let setup = unbanded.setup();
    let banded = HardcopySetup::try_new_with_schematic(
        setup.physical_page().clone(),
        setup.scale(),
        setup.tiling(),
        setup.render().clone(),
        DecorationSetup::try_new(false, false, true, Watermark::None).unwrap(),
        setup.schematic(),
        setup.print_mapping().clone(),
    )
    .unwrap();
    let plan = HardcopyPlan::compile(source(), banded, unbanded.content_extent()).unwrap();
    let pages = rendered_pages(&plan, 72);
    let ink = pages.pages()[0].ink().expect("the provenance band is ink");
    let mut job = resolve_native_printer_job(&plan, &capabilities).unwrap();
    let (width, height) = job.raster_geometry.physical_size_px();
    let reaching = |left: u32| {
        PrinterRasterGeometry::try_new(width, height, left, 0, width - left, height).unwrap()
    };
    job.raster_geometry = reaching(ink.left);
    assert_eq!(validate_printer_publication(&plan, &pages, &job), Ok(()));

    job.raster_geometry = reaching(ink.left + 1);
    let refusal = validate_printer_publication(&plan, &pages, &job).unwrap_err();
    let HardcopyPrintError::InkInPrinterMargin(shortfall) = refusal else {
        panic!("ink in the unprintable margin was not refused for it: {refusal}");
    };
    let micrometres = |pixels: u32| (u64::from(pixels) * 25_400).div_ceil(72);
    assert_eq!(shortfall.margin_um(), micrometres(ink.left + 1));
    assert_eq!(shortfall.ink_um(), micrometres(ink.left));
    assert!(
        refusal
            .to_string()
            .starts_with("The printer can't print within"),
        "{refusal}"
    );
}

#[test]
fn stale_digest_and_unsupported_options_fail_closed() {
    let capabilities =
        PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
    let plan = native_plan(
        &capabilities,
        PrinterMediaSource::AutomaticCompatibleTray,
        600,
        DuplexMode::Off,
        1,
        false,
    );
    let mut changed = capability_descriptor();
    changed.maximum_copies = 1;
    let changed = PrinterCapabilitySnapshot::try_from_descriptor(changed).unwrap();
    assert_eq!(
        resolve_native_printer_job(&plan, &changed),
        Err(HardcopyPrintError::PrinterCapabilitiesChanged)
    );

    let unsupported = native_plan(
        &capabilities,
        PrinterMediaSource::NamedTray("Missing tray".to_owned()),
        600,
        DuplexMode::Off,
        1,
        false,
    );
    assert!(matches!(
        resolve_native_printer_job(&unsupported, &capabilities),
        Err(HardcopyPrintError::UnsupportedMediaSource(_))
    ));
}

#[derive(Default)]
#[cfg(not(target_arch = "wasm32"))]
struct FakeSpoolState {
    events: Vec<&'static str>,
    pages_written: u32,
    fail_on_page: Option<u32>,
    cancel_after_page: Option<u32>,
    abort_fails: bool,
}

#[cfg(not(target_arch = "wasm32"))]
struct FakeSpoolBackend {
    state: Rc<RefCell<FakeSpoolState>>,
    cancellation: HardcopyCancellationToken,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeSpoolBackend for FakeSpoolBackend {
    fn start_job(&mut self) -> Result<String, HardcopyPrintError> {
        self.state.borrow_mut().events.push("start-job");
        Ok("job-17".to_owned())
    }

    fn start_page(&mut self) -> Result<(), HardcopyPrintError> {
        self.state.borrow_mut().events.push("start-page");
        Ok(())
    }

    fn write_page(
        &mut self,
        _page: &rspice_hardcopy::render::PrinterRasterPage,
    ) -> Result<(), HardcopyPrintError> {
        let mut state = self.state.borrow_mut();
        state.pages_written += 1;
        state.events.push("write-page");
        if state.fail_on_page == Some(state.pages_written) {
            return Err(HardcopyPrintError::DriverRejected {
                operation: "fake raster write",
            });
        }
        Ok(())
    }

    fn end_page(&mut self) -> Result<(), HardcopyPrintError> {
        let mut state = self.state.borrow_mut();
        state.events.push("end-page");
        if state.cancel_after_page == Some(state.pages_written) {
            self.cancellation.cancel();
        }
        Ok(())
    }

    fn finish_job(&mut self) -> Result<(), HardcopyPrintError> {
        self.state.borrow_mut().events.push("finish");
        Ok(())
    }

    fn abort_job(&mut self) -> Result<(), HardcopyPrintError> {
        let mut state = self.state.borrow_mut();
        state.events.push("abort");
        if state.abort_fails {
            Err(HardcopyPrintError::DriverRejected {
                operation: "fake abort",
            })
        } else {
            Ok(())
        }
    }
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn spool_transaction_reports_acceptance_only_after_complete_commit() {
    let capabilities =
        PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
    let plan = native_plan(
        &capabilities,
        PrinterMediaSource::AutomaticCompatibleTray,
        72,
        DuplexMode::Off,
        1,
        false,
    );
    let pages = rendered_pages(&plan, 72);
    let state = Rc::new(RefCell::new(FakeSpoolState::default()));
    let cancellation = HardcopyCancellationToken::default();
    let mut backend = FakeSpoolBackend {
        state: state.clone(),
        cancellation: cancellation.clone(),
    };
    let outcome = spool_transaction(
        &mut backend,
        &pages,
        capabilities.device_id(),
        &cancellation,
    )
    .unwrap();
    assert!(matches!(
        outcome,
        HardcopyOutcome::SpoolAccepted {
            pages_accepted: 1,
            ..
        }
    ));
    assert_eq!(state.borrow().events.last(), Some(&"finish"));
    assert!(!state.borrow().events.contains(&"abort"));
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn spool_transaction_aborts_on_write_failure_and_cancellation() {
    let capabilities =
        PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
    let plan = native_plan(
        &capabilities,
        PrinterMediaSource::AutomaticCompatibleTray,
        72,
        DuplexMode::Off,
        1,
        false,
    );
    let pages = rendered_pages(&plan, 72);

    let failed_state = Rc::new(RefCell::new(FakeSpoolState {
        fail_on_page: Some(1),
        ..Default::default()
    }));
    let token = HardcopyCancellationToken::default();
    let mut failed = FakeSpoolBackend {
        state: failed_state.clone(),
        cancellation: token.clone(),
    };
    let failure = spool_transaction(&mut failed, &pages, "printer", &token).unwrap_err();
    assert_eq!(failure.pages_completed(), 0);
    assert_eq!(failed_state.borrow().events.last(), Some(&"abort"));

    let tiled_plan = native_plan_with_layout(
        &capabilities,
        PrinterMediaSource::AutomaticCompatibleTray,
        72,
        DuplexMode::Off,
        1,
        false,
        ContentExtent::try_new(
            Length::from_micrometres(600_000),
            Length::from_micrometres(100_000),
        )
        .unwrap(),
        ScaleMode::EngineeringOneToOne,
        TilingMode::Automatic,
    );
    let tiled_pages = rendered_pages(&tiled_plan, 72);
    assert!(tiled_pages.pages().len() > 1);
    let partial_state = Rc::new(RefCell::new(FakeSpoolState {
        fail_on_page: Some(2),
        ..Default::default()
    }));
    let token = HardcopyCancellationToken::default();
    let mut partial = FakeSpoolBackend {
        state: partial_state.clone(),
        cancellation: token.clone(),
    };
    let failure = spool_transaction(&mut partial, &tiled_pages, "printer", &token).unwrap_err();
    assert_eq!(failure.pages_completed(), 1);
    assert!(matches!(
        failure.failure_outcome(),
        HardcopyOutcome::Failed {
            pages_completed: 1,
            ..
        }
    ));
    assert_eq!(partial_state.borrow().events.last(), Some(&"abort"));

    let cancelled_state = Rc::new(RefCell::new(FakeSpoolState {
        cancel_after_page: Some(1),
        ..Default::default()
    }));
    let token = HardcopyCancellationToken::default();
    let mut cancelled = FakeSpoolBackend {
        state: cancelled_state.clone(),
        cancellation: token.clone(),
    };
    let outcome = spool_transaction(&mut cancelled, &pages, "printer", &token).unwrap();
    assert!(matches!(
        outcome,
        HardcopyOutcome::Cancelled {
            pages_completed: 1,
            ..
        }
    ));
    assert_eq!(cancelled_state.borrow().events.last(), Some(&"abort"));

    let cancelled_cleanup_state = Rc::new(RefCell::new(FakeSpoolState {
        cancel_after_page: Some(1),
        abort_fails: true,
        ..Default::default()
    }));
    let token = HardcopyCancellationToken::default();
    let mut cancelled_cleanup = FakeSpoolBackend {
        state: cancelled_cleanup_state,
        cancellation: token.clone(),
    };
    assert!(matches!(
        spool_transaction(&mut cancelled_cleanup, &pages, "printer", &token),
        Err(HardcopySpoolFailure {
            error: HardcopyPrintError::SpoolCleanupFailed { .. },
            pages_completed: 1,
        })
    ));

    let cleanup_state = Rc::new(RefCell::new(FakeSpoolState {
        fail_on_page: Some(1),
        abort_fails: true,
        ..Default::default()
    }));
    let token = HardcopyCancellationToken::default();
    let mut cleanup_failed = FakeSpoolBackend {
        state: cleanup_state,
        cancellation: token.clone(),
    };
    assert!(matches!(
        spool_transaction(&mut cleanup_failed, &pages, "printer", &token),
        Err(HardcopySpoolFailure {
            error: HardcopyPrintError::SpoolCleanupFailed { .. },
            pages_completed: 0,
        })
    ));
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn publication_preflight_rejects_plan_dpi_mismatch() {
    let capabilities =
        PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
    let plan = native_plan(
        &capabilities,
        PrinterMediaSource::AutomaticCompatibleTray,
        72,
        DuplexMode::Off,
        1,
        false,
    );
    let pages = rendered_pages(&plan, 72);
    let mut resolved = resolve_native_printer_job(&plan, &capabilities).unwrap();
    resolved.resolution_dpi = 600;
    assert!(matches!(
        validate_printer_publication(&plan, &pages, &resolved),
        Err(HardcopyPrintError::PrinterPublicationMismatch(_))
    ));
}

/// A CUPS job is the plan's pages as one PDF, handed to `lp` with the
/// queue's own paper and tray keywords, the sides, the copies and collation;
/// the receipt carries the request id `lp` reports, and the job's file is
/// gone once `lp` has it. A queue that refuses the job fails it and leaves
/// nothing behind either.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn cups_spool_hands_lp_one_pdf_and_reports_its_request_id() {
    let capabilities =
        PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
    let plan = native_plan(
        &capabilities,
        PrinterMediaSource::NamedTray("Upper tray".to_owned()),
        72,
        DuplexMode::LongEdge,
        2,
        true,
    );
    let pages = rendered_pages(&plan, 72);
    let folder = std::env::temp_dir().join(format!("rspice-cups-test-{}", Uuid::new_v4().simple()));
    let submitted = RefCell::new(None);
    let outcome = cups::spool_with(
        &plan,
        &pages,
        &capabilities,
        &HardcopyCancellationToken::default(),
        &folder,
        &|job| {
            let pdf = std::fs::read(job.file).expect("the job's PDF is written before lp");
            *submitted.borrow_mut() = Some((
                job.arguments()
                    .into_iter()
                    .map(|argument| argument.to_string_lossy().into_owned())
                    .collect::<Vec<_>>(),
                pdf,
            ));
            Ok("request id is engineering-printer-04-17 (1 file(s))\n".to_owned())
        },
    )
    .unwrap();
    assert!(
        matches!(
            &outcome,
            HardcopyOutcome::SpoolAccepted { job_id, pages_accepted: 1, .. }
                if job_id == "engineering-printer-04-17"
        ),
        "{outcome:?}"
    );
    let (arguments, pdf) = submitted.into_inner().expect("lp was called");
    for expected in [
        "engineering-printer-04",
        "media=Letter",
        "sides=two-sided-long-edge",
        "collate=true",
        "InputSlot=Upper tray",
    ] {
        assert!(
            arguments.iter().any(|argument| argument == expected),
            "{arguments:?}"
        );
    }
    assert_eq!(
        arguments[arguments.iter().position(|a| a == "-n").unwrap() + 1],
        "2"
    );
    let document = lopdf::Document::load_mem(&pdf).expect("a valid PDF");
    assert_eq!(document.get_pages().len(), 1);
    let page = document.get_pages()[&1];
    let media_box = document
        .get_object(page)
        .and_then(lopdf::Object::as_dict)
        .and_then(|page| page.get(b"MediaBox"))
        .and_then(lopdf::Object::as_array)
        .unwrap()
        .iter()
        .map(|value| value.as_float().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(media_box, [0.0, 0.0, 792.0, 612.0]);
    assert_eq!(std::fs::read_dir(&folder).unwrap().count(), 0);

    let refused = cups::spool_with(
        &plan,
        &pages,
        &capabilities,
        &HardcopyCancellationToken::default(),
        &folder,
        &|_| {
            Err(HardcopyPrintError::Cups {
                operation: "submitting the job",
                message: "lp: Unable to print file".to_owned(),
            })
        },
    )
    .unwrap_err();
    assert!(
        refused.to_string().contains("Unable to print file"),
        "{refused}"
    );
    assert_eq!(std::fs::read_dir(&folder).unwrap().count(), 0);
    std::fs::remove_dir_all(&folder).unwrap();
}
