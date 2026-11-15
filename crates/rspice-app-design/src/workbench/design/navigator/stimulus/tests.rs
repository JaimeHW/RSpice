//! What the library browser and the definition inspector are held to.
//!
//! Both docks are gated here because they are read together: a count in the
//! browser's footer and a count in the inspector's Adopters heading are the
//! same walk of the same design, and a test that only ever opened one of them
//! would not notice the two disagreeing.

use super::*;

/// The dock's own minimum, at the fit the workspace is held to.
pub(crate) const NAVIGATOR: egui::Vec2 = egui::vec2(228.0, 640.0);

/// Every row's three tracks stay positive at the dock's own minimum, so a name
/// is elided rather than painted over the mini or under the revision.
#[test]
fn every_row_track_survives_the_navigator_minimum() {
    let name_track = NAVIGATOR.x
        - row::INDENT
        - crate::properties::source_preview::MINI_SIZE.x
        - row::TEXT_GAP
        - row::RIGHT_INSET
        - row::DOT_TRACK;
    assert!(
        name_track >= 100.0,
        "a name row has only {name_track} points at the dock's minimum"
    );
    assert!(row::ROW_HEIGHT >= crate::properties::source_preview::MINI_SIZE.y + 8.0);
    assert!(SCOPE_BAND + FOOTER_BAND + row::ROW_HEIGHT * 2.0 < NAVIGATOR.y);
}
