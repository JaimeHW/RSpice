//! ASCII RAW value budgets must apply before rows beyond the budget are retained.
use rspice_core::io::ltspice_raw::{RawParseError, parse_raw_plots_bytes_with_limits};
use rspice_core::{ResourceKind, ResourceLimits};

fn plot(complex: bool, row_oriented: bool, points: usize, reordered: bool) -> Vec<u8> {
    let flags = if complex { "complex" } else { "real" };
    let dimensions = if reordered {
        format!("No. Points: {points}\nNo. Variables: 2\nFlags: {flags}\n")
    } else {
        format!("Flags: {flags}\nNo. Variables: 2\nNo. Points: {points}\n")
    };
    let mut text = format!(
        "Title: admission\nPlotname: Transient Analysis\n{dimensions}Variables:\n0 time time\n1 V(out) voltage\nValues:\n"
    );
    for point in 0..2 {
        let (x, y) = if complex {
            (format!("{point},0"), format!("{},3", point + 1))
        } else {
            (point.to_string(), (point + 1).to_string())
        };
        if row_oriented {
            text.push_str(&format!("{point} {x} {y}\n"));
        } else {
            text.push_str(&format!("{point} {x}\n{y}\n"));
        }
    }
    text.into_bytes()
}

fn limits_for(resource: ResourceKind, limit: usize) -> ResourceLimits {
    let mut limits = ResourceLimits::default();
    match resource {
        ResourceKind::ExternalDataValues => limits.max_external_data_values = limit,
        ResourceKind::ResultValues => limits.max_result_values = limit,
        _ => unreachable!(),
    }
    limits
}

#[test]
fn inferred_rows_stop_at_the_budget_before_decoding_the_rest_of_the_file() {
    for complex in [false, true] {
        for row_oriented in [false, true] {
            let valid = plot(complex, row_oriented, 0, false);
            for (resource, per_point) in [
                (
                    ResourceKind::ExternalDataValues,
                    if complex { 4 } else { 2 },
                ),
                (ResourceKind::ResultValues, if complex { 6 } else { 4 }),
            ] {
                let exact = limits_for(resource, 2 * per_point);
                let parsed = parse_raw_plots_bytes_with_limits(&valid, exact).unwrap();
                assert_eq!(parsed.plots[0].header.no_points, 2);
                assert_eq!(parsed.plots[0].waveforms[1].y, [1.0, 2.0]);
                assert_eq!(
                    parsed.plots[0].waveforms[1].y_imag.as_deref(),
                    complex.then_some(&[3.0, 3.0][..])
                );

                // Reaching this invalid tail means the reader consumed past
                // the first excess point instead of stopping at admission.
                let mut oversized = valid.clone();
                oversized.extend([0xff, b'\n']);
                let error = parse_raw_plots_bytes_with_limits(
                    &oversized,
                    limits_for(resource, 2 * per_point - 1),
                )
                .unwrap_err();
                let RawParseError::ResourceLimit(error) = error else {
                    panic!("expected early {resource} admission, got {error}");
                };
                assert_eq!(error.resource, resource);
                assert_eq!(error.requested, 2 * per_point);
                assert_eq!(error.limit, 2 * per_point - 1);
            }
        }
    }
}

#[test]
fn reordered_header_dimensions_are_admitted_before_reading_ascii_rows() {
    for complex in [false, true] {
        for (resource, expected) in [
            (
                ResourceKind::ExternalDataValues,
                if complex { 8 } else { 4 },
            ),
            (ResourceKind::ResultValues, if complex { 12 } else { 8 }),
        ] {
            let mut source = plot(complex, true, 2, true);
            let start = source
                .windows(b"Values:\n".len())
                .position(|window| window == b"Values:\n")
                .unwrap()
                + b"Values:\n".len();
            source.truncate(start);
            source.extend([0xff, b'\n']);
            let error =
                parse_raw_plots_bytes_with_limits(&source, limits_for(resource, expected - 1))
                    .unwrap_err();
            let RawParseError::ResourceLimit(error) = error else {
                panic!("expected final header dimensions to be admitted, got {error}");
            };
            assert_eq!(error.resource, resource);
            assert_eq!(error.requested, expected);
            assert_eq!(error.limit, expected - 1);
        }
    }
}

#[test]
fn inferred_rows_report_whole_file_budget_usage_across_plots() {
    let mut source = plot(false, true, 0, false);
    source.extend(plot(false, false, 0, false));
    source.extend([0xff, b'\n']);
    for (resource, expected) in [
        (ResourceKind::ExternalDataValues, 8),
        (ResourceKind::ResultValues, 16),
    ] {
        let error = parse_raw_plots_bytes_with_limits(&source, limits_for(resource, expected - 1))
            .unwrap_err();
        let RawParseError::ResourceLimit(error) = error else {
            panic!("expected whole-file admission, got {error}");
        };
        assert_eq!(error.resource, resource);
        assert_eq!(error.requested, expected);
        assert_eq!(error.limit, expected - 1);
    }
}
