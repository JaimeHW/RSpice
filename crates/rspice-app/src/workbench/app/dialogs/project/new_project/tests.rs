//! What the New project transaction is allowed to create.
//!
//! The mirror is the interesting case: it exists so the common path needs one
//! field, and it must stop the instant the reader disagrees with it, because a
//! library name that silently reverts is worse than one that was never
//! suggested.

use super::*;

#[test]
fn the_slug_collapses_runs_and_never_yields_an_invalid_segment() {
    assert_eq!(library_slug("Sensor  bridge -- v2"), "sensor_bridge_v2");
    assert_eq!(
        library_slug("__leading and trailing__"),
        "leading_and_trailing"
    );
    assert_eq!(library_slug("///"), DEFAULT_PROJECT_LIBRARY);
    assert_eq!(library_slug(""), DEFAULT_PROJECT_LIBRARY);
    for name in ["Sensor  bridge -- v2", "__x__", "///", "\u{c9}tage"] {
        let slug = library_slug(name);
        assert!(
            crate::state::workspace::validate_cell_view_name_segment(&slug).is_ok(),
            "{name} -> {slug}"
        );
    }
}
