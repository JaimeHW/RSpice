//! What each editable hardcopy setting actually changes.
//!
//! A control that exists and changes nothing is a defect of the same weight as
//! a wrong number, so nothing here trusts the draft: every case edits the field
//! its control writes and then compares published output.
//!
//! The comparison is deliberately made in pixels. Both vector writers stamp the
//! plan's own digest into the artifact, so comparing artifact bytes would
//! report every setting that merely reaches the sealed setup as effective —
//! including one that moves no mark on the page. Rasterising the same sealed
//! plan drops that stamp and leaves the ink. Only the four settings the output
//! format itself owns are compared in the artifact, each against the specific
//! property it governs.
//!
//! The base draft is bound to a resolved drawing sheet, because the
//! drawing-sheet settings only reach the scene through source resolution and
//! would look dead against a synthetic one.

use super::*;

use crate::hardcopy::sources::HardcopySourceIdentity;
use crate::hardcopy::{
    HardcopyDocumentId, HardcopyPlanId, HardcopyScope, PrintColor, PrintMappingEntry,
    PrintRedundancy,
};
use crate::product::ObjectRevision;
use crate::state::SchematicSheetFormat;
use crate::workbench::hardcopy_adapters::render::{
    HardcopyPublicationTimestamp, HardcopyRenderer, HardcopySceneMetadata,
};
use crate::workbench::hardcopy_adapters::sources::resolve_blank_schematic_sheet_with_format;

/// Fixed so two publications differ only where a setting made them differ; a
/// fresh plan identity per compile would make every comparison pass.
const PLAN_ID: u128 = 0x0048_4152_4443_4f50_595f_4834;

/// Preview resolution for the ink comparison. Low enough that a tiled case
/// stays cheap, high enough that a hairline still lands on a pixel.
const INK_DPI: u16 = 72;

/// An authored bleed puts the source extent outside the paper edge, which is
/// what makes the clip policy and the output extent observable at all.
fn sheet_format() -> SchematicSheetFormat {
    SchematicSheetFormat::default()
        .try_update(|draft| draft.bleed_um = 5_000)
        .expect("authored bleed is a legal sheet format")
}

fn sheet(source_key: &str, document_id: u128, display_name: &str) -> ResolvedHardcopyDocument {
    resolve_blank_schematic_sheet_with_format(
        HardcopySourceIdentity::try_new(
            source_key,
            HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(document_id)).unwrap(),
            ObjectRevision::INITIAL,
            display_name,
        )
        .unwrap(),
        HardcopyScope::CurrentSheet,
        Some(&sheet_format()),
    )
    .unwrap()
}

/// The dialog as the Export workflow opens it on a resolved drawing sheet,
/// then moved to a vector artifact so the rasteriser has text to lay out.
fn draft() -> HardcopyDialogState {
    let mut draft = HardcopyDialogState::default();
    draft
        .open_resolved(
            HardcopyWorkflow::Export,
            sheet("hardcopy-setting-effects", 0x4834_0001, "Setting effects"),
            None,
        )
        .expect("blank drawing sheet opens");
    draft.format = OutputFormat::SvgVector;
    draft.refresh_preview();
    draft
}

fn metadata() -> HardcopySceneMetadata {
    let mut metadata =
        HardcopySceneMetadata::try_new("Setting effects", "RSpice hardcopy tests").unwrap();
    metadata.set_publication_timestamp(
        HardcopyPublicationTimestamp::try_new(2026, 8, 11, 9, 0, 0).unwrap(),
    );
    metadata
        .set_header_lines(vec!["hardcopy setting audit".to_owned()])
        .unwrap();
    metadata
        .set_provenance_lines(vec!["retained sheet · sealed setup".to_owned()])
        .unwrap();
    metadata
}

