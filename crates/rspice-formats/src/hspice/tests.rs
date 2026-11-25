use super::*;

/// The header as HSPICE lays it out: counts and version in the first 24 bytes, the title from
/// byte 24, the date from 88, then from byte 256 the column types, the column names (closing
/// parenthesis left off) and the swept parameter, closed by `$&%#`.
fn header(version: &str, kinds: &[u32], names: &[&str], sweep: Option<&str>) -> Vec<u8> {
    // Version 9601 sits in bytes 16 to 20; 2001 and 2013 sit in bytes 20 to 24.
    let version = match version {
        "2001" | "2013" => format!("0000{version}"),
        _ => format!("{version:<8}"),
    };
    let mut text = format!(
        "{:04}{:04}{:04}0000{version}",
        kinds.len(),
        0,
        usize::from(sweep.is_some())
    );
    text.push_str(&format!("{:<64}", "* rc circuit"));
    text.push_str(&format!("{:<24}", "06/05/2020      15:22:51"));
    text.push_str(" Copyright (c) 1986 - 2020 by Synopsys, Inc.");
    while text.len() < 256 {
        text.push(' ');
    }
    for kind in kinds {
        text.push_str(&format!("{kind:<8}"));
    }
    for name in names.iter().copied().chain(sweep) {
        text.push_str(&format!("{:<16}", name.trim_end_matches(')')));
    }
    text.push_str("$&%#");
    text.into_bytes()
}

fn record(body: &[u8], items: usize, big: bool) -> Vec<u8> {
    let word = |value: u32| {
        if big {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        }
    };
    let length = u32::try_from(body.len()).unwrap();
    let mut bytes = Vec::new();
    for value in [4, u32::try_from(items).unwrap(), 4, length] {
        bytes.extend_from_slice(&word(value));
    }
    bytes.extend_from_slice(body);
    bytes.extend_from_slice(&word(length));
    bytes
}

/// A binary file: the header record, then the values a few to a record. `scale_wide` marks
/// the values written in eight bytes when the rest are written in four.
fn binary(header: &[u8], values: &[(f64, bool)], double: bool, big: bool) -> Vec<u8> {
    let mut bytes = record(header, header.len(), big);
    for chunk in values.chunks(5) {
        let mut body = Vec::new();
        for &(value, wide) in chunk {
            match (double || wide, big) {
                (true, false) => body.extend_from_slice(&value.to_le_bytes()),
                (true, true) => body.extend_from_slice(&value.to_be_bytes()),
                (false, false) => body.extend_from_slice(&(value as f32).to_le_bytes()),
                (false, true) => body.extend_from_slice(&(value as f32).to_be_bytes()),
            }
        }
        bytes.extend_from_slice(&record(&body, chunk.len(), big));
    }
    bytes
}

fn narrow(values: &[f64]) -> Vec<(f64, bool)> {
    values.iter().map(|&value| (value, false)).collect()
}

/// An ASCII file: the header wrapped at 80 columns, then the values as HSPICE prints them,
/// `0.12345E+01` (9601) or `0.123456E+001` (2001), nothing between them, wrapped likewise.
fn ascii(header: &[u8], values: &[f64], digits: usize, exponent: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    for line in header.chunks(80) {
        bytes.extend_from_slice(line);
        bytes.push(b'\n');
    }
    let mut text = String::new();
    for &value in values {
        let (mantissa, power) = if value == 0.0 {
            (0.0, 0)
        } else {
            let power = value.abs().log10().floor() as i32 + 1;
            (value.abs() / 10f64.powi(power), power)
        };
        let digits_text = format!("{mantissa:.digits$}");
        let digits_text = if value < 0.0 {
            format!("-{}", &digits_text[1..])
        } else {
            digits_text
        };
        let sign = if power < 0 { '-' } else { '+' };
        text.push_str(&format!(
            "{digits_text}E{sign}{:0exponent$}",
            power.unsigned_abs()
        ));
    }
    for line in text.as_bytes().chunks(78) {
        bytes.extend_from_slice(line);
        bytes.push(b'\n');
    }
    bytes
}

fn signal<'a>(table: &'a DecodedHspiceTable, name: &str) -> &'a DecodedNumericSignal {
    table
        .dataset
        .signals
        .iter()
        .find(|signal| signal.name == name)
        .unwrap_or_else(|| panic!("no {name}"))
}

