use super::*;

/// A readout counts the reader's significant digits, drops the zeros that
/// only pad it out, and never puts an SI prefix on a level or a ratio.
#[test]
fn plain_readouts_count_significant_digits_without_a_prefix() {
    assert_eq!(plain_readout(-60.123_456_7, "dBc", 4), "-60.12 dBc");
    assert_eq!(plain_readout(-60.123_456_7, "dBc", 7), "-60.12346 dBc");
    assert_eq!(plain_readout(0.5, "dB", 7), "0.5 dB");
    assert_eq!(plain_readout(2.0, "", 7), "2");
}

/// Angles read at the chosen digits in the unit Angle display chose.
#[test]
fn angle_readouts_follow_the_angle_display_at_significant_digits() {
    use rspice_app_types::quantity::AngleDisplay;
    let degrees = QuantityPresentationPolicy {
        angle_display: AngleDisplay::Degrees,
        ..QuantityPresentationPolicy::default()
    };
    assert_eq!(
        angle_readout(&degrees, 45.678_f64.to_radians(), 4),
        "45.68 °"
    );
    assert_eq!(
        angle_readout(&degrees, 45.678_f64.to_radians(), 3),
        "45.7 °"
    );
    let radians = QuantityPresentationPolicy {
        angle_display: AngleDisplay::Radians,
        ..QuantityPresentationPolicy::default()
    };
    assert_eq!(angle_readout(&radians, 1.234_567, 4), "1.235 rad");
}

#[test]
fn stat_columns_are_disjoint_at_phone_panel_width() {
    use rspice_ui_kit::panels::PANEL_INSET;
    let width = 240.0;
    let (name, value) = stat_column_widths(width, PANEL_INSET);
    assert!(name > 0.0);
    assert!(value > name);
    // The two columns fill the panel inside the panel inset on both sides,
    // less the gap between them.
    assert!((name + value + 2.0 * PANEL_INSET + 8.0 - width).abs() < f32::EPSILON * width);
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
