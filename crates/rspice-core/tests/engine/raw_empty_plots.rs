//! Empty plots must not consume the following plot's header as samples.
use rspice_core::io::ltspice_raw::parse_raw_plots_bytes_with_limits;

fn header(flags: &str, points: usize, binary: bool) -> Vec<u8> {
    let marker = if binary { "Binary" } else { "Values" };
    format!("Title: empty and inferred\nPlotname: Transient Analysis\nFlags: {flags}\nNo. Variables: 2\nNo. Points: {points}\nVariables:\n0 time time\n1 V(out) voltage\n{marker}:\n").into_bytes()
}

#[test]
fn empty_plots_preserve_the_following_empty_and_populated_plots() {
    for binary in [false, true] {
        for flags in ["real", "real double", "complex", "complex double"] {
            let mut source = header(flags, 0, binary);
            source.extend(header(flags, 0, binary));
            source.extend(header("real double", 1, true));
            source.extend(0.0_f64.to_le_bytes());
            source.extend(2.0_f64.to_le_bytes());
            let file = parse_raw_plots_bytes_with_limits(&source, Default::default()).unwrap();
            assert_eq!(file.plots.len(), 3, "{flags}");
            for plot in &file.plots[..2] {
                assert_eq!(plot.header.no_points, 0);
                assert_eq!(plot.waveforms.len(), 2);
                for waveform in &plot.waveforms {
                    assert!(waveform.x.is_empty());
                    assert!(waveform.y.is_empty());
                    assert_eq!(
                        waveform.y_imag.as_deref(),
                        flags.starts_with("complex").then_some(&[][..])
                    );
                }
            }
            assert_eq!(file.plots[2].waveforms[1].y, [2.0]);
            // A truncated subsequent header remains an error, never an empty tail.
            let mut malformed = header(flags, 0, binary);
            malformed.extend(b"Title: incomplete\n");
            assert!(parse_raw_plots_bytes_with_limits(&malformed, Default::default()).is_err());
        }
    }
}

#[test]
fn zero_point_headers_still_infer_legacy_binary_data_when_no_header_follows() {
    for mixed in [false, true] {
        let mut source = header(if mixed { "real" } else { "real double" }, 0, true);
        for (time, value) in [(0.0_f64, 1.0_f64), (1.0, 2.0)] {
            source.extend(time.to_le_bytes());
            if mixed {
                source.extend((value as f32).to_le_bytes());
            } else {
                source.extend(value.to_le_bytes());
            }
        }
        let file = parse_raw_plots_bytes_with_limits(&source, Default::default()).unwrap();
        assert_eq!(file.plots.len(), 1);
        assert_eq!(file.plots[0].header.no_points, 2);
        assert_eq!(file.plots[0].waveforms[1].y, [1.0, 2.0]);
    }
}

#[test]
fn ascii_size_inference_stops_at_the_next_plot_header() {
    for values in ["0 0 1\n1 1 2\n", "0 0\n1\n1 1\n2\n"] {
        let mut source = header("real", 0, false);
        source.extend(values.as_bytes());
        source.extend(header("real double", 1, true));
        source.extend(2.0_f64.to_le_bytes());
        source.extend(3.0_f64.to_le_bytes());
        let file = parse_raw_plots_bytes_with_limits(&source, Default::default()).unwrap();
        assert_eq!(file.plots.len(), 2);
        assert_eq!(file.plots[0].waveforms[1].y, [1.0, 2.0]);
        assert_eq!(file.plots[1].waveforms[1].y, [3.0]);
    }
}

#[test]
fn inferred_ascii_plots_accept_long_blank_tails_and_preserve_following_headers() {
    let blanks = " \t\r\n".repeat(32_768);
    for values in ["", "0 0 1\n1 1 2\n", "0 0\n1\n1 1\n2\n"] {
        for following_plot in [false, true] {
            let mut source = header("real", 0, false);
            source.extend(values.as_bytes());
            source.extend(blanks.as_bytes());
            if following_plot {
                let next = String::from_utf8(header("real", 1, false)).unwrap();
                source.extend(next.replacen("Title:", "tItLe:", 1).as_bytes());
                source.extend(b"0 2 3\n");
            }
            let file = parse_raw_plots_bytes_with_limits(&source, Default::default()).unwrap();
            assert_eq!(file.plots.len(), if following_plot { 2 } else { 1 });
            assert_eq!(
                file.plots[0].waveforms[1].y,
                if values.is_empty() {
                    &[][..]
                } else {
                    &[1.0, 2.0][..]
                }
            );
            if following_plot {
                assert_eq!(file.plots[1].waveforms[1].y, [3.0]);
            }
        }
    }
}

#[test]
fn value_budget_counts_include_complex_columns_removed_by_table_metadata() {
    let source = b"Title: mixed table\nPlotname: AC Analysis\nFlags: complex\nCommand: RSpiceTableV4 {\"real_variables\":[0],\"units\":[null,null],\"layout\":\"coordinate-first\"}\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 frequency frequency\n1 V(out) voltage\nValues:\n0 1,0 2,3\n";
    let file = parse_raw_plots_bytes_with_limits(source, Default::default()).unwrap();
    let plot = &file.plots[0];
    assert!(plot.waveforms[0].y_imag.is_none());
    assert_eq!(plot.numeric_value_counts(), (4, 6));
}
