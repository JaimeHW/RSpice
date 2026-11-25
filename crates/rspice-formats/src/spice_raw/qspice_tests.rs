use super::*;

fn limits() -> rspice_core::ResourceLimits {
    rspice_core::ResourceLimits::default()
}

/// A header as QSPICE writes it: its own `Abscissa:`, `.param` and `.alias` lines among the
/// usual ones, the point count padded for QSPICE to rewrite, and tab-separated variables.
fn header(plot: &str, flags: &str, points: usize, variables: &[(&str, &str)]) -> Vec<u8> {
    let mut text = Vec::new();
    text.extend_from_slice(b"Title: * filter \xb5 test\n");
    text.extend_from_slice(b"Date: Tue May  5 22:18:29 2026\n");
    text.extend_from_slice(format!("Plotname: {plot}\n").as_bytes());
    text.extend_from_slice(b"Plot Suggestion(s): .OpenLoopGain.\n");
    text.extend_from_slice(format!("Flags: {flags}\n").as_bytes());
    text.extend_from_slice(b"Abscissa:     0.000000000000000e+00     1.000000000000000e-03\n");
    text.extend_from_slice(format!("No. Variables: {}\n", variables.len()).as_bytes());
    text.extend_from_slice(format!("No. Points: {points:<21}\n").as_bytes());
    text.extend_from_slice(b"Command: QSPICE64, Build Apr 18 2026 11:48:13\n");
    text.extend_from_slice(b".param temp=27\n");
    text.extend_from_slice(b".alias I(R1) (0.001mho*V(in,out))\n");
    text.extend_from_slice(b"Variables:\n");
    for (index, (name, kind)) in variables.iter().enumerate() {
        text.extend_from_slice(format!("\t{index}\t{name}\t{kind}\n").as_bytes());
    }
    text
}

fn doubles(values: &[f64]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

fn signal<'a>(plot: &'a DecodedRawPlot, name: &str) -> &'a crate::numeric::DecodedNumericSignal {
    plot.dataset
        .signals
        .iter()
        .find(|signal| signal.name == name)
        .unwrap_or_else(|| panic!("no {name}"))
}

#[test]
fn a_binary_transient_reads_every_column_in_double_precision() {
    let mut bytes = header(
        "Transient Analysis",
        "real",
        3,
        &[
            ("Time", "time"),
            ("V(out)", "voltage"),
            ("I(C1)", "current"),
        ],
    );
    bytes.extend_from_slice(b"Binary:\n");
    bytes.extend_from_slice(&doubles(&[
        0.0,
        0.0,
        1.0e-3, //
        1.0e-6,
        0.1234567890123,
        9.0e-4, //
        2.0e-6,
        0.2345678901234,
        8.1e-4,
    ]));
    let plots = decode_qspice_raw_plots(&bytes, limits()).unwrap();
    let [plot] = plots.as_slice() else {
        panic!("one plot");
    };
    assert_eq!(plot.plot_name, "Transient Analysis");
    assert_eq!(plot.dataset.coordinate_name, "Time");
    assert_eq!(plot.dataset.coordinate, [0.0, 1.0e-6, 2.0e-6]);
    assert_eq!(
        signal(plot, "V(out)").real,
        [0.0, 0.1234567890123, 0.2345678901234]
    );
    assert_eq!(signal(plot, "V(out)").unit.as_deref(), Some("V"));
    assert_eq!(signal(plot, "I(C1)").real, [1.0e-3, 9.0e-4, 8.1e-4]);
}

#[test]
fn a_binary_ac_plot_reads_its_real_frequency_before_complex_values() {
    let mut bytes = header(
        "AC Analysis",
        "complex",
        2,
        &[("Frequency", "frequency"), ("OpenLoopGain", "voltage")],
    );
    bytes.extend_from_slice(b"Binary:\n");
    bytes.extend_from_slice(&doubles(&[100.0, 2.5, -0.5, 200.0, 1.5, -1.25]));
    let plots = decode_qspice_raw_plots(&bytes, limits()).unwrap();
    let plot = &plots[0];
    assert_eq!(plot.dataset.domain, crate::WaveformDomain::Ac);
    assert_eq!(plot.dataset.coordinate, [100.0, 200.0]);
    let gain = signal(plot, "OpenLoopGain");
    assert_eq!(gain.real, [2.5, 1.5]);
    assert_eq!(gain.imag.as_deref(), Some(&[-0.5, -1.25][..]));
}

