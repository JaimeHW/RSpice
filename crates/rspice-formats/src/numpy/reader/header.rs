//! Admission before npyz's unchecked dimension products and header allocation.

use super::{NumpyReadError, NumpyReadFailure, read_error};
use py_literal::Value;

// A one- or two-dimensional numeric array needs only a small flat dictionary.
// Bound both parser work and recursion, independently of the sample budget.
const MAX_HEADER_BYTES: usize = 64 * 1024;
const MAX_HEADER_DEPTH: usize = 16;

fn invalid(format: &str, detail: impl Into<String>) -> NumpyReadError {
    read_error(
        format,
        NumpyReadFailure::Header(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            detail.into(),
        )),
    )
}

fn header_text<'a>(bytes: &'a [u8], format: &str) -> Result<&'a str, NumpyReadError> {
    if bytes.get(..6) != Some(b"\x93NUMPY") {
        return Err(invalid(format, "missing NPY signature"));
    }
    let (prefix, length_bytes) = match bytes.get(6..8) {
        Some([1, 0]) => (10, 2),
        Some([2 | 3, 0]) => (12, 4),
        _ => return Err(invalid(format, "unsupported or truncated NPY version")),
    };
    let raw = bytes
        .get(8..prefix)
        .ok_or_else(|| invalid(format, "truncated NPY header length"))?;
    let length = if length_bytes == 2 {
        u16::from_le_bytes([raw[0], raw[1]]) as usize
    } else {
        usize::try_from(u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]))
            .map_err(|_| invalid(format, "NPY header length exceeds this platform"))?
    };
    if length > MAX_HEADER_BYTES {
        return Err(invalid(format, "NPY header exceeds the 65536-byte limit"));
    }
    let header = bytes
        .get(prefix..prefix + length)
        .ok_or_else(|| invalid(format, "NPY header extends beyond the available bytes"))?;
    bound_nesting(header, format)?;
    std::str::from_utf8(header.strip_suffix(b"\n").unwrap_or(header))
        .map_err(|_| invalid(format, "NPY header is not UTF-8"))
}

// Only count containers outside Python strings. Handle escaped delimiters and
// triple-quoted strings so an inert bracket in metadata is never recursion.
fn bound_nesting(bytes: &[u8], format: &str) -> Result<(), NumpyReadError> {
    let mut cursor = 0;
    let mut depth = 0usize;
    while cursor < bytes.len() {
        match bytes[cursor] {
            quote @ (b'\'' | b'"') => {
                let width = if bytes.get(cursor..cursor + 3) == Some(&[quote; 3]) {
                    3
                } else {
                    1
                };
                let delimiter = &bytes[cursor..cursor + width];
                cursor += width;
                loop {
                    if cursor >= bytes.len() {
                        return Err(invalid(format, "unterminated NPY header string"));
                    }
                    if bytes[cursor] == b'\\' {
                        cursor += 2;
                    } else if bytes.get(cursor..cursor + width) == Some(delimiter) {
                        cursor += width;
                        break;
                    } else {
                        cursor += 1;
                    }
                }
            }
            b'[' | b'(' | b'{' => {
                depth += 1;
                if depth > MAX_HEADER_DEPTH {
                    return Err(invalid(format, "NPY header nesting limit exceeded"));
                }
                cursor += 1;
            }
            b']' | b')' | b'}' => {
                depth = depth.saturating_sub(1);
                cursor += 1;
            }
            _ => cursor += 1,
        }
    }
    Ok(())
}

