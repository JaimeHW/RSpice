//! One row-major NumPy matrix with its coordinate in column zero.

use num_complex::Complex64;

use super::{MAX_COLUMNS, NamedArray, encode_complex_array, encode_real_array};

/// Encode coordinate and signal columns as one NPY matrix. A complex signal
/// makes the full matrix complex128, because an NPY array has one dtype.
pub fn encode_npy(coordinate: &[f64], signals: &[NamedArray<'_>]) -> Result<Vec<u8>, String> {
    let columns = signals
        .len()
        .checked_add(1)
        .ok_or_else(|| "NumPy matrix has too many columns".to_owned())?;
    if columns > MAX_COLUMNS {
        return Err(format!(
            "This result has {columns} columns; RSpice reads at most {MAX_COLUMNS} from a NumPy source."
        ));
    }
    if coordinate.is_empty() || signals.is_empty() {
        return Err("A NumPy matrix needs coordinate samples and at least one signal.".into());
    }
    for signal in signals {
        if signal.real.len() != coordinate.len()
            || signal
                .imag
                .is_some_and(|imag| imag.len() != coordinate.len())
        {
            return Err(format!(
                "'{}' has {} samples against {} coordinate samples; the export is refused rather than padded or truncated.",
                signal.name,
                signal.real.len(),
                coordinate.len()
            ));
        }
    }

    let rows = coordinate.len();
    let shape = [rows as u64, columns as u64];
    let capacity = rows
        .checked_mul(columns)
        .ok_or_else(|| "NumPy matrix exceeds the supported sample count".to_owned())?;
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
        assert!(encode_npy(&[0.0, 1.0], &[signal]).is_err());
    }
}
