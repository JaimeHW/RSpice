//! Block codecs: `XOR+shuffle+LZ4` for float columns, `plain` when LZ4 does not shrink a block.
//!
//! `XOR+shuffle` XORs each value's bits with the previous value's (the first with zero), then writes
//! byte 0 of every value, then byte 1, and so on. Neighbouring samples share their sign, exponent and
//! leading mantissa bits, so the high byte planes become long runs LZ4 compresses well. `plain` is
//! the raw little-endian values, so a reader never pays a transform for incompressible data.

#[cfg(test)]
use crate::StoreError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Codec {
    Plain = 0,
    XorShuffleLz4 = 1,
}

impl Codec {}

/// Largest encoded size of a block of `raw_len` bytes, whichever codec wins.
pub(crate) fn max_encoded_len(raw_len: usize) -> usize {
    lz4_flex::block::get_maximum_output_size(raw_len).max(raw_len)
}

/// XOR each value with its predecessor and split the result into eight byte planes.
fn xor_shuffle(values: &[f64], planes: &mut [u8]) {
    let n = values.len();
    let planes = &mut planes[..n * 8];
    let (p0, rest) = planes.split_at_mut(n);
    let (p1, rest) = rest.split_at_mut(n);
    let (p2, rest) = rest.split_at_mut(n);
    let (p3, rest) = rest.split_at_mut(n);
    let (p4, rest) = rest.split_at_mut(n);
    let (p5, rest) = rest.split_at_mut(n);
    let (p6, p7) = rest.split_at_mut(n);
    let mut previous = 0u64;
    for (i, value) in values.iter().enumerate() {
        let bits = value.to_bits();
        let delta = (bits ^ previous).to_le_bytes();
        previous = bits;
        p0[i] = delta[0];
        p1[i] = delta[1];
        p2[i] = delta[2];
        p3[i] = delta[3];
        p4[i] = delta[4];
        p5[i] = delta[5];
        p6[i] = delta[6];
        p7[i] = delta[7];
    }
}

#[cfg(test)]
fn xor_unshuffle(planes: &[u8], out: &mut [f64]) {
    let n = out.len();
    let planes = &planes[..n * 8];
    let (p0, rest) = planes.split_at(n);
    let (p1, rest) = rest.split_at(n);
    let (p2, rest) = rest.split_at(n);
    let (p3, rest) = rest.split_at(n);
    let (p4, rest) = rest.split_at(n);
    let (p5, rest) = rest.split_at(n);
    let (p6, p7) = rest.split_at(n);
    let mut previous = 0u64;
    for (i, value) in out.iter_mut().enumerate() {
        let delta = u64::from_le_bytes([p0[i], p1[i], p2[i], p3[i], p4[i], p5[i], p6[i], p7[i]]);
        previous ^= delta;
        *value = f64::from_bits(previous);
    }
}

fn write_plain(values: &[f64], out: &mut [u8]) {
    for (chunk, value) in out.chunks_exact_mut(8).zip(values) {
        chunk.copy_from_slice(&value.to_le_bytes());
    }
}

/// Encode a float column into `out` (at least [`max_encoded_len`] bytes); returns the codec chosen
/// and the bytes used. `planes` is scratch of at least `values.len() * 8` bytes.
pub(crate) fn encode_f64(values: &[f64], planes: &mut [u8], out: &mut [u8]) -> (Codec, usize) {
    let raw_len = values.len() * 8;
    xor_shuffle(values, planes);
    match lz4_flex::block::compress_into(&planes[..raw_len], out) {
        Ok(len) if len < raw_len => (Codec::XorShuffleLz4, len),
        _ => {
            write_plain(values, &mut out[..raw_len]);
            (Codec::Plain, raw_len)
        }
    }
}

#[cfg(test)]
/// Decode a float column of `out.len()` values. `planes` is reusable scratch.
pub(crate) fn decode_f64(
    codec: Codec,
    encoded: &[u8],
    out: &mut [f64],
    planes: &mut Vec<u8>,
    offset: u64,
) -> Result<(), StoreError> {
    let raw_len = out.len() * 8;
    match codec {
        Codec::Plain => {
            if encoded.len() != raw_len {
                return Err(StoreError::corrupt(
                    offset,
                    "a plain float block has the wrong length",
                ));
            }
            for (value, chunk) in out.iter_mut().zip(encoded.chunks_exact(8)) {
                *value = f64::from_le_bytes(chunk.try_into().unwrap_or_default());
            }
        }
        Codec::XorShuffleLz4 => {
            if planes.len() < raw_len {
                planes.resize(raw_len, 0);
            }
            let used = lz4_flex::block::decompress_into(encoded, &mut planes[..raw_len])
                .map_err(|_| StoreError::corrupt(offset, "a float block does not decompress"))?;
            if used != raw_len {
                return Err(StoreError::corrupt(
                    offset,
                    "a float block decompresses to the wrong length",
                ));
            }
            xor_unshuffle(&planes[..raw_len], out);
        }
    }
    Ok(())
}
