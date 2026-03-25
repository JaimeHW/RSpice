use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use rspice_ui::io::parse_cadence_psf_binary;

fn to_u32(value: usize) -> u32 {
    u32::try_from(value).expect("benchmark fixture value exceeds u32 range")
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn patch_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
}

fn patch_u32_to_current_len(bytes: &mut [u8], current_len: usize, offset: usize) {
    let value = to_u32(current_len);
    patch_u32(bytes, offset, value);
}

fn push_f64(bytes: &mut Vec<u8>, value: f64) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_string(bytes: &mut Vec<u8>, value: &str) {
    push_u32(bytes, to_u32(value.len()));
    bytes.extend_from_slice(value.as_bytes());
    let pad = (4 - (value.len() % 4)) % 4;
    bytes.extend(std::iter::repeat_n(0u8, pad));
}

fn push_named_int(bytes: &mut Vec<u8>, name: &str, value: i32) {
    push_u32(bytes, 34);
    push_string(bytes, name);
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_signal_ref(bytes: &mut Vec<u8>, id: u32, name: &str, type_id: u32) {
    push_u32(bytes, id);
    push_string(bytes, name);
    push_u32(bytes, type_id);
}

fn build_large_non_windowed_real_psf(point_count: usize, trace_count: usize) -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_end_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", point_count as i32);
    push_named_int(&mut bytes, "PSF traces", trace_count as i32);
    let current_len = bytes.len();
    patch_u32_to_current_len(bytes.as_mut_slice(), current_len, header_end_pos);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_end_pos = bytes.len();
    push_u32(&mut bytes, 0);

    // Trace type: real scalar.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);

    // Sweep type: real scalar.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);

    let current_len = bytes.len();
    patch_u32_to_current_len(bytes.as_mut_slice(), current_len, types_end_pos);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_end_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 2);
    let current_len = bytes.len();
    patch_u32_to_current_len(bytes.as_mut_slice(), current_len, sweep_end_pos);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_end_pos = bytes.len();
    push_u32(&mut bytes, 0);
    for trace_idx in 0..trace_count {
        push_u32(&mut bytes, 16);
        push_signal_ref(
            &mut bytes,
            200 + to_u32(trace_idx),
            &format!("V(out{})", trace_idx),
            1,
        );
    }
    let current_len = bytes.len();
    patch_u32_to_current_len(bytes.as_mut_slice(), current_len, trace_end_pos);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_end_pos = bytes.len();
    push_u32(&mut bytes, 0);
    for point_idx in 0..point_count {
        push_u32(&mut bytes, to_u32(point_idx));
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, point_idx as f64 * 1e-9);
        for trace_idx in 0..trace_count {
            push_f64(&mut bytes, 0.0);
            push_f64(
                &mut bytes,
                (trace_idx as f64 + 1.0) * 1e-3 + point_idx as f64 * 1e-6,
            );
        }
    }
    let current_len = bytes.len();
    patch_u32_to_current_len(bytes.as_mut_slice(), current_len, value_end_pos);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, to_u32(start));
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, to_u32(toc_offset));
    bytes
}

