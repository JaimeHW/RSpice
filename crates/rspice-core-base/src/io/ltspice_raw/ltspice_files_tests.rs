//! Rawfiles LTspice wrote itself: LTspice 17.2.4 for macOS in batch mode
//! (`LTspice -b deck.cir`) ran the decks beside them in
//! `tests/fixtures/ltspice`, one file per layout it writes. Every header is
//! UTF-16LE with no byte-order mark.
//!
//! Pinned values are the bits an independent reader decodes at the same
//! offsets (a float32 widened exactly, a double as stored), and the RC
//! circuits also meet their closed forms within LTspice's tolerances.

use std::f64::consts::PI;
use std::path::{Path, PathBuf};

use super::*;

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/ltspice")
        .join(name)
}

fn fixture_bytes(name: &str) -> Vec<u8> {
    std::fs::read(fixture_path(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn fixture(name: &str) -> RawWaveformData {
    parse_raw_file(&fixture_path(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn column<'a>(plot: &'a RawWaveformData, name: &str) -> &'a RawWaveform {
    plot.waveforms
        .iter()
        .find(|waveform| waveform.name == name)
        .unwrap_or_else(|| panic!("no column {name}"))
}

fn assert_bits(actual: f64, bits: u64, what: &str) {
    assert_eq!(
        actual.to_bits(),
        bits,
        "{what}: {actual:e} is not {:e}",
        f64::from_bits(bits)
    );
}

const RC_SECONDS: f64 = 1e-6;

/// An RC step response from 0 V to 1 V, the source rising in 1 ps.
fn assert_rc_charge(plot: &RawWaveformData) {
    let time = &column(plot, "time").y;
    let out = &column(plot, "V(out)").y;
    assert!(time.windows(2).all(|pair| pair[0] < pair[1]), "{time:?}");
    for (&t, &v) in time.iter().zip(out) {
        let expected = 1.0 - (-t / RC_SECONDS).exp();
        assert!(
            (v - expected).abs() < 1e-4,
            "V(out)({t:e}) = {v}, closed form {expected}"
        );
    }
}

#[test]
fn a_compressed_single_precision_transient_reads_with_its_compression_marks_cleared() {
    let plot = fixture("rc_tran.raw");
    assert_eq!(plot.header.title, "* rc transient");
    assert_eq!(plot.header.plotname, "Transient Analysis");
    assert_eq!(plot.header.flags, ["real", "forward"]);
    assert_eq!(plot.header.no_points, 23);
    let names: Vec<_> = plot.variables.iter().map(|v| v.name.as_str()).collect();
    assert_eq!(
        names,
        ["time", "V(in)", "V(out)", "I(C1)", "I(R1)", "I(V1)"]
    );
    // LTspice stores this time as -1.0677818370558842e-07: the sign bit marks
    // a point its compression kept.
    assert_bits(column(&plot, "time").y[2], 0x3e7c_a9bd_ac59_0887, "time[2]");
    assert!(
        plot.waveforms
            .iter()
            .all(|w| w.x[2] == column(&plot, "time").y[2])
    );
    assert_bits(
        column(&plot, "V(out)").y[22],
        0x3fef_c8cf_e000_0000,
        "V(out)[22]",
    );
    assert_rc_charge(&plot);
}

#[test]
fn a_double_transient_reads_every_value_as_a_double() {
    let plot = fixture("rc_tran_double.raw");
    assert_eq!(plot.header.flags, ["real", "double", "forward"]);
    assert_bits(column(&plot, "time").y[2], 0x3e7c_a9bd_ac59_0887, "time[2]");
    assert_bits(
        column(&plot, "V(out)").y[22],
        0x3fef_c8cf_d636_a622,
        "V(out)[22]",
    );
    assert_rc_charge(&plot);
}

#[test]
fn a_fast_access_transient_reads_column_by_column() {
    let plot = fixture("rc_fast.raw");
    assert_eq!(
        plot.header.flags,
        ["real", "forward", "nocompression", "fastaccess"]
    );
    assert_eq!(plot.header.no_points, 277);
    assert_bits(
        column(&plot, "time").y[100],
        0x3eb9_0ec5_7e42_153d,
        "time[100]",
    );
    assert_bits(
        column(&plot, "V(out)").y[100],
        0x3fe8_d034_8000_0000,
        "V(out)[100]",
    );
    assert_bits(
        column(&plot, "I(V1)").y[276],
        0xbedc_419e_a000_0000,
        "I(V1)[276]",
    );
    assert_rc_charge(&plot);
}

#[test]
fn a_decade_ac_sweep_reads_complex_doubles() {
    let plot = fixture("rc_ac.raw");
    assert_eq!(plot.header.flags, ["complex", "forward", "log"]);
    assert_eq!(plot.header.no_points, 121);
    let frequency = column(&plot, "frequency");
    assert_eq!(frequency.y.first(), Some(&1.0));
    assert_eq!(frequency.y.last(), Some(&1e6));
    assert!(frequency.y_imag.as_ref().unwrap().iter().all(|v| *v == 0.0));
    let out = column(&plot, "V(out)");
    let out_imag = out.y_imag.as_ref().unwrap();
    assert_bits(out.y[60], 0x3fef_ffad_3605_cbce, "Re V(out)[60]");
    assert_bits(out_imag[60], 0xbf79_bc23_2156_3524, "Im V(out)[60]");
    for ((&f, &re), &im) in frequency.y.iter().zip(&out.y).zip(out_imag) {
        // H = 1 / (1 + j w RC)
        let w_rc = 2.0 * PI * f * RC_SECONDS;
        let denominator = 1.0 + w_rc * w_rc;
        assert!((re - 1.0 / denominator).abs() < 1e-12, "Re H({f}) = {re}");
        assert!((im + w_rc / denominator).abs() < 1e-12, "Im H({f}) = {im}");
    }
}

#[test]
fn an_operating_point_reads_its_one_point() {
    let plot = fixture("rc_op.raw");
    assert_eq!(plot.header.plotname, "Operating Point");
    assert_eq!(plot.header.no_points, 1);
    assert!(plot.has_ordinal_axis().unwrap());
    assert_eq!(column(&plot, "V(in)").y, [5.0]);
    assert_eq!(column(&plot, "V(out)").y, [3.75]);
    assert_bits(column(&plot, "I(R1)").y[0], 0x3f54_7ae1_4000_0000, "I(R1)");
}

#[test]
fn a_noise_spectrum_reads_its_unicode_title_and_densities() {
    let plot = fixture("rc_noise.raw");
    assert_eq!(
        plot.header.plotname,
        "Noise Spectral Density - (V/Hz\u{bd} or A/Hz\u{bd})"
    );
    assert_eq!(plot.header.flags, ["real", "forward", "log"]);
    assert_eq!(plot.header.no_points, 61);
    let onoise = column(&plot, "V(onoise)");
    assert_bits(onoise.y[30], 0x3e31_7c6f_6000_0000, "V(onoise)[30]");
    // A 1 kOhm resistor's thermal noise at 27 C through the RC low-pass.
    let thermal = (4.0 * 1.380649e-23 * 300.15 * 1e3_f64).sqrt();
    for (&f, &density) in onoise.x.iter().zip(&onoise.y) {
        let w_rc = 2.0 * PI * f * RC_SECONDS;
        let expected = thermal / (1.0 + w_rc * w_rc).sqrt();
        assert!(
            ((density - expected) / expected).abs() < 1e-6,
            "onoise({f}) = {density:e}"
        );
    }
}

#[test]
fn a_dc_sweep_reads_its_sweep_source_as_the_scale() {
    let plot = fixture("rc_dc.raw");
    assert_eq!(plot.header.plotname, "DC transfer characteristic");
    assert_eq!(plot.variables[0].name, "v1");
    let out = column(&plot, "V(out)");
    assert_eq!(out.x.len(), 21);
    assert_eq!(out.x.last(), Some(&5.0));
    assert!(out.x.iter().zip(&out.y).all(|(v1, v)| *v == 0.75 * v1));
}

#[test]
fn a_stepped_transient_reads_every_step_in_one_plot() {
    let plot = fixture("rc_step.raw");
    assert_eq!(plot.header.flags, ["real", "forward", "stepped"]);
    assert_eq!(plot.header.no_points, 105);
    let time = &column(&plot, "time").y;
    let starts: Vec<_> = (0..time.len()).filter(|&i| time[i] == 0.0).collect();
    assert_eq!(starts, [0, 55, 86]);
    assert_bits(time[54], 0x3ed4_f8b5_88e3_68f0, "time[54]");
    assert_bits(
        column(&plot, "V(out)").y[54],
        0x3fef_c8dc_0000_0000,
        "V(out)[54]",
    );
    assert_bits(
        column(&plot, "V(out)").y[104],
        0x3fe6_d4fd_8000_0000,
        "V(out)[104]",
    );
}

#[test]
fn a_byte_order_mark_before_an_ltspice_header_reads_the_same_file() {
    let bytes = fixture_bytes("rc_op.raw");
    let marked = [&[0xFF, 0xFE][..], &bytes].concat();
    let plot = parse_raw_reader(&mut marked.as_slice()).unwrap();
    assert_eq!(column(&plot, "V(out)").y, [3.75]);
}

#[test]
fn an_ltspice_header_without_a_data_line_is_refused_with_its_last_line() {
    let bytes = fixture_bytes("rc_op.raw");
    let text: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect();
    let header = String::from_utf16_lossy(&text);
    let cut = header.find("Binary:").unwrap();
    let truncated: Vec<u8> = header[..cut]
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect();
    let error = parse_raw_reader(&mut truncated.as_slice()).unwrap_err();
    let message = error.to_string();
    assert!(
        matches!(error, RawParseError::InvalidHeader(_))
            && message.contains("line 15")
            && message.contains("no data section"),
        "{message}"
    );
}

#[test]
fn a_truncated_ltspice_data_block_is_refused_as_truncated() {
    let bytes = fixture_bytes("rc_tran.raw");
    let error = parse_raw_reader(&mut &bytes[..bytes.len() - 4]).unwrap_err();
    let message = error.to_string();
    assert!(
        message.contains("640 byte(s)") && message.contains("644") && message.contains("truncated"),
        "{message}"
    );
}

fn utf16le(text: &str) -> Vec<u8> {
    text.encode_utf16().flat_map(u16::to_le_bytes).collect()
}

const ASCII_TRANSIENT: &str = "Title: * ascii\nDate: today\nPlotname: Transient Analysis\n\
Flags: real forward\nNo. Variables: 2\nNo. Points: 3\nOffset: 0\n\
Command: Linear Technology Corporation LTspice\nVariables:\n\t0\ttime\ttime\n\
\t1\tV(out)\tvoltage\nValues:\n0\t\t0.000000000000000e+000\n\t1.000000e+000\n\n\
1\t\t-1.000000000000000e-006\n\t6.321206e-001\n\n2\t\t2.000000000000000e-006\n\t8.646647e-001\n\n";

#[test]
fn a_utf16_ascii_section_reads_as_its_utf8_twin() {
    // LTspice's ASCII rawfile (its `-ascii` switch) could not be produced in
    // headless batch mode, so this one is LTspice's layout spelled by hand:
    // the same text in both encodings must read to the same values.
    let twin = parse_raw_reader(&mut ASCII_TRANSIENT.as_bytes()).unwrap();
    let plot = parse_raw_reader(&mut utf16le(ASCII_TRANSIENT).as_slice()).unwrap();
    assert_eq!(column(&plot, "time").y, [0.0, 1e-6, 2e-6]);
    for (left, right) in plot.waveforms.iter().zip(&twin.waveforms) {
        assert_eq!(left.x, right.x);
        assert_eq!(left.y, right.y);
    }
    assert_eq!(column(&plot, "V(out)").y, [1.0, 0.6321206, 0.8646647]);
}

#[test]
fn utf16_plots_follow_one_another() {
    let file = [utf16le(ASCII_TRANSIENT), utf16le(ASCII_TRANSIENT)].concat();
    let plots = parse_raw_plots_bytes_with_limits(&file, ResourceLimits::default()).unwrap();
    assert_eq!(plots.plots.len(), 2);
    assert_eq!(column(&plots.plots[1], "time").y, [0.0, 1e-6, 2e-6]);
}

#[test]
fn a_utf8_byte_order_mark_is_not_part_of_the_title() {
    let marked = [&[0xEF, 0xBB, 0xBF][..], ASCII_TRANSIENT.as_bytes()].concat();
    let plot = parse_raw_reader(&mut marked.as_slice()).unwrap();
    assert_eq!(plot.header.title, "* ascii");
}

#[test]
fn an_unknown_informational_flag_is_ignored_but_unpadded_is_refused() {
    let informational = ASCII_TRANSIENT.replace("real forward", "real forward sparkly");
    assert!(parse_raw_reader(&mut informational.as_bytes()).is_ok());
    let unpadded = ASCII_TRANSIENT.replace("real forward", "real unpadded");
    let error = parse_raw_reader(&mut unpadded.as_bytes()).unwrap_err();
    let message = error.to_string();
    assert!(
        matches!(error, RawParseError::UnsupportedFormat(_))
            && message.contains("line 4")
            && message.contains("'unpadded'"),
        "{message}"
    );
}

#[test]
fn a_non_transient_negative_scale_keeps_its_sign() {
    let dc = ASCII_TRANSIENT
        .replace("Transient Analysis", "DC transfer characteristic")
        .replace("\ttime\ttime", "\tv1\tvoltage");
    let plot = parse_raw_reader(&mut dc.as_bytes()).unwrap();
    assert_eq!(column(&plot, "v1").y, [0.0, -1e-6, 2e-6]);
}
