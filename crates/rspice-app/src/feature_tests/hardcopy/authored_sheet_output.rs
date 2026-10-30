//! A drawing sheet's output with nothing saved for it. Exported, it is the
//! sheet itself, on its own paper at 1:1, with no band its title block would
//! only repeat. Printed, it is fitted inside the printer's margin, so it is
//! never refused; at a 1:1 the user chose, a printer that cannot reach the
//! sheet's ink says so in numbers.

use crate::feature_tests::hardcopy::{hardcopy_parts, resolve_retained_hardcopy_source};
use crate::product::ContentDigest;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::hardcopy::HardcopyWorkflow;
use crate::workbench::app::dialogs::hardcopy::publish::{
    active_retained_source_selection, apply_open_resolved, current_metadata, current_plan,
};
use crate::workbench::hardcopy_adapters::print::{
    PrinterCapabilityDescriptor, PrinterCapabilitySnapshot, PrinterPaperCapability,
    PrinterResolutionCapability, PrinterTrayCapability,
};
use crate::workbench::hardcopy_adapters::sources::ResolvedHardcopyDocument;
use crate::workbench::state::WorkspaceDocumentId;
use rspice_hardcopy::render::{
    HardcopyRenderError, HardcopyRenderer, HardcopySceneMetadata, RenderedPrinterPages,
};
use rspice_hardcopy_contract::{
    BackgroundMode, DuplexMode, FontPolicy, HardcopyPlan, HardcopySetup, Length, Orientation,
    OutputFormat, PageMargins, PaperSize, PrinterJobSettings, PrinterMediaSource,
    PrinterRasterGeometry, RenderSetup, RenderTarget, ScaleMode, StandardPaper,
};

/// An application whose active schematic stands on an ISO A3 landscape
/// sheet, with nothing saved for its output, and the dialog opened on it for
/// export the way the File menu opens it.
fn exporting_an_a3_sheet() -> (RSpiceApp, ResolvedHardcopyDocument) {
    let mut app = RSpiceApp::test_instance();
    let reference = app.state.model.workspace.content.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let design = &mut app.state.model.workspace.content.design_management;
    let mut settings = design.drawing_sheet_settings().clone();
    settings.default_format = settings
        .default_format
        .try_update(|draft| {
            draft.authored_size = crate::state::AuthoredDrawingSheetSize::Standard {
                standard: crate::state::DrawingSheetStandard::IsoA3,
            };
            draft.orientation = crate::state::SchematicPageOrientation::Landscape;
        })
        .unwrap();
    let revision = design.revision();
    design
        .update_drawing_sheet_settings(revision, settings)
        .unwrap();
    let (source_key, scope) = active_retained_source_selection(&hardcopy_parts(&mut app)).unwrap();
    let resolved = resolve_retained_hardcopy_source(&app.state, &source_key, scope).unwrap();
    apply_open_resolved(
        &mut hardcopy_parts(&mut app),
        HardcopyWorkflow::Export,
        resolved.clone(),
    )
    .unwrap();
    (app, resolved)
}

/// The exported page is the A3 sheet: 420 × 297 mm in both the PDF and the
/// SVG, drawn at 1:1, with no header, legend or provenance band.
#[test]
fn an_a3_sheet_with_nothing_saved_exports_on_a3_at_one_to_one() {
    let (mut app, resolved) = exporting_an_a3_sheet();
    for format in [OutputFormat::PdfVector, OutputFormat::SvgVector] {
        app.state.hardcopy.hardcopy.format = format;
        app.state.hardcopy.hardcopy.refresh_preview();
        let parts = hardcopy_parts(&mut app);
        let plan = current_plan(&parts, &resolved).expect("the sealed preview publishes");
        assert_eq!(
            plan.pagination().scale().hundredths_percent(),
            10_000,
            "{format:?} is not drawn at 1:1"
        );
        assert_eq!(plan.pagination().pages().len(), 1);
        let decorations = plan.setup().decorations();
        assert!(
            !decorations.includes_header()
                && !decorations.includes_legends()
                && !decorations.includes_provenance()
        );
        let metadata = current_metadata(&parts).unwrap();
        let publication = HardcopyRenderer::render_resolved(&plan, &resolved, metadata)
            .expect("the sheet publishes");
        let bytes = publication.parts()[0].bytes();
        match format {
            OutputFormat::PdfVector => {
                let pdf = lopdf::Document::load_mem(bytes).expect("a valid PDF");
                let pages = pdf.get_pages();
                assert_eq!(pages.len(), 1);
                let page = pdf.get_dictionary(*pages.values().next().unwrap()).unwrap();
                let media_box = page
                    .get(b"MediaBox")
                    .and_then(lopdf::Object::as_array)
                    .expect("the page names its paper")
                    .iter()
                    .map(|value| value.as_float().unwrap())
                    .collect::<Vec<_>>();
                let points = |millimetres: f32| millimetres / 25.4 * 72.0;
                assert_eq!(media_box.len(), 4);
                assert!(
                    (media_box[2] - media_box[0] - points(420.0)).abs() < 0.01
                        && (media_box[3] - media_box[1] - points(297.0)).abs() < 0.01,
                    "the PDF page is {media_box:?} pt, not A3 landscape"
                );
            }
            _ => {
                let svg = std::str::from_utf8(bytes).unwrap();
                assert!(
                    svg.contains(
                        r#"width="420.000mm" height="297.000mm" viewBox="0 0 420000 297000""#
                    ),
                    "the SVG page is not A3 landscape at 1:1"
                );
                assert!(
                    !svg.contains("data-rspice-decoration"),
                    "the sheet carries a band its title block repeats"
                );
            }
        }
    }
}

