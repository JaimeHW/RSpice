//! RF text must retain its numeric meaning through unit and pair conversion.
use rspice_formats::{TouchstoneError, read_touchstone_bytes, read_touchstone_bytes_with_limit};

fn noise_file(version: u32, unit: &str, network: &str, noise: &str) -> String {
    let network = format!("{network} 1 0 1 0 1 0 1 0\n");
    if version == 1 {
        format!("# {unit} S RI R 50\n{network}{noise}\n")
    } else {
        format!(
            "[Version] 2.0\n# {unit} S RI R 50\n[Number of Ports] 2\n[Two-Port Data Order] 21_12\n[Number of Frequencies] 1\n[Number of Noise Frequencies] 1\n[Network Data]\n{network}[Noise Data]\n{noise}\n[End]\n"
        )
    }
}

#[test]
fn nonzero_network_tokens_cannot_underflow_to_zero() {
    for format in ["RI", "MA", "DB"] {
        for position in 0..3 {
            for literal in ["1e-999", "-1D-999", "2e-324", "-2d-324"] {
                let mut fields = ["1", "1", "45"];
                fields[position] = literal;
                let source = format!("# Hz S {format} R 50\n{}\n", fields.join(" "));
                let error = read_touchstone_bytes("network.s1p", source.as_bytes()).unwrap_err();
                assert!(error.to_string().contains("underflow"), "{source}: {error}");
                assert!(error.to_string().contains(literal), "{error}");
            }
        }
    }
}

#[test]
fn frequencies_are_scaled_as_decimals_before_binary_rounding() {
    for (unit, literal, expected) in [
        ("Hz", "5e-324", 5e-324_f64),
        ("kHz", "1e-326", 1e-323_f64),
        ("MHz", "5D-330", 5e-324_f64),
        ("GHz", "1e-330", 1e-321_f64),
        ("GHz", "-0e-999", -0.0_f64),
        ("GHz", "0e999", 0.0_f64),
        ("GHz", "1.0000000000000001e-2", 1.0000000000000001e7_f64),
    ] {
        // Inference and physical line wrapping must share the same conversion.
        for name in ["network.s1p", "network.data"] {
            let source = format!("# {unit} S RI R 50\n{literal}\n5e-324 -0e-999\n");
            let data = read_touchstone_bytes(name, source.as_bytes()).unwrap();
            assert_eq!(
                data.x_signal.as_ref().unwrap().data[0].to_bits(),
                expected.to_bits(),
                "{source}"
            );
            assert_eq!(
                data.get_signal("S11_RE").unwrap().data[0].to_bits(),
                5e-324_f64.to_bits()
            );
            assert_eq!(
                data.get_signal("S11_IM").unwrap().data[0].to_bits(),
                (-0.0_f64).to_bits()
            );
        }
    }
}

#[test]
fn independent_noise_records_refuse_underflow_in_every_field() {
    for version in [1, 2] {
        for position in 0..5 {
            let mut fields = ["0.5", "3", "0.5", "45", "1"];
            fields[position] = "1D-999";
            let source = noise_file(version, "Hz", "1", &fields.join(" "));
            let error = read_touchstone_bytes("noise.s2p", source.as_bytes()).unwrap_err();
            assert!(
                error.to_string().contains("underflow"),
                "{position}: {error}"
            );
        }
    }
}

#[test]
fn independent_noise_frequency_units_preserve_subnormals() {
    for version in [1, 2] {
        let source = noise_file(version, "GHz", "2e-330", "1d-330 3 0.5 45 1");
        let data = read_touchstone_bytes("noise.s2p", source.as_bytes()).unwrap();
        assert_eq!(data.x_signal.as_ref().unwrap().data, [2e-321]);
        assert_eq!(
            data.get_signal("Rn").unwrap().x_values.as_deref(),
            Some([1e-321].as_slice())
        );
    }
}

#[test]
fn decibel_conversion_cannot_silently_erase_a_finite_coefficient() {
    let source = b"# Hz S DB R 50\n1 -10000 0\n";
    let error = read_touchstone_bytes("network.s1p", source).unwrap_err();
    assert!(error.to_string().contains("underflow"), "{error}");
    // Admission must still precede the numeric failure.
    assert!(matches!(
        read_touchstone_bytes_with_limit("network.s1p", source, 2),
        Err(TouchstoneError::ValueLimit {
            requested: 3,
            limit: 2
        })
    ));
}
