//! Worker noise summary records and lossless retained-result conversions.

use rspice_results::noise::{
    NoiseContributorRow, NoiseFigureEvidence, NoiseSummary, PeriodicNoiseConversionEvidence,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerNoiseSummary {
    #[serde(default)]
    pub input_quantity: Option<rspice_core::analysis::noise::NoiseInputQuantity>,
    #[serde(default)]
    pub conversion: Option<PeriodicNoiseConversionEvidence>,
    #[serde(default)]
    pub noise_figure: Option<std::sync::Arc<NoiseFigureEvidence>>,
    pub rows: Vec<WorkerNoiseContributorRow>,
    #[serde(default)]
    pub total_rms: Option<f64>,
    #[serde(default)]
    pub input_rms: Option<f64>,
    pub band: (f64, f64),
}

impl From<NoiseSummary> for WorkerNoiseSummary {
    fn from(value: NoiseSummary) -> Self {
        Self {
            input_quantity: value.input_quantity,
            conversion: value.conversion,
            noise_figure: value.noise_figure,
            rows: value
                .rows
                .into_iter()
                .map(WorkerNoiseContributorRow::from)
                .collect(),
            total_rms: value.total_rms,
            input_rms: value.input_rms,
            band: value.band,
        }
    }
}

impl From<WorkerNoiseSummary> for NoiseSummary {
    fn from(value: WorkerNoiseSummary) -> Self {
        Self {
            input_quantity: value.input_quantity,
            conversion: value.conversion,
            noise_figure: value.noise_figure,
            rows: value
                .rows
                .into_iter()
                .map(NoiseContributorRow::from)
                .collect(),
            total_rms: value.total_rms,
            input_rms: value.input_rms,
            band: value.band,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerNoiseContributorRow {
    pub device: String,
    pub mechanism: String,
    pub power: f64,
    pub share_pct: f64,
}

impl From<NoiseContributorRow> for WorkerNoiseContributorRow {
    fn from(value: NoiseContributorRow) -> Self {
        Self {
            device: value.device,
            mechanism: value.mechanism.to_string(),
            power: value.power,
            share_pct: value.share_pct,
        }
    }
}

impl From<WorkerNoiseContributorRow> for NoiseContributorRow {
    fn from(value: WorkerNoiseContributorRow) -> Self {
        Self {
            device: value.device,
            mechanism: value.mechanism,
            power: value.power,
            share_pct: value.share_pct,
        }
    }
}
