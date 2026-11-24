//! MAT-file Level 4: what MATLAB writes with `save -v4` and SciPy with `savemat(format='4')`.
//!
//! MathWorks documents the layout in *MAT-File Format*, "Level 4 MAT-File Format". A file is
//! its variables one after another, each a 20-byte header of five 32-bit integers (type, rows,
//! columns, imaginary flag, name length with its terminating NUL), the name, then the real part
//! and, when the flag is set, the imaginary part, both column by column. The type is the
//! decimal `MOPT`: `M` the byte order (0 little-endian, 1 big-endian IEEE), `O` always 0, `P` the
//! element type (double, single, int32, int16, uint16, uint8) and `T` the matrix kind (0 full
//! numeric, 1 text, 2 sparse).

use super::{MatlabReadError, MatlabReadFailure, NamedArray, adapter_error};

const HEADER_BYTES: usize = 20;
/// The bytes of each element type `P` names, in its order.
const ELEMENT_BYTES: [usize; 6] = [8, 4, 4, 2, 2, 1];

/// One variable's header, read in the byte order its type states.
struct Header {
    big_endian: bool,
    element: usize,
    kind: i32,
    rows: usize,
    columns: usize,
    complex: bool,
    name_bytes: usize,
}

fn header(bytes: &[u8]) -> Option<Header> {
    let field = |index: usize, big_endian: bool| {
        let word: [u8; 4] = bytes.get(index * 4..index * 4 + 4)?.try_into().ok()?;
        Some(if big_endian {
            i32::from_be_bytes(word)
        } else {
            i32::from_le_bytes(word)
        })
    };
    // The byte order is the one that reads the type as a type: `M` is 0 for little-endian
    // and 1 for big-endian, and no other order makes `MOPT` a small decimal of that form.
    [false, true].into_iter().find_map(|big_endian| {
        let kind = field(0, big_endian)?;
        let (order, rest) = (kind / 1000, kind % 1000);
        let (zero, element, matrix) = (rest / 100, rest % 100 / 10, rest % 10);
        let rows = usize::try_from(field(1, big_endian)?).ok()?;
        let columns = usize::try_from(field(2, big_endian)?).ok()?;
        let complex = field(3, big_endian)?;
        let name_bytes = usize::try_from(field(4, big_endian)?).ok()?;
        (kind >= 0
            && order == i32::from(big_endian)
            && zero == 0
            && (0..=5).contains(&element)
            && (0..=2).contains(&matrix)
            && matches!(complex, 0 | 1)
            && name_bytes >= 1)
            .then_some(Header {
                big_endian,
                element: ELEMENT_BYTES[element as usize],
                kind: element * 10 + matrix,
                rows,
                columns,
                complex: complex == 1,
                name_bytes,
            })
    })
}

/// Whether `bytes` open as a Level 4 MAT-file: a variable header that reads as one, and a
/// name that ends where it says.
pub(super) fn looks_like_level4(bytes: &[u8]) -> bool {
    header(bytes).is_some_and(|header| {
        bytes
            .get(HEADER_BYTES..HEADER_BYTES + header.name_bytes)
            .is_some_and(|name| {
                name.last() == Some(&0) && name[..name.len() - 1].iter().all(u8::is_ascii_graphic)
            })
    })
}

/// Every variable of a Level 4 MAT-file, as numbers.
pub(super) fn parse(
    bytes: &[u8],
    max_variables: usize,
    format: &str,
) -> Result<Vec<NamedArray>, MatlabReadError> {
    let fail = |reason| adapter_error(format, reason);
    let mut arrays = Vec::new();
    let mut offset = 0;
    while offset < bytes.len() {
        let variable = arrays.len() + 1;
        if arrays.len() == max_variables {
            return Err(fail(MatlabReadFailure::VariableLimit {
                variables: variable,
                limit: max_variables,
            }));
        }
        let header = bytes
            .get(offset..)
            .and_then(header)
            .ok_or_else(|| fail(MatlabReadFailure::Level4Header { variable }))?;
        let name_start = offset + HEADER_BYTES;
        let name = bytes
            .get(name_start..name_start + header.name_bytes)
            .filter(|name| name.last() == Some(&0))
            .and_then(|name| std::str::from_utf8(&name[..name.len() - 1]).ok())
            .ok_or(MatlabReadFailure::Truncated { variable })
            .map_err(fail)?
            .to_owned();
        let what = match header.kind % 10 {
            1 => Some("text"),
            2 => Some("a sparse matrix"),
            _ => None,
        };
        if let Some(what) = what {
            return Err(fail(MatlabReadFailure::NotNumeric { name, what }));
        }
        let count = header.rows.checked_mul(header.columns).ok_or_else(|| {
            fail(MatlabReadFailure::ShapeOverflow {
                name: name.clone(),
                shape: vec![header.rows, header.columns],
            })
        })?;
        let part_bytes = count
            .checked_mul(header.element)
            .ok_or(MatlabReadFailure::Truncated { variable })
            .map_err(fail)?;
        let mut cursor = name_start + header.name_bytes;
        let mut part = || -> Result<Vec<f64>, MatlabReadError> {
            let data = cursor
                .checked_add(part_bytes)
                .and_then(|end| bytes.get(cursor..end))
                .ok_or_else(|| fail(MatlabReadFailure::Truncated { variable }))?;
            cursor += part_bytes;
            Ok(data
                .chunks_exact(header.element)
                .map(|value| element_value(value, header.kind / 10, header.big_endian))
                .collect())
        };
        let real = part()?;
        let imag = header.complex.then(&mut part).transpose()?;
        arrays.push(NamedArray {
            name,
            size: vec![header.rows, header.columns],
            real,
            imag,
        });
        offset = cursor;
    }
    Ok(arrays)
}

fn element_value(bytes: &[u8], element: i32, big_endian: bool) -> f64 {
    macro_rules! read {
        ($kind:ty) => {{
            let word = bytes.try_into().expect("an element is its type's width");
            (if big_endian {
                <$kind>::from_be_bytes(word)
            } else {
                <$kind>::from_le_bytes(word)
            }) as f64
        }};
    }
    match element {
        0 => read!(f64),
        1 => read!(f32),
        2 => read!(i32),
        3 => read!(i16),
        4 => read!(u16),
        _ => f64::from(bytes[0]),
    }
}