pub(super) fn preflight(bytes: &[u8], format: &str) -> Result<(Vec<usize>, usize), NumpyReadError> {
    let value = header_text(bytes, format)?
        .parse::<Value>()
        .map_err(|error| invalid(format, format!("invalid NPY dictionary: {error}")))?;
    let Value::Dict(entries) = value else {
        return Err(invalid(format, "NPY header must be a dictionary"));
    };
    let mut shape_value = None;
    for (key, value) in entries {
        if key.as_string().is_some_and(|key| key == "shape") && shape_value.replace(value).is_some()
        {
            return Err(invalid(format, "NPY header repeats 'shape'"));
        }
    }
    let dimensions = match shape_value {
        Some(Value::Tuple(dimensions) | Value::List(dimensions)) => dimensions,
        _ => return Err(invalid(format, "NPY shape must be a tuple or list")),
    };
    let shape = dimensions
        .into_iter()
        .map(|dimension| {
            let Value::Integer(integer) = dimension else {
                return Err(invalid(format, "NPY dimensions must be integers"));
            };
            let dimension = u64::try_from(integer)
                .map_err(|_| invalid(format, "NPY dimension is outside the u64 range"))?;
            usize::try_from(dimension)
                .map_err(|_| read_error(format, NumpyReadFailure::DimensionTooLarge(dimension)))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if shape.is_empty() || shape.len() > 2 {
        return Err(read_error(
            format,
            NumpyReadFailure::WaveformDimensions(shape),
        ));
    }
    let count = shape
        .iter()
        .try_fold(1usize, |count, dimension| count.checked_mul(*dimension))
        .ok_or_else(|| {
            read_error(
                format,
                NumpyReadFailure::ShapeProductOverflow(shape.clone()),
            )
        })?;
    Ok((shape, count))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(version: u8, header: &str) -> Vec<u8> {
        let mut bytes = b"\x93NUMPY".to_vec();
        bytes.extend([version, 0]);
        if version == 1 {
            bytes.extend_from_slice(&(header.len() as u16).to_le_bytes());
        } else {
            bytes.extend_from_slice(&(header.len() as u32).to_le_bytes());
        }
        bytes.extend_from_slice(header.as_bytes());
        bytes
    }

    #[test]
    fn declared_header_sizes_are_bounded_before_the_dependency_allocates() {
        for version in [2, 3] {
            let mut bytes = b"\x93NUMPY".to_vec();
            bytes.extend([version, 0]);
            bytes.extend_from_slice(&u32::MAX.to_le_bytes());
            let error = super::super::decode_npy(&bytes, 1024, "numpy_npy").unwrap_err();
            assert!(error.to_string().contains("65536-byte limit"));
        }
        let mut truncated = file(1, "{'shape': (2,)}\n");
        truncated.pop();
        assert!(
            preflight(&truncated, "numpy_npy")
                .unwrap_err()
                .to_string()
                .contains("available bytes")
        );
    }

    #[test]
    fn bounded_headers_support_versions_escaped_keys_and_literal_strings() {
        for version in [1, 2, 3] {
            let bytes = file(
                version,
                r#"{'descr': '<f8', 'fortran_order': False, 'sh\x61pe': (2, 3), 'note': '[\"\'([[[[[[[[[[[[[[[[[[[[[[[[[[[['}"#,
            );
            assert_eq!(preflight(&bytes, "numpy_npy").unwrap(), (vec![2, 3], 6));
        }
        let padded = format!("{{'shape': (2,)}}{}\n", " ".repeat(MAX_HEADER_BYTES - 16));
        assert_eq!(padded.len(), MAX_HEADER_BYTES);
        assert_eq!(preflight(&file(2, &padded), "numpy_npy").unwrap().1, 2);
        bound_nesting(b"'''[\"'([[[[[[[[[[[[[[[[[[[[[[[[[[[['''", "numpy_npy").unwrap();
    }

    #[test]
    fn malformed_and_deep_shapes_are_refused_before_unchecked_products() {
        for header in [
            "{'shape': (2,), 'shape': (3,)}".to_owned(),
            "{'shape': (-1,)}".to_owned(),
            "{'shape': (18446744073709551616,)}".to_owned(),
            "{'shape': (1.5,)}".to_owned(),
            "{'shape': (1, 2, 3)}".to_owned(),
            "{'shape': ()}".to_owned(),
            "{'shape': 'unterminated}".to_owned(),
        ] {
            assert!(
                preflight(&file(1, &header), "numpy_npy").is_err(),
                "{header}"
            );
        }
        let deep = format!("{}0{}", "[".repeat(1000), "]".repeat(1000));
        let error = preflight(&file(1, &deep), "numpy_npy").unwrap_err();
        assert!(error.to_string().contains("nesting limit"));
    }
}
