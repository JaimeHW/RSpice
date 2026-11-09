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
            let panes: f32 = (0..pane_count)
                .map(|ordinal| geometry.pane_height(ordinal, pane_count))
                .sum();
            let used = panes + geometry.shared_x_height + seams;
            assert!((0.0..=WAVE_SHARED_X_HEIGHT).contains(&geometry.shared_x_height));
            assert!(used <= available.max(0.0) + 1.0e-3);
        }
    }
}

#[test]
fn a_companion_pane_takes_two_thirds_of_the_primary() {
    // Single-pane sheets keep the whole stack; the mockup only weights a
    // pane down when it has a primary quantity to sit beneath.
    assert_eq!(pane_weight(0, 1), 1.0);
    assert_eq!(pane_weight(0, 2), 3.0);
    assert_eq!(pane_weight(1, 2), 2.0);
    assert_eq!(pane_weight(2, 3), 2.0);

    let geometry = wave_stack_geometry(600.0, 2);
    let primary = geometry.pane_height(0, 2);
    let companion = geometry.pane_height(1, 2);
    assert!(primary > companion);
    assert!((companion / primary - 2.0 / 3.0).abs() < 1.0e-4);
}
