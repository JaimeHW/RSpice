//! Exact QPAC display traces derived from retained complex MNA responses.
use super::*;
use rspice_core::engine::{QpacAnalysisResult, QpacInputQuantity};

type Trace = (String, &'static str, Vec<rspice_core::Complex64>);

impl AnalysisResultPayload {
    pub(crate) fn qpac_display_traces(result: &QpacAnalysisResult) -> Result<Vec<Trace>, String> {
        let grid = result
            .validate_retained_payload_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|e| e.to_string())?;
        let metadata = &result.metadata;
        let tuple = &metadata.request.output_lattice;
        let index = grid.index_of(tuple).expect("validated QPAC tuple");
        let drive = metadata.drive();
        let suffix = format!(" [k={tuple:?}]");
        let mut traces = Vec::new();
        for row in 0..metadata.node_names.len() + metadata.branch_names.len() {
            let (name, unit) = if row < metadata.node_names.len() {
                (format!("V({}){suffix}", metadata.node_names[row]), "V")
            } else {
                (
                    format!(
                        "I({}){suffix}",
                        metadata.branch_names[row - metadata.node_names.len()]
                    ),
                    "A",
                )
            };
            let values: Vec<_> = result
                .unit_solutions
                .iter()
                .map(|s| s.spectra[row][index] * drive)
                .collect();
            if values
                .iter()
                .any(|v| !v.re.is_finite() || !v.im.is_finite() || !v.norm().is_finite())
            {
                return Err("QPAC displayed response overflowed at the authored drive".into());
            }
            traces.push((name, unit, values));
        }
        let output = format!(
            "V({},{})",
            metadata.request.output_node, metadata.request.output_ref
        );
        let (input_unit, gain_unit) = match metadata.input_quantity {
            QpacInputQuantity::Voltage => ("V", "1"),
            QpacInputQuantity::Current => ("I", "Ω"),
        };
        traces.push((
            format!("{output}{suffix}"),
            "V",
            result.output_response.clone(),
        ));
        traces.push((
            format!(
                "H({output}/{input_unit}({})) [in={:?}; out={tuple:?}]",
                metadata.request.input_source, metadata.request.input_lattice
            ),
            gain_unit,
            result.output_transfer.clone(),
        ));
        if traces
            .iter()
            .flat_map(|(_, _, values)| values)
            .any(|v| !v.norm().is_finite())
        {
            return Err("QPAC displayed magnitude overflowed".into());
        }
        Ok(traces)
    }
}

pub(super) fn validate_display(
    result: &QpacAnalysisResult,
    waveforms: &[WaveformData],
) -> Result<(), String> {
    let traces = AnalysisResultPayload::qpac_display_traces(result)?;
    if waveforms.len() != traces.len().saturating_mul(2) {
        return Err("QPAC saved display is missing a magnitude or phase trace".into());
    }
    for (name, unit, coefficients) in traces {
        let magnitude = waveforms
            .iter()
            .find(|w| w.name == format!("|{name}|"))
            .ok_or_else(|| format!("QPAC magnitude for {name} is missing"))?;
        let phase = waveforms
            .iter()
            .find(|w| w.name == format!("phase({name})"))
            .ok_or_else(|| format!("QPAC phase for {name} is missing"))?;
        let complex = magnitude
            .complex
            .as_ref()
            .ok_or_else(|| format!("QPAC complex response for {name} is missing"))?;
        if magnitude.unit.as_deref() != Some(unit)
            || phase.unit.as_deref() != Some("°")
            || phase.complex.is_some()
            || complex.source_name != name
            || *magnitude.x != result.metadata.request.offsets_hz
            || magnitude.x != phase.x
            || [
                magnitude.y.len(),
                phase.y.len(),
                complex.real.len(),
                complex.imag.len(),
            ]
            .iter()
            .any(|n| *n != coefficients.len())
        {
            return Err(format!(
                "QPAC trace shape, offset axis or units differ for {name}"
            ));
        }
        for (i, value) in coefficients.iter().enumerate() {
            if complex.real[i] != value.re
                || complex.imag[i] != value.im
                || magnitude.y[i] != value.re.hypot(value.im)
                || phase.y[i] != value.im.atan2(value.re).to_degrees()
            {
                return Err(format!(
                    "QPAC trace {name} differs from its retained complex response"
                ));
            }
        }
    }
    Ok(())
}
