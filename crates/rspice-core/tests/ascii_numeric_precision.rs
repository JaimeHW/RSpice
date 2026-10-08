//! A finite decimal must not silently disappear while reading waveform data.
use rspice_core::io::{VcdValue, parse_raw_reader, parse_vcd_reader};
use std::io::Cursor;

fn raw(complex: bool, rows: bool, fields: [&str; 4]) -> String {
    let [coordinate, coordinate_imag, real, imag] = fields;
    let (flag, coordinate, signal, second) = if complex {
        (
            "complex",
            format!("{coordinate},{coordinate_imag}"),
            format!("{real},{imag}"),
            ["1,0", "2,3"],
        )
    } else {
        ("real", coordinate.to_owned(), real.to_owned(), ["1", "2"])
    };
    let separator = if rows { " " } else { "\n" };
    format!(
        "Title: precision\nPlotname: Transient Analysis\nFlags: {flag}\nNo. Variables: 2\nNo. Points: 2\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 {coordinate}{separator}{signal}\n1 {}{separator}{}\n",
        second[0], second[1]
    )
}

fn vcd(literal: &str) -> String {
    format!(
        "$timescale 1 ns $end\n$var real 1 ! V(out) $end\n$enddefinitions $end\n#0\nr{literal} !\n#1\nr2 !\n"
    )
}

#[test]
fn raw_ascii_coordinates_and_components_refuse_underflow_in_both_layouts() {
    for rows in [false, true] {
        for complex in [false, true] {
            for position in if complex {
                vec![0, 1, 2, 3]
            } else {
                vec![0, 2]
            } {
                for literal in ["1e-999", "-1e-999", "2e-324", "-2e-324"] {
                    let mut fields = ["0", "0", "1", "2"];
                    fields[position] = literal;
                    let error =
                        parse_raw_reader(&mut Cursor::new(raw(complex, rows, fields))).unwrap_err();
                    assert!(
                        error.to_string().contains("underflow"),
                        "{complex}/{rows}/{position}: {error}"
                    );
                    assert!(error.to_string().contains(literal), "{error}");
                }
            }
        }
    }
}

#[test]
fn raw_ascii_signed_zero_and_representable_subnormals_keep_every_bit() {
    for rows in [false, true] {
        for complex in [false, true] {
            for (literal, expected) in [
                ("0e999", 0.0_f64),
                ("-0e-999", -0.0_f64),
                ("5e-324", 5e-324_f64),
                ("-5e-324", -5e-324_f64),
            ] {
                let data =
                    parse_raw_reader(&mut Cursor::new(raw(complex, rows, [literal; 4]))).unwrap();
                for waveform in &data.waveforms {
                    assert_eq!(waveform.x[0].to_bits(), expected.to_bits());
                    assert_eq!(waveform.y[0].to_bits(), expected.to_bits());
                    if let Some(imaginary) = &waveform.y_imag {
                        assert_eq!(imaginary[0].to_bits(), expected.to_bits());
                    }
                }
            }
        }
    }
}

#[test]
fn vcd_real_changes_refuse_underflow_with_a_source_line() {
    for literal in ["1e-999", "-1e-999", "2e-324", "-2e-324"] {
        let error = parse_vcd_reader(vcd(literal).as_bytes()).unwrap_err();
        let message = error.to_string();
        assert!(
            message.contains("underflow") && message.contains("line 5"),
            "{message}"
        );
        assert!(message.contains(literal), "{message}");
    }
}

#[test]
fn vcd_real_signed_zero_and_representable_subnormals_keep_every_bit() {
    for (literal, expected) in [
        ("0e999", 0.0_f64),
        ("-0e-999", -0.0_f64),
        ("5e-324", 5e-324_f64),
        ("-5e-324", -5e-324_f64),
    ] {
        let data = parse_vcd_reader(vcd(literal).as_bytes()).unwrap();
        let VcdValue::Real(value) = data.signals[0].changes[0].value else {
            panic!("expected a real event")
        };
        assert_eq!(value.to_bits(), expected.to_bits());
    }
}