#[test]
fn a_binary_9601_transient_reads_its_names_units_and_single_precision_values() {
    let names = ["TIME", "v(0", "v(vo", "v(vs", "i(vs"];
    let head = header("9601", &[1, 1, 1, 1, 8], &names, None);
    let rows = [
        0.0, 0.0, 0.0, 0.0, 0.0, //
        1.0e-9, 0.0, 0.25, 1.0, -7.5e-4, //
        2.0e-9, 0.0, 0.5, 1.0, -5.0e-4, //
        1.0e30,
    ];
    for big in [false, true] {
        let decoded = decode_hspice(&binary(&head, &narrow(&rows), false, big)).unwrap();
        assert_eq!(decoded.title, "* rc circuit");
        let [table] = decoded.tables.as_slice() else {
            panic!("one table");
        };
        assert!(table.sweep.is_none());
        let dataset = &table.dataset;
        assert_eq!(dataset.domain, WaveformDomain::Transient);
        assert_eq!(dataset.coordinate_name, "TIME");
        assert_eq!(dataset.coordinate_unit.as_deref(), Some("s"));
        assert_eq!(
            dataset.coordinate,
            [0.0, f64::from(1.0e-9_f32), f64::from(2.0e-9_f32)]
        );
        let names = dataset
            .signals
            .iter()
            .map(|signal| signal.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, ["v(0)", "v(vo)", "v(vs)", "i(vs)"]);
        assert_eq!(signal(table, "v(vo)").real, [0.0, 0.25, 0.5]);
        assert_eq!(signal(table, "v(vo)").unit.as_deref(), Some("V"));
        let current = signal(table, "i(vs)");
        assert_eq!(current.unit.as_deref(), Some("A"));
        assert_eq!(current.real[1], f64::from(-7.5e-4_f32));
        assert!(current.imag.is_none());
    }
}

#[test]
fn a_binary_2001_ac_file_reads_voltages_and_currents_as_complex_and_decibels_as_real() {
    let head = header(
        "2001",
        &[2, 1, 8, 6],
        &["HERTZ", "v(out", "i(vin", "vdb(out"],
        None,
    );
    let rows = [
        1.0, 0.5, -0.25, -1.0e-3, 2.0e-3, -6.0, //
        10.0, 0.25, -0.5, -2.0e-3, 1.0e-3, -12.0, //
        1.0e30,
    ];
    let decoded = decode_hspice(&binary(&head, &narrow(&rows), true, false)).unwrap();
    let table = &decoded.tables[0];
    assert_eq!(table.dataset.domain, WaveformDomain::Ac);
    assert_eq!(table.dataset.coordinate_unit.as_deref(), Some("Hz"));
    assert_eq!(table.dataset.coordinate, [1.0, 10.0]);
    let voltage = signal(table, "v(out)");
    assert_eq!(voltage.real, [0.5, 0.25]);
    assert_eq!(voltage.imag.as_deref(), Some(&[-0.25, -0.5][..]));
    let current = signal(table, "i(vin)");
    assert_eq!(current.real, [-1.0e-3, -2.0e-3]);
    assert_eq!(current.imag.as_deref(), Some(&[2.0e-3, 1.0e-3][..]));
    let decibels = signal(table, "vdb(out)");
    assert_eq!(decibels.real, [-6.0, -12.0]);
    assert!(decibels.imag.is_none());
    assert!(decibels.unit.is_none());
}

#[test]
fn a_binary_2013_file_reads_a_double_scale_beside_single_columns() {
    let head = header("2013", &[1, 1, 1], &["TIME", "v(in", "v(out"], None);
    let time = 1.000_000_000_1e-9;
    let values = [
        (0.0, true),
        (1.8, false),
        (0.0, false),
        (time, true),
        (1.7, false),
        (0.1, false),
        (1.0e30, true),
    ];
    let decoded = decode_hspice(&binary(&head, &values, false, false)).unwrap();
    let table = &decoded.tables[0];
    assert_eq!(table.dataset.coordinate, [0.0, time]);
    assert_eq!(signal(table, "v(out)").real, [0.0, f64::from(0.1_f32)]);
}

#[test]
fn a_swept_file_reads_one_table_per_sweep_value() {
    let head = header("9601", &[3, 1], &["volt", "v(out"], Some("temp"));
    let rows = [
        25.0, 0.0, 0.0, 1.0, 0.5, 1.0e30, //
        85.0, 0.0, 0.0, 1.0, 0.4, 2.0, 0.8, 1.0e30,
    ];
    let decoded = decode_hspice(&binary(&head, &narrow(&rows), false, false)).unwrap();
    assert_eq!(decoded.tables.len(), 2);
    let (cold, hot) = (&decoded.tables[0], &decoded.tables[1]);
    assert_eq!(cold.sweep, Some(("temp".to_owned(), 25.0)));
    assert_eq!(hot.sweep, Some(("temp".to_owned(), 85.0)));
    assert_eq!(cold.dataset.domain, WaveformDomain::DcSweep);
    assert_eq!(cold.dataset.coordinate_name, "volt");
    assert!(cold.dataset.coordinate_unit.is_none());
    assert_eq!(cold.dataset.coordinate, [0.0, 1.0]);
    assert_eq!(hot.dataset.coordinate, [0.0, 1.0, 2.0]);
    assert_eq!(
        signal(hot, "v(out)").real,
        [0.0, f64::from(0.4_f32), f64::from(0.8_f32)]
    );
}