#[test]
fn a_stepped_plot_reads_one_run_per_step_named_by_its_parameter_columns() {
    let mut bytes = header(
        "Transient Analysis",
        "real stepped",
        5,
        &[
            ("Time", "time"),
            ("V(out)", "voltage"),
            ("VIN", "parameter"),
            ("R1", "parameter"),
        ],
    );
    bytes.extend_from_slice(b"Binary:\n");
    bytes.extend_from_slice(&doubles(&[
        0.0, 0.0, 1.0, 1000.0, //
        1.0e-3, 0.5, 1.0, 1000.0, //
        0.0, 0.0, 10.0, 1000.0, //
        1.0e-3, 5.0, 10.0, 1000.0, //
        2.0e-3, 7.5, 10.0, 1000.0,
    ]));
    let plots = decode_qspice_raw_plots(&bytes, limits()).unwrap();
    assert_eq!(plots.len(), 2);
    let steps = plots
        .iter()
        .map(|plot| plot.step.clone().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(steps[0].index, 0);
    assert_eq!(
        steps[0].coordinate,
        [("VIN".to_owned(), 1.0), ("R1".to_owned(), 1000.0)]
    );
    assert_eq!(
        steps[1].coordinate,
        [("VIN".to_owned(), 10.0), ("R1".to_owned(), 1000.0)]
    );
    assert_eq!(plots[1].dataset.coordinate, [0.0, 1.0e-3, 2.0e-3]);
    let names = plots[1]
        .dataset
        .signals
        .iter()
        .map(|signal| signal.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(names, ["V(out)"]);
    assert_eq!(signal(&plots[1], "V(out)").real, [0.0, 5.0, 7.5]);
}

#[test]
fn ascii_plots_read_including_a_complex_plot_whose_frequency_is_written_real() {
    let mut bytes = header(
        "AC Analysis",
        "complex",
        2,
        &[("Frequency", "frequency"), ("V(out)", "voltage")],
    );
    bytes.extend_from_slice(b"Values:\n");
    bytes.extend_from_slice(
        b"0\t\t1.000000000000000e+00\n\t9.999605231408785e-01,-6.282937266758373e-03\n\
          1\t\t1.258925411794167e+00\n\t9.999374348393328e-01,-7.909566755942080e-03\n",
    );
    let plots = decode_qspice_raw_plots(&bytes, limits()).unwrap();
    let plot = &plots[0];
    assert_eq!(plot.dataset.coordinate, [1.0, 1.258925411794167]);
    let output = signal(plot, "V(out)");
    assert_eq!(output.real, [0.9999605231408785, 0.9999374348393328]);
    assert_eq!(
        output.imag.as_deref(),
        Some(&[-6.282937266758373e-3, -7.90956675594208e-3][..])
    );

    let mut bytes = header(
        "DC Transfer Characteristic",
        "real",
        2,
        &[("V1", "voltage"), ("I(V1)", "current")],
    );
    bytes.extend_from_slice(b"Values:\n");
    bytes.extend_from_slice(
        b"0\t\t0.000000000000000e+00\n\t0.000000000000000e+00\n\
          1\t\t1.000000000000000e+00\n\t-1.000000000000000e-03\n",
    );
    let plots = decode_qspice_raw_plots(&bytes, limits()).unwrap();
    assert_eq!(plots[0].dataset.coordinate_name, "V1");
    assert_eq!(signal(&plots[0], "I(V1)").real, [0.0, -1.0e-3]);
}

#[test]
fn a_file_cut_short_is_refused_in_plain_words() {
    let mut bytes = header(
        "Transient Analysis",
        "real",
        3,
        &[("Time", "time"), ("V(out)", "voltage")],
    );
    bytes.extend_from_slice(b"Binary:\n");
    bytes.extend_from_slice(&doubles(&[0.0, 0.0, 1.0e-6, 0.5]));
    let error = decode_qspice_raw_plots(&bytes, limits()).unwrap_err();
    assert_eq!(
        error.to_string(),
        "the values are cut short: 3 points of 2 variables need 48 bytes, and the file holds 32"
    );
}

#[test]
fn a_file_in_a_rawfile_layout_under_the_qraw_name_reads_as_written() {
    // Single-precision columns beside a double scale, as LTspice writes them.
    let mut bytes = header(
        "Transient Analysis",
        "real",
        2,
        &[("Time", "time"), ("V(out)", "voltage")],
    );
    bytes.extend_from_slice(b"Binary:\n");
    for (time, value) in [(0.0_f64, 0.5_f32), (1.0e-6, 0.75)] {
        bytes.extend_from_slice(&time.to_le_bytes());
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    let plots = decode_qspice_raw_plots(&bytes, limits()).unwrap();
    assert_eq!(plots[0].dataset.coordinate, [0.0, 1.0e-6]);
    assert_eq!(signal(&plots[0], "V(out)").real, [0.5, 0.75]);

    // Column by column, the frequency written once.
    let mut bytes = header(
        "AC Analysis",
        "complex fastaccess",
        2,
        &[("Frequency", "frequency"), ("V(out)", "voltage")],
    );
    bytes.extend_from_slice(b"Binary:\n");
    bytes.extend_from_slice(&doubles(&[10.0, 100.0, 1.0, -2.0, 3.0, -4.0]));
    let plots = decode_qspice_raw_plots(&bytes, limits()).unwrap();
    assert_eq!(plots[0].dataset.coordinate, [10.0, 100.0]);
    let output = signal(&plots[0], "V(out)");
    assert_eq!(output.real, [1.0, 3.0]);
    assert_eq!(output.imag.as_deref(), Some(&[-2.0, -4.0][..]));
}
