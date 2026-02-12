use super::*;

#[derive(Clone, Copy)]
enum SampleEncoding {
    Real,
    Complex,
    Int8,
    Int32,
    UnknownWord,
}

impl SampleEncoding {
    fn type_code(self) -> u32 {
        match self {
            Self::Real => 11,
            Self::Complex => 12,
            Self::Int8 => 1,
            Self::Int32 => 5,
            Self::UnknownWord => 99,
        }
    }
}

pub(crate) fn build_non_windowed_real_psf() -> Vec<u8> {
    build_simple_non_windowed_psf(SampleEncoding::Real)
}

pub(crate) fn build_non_windowed_complex_psf() -> Vec<u8> {
    build_simple_non_windowed_psf(SampleEncoding::Complex)
}

pub(crate) fn build_non_windowed_int8_psf() -> Vec<u8> {
    build_simple_non_windowed_psf(SampleEncoding::Int8)
}

pub(crate) fn build_non_windowed_int32_psf() -> Vec<u8> {
    build_simple_non_windowed_psf(SampleEncoding::Int32)
}

pub(crate) fn build_non_windowed_unknown_word_psf() -> Vec<u8> {
    build_simple_non_windowed_psf(SampleEncoding::UnknownWord)
}

pub(crate) fn build_non_windowed_struct_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Root type: struct with two members.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "sigtype");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    // Member 1: real scalar
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "dc");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    // Member 2: complex scalar
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 3);
    push_string(&mut bytes, "ac");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 12);
    push_u32(&mut bytes, 18); // struct end
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 2);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(out)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    // Point 0
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 1.0); // dc
    push_f64(&mut bytes, 2.0); // ac.re
    push_f64(&mut bytes, 0.5); // ac.im

    // Point 1
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 1.0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 1.5); // dc
    push_f64(&mut bytes, 2.5); // ac.re
    push_f64(&mut bytes, -0.25); // ac.im

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_non_windowed_mixed_real_and_string_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Real scalar type
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "realtype");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    // String scalar type
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "stringtype");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 2);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 1);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(out)", 1);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 201, "meta", 2);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    // Point 0
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 1.25);
    push_f64(&mut bytes, 0.0);
    push_string(&mut bytes, "A");

    // Point 1
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 1.0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 2.5);
    push_f64(&mut bytes, 0.0);
    push_string(&mut bytes, "B2");

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_non_windowed_array_real_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Top-level array trace type: real elements.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "realarray");
    push_u32(&mut bytes, 11);
    push_u32(&mut bytes, 3);
    // Sweep type: scalar real.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 2);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(arr)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    // Point 0
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 0.0);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 1.0);
    push_f64(&mut bytes, 2.0);

    // Point 1
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 1.0);
    push_f64(&mut bytes, 0.0);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 1.5);
    push_f64(&mut bytes, 2.5);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_non_windowed_array_complex_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Top-level array trace type: complex elements.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "complexarray");
    push_u32(&mut bytes, 12);
    push_u32(&mut bytes, 3);
    // Sweep type: scalar real.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 2);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "I(arr)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    // Point 0
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 0.0);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 1.0);
    push_f64(&mut bytes, 0.25);
    push_f64(&mut bytes, 2.0);
    push_f64(&mut bytes, -0.5);

    // Point 1
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 1.0);
    push_f64(&mut bytes, 0.0);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 1.5);
    push_f64(&mut bytes, 0.125);
    push_f64(&mut bytes, 2.5);
    push_f64(&mut bytes, -0.75);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_non_windowed_struct_with_array_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Root type: struct with scalar and array members.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "sigtype");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    // gain: real scalar
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "gain");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    // taps: real array
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 3);
    push_string(&mut bytes, "taps");
    push_u32(&mut bytes, 11);
    push_u32(&mut bytes, 3);
    push_u32(&mut bytes, 18);
    // Sweep type: scalar real.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 4);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 4);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(out)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    // Point 0
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 10.0); // gain
    push_u32(&mut bytes, 2); // taps count
    push_f64(&mut bytes, 0.1);
    push_f64(&mut bytes, 0.2);

    // Point 1
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 1.0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 11.0); // gain
    push_u32(&mut bytes, 2); // taps count
    push_f64(&mut bytes, 0.15);
    push_f64(&mut bytes, 0.25);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_non_windowed_variable_length_array_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Top-level array trace type: real elements.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "realarray");
    push_u32(&mut bytes, 11);
    push_u32(&mut bytes, 3);
    // Sweep type: scalar real.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 2);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(arr)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    // Point 0: one array value.
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 0.0);
    push_u32(&mut bytes, 1);
    push_f64(&mut bytes, 1.0);

    // Point 1: three array values.
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 1.0);
    push_f64(&mut bytes, 0.0);
    push_u32(&mut bytes, 3);
    push_f64(&mut bytes, 1.5);
    push_f64(&mut bytes, 2.5);
    push_f64(&mut bytes, 3.5);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_windowed_real_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    push_named_int(&mut bytes, "PSF traces", 1);
    push_named_int(&mut bytes, "PSF window size", 24);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
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
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 2);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(out)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 1.0);

    let mut trace_payload = Vec::new();
    push_f64(&mut trace_payload, 1.0);
    push_f64(&mut trace_payload, 2.0);
    push_windowed_trace_payload(&mut bytes, &trace_payload, 24);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_windowed_array_real_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    push_named_int(&mut bytes, "PSF traces", 1);
    push_named_int(&mut bytes, "PSF window size", 48);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Trace type: array of real.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "realarray");
    push_u32(&mut bytes, 11);
    push_u32(&mut bytes, 3);
    // Sweep type: real scalar.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 2);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(arr)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 1.0);

    let mut trace_payload = Vec::new();
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 1.0);
    push_f64(&mut trace_payload, 2.0);
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 1.5);
    push_f64(&mut trace_payload, 2.5);
    push_windowed_trace_payload(&mut bytes, &trace_payload, 48);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_windowed_array_real_unaligned_payload_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    push_named_int(&mut bytes, "PSF traces", 1);
    // 50-byte windows intentionally force non-4-byte-aligned payload start.
    push_named_int(&mut bytes, "PSF window size", 50);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Trace type: array of real.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "realarray");
    push_u32(&mut bytes, 11);
    push_u32(&mut bytes, 3);
    // Sweep type: real scalar.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 2);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(arr)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 1.0);

    let mut trace_payload = Vec::new();
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 1.0);
    push_f64(&mut trace_payload, 2.0);
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 1.5);
    push_f64(&mut trace_payload, 2.5);
    push_windowed_trace_payload(&mut bytes, &trace_payload, 50);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_windowed_array_complex_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    push_named_int(&mut bytes, "PSF traces", 1);
    push_named_int(&mut bytes, "PSF window size", 80);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Trace type: array of complex.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "complexarray");
    push_u32(&mut bytes, 12);
    push_u32(&mut bytes, 3);
    // Sweep type: real scalar.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 2);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "I(arr)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 1.0);

    let mut trace_payload = Vec::new();
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 1.0);
    push_f64(&mut trace_payload, 0.25);
    push_f64(&mut trace_payload, 2.0);
    push_f64(&mut trace_payload, -0.5);
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 1.5);
    push_f64(&mut trace_payload, 0.125);
    push_f64(&mut trace_payload, 2.5);
    push_f64(&mut trace_payload, -0.75);
    push_windowed_trace_payload(&mut bytes, &trace_payload, 80);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_windowed_struct_with_array_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    push_named_int(&mut bytes, "PSF traces", 1);
    push_named_int(&mut bytes, "PSF window size", 64);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Root type: struct with scalar and array.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "sigtype");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "gain");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 3);
    push_string(&mut bytes, "taps");
    push_u32(&mut bytes, 11);
    push_u32(&mut bytes, 3);
    push_u32(&mut bytes, 18);
    // Sweep type: real scalar.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 4);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 4);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(out)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 1.0);

    let mut trace_payload = Vec::new();
    push_f64(&mut trace_payload, 10.0);
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 0.1);
    push_f64(&mut trace_payload, 0.2);
    push_f64(&mut trace_payload, 11.0);
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 0.15);
    push_f64(&mut trace_payload, 0.25);
    push_windowed_trace_payload(&mut bytes, &trace_payload, 64);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_windowed_variable_length_array_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    push_named_int(&mut bytes, "PSF traces", 1);
    push_named_int(&mut bytes, "PSF window size", 48);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Trace type: array of real.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "realarray");
    push_u32(&mut bytes, 11);
    push_u32(&mut bytes, 3);
    // Sweep type: real scalar.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 2);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(arr)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 1.0);

    let mut trace_payload = Vec::new();
    push_u32(&mut trace_payload, 1);
    push_f64(&mut trace_payload, 1.0);
    push_u32(&mut trace_payload, 3);
    push_f64(&mut trace_payload, 1.5);
    push_f64(&mut trace_payload, 2.5);
    push_f64(&mut trace_payload, 3.5);
    push_windowed_trace_payload(&mut bytes, &trace_payload, 48);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_non_windowed_array_of_struct_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Top-level trace: array of struct(type_id=2).
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "array_of_struct");
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 3);
    // Struct element type.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "elem");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 3);
    push_string(&mut bytes, "dc");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 4);
    push_string(&mut bytes, "ac");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 12);
    push_u32(&mut bytes, 18);
    // Sweep type.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 5);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 5);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(out)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    // Point 0
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 0.0);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 1.0);
    push_f64(&mut bytes, 2.0);
    push_f64(&mut bytes, 0.5);
    push_f64(&mut bytes, 1.1);
    push_f64(&mut bytes, 2.1);
    push_f64(&mut bytes, 0.6);

    // Point 1
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 1.0);
    push_f64(&mut bytes, 0.0);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 1.5);
    push_f64(&mut bytes, 2.5);
    push_f64(&mut bytes, -0.2);
    push_f64(&mut bytes, 1.6);
    push_f64(&mut bytes, 2.6);
    push_f64(&mut bytes, -0.3);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_non_windowed_nested_array_real_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Top-level: array of type_id=2.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "array2d");
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 3);
    // Inner type: array of real.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "inner");
    push_u32(&mut bytes, 11);
    push_u32(&mut bytes, 3);
    // Sweep type.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 3);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 3);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(out)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    // Point 0
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 0.0);
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 1.0);
    push_f64(&mut bytes, 2.0);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 3.0);
    push_f64(&mut bytes, 4.0);

    // Point 1
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 1.0);
    push_f64(&mut bytes, 0.0);
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 1.5);
    push_f64(&mut bytes, 2.5);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 3.5);
    push_f64(&mut bytes, 4.5);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_windowed_array_of_struct_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    push_named_int(&mut bytes, "PSF traces", 1);
    push_named_int(&mut bytes, "PSF window size", 128);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Top-level trace: array of struct(type_id=2).
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "array_of_struct");
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 3);
    // Struct element type.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "elem");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 3);
    push_string(&mut bytes, "dc");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 4);
    push_string(&mut bytes, "ac");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 12);
    push_u32(&mut bytes, 18);
    // Sweep type.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 5);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 5);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(out)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 1.0);

    let mut trace_payload = Vec::new();
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 1.0);
    push_f64(&mut trace_payload, 2.0);
    push_f64(&mut trace_payload, 0.5);
    push_f64(&mut trace_payload, 1.1);
    push_f64(&mut trace_payload, 2.1);
    push_f64(&mut trace_payload, 0.6);
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 1.5);
    push_f64(&mut trace_payload, 2.5);
    push_f64(&mut trace_payload, -0.2);
    push_f64(&mut trace_payload, 1.6);
    push_f64(&mut trace_payload, 2.6);
    push_f64(&mut trace_payload, -0.3);
    push_windowed_trace_payload(&mut bytes, &trace_payload, 128);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_windowed_nested_array_real_psf() -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    push_named_int(&mut bytes, "PSF traces", 1);
    push_named_int(&mut bytes, "PSF window size", 112);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    // Top-level: array of type_id=2.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "array2d");
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 3);
    // Inner type: array of real.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_string(&mut bytes, "inner");
    push_u32(&mut bytes, 11);
    push_u32(&mut bytes, 3);
    // Sweep type.
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 3);
    push_string(&mut bytes, "real");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 11);
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 3);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(out)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 2);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 1.0);

    let mut trace_payload = Vec::new();
    push_u32(&mut trace_payload, 2);
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 1.0);
    push_f64(&mut trace_payload, 2.0);
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 3.0);
    push_f64(&mut trace_payload, 4.0);
    push_u32(&mut trace_payload, 2);
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 1.5);
    push_f64(&mut trace_payload, 2.5);
    push_u32(&mut trace_payload, 2);
    push_f64(&mut trace_payload, 3.5);
    push_f64(&mut trace_payload, 4.5);
    push_windowed_trace_payload(&mut bytes, &trace_payload, 112);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);
    bytes
}