fn plan(draft: &HardcopyDialogState) -> Result<HardcopyPlan, String> {
    let setup = draft.build_setup().map_err(|error| error.to_string())?;
    let document = draft.resolved_document.as_ref().expect("resolved source");
    let extent = document
        .content_extent_for_setup(setup.schematic())
        .map_err(|error| error.to_string())?;
    HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(uuid::Uuid::from_u128(PLAN_ID)).unwrap(),
        document.authority().clone(),
        setup,
        extent,
    )
    .map_err(|error| error.to_string())
}

/// Every planned page of the sealed plan, rasterised and concatenated with its
/// own dimensions so a page that only changes size still registers.
fn ink(draft: &HardcopyDialogState) -> Result<Vec<u8>, String> {
    let plan = plan(draft)?;
    let document = draft.resolved_document.as_ref().expect("resolved source");
    let mut pixels = Vec::new();
    for index in 0..plan.pagination().pages().len() {
        let pages = HardcopyRenderer::render_preview_pages_resolved(
            &plan,
            document,
            metadata(),
            &[index],
            INK_DPI,
            || false,
        )
        .map_err(|error| error.to_string())?;
        for page in pages {
            pixels.extend_from_slice(&page.width().to_le_bytes());
            pixels.extend_from_slice(&page.height().to_le_bytes());
            pixels.extend_from_slice(page.rgba());
        }
    }
    Ok(pixels)
}

fn artifact(draft: &HardcopyDialogState) -> Result<Vec<u8>, String> {
    artifact_with(draft, metadata())
}

fn artifact_with(
    draft: &HardcopyDialogState,
    metadata: HardcopySceneMetadata,
) -> Result<Vec<u8>, String> {
    let plan = plan(draft)?;
    let document = draft.resolved_document.as_ref().expect("resolved source");
    HardcopyRenderer::render_resolved(&plan, document, metadata)
        .map_err(|error| error.to_string())
        .map(|publication| {
            publication
                .parts()
                .iter()
                .flat_map(|part| part.bytes().to_vec())
                .collect()
        })
}

/// One editable setting, the mode its control is reachable in, and the edit
/// that control performs.
struct Setting {
    name: &'static str,
    /// Applied to both sides. A setting that only bites in one mode is
    /// compared against that mode instead of against the default.
    context: fn(&mut HardcopyDialogState),
    edit: fn(&mut HardcopyDialogState),
}

fn unchanged(_: &mut HardcopyDialogState) {}

/// Two hundred percent forces the content across a page boundary, which is the
/// only state in which an overlap, a page grid, or a page coordinate exists.
fn tiled(draft: &mut HardcopyDialogState) {
    draft.scale = ScaleMode::CustomPercent {
        hundredths_percent: 20_000,
    };
}

