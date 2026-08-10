//! Drawing-sheet numbering and release-control compatibility tests.
//!
//! These cases keep catalog page authority and strict calendar validation
//! together without growing the broader design-management test module.

use super::*;

#[test]
fn sheet_page_number_authority_honors_stable_and_per_print_set_modes() {
    let mut catalog = SheetCatalog::default();
    let first = catalog.create_sheet(sheet("Input", 7), None).unwrap();
    let second = catalog
        .create_sheet(sheet("Output", 9), Some(first))
        .unwrap();

    assert_eq!(catalog.page_number_and_count(first), Some((7, 2)));
    assert_eq!(catalog.page_number_and_count(second), Some((9, 2)));

    let mut settings = catalog.settings().clone();
    settings.page_numbering = SheetPageNumbering::PerPrintSet;
    catalog.set_settings(catalog.revision(), settings).unwrap();

    assert_eq!(catalog.page_number_and_count(first), Some((1, 2)));
    assert_eq!(catalog.page_number_and_count(second), Some((2, 2)));
}

#[test]
fn drawing_sheet_calendar_conversion_uses_exact_iso_format() {
    assert_eq!(civil_date_from_unix_days(0), "1970-01-01");
    assert_eq!(civil_date_from_unix_days(20_663), "2026-07-29");
}

#[test]
fn drawing_sheet_document_control_is_backward_compatible_and_calendar_strict() {
    let mut legacy = serde_json::to_value(DrawingSheetProjectSettings::default()).unwrap();
    legacy.as_object_mut().unwrap().remove("document_control");
    let restored: DrawingSheetProjectSettings = serde_json::from_value(legacy).unwrap();
    assert_eq!(
        restored.document_control,
        DrawingSheetDocumentControl::default()
    );
    assert_eq!(restored.document_control.display_date(), "UNRELEASED");

    let invalid_date = DrawingSheetDocumentControl {
        revision: "A".to_owned(),
        revision_date_utc: "2026-02-30".to_owned(),
        status: DrawingSheetReleaseStatus::InReview,
        change_reference: "ECO-1042".to_owned(),
    };
    assert!(matches!(
        invalid_date.validate(),
        Err(DesignManagementError::InvalidText {
            field: "drawing sheet document revision date",
            ..
        })
    ));

    let released_draft = DrawingSheetDocumentControl {
        revision_date_utc: "2026-08-04".to_owned(),
        status: DrawingSheetReleaseStatus::Released,
        ..DrawingSheetDocumentControl::default()
    };
    assert!(matches!(
        released_draft.validate(),
        Err(DesignManagementError::InvalidText {
            field: "released drawing sheet document revision",
            ..
        })
    ));

    let released = DrawingSheetDocumentControl {
        revision: "B".to_owned(),
        revision_date_utc: "2026-08-04".to_owned(),
        status: DrawingSheetReleaseStatus::Released,
        change_reference: "ECO-1042".to_owned(),
    };
    released.validate().unwrap();
    assert_eq!(released.display_date(), "2026-08-04");
}
