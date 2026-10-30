//! Tests for the publish dialog's output contract.
//!
//! The cases pin that the dialog never misrepresents a print target, and that
//! the mapping stage stays transactional until publication actually commits.

use super::*;

/// Bleed is the print setup's alone. A sheet saved with an export bleed of
/// its own starts its first print setup with it where the page's margins
/// hold it; otherwise, and for every other sheet, the default bleed stands.
#[test]
fn a_sheet_saved_with_a_bleed_seeds_its_first_print_setups_bleed() {
    use crate::state::SchematicSheetFormat;
    use rspice_hardcopy_contract::{Bleed, HardcopySetup, Length};

    let saved_with = |bleed_um: u64| -> SchematicSheetFormat {
        let mut saved = serde_json::to_value(SchematicSheetFormat::default()).unwrap();
        saved["bleed_um"] = serde_json::json!(bleed_um);
        serde_json::from_value(saved).unwrap()
    };
    let seeded_bleed = |format| {
        setup_seeded_from_sheet_format(format)
            .unwrap()
            .physical_page()
            .bleed()
    };
    let default_bleed = HardcopySetup::default().physical_page().bleed();
    assert_eq!(
        seeded_bleed(saved_with(3_175)),
        Bleed::Uniform(Length::from_micrometres(3_175))
    );
    assert_eq!(seeded_bleed(SchematicSheetFormat::default()), default_bleed);
    assert_eq!(seeded_bleed(saved_with(500_000)), default_bleed);
}

#[test]
fn authored_sheet_formats_seed_output_media_without_reverse_coercion() {
    use crate::state::{SchematicPageOrientation, SchematicPageSize, SchematicSheetFormat};
    use rspice_hardcopy_contract::{
        HardcopySetup, Orientation, PaperSize, PhysicalPageSetup, StandardPaper,
    };

    for size in [
        SchematicPageSize::A4,
        SchematicPageSize::A3,
        SchematicPageSize::UsLetter,
        SchematicPageSize::UsLedger,
    ] {
        for orientation in [
            SchematicPageOrientation::Portrait,
            SchematicPageOrientation::Landscape,
        ] {
            let expected = SchematicSheetFormat::standard(size, orientation);
            let setup = setup_seeded_from_sheet_format(expected).unwrap();
            assert_eq!(
                setup.physical_page().orientation(),
                match orientation {
                    SchematicPageOrientation::Portrait => Orientation::Portrait,
                    SchematicPageOrientation::Landscape => Orientation::Landscape,
                }
            );
        }
    }

    let custom = SchematicSheetFormat::try_custom(
        "Review board",
        304_800,
        457_200,
        SchematicPageOrientation::Landscape,
    )
    .unwrap();
    let setup = setup_seeded_from_sheet_format(custom).unwrap();
    // Output on paper the size of the sheet is already trimmed, so it starts
    // without trimming marks; everything else the sheet draws stays on.
    let schematic = setup.schematic();
    assert!(!schematic.crop_marks());
    assert!(
        schematic.includes_paper()
            && schematic.includes_border()
            && schematic.includes_title_block()
            && schematic.includes_zones()
    );
    let PaperSize::Custom(paper) = setup.physical_page().paper() else {
        panic!("a custom authored sheet seeds custom output media");
    };
    assert_eq!(
        paper.dimensions(),
        (
            rspice_hardcopy_contract::Length::from_micrometres(304_800),
            rspice_hardcopy_contract::Length::from_micrometres(457_200),
        )
    );

    let default = HardcopySetup::default();
    let automatic = HardcopySetup::try_new(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(StandardPaper::A4),
            default.physical_page().margins(),
            default.physical_page().bleed(),
            Orientation::AutomaticPerPage,
        )
        .unwrap(),
        default.scale(),
        default.tiling(),
        default.render().clone(),
        default.decorations().clone(),
        default.print_mapping().clone(),
    )
    .unwrap();
    assert_eq!(
        automatic.physical_page().orientation(),
        Orientation::AutomaticPerPage,
        "automatic orientation remains an output-media choice and is never converted into authored-sheet state"
    );
}

#[test]
fn artifact_names_are_portable_and_nonempty() {
    assert_eq!(safe_filename("top / schematic"), "top-schematic");
    assert_eq!(safe_filename("***"), "rspice-hardcopy");
    assert_eq!(safe_filename("afe--out"), "afe-out");
}

#[test]
fn file_contract_does_not_misrepresent_print_targets() {
    assert_eq!(format_file_contract(OutputFormat::PdfA).unwrap().0, "pdf");
    assert!(format_file_contract(OutputFormat::NativePrinter).is_err());
    assert!(format_file_contract(OutputFormat::BrowserPrintDocument).is_err());
}

#[test]
fn identity_bands_are_exactly_one_line_each() {
    let (header, provenance) =
        identity_lines("P", "7", "top / schematic", "source:key", "3", "0123");
    assert!(!header.contains('\n'));
    assert!(!provenance.contains('\n'));
    assert!(header.contains("top / schematic"));
    assert!(provenance.contains("source:key"));
}

#[test]
fn selected_export_name_receives_the_contract_extension() {
    let mut path = std::path::PathBuf::from("review-output");
    crate::io::file_exchange::ensure_file_extension(&mut path, "pdf");
    assert_eq!(path, std::path::PathBuf::from("review-output.pdf"));
}

#[test]
fn default_printer_resolution_prefers_600_then_a_bounded_highest_mode() {
    assert_eq!(preferred_resolution(vec![300, 600, 1_200]), Some(600));
    assert_eq!(
        preferred_resolution(vec![300, 720, 1_200, 2_400]),
        Some(1_200)
    );
    assert_eq!(preferred_resolution(vec![2_400, 4_800]), Some(2_400));
    assert_eq!(preferred_resolution(Vec::new()), None);
}