#[test]
fn ascii_files_read_in_both_widths_with_the_header_wrapped_at_eighty_columns() {
    let names = ["HERTZ", "v(1", "v(2", "i(v1"];
    let head = header("9601", &[2, 1, 1, 8], &names, None);
    let rows = [
        1.0,
        1.0,
        0.0,
        0.5,
        -3.1416e-11,
        -0.5,
        3.1416e-11, //
        1.0e9,
        1.0,
        0.0,
        0.0127,
        -0.0796,
        -0.4873,
        0.0796, //
        1.0e30,
    ];
    let decoded = decode_hspice(&ascii(&head, &rows, 5, 2)).unwrap();
    let table = &decoded.tables[0];
    assert_eq!(table.dataset.coordinate, [1.0, 1.0e9]);
    let output = signal(table, "v(2)");
    assert_eq!(output.real, [0.5, 0.0127]);
    assert_eq!(output.imag.as_deref(), Some(&[-3.1416e-11, -0.0796][..]));
    assert_eq!(signal(table, "i(v1)").real, [-0.5, -0.4873]);

    let head = header("2001", &[1, 1, 8], &["TIME", "v(vout", "i(vdd"], None);
    let rows = [0.0, 1.8, -1.8174e-2, 2.5e-11, 1.80063, -2.17145e-5, 1.0e30];
    let decoded = decode_hspice(&ascii(&head, &rows, 6, 3)).unwrap();
    let table = &decoded.tables[0];
    assert_eq!(table.dataset.coordinate, [0.0, 2.5e-11]);
    assert_eq!(signal(table, "v(vout)").real, [1.8, 1.80063]);
    assert_eq!(signal(table, "i(vdd)").real, [-1.8174e-2, -2.17145e-5]);
}

#[test]
fn damaged_files_are_refused_in_plain_words() {
    let head = header("9601", &[1, 1], &["TIME", "v(out"], None);
    let whole = binary(
        &head,
        &narrow(&[0.0, 1.0, 1.0e-9, 2.0, 1.0e30]),
        false,
        false,
    );
    let error = decode_hspice(&whole[..whole.len() - 6]).unwrap_err();
    assert_eq!(error.to_string(), "the file is cut short inside record 2");

    let open = binary(&head, &narrow(&[0.0, 1.0, 1.0e-9, 2.0]), false, false);
    let error = decode_hspice(&open).unwrap_err();
    assert_eq!(
        error.to_string(),
        "the file ends inside a table, before the value that closes it"
    );

    let unknown = binary(
        &header("1234", &[1, 1], &["TIME", "v(out"], None),
        &narrow(&[1.0e30]),
        false,
        false,
    );
    let error = decode_hspice(&unknown).unwrap_err();
    assert!(
        error
            .to_string()
            .starts_with("the header names output version '1234'"),
        "{error}"
    );

    let error = decode_hspice(b"TIME v(out)\n0 1\n").unwrap_err();
    assert!(
        error.to_string().contains("no `$&%#` closes a header"),
        "{error}"
    );
}

#[test]
fn measurement_tables_read_every_row_and_a_failed_measurement_as_unavailable() {
    let table = "$DATA1 SOURCE='HSPICE' VERSION='L-2016.06-SP1 win64' PARAM_COUNT=0\n\
.TITLE 'inverter'\n \
charge           power            tpdr             tpdf            \n \
temper           alter#          \n \
-1.826e-013       3.288e-013       3.464e-010       failed         \n  \
25.0000       1               \n \
-1.900e-013       3.300e-013       3.500e-010       3.853e-010     \n  \
25.0000       2               \n";
    let decoded = decode_hspice_measure(table.as_bytes()).unwrap();
    assert_eq!(decoded.title, "inverter");
    let dataset = &decoded.dataset;
    assert_eq!(dataset.coordinate_name, "point");
    assert_eq!(dataset.coordinate, [0.0, 1.0]);
    let names = dataset
        .signals
        .iter()
        .map(|signal| signal.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        ["charge", "power", "tpdr", "tpdf", "temper", "alter#"]
    );
    assert_eq!(dataset.signals[0].real, [-1.826e-13, -1.9e-13]);
    assert!(dataset.signals[3].real[0].is_nan());
    assert_eq!(dataset.signals[3].real[1], 3.853e-10);
    assert_eq!(dataset.signals[5].real, [1.0, 2.0]);

    let rf = "DATA1 SOURCE='HSPICERF' VERSION='A-2008.03'\r\n.TITLE '#folded cascode design'\r\n gain pm ugbw temper alter#\r\n  1.873349e+001\t  -9.236808e+001\t  3.796976e+008\t     25.000000 1\r\n";
    let decoded = decode_hspice_measure(rf.as_bytes()).unwrap();
    assert_eq!(decoded.title, "#folded cascode design");
    assert_eq!(decoded.dataset.signals[0].real, [18.73349]);
    assert_eq!(decoded.dataset.signals[2].real, [3.796976e8]);

    let error =
        decode_hspice_measure(b"$DATA1 SOURCE='HSPICE'\n.TITLE ''\n a b\n 1\n").unwrap_err();
    assert_eq!(
        error.to_string(),
        "the table names 2 measurements but holds 1 values, which is not a whole number of rows"
    );
}
