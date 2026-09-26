//! Positive-frequency QPSS display coefficients projected from the full signed spectrum.
use super::*;

impl AnalysisResultPayload {
    pub fn qpss_display_traces(
        point: &rspice_core::engine::QpssOperatingPoint,
    ) -> Result<Vec<qpxf::QpxfDisplayTrace>, String> {
        let grid = point
            .validate_retained_payload_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|e| e.to_string())?;
        let mut indices: Vec<_> = grid
            .frequencies_hz()
            .iter()
            .enumerate()
            .filter_map(|(i, f)| (*f >= 0.0).then_some(i))
            .collect();
        indices.sort_by(|a, b| grid.frequencies_hz()[*a].total_cmp(&grid.frequencies_hz()[*b]));
        point
            .spectra()
            .iter()
            .enumerate()
            .map(|(row, coefficients)| {
                let (name, unit) = if row < point.node_names().len() {
                    (format!("V({})", point.node_names()[row]), "V")
                } else {
                    (
                        format!(
                            "I({})",
                            point.branch_names()[row - point.node_names().len()]
                        ),
                        "A",
                    )
                };
                let values: Vec<_> = indices
                    .iter()
                    .map(|&i| coefficients[i] * if i == grid.dc_index() { 1.0 } else { 2.0 })
                    .collect();
                if values.iter().any(|v| !v.re.hypot(v.im).is_finite()) {
                    return Err("QPSS display magnitude overflowed".into());
                }
                let (real, imaginary) = values.iter().map(|v| (v.re, v.im)).unzip();
                Ok(qpxf::QpxfDisplayTrace {
                    name,
                    unit,
                    frequencies: indices.iter().map(|&i| grid.frequencies_hz()[i]).collect(),
                    real,
                    imaginary: Some(imaginary),
                })
            })
            .collect()
    }
}