const INK_SETTINGS: &[Setting] = &[
    Setting {
        name: "paper",
        context: unchanged,
        edit: |draft| draft.paper = PaperDraft::Standard(StandardPaper::A3),
    },
    Setting {
        name: "paper · match authored sheets",
        context: unchanged,
        edit: |draft| draft.paper = PaperDraft::MatchAuthoredSheets,
    },
    Setting {
        name: "orientation",
        context: unchanged,
        edit: |draft| draft.orientation = Orientation::Portrait,
    },
    Setting {
        name: "margin_top",
        context: unchanged,
        edit: |draft| draft.margin_top = "1.25".to_owned(),
    },
    Setting {
        name: "margin_right",
        context: unchanged,
        edit: |draft| draft.margin_right = "1.25".to_owned(),
    },
    Setting {
        name: "margin_bottom",
        context: unchanged,
        edit: |draft| draft.margin_bottom = "1.25".to_owned(),
    },
    Setting {
        name: "margin_left",
        context: unchanged,
        edit: |draft| draft.margin_left = "1.25".to_owned(),
    },
    Setting {
        name: "bleed",
        context: unchanged,
        edit: |draft| draft.bleed = "0.125".to_owned(),
    },
    Setting {
        name: "scale",
        context: unchanged,
        edit: |draft| draft.scale = ScaleMode::EngineeringOneToOne,
    },
    Setting {
        name: "custom_scale_percent",
        context: tiled,
        edit: |draft| draft.custom_scale_percent = "150".to_owned(),
    },
    Setting {
        name: "overlap",
        context: tiled,
        edit: |draft| draft.overlap = "0.75".to_owned(),
    },
    Setting {
        name: "registration_marks",
        context: tiled,
        edit: |draft| draft.registration_marks = !draft.registration_marks,
    },
    Setting {
        name: "schematic_extent",
        // The authored extent and the complete-content extent only differ
        // where something is being left out, so the clip policy is the mode
        // this control is reachable in.
        context: |draft| {
            draft.outside_sheet_content = OutsideSheetContentPolicy::ClipToAuthoredSheet
        },
        edit: |draft| draft.schematic_extent = SchematicHardcopyExtent::CompleteSchematicContent,
    },
    Setting {
        name: "outside_sheet_content",
        context: unchanged,
        edit: |draft| draft.outside_sheet_content = OutsideSheetContentPolicy::ClipToAuthoredSheet,
    },
    Setting {
        name: "crop_marks",
        context: unchanged,
        edit: |draft| draft.crop_marks = !draft.crop_marks,
    },
    Setting {
        name: "include_sheet_paper",
        context: unchanged,
        edit: |draft| draft.include_sheet_paper = !draft.include_sheet_paper,
    },
    Setting {
        name: "include_sheet_border",
        context: unchanged,
        edit: |draft| draft.include_sheet_border = !draft.include_sheet_border,
    },
    Setting {
        name: "include_sheet_title_block",
        context: unchanged,
        edit: |draft| draft.include_sheet_title_block = !draft.include_sheet_title_block,
    },
    Setting {
        name: "include_sheet_zones",
        context: unchanged,
        edit: |draft| draft.include_sheet_zones = !draft.include_sheet_zones,
    },
    Setting {
        name: "include_schematic_grid",
        context: unchanged,
        edit: |draft| draft.include_schematic_grid = !draft.include_schematic_grid,
    },
    Setting {
        name: "color_mapping",
        context: unchanged,
        edit: |draft| draft.color_mapping = ColorMapping::Monochrome,
    },
    Setting {
        name: "background",
        context: unchanged,
        edit: |draft| draft.background = BackgroundMode::Transparent,
    },
    Setting {
        name: "include_legends",
        context: unchanged,
        edit: |draft| draft.include_legends = !draft.include_legends,
    },
    Setting {
        name: "include_header",
        context: unchanged,
        edit: |draft| draft.include_header = !draft.include_header,
    },
    Setting {
        name: "include_provenance",
        context: unchanged,
        edit: |draft| draft.include_provenance = !draft.include_provenance,
    },
    Setting {
        name: "print_mapping",
        context: unchanged,
        edit: |draft| {
            let entries = draft
                .print_mapping
                .entries()
                .iter()
                .map(|entry| {
                    PrintMappingEntry::try_new(
                        entry.object().clone(),
                        PrintColor::GrayPercent(50),
                        PrintRedundancy::DashedLine {
                            width: Length::from_micrometres(400),
                            dash: Length::from_micrometres(1_200),
                            gap: Length::from_micrometres(600),
                        },
                        entry.include_in_legend(),
                    )
                    .unwrap()
                })
                .collect();
            draft.print_mapping =
                PrintMappingTable::try_new(draft.print_mapping.save_scope().clone(), entries)
                    .unwrap();
        },
    },
    Setting {
        name: "soft_proof",
        context: unchanged,
        edit: |draft| draft.soft_proof = !draft.soft_proof,
    },
];

/// Every setting is checked before anything is reported, so one dead control
/// cannot hide the next one behind it.
#[test]
fn every_page_and_scene_setting_moves_the_ink_on_the_page() {
    let mut inert = Vec::new();
    for setting in INK_SETTINGS {
        let mut before = draft();
        (setting.context)(&mut before);
        let mut after = draft();
        (setting.context)(&mut after);
        (setting.edit)(&mut after);

        match (ink(&before), ink(&after)) {
            (Ok(baseline), Ok(changed)) if baseline == changed => {
                inert.push(format!("{}: changed no ink on the page", setting.name));
            }
            (Ok(_), Ok(_)) => {}
            (Err(error), _) => inert.push(format!(
                "{}: baseline is unpublishable: {error}",
                setting.name
            )),
            (_, Err(error)) => {
                inert.push(format!("{}: edit is unpublishable: {error}", setting.name))
            }
        }
    }
    assert!(inert.is_empty(), "{}", inert.join("\n"));
}

