//! Which project transitions authorize a destructive action.
//!
//! Browser copy-import and canonical save are distinct origins here because
//! conflating them overwrites the operator's file without a picker. The
//! remaining cases hold the reverse: only a verified canonical completion may
//! act immediately, and a live mirror snapshot waits out a local run rather
//! than landing on top of it.

use super::*;

#[test]
fn only_verified_canonical_completion_authorizes_an_immediate_destructive_action() {
    assert!(SaveRequestOutcome::CanonicalComplete.authorizes_immediate_destructive_action());
    assert!(
        !SaveRequestOutcome::CanonicalPending(crate::product::TransactionId::new())
            .authorizes_immediate_destructive_action()
    );
    assert!(!SaveRequestOutcome::CopyOnly.authorizes_immediate_destructive_action());
    assert!(!SaveRequestOutcome::CopyPending.authorizes_immediate_destructive_action());
    assert!(!SaveRequestOutcome::Cancelled.authorizes_immediate_destructive_action());
    assert!(
        !SaveRequestOutcome::Failed("disk unavailable".to_owned())
            .authorizes_immediate_destructive_action()
    );
}

#[test]
fn browser_canonical_origin_is_distinct_from_copy_import_semantics() {
    let canonical = ProjectLoadOrigin::BrowserCanonical("sensor-afe.rspiceproj");
    let imported = ProjectLoadOrigin::BrowserImport("sensor-afe.rspiceproj");

    assert_eq!(canonical.display_label(), "sensor-afe.rspiceproj");
    assert_eq!(canonical.success_prefix(), "Opened project");
    assert_eq!(imported.success_prefix(), "Imported project");
    assert!(canonical.recent_path().is_none());
}

#[test]
fn browser_project_copy_never_opens_a_save_picker_over_a_canonical_handle() {
    assert!(browser_save_picker_is_safe(true, None, true));
    assert!(!browser_save_picker_is_safe(true, Some(7), true));
    assert!(!browser_save_picker_is_safe(true, Some(7), false));
    assert!(browser_save_picker_is_safe(false, Some(7), true));
}

#[test]
fn browser_project_copy_never_changes_canonical_save_name() {
    let mut canonical_name = Some("source-project.rspiceproj".to_owned());
    accept_browser_canonical_display_name(&mut canonical_name, true, "independent-copy.rspiceproj");
    assert_eq!(canonical_name.as_deref(), Some("source-project.rspiceproj"));

    accept_browser_canonical_display_name(
        &mut canonical_name,
        false,
        "canonical-project.rspiceproj",
    );
    assert_eq!(
        canonical_name.as_deref(),
        Some("canonical-project.rspiceproj")
    );
}
