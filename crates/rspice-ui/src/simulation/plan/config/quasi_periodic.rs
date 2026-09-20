//! QPSS authoring without dropping any driven-engine configuration field.
use super::*;
use crate::simulation::multi_run::{AnalysisSpec, HbToneSpec, QpssControls};
use rspice_core::analysis::quasi_periodic::QuasiPeriodicSampling;
use rspice_core::engine::{QpssInitialState, QpssSourceTone};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QpssDraft {
    pub tones: String,
    pub harmonics: String,
    pub max_iterations: String,
    pub relative_tolerance: String,
    pub current_absolute_tolerance: String,
    pub voltage_absolute_tolerance: String,
    pub max_backtracks: String,
    pub max_mixing_order: String,
    pub oversample: String,
    pub collocation_points: String,
    pub source_tones: String,
    pub dc_initialization: bool,
    pub autonomous: bool,
    pub oscillator_node: String,
}

impl Default for QpssDraft {
    fn default() -> Self {
        Self {
            tones: "1G, 1.001G".into(),
            harmonics: "7, 7".into(),
            max_iterations: "100".into(),
            relative_tolerance: "1e-6".into(),
            current_absolute_tolerance: "1e-12".into(),
            voltage_absolute_tolerance: "1e-9".into(),
            max_backtracks: "20".into(),
            max_mixing_order: String::new(),
            oversample: "2".into(),
            collocation_points: String::new(),
            source_tones: String::new(),
            dc_initialization: false,
            autonomous: false,
            oscillator_node: String::new(),
        }
    }
}

fn counts(value: &str, label: &str) -> Result<Vec<usize>, String> {
    value
        .split(',')
        .map(str::trim)
        .map(|value| parse_positive_usize(value, label))
        .collect()
}

impl QpssDraft {
    pub(crate) fn to_spec(&self) -> Result<AnalysisSpec, String> {
        let frequencies = self
            .tones
            .split(',')
            .map(str::trim)
            .map(|value| parse_positive(value, "QPSS tone"))
            .collect::<Result<Vec<_>, _>>()?;
        let harmonics = counts(&self.harmonics, "QPSS harmonic order")?;
        if frequencies.len() != harmonics.len() {
            return Err("QPSS must declare one harmonic order per tone".into());
        }
        let tones = frequencies
            .into_iter()
            .zip(harmonics)
            .enumerate()
            .map(|(index, (frequency, harmonics))| {
                HbToneSpec::new(frequency, harmonics).with_name(format!("tone{}", index + 1))
            })
            .collect::<Vec<_>>();
        let sampling = if self.collocation_points.trim().is_empty() {
            QuasiPeriodicSampling::Oversample(counts(&self.oversample, "QPSS oversampling")?)
        } else {
            QuasiPeriodicSampling::Exact(counts(
                &self.collocation_points,
                "QPSS collocation points",
            )?)
        };
        let mut source_tones = Vec::new();
        for entry in self
            .source_tones
            .split([',', ';', '\n'])
            .map(str::trim)
            .filter(|entry| !entry.is_empty())
        {
            let (source, number) = entry.rsplit_once('=').ok_or_else(|| {
                "QPSS source assignments use source=tone, such as V1=1, V2=2".to_owned()
            })?;
            source_tones.push(QpssSourceTone {
                source: source.trim().to_owned(),
                tone: parse_positive_usize(number.trim(), "QPSS source tone number")? - 1,
            });
        }
        let controls = QpssControls {
            current_absolute_tolerance: parse_positive(
                &self.current_absolute_tolerance,
                "QPSS current tolerance",
            )?,
            voltage_absolute_tolerance: parse_positive(
                &self.voltage_absolute_tolerance,
                "QPSS voltage tolerance",
            )?,
            max_backtracks: self.max_backtracks.trim().parse::<usize>().map_err(|_| {
                "QPSS maximum backtracks must be an integer from 0 to 60".to_owned()
            })?,
            max_mixing_order: if self.max_mixing_order.trim().is_empty() {
                None
            } else {
                Some(parse_positive_usize(
                    &self.max_mixing_order,
                    "QPSS mixing order",
                )?)
            },
            sampling,
            initial_state: if self.dc_initialization {
                QpssInitialState::DcOperatingPoint
            } else {
                QpssInitialState::Zero
            },
            source_tones,
        };
        let spec = AnalysisSpec::Qpss {
            tones,
            max_iterations: parse_positive_usize(&self.max_iterations, "QPSS iteration budget")?,
            relative_tolerance: parse_positive(
                &self.relative_tolerance,
                "QPSS relative tolerance",
            )?,
            autonomous: self.autonomous,
            oscillator_node: (!self.oscillator_node.trim().is_empty())
                .then(|| self.oscillator_node.trim().to_owned()),
            controls,
        };
        spec.validate()?;
        Ok(spec)
    }
}

pub(super) fn validate_qpss(draft: &QpssDraft) -> Option<String> {
    draft.to_spec().err()
}
