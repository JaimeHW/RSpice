//! One row-major NumPy matrix with its coordinate in column zero.

use num_complex::Complex64;

use super::{MAX_COLUMNS, NamedArray, NumpyWriteError, encode_complex_array, encode_real_array};

/// Encode coordinate and signal columns as one NPY matrix. A complex signal
/// makes the full matrix complex128, because an NPY array has one dtype.
pub fn encode_npy(
    coordinate: &[f64],
    signals: &[NamedArray<'_>],
) -> Result<Vec<u8>, NumpyWriteError> {
    let columns = signals
        .len()
        .checked_add(1)
        .ok_or(NumpyWriteError::MatrixColumnLimit { columns: None })?;
    if columns > MAX_COLUMNS {
        return Err(NumpyWriteError::MatrixColumnLimit {
            columns: Some(columns),
        });
    }
    if coordinate.is_empty() || signals.is_empty() {
        return Err(NumpyWriteError::EmptyMatrix);
    }
    for signal in signals {
        if signal.real.len() != coordinate.len()
            || signal
                .imag
                .is_some_and(|imag| imag.len() != coordinate.len())
        {
            return Err(NumpyWriteError::SampleCount {
                name: signal.name.to_owned(),
                real: signal.real.len(),
                imag: signal.imag.map(<[f64]>::len),
                coordinate: coordinate.len(),
            });
        }
    }

    let rows = coordinate.len();
    let shape = [rows as u64, columns as u64];
    let capacity = rows
        .checked_mul(columns)
        .ok_or(NumpyWriteError::MatrixSizeOverflow { rows, columns })?;
    if signals.iter().any(|signal| signal.imag.is_some()) {
        let mut values = Vec::with_capacity(capacity);
        for (row, &x) in coordinate.iter().enumerate() {
            values.push(Complex64::new(x, 0.0));
            for signal in signals {
                values.push(Complex64::new(
                    signal.real[row],
                    signal.imag.map_or(0.0, |imag| imag[row]),
                ));
            }
        }
        encode_complex_array(&shape, &values)
    } else {
        let mut values = Vec::with_capacity(capacity);
        for (row, &x) in coordinate.iter().enumerate() {
            values.push(x);
            for signal in signals {
                values.push(signal.real[row]);
            }
        }
        encode_real_array(&shape, &values)
    }
}

#[cfg(test)]
mod tests {
    use super::{NamedArray, encode_npy};

    #[test]
    fn rejects_unbalanced_columns_before_matrix_indexing() {
        let signal = NamedArray {
            name: "V(out)",
            real: &[1.0],
            imag: None,
        };
        let error = encode_npy(&[0.0, 1.0], &[signal]).unwrap_err();
        assert!(
            matches!(error, super::NumpyWriteError::SampleCount { name, real: 1, imag: None, coordinate: 2 } if name == "V(out)")
        );
    }
}
