use super::*;

fn source(coordinate: &str, real: &str, imaginary: &str, complex: bool) -> String {
    let samples = if complex {
        format!(r#""real":[{real},null],"imag":[{imaginary},null]"#)
    } else {
        format!(r#""values":[{real},null]"#)
    };
    format!(
        r#"{{"schema":"rspice-waveform-dataset/3","analysis":"transient","coordinate":{{"name":"time","unit":"s","values":[{coordinate},1.0]}},"signals":[{{"name":"V(out)","unit":"V",{samples}}}]}}"#
    )
}

#[test]
fn native_bundles_refuse_lossy_numeric_literals_in_every_sample_component() {
    for kind in [NativeBundleKind::Result, NativeBundleKind::Dataset] {
        for literal in [
            "1e-999",
            "-1e-999",
            "2e-324",
            "-2e-324",
            "9007199254740993",
            "-9007199254740993",
            "18446744073709551617",
            "-18446744073709551617",
        ] {
            for complex in [false, true] {
                for component in 0..if complex { 3 } else { 2 } {
                    let mut numbers = ["0.0", "1.0", "2.0"];
                    numbers[component] = literal;
                    let source = source(numbers[0], numbers[1], numbers[2], complex);
                    let error = decode_native_bundle(&pack_source(&source, kind), kind, limits())
                        .expect_err("a checked bundle must not silently alter numeric samples");
                    let expected = if literal.contains('e') {
                        "underflow"
                    } else {
                        "exact"
                    };
                    assert!(error.to_string().contains(expected), "{literal}: {error}");
                }
            }
        }
    }
}

#[test]
fn native_bundles_preserve_exact_floats_large_integers_and_missing_samples() {
    for kind in [NativeBundleKind::Result, NativeBundleKind::Dataset] {
        for literal in [
            "-0",
            "-0.0",
            "0e-999",
            "-0e-999",
            "5e-324",
            "-5e-324",
            "9007199254740992",
            "9007199254740994",
            "18446744073709551616",
            "-18446744073709551616",
            "9007199254740993.0",
        ] {
            let expected = literal.parse::<f64>().unwrap();
            for complex in [false, true] {
                let source = source(literal, literal, literal, complex);
                let decoded =
                    decode_native_bundle(&pack_source(&source, kind), kind, limits()).unwrap();
                assert_eq!(
                    decoded.coordinate[0].to_bits(),
                    expected.to_bits(),
                    "{literal}"
                );
                assert_eq!(
                    decoded.signals[0].real[0].to_bits(),
                    expected.to_bits(),
                    "{literal}"
                );
                assert!(decoded.signals[0].real[1].is_nan());
                if complex {
                    let imag = decoded.signals[0].imag.as_ref().unwrap();
                    assert_eq!(imag[0].to_bits(), expected.to_bits(), "{literal}");
                    assert!(imag[1].is_nan());
                }
            }
        }
    }
}