pub(crate) fn build_non_windowed_array_of_struct_bare_descriptor_psf() -> Vec<u8> {
    let mut bytes = build_non_windowed_array_of_struct_psf();
    patch_top_type_array_descriptor(&mut bytes, DataType::Struct.to_u32());
    bytes
}

pub(crate) fn build_non_windowed_nested_array_real_bare_descriptor_psf() -> Vec<u8> {
    let mut bytes = build_non_windowed_nested_array_real_psf();
    patch_top_type_array_descriptor(&mut bytes, DataType::Array.to_u32());
    bytes
}

pub(crate) fn build_windowed_array_of_struct_bare_descriptor_psf() -> Vec<u8> {
    let mut bytes = build_windowed_array_of_struct_psf();
    patch_top_type_array_descriptor(&mut bytes, DataType::Struct.to_u32());
    bytes
}

pub(crate) fn build_windowed_nested_array_real_bare_descriptor_psf() -> Vec<u8> {
    let mut bytes = build_windowed_nested_array_real_psf();
    patch_top_type_array_descriptor(&mut bytes, DataType::Array.to_u32());
    bytes
}

fn build_simple_non_windowed_psf(sample_encoding: SampleEncoding) -> Vec<u8> {
    let mut bytes = Vec::new();

    let header_start = bytes.len();
    push_u32(&mut bytes, 0);
    let header_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_named_int(&mut bytes, "PSF sweep points", 2);
    let header_end = bytes.len() as u32;
    patch_u32(&mut bytes, header_eofs_pos, header_end);

    let types_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let types_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 1);
    push_string(&mut bytes, "sigtype");
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, sample_encoding.type_code());
    let types_end = bytes.len() as u32;
    patch_u32(&mut bytes, types_eofs_pos, types_end);

    let sweep_start = bytes.len();
    push_u32(&mut bytes, 0);
    let sweep_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 100, "time", 1);
    let sweep_end = bytes.len() as u32;
    patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

    let trace_start = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 22);
    let trace_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 16);
    push_signal_ref(&mut bytes, 200, "V(out)", 1);
    let trace_end = bytes.len() as u32;
    patch_u32(&mut bytes, trace_eofs_pos, trace_end);

    let value_start = bytes.len();
    push_u32(&mut bytes, 0);
    let value_eofs_pos = bytes.len();
    push_u32(&mut bytes, 0);

    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 0.0);
    push_f64(&mut bytes, 0.0);
    push_sample(&mut bytes, sample_encoding, 0);

    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 0);
    push_f64(&mut bytes, 1.0);
    push_f64(&mut bytes, 0.0);
    push_sample(&mut bytes, sample_encoding, 1);

    let value_end = bytes.len() as u32;
    patch_u32(&mut bytes, value_eofs_pos, value_end);

    let toc_offset = bytes.len();
    for (kind, start) in [
        (0u32, header_start),
        (1u32, types_start),
        (2u32, sweep_start),
        (3u32, trace_start),
        (4u32, value_start),
    ] {
        push_u32(&mut bytes, kind);
        push_u32(&mut bytes, start as u32);
    }
    bytes.extend_from_slice(&[0u8; 8]);
    push_u32(&mut bytes, toc_offset as u32);

    bytes
}

