//! Simulation results Python bindings with NumPy integration.
//!
//! Each analysis owns a module, so a result type is found by the analysis that
//! produces it:
//!
//! | Module                  | Types                                                          |
//! |-------------------------|----------------------------------------------------------------|
//! | [`dc`]                   | `SimulationResult`, `DcSweepResult`, `DeviceOperatingPoint`     |
//! | [`transient`]            | `TransientResult`, `DigitalEvent`, `DigitalBus`, `BusEvent`     |
//! | [`transient_compression`]| `CompressedTransientResult`, `TransientCheckpoint`              |
//! | [`fft`]                  | `FftResult`, `FftBin`, `FftMetrics`, `FftHarmonic`               |
//! | [`fourier`]              | `FourierResult`, `Harmonic`                                     |
//! | [`ac`]                   | `AcResult`, `ComplexValue`                                      |
//! | [`distortion`]           | `DistortionResult`                                              |
//! | [`envelope`]             | `EnvelopeResult`                                                |
//! | [`s_parameters`]         | `SParameterResult`                                              |
//! | [`stability`]            | `StbResult`, `PoleZeroResult`, `TransferFunctionResult`         |
//! | [`noise`]                | `NoiseResult`, `NoiseContribution`, `PeriodicNoiseResult`, `PeriodicNoiseContribution`, `OscillatorNoiseResult` |
//! | [`pss`] / [`hb`] / [`pac`] | `PssResult`, `HbResult`, `PacResult`                          |
//! | [`sensitivity`]          | `SensitivityResult`, `ElementSensitivity`, `AcSensitivityResult`, `AcSensitivity` |
//! | [`monte_carlo`]          | `MonteCarloResult`, `VariableStatistics`                        |
//! | [`verification`]         | `Measurement`, `AnalysisRecord`, `RunReport`                    |
//!
//! Four modules are cross-cutting rather than per-analysis, because every
//! family depends on them and none of them belongs to one analysis:
//! [`access`] owns the error contract, [`state`] the pickle encoding,
//! [`export_bridge`] the shared serialization plumbing, and [`document`] the
//! uniform `signals()`/`scalars()`/`device_observables()`/`document()` view
//! every family answers over the one shared result document.
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
use rspice_core::execution::AnalysisResultDocument;
use rspice_core::solver::SimulationResult;
use std::path::PathBuf;

mod ac;
mod access;
mod dc;
mod distortion;
mod document;
mod envelope;
mod event_state;
mod export_bridge;
mod fft;
mod fourier;
mod hb;
mod monte_carlo;
mod noise;
mod pac;
/// Chaos coverage for the pickle-state decoders in [`state`]. The pickled
/// state of a result is untrusted machine-written input, so the decoders that
/// read it back get the same treatment the netlist parser gets.
#[cfg(test)]
mod pickle_state_chaos;
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
use document::DocumentEvidence;
use event_state::*;
use export_bridge::*;
use state::*;

pub(crate) use ac::{PyAcResult, PyComplexValue, validated_ac_schema};
pub(crate) use access::{NodeIdentifier, is_ground_name};
pub(crate) use dc::{PyDcSweepResult, PyDeviceOperatingPoint, PySimulationResult};
pub(crate) use distortion::PyDistortionResult;
pub(crate) use document::{
    CarriesDocumentEvidence, PyDeviceObservable, PyResultScalar, PySignalDescriptor,
    bound as bind_document_identity,
};
pub(crate) use envelope::PyEnvelopeResult;
pub(crate) use fft::{
    PyTransientFftBin, PyTransientFftHarmonic, PyTransientFftMetrics, PyTransientFftResult,
};
pub(crate) use fourier::{PyFourierResult, PyHarmonic};
pub(crate) use hb::PyHbResult;
pub(crate) use monte_carlo::{PyMonteCarloResult, PyVariableStatistics};
pub(crate) use noise::{
    PyNoiseContribution, PyNoiseResult, PyOscillatorNoiseResult, PyPeriodicNoiseContribution,
    PyPeriodicNoiseResult, periodic_noise_probe,
};
pub(crate) use pac::PyPacResult;
pub(crate) use projection::PyProjectedSignal;
pub(crate) use pss::{PyFloquetSpectrumCertificate, PyFloquetSpectrumEvidence, PyPssResult};
pub(crate) use s_parameters::PySParameterResult;
pub(crate) use sensitivity::{
    PyAcSensitivity, PyAcSensitivityResult, PyElementSensitivity, PySensitivityResult,
};
pub(crate) use stability::{
    PyPoleZeroResult, PyRootSetEvidence, PySpectrumCertificate, PyStbResult,
    PyTransferFunctionResult,
};
pub(crate) use transient::{
    PyBusEvent, PyDigitalBus, PyDigitalEvent, PyTransientResult, bus_event_rows, digital_bus_list,
};
pub(crate) use transient_compression::{PyCompressedTransientResult, PyTransientCheckpoint};
pub(crate) use verification::{
    PyAnalysisRecord, PyMeasurement, PyRunAxisAssignment, PyRunCoordinate, PyRunReport,
};
