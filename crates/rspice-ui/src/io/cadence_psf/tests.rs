    use super::test_helpers::{
        build_non_windowed_array_complex_psf,
        build_non_windowed_array_of_struct_bare_descriptor_psf,
        build_non_windowed_array_of_struct_psf, build_non_windowed_array_real_psf,
        build_non_windowed_complex_psf, build_non_windowed_int32_psf, build_non_windowed_int8_psf,
        build_non_windowed_mixed_real_and_string_psf,
        build_non_windowed_nested_array_real_bare_descriptor_psf,
        build_non_windowed_nested_array_real_psf, build_non_windowed_real_psf,
        build_non_windowed_struct_psf, build_non_windowed_struct_with_array_psf,
        build_non_windowed_unknown_word_psf, build_non_windowed_variable_length_array_psf,
        build_windowed_array_complex_psf, build_windowed_array_of_struct_bare_descriptor_psf,
        build_windowed_array_of_struct_psf, build_windowed_array_real_psf,
        build_windowed_array_real_unaligned_payload_psf,
        build_windowed_nested_array_real_bare_descriptor_psf, build_windowed_nested_array_real_psf,
        build_windowed_real_psf, build_windowed_struct_with_array_psf,
        build_windowed_variable_length_array_psf,
    };
    use super::*;
    use std::collections::HashMap;

    fn patch_header_int(bytes: &mut [u8], key: &str, value: i32) {
        let toc = parse_toc(bytes).expect("fixture must contain valid TOC");
        let entry = toc
            .section(SectionKind::Header)
            .expect("fixture must contain header section");
        let mut idx = entry.start + 8;
        while idx + 4 <= entry.end {
            let block = peek_u32(&bytes[idx..idx + 4]);
            idx += 4;
            if !(33..=35).contains(&block) {
                break;
            }

            let name_len = peek_u32(&bytes[idx..idx + 4]) as usize;
            idx += 4;
            let name_start = idx;
            let name_end = name_start + name_len;
            let name = std::str::from_utf8(&bytes[name_start..name_end]).unwrap_or_default();
            let name_pad = (4 - (name_len % 4)) % 4;
            idx = name_end + name_pad;

            match block {
                33 => {
                    let value_len = peek_u32(&bytes[idx..idx + 4]) as usize;
                    idx += 4;
                    let value_pad = (4 - (value_len % 4)) % 4;
                    idx += value_len + value_pad;
                }
                34 => {
                    if name == key {
                        bytes[idx..idx + 4].copy_from_slice(&value.to_be_bytes());
                        return;
                    }
                    idx += 4;
                }
                35 => {
                    idx += 8;
                }
                _ => unreachable!(),
            }
        }
        panic!("header key '{}' not found in fixture", key);
    }

    fn patch_first_window_block_count(bytes: &mut [u8], window_count: u32) {
        let toc = parse_toc(bytes).expect("fixture must contain valid TOC");
        let entry = toc
            .section(SectionKind::Value)
            .expect("fixture must contain value section");
        let mut idx = entry.start + 8;
        assert_eq!(peek_u32(&bytes[idx..idx + 4]), 20);
        idx += 4;
        let zero_pad_len = peek_u32(&bytes[idx..idx + 4]) as usize;
        idx += 4 + zero_pad_len;
        assert_eq!(peek_u32(&bytes[idx..idx + 4]), 16);
        idx += 4;
        // block_init low 16-bits stores window_count in PSF payloads.
        bytes[idx..idx + 4].copy_from_slice(&window_count.to_be_bytes());
    }

    #[test]
    fn test_parse_non_windowed_real_psf_binary() {
        let bytes = build_non_windowed_real_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].name, "time");
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);

        assert_eq!(parsed.real_signals.len(), 1);
        assert_eq!(parsed.real_signals[0].name, "V(out)");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 2.0]);
        assert!(parsed.complex_signals.is_empty());
    }

    #[test]
    fn test_parse_non_windowed_complex_psf_binary() {
        let bytes = build_non_windowed_complex_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.real_signals.is_empty());

        assert_eq!(parsed.complex_signals.len(), 1);
        assert_eq!(parsed.complex_signals[0].name, "V(out)");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(1.0, 0.5), (2.0, -0.25)]
        );
    }

    #[test]
    fn test_parse_non_windowed_int8_psf_binary() {
        let bytes = build_non_windowed_int8_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 1);
        assert_eq!(parsed.real_signals[0].name, "V(out)");
        assert_eq!(parsed.real_signals[0].values, vec![7.0, 255.0]);
    }

    #[test]
    fn test_parse_non_windowed_int32_psf_binary() {
        let bytes = build_non_windowed_int32_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 1);
        assert_eq!(parsed.real_signals[0].name, "V(out)");
        assert_eq!(parsed.real_signals[0].values, vec![1024.0, -2.0]);
    }

    #[test]
    fn test_parse_non_windowed_unknown_scalar_type_is_ignored() {
        let bytes = build_non_windowed_unknown_word_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.real_signals.is_empty());
        assert!(parsed.complex_signals.is_empty());
    }

    #[test]
    fn test_parse_non_windowed_struct_psf_binary() {
        let bytes = build_non_windowed_struct_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].name, "time");
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);

        assert_eq!(parsed.real_signals.len(), 1);
        assert_eq!(parsed.real_signals[0].name, "V(out).dc");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);

        assert_eq!(parsed.complex_signals.len(), 1);
        assert_eq!(parsed.complex_signals[0].name, "V(out).ac");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(2.0, 0.5), (2.5, -0.25)]
        );
    }

    #[test]
    fn test_parse_non_windowed_string_trace_is_ignored() {
        let bytes = build_non_windowed_mixed_real_and_string_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.real_signals.len(), 1);
        assert_eq!(parsed.real_signals[0].name, "V(out)");
        assert_eq!(parsed.real_signals[0].values, vec![1.25, 2.5]);
        assert!(parsed.complex_signals.is_empty());
    }

    #[test]
    fn test_parse_non_windowed_real_array_psf_binary() {
        let bytes = build_non_windowed_array_real_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(arr)[0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(arr)[1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
    }

    #[test]
    fn test_parse_non_windowed_complex_array_psf_binary() {
        let bytes = build_non_windowed_array_complex_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.real_signals.is_empty());
        assert_eq!(parsed.complex_signals.len(), 2);
        assert_eq!(parsed.complex_signals[0].name, "I(arr)[0]");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(1.0, 0.25), (1.5, 0.125)]
        );
        assert_eq!(parsed.complex_signals[1].name, "I(arr)[1]");
        assert_eq!(
            parsed.complex_signals[1].values,
            vec![(2.0, -0.5), (2.5, -0.75)]
        );
    }

    #[test]
    fn test_parse_non_windowed_struct_with_array_psf_binary() {
        let bytes = build_non_windowed_struct_with_array_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 3);
        assert_eq!(parsed.real_signals[0].name, "V(out).gain");
        assert_eq!(parsed.real_signals[0].values, vec![10.0, 11.0]);
        assert_eq!(parsed.real_signals[1].name, "V(out).taps[0]");
        assert_eq!(parsed.real_signals[1].values, vec![0.1, 0.15]);
        assert_eq!(parsed.real_signals[2].name, "V(out).taps[1]");
        assert_eq!(parsed.real_signals[2].values, vec![0.2, 0.25]);
    }

    #[test]
    fn test_parse_non_windowed_variable_length_array_pads_missing_values() {
        let bytes = build_non_windowed_variable_length_array_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 3);
        assert_eq!(parsed.real_signals[0].name, "V(arr)[0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(arr)[1]");
        assert!(parsed.real_signals[1].values[0].is_nan());
        assert_eq!(parsed.real_signals[1].values[1], 2.5);
        assert_eq!(parsed.real_signals[2].name, "V(arr)[2]");
        assert!(parsed.real_signals[2].values[0].is_nan());
        assert_eq!(parsed.real_signals[2].values[1], 3.5);
    }

    #[test]
    fn test_parse_non_windowed_array_of_struct_psf_binary() {
        let bytes = build_non_windowed_array_of_struct_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0].dc");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[1].dc");
        assert_eq!(parsed.real_signals[1].values, vec![1.1, 1.6]);
        assert_eq!(parsed.complex_signals.len(), 2);
        assert_eq!(parsed.complex_signals[0].name, "V(out)[0].ac");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(2.0, 0.5), (2.5, -0.2)]
        );
        assert_eq!(parsed.complex_signals[1].name, "V(out)[1].ac");
        assert_eq!(
            parsed.complex_signals[1].values,
            vec![(2.1, 0.6), (2.6, -0.3)]
        );
    }

    #[test]
    fn test_parse_non_windowed_array_of_struct_bare_descriptor_psf_binary() {
        let bytes = build_non_windowed_array_of_struct_bare_descriptor_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0].dc");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[1].dc");
        assert_eq!(parsed.real_signals[1].values, vec![1.1, 1.6]);
        assert_eq!(parsed.complex_signals.len(), 2);
        assert_eq!(parsed.complex_signals[0].name, "V(out)[0].ac");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(2.0, 0.5), (2.5, -0.2)]
        );
        assert_eq!(parsed.complex_signals[1].name, "V(out)[1].ac");
        assert_eq!(
            parsed.complex_signals[1].values,
            vec![(2.1, 0.6), (2.6, -0.3)]
        );
    }

    #[test]
    fn test_parse_non_windowed_nested_array_real_psf_binary() {
        let bytes = build_non_windowed_nested_array_real_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 4);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0][0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[0][1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
        assert_eq!(parsed.real_signals[2].name, "V(out)[1][0]");
        assert_eq!(parsed.real_signals[2].values, vec![3.0, 3.5]);
        assert_eq!(parsed.real_signals[3].name, "V(out)[1][1]");
        assert_eq!(parsed.real_signals[3].values, vec![4.0, 4.5]);
    }

    #[test]
    fn test_parse_non_windowed_nested_array_real_bare_descriptor_psf_binary() {
        let bytes = build_non_windowed_nested_array_real_bare_descriptor_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 4);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0][0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[0][1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
        assert_eq!(parsed.real_signals[2].name, "V(out)[1][0]");
        assert_eq!(parsed.real_signals[2].values, vec![3.0, 3.5]);
        assert_eq!(parsed.real_signals[3].name, "V(out)[1][1]");
        assert_eq!(parsed.real_signals[3].values, vec![4.0, 4.5]);
    }

    #[test]
    fn test_parse_windowed_real_psf_binary() {
        let bytes = build_windowed_real_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].name, "time");
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert_eq!(parsed.real_signals.len(), 1);
        assert_eq!(parsed.real_signals[0].name, "V(out)");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 2.0]);
        assert!(parsed.complex_signals.is_empty());
    }

    #[test]
    fn test_parse_windowed_real_array_psf_binary() {
        let bytes = build_windowed_array_real_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(arr)[0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(arr)[1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
    }

    #[test]
    fn test_parse_windowed_real_array_psf_binary_with_unaligned_payload_start() {
        let bytes = build_windowed_array_real_unaligned_payload_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(arr)[0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(arr)[1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
    }

    #[test]
    fn test_parse_windowed_complex_array_psf_binary() {
        let bytes = build_windowed_array_complex_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.real_signals.is_empty());
        assert_eq!(parsed.complex_signals.len(), 2);
        assert_eq!(parsed.complex_signals[0].name, "I(arr)[0]");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(1.0, 0.25), (1.5, 0.125)]
        );
        assert_eq!(parsed.complex_signals[1].name, "I(arr)[1]");
        assert_eq!(
            parsed.complex_signals[1].values,
            vec![(2.0, -0.5), (2.5, -0.75)]
        );
    }

    #[test]
    fn test_parse_windowed_struct_with_array_psf_binary() {
        let bytes = build_windowed_struct_with_array_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 3);
        assert_eq!(parsed.real_signals[0].name, "V(out).gain");
        assert_eq!(parsed.real_signals[0].values, vec![10.0, 11.0]);
        assert_eq!(parsed.real_signals[1].name, "V(out).taps[0]");
        assert_eq!(parsed.real_signals[1].values, vec![0.1, 0.15]);
        assert_eq!(parsed.real_signals[2].name, "V(out).taps[1]");
        assert_eq!(parsed.real_signals[2].values, vec![0.2, 0.25]);
    }

    #[test]
    fn test_parse_windowed_variable_length_array_pads_missing_values() {
        let bytes = build_windowed_variable_length_array_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 3);
        assert_eq!(parsed.real_signals[0].name, "V(arr)[0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(arr)[1]");
        assert!(parsed.real_signals[1].values[0].is_nan());
        assert_eq!(parsed.real_signals[1].values[1], 2.5);
        assert_eq!(parsed.real_signals[2].name, "V(arr)[2]");
        assert!(parsed.real_signals[2].values[0].is_nan());
        assert_eq!(parsed.real_signals[2].values[1], 3.5);
    }

    #[test]
    fn test_parse_windowed_array_of_struct_psf_binary() {
        let bytes = build_windowed_array_of_struct_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0].dc");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[1].dc");
        assert_eq!(parsed.real_signals[1].values, vec![1.1, 1.6]);
        assert_eq!(parsed.complex_signals.len(), 2);
        assert_eq!(parsed.complex_signals[0].name, "V(out)[0].ac");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(2.0, 0.5), (2.5, -0.2)]
        );
        assert_eq!(parsed.complex_signals[1].name, "V(out)[1].ac");
        assert_eq!(
            parsed.complex_signals[1].values,
            vec![(2.1, 0.6), (2.6, -0.3)]
        );
    }

    #[test]
    fn test_parse_windowed_array_of_struct_bare_descriptor_psf_binary() {
        let bytes = build_windowed_array_of_struct_bare_descriptor_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0].dc");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[1].dc");
        assert_eq!(parsed.real_signals[1].values, vec![1.1, 1.6]);
        assert_eq!(parsed.complex_signals.len(), 2);
        assert_eq!(parsed.complex_signals[0].name, "V(out)[0].ac");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(2.0, 0.5), (2.5, -0.2)]
        );
        assert_eq!(parsed.complex_signals[1].name, "V(out)[1].ac");
        assert_eq!(
            parsed.complex_signals[1].values,
            vec![(2.1, 0.6), (2.6, -0.3)]
        );
    }

    #[test]
    fn test_parse_windowed_nested_array_real_psf_binary() {
        let bytes = build_windowed_nested_array_real_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 4);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0][0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[0][1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
        assert_eq!(parsed.real_signals[2].name, "V(out)[1][0]");
        assert_eq!(parsed.real_signals[2].values, vec![3.0, 3.5]);
        assert_eq!(parsed.real_signals[3].name, "V(out)[1][1]");
        assert_eq!(parsed.real_signals[3].values, vec![4.0, 4.5]);
    }

    #[test]
    fn test_parse_windowed_nested_array_real_bare_descriptor_psf_binary() {
        let bytes = build_windowed_nested_array_real_bare_descriptor_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 4);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0][0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[0][1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
        assert_eq!(parsed.real_signals[2].name, "V(out)[1][0]");
        assert_eq!(parsed.real_signals[2].values, vec![3.0, 3.5]);
        assert_eq!(parsed.real_signals[3].name, "V(out)[1][1]");
        assert_eq!(parsed.real_signals[3].values, vec![4.0, 4.5]);
    }

    #[test]
    fn test_parse_windowed_rejects_trace_count_mismatch() {
        let mut bytes = build_windowed_real_psf();
        patch_header_int(&mut bytes, "PSF traces", 2);
        let err = parse_cadence_psf_binary(&bytes).expect_err("mismatched trace count must fail");
        assert!(err.to_string().contains("trace count mismatch"));
    }

    #[test]
    fn test_parse_windowed_rejects_zero_sample_block() {
        let mut bytes = build_windowed_real_psf();
        patch_first_window_block_count(&mut bytes, 0);
        let err = parse_cadence_psf_binary(&bytes).expect_err("zero-sample block must fail");
        assert!(err.to_string().contains("zero samples"));
    }

    #[test]
    fn test_parse_windowed_rejects_block_overshoot() {
        let mut bytes = build_windowed_real_psf();
        patch_first_window_block_count(&mut bytes, 3);
        let err = parse_cadence_psf_binary(&bytes)
            .expect_err("block larger than sweep-points declaration must fail");
        assert!(err.to_string().contains("exceeds declared sweep points"));
    }

    #[test]
    fn test_parse_truncated_binary_fails() {
        let mut bytes = build_non_windowed_real_psf();
        bytes.truncate(bytes.len().saturating_sub(9));
        let err = parse_cadence_psf_binary(&bytes).expect_err("truncated input should fail");
        assert!(
            err.to_string().contains("TOC")
                || err.to_string().contains("truncated")
                || err.to_string().contains("invalid")
        );
    }

    #[test]
    fn test_parse_header_preserves_signed_int_values() {
        let mut bytes = build_non_windowed_real_psf();
        patch_header_int(&mut bytes, "PSF sweep points", -2);

        let toc = parse_toc(&bytes).expect("fixture must contain valid TOC");
        let header = parse_header(
            &bytes,
            toc.section(SectionKind::Header)
                .expect("fixture must contain header section"),
        )
        .expect("header parse should succeed");

        assert_eq!(
            header.get("PSF sweep points"),
            Some(&CadencePsfValue::Int(-2))
        );
    }

    #[test]
    fn test_header_usize_accepts_integral_real_values() {
        let mut header = HashMap::new();
        header.insert("PSF sweep points".to_string(), CadencePsfValue::Real(8.0));
        assert_eq!(header_usize(&header, "PSF sweep points").unwrap(), 8);
    }

    #[test]
    fn test_header_usize_rejects_negative_int_values() {
        let mut header = HashMap::new();
        header.insert("PSF sweep points".to_string(), CadencePsfValue::Int(-1));
        let err = header_usize(&header, "PSF sweep points")
            .expect_err("negative integer header counts must fail");
        assert!(err.to_string().contains("non-negative integer count"));
    }

    #[test]
    fn test_header_usize_rejects_non_finite_real_values() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut header = HashMap::new();
            header.insert("PSF sweep points".to_string(), CadencePsfValue::Real(value));
            let err = header_usize(&header, "PSF sweep points")
                .expect_err("non-finite real header values must fail");
            assert!(err.to_string().contains("must be finite"));
        }
    }

    #[test]
    fn test_header_usize_rejects_fractional_real_values() {
        let mut header = HashMap::new();
        header.insert("PSF sweep points".to_string(), CadencePsfValue::Real(2.5));
        let err =
            header_usize(&header, "PSF sweep points").expect_err("fractional counts must fail");
        assert!(err.to_string().contains("integer count"));
    }
