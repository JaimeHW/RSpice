//! Narrow rendering-policy tests for the Page Setup dialog.
//!
//! The cases exercise responsive control stacking and primary-action gating
//! without constructing the full application shell.

use super::{elide_to_width, format_controls_stack, page_setup_primary_enabled};

#[test]
fn initial_governed_sheet_can_be_created_without_artificial_edits() {
    assert!(page_setup_primary_enabled(false, true, false, true));
    assert!(!page_setup_primary_enabled(false, true, false, false));
    assert!(!page_setup_primary_enabled(false, false, false, true));
}

/// A measurement cell narrower than its value used to spin forever, which
/// froze the whole application on the Scope & inheritance section.
#[test]
fn measurement_values_elide_instead_of_looping() {
    let width_of = |text: &str| text.chars().count() as f32 * 7.0;
    let value = "ISO A4 · landscape · 297 × 210 mm";
    for max_width in [0.0, 1.0, 12.0, 40.0, 90.0, 160.0] {
        let elided = elide_to_width(value, max_width, width_of);
        assert!(elided.ends_with('\u{2026}'), "{max_width} → {elided:?}");
        assert!(
            width_of(&elided) <= max_width.max(width_of("\u{2026}")),
            "{max_width} → {elided:?}"
        );
        assert!(value.starts_with(elided.trim_end_matches('\u{2026}')));
    }
    assert_eq!(elide_to_width(value, 4000.0, width_of), value);
}

#[test]
fn phone_format_controls_stack_before_touch_targets_can_overlap() {
    assert!(format_controls_stack(342.0));
    assert!(format_controls_stack(479.0));
    assert!(!format_controls_stack(480.0));
    assert!(!format_controls_stack(760.0));
}
