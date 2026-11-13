//! Pairing two operands on one grid, windowing it on the sweep, and placing
//! the arrows that say which way the sweep travels.

use super::*;
use std::f64::consts::PI;

/// A Lissajous figure: sin(ωt) against sin(ωt + φ), both sampled on one
/// transient grid. Every plotted point is the closed form at its grid point,
/// and lies on the ellipse x² − 2xy·cos φ + y² = sin² φ.
#[test]
fn a_lissajous_figure_is_the_closed_form_at_every_grid_point() {
    let omega = 2.0 * PI * 1.0e3;
    let phi = PI / 3.0;
    let grid: Vec<f64> = (0..=1_000).map(|step| f64::from(step) * 1.0e-6).collect();
    let x: Vec<f64> = grid.iter().map(|t| (omega * t).sin()).collect();
    let y: Vec<f64> = grid.iter().map(|t| (omega * t + phi).sin()).collect();
    let curve = resample((&grid, &x), (&grid, &y), SampleInterpolation::Linear).unwrap();
    assert!(!curve.interpolated, "one grid pairs sample by sample");
    assert_eq!(curve.sweep, grid);
    for (k, &t) in grid.iter().enumerate() {
        assert_eq!(
            curve.x[k].to_bits(),
            (omega * t).sin().to_bits(),
            "x at t = {t}"
        );
        assert_eq!(
            curve.y[k].to_bits(),
            (omega * t + phi).sin().to_bits(),
            "y at t = {t}"
        );
        let (px, py) = (curve.x[k], curve.y[k]);
        let ellipse = px.mul_add(px, py.mul_add(py, -2.0 * px * py * phi.cos()));
        assert!(
            (ellipse - phi.sin().powi(2)).abs() < 1.0e-12,
            "({px}, {py}) at t = {t} is off the ellipse by {}",
            ellipse - phi.sin().powi(2)
        );
    }
}

/// Operands on different grids meet on every sample of either, inside the
/// stretch both cover, each read there by the chosen interpolation: a
/// sample of one operand is exact for it, and the other is interpolated.
#[test]
fn different_grids_meet_on_every_sample_of_either_inside_their_overlap() {
    let x_grid = [0.0, 1.0, 2.0, 3.0, 4.0];
    let x_values = [0.0, 10.0, 20.0, 30.0, 40.0];
    let y_grid = [0.5, 1.0, 2.5, 6.0];
    let y_values = [5.0, 6.0, 9.0, 16.0];
    let curve = resample(
        (&x_grid, &x_values),
        (&y_grid, &y_values),
        SampleInterpolation::Linear,
    )
    .unwrap();
    assert!(curve.interpolated);
    // 0 is before the second operand starts and 6 after the first ends; 1 is
    // a sample of both and appears once.
    assert_eq!(curve.sweep, [0.5, 1.0, 2.0, 2.5, 3.0, 4.0]);
    assert_eq!(curve.x, [5.0, 10.0, 20.0, 25.0, 30.0, 40.0]);
    assert_eq!(curve.y, [5.0, 6.0, 8.0, 9.0, 10.0, 12.0]);

    let nearest = resample(
        (&x_grid, &x_values),
        (&y_grid, &y_values),
        SampleInterpolation::Nearest,
    )
    .unwrap();
    assert_eq!(nearest.y, [5.0, 6.0, 9.0, 9.0, 9.0, 9.0]);
}

#[test]
fn a_sweep_that_turns_back_pairs_only_on_one_grid() {
    let loop_grid = [0.0, 1.0, 2.0, 1.0, 0.0];
    let up = [0.0, 1.0, 2.0, 3.0, 4.0];
    let curve = resample(
        (&loop_grid, &up),
        (&loop_grid, &up),
        SampleInterpolation::Linear,
    )
    .expect("one grid pairs by sample even when it turns back");
    assert!(!curve.ascending);
    assert_eq!(window(&curve, (0.5, 0.6)), 0..5, "the whole loop is drawn");

    let other = [0.0, 0.5, 1.0];
    assert_eq!(
        resample(
            (&loop_grid, &up),
            (&other, &other),
            SampleInterpolation::Linear
        ),
        Err(XyRefusal::TurnsBack)
    );
    assert_eq!(
        resample(
            (&[0.0, 1.0], &[0.0, 1.0]),
            (&[2.0, 3.0], &[0.0, 1.0]),
            SampleInterpolation::Linear
        ),
        Err(XyRefusal::Disjoint)
    );
}

