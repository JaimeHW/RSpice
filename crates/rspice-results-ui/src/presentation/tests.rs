use super::*;

/// Angles read in the unit Angle display chose: degrees to the hundredth,
/// against the number, at any digits preference; radians at the reader's
/// significant digits.
#[test]
fn angle_readouts_follow_the_angle_display() {
    use rspice_vocabulary::quantity::AngleDisplay;
    let degrees = QuantityPresentationPolicy {
        angle_display: AngleDisplay::Degrees,
        ..QuantityPresentationPolicy::default()
    };
    assert_eq!(
        angle_readout(&degrees, 45.678_f64.to_radians(), 4),
        "45.68\u{b0}"
    );
    assert_eq!(
        angle_readout(&degrees, 45.678_f64.to_radians(), 3),
        "45.68\u{b0}"
    );
    assert_eq!(
        angle_readout(&degrees, 45.0_f64.to_radians(), 4),
        "45.00\u{b0}"
    );
    assert_eq!(
        angle_readout(&degrees, (-0.001_f64).to_radians(), 4),
        "0.00\u{b0}"
    );
    let radians = QuantityPresentationPolicy {
        angle_display: AngleDisplay::Radians,
        ..QuantityPresentationPolicy::default()
    };
    assert_eq!(angle_readout(&radians, 1.234_567, 4), "1.235 rad");
    assert_eq!(angle_readout(&radians, 0.5, 4), "0.5000 rad");
}

/// A complex readout names its unit once; a polar one is its magnitude at
/// the reader's digits and its angle to the hundredth of a degree.
#[test]
fn complex_and_polar_readouts_name_one_unit() {
    let degrees = QuantityPresentationPolicy {
        angle_display: rspice_vocabulary::quantity::AngleDisplay::Degrees,
        ..QuantityPresentationPolicy::default()
    };
    assert_eq!(complex_readout(1.2, 3.4, "V", 4), "1.200 + 3.400j V");
    assert_eq!(
        polar_readout(&degrees, 1.2, "V", 45.0_f64.to_radians(), 4),
        "1.200 V \u{2220} 45.00\u{b0}"
    );
    assert_eq!(
        polar_readout(&degrees, 0.374, "", (-112.385_f64).to_radians(), 4),
        "0.3740 \u{2220} \u{2212}112.39\u{b0}"
    );
}

#[test]
fn wheel_zoom_keeps_smith_axes_at_equal_scale() {
    let change = square_xy_view_change(
        (-1.12, 1.12),
        (-1.12, 1.12),
        rspice_ui_kit::plot::ViewChange {
            x: Some((-0.5, 0.5)),
            ..Default::default()
        },
    );
    let x = change.x.expect("coupled x range");
    let y = change.y.expect("coupled y range");

    assert!(((x.1 - x.0) - (y.1 - y.0)).abs() < 1.0e-12);
    assert!((x.1 - x.0 - 1.0).abs() < 1.0e-12);
}

#[test]
fn box_zoom_expands_shorter_axis_instead_of_distorting_chart() {
    let change = square_xy_view_change(
        (-1.12, 1.12),
        (-1.12, 1.12),
        rspice_ui_kit::plot::ViewChange {
            x: Some((-0.75, 0.75)),
            y: Some((-0.25, 0.25)),
            ..Default::default()
        },
    );
    let x = change.x.expect("square x range");
    let y = change.y.expect("square y range");

    assert!(((x.1 - x.0) - (y.1 - y.0)).abs() < 1.0e-12);
    assert!((y.1 - y.0 - 1.5).abs() < 1.0e-12);
}

#[test]
fn fit_request_remains_a_reset() {
    let change = square_xy_view_change(
        (-0.5, 0.5),
        (-0.5, 0.5),
        rspice_ui_kit::plot::ViewChange {
            reset: true,
            ..Default::default()
        },
    );

    assert!(change.reset);
    assert!(change.x.is_none());
    assert!(change.y.is_none());
}

/// A fixed-place column prints every row at its one precision, so the
/// decimal points align, with a true minus sign and never a negative zero.
#[test]
fn a_fixed_place_figure_keeps_its_places_and_its_sign() {
    assert_eq!(fixed_places(41.0, SHARE_PLACES), "41.0");
    assert_eq!(fixed_places(28.24, SHARE_PLACES), "28.2");
    assert_eq!(fixed_places(-79.99566, DECIBEL_PLACES), "\u{2212}80.00");
    assert_eq!(fixed_places(-1.938_200_26, DECIBEL_PLACES), "\u{2212}1.94");
    assert_eq!(fixed_places(-0.004, DECIBEL_PLACES), "0.00");
}