/// Single-page mode is a guard over the scale, and the scale control is what
/// makes a drawing fit. Deriving a scale here instead would be a second
/// spelling of fit-to-printable-area, so what this pins is that the refusal is
/// real and that the remedy it names is the one that works.
#[test]
fn single_page_mode_refuses_overflow_and_the_scale_it_names_publishes_it() {
    let automatic = draft();
    let mut single = draft();
    single.tiling = TilingMode::SinglePage;
    assert_eq!(
        ink(&single).expect("content that fits publishes as a single page"),
        ink(&automatic).expect("automatic tiling publishes"),
        "single-page mode changed a page that already fit"
    );

    let mut refused = draft();
    tiled(&mut refused);
    refused.tiling = TilingMode::SinglePage;
    let refusal = plan(&refused).expect_err("content that overflows cannot be one page");
    assert!(
        refusal.contains("single-page mode needs"),
        "single-page mode silently accepted overflowing content"
    );
    assert!(
        refusal.contains("fit-to-printable-area"),
        "the refusal does not name the control that fixes it: {refusal}"
    );

    let mut fitted = refused;
    fitted.scale = ScaleMode::FitPrintableArea;
    assert_eq!(
        plan(&fitted)
            .expect("the named scale publishes what single-page mode refused")
            .pagination()
            .pages()
            .len(),
        1
    );
}

/// Soft proof is an on-screen simulation of print colour, so it must reach the
/// preview raster and must never reach the artifact.
#[test]
fn soft_proof_is_a_preview_treatment_and_not_an_artifact_one() {
    let mut proofed = draft();
    proofed.soft_proof = true;
    let mut direct = draft();
    direct.soft_proof = false;

    let proofed_artifact = artifact(&proofed).expect("proofed draft publishes");
    let direct_artifact = artifact(&direct).expect("direct draft publishes");
    // Both writers stamp the plan digest, and soft proof is part of the sealed
    // setup, so the artifacts differ there and only there.
    assert_eq!(
        strip_plan_metadata(&proofed_artifact),
        strip_plan_metadata(&direct_artifact),
        "soft proof changed the published drawing, not only the proof"
    );
}

/// The SVG writer records its plan in a `<metadata>` element. Removing it is
/// what separates "the setting reached the sealed plan" from "the setting
/// changed the drawing".
fn strip_plan_metadata(artifact: &[u8]) -> Vec<u8> {
    let text = String::from_utf8_lossy(artifact);
    let (Some(start), Some(end)) = (text.find("<metadata>"), text.find("</metadata>")) else {
        panic!("the SVG writer no longer records a plan metadata element");
    };
    let mut stripped = text[..start].as_bytes().to_vec();
    stripped.extend_from_slice(text[end..].as_bytes());
    stripped
}

