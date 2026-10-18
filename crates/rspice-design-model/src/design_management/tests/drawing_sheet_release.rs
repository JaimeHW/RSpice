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
    // An unreleased document has no revision index and no date yet; its
    // Document status says where it stands.
    assert_eq!(restored.document_control.revision_index(), "");
    assert_eq!(restored.document_control.revision_date_utc, "");
    restored.document_control.validate().unwrap();
    // A project saved when the index held the word DRAFT letters no index.
    let placeholder = DrawingSheetDocumentControl {
        revision: "DRAFT".to_owned(),
        ..DrawingSheetDocumentControl::default()
    };
    placeholder.validate().unwrap();
    assert_eq!(placeholder.revision_index(), "");

    let invalid_date = DrawingSheetDocumentControl {
        revision: "A".to_owned(),
        revision_date_utc: "2026-02-30".to_owned(),
        status: DrawingSheetReleaseStatus::InReview,
        change_reference: "ECO-1042".to_owned(),
    };
    assert_eq!(
        invalid_date.validate().unwrap_err().to_string(),
        "The date must be a calendar date written YYYY-MM-DD."
    );

    let released_draft = DrawingSheetDocumentControl {
        revision_date_utc: "2026-08-04".to_owned(),
        status: DrawingSheetReleaseStatus::Released,
        ..DrawingSheetDocumentControl::default()
    };
    assert_eq!(
        released_draft.validate().unwrap_err().to_string(),
        "A released or obsolete document needs a revision."
    );
    let undated = DrawingSheetDocumentControl {
        revision: "B".to_owned(),
        status: DrawingSheetReleaseStatus::Obsolete,
        ..DrawingSheetDocumentControl::default()
    };
    assert_eq!(
        undated.validate().unwrap_err().to_string(),
        "A released or obsolete document needs a date."
    );

    let released = DrawingSheetDocumentControl {
        revision: "B".to_owned(),
        revision_date_utc: "2026-08-04".to_owned(),
        status: DrawingSheetReleaseStatus::Released,
        change_reference: "ECO-1042".to_owned(),
    };
    released.validate().unwrap();
    assert_eq!(released.revision_index(), "B");

    // A released document must name its index, and the old placeholder is
    // not one.
    for revision in ["", "DRAFT"] {
        let unindexed = DrawingSheetDocumentControl {
            revision: revision.to_owned(),
            ..released.clone()
        };
        assert!(matches!(
            unindexed.validate(),
            Err(DesignManagementError::DocumentControl(message)) if message.contains("revision")
        ));
    }
}
