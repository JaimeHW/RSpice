//! Tests for the publish dialog's output contract.
//!
//! The cases pin that the dialog never misrepresents a print target, and that
//! the mapping stage stays transactional until publication actually commits.

use super::*;

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
