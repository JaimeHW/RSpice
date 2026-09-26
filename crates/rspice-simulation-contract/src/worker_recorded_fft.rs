//! Recorded `.FFT` spectra across the browser-worker boundary.
//!
//! A spectrum is three numeric columns plus the evidence that describes them.
//! The columns ride the transfer-buffer channel like every other numeric
//! array; the evidence is short and rides the JSON envelope, and it is
//! re-validated on arrival, so a worker cannot hand back a spectrum whose
//! description does not fit its own coefficients.

use serde::{Deserialize, Serialize};

use crate::worker_transport::WorkerF64Series;
use rspice_results::fft::recorded::RecordedFftSpectrum;
use rspice_results::fft::spectrum::FftSpectrumEvidence;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerRecordedFftSpectrum {
    pub request_key: String,
    pub evidence: FftSpectrumEvidence,
    pub frequency: Vec<f64>,
    pub real: Vec<f64>,
    pub imaginary: Vec<f64>,
}

impl From<&RecordedFftSpectrum> for WorkerRecordedFftSpectrum {
    fn from(spectrum: &RecordedFftSpectrum) -> Self {
        Self {
            request_key: spectrum.request_key.clone(),
            evidence: spectrum.evidence.clone(),
            frequency: spectrum.frequency.clone(),
            real: spectrum.real.clone(),
            imaginary: spectrum.imaginary.clone(),
        }
    }
}

impl From<WorkerRecordedFftSpectrum> for RecordedFftSpectrum {
    fn from(spectrum: WorkerRecordedFftSpectrum) -> Self {
        Self {
            request_key: spectrum.request_key,
            evidence: spectrum.evidence,
            frequency: spectrum.frequency,
            real: spectrum.real,
            imaginary: spectrum.imaginary,
        }
    }
}

impl WorkerRecordedFftSpectrum {
    pub fn numeric_value_count(&self) -> usize {
        self.frequency
            .len()
            .saturating_add(self.real.len())
            .saturating_add(self.imaginary.len())
    }
}

pub fn worker_spectra(
    spectra: Vec<std::sync::Arc<RecordedFftSpectrum>>,
) -> Vec<WorkerRecordedFftSpectrum> {
    spectra
        .iter()
        .map(|spectrum| WorkerRecordedFftSpectrum::from(spectrum.as_ref()))
        .collect()
}

pub fn recorded_spectra(
    spectra: Vec<WorkerRecordedFftSpectrum>,
) -> Vec<std::sync::Arc<RecordedFftSpectrum>> {
    spectra
        .into_iter()
        .map(|spectrum| std::sync::Arc::new(RecordedFftSpectrum::from(spectrum)))
        .collect()
}

/// Refuse a spectrum whose evidence does not describe its own columns.
pub fn validate_worker_spectra(spectra: &[WorkerRecordedFftSpectrum]) -> Result<(), String> {
    for spectrum in spectra {
        RecordedFftSpectrum::from(spectrum.clone()).validate()?;
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerRecordedFftSpectrumTransport {
    request_key: String,
    evidence: FftSpectrumEvidence,
    frequency: WorkerF64Series,
    real: WorkerF64Series,
    imaginary: WorkerF64Series,
}

impl WorkerRecordedFftSpectrumTransport {
    pub fn from_spectra(
        spectra: Vec<WorkerRecordedFftSpectrum>,
        buffers: &mut Vec<Vec<f64>>,
    ) -> Vec<Self> {
        spectra
            .into_iter()
            .map(|spectrum| Self {
                request_key: spectrum.request_key,
                evidence: spectrum.evidence,
                frequency: WorkerF64Series::from_vec(spectrum.frequency, buffers),
                real: WorkerF64Series::from_vec(spectrum.real, buffers),
                imaginary: WorkerF64Series::from_vec(spectrum.imaginary, buffers),
            })
            .collect()
    }

    pub fn into_spectra(
        transported: Vec<Self>,
        buffers: &[Vec<f64>],
    ) -> Result<Vec<WorkerRecordedFftSpectrum>, String> {
        let mut spectra = Vec::with_capacity(transported.len());
        for spectrum in transported {
            spectra.push(WorkerRecordedFftSpectrum {
                request_key: spectrum.request_key,
                evidence: spectrum.evidence,
                frequency: spectrum.frequency.into_vec(buffers)?,
                real: spectrum.real.into_vec(buffers)?,
                imaginary: spectrum.imaginary.into_vec(buffers)?,
            });
        }
        validate_worker_spectra(&spectra)?;
        Ok(spectra)
    }
}
