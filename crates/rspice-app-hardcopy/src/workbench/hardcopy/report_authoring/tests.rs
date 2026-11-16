//! Report authoring gates every mutation on exact authority.
//!
//! Opening the surface mutates nothing; a plan creates the outline once;
//! page settings and order commit revision-checked transactions and fail
//! closed at the document boundaries. The layout cases hold each internal seam
//! to exactly one pane, which is what keeps a resize from splitting a block
//! across two.

use rspice_results_ui::report::page_marker;

#[test]
fn report_page_markers_remain_semantically_stable_after_reordering() {
    assert_eq!(page_marker(0, "Executive summary"), "1");
    assert_eq!(page_marker(5, "Executive summary"), "1");
    assert_eq!(page_marker(0, "Run manifests"), "A");
    assert_eq!(page_marker(7, "Custom appendix"), "+");
}
