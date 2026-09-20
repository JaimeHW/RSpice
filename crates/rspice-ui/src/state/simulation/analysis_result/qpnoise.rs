//! Noise spectra keep physical output axes, dimensions and undefined referral gaps.
use super::qpxf::QpxfDisplayTrace;
use super::*;
use rspice_core::engine::{QpnoiseAnalysisResult, QpnoiseObservation, QpnoiseValue, QpxfQuantity};

pub(crate) fn output_label(response: &QpnoiseAnalysisResult, index: usize) -> String {
    let output = &response.metadata.request.outputs[index];
    let name = match &output.observation {
        QpnoiseObservation::Voltage { positive, negative } => format!("V({positive},{negative})"),
        QpnoiseObservation::BranchCurrent { branch } => format!("I({branch})"),
    };
    format!("output {}: {name} {:?}", index + 1, output.lattice)
}
fn units(current: bool) -> (&'static str, &'static str) {
    if current {
        ("A²/Hz", "A/√Hz")
    } else {
        ("V²/Hz", "V/√Hz")
    }
}
fn segments(
    traces: &mut Vec<QpxfDisplayTrace>,
    name: String,
    unit: &'static str,
    frequencies: &[f64],
    values: &[QpnoiseValue],
    root: bool,
) {
    let mut start = 0;
    let mut segment = 0;
    while start < values.len() {
        if !matches!(values[start], QpnoiseValue::Finite(_)) {
            start += 1;
            continue;
        }
        let mut end = start + 1;
        while end < values.len() && matches!(values[end], QpnoiseValue::Finite(_)) {
            end += 1;
        }
        segment += 1;
        let suffix = if start == 0 && end == values.len() {
            String::new()
        } else {
            format!(" [segment {segment}]")
        };
        let real = values[start..end]
            .iter()
            .filter_map(|v| match v {
                QpnoiseValue::Finite(v) => Some(if root { v.sqrt() } else { *v }),
                _ => None,
            })
            .collect();
        traces.push(QpxfDisplayTrace {
            name: format!("{name}{suffix}"),
            unit,
            frequencies: frequencies[start..end].to_vec(),
            real,
            imaginary: None,
        });
        start = end;
    }
}
fn density(
    traces: &mut Vec<QpxfDisplayTrace>,
    name: String,
    current: bool,
    frequencies: &[f64],
    values: &[QpnoiseValue],
) {
    let (power, amplitude) = units(current);
    segments(
        traces,
        format!("PSD({name})"),
        power,
        frequencies,
        values,
        false,
    );
    segments(
        traces,
        format!("ASD({name})"),
        amplitude,
        frequencies,
        values,
        true,
    );
}
impl AnalysisResultPayload {
    pub(crate) fn qpnoise_display_traces(
        response: &QpnoiseAnalysisResult,
    ) -> Result<Vec<QpxfDisplayTrace>, String> {
        response
            .validate_retained_payload_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|e| e.to_string())?;
        let mut traces = Vec::new();
        let request = &response.metadata.request;
        let n = request.outputs.len();
        for (row, spectrum) in response.outputs.iter().enumerate() {
            let label = output_label(response, row);
            let current = matches!(
                request.outputs[row].observation,
                QpnoiseObservation::BranchCurrent { .. }
            );
            let values: Vec<_> = response
                .total_covariances
                .iter()
                .map(|c| QpnoiseValue::Finite(c.values[row * n + row].re))
                .collect();
            density(
                &mut traces,
                label.clone(),
                current,
                &spectrum.frequencies_hz,
                &values,
            );
            for (source, law) in response.sources.iter().enumerate() {
                let values: Vec<_> = response
                    .points
                    .iter()
                    .map(|p| {
                        QpnoiseValue::Finite(p.source_covariances[source].values[row * n + row].re)
                    })
                    .collect();
                density(
                    &mut traces,
                    format!("{label}; source {}: {}", source + 1, law.name),
                    current,
                    &spectrum.frequencies_hz,
                    &values,
                );
            }
            if let (Some(input), Some(values)) =
                (&response.metadata.input_source, &spectrum.input_noise)
            {
                density(
                    &mut traces,
                    format!("input {} referred from {label}", input.name),
                    input.quantity == QpxfQuantity::Current,
                    &spectrum.frequencies_hz,
                    values,
                );
            }
            if let Some(values) = &spectrum.noise_figure_db {
                segments(
                    &mut traces,
                    format!("NF({label})"),
                    "dB",
                    &spectrum.frequencies_hz,
                    values,
                    false,
                );
            }
            for column in row + 1..n {
                let other_current = matches!(
                    request.outputs[column].observation,
                    QpnoiseObservation::BranchCurrent { .. }
                );
                let unit = if current != other_current {
                    "V·A/Hz"
                } else {
                    units(current).0
                };
                let values: Vec<_> = response
                    .total_covariances
                    .iter()
                    .map(|c| c.values[row * n + column])
                    .collect();
                if values.iter().any(|v| !v.re.hypot(v.im).is_finite()) {
                    return Err("QPNOISE cross-spectrum magnitude overflowed".into());
                }
                let (real, imaginary) = values.iter().map(|v| (v.re, v.im)).unzip();
                traces.push(QpxfDisplayTrace {
                    name: format!("C({label}; {})", output_label(response, column)),
                    unit,
                    frequencies: spectrum.frequencies_hz.clone(),
                    real,
                    imaginary: Some(imaginary),
                });
            }
        }
        Ok(traces)
    }
    /// Conservative resident-byte accounting of the primary and derived arrays.
    pub(crate) fn qpnoise_response_bytes(r: &QpnoiseAnalysisResult) -> usize {
        use rspice_core::analysis::quasi_periodic::QuasiPeriodicNoiseSpectrum as Spectrum;
        let add = |a: usize, b: usize| a.saturating_add(b);
        let matrix = |c: &rspice_core::analysis::quasi_periodic::QuasiPeriodicNoiseCovariance| {
            c.values
                .len()
                .saturating_mul(16)
                .saturating_add(c.roundoff_bounds.len().saturating_mul(8))
        };
        let mut bytes = 0usize;
        for source in &r.sources {
            bytes = bytes
                .saturating_add(source.name.len())
                .saturating_add(source.injections.len().saturating_mul(32));
            bytes = bytes.saturating_add(match &source.spectrum {
                Spectrum::White { density, .. } => density.len().saturating_mul(8),
                Spectrum::PowerLaw {
                    modulation,
                    modulation_lattices,
                    ..
                } => modulation.len().saturating_mul(16).saturating_add(
                    modulation_lattices.as_ref().map_or(0, |ts| {
                        ts.iter().map(|t| t.len().saturating_mul(4)).fold(0, add)
                    }),
                ),
            });
        }
        for point in &r.points {
            for adjoint in &point.adjoints {
                bytes = bytes.saturating_add(
                    adjoint
                        .sensitivities
                        .iter()
                        .map(|row| row.len().saturating_mul(16))
                        .fold(0, add),
                );
            }
            bytes = bytes.saturating_add(point.source_covariances.iter().map(matrix).fold(0, add));
        }
        bytes = bytes.saturating_add(r.total_covariances.iter().map(matrix).fold(0, add));
        for output in &r.outputs {
            // Axis, complex referral, two status-bearing quantities and summary storage.
            bytes = bytes
                .saturating_add(output.frequencies_hz.len().saturating_mul(80))
                .saturating_add(r.sources.len().saturating_mul(64));
        }
        let m = &r.metadata;
        bytes = bytes.saturating_add(
            m.input_lattices
                .iter()
                .map(|t| t.len().saturating_mul(4))
                .fold(0, add),
        );
        bytes = bytes.saturating_add(
            m.node_names
                .iter()
                .chain(&m.branch_names)
                .map(String::len)
                .fold(0, add),
        );
        bytes.saturating_add(m.request.frequencies_hz.len().saturating_mul(8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn qpnoise_result_plot_segments_do_not_bridge_undefined_samples() {
        use rspice_core::engine::QpnoiseUnavailable;
        let mut traces = Vec::new();
        segments(
            &mut traces,
            "input".into(),
            "V²/Hz",
            &[1.0, 2.0, 3.0, 4.0],
            &[
                QpnoiseValue::Finite(1.0),
                QpnoiseValue::Unavailable(QpnoiseUnavailable::ZeroInputTransfer),
                QpnoiseValue::Finite(4.0),
                QpnoiseValue::Finite(9.0),
            ],
            false,
        );
        assert_eq!(traces.len(), 2);
        assert_eq!(traces[0].frequencies, vec![1.0]);
        assert_eq!(traces[1].frequencies, vec![3.0, 4.0]);
        assert_ne!(traces[0].name, traces[1].name);
    }
}