fn push_sample(bytes: &mut Vec<u8>, sample_encoding: SampleEncoding, sample_idx: usize) {
    match sample_encoding {
        SampleEncoding::Real => match sample_idx {
            0 => push_f64(bytes, 1.0),
            1 => push_f64(bytes, 2.0),
            _ => unreachable!("test helper has exactly two samples"),
        },
        SampleEncoding::Complex => match sample_idx {
            0 => {
                push_f64(bytes, 1.0);
                push_f64(bytes, 0.5);
            }
            1 => {
                push_f64(bytes, 2.0);
                push_f64(bytes, -0.25);
            }
            _ => unreachable!("test helper has exactly two samples"),
        },
        SampleEncoding::Int8 => match sample_idx {
            0 => push_u8_padded(bytes, 7),
            1 => push_u8_padded(bytes, 255),
            _ => unreachable!("test helper has exactly two samples"),
        },
        SampleEncoding::Int32 => match sample_idx {
            0 => push_i32(bytes, 1024),
            1 => push_i32(bytes, -2),
            _ => unreachable!("test helper has exactly two samples"),
        },
        SampleEncoding::UnknownWord => match sample_idx {
            0 => push_u32(bytes, 0xDEAD_BEEF),
            1 => push_u32(bytes, 0xC001_D00D),
            _ => unreachable!("test helper has exactly two samples"),
        },
    }
}

