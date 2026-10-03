use super::*;

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