/// The manual grid chooses how many sheets the drawing is spread over, so the
/// fields have to move ink and not merely restate the automatic answer. A grid
/// too small to cover is the one thing they may not ask for.
#[test]
fn the_manual_grid_decides_how_many_sheets_the_drawing_is_spread_over() {
    let mut automatic = draft();
    tiled(&mut automatic);
    let pagination = plan(&automatic).expect("automatic tiling plans");
    let (columns, rows) = (
        pagination.pagination().columns(),
        pagination.pagination().rows(),
    );
    assert!(columns > 1, "the fixture does not tile across");

    let mut manual = draft();
    tiled(&mut manual);
    manual.tiling = TilingMode::Manual { columns, rows };
    manual.manual_columns = (columns + 1).to_string();
    manual.manual_rows = rows.to_string();
    let wider = plan(&manual).expect("a grid larger than the minimum covers the content");
    assert_eq!(
        wider.pagination().pages().len(),
        usize::from(columns + 1) * usize::from(rows),
        "the requested grid is not the grid that was published"
    );
    assert_ne!(
        ink(&manual).expect("the wider grid publishes"),
        ink(&automatic).expect("automatic tiling publishes"),
        "asking for more sheets published the same pages"
    );
    // The surplus the extra sheet buys is overlap, and the seam it leaves is
    // wider than the overlap the operator declared rather than equal to it.
    let declared = Length::parse_decimal(&manual.overlap, manual.display_unit).unwrap();
    let seam = wider
        .pagination()
        .tile_overlap()
        .expect("a tiled grid has a seam");
    assert!(seam > declared, "the extra sheet was not spent on the seam");

    manual.manual_columns = (columns - 1).to_string();
    let refusal = plan(&manual).expect_err("a grid that cannot cover is refused");
    assert!(
        refusal.contains("is smaller than the"),
        "unexpected refusal: {refusal}"
    );
}

/// Checked in the artifact because that is where the mark is named rather than
/// merely present: the rasterised pages prove it is drawn, the text proves it
/// is the word the setting asks for. `the_watermark_reaches_every_rasterised_page`
/// is the ink half.
#[test]
fn the_watermark_reaches_the_published_artifact() {
    let mut marked = draft();
    marked.watermark = Watermark::Draft;
    let mut plain = draft();
    plain.watermark = Watermark::None;

    let marked = artifact(&marked).expect("watermarked draft publishes");
    assert!(
        String::from_utf8_lossy(&marked).contains("DRAFT"),
        "the watermark never reached the artifact"
    );
    assert!(
        !String::from_utf8_lossy(&artifact(&plain).expect("plain draft publishes"))
            .contains("DRAFT")
    );
}

#[test]
fn the_output_format_decides_the_artifact_it_writes() {
    let mut svg = draft();
    svg.format = OutputFormat::SvgVector;
    let mut pdf = draft();
    pdf.format = OutputFormat::PdfVector;

    assert_ne!(
        artifact(&svg).expect("SVG publishes"),
        artifact(&pdf).expect("PDF publishes"),
        "the output format never reached the writer"
    );
}

#[test]
fn embedding_fonts_puts_the_typeface_in_the_artifact() {
    // An SVG cannot outline text, so dropping the fonts is only legal once the
    // page carries none: no sheet geometry, no legend, and no header or
    // provenance line in the metadata.
    let lettering_free = |draft: &mut HardcopyDialogState| {
        draft.include_sheet_paper = false;
        draft.include_sheet_border = false;
        draft.include_sheet_title_block = false;
        draft.include_sheet_zones = false;
        draft.include_schematic_grid = false;
        draft.crop_marks = false;
    };
    let bare = || {
        let mut metadata =
            HardcopySceneMetadata::try_new("Setting effects", "RSpice hardcopy tests").unwrap();
        metadata.set_publication_timestamp(
            HardcopyPublicationTimestamp::try_new(2026, 8, 11, 9, 0, 0).unwrap(),
        );
        metadata
    };
    let mut embedded = draft();
    lettering_free(&mut embedded);
    let mut linked = draft();
    lettering_free(&mut linked);
    linked.embed_fonts = false;
    linked.searchable_text = false;

    let embedded = artifact_with(&embedded, bare()).expect("embedded draft publishes");
    let linked = artifact_with(&linked, bare()).expect("linked draft publishes");
    assert!(
        String::from_utf8_lossy(&embedded).contains("@font-face"),
        "embedding fonts wrote no typeface into the artifact"
    );
    assert!(!String::from_utf8_lossy(&linked).contains("@font-face"));
}