/// Pages `setup`'s layout would send to a printer whose driver reports
/// `geometry` at `dpi`.
fn printer_pages(
    setup: &HardcopySetup,
    resolved: &ResolvedHardcopyDocument,
    metadata: HardcopySceneMetadata,
    geometry: PrinterRasterGeometry,
    dpi: u16,
) -> Result<RenderedPrinterPages, HardcopyRenderError> {
    let job = PrinterJobSettings::try_new(
        ContentDigest::from_bytes([0x41; 32]),
        "8",
        geometry,
        PrinterMediaSource::AutomaticCompatibleTray,
        dpi,
        DuplexMode::Off,
        1,
        false,
    )
    .unwrap();
    let render = setup.render();
    let printed = HardcopySetup::try_new_with_schematic(
        setup.physical_page().clone(),
        setup.scale(),
        setup.tiling(),
        RenderSetup::try_new(
            RenderTarget::SystemPrinter {
                printer_id: "Engineering printer".to_owned(),
                job,
            },
            OutputFormat::NativePrinter,
            render.color_mapping(),
            BackgroundMode::White,
            FontPolicy::new(true, false),
            render.soft_proof_print_safe_colors(),
        )
        .unwrap(),
        setup.decorations().clone(),
        setup.schematic(),
        setup.print_mapping().clone(),
    )
    .unwrap();
    let extent = resolved
        .content_extent_for_setup(setup.schematic())
        .unwrap();
    let plan = HardcopyPlan::compile(resolved.authority().clone(), printed, extent).unwrap();
    HardcopyRenderer::render_printer_pages_resolved(&plan, resolved, metadata, dpi)
}

/// A landscape page `width_um` × `height_um` at 300 dpi whose driver cannot
/// print within `margin_um` of any edge.
fn landscape_geometry(width_um: u32, height_um: u32, margin_um: u32) -> PrinterRasterGeometry {
    let pixels = |micrometres: u32| (micrometres * 300).div_ceil(25_400);
    let (width, height, margin) = (pixels(width_um), pixels(height_um), pixels(margin_um));
    PrinterRasterGeometry::try_new(
        width,
        height,
        margin,
        margin,
        width - 2 * margin,
        height - 2 * margin,
    )
    .unwrap()
}

/// Printed at 1:1 on an A3 printer, the sheet clears a 0.4 mm unprintable
/// margin. Its ISO size designation is lettered in the strip outside the grid
/// reference border, half a millimetre from the paper's edge, so a 4.2 mm
/// margin reaches its ink, and the refusal says both distances.
#[test]
fn a_printer_margin_wider_than_the_sheets_ink_refuses_in_numbers() {
    let (mut app, resolved) = exporting_an_a3_sheet();
    let parts = hardcopy_parts(&mut app);
    let as_drawn = parts.dialog.build_setup().unwrap();
    let metadata = current_metadata(&parts).unwrap();
    let print = |margin_um| {
        printer_pages(
            &as_drawn,
            &resolved,
            metadata.clone(),
            landscape_geometry(420_000, 297_000, margin_um),
            300,
        )
    };

    let printed = print(400).expect("a 0.4 mm margin clears the sheet's ink");
    let page = &printed.pages()[0];
    let ink = page.ink().expect("the sheet has ink");
    let nearest = [
        ink.left,
        ink.top,
        page.width() - ink.right,
        page.height() - ink.bottom,
    ]
    .into_iter()
    .min()
    .unwrap();
    let ink_mm = f64::from(nearest) * 25.4 / 300.0;
    assert!(
        ink_mm > 0.4 && ink_mm < 4.2,
        "the sheet's ink is {ink_mm} mm in"
    );

    let refusal = print(4_200)
        .expect_err("a 4.2 mm margin reaches the sheet's ink")
        .to_string();
    assert_eq!(
        refusal,
        format!(
            "The printer can't print within 4.2 mm of the paper's edge, but this page has ink \
             {:.1} mm from it. Print it scaled to fit, or choose a larger paper.",
            (u64::from(nearest) * 25_400).div_ceil(300) as f64 / 1_000.0
        )
    );
}

