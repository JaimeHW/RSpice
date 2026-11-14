use super::*;

/// A held node's samples can differ in their last bit, and a fit to that span
/// fills the pane with rounding. A real ripple on the same level keeps its
/// span, and so does a small signal around zero.
#[test]
fn a_fit_reads_rounding_on_a_held_node_as_one_level() {
    let level = 0.999_995_000_024_839_f64;
    let next = f64::from_bits(level.to_bits() + 1);
    assert_eq!(
        pane_auto_y(Some((level, next)), &[], &[]),
        Some((level / 1.1, next * 1.1))
    );
    for span in [(level, level + 1.0e-6), (-1.0e-15, 1.0e-15)] {
        assert_eq!(pane_auto_y(Some(span), &[], &[]), Some(span));
    }
    assert_eq!(pane_auto_y(Some((0.0, 0.0)), &[], &[]), Some((-1.0, 1.0)));
    assert_eq!(
        pane_auto_y(Some((-5.0, -5.0)), &[], &[]),
        Some((-6.0, -4.0))
    );
}

#[test]
fn multi_pane_geometry_never_grows_past_its_strip_budget() {
    for available in [0.0, 18.0, 60.0, 140.0, 320.0] {
        for pane_count in 1..=8 {
            let geometry = wave_stack_geometry(available, pane_count);
            let seams = geometry.seam_height * pane_count.saturating_sub(1) as f32;
            let used = geometry.pane_height * pane_count as f32 + geometry.shared_x_height + seams;
            assert!((0.0..=WAVE_SHARED_X_HEIGHT).contains(&geometry.shared_x_height));
            assert!(used <= available.max(0.0) + 1.0e-3);
        }
    }
}

/// Every pane of a strip takes the same share of it, whatever its place: a
/// weighted first pane left the B pane of a B–H strip two tick labels.
#[test]
fn the_panes_of_a_strip_split_it_equally() {
    for pane_count in 2..=4 {
        let geometry = wave_stack_geometry(600.0, pane_count);
        let seams = geometry.seam_height * (pane_count - 1) as f32;
        let expected = (600.0 - geometry.shared_x_height - seams) / pane_count as f32;
        assert!(
            (geometry.pane_height - expected).abs() < 1.0e-3,
            "{pane_count} panes: {} of {expected}",
            geometry.pane_height
        );
    }
}
