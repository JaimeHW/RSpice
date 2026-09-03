//! Simulation results Python bindings with NumPy integration.
//!
//! Each analysis owns a module, so a result type is found by the analysis that
//! produces it:
//!
//! | Module                  | Types                                                          |
//! |-------------------------|----------------------------------------------------------------|
//! | [`dc`]                   | `SimulationResult`, `DcSweepResult`, `DeviceOperatingPoint`     |
//! | [`transient`]            | `TransientResult`                                               |
//! | [`transient_compression`]| `CompressedTransientResult`, `TransientCheckpoint`              |
//! | [`fft`]                  | `FftResult`, `FftBin`, `FftMetrics`, `FftHarmonic`               |
//! | [`fourier`]              | `FourierResult`, `Harmonic`                                     |
//! | [`ac`]                   | `AcResult`, `ComplexValue`                                      |
//! | [`distortion`]           | `DistortionResult`                                              |
//! | [`s_parameters`]         | `SParameterResult`                                              |
//! | [`stability`]            | `StbResult`, `PoleZeroResult`, `TransferFunctionResult`         |
//! | [`noise`]                | `NoiseResult`, `NoiseContribution`, `PeriodicNoiseResult`, `PeriodicNoiseContribution`, `OscillatorNoiseResult` |
//! | [`pss`] / [`hb`] / [`pac`] | `PssResult`, `HbResult`, `PacResult`                          |
//! | [`sensitivity`]          | `SensitivityResult`, `ElementSensitivity`, `AcSensitivityResult`, `AcSensitivity` |
//! | [`monte_carlo`]          | `MonteCarloResult`, `VariableStatistics`                        |
//! | [`verification`]         | `Measurement`, `AnalysisRecord`, `RunReport`                    |
//!
//! Three modules are cross-cutting rather than per-analysis, because every
//! family depends on them and none of them belongs to one analysis:
//! [`access`] owns the error contract, [`state`] the pickle encoding, and
//! [`export_bridge`] the shared serialization plumbing.
//!
//! Error discipline: every accessor raises `IndexError` for out-of-range
//! indices and `KeyError` for unknown node/branch names — silent zeros are
//! never fabricated.
//!
//! This file holds only the imports the families share, the module wiring, and
//! the re-exports the rest of the crate binds against.

use numpy::{PyArray1, ToPyArray};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rspice_core::Complex64;
use rspice_core::analysis::fourier::FourierError;
use rspice_core::analysis::{
    AcResult, AcSensitivityResult, DistortionAnalysisResult, DistortionProduct,
    HbContinuationLimitation, HbReactiveKind,
};
use rspice_core::analysis::{FourierAnalysis, FourierConfig};
use rspice_core::engine::TransientResult;
use rspice_core::solver::SimulationResult;
use std::path::PathBuf;

mod ac;
mod access;
mod dc;
mod distortion;
mod export_bridge;
mod fft;
mod fourier;
mod hb;
mod monte_carlo;
mod noise;
mod pac;
mod projection;
mod pss;
mod s_parameters;
mod sensitivity;
mod stability;
mod state;
mod transient;
mod transient_compression;
mod verification;

// Cross-cutting helpers, re-imported here so each family module reaches
// them through its own `use super::*`.
use access::*;
use export_bridge::*;
use state::*;

pub(crate) use ac::{PyAcResult, PyComplexValue};
pub(crate) use access::{NodeIdentifier, is_ground_name};
pub(crate) use dc::{PyDcSweepResult, PyDeviceOperatingPoint, PySimulationResult};
pub(crate) use distortion::PyDistortionResult;
pub(crate) use fft::{
    PyTransientFftBin, PyTransientFftHarmonic, PyTransientFftMetrics, PyTransientFftResult,
};
pub(crate) use fourier::{PyFourierResult, PyHarmonic};
pub(crate) use hb::PyHbResult;
pub(crate) use monte_carlo::{PyMonteCarloResult, PyVariableStatistics};
pub(crate) use noise::{
    PyNoiseContribution, PyNoiseResult, PyOscillatorNoiseResult, PyPeriodicNoiseContribution,
    PyPeriodicNoiseResult,
};
pub(crate) use pac::PyPacResult;
pub(crate) use projection::PyProjectedSignal;
pub(crate) use pss::{PyFloquetSpectrumCertificate, PyFloquetSpectrumEvidence, PyPssResult};
pub(crate) use s_parameters::{PySParameterResult, SParameterNoiseData};
pub(crate) use sensitivity::{
    PyAcSensitivity, PyAcSensitivityResult, PyElementSensitivity, PySensitivityResult,
};
pub(crate) use stability::{
    PyPoleZeroResult, PyRootSetEvidence, PySpectrumCertificate, PyStbResult,
    PyTransferFunctionResult,
};
pub(crate) use transient::PyTransientResult;
pub(crate) use transient_compression::{PyCompressedTransientResult, PyTransientCheckpoint};
pub(crate) use verification::{
    PyAnalysisRecord, PyMeasurement, PyRunAxisAssignment, PyRunCoordinate, PyRunReport,
};