/// A printer offering `papers` (platform id, name, portrait micrometres).
fn printer(papers: &[(i16, &str, u64, u64)]) -> PrinterCapabilitySnapshot {
    PrinterCapabilitySnapshot::try_from_descriptor(PrinterCapabilityDescriptor {
        device_id: "engineering-printer".to_owned(),
        display_name: "Engineering printer".to_owned(),
        driver_name: "RSpice deterministic test driver".to_owned(),
        port_name: "TESTPORT:".to_owned(),
        driver_spec_version: 1,
        driver_version: 1,
        papers: papers
            .iter()
            .map(|&(id, name, width, height)| {
                PrinterPaperCapability::try_new(id, name, width, height).unwrap()
            })
            .collect(),
        trays: vec![PrinterTrayCapability::try_new(1, "Upper tray").unwrap()],
        resolutions: vec![PrinterResolutionCapability::try_new(300, 300).unwrap()],
        duplex_modes: vec![DuplexMode::Off],
        maximum_copies: 99,
        supports_collation: false,
        supports_color: true,
    })
    .unwrap()
}

const LETTER: (i16, &str, u64, u64) = (1, "Letter", 215_900, 279_400);
const A4: (i16, &str, u64, u64) = (9, "A4", 210_000, 297_000);
const A3: (i16, &str, u64, u64) = (8, "A3", 297_000, 420_000);

/// Printed with nothing saved, the A3 sheet is fitted to the printable area
/// of the paper the printer gives it, landscape like the sheet, inside the
/// driver's 4.2 mm margin on every side, and prints without a refusal. The
/// scale it comes to is the fit of 420 × 297 mm inside that margin.
fn fits_on(papers: &[(i16, &str, u64, u64)], default_paper: Option<i16>, expected: StandardPaper) {
    let (mut app, resolved) = exporting_an_a3_sheet();
    let parts = hardcopy_parts(&mut app);
    let metadata = current_metadata(&parts).unwrap();
    let dialog = parts.dialog;
    assert!(
        dialog.fit_unsaved_sheet_to_printer(&printer(papers), default_paper),
        "a sheet with nothing saved was not fitted to the printer"
    );
    let (portrait_width, portrait_height) = expected.portrait_dimensions();
    let (width_um, height_um) = (
        u32::try_from(portrait_height.micrometres()).unwrap(),
        u32::try_from(portrait_width.micrometres()).unwrap(),
    );
    let geometry = landscape_geometry(width_um, height_um, 4_200);
    dialog.keep_unsaved_sheet_inside(geometry, 300);
    dialog.refresh_preview();

    let setup = dialog.build_setup().unwrap();
    let page = setup.physical_page();
    assert_eq!(page.paper(), &PaperSize::Standard(expected));
    assert_eq!(page.orientation(), Orientation::Landscape);
    // 50 px at 300 dpi is 4.234 mm, rounded up to the tenth.
    assert_eq!(
        page.margins(),
        PageMargins::uniform(Length::from_micrometres(4_300))
    );
    assert_eq!(setup.scale(), ScaleMode::FitPrintableArea);
    assert!(!setup.decorations().includes_header());
    let fit = f64::min(
        f64::from(width_um - 8_600) / 420_000.0,
        f64::from(height_um - 8_600) / 297_000.0,
    );
    let scale = dialog
        .preview_plan
        .as_ref()
        .expect("the fitted sheet previews")
        .pagination()
        .scale()
        .hundredths_percent();
    assert!(
        (f64::from(scale) - fit * 10_000.0).abs() <= 1.0,
        "{expected:?} prints at {scale} hundredths of a percent, not the fit {fit}"
    );

    printer_pages(&setup, &resolved, metadata, geometry, 300)
        .expect("a print with nothing saved is never refused for the printer's margin");
}

#[test]
fn printed_with_nothing_saved_an_a3_sheet_fits_an_a3_printers_printable_area() {
    fits_on(&[LETTER, A4, A3], Some(LETTER.0), StandardPaper::A3);
}

#[test]
fn printed_with_nothing_saved_on_a_letter_and_a4_printer_an_a3_sheet_fits_its_default_paper() {
    fits_on(&[LETTER, A4], Some(LETTER.0), StandardPaper::Letter);
    // A driver that names no default paper gets the sheet's own family.
    fits_on(&[LETTER, A4], None, StandardPaper::A4);
}