#[test]
fn preserving_searchable_text_leaves_text_a_reader_can_extract() {
    let mut searchable = draft();
    searchable.format = OutputFormat::PdfVector;
    let mut outlined = draft();
    outlined.format = OutputFormat::PdfVector;
    outlined.searchable_text = false;

    let extract = |bytes: &[u8]| {
        let parsed = lopdf::Document::load_mem(bytes).expect("valid PDF");
        let pages = parsed.get_pages().keys().copied().collect::<Vec<_>>();
        parsed.extract_text(&pages).unwrap_or_default()
    };
    assert!(
        extract(&artifact(&searchable).expect("searchable PDF publishes"))
            .contains("Setting effects"),
        "a searchable PDF has no extractable title"
    );
    assert!(
        !extract(&artifact(&outlined).expect("outlined PDF publishes")).contains("Setting effects"),
        "outlining text left extractable text behind"
    );
}

#[test]
fn the_resolution_a_raster_export_commits_reaches_the_artifact() {
    // A raster target has no text to embed or search; the dialog clears both
    // when the format changes, and the contract refuses the setup otherwise.
    let raster = |dpi| {
        let mut draft = draft();
        draft.format = OutputFormat::Png { dpi };
        draft.embed_fonts = false;
        draft.searchable_text = false;
        draft
    };

    assert_ne!(
        artifact(&raster(150)).expect("150 dpi publishes"),
        artifact(&raster(300)).expect("300 dpi publishes"),
        "raster resolution changed nothing in the published artifact"
    );
    // The field is a text buffer over that value: reopening on a committed
    // resolution must show it, or the control would report a number the
    // artifact does not use.
    assert_eq!(
        HardcopyDialogState::from_setup(raster(300).build_setup().expect("raster setup"))
            .raster_dpi_draft,
        "300"
    );
}

#[test]
fn display_units_change_the_fields_and_nothing_the_page_can_see() {
    let inches = draft();
    let mut millimetres = draft();
    millimetres
        .set_display_unit(LengthUnit::Millimetres)
        .expect("valid draft converts");

    assert_ne!(millimetres.margin_top, inches.margin_top);
    assert_eq!(
        millimetres.build_setup().expect("converted setup"),
        inches.build_setup().expect("setup"),
        "changing the display unit moved the sealed setup"
    );
}

/// The printer identity and its authenticated job are one atomic setting, and
/// they only reach a render target on the platform whose dialog offers them.
#[test]
#[cfg(target_os = "windows")]
fn the_selected_printer_reaches_the_sealed_render_target() {
    let job = |media, dpi, duplex, copies, collate| {
        PrinterJobSettings::try_new(
            crate::product::ContentDigest::from_bytes([0x48; 32]),
            "paper-letter",
            PrinterRasterGeometry::try_new(1_200, 900, 0, 0, 1_200, 900).unwrap(),
            media,
            dpi,
            duplex,
            copies,
            collate,
        )
        .unwrap()
    };
    let mut draft = draft();
    draft.workflow = HardcopyWorkflow::Print;
    draft.format = runtime_print_format();
    draft.embed_fonts = false;
    draft.searchable_text = false;

    draft.set_printer(
        "Hardcopy audit device".to_owned(),
        job(
            PrinterMediaSource::ManualFeed,
            300,
            DuplexMode::Off,
            2,
            true,
        ),
    );
    let manual_feed = draft.build_setup().expect("printer target compiles");

    draft.set_printer(
        "Hardcopy audit device".to_owned(),
        job(
            PrinterMediaSource::AutomaticCompatibleTray,
            600,
            DuplexMode::LongEdge,
            1,
            false,
        ),
    );
    let automatic_tray = draft.build_setup().expect("second printer job compiles");

    assert_ne!(
        manual_feed.render().target(),
        automatic_tray.render().target(),
        "the printer job is not carried by the sealed render target"
    );

    draft.clear_printer();
    assert!(
        draft.build_setup().is_err(),
        "a print job compiles without an authenticated printer identity"
    );
}

