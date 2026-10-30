//! A drawing sheet's output with nothing saved for it: the sheet itself, on
//! its own paper at 1:1, with no band its title block would only repeat, and
//! a printer that cannot reach the sheet's ink saying so in numbers.

use crate::feature_tests::hardcopy::{hardcopy_parts, resolve_retained_hardcopy_source};
use crate::product::ContentDigest;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::hardcopy::HardcopyWorkflow;
use crate::workbench::app::dialogs::hardcopy::publish::{
    active_retained_source_selection, apply_open_resolved, current_metadata, current_plan,
};
use crate::workbench::hardcopy_adapters::sources::ResolvedHardcopyDocument;
use crate::workbench::state::WorkspaceDocumentId;
use rspice_hardcopy::render::HardcopyRenderer;
use rspice_hardcopy_contract::OutputFormat;

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

/// Printed at 1:1 on an A3 printer, the sheet clears a 0.4 mm unprintable
/// margin. Its ISO size designation is lettered in the strip outside the grid
/// reference border, half a millimetre from the paper's edge, so a 4.2 mm
/// margin reaches its ink, and the refusal says both distances.
#[test]
fn a_printer_margin_wider_than_the_sheets_ink_refuses_in_numbers() {
    use rspice_hardcopy_contract::{
        BackgroundMode, DuplexMode, FontPolicy, HardcopyPlan, HardcopySetup, PrinterJobSettings,
        PrinterMediaSource, PrinterRasterGeometry, RenderSetup, RenderTarget,
    };

    const DPI: u16 = 300;
    let (mut app, resolved) = exporting_an_a3_sheet();
    let parts = hardcopy_parts(&mut app);
    let seeded = parts.dialog.build_setup().unwrap();
    let metadata = current_metadata(&parts).unwrap();
    let pixels = |micrometres: u32| (micrometres * u32::from(DPI)).div_ceil(25_400);
    let (width, height) = (pixels(420_000), pixels(297_000));
    let print = |margin: u32| {
        let geometry = PrinterRasterGeometry::try_new(
            width,
            height,
            margin,
            margin,
            width - 2 * margin,
            height - 2 * margin,
        )
        .unwrap();
        let job = PrinterJobSettings::try_new(
            ContentDigest::from_bytes([0x41; 32]),
            "8",
            geometry,
            PrinterMediaSource::AutomaticCompatibleTray,
            DPI,
            DuplexMode::Off,
            1,
            false,
        )
        .unwrap();
        let render = seeded.render();
        let setup = HardcopySetup::try_new_with_schematic(
            seeded.physical_page().clone(),
            seeded.scale(),
            seeded.tiling(),
            RenderSetup::try_new(
                RenderTarget::SystemPrinter {
                    printer_id: "A3 plotter".to_owned(),
                    job,
                },
                OutputFormat::NativePrinter,
                render.color_mapping(),
                BackgroundMode::White,
                FontPolicy::new(true, false),
                render.soft_proof_print_safe_colors(),
            )
            .unwrap(),
            seeded.decorations().clone(),
            seeded.schematic(),
            seeded.print_mapping().clone(),
        )
        .unwrap();
        let extent = resolved
            .content_extent_for_setup(seeded.schematic())
            .unwrap();
        let plan = HardcopyPlan::compile(resolved.authority().clone(), setup, extent).unwrap();
        HardcopyRenderer::render_printer_pages_resolved(&plan, &resolved, metadata.clone(), DPI)
    };

    let printed = print(pixels(400)).expect("a 0.4 mm margin clears the sheet's ink");
    let ink = printed.pages()[0].ink().expect("the sheet has ink");
    let nearest = [ink.left, ink.top, width - ink.right, height - ink.bottom]
        .into_iter()
        .min()
        .unwrap();
    let ink_mm = f64::from(nearest) * 25.4 / f64::from(DPI);
    assert!(
        ink_mm > 0.4 && ink_mm < 4.2,
        "the sheet's ink is {ink_mm} mm in"
    );

    let refusal = print(pixels(4_200))
        .expect_err("a 4.2 mm margin reaches the sheet's ink")
        .to_string();
    assert_eq!(
        refusal,
        format!(
            "The printer can't print within 4.2 mm of the paper's edge, but this page has ink \
             {:.1} mm from it. Print it scaled to fit, or choose a larger paper.",
            (u64::from(nearest) * 25_400).div_ceil(u64::from(DPI)) as f64 / 1_000.0
        )
    );
}
