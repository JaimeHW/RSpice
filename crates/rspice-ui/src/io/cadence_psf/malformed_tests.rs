use super::test_helpers::{build_non_windowed_real_psf, build_windowed_real_psf};
use super::*;

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

fn patch_section_end_offset(bytes: &mut [u8], kind: SectionKind, end_offset: u32) {
    let toc = parse_toc(bytes).expect("fixture must contain valid TOC");
    let entry = toc
        .section(kind)
        .expect("fixture must contain requested section");
    let end_offset_field = match kind {
        SectionKind::Type | SectionKind::Trace => entry.start + 12,
        _ => entry.start + 4,
    };
    bytes[end_offset_field..end_offset_field + 4].copy_from_slice(&end_offset.to_be_bytes());
}

fn patch_section_block_type(bytes: &mut [u8], kind: SectionKind, block_type: u32) {
    let toc = parse_toc(bytes).expect("fixture must contain valid TOC");
    let entry = toc
        .section(kind)
        .expect("fixture must contain requested section");
    let block_field = entry.start + 8;
    bytes[block_field..block_field + 4].copy_from_slice(&block_type.to_be_bytes());
}

#[test]
fn test_parse_non_windowed_fixture_accepts_full_payload() {
    let payload = build_non_windowed_real_psf();
    let parsed = parse_cadence_psf_binary(&payload).expect("full fixture should parse");
    assert!(!parsed.sweeps.is_empty());
    assert!(!parsed.real_signals.is_empty());
}

#[test]
fn test_parse_rejects_all_non_windowed_prefix_truncations() {
    let payload = build_non_windowed_real_psf();
    for truncated_len in 0..payload.len() {
        let truncated = &payload[..truncated_len];
        assert!(
            parse_cadence_psf_binary(truncated).is_err(),
            "expected truncation at {} bytes to fail",
            truncated_len
        );
    }
}

#[test]
fn test_parse_rejects_all_windowed_prefix_truncations() {
    let payload = build_windowed_real_psf();
    for truncated_len in 0..payload.len() {
        let truncated = &payload[..truncated_len];
        assert!(
            parse_cadence_psf_binary(truncated).is_err(),
            "expected truncation at {} bytes to fail",
            truncated_len
        );
    }
}

#[test]
fn test_parse_rejects_out_of_bounds_value_section_end_offset() {
    let mut payload = build_non_windowed_real_psf();
    let out_of_bounds = (payload.len() as u32).saturating_add(1024);
    patch_section_end_offset(&mut payload, SectionKind::Value, out_of_bounds);

    let err = parse_cadence_psf_binary(&payload).expect_err("out-of-bounds section end must fail");
    assert!(err.to_string().contains("invalid value section end offset"));
}

#[test]
fn test_parse_rejects_invalid_type_section_block_tag() {
    let mut payload = build_non_windowed_real_psf();
    patch_section_block_type(&mut payload, SectionKind::Type, 99);

    let err = parse_cadence_psf_binary(&payload).expect_err("invalid type block tag must fail");
    assert!(err.to_string().contains("type section expected block 22"));
}

#[test]
fn test_parse_rejects_negative_sweep_point_count() {
    let mut payload = build_non_windowed_real_psf();
    patch_header_int(&mut payload, "PSF sweep points", -1);

    let err = parse_cadence_psf_binary(&payload).expect_err("negative sweep count must fail");
    assert!(err.to_string().contains("PSF sweep points"));
    assert!(err.to_string().contains("non-negative"));
}

#[test]
fn test_parse_windowed_rejects_trace_count_mismatch() {
    let mut payload = build_windowed_real_psf();
    patch_header_int(&mut payload, "PSF traces", 99);

    let err = parse_cadence_psf_binary(&payload).expect_err("trace count mismatch must fail");
    assert!(err.to_string().contains("trace count mismatch"));
}

#[test]
fn test_parse_does_not_panic_under_deterministic_byte_flips() {
    let payload = build_non_windowed_real_psf();
    let mut observed_errors = 0usize;

    for idx in (0..payload.len()).step_by(17) {
        let mut mutated = payload.clone();
        mutated[idx] ^= 0x5a;
        if parse_cadence_psf_binary(&mutated).is_err() {
            observed_errors += 1;
        }
    }

    assert!(
        observed_errors > 0,
        "expected deterministic mutations to trigger at least one parse error"
    );
}
