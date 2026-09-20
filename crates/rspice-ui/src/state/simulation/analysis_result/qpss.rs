//! Saved QPSS displays are exact projections of their retained MNA spectrum.
use super::*;

pub(super) fn validate_display(
    point: &rspice_core::engine::QpssOperatingPoint,
    waveforms: &[WaveformData],
) -> Result<(), String> {
    let grid = point
        .validate_retained_payload_with_abort(
            &rspice_core::ResourceLimits::default(),
            &rspice_core::NoAbort,
        )
        .map_err(|error| error.to_string())?;
    let mut indices: Vec<_> = grid
        .frequencies_hz()
        .iter()
        .enumerate()
        .filter_map(|(index, frequency)| (*frequency >= 0.0).then_some(index))
        .collect();
    indices.sort_by(|a, b| grid.frequencies_hz()[*a].total_cmp(&grid.frequencies_hz()[*b]));
    if waveforms.len() != point.spectra().len().saturating_mul(2) {
        return Err("QPSS saved display is missing a magnitude or phase trace".into());
    }
    for (row, coefficients) in point.spectra().iter().enumerate() {
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
        let magnitude = waveforms
            .iter()
            .find(|trace| trace.name == format!("|{name}|"))
            .ok_or_else(|| format!("QPSS magnitude trace for {name} is missing"))?;
        let phase = waveforms
            .iter()
            .find(|trace| trace.name == format!("phase({name})"))
            .ok_or_else(|| format!("QPSS phase trace for {name} is missing"))?;
        let complex = magnitude
            .complex
            .as_ref()
            .ok_or_else(|| format!("QPSS complex spectrum for {name} is missing"))?;
        if magnitude.unit.as_deref() != Some(unit)
            || phase.unit.as_deref() != Some("°")
            || phase.complex.is_some()
            || complex.source_name != name
            || [
                magnitude.x.len(),
                magnitude.y.len(),
                phase.x.len(),
                phase.y.len(),
                complex.real.len(),
                complex.imag.len(),
            ]
            .iter()
            .any(|length| *length != indices.len())
        {
            return Err(format!(
                "QPSS trace shape, identity or unit differs for {name}"
            ));
        }
        for (display, &index) in indices.iter().enumerate() {
            let value = coefficients[index] * if index == grid.dc_index() { 1.0 } else { 2.0 };
            let frequency = grid.frequencies_hz()[index];
            if magnitude.x[display] != frequency
                || phase.x[display] != frequency
                || complex.real[display] != value.re
                || complex.imag[display] != value.im
                || magnitude.y[display] != value.re.hypot(value.im)
                || phase.y[display] != value.im.atan2(value.re).to_degrees()
            {
                return Err(format!(
                    "QPSS saved trace {name} differs from its retained signed spectrum"
                ));
            }
        }
    }
    Ok(())
}