fn push_signal_ref(bytes: &mut Vec<u8>, id: u32, name: &str, type_id: u32) {
    push_u32(bytes, id);
    push_string(bytes, name);
    push_u32(bytes, type_id);
}

fn push_named_int(bytes: &mut Vec<u8>, name: &str, value: u32) {
    push_u32(bytes, 34);
    push_string(bytes, name);
    push_u32(bytes, value);
}

fn push_string(bytes: &mut Vec<u8>, s: &str) {
    push_u32(bytes, s.len() as u32);
    bytes.extend_from_slice(s.as_bytes());
    let pad = (4 - (s.len() % 4)) % 4;
    bytes.extend(std::iter::repeat(0u8).take(pad));
}

fn push_f64(bytes: &mut Vec<u8>, value: f64) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_i32(bytes: &mut Vec<u8>, value: i32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_u8_padded(bytes: &mut Vec<u8>, value: u8) {
    bytes.push(value);
    bytes.extend_from_slice(&[0u8; 3]);
}

fn push_windowed_trace_payload(bytes: &mut Vec<u8>, payload: &[u8], window_size: usize) {
    assert!(
        payload.len() <= window_size,
        "payload {} exceeds window_size {}",
        payload.len(),
        window_size
    );
    let mut window_block = vec![0u8; window_size];
    let start = window_size - payload.len();
    window_block[start..].copy_from_slice(payload);
    bytes.extend_from_slice(&window_block);
}

fn patch_top_type_array_descriptor(bytes: &mut [u8], new_descriptor: u32) {
    let toc = parse_toc(bytes).expect("fixture must contain valid TOC");
    let entry = toc
        .section(SectionKind::Type)
        .expect("fixture must contain type section");

    assert!(
        entry.start + 16 <= bytes.len(),
        "type section header must exist"
    );
    let block_type = peek_u32(&bytes[entry.start + 8..entry.start + 12]);
    assert_eq!(block_type, 22, "type section must be block 22");

    let mut idx = entry.start + 16;
    assert_eq!(
        peek_u32(&bytes[idx..idx + 4]),
        16,
        "first type item must be block 16"
    );
    idx += 4; // block
    idx += 4; // type id

    let name_len = peek_u32(&bytes[idx..idx + 4]) as usize;
    idx += 4;
    let name_pad = (4 - (name_len % 4)) % 4;
    idx += name_len + name_pad;

    patch_u32(bytes, idx, new_descriptor);
}

fn patch_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
}