fn build_large_windowed_real_psf(
    point_count: usize,
    trace_count: usize,
    window_block_size: usize,
) -> Vec<u8> {
    let mut bytes = Vec::new();
    let window_size = window_block_size
        .checked_mul(8)
        .expect("window size overflow");

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_end_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", point_count as i32);
    push_named_int(&mut bytes, "PSF traces", trace_count as i32);
    push_named_int(&mut bytes, "PSF window size", window_size as i32);
    let current_len = bytes.len();
    patch_u32_to_current_len(bytes.as_mut_slice(), current_len, header_end_pos);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_end_pos = bytes.len();
    push_u32(&mut bytes, 0);

    // Trace type: real scalar.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);

    // Sweep type: real scalar.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);

    let current_len = bytes.len();
    patch_u32_to_current_len(bytes.as_mut_slice(), current_len, types_end_pos);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_end_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 2);
    let current_len = bytes.len();
    patch_u32_to_current_len(bytes.as_mut_slice(), current_len, sweep_end_pos);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_end_pos = bytes.len();
    push_u32(&mut bytes, 0);
    for trace_idx in 0..trace_count {
        push_u32(&mut bytes, 16);
        push_signal_ref(
            &mut bytes,
            200 + to_u32(trace_idx),
            &format!("V(out{})", trace_idx),
            1,
        );
    }
    let current_len = bytes.len();
    patch_u32_to_current_len(bytes.as_mut_slice(), current_len, trace_end_pos);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_end_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 0);

    let mut emitted_points = 0usize;
    while emitted_points < point_count {
        let block_points = (point_count - emitted_points).min(window_block_size);
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, to_u32(block_points));
        for local_idx in 0..block_points {
            let sample_idx = emitted_points + local_idx;
            push_f64(&mut bytes, sample_idx as f64 * 1e-9);
        }

        for trace_idx in 0..trace_count {
            let data_len = block_points
                .checked_mul(8)
                .expect("windowed trace data length overflow");
            let left_pad = window_size
                .checked_sub(data_len)
                .expect("window size smaller than trace data payload");
            bytes.extend(std::iter::repeat_n(0u8, left_pad));
            for local_idx in 0..block_points {
                let sample_idx = emitted_points + local_idx;
                push_f64(
                    &mut bytes,
                    (trace_idx as f64 + 1.0) * 1e-3 + sample_idx as f64 * 1e-6,
                );
            }
        }

        emitted_points += block_points;
    }
    let current_len = bytes.len();
    patch_u32_to_current_len(bytes.as_mut_slice(), current_len, value_end_pos);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, to_u32(start));
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, to_u32(toc_offset));
    bytes
}

fn bench_parse_large_non_windowed(c: &mut Criterion) {
    let point_count = 20_000usize;
    let trace_count = 8usize;
    let payload = build_large_non_windowed_real_psf(point_count, trace_count);

    let sanity = parse_cadence_psf_binary(&payload).expect("non-windowed fixture must parse");
    assert_eq!(sanity.sweeps[0].values.len(), point_count);
    assert_eq!(sanity.real_signals.len(), trace_count);

    let mut group = c.benchmark_group("cadence_psf_non_windowed");
    group.throughput(Throughput::Bytes(payload.len() as u64));
    group.bench_function(
        BenchmarkId::new("parse", format!("{}pts_{}traces", point_count, trace_count)),
        |b| {
            b.iter(|| {
                let parsed =
                    parse_cadence_psf_binary(black_box(payload.as_slice())).expect("parse failed");
                black_box(parsed.real_signals.len());
            });
        },
    );
    group.finish();
}

fn bench_parse_large_windowed(c: &mut Criterion) {
    let point_count = 50_000usize;
    let trace_count = 8usize;
    let payload = build_large_windowed_real_psf(point_count, trace_count, 256);

    let sanity = parse_cadence_psf_binary(&payload).expect("windowed fixture must parse");
    assert_eq!(sanity.sweeps[0].values.len(), point_count);
    assert_eq!(sanity.real_signals.len(), trace_count);

    let mut group = c.benchmark_group("cadence_psf_windowed");
    group.throughput(Throughput::Bytes(payload.len() as u64));
    group.bench_function(
        BenchmarkId::new("parse", format!("{}pts_{}traces", point_count, trace_count)),
        |b| {
            b.iter(|| {
                let parsed =
                    parse_cadence_psf_binary(black_box(payload.as_slice())).expect("parse failed");
                black_box(parsed.real_signals.len());
            });
        },
    );
    group.finish();
}

criterion_group!(
    cadence_psf,
    bench_parse_large_non_windowed,
    bench_parse_large_windowed
);
criterion_main!(cadence_psf);
