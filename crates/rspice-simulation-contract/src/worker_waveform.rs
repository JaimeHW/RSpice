//! Sampled waveform data exchanged with the simulation worker.

use serde::{Deserialize, Serialize};

use rspice_results::waveform::WaveformData;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerWaveform {
    pub name: String,
    pub x_values: Vec<f64>,
    pub y_values: Vec<f64>,
    pub y_unit: String,
    pub is_complex: bool,
    pub y_imag: Option<Vec<f64>>,
}

impl From<WaveformData> for WorkerWaveform {
    fn from(value: WaveformData) -> Self {
        Self {
            name: value.name,
            x_values: value.x_values,
            y_values: value.y_values,
            y_unit: value.y_unit,
            is_complex: value.is_complex,
            y_imag: value.y_imag,
        }
    }
}

impl From<WorkerWaveform> for WaveformData {
    fn from(value: WorkerWaveform) -> Self {
        Self {
            name: value.name,
            x_values: value.x_values,
            y_values: value.y_values,
            y_unit: value.y_unit,
            is_complex: value.is_complex,
            y_imag: value.y_imag,
        }
    }
}
