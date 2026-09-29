//! Waveform interchange projections over canonical retained results.

use super::{TouchstoneError, WaveformSignal};
use rspice_results::analysis_type::AnalysisType;
use rspice_results::family_metadata::AnalysisResultFamilyMetadata;
use rspice_results::result_import::ResultImportFormat;
use rspice_results::result_import::waveforms::ImportedWaveforms;
use rspice_results::waveform::RetainedWaveform;
use std::collections::BTreeMap;
use std::sync::Arc;

type ComplexComponentColumns = (Option<WaveformSignal>, Option<WaveformSignal>);

/// Exact source data before the host grants import authority or adds presentation.
#[derive(Debug)]
pub struct DecodedTouchstone {
    pub source_format: ResultImportFormat,
    pub data: ImportedWaveforms,
    pub family_metadata: AnalysisResultFamilyMetadata,
    pub notes: Vec<String>,
}

/// Decode network/noise evidence and preserve its independently sampled axes.
pub fn decode_touchstone(
    source_name: &str,
    bytes: &[u8],
    identified_format: ResultImportFormat,
) -> Result<DecodedTouchstone, TouchstoneError> {
    let dataset = super::read_touchstone_bytes(source_name, bytes)?;
    let version = dataset
        .metadata
        .get("touchstone_version")
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or_else(|| "Touchstone adapter did not return a version identity".to_owned())?;
    let parsed_format = if version >= 2 {
        ResultImportFormat::TouchstoneV2
    } else {
        ResultImportFormat::TouchstoneV1
    };
    if identified_format != parsed_format {
        return Err(format!(
            "the source was identified as '{}' but declares '{}'; refusing an ambiguous import",
            identified_format.canonical_id(),
            parsed_format.canonical_id()
        )
        .into());
    }
    let x = dataset
        .x_signal
        .as_ref()
        .ok_or_else(|| "Touchstone source has no frequency axis".to_owned())?;
    let coordinate = Arc::new(x.data.clone());
    let mut components: BTreeMap<String, ComplexComponentColumns> = BTreeMap::new();
    let mut waveforms = Vec::new();
    for signal in dataset.signals {
        if matches!(signal.name.as_str(), "Fmin" | "Rn") {
            let axis = signal
                .x_values
                .map(Arc::new)
                .unwrap_or_else(|| Arc::clone(&coordinate));
            if signal.data.len() != axis.len() {
                return Err(format!(
                    "Touchstone noise parameter {} does not match its frequency grid",
                    signal.name
                )
                .into());
            }
            let mut waveform = RetainedWaveform::new(signal.name, axis, signal.data);
            waveform.unit = Some(signal.unit);
            waveforms.push(waveform);
            continue;
        }
        let (base, imaginary) = if let Some(base) = signal.name.strip_suffix("_RE") {
            (base, false)
        } else if let Some(base) = signal.name.strip_suffix("_IM") {
            (base, true)
        } else {
            return Err(format!(
                "Touchstone adapter returned an untyped component '{}'",
                signal.name
            )
            .into());
        };
        let base = base.to_owned();
        let entry = components.entry(base.clone()).or_default();
        let slot = if imaginary {
            &mut entry.1
        } else {
            &mut entry.0
        };
        if slot.replace(signal).is_some() {
            return Err(format!("Touchstone source repeats component '{base}'").into());
        }
    }
    for (name, (real, imaginary)) in components {
        let real = real.ok_or_else(|| format!("Touchstone source is missing {name}_RE"))?;
        let imaginary =
            imaginary.ok_or_else(|| format!("Touchstone source is missing {name}_IM"))?;
        let axis = real
            .x_values
            .map(Arc::new)
            .unwrap_or_else(|| Arc::clone(&coordinate));
        let imaginary_axis = imaginary.x_values.as_deref().unwrap_or(&coordinate);
        if real.data.len() != axis.len()
            || imaginary.data.len() != axis.len()
            || imaginary_axis != axis.as_slice()
        {
            return Err(
                format!("Touchstone parameter {name} does not match the frequency grid").into(),
            );
        }
        let real = real.data;
        let imaginary = imaginary.data;
        let magnitude = real
            .iter()
            .zip(&imaginary)
            .map(|(real, imaginary)| real.hypot(*imaginary))
            .collect::<Vec<_>>();
        waveforms.push(
            RetainedWaveform::new(format!("|{name}|"), axis, magnitude)
                .with_complex_components(name, real, imaginary)
                .with_unit("1"),
        );
    }
    let reference_impedances_ohm = dataset
        .metadata
        .get("z0_ports")
        .ok_or_else(|| "Touchstone source has no reference-impedance metadata".to_owned())?
        .split(',')
        .map(|value| {
            value
                .parse::<f64>()
                .map_err(|source| TouchstoneError::InvalidFloat {
                    detail: "Touchstone adapter returned invalid impedance metadata".to_owned(),
                    source,
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let family_metadata = AnalysisResultFamilyMetadata::SParameter {
        noise_reference_temperature_kelvin: dataset
            .metadata
            .get("noise_reference_temperature_kelvin")
            .map(|value| {
                value
                    .parse::<f64>()
                    .map_err(|source| TouchstoneError::InvalidFloat {
                        detail: "Invalid Touchstone noise reference temperature".to_owned(),
                        source,
                    })
            })
            .transpose()?,
        reference_impedances_ohm,
    };
    family_metadata.validate_for(AnalysisType::SParameter)?;
    Ok(DecodedTouchstone {
        source_format: parsed_format,
        data: ImportedWaveforms {
            coordinate_name: "frequency".to_owned(),
            sample_count: coordinate.len(),
            waveforms,
        },
        family_metadata,
        notes: if dataset
            .metadata
            .contains_key("noise_reference_temperature_kelvin")
        {
            vec!["Noise parameters use the conventional 290 K source reference. This does not establish the device temperature. The independently sampled noise sweep is retained without interpolation.".into()]
        } else {
            Vec::new()
        },
    })
}
