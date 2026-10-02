use super::*;

#[test]
fn stat_columns_are_disjoint_at_phone_panel_width() {
    let width = 240.0;
    let (name, value) = stat_column_widths(width);
    assert!(name > 0.0);
    assert!(value > name);
    assert!((name + value + 24.0 + 8.0 - width).abs() < f32::EPSILON * width);
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