/// A window on the sweep keeps exactly the points inside it.
#[test]
fn a_window_keeps_the_points_whose_sweep_lies_inside_it() {
    let grid: Vec<f64> = (0..10).map(f64::from).collect();
    let curve = resample((&grid, &grid), (&grid, &grid), SampleInterpolation::Linear).unwrap();
    assert_eq!(window(&curve, (2.5, 6.0)), 3..7);
    assert_eq!(window(&curve, (20.0, 30.0)), 10..10);
}

/// An arrow every 150 points of curve, the first half a spacing in, each
/// pointing along its segment; a short curve still gets one at its middle.
#[test]
fn arrows_are_sparse_and_point_the_way_the_curve_travels() {
    let line: Vec<egui::Pos2> = (0..=60)
        .map(|step| egui::pos2(step as f32 * 10.0, 50.0))
        .collect();
    let arrows = direction_arrows(&line);
    let at: Vec<f32> = arrows.iter().map(|(point, _)| point.x).collect();
    assert_eq!(at, [75.0, 225.0, 375.0, 525.0]);
    assert!(
        arrows
            .iter()
            .all(|(_, forward)| *forward == egui::vec2(1.0, 0.0))
    );

    let back: Vec<egui::Pos2> = line.iter().rev().copied().collect();
    assert!(
        direction_arrows(&back)
            .iter()
            .all(|(_, forward)| *forward == egui::vec2(-1.0, 0.0)),
        "the reverse sweep points the other way"
    );

    let short = [egui::pos2(0.0, 0.0), egui::pos2(0.0, 40.0)];
    let arrows = direction_arrows(&short);
    assert_eq!(arrows.len(), 1);
    assert_eq!(arrows[0].0, egui::pos2(0.0, 20.0));

    let broken = [
        egui::pos2(0.0, 0.0),
        egui::pos2(f32::NAN, 0.0),
        egui::pos2(10.0, 0.0),
    ];
    assert!(
        direction_arrows(&broken).is_empty(),
        "a gap is not travelled"
    );
}

/// A nested sweep names each member's columns with its point; a member's
/// curve reads the abscissa's column of that same point, and a column of
/// no nested sweep names none.
#[test]
fn a_nested_sweeps_member_reads_the_abscissa_of_its_own_point() {
    assert_eq!(
        member_abscissa_name("I(VD) [VG=2; point 2]", "V(D) [VG=1.5; point 1]").as_deref(),
        Some("V(D) [VG=2; point 2]")
    );
    assert_eq!(
        member_abscissa_name("I(VD) [VG=2; point 2]", "V(IN)").as_deref(),
        Some("V(IN) [VG=2; point 2]")
    );
    assert_eq!(
        member_abscissa_name("V(OUT)", "V(D) [VG=1.5; point 1]"),
        None
    );
    assert_eq!(member_abscissa_name("D(data[3])", "V(IN)"), None);
    // A pane drawn against one member's V(D) would read every member's
    // V(D) against itself.
    assert!(is_own_abscissa(
        "V(D) [VG=2; point 2]",
        "V(D) [VG=1.5; point 1]"
    ));
    assert!(is_own_abscissa("V(a)", "V(a)"));
    assert!(!is_own_abscissa(
        "I(VD) [VG=2; point 2]",
        "V(D) [VG=1.5; point 1]"
    ));
    assert!(!is_own_abscissa("V(b)", "V(a)"));
}

/// A Lissajous coordinate on its own ±1.08 V axis: half a period of a 1 kHz
/// sine is sin(π), whose last bit is 1.2e-16 V. Read at the axis'
/// millivolt resolution it is 0 V, not 122 aV, and every other coordinate
/// on that axis states the same millivolt digit.
#[test]
fn a_coordinate_reads_to_the_resolution_of_its_axis() {
    let axis = (-1.08, 1.08);
    let half_period = (2.0 * PI * 1.0e3 * 500.0e-6).sin();
    assert!(half_period != 0.0 && half_period.abs() < 1.0e-15);
    assert_eq!(at_resolution(half_period, axis, false, "V"), "0 V");
    assert_eq!(at_resolution(-half_period, axis, false, "V"), "0 V");
    assert_eq!(at_resolution(1.0, axis, false, "V"), "1 V");
    assert_eq!(at_resolution(0.5, axis, false, "V"), "500 mV");
    assert_eq!(
        at_resolution(-(3.0f64.sqrt()) / 2.0, axis, false, "V"),
        "-866 mV"
    );
    // Zoomed in tenfold, one more digit means something.
    assert_eq!(
        at_resolution(0.866_225_4, (0.8, 0.9), false, "V"),
        "866.2 mV"
    );
}