#[test]
fn the_mapping_save_scope_and_name_reach_the_saved_mapping() {
    let mut draft = draft();
    draft.open_subflow(HardcopyDialogPage::PrintMapping);
    assert_eq!(
        draft.mapping_scope_draft,
        PrintMappingSaveScope::Document,
        "a resolved document opens on its own embedded mapping"
    );

    draft.mapping_scope_draft = PrintMappingSaveScope::ProjectPrintSet(String::new());
    draft.mapping_name_draft = "Fabrication review".to_owned();
    draft.accept_subflow().expect("named project set saves");

    assert_eq!(
        draft.print_mapping.save_scope(),
        &PrintMappingSaveScope::ProjectPrintSet("Fabrication review".to_owned())
    );
    assert_eq!(
        draft
            .build_setup()
            .expect("setup compiles")
            .print_mapping()
            .save_scope(),
        &PrintMappingSaveScope::ProjectPrintSet("Fabrication review".to_owned()),
        "the chosen save scope never reached the committed setup"
    );

    draft.open_subflow(HardcopyDialogPage::PrintMapping);
    draft.mapping_scope_draft = PrintMappingSaveScope::PortablePersonalPreset(String::new());
    draft.accept_subflow().expect("personal preset saves");
    assert_eq!(
        draft.print_mapping.save_scope(),
        &PrintMappingSaveScope::PortablePersonalPreset("Fabrication review".to_owned()),
        "the scope selector and the name field are not read together"
    );
}

#[test]
fn a_name_the_catalog_would_refuse_is_refused_at_the_field() {
    let mut draft = draft();
    draft.open_subflow(HardcopyDialogPage::PrintMapping);
    draft.mapping_scope_draft = PrintMappingSaveScope::ProjectPrintSet(String::new());

    for name in ["", "  ", " padded", "trailing ", "line\nbreak"] {
        draft.mapping_name_draft = name.to_owned();
        let refusal = draft
            .mapping_scope_refusal()
            .unwrap_or_else(|| panic!("{name:?} was accepted by the field"));
        assert!(
            refusal.contains("project print set mapping name"),
            "the refusal does not name the field: {refusal}"
        );
        assert!(
            draft.accept_subflow().is_err(),
            "{name:?} was refused at the field but accepted on save"
        );
    }

    draft.mapping_name_draft = "Fabrication review".to_owned();
    assert!(draft.mapping_scope_refusal().is_none());
    assert!(draft.accept_subflow().is_ok());
}

#[test]
fn the_document_scope_needs_no_name_and_never_reports_one_as_missing() {
    let mut draft = draft();
    draft.open_subflow(HardcopyDialogPage::PrintMapping);
    draft.mapping_scope_draft = PrintMappingSaveScope::Document;
    draft.mapping_name_draft = String::new();

    assert!(draft.mapping_scope_refusal().is_none());
    assert_eq!(
        draft.edited_mapping_scope(),
        PrintMappingSaveScope::Document
    );
    draft.accept_subflow().expect("document scope saves");
    assert_eq!(
        draft.print_mapping.save_scope(),
        &PrintMappingSaveScope::Document
    );
}

#[test]
fn the_selected_source_reaches_the_sealed_plan() {
    let first = plan(&draft()).expect("first source plans");

    let mut other = HardcopyDialogState::default();
    other
        .open_resolved(
            HardcopyWorkflow::Export,
            sheet("hardcopy-setting-effects-b", 0x4834_0002, "Second sheet"),
            None,
        )
        .expect("second sheet opens");
    other.format = OutputFormat::SvgVector;
    other.refresh_preview();

    assert_ne!(
        first.content_digest(),
        plan(&other).expect("second source plans").content_digest(),
        "the selected source does not reach the sealed plan"
    );
}

#[test]
fn the_preview_page_and_zoom_never_reach_the_sealed_plan() {
    let mut viewed = draft();
    tiled(&mut viewed);
    let baseline = plan(&viewed).expect("tiled draft plans");

    viewed.preview_page = 1;
    viewed.preview_zoom_percent = 220;
    assert_eq!(
        plan(&viewed).expect("tiled draft still plans").setup(),
        baseline.setup(),
        "a preview viewing control leaked into the sealed setup"
    );
}
