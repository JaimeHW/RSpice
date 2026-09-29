//! Touchstone datasets from completed sampled S-parameter output.

use crate::waveform_io::{SignalType, TouchstoneError, WaveformDataset, WaveformSignal};
use rspice_results::waveform::WaveformData;
use std::collections::HashMap;

/// Retained port references take precedence over the supplied configuration fallback.
pub fn project_sparameter_waveforms(
    frequencies: &[f64],
    waveforms: &HashMap<String, WaveformData>,
    references: Option<&[f64]>,
    noise_temperature: Option<f64>,
    z0: f64,
    z0_by_port: &[f64],
    touchstone_version: usize,
) -> Result<WaveformDataset, TouchstoneError> {
    if frequencies.is_empty() {
        return Err("frequency vector is empty".into());
    }

    let mut entries: std::collections::HashMap<(usize, usize), &WaveformData> =
        std::collections::HashMap::new();
    let mut max_port = 0usize;
    for (name, waveform) in waveforms {
        let matrix_index = parse_sparameter_waveform_name(name)
            .or_else(|| parse_sparameter_waveform_name(&waveform.name));
        let Some((row, col)) = matrix_index else {
            continue;
        };
        if entries.insert((row, col), waveform).is_some() {
            return Err(format!("duplicate S-parameter waveform for S{}{}", row, col).into());
        }
        max_port = max_port.max(row).max(col);
    }
    if max_port == 0 {
        return Err("no complete S-parameter matrix waveforms found".into());
    }
    let z0_by_port = references.unwrap_or(z0_by_port);
    let port_references = if z0_by_port.is_empty() {
        vec![z0; max_port]
    } else if z0_by_port.len() == max_port {
        z0_by_port.to_vec()
    } else {
        return Err(format!(
            "expected {} per-port reference values, got {}",
            max_port,
            z0_by_port.len()
        )
        .into());
    };
    for (idx, value) in port_references.iter().enumerate() {
        if !value.is_finite() || *value <= 0.0 {
            return Err(format!(
                "invalid Touchstone reference impedance for port {}",
                idx + 1
            )
            .into());
        }
    }
    let has_non_uniform_reference = port_references
        .iter()
        .any(|value| (*value - port_references[0]).abs() > 1e-18);
    if touchstone_version < 2 && has_non_uniform_reference {
        return Err("Touchstone v1 export does not support per-port reference impedance".into());
    }

    let mut dataset = WaveformDataset::new("S-Parameters");
    dataset.analysis = "S-Parameter".to_string();
    dataset
        .metadata
        .insert("z0".to_string(), format!("{}", port_references[0]));
    dataset.metadata.insert(
        "z0_ports".to_string(),
        port_references
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(","),
    );
    dataset
        .metadata
        .insert("num_ports".to_string(), max_port.to_string());
    dataset.metadata.insert(
        "touchstone_version".to_string(),
        touchstone_version.to_string(),
    );

    let mut x = WaveformSignal::new("frequency", SignalType::Frequency);
    x.data = frequencies.to_vec();
    dataset.set_x(x);

    for row in 1..=max_port {
        for col in 1..=max_port {
            let name = sparameter_name(row, col, max_port);
            let waveform = entries
                .get(&(row, col))
                .copied()
                .ok_or_else(|| format!("missing {} waveform", name))?;
            let imag = waveform
                .y_imag
                .as_ref()
                .ok_or_else(|| format!("{} waveform is missing imaginary component", name))?;
            if waveform.y_values.len() != frequencies.len() || imag.len() != frequencies.len() {
                return Err(format!(
                    "{} waveform length mismatch (freq={}, re={}, im={})",
                    name,
                    frequencies.len(),
                    waveform.y_values.len(),
                    imag.len()
                )
                .into());
            }
            push_complex_signal_pair(&mut dataset, &name, waveform)?;
        }
    }

    if let Some(temperature) = noise_temperature {
        dataset.metadata.insert(
            "noise_reference_temperature_kelvin".into(),
            temperature.to_string(),
        );
        for name in ["Fmin", "Rn", "Sopt"] {
            let waveform = waveforms.get(name).ok_or_else(|| format!("Touchstone noise export requires two-port {name} data; use a result bundle for full port-noise covariance"))?;
            if waveform.y_values.len() != frequencies.len() {
                return Err(format!(
                    "Touchstone noise waveform {name} does not match the frequency grid"
                )
                .into());
            }
            if name == "Sopt" {
                if waveform
                    .y_imag
                    .as_ref()
                    .is_none_or(|values| values.len() != frequencies.len())
                {
                    return Err(
                        "Touchstone noise export requires both complete Sopt components".into(),
                    );
                }
                push_complex_signal_pair(&mut dataset, name, waveform)?;
            } else {
                let mut signal = WaveformSignal::new(name, SignalType::Unknown);
                signal.unit = if name == "Rn" { "Ω" } else { "1" }.into();
                signal.data = waveform.y_values.clone();
                dataset.add_signal(signal);
            }
        }
    }
    Ok(dataset)
}

fn push_complex_signal_pair(
    dataset: &mut WaveformDataset,
    name: &str,
    waveform: &WaveformData,
) -> Result<(), TouchstoneError> {
    let imag = waveform
        .y_imag
        .as_ref()
        .ok_or_else(|| format!("{} waveform is missing imaginary component", name))?;

    let mut real_signal = WaveformSignal::new(format!("{}_RE", name), SignalType::SParameter);
    real_signal.data = waveform.y_values.clone();
    dataset.add_signal(real_signal);

    let mut imag_signal = WaveformSignal::new(format!("{}_IM", name), SignalType::SParameter);
    imag_signal.data = imag.clone();
    dataset.add_signal(imag_signal);

    Ok(())
}

fn parse_sparameter_waveform_name(name: &str) -> Option<(usize, usize)> {
    let normalized = name.trim().to_ascii_uppercase().replace(' ', "");
    let rest = normalized.strip_prefix('S')?;
    if let Some(inner) = rest
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
    {
        let (row, col) = inner.split_once(',')?;
        let row = row.trim().parse::<usize>().ok()?;
        let col = col.trim().parse::<usize>().ok()?;
        return (row > 0 && col > 0).then_some((row, col));
    }
    if let Some((row, col)) = rest.split_once('_') {
        let row = row.trim().parse::<usize>().ok()?;
        let col = col.trim().parse::<usize>().ok()?;
        return (row > 0 && col > 0).then_some((row, col));
    }
    if rest.len() == 2 && rest.chars().all(|ch| ch.is_ascii_digit()) {
        let row = rest[0..1].parse::<usize>().ok()?;
        let col = rest[1..2].parse::<usize>().ok()?;
        return Some((row, col));
    }
    None
}

fn sparameter_name(row: usize, col: usize, num_ports: usize) -> String {
    if num_ports <= 9 {
        format!("S{}{}", row, col)
    } else {
        format!("S{}_{}", row, col)
    }
}
