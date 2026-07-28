//! Shared plumbing between result types and the writers in `crate::export`.
//!
//! DC-sweep, transient, and AC results all serialize to the same three formats
//! and all need the same two transformations first: a per-signal series has to
//! be transposed into per-row records, and complex data has to be interleaved
//! into real columns with matching headers. That work lives here rather than
//! being repeated in each family module.

use super::*;

/// Serialize a raw plot, mapping a layout failure onto a Python exception.
pub(super) fn raw_export_bytes(
    plot: &crate::export::RawPlot,
    format: crate::export::RawFormat,
) -> PyResult<Vec<u8>> {
    crate::export::raw_bytes(plot, format).map_err(crate::errors::value_error)
}

/// Write an exported artifact, reporting the caller's path on failure.
///
/// Verbatim Windows prefixes are stripped so the message names the path the
/// caller passed rather than the extended-length form.
pub(super) fn write_export_file(path: &std::path::Path, bytes: &[u8]) -> PyResult<()> {
    crate::export::write_bytes(path, bytes).map_err(|error| {
        pyo3::exceptions::PyOSError::new_err(format!(
            "could not write '{}': {error}",
            crate::errors::public_path_string(path)
        ))
    })
}

/// Reshape column-major real series into CSV rows.
///
/// Callers validate series lengths through `RawPlot`, so a short series here
/// yields a NaN cell rather than a panic on a malformed core result.
pub(super) fn transpose_real(series: &[Vec<rspice_core::Complex64>]) -> Vec<Vec<f64>> {
    let points = series.first().map_or(0, Vec::len);
    (0..points)
        .map(|point| {
            series
                .iter()
                .map(|column| column.get(point).map_or(f64::NAN, |value| value.re))
                .collect()
        })
        .collect()
}

/// Reshape column-major complex series into CSV rows, splitting each complex
/// column into adjacent real and imaginary cells.
pub(super) fn transpose_complex(series: &[Vec<rspice_core::Complex64>]) -> Vec<Vec<f64>> {
    let points = series.first().map_or(0, Vec::len);
    (0..points)
        .map(|point| {
            series
                .iter()
                .flat_map(|column| {
                    let value = column
                        .get(point)
                        .copied()
                        .unwrap_or(rspice_core::Complex64::new(f64::NAN, f64::NAN));
                    [value.re, value.im]
                })
                .collect()
        })
        .collect()
}

/// Split complex column headers into `<name>_real` / `<name>_imag` pairs, so
/// a CSV round-trips a phasor without choosing a lossy polar convention.
pub(super) fn complex_csv_headers(names: &[String]) -> Vec<String> {
    names
        .iter()
        .flat_map(|name| [format!("{name}_real"), format!("{name}_imag")])
        .collect()
}
