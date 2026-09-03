//! Projections from every core result type into [`AnalysisResultDocument`].
//!
//! Each constructor takes the identity inputs explicitly — the analysis
//! instance, and for a post-process its parent — and returns a builder so the
//! caller can still attach the coordinate, topology fingerprint, and artifact
//! namespaces it owns. Nothing here infers identity from a result's contents.
//!
//! Every projection is fallible. A source result whose names, shapes, or values
//! cannot be represented exactly is rejected with
//! [`ResultDocumentError::SourceResult`] naming the offending series, never
//! reshaped into something that merely looks plausible.
//!
//! [`AnalysisResultDocument`]: super::AnalysisResultDocument

use std::collections::BTreeMap;

use num_complex::Complex64;

use super::payload::{
    AcPayload, CompressionReportDocument, DcSweepPayload, DigitalEventPoint, DigitalEventTrace,
    DistortionPayload, DistortionProductSeries, DistortionProductTag, DistortionTone,
    EnvelopeCarrierDocument, EnvelopeContinuationDocument, EnvelopeNodeSpectrum, EnvelopePayload,
    FftMetricsDocument, FftPayload, FftSourceDocument, FloquetEvidenceDocument, FourierPayload,
    HarmonicBalancePayload, HbReactiveSpectrumDocument, MonteCarloPayload,
    MonteCarloVariableStatistics, NamedObservable, NamedObservableSeries, NoiseContributionSeries,
    NoisePayload, NoiseSourceIdentityDocument, NyquistSample, OperatingPointPayload,
    PNoiseBandwidth, PNoiseContribution, PNoiseContributor, PNoisePayload, PacConversionEntry,
    PacConversionMatrixDocument, PacPayload, PacSidebandDescriptor, PoleZeroPayload, PortDocument,
    PortNoisePayload, RealEventPoint, RealEventTrace, ResultPayload, RootSetEvidenceDocument,
    SParameterPayload, SensitivityEntry, SensitivityPayload, StabilityPayload,
    TransferFunctionPayload, TransientPayload,
};
use super::{
    AnalysisResultDocument, AnalysisResultDocumentBuilder, AxisValues, ComplexSample,
    DeviceParameterSeries, DeviceStateSeries, ResultAxis, ResultAxisKind, ResultDocumentError,
    ResultScalar, ResultSignal, ScalarValue, SeriesAvailability, SeriesQualifier, SeriesValues,
};
use crate::Value;
use crate::analysis::ac::AcResult;
use crate::analysis::distortion::{DistortionAnalysisResult, DistortionPointResult};
use crate::analysis::fourier::FourierResult;
use crate::analysis::harmonic_balance::HbResult;
use crate::analysis::monte_carlo::MonteCarloResult;
use crate::analysis::noise::{NoiseResult, PortNoiseCorrelationResult};
use crate::analysis::pac::PacResult;
use crate::analysis::pnoise::{PhaseNoisePoint, PnoiseResult};
use crate::analysis::pole_zero::PoleZeroResult;
use crate::analysis::pss::PssResult;
use crate::analysis::s_param::SParameterResult;
use crate::analysis::sensitivity::SensitivityResult;
use crate::analysis::stb::StbResult;
use crate::analysis::transfer::TransferFunctionResult;
use crate::circuit::DeviceOpReport;
use crate::engine::EnvelopeResult;
use crate::engine::waveform::TransientCompressionReport;
use crate::engine::{DcSweepPointResult, TransientFftResult, TransientResult};
use crate::execution::plan::AnalysisInstanceId;
use crate::execution::schema::{
    SignalDescriptor, SignalKind, SignalOwner, SignalShape, SignalUnit, SignalValueType,
};
use crate::solver::SimulationResult;

//=============================================================================
// Shared helpers
//=============================================================================

fn source_error(location: &'static str, detail: impl Into<String>) -> ResultDocumentError {
    ResultDocumentError::SourceResult {
        location,
        detail: detail.into(),
    }
}

fn descriptor(
    location: &'static str,
    canonical: String,
    display: String,
    kind: SignalKind,
    unit: SignalUnit,
    value_type: SignalValueType,
    shape: SignalShape,
    owner: SignalOwner,
) -> Result<SignalDescriptor, ResultDocumentError> {
    SignalDescriptor::new(canonical, display, kind, unit, value_type, shape, owner)
        .map_err(|error| source_error(location, error.to_string()))
}

fn series_shape(point_count: usize) -> SignalShape {
    if point_count == 1 {
        SignalShape::Scalar
    } else {
        SignalShape::Vector
    }
}

fn voltage_descriptor(
    location: &'static str,
    node: &str,
    value_type: SignalValueType,
    point_count: usize,
) -> Result<SignalDescriptor, ResultDocumentError> {
    descriptor(
        location,
        format!("v({node})"),
        format!("V({node})"),
        SignalKind::Voltage,
        SignalUnit::Volt,
        value_type,
        series_shape(point_count),
        SignalOwner::Node(node.to_owned()),
    )
}

fn current_descriptor(
    location: &'static str,
    branch: &str,
    value_type: SignalValueType,
    point_count: usize,
) -> Result<SignalDescriptor, ResultDocumentError> {
    descriptor(
        location,
        format!("i({branch})"),
        format!("I({branch})"),
        SignalKind::Current,
        SignalUnit::Ampere,
        value_type,
        series_shape(point_count),
        SignalOwner::Branch(branch.to_owned()),
    )
}

fn analysis_descriptor(
    location: &'static str,
    canonical: &str,
    display: &str,
    unit: SignalUnit,
    value_type: SignalValueType,
    point_count: usize,
) -> Result<SignalDescriptor, ResultDocumentError> {
    descriptor(
        location,
        canonical.to_owned(),
        display.to_owned(),
        SignalKind::Scalar,
        unit,
        value_type,
        series_shape(point_count),
        SignalOwner::Analysis,
    )
}

fn finite_samples(
    location: &'static str,
    name: &str,
    values: &[Value],
) -> Result<Vec<Option<f64>>, ResultDocumentError> {
    let mut samples = Vec::with_capacity(values.len());
    for (index, value) in values.iter().enumerate() {
        if !value.is_finite() {
            return Err(source_error(
                location,
                format!("'{name}' sample {index} is {value}, which JSON cannot represent"),
            ));
        }
        samples.push(Some(*value));
    }
    Ok(samples)
}

fn finite_complex_samples(
    location: &'static str,
    name: &str,
    values: &[Complex64],
) -> Result<Vec<Option<ComplexSample>>, ResultDocumentError> {
    let mut samples = Vec::with_capacity(values.len());
    for (index, value) in values.iter().enumerate() {
        if !value.re.is_finite() || !value.im.is_finite() {
            return Err(source_error(
                location,
                format!("'{name}' sample {index} is not finite"),
            ));
        }
        samples.push(Some(ComplexSample::new(value.re, value.im)));
    }
    Ok(samples)
}

fn finite_axis(
    location: &'static str,
    name: &str,
    values: &[Value],
) -> Result<Vec<f64>, ResultDocumentError> {
    for (index, value) in values.iter().enumerate() {
        if !value.is_finite() {
            return Err(source_error(
                location,
                format!("axis '{name}' coordinate {index} is {value}"),
            ));
        }
    }
    Ok(values.to_vec())
}

fn finite_scalar(
    location: &'static str,
    name: &str,
    value: Value,
) -> Result<ScalarValue, ResultDocumentError> {
    if value.is_finite() {
        Ok(ScalarValue::Real { value: Some(value) })
    } else {
        Err(source_error(
            location,
            format!("scalar '{name}' is {value}, which JSON cannot represent"),
        ))
    }
}

/// A statistic the producing result reports as `NaN` when it is undefined.
fn defined_or_missing(value: Value) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn real_scalar(
    location: &'static str,
    name: &str,
    display: &str,
    unit: SignalUnit,
    value: Value,
) -> Result<ResultScalar, ResultDocumentError> {
    ResultScalar::new(
        name,
        display,
        Some(unit),
        finite_scalar(location, name, value)?,
    )
}

fn count_scalar(
    name: &str,
    display: &str,
    value: usize,
) -> Result<ResultScalar, ResultDocumentError> {
    ResultScalar::new(
        name,
        display,
        Some(SignalUnit::Dimensionless),
        ScalarValue::Count {
            value: value as u64,
        },
    )
}

fn boolean_scalar(
    name: &str,
    display: &str,
    value: bool,
) -> Result<ResultScalar, ResultDocumentError> {
    ResultScalar::new(name, display, None, ScalarValue::Boolean { value })
}

fn decibel() -> SignalUnit {
    SignalUnit::Custom("dB".to_owned())
}

fn volt_squared_per_hertz() -> SignalUnit {
    SignalUnit::Custom("V^2/Hz".to_owned())
}

fn ampere_squared_per_hertz() -> SignalUnit {
    SignalUnit::Custom("A^2/Hz".to_owned())
}

fn percent() -> SignalUnit {
    SignalUnit::Custom("percent".to_owned())
}

fn dbc_per_hertz() -> SignalUnit {
    SignalUnit::Custom("dBc/Hz".to_owned())
}

fn require_named_columns(
    location: &'static str,
    role: &str,
    names: &[String],
    values: usize,
) -> Result<(), ResultDocumentError> {
    if names.len() != values {
        return Err(source_error(
            location,
            format!("{role} has {values} columns but {} names", names.len()),
        ));
    }
    if let Some(index) = names.iter().position(|name| name.trim().is_empty()) {
        return Err(source_error(
            location,
            format!("{role} column {index} has no name"),
        ));
    }
    Ok(())
}

/// Build one device-state history per device seen in the supplied reports.
///
/// A device that is absent from one report keeps its column and records an
/// absent region and absent parameter values at that point.
fn device_states_from_reports(
    location: &'static str,
    reports: &[&DeviceOpReport],
) -> Result<Vec<DeviceStateSeries>, ResultDocumentError> {
    let point_count = reports.len();
    let mut order: Vec<String> = Vec::new();
    let mut kinds: BTreeMap<String, Option<String>> = BTreeMap::new();
    let mut parameter_order: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for report in reports {
        for entry in &report.entries {
            if entry.name.trim().is_empty() {
                return Err(source_error(
                    location,
                    "a device operating-point entry has no name",
                ));
            }
            let key = entry.name.to_ascii_lowercase();
            if !kinds.contains_key(&key) {
                order.push(key.clone());
                kinds.insert(key.clone(), Some(entry.device_kind.to_owned()));
                parameter_order.insert(key.clone(), Vec::new());
            }
            let parameters = parameter_order
                .get_mut(&key)
                .ok_or_else(|| source_error(location, "device parameter order lost its device"))?;
            for (parameter, _) in &entry.params {
                if !parameters.iter().any(|known| known.as_str() == *parameter) {
                    parameters.push((*parameter).to_owned());
                }
            }
        }
    }

    let mut states = Vec::with_capacity(order.len());
    for key in order {
        let parameters = parameter_order
            .get(&key)
            .ok_or_else(|| source_error(location, "device parameter order lost its device"))?;
        let mut regions = Vec::with_capacity(point_count);
        let mut columns: Vec<Vec<Option<f64>>> = parameters
            .iter()
            .map(|_| Vec::with_capacity(point_count))
            .collect();
        let mut display_name = key.clone();
        for report in reports {
            let entry = report
                .entries
                .iter()
                .find(|entry| entry.name.eq_ignore_ascii_case(&key));
            match entry {
                Some(entry) => {
                    display_name = entry.name.clone();
                    regions.push(entry.region.map(str::to_owned));
                    for (index, parameter) in parameters.iter().enumerate() {
                        let value = entry
                            .params
                            .iter()
                            .find(|(name, _)| *name == parameter.as_str())
                            .map(|(_, value)| *value);
                        let value = match value {
                            Some(value) if !value.is_finite() => {
                                return Err(source_error(
                                    location,
                                    format!("device '{key}' parameter '{parameter}' is {value}"),
                                ));
                            }
                            other => other,
                        };
                        columns
                            .get_mut(index)
                            .ok_or_else(|| {
                                source_error(location, "device parameter column vanished")
                            })?
                            .push(value);
                    }
                }
                None => {
                    regions.push(None);
                    for column in &mut columns {
                        column.push(None);
                    }
                }
            }
        }
        let device_kind = kinds.get(&key).cloned().flatten();
        let parameters = parameters
            .iter()
            .zip(columns)
            .map(|(name, values)| DeviceParameterSeries {
                name: name.clone(),
                unit: None,
                values,
            })
            .collect();
        states.push(DeviceStateSeries::new(
            display_name,
            device_kind,
            regions,
            parameters,
        )?);
    }
    Ok(states)
}

//=============================================================================
// Constructors
//=============================================================================

impl AnalysisResultDocument {
    /// Project one converged operating point.
    pub fn from_operating_point(
        analysis: AnalysisInstanceId,
        result: &SimulationResult,
        device_report: Option<&DeviceOpReport>,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "operating-point result";
        require_named_columns(
            LOCATION,
            "operating-point node voltages",
            &result.node_names,
            result.node_voltages.len(),
        )?;
        require_named_columns(
            LOCATION,
            "operating-point branch currents",
            &result.branch_names,
            result.branch_currents.len(),
        )?;

        let mut signals = Vec::new();
        for (name, value) in result.node_names.iter().zip(&result.node_voltages) {
            signals.push(ResultSignal::new(
                voltage_descriptor(LOCATION, name, SignalValueType::Real, 1)?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real {
                    samples: finite_samples(LOCATION, name, std::slice::from_ref(value))?,
                },
            )?);
        }
        for (name, value) in result.branch_names.iter().zip(&result.branch_currents) {
            signals.push(ResultSignal::new(
                current_descriptor(LOCATION, name, SignalValueType::Real, 1)?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real {
                    samples: finite_samples(LOCATION, name, std::slice::from_ref(value))?,
                },
            )?);
        }

        let mut observables = Vec::with_capacity(result.dc_observables.len());
        for (name, value) in &result.dc_observables {
            if !value.is_finite() {
                return Err(source_error(
                    LOCATION,
                    format!("observable '{name}' is {value}"),
                ));
            }
            observables.push(NamedObservable {
                name: name.clone(),
                unit: None,
                value: Some(*value),
            });
        }

        let device_states = match device_report {
            Some(report) => device_states_from_reports(LOCATION, &[report])?,
            None => Vec::new(),
        };

        Ok(Self::builder(
            analysis,
            ResultPayload::Op(OperatingPointPayload { observables }),
            1,
        )
        .signals(signals)
        .device_states(device_states))
    }

    /// Project one DC sweep over its authored sweep variable.
    pub fn from_dc_sweep(
        analysis: AnalysisInstanceId,
        sweep_variable: &str,
        sweep_unit: SignalUnit,
        points: &[DcSweepPointResult],
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "DC sweep result";
        let first = points
            .first()
            .ok_or_else(|| source_error(LOCATION, "a DC sweep needs at least one point"))?;
        let point_count = points.len();
        let node_names = &first.result.node_names;
        let branch_names = &first.result.branch_names;
        require_named_columns(
            LOCATION,
            "DC sweep node voltages",
            node_names,
            first.result.node_voltages.len(),
        )?;
        require_named_columns(
            LOCATION,
            "DC sweep branch currents",
            branch_names,
            first.result.branch_currents.len(),
        )?;
        for point in points {
            if &point.result.node_names != node_names || &point.result.branch_names != branch_names
            {
                return Err(source_error(
                    LOCATION,
                    "DC sweep points do not share one signal schema",
                ));
            }
        }

        let sweep_values = points
            .iter()
            .map(|point| point.sweep_value)
            .collect::<Vec<_>>();
        let axis = ResultAxis::new(
            format!("sweep:{}", sweep_variable.trim().to_ascii_lowercase()),
            sweep_variable.trim(),
            ResultAxisKind::SweepValue,
            sweep_unit,
            AxisValues::Real {
                values: finite_axis(LOCATION, sweep_variable, &sweep_values)?,
            },
        )?;

        let mut signals = Vec::with_capacity(node_names.len() + branch_names.len());
        for (index, name) in node_names.iter().enumerate() {
            let column = points
                .iter()
                .map(|point| {
                    point
                        .result
                        .node_voltages
                        .get(index)
                        .copied()
                        .ok_or_else(|| {
                            source_error(
                                LOCATION,
                                format!("node '{name}' is missing at a sweep point"),
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            signals.push(ResultSignal::new(
                voltage_descriptor(LOCATION, name, SignalValueType::Real, point_count)?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real {
                    samples: finite_samples(LOCATION, name, &column)?,
                },
            )?);
        }
        for (index, name) in branch_names.iter().enumerate() {
            let column = points
                .iter()
                .map(|point| {
                    point
                        .result
                        .branch_currents
                        .get(index)
                        .copied()
                        .ok_or_else(|| {
                            source_error(
                                LOCATION,
                                format!("branch '{name}' is missing at a sweep point"),
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            signals.push(ResultSignal::new(
                current_descriptor(LOCATION, name, SignalValueType::Real, point_count)?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real {
                    samples: finite_samples(LOCATION, name, &column)?,
                },
            )?);
        }

        let mut observable_names: Vec<String> = Vec::new();
        for point in points {
            for (name, _) in &point.result.dc_observables {
                if !observable_names.iter().any(|known| known == name) {
                    observable_names.push(name.clone());
                }
            }
        }
        let mut observables = Vec::with_capacity(observable_names.len());
        for name in observable_names {
            let mut values = Vec::with_capacity(point_count);
            for point in points {
                let value = point.result.try_dc_observable_named(&name);
                if let Some(value) = value
                    && !value.is_finite()
                {
                    return Err(source_error(
                        LOCATION,
                        format!("observable '{name}' is {value}"),
                    ));
                }
                values.push(value);
            }
            observables.push(NamedObservableSeries {
                name,
                unit: None,
                values,
            });
        }

        let reports = points
            .iter()
            .map(|point| &point.device_op_report)
            .collect::<Vec<_>>();
        let device_states = device_states_from_reports(LOCATION, &reports)?;

        Ok(Self::builder(
            analysis,
            ResultPayload::Dc(DcSweepPayload {
                sweep_variable: sweep_variable.trim().to_ascii_lowercase(),
                observables,
            }),
            point_count,
        )
        .axis(axis)
        .signals(signals)
        .device_states(device_states))
    }

    /// Project one AC small-signal frequency sweep.
    pub fn from_ac(
        analysis: AnalysisInstanceId,
        points: &[AcResult],
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "AC result";
        let (axis, signals) = complex_frequency_sweep(
            LOCATION,
            points,
            |point| point.frequency,
            |point| (&point.node_names, &point.voltages),
            |point| (&point.branch_names, &point.currents),
            None,
        )?;
        let point_count = points.len();
        Ok(
            Self::builder(analysis, ResultPayload::Ac(AcPayload {}), point_count)
                .axis(axis)
                .signals(signals),
        )
    }

    /// Project one transient result and, optionally, its compression report.
    pub fn from_transient(
        analysis: AnalysisInstanceId,
        result: &TransientResult,
        compression: Option<&TransientCompressionReport>,
        fft_children: Vec<super::payload::FftChildReference>,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "transient result";
        let point_count = result.time.len();
        if point_count == 0 {
            return Err(source_error(
                LOCATION,
                "a transient needs at least one sample",
            ));
        }
        if result.step_sizes.len() != point_count {
            return Err(source_error(
                LOCATION,
                format!(
                    "transient step sizes have {} entries for {point_count} samples",
                    result.step_sizes.len()
                ),
            ));
        }
        require_named_columns(
            LOCATION,
            "transient node voltages",
            &result.node_names,
            result.voltages.len(),
        )?;
        require_named_columns(
            LOCATION,
            "transient branch currents",
            &result.branch_names,
            result.branch_currents.len(),
        )?;

        let axis = ResultAxis::new(
            "time",
            "Time",
            ResultAxisKind::Time,
            SignalUnit::Second,
            AxisValues::Real {
                values: finite_axis(LOCATION, "time", &result.time)?,
            },
        )?;

        let mut signals = Vec::with_capacity(result.voltages.len() + result.branch_currents.len());
        for (name, waveform) in result.node_names.iter().zip(&result.voltages) {
            signals.push(projected_real_series(
                LOCATION,
                voltage_descriptor(LOCATION, name, SignalValueType::Real, point_count)?,
                name,
                waveform,
                point_count,
            )?);
        }
        for (name, waveform) in result.branch_names.iter().zip(&result.branch_currents) {
            signals.push(projected_real_series(
                LOCATION,
                current_descriptor(LOCATION, name, SignalValueType::Real, point_count)?,
                name,
                waveform,
                point_count,
            )?);
        }

        let mut store_traces = Vec::with_capacity(result.store_traces.len());
        for trace in &result.store_traces {
            if trace.values.len() != point_count {
                return Err(source_error(
                    LOCATION,
                    format!(
                        "store trace '{}' has {} samples for {point_count} times",
                        trace.name,
                        trace.values.len()
                    ),
                ));
            }
            store_traces.push(NamedObservableSeries {
                name: trace.name.clone(),
                unit: None,
                values: finite_samples(LOCATION, &trace.name, &trace.values)?,
            });
        }

        let mut device_columns: BTreeMap<String, Vec<DeviceParameterSeries>> = BTreeMap::new();
        let mut device_order: Vec<String> = Vec::new();
        for trace in &result.device_op_traces {
            if trace.values.len() != point_count {
                return Err(source_error(
                    LOCATION,
                    format!(
                        "device trace '{}:{}' has {} samples for {point_count} times",
                        trace.device_name,
                        trace.parameter,
                        trace.values.len()
                    ),
                ));
            }
            let entry = device_columns
                .entry(trace.device_name.clone())
                .or_insert_with(|| {
                    device_order.push(trace.device_name.clone());
                    Vec::new()
                });
            entry.push(DeviceParameterSeries {
                name: trace.parameter.clone(),
                unit: None,
                values: finite_samples(LOCATION, &trace.parameter, &trace.values)?,
            });
        }
        let mut device_states = Vec::with_capacity(device_order.len());
        for name in device_order {
            let parameters = device_columns
                .remove(&name)
                .ok_or_else(|| source_error(LOCATION, "device trace grouping lost a device"))?;
            device_states.push(DeviceStateSeries::new(name, None, Vec::new(), parameters)?);
        }

        let mut digital_traces = Vec::with_capacity(result.digital_traces.len());
        for trace in &result.digital_traces {
            let mut points = Vec::with_capacity(trace.points.len());
            for point in &trace.points {
                if !point.time.is_finite() {
                    return Err(source_error(
                        LOCATION,
                        format!(
                            "digital trace '{}' has a non-finite event time",
                            trace.node_name
                        ),
                    ));
                }
                points.push(DigitalEventPoint {
                    time: point.time,
                    state: point.value.state.into(),
                    strength: point.value.strength.into(),
                });
            }
            digital_traces.push(DigitalEventTrace {
                node_name: trace.node_name.clone(),
                points,
            });
        }

        let mut real_traces = Vec::with_capacity(result.real_traces.len());
        for trace in &result.real_traces {
            let mut points = Vec::with_capacity(trace.points.len());
            for point in &trace.points {
                if !point.time.is_finite() || !point.value.is_finite() {
                    return Err(source_error(
                        LOCATION,
                        format!(
                            "real event trace '{}' has a non-finite sample",
                            trace.node_name
                        ),
                    ));
                }
                points.push(RealEventPoint {
                    time: point.time,
                    value: point.value,
                });
            }
            real_traces.push(RealEventTrace {
                node_name: trace.node_name.clone(),
                points,
            });
        }

        let payload = TransientPayload {
            step_sizes: finite_axis(LOCATION, "step size", &result.step_sizes)?,
            store_traces,
            digital_traces,
            real_traces,
            fft_children,
            compression: compression.map(CompressionReportDocument::from),
        };

        Ok(
            Self::builder(analysis, ResultPayload::Tran(payload), point_count)
                .axis(axis)
                .signals(signals)
                .device_states(device_states),
        )
    }

    /// Project one noise sweep.
    pub fn from_noise(
        analysis: AnalysisInstanceId,
        points: &[NoiseResult],
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "noise result";
        let first = points
            .first()
            .ok_or_else(|| source_error(LOCATION, "a noise sweep needs at least one point"))?;
        let point_count = points.len();
        let (axis, mut signals) = complex_frequency_sweep(
            LOCATION,
            points,
            |point| point.frequency,
            |point| (&point.node_names, &point.voltages),
            |point| (&point.branch_names, &point.currents),
            None,
        )?;

        let totals: [(&str, &str, SignalUnit, fn(&NoiseResult) -> Value); 3] = [
            (
                "onoise_spectrum",
                "ONOISE",
                volt_squared_per_hertz(),
                |point| point.output_noise_density,
            ),
            (
                "inoise_spectrum",
                "INOISE",
                volt_squared_per_hertz(),
                |point| point.input_referred_density,
            ),
            (
                "input_gain_squared",
                "GAIN^2",
                SignalUnit::Dimensionless,
                |point| point.input_gain_squared,
            ),
        ];
        for (canonical, display, unit, extract) in totals {
            let column = points.iter().map(extract).collect::<Vec<_>>();
            signals.push(ResultSignal::new(
                analysis_descriptor(
                    LOCATION,
                    canonical,
                    display,
                    unit,
                    SignalValueType::Real,
                    point_count,
                )?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real {
                    samples: finite_samples(LOCATION, canonical, &column)?,
                },
            )?);
        }

        let mut catalog = Vec::with_capacity(first.contribution_catalog.len());
        for identity in &first.contribution_catalog {
            catalog.push(NoiseSourceIdentityDocument {
                device: identity.device.clone(),
                mechanism: identity.mechanism.clone(),
            });
        }

        let mut order: Vec<(String, Option<String>)> = Vec::new();
        for point in points {
            for contribution in &point.contributions {
                let key = (
                    contribution.identity.device.clone(),
                    contribution.identity.mechanism.clone(),
                );
                if !order.contains(&key) {
                    order.push(key);
                }
            }
        }
        let mut contributions = Vec::with_capacity(order.len());
        for (device, mechanism) in order {
            let mut mechanism_kind = None;
            let mut output = Vec::with_capacity(point_count);
            let mut input = Vec::with_capacity(point_count);
            let mut share = Vec::with_capacity(point_count);
            for point in points {
                let found = point.contributions.iter().find(|contribution| {
                    contribution.identity.device == device
                        && contribution.identity.mechanism == mechanism
                });
                match found {
                    Some(contribution) => {
                        mechanism_kind = Some(contribution.noise_type.into());
                        for (value, column) in [
                            (contribution.output_contribution, &mut output),
                            (contribution.input_contribution, &mut input),
                            (contribution.percentage, &mut share),
                        ] {
                            if !value.is_finite() {
                                return Err(source_error(
                                    LOCATION,
                                    format!("noise contribution of '{device}' is {value}"),
                                ));
                            }
                            column.push(Some(value));
                        }
                    }
                    None => {
                        output.push(None);
                        input.push(None);
                        share.push(None);
                    }
                }
            }
            let mechanism_kind = mechanism_kind.ok_or_else(|| {
                source_error(
                    LOCATION,
                    format!("noise contributor '{device}' has no samples"),
                )
            })?;
            contributions.push(NoiseContributionSeries {
                identity: NoiseSourceIdentityDocument { device, mechanism },
                mechanism_kind,
                output_contribution: output,
                input_contribution: input,
                percentage: share,
            });
        }

        let payload = NoisePayload {
            contribution_catalog: catalog,
            mechanisms_unavailable: first.mechanisms_unavailable.clone(),
            contributions,
        };
        Ok(
            Self::builder(analysis, ResultPayload::Noise(payload), point_count)
                .axis(axis)
                .signals(signals),
        )
    }

    /// Project one S-parameter sweep.
    pub fn from_s_parameters(
        analysis: AnalysisInstanceId,
        result: &SParameterResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "S-parameter result";
        if result.data.is_empty() {
            return Err(source_error(
                LOCATION,
                "an S-parameter sweep needs at least one frequency",
            ));
        }
        if result.ports.len() != result.num_ports {
            return Err(source_error(
                LOCATION,
                "the port list disagrees with the declared port count",
            ));
        }
        let point_count = result.data.len();
        let frequencies = result
            .data
            .iter()
            .map(|matrix| matrix.frequency)
            .collect::<Vec<_>>();
        let axis = ResultAxis::new(
            "frequency",
            "Frequency",
            ResultAxisKind::Frequency,
            SignalUnit::Hertz,
            AxisValues::Real {
                values: finite_axis(LOCATION, "frequency", &frequencies)?,
            },
        )?;

        let mut signals = Vec::with_capacity(result.num_ports * result.num_ports);
        for row in 1..=result.num_ports {
            for column in 1..=result.num_ports {
                let name = format!("s({row},{column})");
                let values = result
                    .data
                    .iter()
                    .map(|matrix| matrix.get(row, column))
                    .collect::<Vec<_>>();
                signals.push(ResultSignal::new(
                    analysis_descriptor(
                        LOCATION,
                        &name,
                        &format!("S({row},{column})"),
                        SignalUnit::Dimensionless,
                        SignalValueType::Complex,
                        point_count,
                    )?,
                    None,
                    SeriesAvailability::Available,
                    SeriesValues::Complex {
                        samples: finite_complex_samples(LOCATION, &name, &values)?,
                    },
                )?);
            }
        }

        let payload = SParameterPayload {
            reference_impedance: result.z0,
            ports: result
                .ports
                .iter()
                .map(|port| PortDocument {
                    number: port.number,
                    node_positive: port.node_pos.clone(),
                    node_negative: port.node_neg.clone(),
                    reference_impedance: port.z0,
                })
                .collect(),
            angular_frequencies: result.data.iter().map(|matrix| matrix.omega).collect(),
        };

        Ok(
            Self::builder(analysis, ResultPayload::Sp(payload), point_count)
                .axis(axis)
                .signals(signals),
        )
    }

    /// Project one port-noise correlation sweep.
    ///
    /// The analysis identity is the `.SP` card that produced it, because
    /// port-noise is that card's optional second result.
    pub fn from_port_noise(
        analysis: AnalysisInstanceId,
        points: &[PortNoiseCorrelationResult],
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "port-noise result";
        let first = points
            .first()
            .ok_or_else(|| source_error(LOCATION, "a port-noise sweep needs at least one point"))?;
        let port_count = first.current_correlation.len();
        let point_count = points.len();
        for point in points {
            if point.current_correlation.len() != port_count
                || point
                    .current_correlation
                    .iter()
                    .any(|row| row.len() != port_count)
            {
                return Err(source_error(
                    LOCATION,
                    "port-noise correlation matrices are not square and uniform",
                ));
            }
        }
        let frequencies = points
            .iter()
            .map(|point| point.frequency)
            .collect::<Vec<_>>();
        let axis = ResultAxis::new(
            "frequency",
            "Frequency",
            ResultAxisKind::Frequency,
            SignalUnit::Hertz,
            AxisValues::Real {
                values: finite_axis(LOCATION, "frequency", &frequencies)?,
            },
        )?;

        let mut signals = Vec::with_capacity(port_count * port_count);
        for row in 0..port_count {
            for column in 0..port_count {
                let name = format!("cy({},{})", row + 1, column + 1);
                let values = points
                    .iter()
                    .map(|point| {
                        point
                            .current_correlation
                            .get(row)
                            .and_then(|entries| entries.get(column))
                            .copied()
                            .ok_or_else(|| source_error(LOCATION, "correlation entry vanished"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                signals.push(ResultSignal::new(
                    analysis_descriptor(
                        LOCATION,
                        &name,
                        &format!("CY({},{})", row + 1, column + 1),
                        ampere_squared_per_hertz(),
                        SignalValueType::Complex,
                        point_count,
                    )?,
                    None,
                    SeriesAvailability::Available,
                    SeriesValues::Complex {
                        samples: finite_complex_samples(LOCATION, &name, &values)?,
                    },
                )?);
            }
        }

        Ok(Self::builder(
            analysis,
            ResultPayload::PortNoise(PortNoisePayload { port_count }),
            point_count,
        )
        .axis(axis)
        .signals(signals))
    }

    /// Project one `.DISTO` sweep with its Volterra products.
    pub fn from_distortion(
        analysis: AnalysisInstanceId,
        result: &DistortionAnalysisResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "distortion result";
        let first = result
            .points
            .first()
            .ok_or_else(|| source_error(LOCATION, "a distortion sweep needs at least one point"))?;
        let point_count = result.points.len();
        let products = first
            .products
            .iter()
            .map(|product| product.product)
            .collect::<Vec<_>>();
        for point in &result.points {
            let observed = point
                .products
                .iter()
                .map(|product| product.product)
                .collect::<Vec<_>>();
            if observed != products {
                return Err(source_error(
                    LOCATION,
                    "distortion points do not share one product set",
                ));
            }
            if point.fundamental_f2.is_some() != first.fundamental_f2.is_some() {
                return Err(source_error(
                    LOCATION,
                    "distortion points disagree about two-tone mode",
                ));
            }
        }

        let frequencies = result
            .points
            .iter()
            .map(|point| point.fundamental_f1.frequency)
            .collect::<Vec<_>>();
        let axis = ResultAxis::new(
            "frequency",
            "Frequency",
            ResultAxisKind::Frequency,
            SignalUnit::Hertz,
            AxisValues::Real {
                values: finite_axis(LOCATION, "frequency", &frequencies)?,
            },
        )?;

        let responses = |select: &dyn Fn(&DistortionPointResult) -> Option<&AcResult>| {
            result
                .points
                .iter()
                .map(|point| {
                    select(point).cloned().ok_or_else(|| {
                        source_error(LOCATION, "a distortion response vanished between checks")
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        };

        let mut signals = distortion_response_signals(
            LOCATION,
            &responses(&|point| Some(&point.fundamental_f1))?,
            Some(SeriesQualifier::DistortionFundamental {
                tone: DistortionTone::F1,
            }),
        )?;
        if first.fundamental_f2.is_some() {
            signals.extend(distortion_response_signals(
                LOCATION,
                &responses(&|point| point.fundamental_f2.as_ref())?,
                Some(SeriesQualifier::DistortionFundamental {
                    tone: DistortionTone::F2,
                }),
            )?);
        }

        let mut product_series = Vec::with_capacity(products.len());
        for (index, product) in products.iter().enumerate() {
            let tag = DistortionProductTag::from(*product);
            signals.extend(distortion_response_signals(
                LOCATION,
                &responses(&|point| point.products.get(index).map(|entry| &entry.response))?,
                Some(SeriesQualifier::DistortionProduct { product: tag }),
            )?);
            let product_frequencies = result
                .points
                .iter()
                .map(|point| {
                    point
                        .products
                        .get(index)
                        .map(|entry| entry.response.frequency)
                        .ok_or_else(|| source_error(LOCATION, "a distortion product vanished"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            product_series.push(DistortionProductSeries {
                product: tag,
                order: tag.order(),
                frequencies: finite_axis(LOCATION, "product frequency", &product_frequencies)?,
            });
        }

        let payload = DistortionPayload {
            f2_over_f1: result.f2_over_f1,
            products: product_series,
        };
        Ok(
            Self::builder(analysis, ResultPayload::Distortion(payload), point_count)
                .axis(axis)
                .signals(signals),
        )
    }

    /// Project one `.TF` small-signal transfer-function result.
    pub fn from_transfer_function(
        analysis: AnalysisInstanceId,
        result: &TransferFunctionResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "transfer-function result";
        let scalars = vec![
            real_scalar(
                LOCATION,
                "transfer_gain",
                "Transfer gain",
                SignalUnit::Dimensionless,
                result.gain,
            )?,
            real_scalar(
                LOCATION,
                "input_impedance",
                "Input impedance",
                SignalUnit::Ohm,
                result.input_impedance,
            )?,
            real_scalar(
                LOCATION,
                "output_impedance",
                "Output impedance",
                SignalUnit::Ohm,
                result.output_impedance,
            )?,
        ];
        let payload = TransferFunctionPayload {
            output: result.output.clone(),
            input: result.input.clone(),
        };
        Ok(Self::builder(analysis, ResultPayload::Tf(payload), 0).scalars(scalars))
    }

    /// Project one `.STB` loop-gain result.
    pub fn from_stability(
        analysis: AnalysisInstanceId,
        result: &StbResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "stability result";
        let point_count = result.bode_points.len();
        let frequencies = result
            .bode_points
            .iter()
            .map(|point| point.frequency)
            .collect::<Vec<_>>();
        let axis = ResultAxis::new(
            "frequency",
            "Frequency",
            ResultAxisKind::Frequency,
            SignalUnit::Hertz,
            AxisValues::Real {
                values: finite_axis(LOCATION, "frequency", &frequencies)?,
            },
        )?;

        let magnitude = result
            .bode_points
            .iter()
            .map(|point| point.magnitude)
            .collect::<Vec<_>>();
        let magnitude_db = result
            .bode_points
            .iter()
            .map(|point| point.magnitude_db)
            .collect::<Vec<_>>();
        let phase = result
            .bode_points
            .iter()
            .map(|point| point.phase_deg)
            .collect::<Vec<_>>();
        let loop_gain = result
            .bode_points
            .iter()
            .map(|point| point.loop_gain)
            .collect::<Vec<_>>();

        let signals = vec![
            ResultSignal::new(
                analysis_descriptor(
                    LOCATION,
                    "loop_gain",
                    "L(jw)",
                    SignalUnit::Dimensionless,
                    SignalValueType::Complex,
                    point_count,
                )?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Complex {
                    samples: finite_complex_samples(LOCATION, "loop_gain", &loop_gain)?,
                },
            )?,
            ResultSignal::new(
                analysis_descriptor(
                    LOCATION,
                    "loop_gain_magnitude",
                    "|L(jw)|",
                    SignalUnit::Dimensionless,
                    SignalValueType::Real,
                    point_count,
                )?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real {
                    samples: finite_samples(LOCATION, "loop_gain_magnitude", &magnitude)?,
                },
            )?,
            ResultSignal::new(
                analysis_descriptor(
                    LOCATION,
                    "loop_gain_db",
                    "|L(jw)| dB",
                    decibel(),
                    SignalValueType::Real,
                    point_count,
                )?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real {
                    samples: finite_samples(LOCATION, "loop_gain_db", &magnitude_db)?,
                },
            )?,
            ResultSignal::new(
                analysis_descriptor(
                    LOCATION,
                    "loop_gain_phase",
                    "arg L(jw)",
                    SignalUnit::Degree,
                    SignalValueType::Real,
                    point_count,
                )?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real {
                    samples: finite_samples(LOCATION, "loop_gain_phase", &phase)?,
                },
            )?,
        ];

        let margins = &result.margins;
        let scalars = vec![
            real_scalar(
                LOCATION,
                "gain_margin_db",
                "Gain margin",
                decibel(),
                margins.gain_margin_db,
            )?,
            real_scalar(
                LOCATION,
                "gain_margin_frequency",
                "Gain margin frequency",
                SignalUnit::Hertz,
                margins.gain_margin_freq,
            )?,
            real_scalar(
                LOCATION,
                "phase_margin_degrees",
                "Phase margin",
                SignalUnit::Degree,
                margins.phase_margin_deg,
            )?,
            real_scalar(
                LOCATION,
                "phase_margin_frequency",
                "Phase margin frequency",
                SignalUnit::Hertz,
                margins.phase_margin_freq,
            )?,
            real_scalar(
                LOCATION,
                "dc_loop_gain_db",
                "DC loop gain",
                decibel(),
                margins.dc_gain_db,
            )?,
            real_scalar(
                LOCATION,
                "unity_gain_bandwidth",
                "Unity gain bandwidth",
                SignalUnit::Hertz,
                margins.unity_gain_bandwidth,
            )?,
            boolean_scalar(
                "conditionally_stable",
                "Conditionally stable",
                margins.conditionally_stable,
            )?,
            count_scalar(
                "unity_gain_crossovers",
                "Unity gain crossovers",
                margins.num_crossovers,
            )?,
        ];

        let mut nyquist = Vec::with_capacity(result.nyquist_points.len());
        for point in &result.nyquist_points {
            if !point.frequency.is_finite() || !point.real.is_finite() || !point.imag.is_finite() {
                return Err(source_error(LOCATION, "a Nyquist sample is not finite"));
            }
            nyquist.push(NyquistSample {
                frequency: point.frequency,
                real: point.real,
                imaginary: point.imag,
            });
        }

        let payload = StabilityPayload {
            success: result.success,
            warnings: result.warnings.clone(),
            nyquist,
        };
        let builder =
            Self::builder(analysis, ResultPayload::Stb(payload), point_count).scalars(scalars);
        Ok(if point_count == 0 {
            builder
        } else {
            builder.axis(axis).signals(signals)
        })
    }

    /// Project one `.SENS` result.
    pub fn from_sensitivity(
        analysis: AnalysisInstanceId,
        result: &SensitivityResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "sensitivity result";
        let mut entries = Vec::with_capacity(result.sensitivities.len());
        for sensitivity in &result.sensitivities {
            for (label, value) in [
                ("nominal value", sensitivity.nominal_value),
                ("absolute sensitivity", sensitivity.absolute),
                ("normalized sensitivity", sensitivity.normalized),
            ] {
                if !value.is_finite() {
                    return Err(source_error(
                        LOCATION,
                        format!("{label} of '{}' is {value}", sensitivity.vector_name),
                    ));
                }
            }
            entries.push(SensitivityEntry {
                vector_name: sensitivity.vector_name.clone(),
                element: sensitivity.element.clone(),
                element_kind: sensitivity.element_type.into(),
                parameter: sensitivity.parameter.clone(),
                nominal_value: sensitivity.nominal_value,
                absolute: sensitivity.absolute,
                normalized: sensitivity.normalized,
            });
        }
        let scalars = vec![real_scalar(
            LOCATION,
            "output_value",
            "Output value",
            SignalUnit::Dimensionless,
            result.output_value,
        )?];
        let payload = SensitivityPayload {
            output: result.output.clone(),
            entries,
        };
        Ok(Self::builder(analysis, ResultPayload::Sensitivity(payload), 0).scalars(scalars))
    }

    /// Project one `.PZ` result with its root-set evidence.
    pub fn from_pole_zero(
        analysis: AnalysisInstanceId,
        result: &PoleZeroResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "pole-zero result";
        let project = |roots: &[Complex64], role: &'static str| {
            roots
                .iter()
                .map(|root| {
                    if root.re.is_finite() && root.im.is_finite() {
                        Ok(ComplexSample::new(root.re, root.im))
                    } else {
                        Err(source_error(LOCATION, format!("a {role} is not finite")))
                    }
                })
                .collect::<Result<Vec<_>, _>>()
        };
        let payload = PoleZeroPayload {
            input: result.input.clone(),
            output: result.output.clone(),
            poles: project(&result.poles, "pole")?,
            zeros: project(&result.zeros, "zero")?,
            pole_evidence: RootSetEvidenceDocument::from(&result.pole_evidence),
            zero_evidence: RootSetEvidenceDocument::from(&result.zero_evidence),
            dc_gain: result.dc_gain,
            high_frequency_gain: result.hf_gain,
        };
        Ok(Self::builder(analysis, ResultPayload::PoleZero(payload), 0))
    }

    /// Project one `.FOUR` harmonic decomposition of an authored output.
    pub fn from_fourier(
        analysis: AnalysisInstanceId,
        parent: AnalysisInstanceId,
        output: &str,
        output_unit: SignalUnit,
        result: &FourierResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "Fourier result";
        if result.harmonics.is_empty() {
            return Err(source_error(LOCATION, "a Fourier result needs harmonics"));
        }
        let point_count = result.harmonics.len();
        let harmonic_axis = ResultAxis::new(
            "harmonic",
            "Harmonic",
            ResultAxisKind::HarmonicIndex,
            SignalUnit::Dimensionless,
            AxisValues::Integer {
                values: result
                    .harmonics
                    .iter()
                    .map(|harmonic| harmonic.harmonic_number as i64)
                    .collect(),
            },
        )?;
        let frequencies = result
            .harmonics
            .iter()
            .map(|harmonic| harmonic.frequency)
            .collect::<Vec<_>>();
        let frequency_axis = ResultAxis::new(
            "frequency",
            "Frequency",
            ResultAxisKind::Frequency,
            SignalUnit::Hertz,
            AxisValues::Real {
                values: finite_axis(LOCATION, "frequency", &frequencies)?,
            },
        )?;

        let magnitudes = result
            .harmonics
            .iter()
            .map(|harmonic| harmonic.magnitude)
            .collect::<Vec<_>>();
        let phases = result
            .harmonics
            .iter()
            .map(|harmonic| harmonic.phase)
            .collect::<Vec<_>>();
        let signals = vec![
            ResultSignal::new(
                analysis_descriptor(
                    LOCATION,
                    "harmonic_magnitude",
                    "Harmonic magnitude",
                    output_unit.clone(),
                    SignalValueType::Real,
                    point_count,
                )?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real {
                    samples: finite_samples(LOCATION, "harmonic_magnitude", &magnitudes)?,
                },
            )?,
            ResultSignal::new(
                analysis_descriptor(
                    LOCATION,
                    "harmonic_phase",
                    "Harmonic phase",
                    SignalUnit::Degree,
                    SignalValueType::Real,
                    point_count,
                )?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real {
                    samples: finite_samples(LOCATION, "harmonic_phase", &phases)?,
                },
            )?,
        ];

        let mut scalars = vec![
            real_scalar(
                LOCATION,
                "fundamental_frequency",
                "Fundamental frequency",
                SignalUnit::Hertz,
                result.fundamental_freq,
            )?,
            real_scalar(
                LOCATION,
                "dc_component",
                "DC component",
                output_unit,
                result.dc_component,
            )?,
        ];
        let thd = match result.thd {
            Some(value) if !value.is_finite() => {
                return Err(source_error(LOCATION, format!("THD is {value}")));
            }
            other => other,
        };
        scalars.push(ResultScalar::new(
            "total_harmonic_distortion",
            "THD",
            Some(percent()),
            ScalarValue::Real { value: thd },
        )?);

        Ok(Self::builder(
            analysis,
            ResultPayload::Fourier(FourierPayload {
                output: output.to_owned(),
            }),
            point_count,
        )
        .parent_analysis(parent)
        .axis(harmonic_axis)
        .axis(frequency_axis)
        .signals(signals)
        .scalars(scalars))
    }

    /// Project one transient `.FFT` spectrum.
    pub fn from_transient_fft(
        analysis: AnalysisInstanceId,
        parent: AnalysisInstanceId,
        output_unit: SignalUnit,
        result: &TransientFftResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "transient FFT result";
        if result.bins.is_empty() {
            return Err(source_error(
                LOCATION,
                "an FFT spectrum needs at least one bin",
            ));
        }
        let point_count = result.bins.len();
        let bin_axis = ResultAxis::new(
            "bin",
            "Bin",
            ResultAxisKind::BinIndex,
            SignalUnit::Dimensionless,
            AxisValues::Integer {
                values: result.bins.iter().map(|bin| bin.index as i64).collect(),
            },
        )?;
        let frequencies = result
            .bins
            .iter()
            .map(|bin| bin.frequency)
            .collect::<Vec<_>>();
        let frequency_axis = ResultAxis::new(
            "frequency",
            "Frequency",
            ResultAxisKind::Frequency,
            SignalUnit::Hertz,
            AxisValues::Real {
                values: finite_axis(LOCATION, "frequency", &frequencies)?,
            },
        )?;

        let coefficients = result
            .bins
            .iter()
            .map(|bin| Complex64::new(bin.real, bin.imaginary))
            .collect::<Vec<_>>();
        let magnitudes = result
            .bins
            .iter()
            .map(|bin| bin.magnitude)
            .collect::<Vec<_>>();
        let phases = result
            .bins
            .iter()
            .map(|bin| bin.phase_degrees)
            .collect::<Vec<_>>();
        let signals = vec![
            ResultSignal::new(
                analysis_descriptor(
                    LOCATION,
                    "spectrum",
                    "Spectrum",
                    output_unit.clone(),
                    SignalValueType::Complex,
                    point_count,
                )?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Complex {
                    samples: finite_complex_samples(LOCATION, "spectrum", &coefficients)?,
                },
            )?,
            ResultSignal::new(
                analysis_descriptor(
                    LOCATION,
                    "spectrum_magnitude",
                    "Magnitude",
                    output_unit,
                    SignalValueType::Real,
                    point_count,
                )?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real {
                    samples: finite_samples(LOCATION, "spectrum_magnitude", &magnitudes)?,
                },
            )?,
            ResultSignal::new(
                analysis_descriptor(
                    LOCATION,
                    "spectrum_phase",
                    "Phase",
                    SignalUnit::Degree,
                    SignalValueType::Real,
                    point_count,
                )?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real {
                    samples: finite_samples(LOCATION, "spectrum_phase", &phases)?,
                },
            )?,
        ];

        let metrics = match &result.metrics {
            None => None,
            Some(metrics) => {
                let mut harmonics = Vec::with_capacity(metrics.largest_harmonics.len());
                for harmonic in &metrics.largest_harmonics {
                    harmonics.push(super::payload::FftHarmonicDocument {
                        rank: harmonic.rank,
                        bin: harmonic.bin,
                        frequency: harmonic.frequency,
                        magnitude: harmonic.magnitude,
                        magnitude_db: harmonic.magnitude_db,
                        phase_degrees: harmonic.phase_degrees,
                    });
                }
                Some(FftMetricsDocument {
                    fundamental_magnitude: metrics.fundamental_magnitude,
                    thd_ratio: metrics.thd_ratio,
                    thd_db: metrics.thd_db,
                    sndr_db: metrics.sndr_db,
                    enob_bits: metrics.enob_bits,
                    snr_db: metrics.snr_db,
                    sfdr_db: metrics.sfdr_db,
                    sfdr_spur_bin: metrics.sfdr_spur_bin,
                    sfdr_spur_frequency: metrics.sfdr_spur_frequency,
                    largest_harmonics: harmonics,
                })
            }
        };

        let payload = FftPayload {
            source: FftSourceDocument::from(&result.output),
            output_name: result.output_name.clone(),
            physical_type: result.physical_type.to_owned(),
            start_time: result.start_time,
            stop_time: result.stop_time,
            sample_interval: result.sample_interval,
            sample_count: result.point_count,
            accurate_sampling: result.accurate_sampling,
            coefficient_format: result.format.into(),
            compatibility_mode: result.mode.into(),
            window: result.window.into(),
            window_name: result.window_name.clone(),
            alpha: result.alpha,
            coherent_gain: result.coherent_gain,
            frequency_resolution: result.frequency_resolution,
            fundamental_bin: result.fundamental_bin,
            minimum_metric_bin: result.minimum_metric_bin,
            maximum_metric_bin: result.maximum_metric_bin,
            metrics,
        };

        Ok(
            Self::builder(analysis, ResultPayload::Fft(payload), point_count)
                .parent_analysis(parent)
                .axis(bin_axis)
                .axis(frequency_axis)
                .signals(signals),
        )
    }

    /// Project one Monte Carlo run.
    ///
    /// Per-trial samples stay in the payload's statistics because the producing
    /// result declares no unit for an output variable, and the document does
    /// not invent one.
    pub fn from_monte_carlo(
        analysis: AnalysisInstanceId,
        result: &MonteCarloResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "Monte Carlo result";
        let mut names = result.variables.keys().cloned().collect::<Vec<_>>();
        names.sort();
        let mut statistics = Vec::with_capacity(names.len());
        for name in names {
            let variable = result
                .variables
                .get(&name)
                .ok_or_else(|| source_error(LOCATION, "a Monte Carlo variable vanished"))?;
            let samples = finite_samples(LOCATION, &name, &variable.samples)?;
            if !variable.bin_edges.is_empty()
                && variable.bin_edges.len() != variable.histogram.len() + 1
            {
                return Err(source_error(
                    LOCATION,
                    format!("variable '{name}' has an inconsistent histogram"),
                ));
            }
            statistics.push(MonteCarloVariableStatistics {
                name: variable.name.clone(),
                samples,
                mean: defined_or_missing(variable.mean),
                standard_deviation: defined_or_missing(variable.std_dev),
                minimum: defined_or_missing(variable.min),
                maximum: defined_or_missing(variable.max),
                histogram: variable.histogram.clone(),
                bin_edges: finite_axis(LOCATION, "histogram edge", &variable.bin_edges)?,
            });
        }

        let scalars = vec![
            count_scalar("completed_runs", "Completed runs", result.num_runs)?,
            count_scalar("failed_runs", "Failed runs", result.num_failures)?,
            boolean_scalar("all_converged", "All runs converged", result.all_converged)?,
        ];
        Ok(Self::builder(
            analysis,
            ResultPayload::MonteCarlo(MonteCarloPayload { statistics }),
            0,
        )
        .scalars(scalars))
    }

    /// Project one periodic steady-state result.
    pub fn from_pss(
        analysis: AnalysisInstanceId,
        result: &PssResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "PSS result";
        let point_count = result.time.len();
        if point_count == 0 {
            return Err(source_error(LOCATION, "a PSS result needs a time grid"));
        }
        require_named_columns(
            LOCATION,
            "PSS periodic waveforms",
            &result.node_names,
            result.waveforms.len(),
        )?;
        let axis = ResultAxis::new(
            "time",
            "Time",
            ResultAxisKind::Time,
            SignalUnit::Second,
            AxisValues::Real {
                values: finite_axis(LOCATION, "time", &result.time)?,
            },
        )?;
        let mut signals = Vec::with_capacity(result.waveforms.len());
        for (name, waveform) in result.node_names.iter().zip(&result.waveforms) {
            if waveform.values.len() != point_count {
                return Err(source_error(
                    LOCATION,
                    format!(
                        "periodic waveform '{name}' has {} samples for {point_count} times",
                        waveform.values.len()
                    ),
                ));
            }
            signals.push(ResultSignal::new(
                voltage_descriptor(LOCATION, name, SignalValueType::Real, point_count)?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real {
                    samples: finite_samples(LOCATION, name, &waveform.values)?,
                },
            )?);
        }

        let scalars = vec![
            real_scalar(
                LOCATION,
                "period",
                "Period",
                SignalUnit::Second,
                result.period,
            )?,
            real_scalar(
                LOCATION,
                "fundamental_frequency",
                "Fundamental frequency",
                SignalUnit::Hertz,
                result.frequency,
            )?,
            count_scalar(
                "shooting_iterations",
                "Shooting iterations",
                result.iterations,
            )?,
            real_scalar(
                LOCATION,
                "residual_norm",
                "Residual norm",
                SignalUnit::Dimensionless,
                result.residual_norm,
            )?,
            boolean_scalar(
                "period_detected",
                "Period auto-detected",
                result.period_detected,
            )?,
        ];

        let mut multipliers = Vec::with_capacity(result.floquet_multipliers.len());
        for multiplier in &result.floquet_multipliers {
            if !multiplier.re.is_finite() || !multiplier.im.is_finite() {
                return Err(source_error(LOCATION, "a Floquet multiplier is not finite"));
            }
            multipliers.push(ComplexSample::new(multiplier.re, multiplier.im));
        }
        let payload = super::payload::PssPayload {
            floquet_multipliers: multipliers,
            floquet_evidence: FloquetEvidenceDocument::from(&result.floquet_evidence),
            floquet_orbit_kind: result.floquet_orbit_kind.into(),
            trivial_floquet_multiplier_index: result.trivial_floquet_multiplier_index,
        };

        Ok(
            Self::builder(analysis, ResultPayload::Pss(payload), point_count)
                .axis(axis)
                .signals(signals)
                .scalars(scalars),
        )
    }

    /// Project one periodic AC result with all of its sideband spectra.
    pub fn from_pac(
        analysis: AnalysisInstanceId,
        result: &PacResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "PAC result";
        let point_count = result.frequencies.len();
        if point_count == 0 {
            return Err(source_error(
                LOCATION,
                "a PAC sweep needs at least one frequency",
            ));
        }
        let axis = ResultAxis::new(
            "offset_frequency",
            "Offset frequency",
            ResultAxisKind::OffsetFrequency,
            SignalUnit::Hertz,
            AxisValues::Real {
                values: finite_axis(LOCATION, "offset frequency", &result.frequencies)?,
            },
        )?;

        let mut signals = Vec::new();
        let mut sidebands = Vec::new();
        for sideband in result.sideband_indices() {
            let qualifier = SeriesQualifier::PacSideband { sideband };
            let mut offsets = Vec::with_capacity(point_count);
            let mut absolutes = Vec::with_capacity(point_count);
            let mut node_columns = vec![Vec::with_capacity(point_count); result.node_names.len()];
            let mut branch_columns =
                vec![Vec::with_capacity(point_count); result.branch_names.len()];
            for index in 0..point_count {
                let data = result.get_sideband_data(index, sideband).ok_or_else(|| {
                    source_error(
                        LOCATION,
                        format!("sideband {sideband} is missing at frequency index {index}"),
                    )
                })?;
                offsets.push(data.frequency_offset);
                absolutes.push(data.absolute_frequency);
                if data.node_voltages.len() != result.node_names.len()
                    || data.branch_currents.len() != result.branch_names.len()
                {
                    return Err(source_error(
                        LOCATION,
                        "a PAC sideband record disagrees with the result schema",
                    ));
                }
                for (column, value) in node_columns.iter_mut().zip(&data.node_voltages) {
                    column.push(*value);
                }
                for (column, value) in branch_columns.iter_mut().zip(&data.branch_currents) {
                    column.push(*value);
                }
            }
            for (name, column) in result.node_names.iter().zip(&node_columns) {
                signals.push(ResultSignal::new(
                    voltage_descriptor(LOCATION, name, SignalValueType::Complex, point_count)?,
                    Some(qualifier.clone()),
                    SeriesAvailability::Available,
                    SeriesValues::Complex {
                        samples: finite_complex_samples(LOCATION, name, column)?,
                    },
                )?);
            }
            for (name, column) in result.branch_names.iter().zip(&branch_columns) {
                signals.push(ResultSignal::new(
                    current_descriptor(LOCATION, name, SignalValueType::Complex, point_count)?,
                    Some(qualifier.clone()),
                    SeriesAvailability::Available,
                    SeriesValues::Complex {
                        samples: finite_complex_samples(LOCATION, name, column)?,
                    },
                )?);
            }
            sidebands.push(PacSidebandDescriptor {
                sideband,
                frequency_offsets: finite_axis(LOCATION, "sideband offset", &offsets)?,
                absolute_frequencies: finite_axis(LOCATION, "sideband frequency", &absolutes)?,
            });
        }

        let conversion_matrix = if result.conversion_matrix.is_materialized() {
            let mut entries = Vec::new();
            let sideband_indices = result.conversion_matrix.sideband_indices();
            for frequency_index in 0..result.conversion_matrix.num_frequencies() {
                for output_sideband in &sideband_indices {
                    for input_sideband in &sideband_indices {
                        let value = result
                            .conversion_matrix
                            .get(frequency_index, *output_sideband, *input_sideband)
                            .map_err(|error| source_error(LOCATION, error.to_string()))?;
                        if !value.re.is_finite() || !value.im.is_finite() {
                            return Err(source_error(
                                LOCATION,
                                "a PAC conversion element is not finite",
                            ));
                        }
                        entries.push(PacConversionEntry {
                            frequency_index,
                            output_sideband: *output_sideband,
                            input_sideband: *input_sideband,
                            value: ComplexSample::new(value.re, value.im),
                        });
                    }
                }
            }
            Some(PacConversionMatrixDocument { entries })
        } else {
            None
        };

        let scalars = vec![
            count_scalar("newton_iterations", "Newton iterations", result.iterations)?,
            real_scalar(
                LOCATION,
                "residual_norm",
                "Residual norm",
                SignalUnit::Dimensionless,
                result.residual,
            )?,
        ];

        let payload = PacPayload {
            fundamental_frequency: result.fundamental_frequency,
            sideband_minimum: result.sideband_min,
            sideband_maximum: result.sideband_max,
            input_source: result.input_source.clone(),
            output_node: result.output_node.clone(),
            iterations: result.iterations,
            residual: result.residual,
            sidebands,
            conversion_matrix,
        };

        Ok(
            Self::builder(analysis, ResultPayload::Pac(payload), point_count)
                .axis(axis)
                .signals(signals)
                .scalars(scalars),
        )
    }

    /// Project one phase-noise result.
    pub fn from_pnoise(
        analysis: AnalysisInstanceId,
        result: &PnoiseResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "PNoise result";
        let point_count = result.spectral_points.len();
        if point_count == 0 {
            return Err(source_error(
                LOCATION,
                "a phase-noise sweep needs at least one point",
            ));
        }
        let offsets = result
            .spectral_points
            .iter()
            .map(|point| point.offset_freq)
            .collect::<Vec<_>>();
        let axis = ResultAxis::new(
            "offset_frequency",
            "Offset frequency",
            ResultAxisKind::OffsetFrequency,
            SignalUnit::Hertz,
            AxisValues::Real {
                values: finite_axis(LOCATION, "offset frequency", &offsets)?,
            },
        )?;

        let density = result
            .spectral_points
            .iter()
            .map(|point| point.pn_dbc_hz)
            .collect::<Vec<_>>();
        let mut signals = vec![ResultSignal::new(
            analysis_descriptor(
                LOCATION,
                "phase_noise",
                "Phase noise",
                dbc_per_hertz(),
                SignalValueType::Real,
                point_count,
            )?,
            None,
            SeriesAvailability::Available,
            SeriesValues::Real {
                samples: finite_samples(LOCATION, "phase_noise", &density)?,
            },
        )?];

        let optional: [(&str, &str, fn(&PhaseNoisePoint) -> Option<Value>); 3] = [
            ("am_noise", "AM noise", |point| point.am_noise),
            ("upper_sideband_noise", "Upper sideband", |point| {
                point.upper_sideband
            }),
            ("lower_sideband_noise", "Lower sideband", |point| {
                point.lower_sideband
            }),
        ];
        for (canonical, display, extract) in optional {
            let column = result
                .spectral_points
                .iter()
                .map(extract)
                .collect::<Vec<_>>();
            if column.iter().all(Option::is_none) {
                continue;
            }
            for value in column.iter().flatten() {
                if !value.is_finite() {
                    return Err(source_error(
                        LOCATION,
                        format!("'{canonical}' sample is {value}"),
                    ));
                }
            }
            signals.push(ResultSignal::new(
                analysis_descriptor(
                    LOCATION,
                    canonical,
                    display,
                    dbc_per_hertz(),
                    SignalValueType::Real,
                    point_count,
                )?,
                None,
                SeriesAvailability::Available,
                SeriesValues::Real { samples: column },
            )?);
        }

        let mut scalars = vec![
            real_scalar(
                LOCATION,
                "carrier_frequency",
                "Carrier frequency",
                SignalUnit::Hertz,
                result.carrier_freq,
            )?,
            boolean_scalar("converged", "Converged", result.converged)?,
        ];
        if let Some(jitter) = result.rms_jitter {
            scalars.push(real_scalar(
                LOCATION,
                "rms_jitter",
                "RMS jitter",
                SignalUnit::Second,
                jitter,
            )?);
        }
        if let Some(phase_error) = result.rms_phase_error {
            scalars.push(real_scalar(
                LOCATION,
                "rms_phase_error",
                "RMS phase error",
                SignalUnit::Radian,
                phase_error,
            )?);
        }

        let mut contributors = Vec::with_capacity(result.contributors.len());
        for contributor in &result.contributors {
            let mut contributions = Vec::with_capacity(contributor.contributions.len());
            for (offset, value) in &contributor.contributions {
                if !offset.is_finite() || !value.is_finite() {
                    return Err(source_error(
                        LOCATION,
                        format!("contributor '{}' has a non-finite sample", contributor.name),
                    ));
                }
                contributions.push(PNoiseContribution {
                    offset_frequency: *offset,
                    contribution_dbc_per_hz: *value,
                });
            }
            contributors.push(PNoiseContributor {
                name: contributor.name.clone(),
                device_type: contributor.device_type.clone(),
                contributions,
                percentage: contributor.percentage,
            });
        }

        let payload = PNoisePayload {
            output_node: result.output_node.clone(),
            jitter_bandwidth: result
                .jitter_bandwidth
                .map(|(start, stop)| PNoiseBandwidth { start, stop }),
            contributors,
        };

        Ok(
            Self::builder(analysis, ResultPayload::PNoise(payload), point_count)
                .axis(axis)
                .signals(signals)
                .scalars(scalars),
        )
    }

    /// Project one harmonic-balance result.
    pub fn from_harmonic_balance(
        analysis: AnalysisInstanceId,
        result: &HbResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "harmonic-balance result";
        let (axes, signals) = harmonic_balance_series(LOCATION, result)?;
        let point_count = result.harmonic_frequencies.len();
        let scalars = vec![
            boolean_scalar("converged", "Converged", result.converged)?,
            count_scalar("newton_iterations", "Newton iterations", result.iterations)?,
            real_scalar(
                LOCATION,
                "residual_norm",
                "Residual norm",
                SignalUnit::Dimensionless,
                result.residual_norm,
            )?,
            real_scalar(
                LOCATION,
                "fundamental_frequency",
                "Fundamental frequency",
                SignalUnit::Hertz,
                result.fundamental_freq,
            )?,
            count_scalar("harmonic_count", "Harmonics", result.num_harmonics)?,
            real_scalar(
                LOCATION,
                "solve_time_seconds",
                "Solve time",
                SignalUnit::Second,
                result.solve_time_seconds,
            )?,
        ];

        let mut reactive_spectra = Vec::with_capacity(result.reactive_spectra.len());
        for spectrum in &result.reactive_spectra {
            reactive_spectra.push(HbReactiveSpectrumDocument {
                device_name: spectrum.device_name.clone(),
                kind: spectrum.kind.into(),
                voltage_coefficients: complex_document_samples(
                    LOCATION,
                    &spectrum.device_name,
                    &spectrum.voltage_coefficients,
                )?,
                current_coefficients: complex_document_samples(
                    LOCATION,
                    &spectrum.device_name,
                    &spectrum.current_coefficients,
                )?,
                dc_current_is_exact: spectrum.dc_current_is_exact,
            });
        }

        let payload = HarmonicBalancePayload {
            tones: result.tones.clone(),
            reactive_spectra,
            continuation_limitations: result
                .continuation_limitations
                .iter()
                .map(Into::into)
                .collect(),
        };

        let mut builder = Self::builder(analysis, ResultPayload::Hb(payload), point_count);
        for axis in axes {
            builder = builder.axis(axis);
        }
        Ok(builder.signals(signals).scalars(scalars))
    }

    /// Project one envelope-following result.
    pub fn from_envelope(
        analysis: AnalysisInstanceId,
        result: &EnvelopeResult,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        const LOCATION: &str = "envelope result";
        let transient =
            Self::from_transient(analysis, result.continued_transient(), None, Vec::new())?;
        let carrier = result.carrier();
        let mut node_spectra = Vec::with_capacity(carrier.spectral_voltages.len());
        for spectrum in &carrier.spectral_voltages {
            node_spectra.push(EnvelopeNodeSpectrum {
                node_name: spectrum.node_name.clone(),
                coefficients: complex_document_samples(
                    LOCATION,
                    &spectrum.node_name,
                    &spectrum.coefficients,
                )?,
            });
        }
        let state = result.state();
        let payload = EnvelopePayload {
            continuation: EnvelopeContinuationDocument {
                guarantee: result.guarantee().into(),
                carrier_fundamental_frequency: state.fundamental_freq(),
                carrier_harmonics: state.num_harmonics(),
                hb_config_identity: state.hb_config_identity().to_owned(),
                canonical_frozen_sources: state.canonical_frozen_sources().to_vec(),
                original_netlist_identity: state.original_netlist_identity().to_owned(),
                resolved_simulation_identity: state.resolved_simulation_identity().to_owned(),
                history_step: state.history_step(),
                time_origin: state.time_origin(),
                slow_time_duration: result.slow_time_duration(),
                slow_time_max_step: result.slow_time_max_step(),
            },
            carrier: EnvelopeCarrierDocument {
                converged: carrier.converged,
                iterations: carrier.iterations,
                residual_norm: carrier.residual_norm,
                fundamental_frequency: carrier.fundamental_freq,
                harmonic_frequencies: finite_axis(
                    LOCATION,
                    "carrier harmonic frequency",
                    &carrier.harmonic_frequencies,
                )?,
                node_spectra,
            },
            transient: transient_payload_of(&transient)?,
        };
        Ok(transient.replace_payload(ResultPayload::Envelope(payload)))
    }
}

//=============================================================================
// Shared projections
//=============================================================================

fn complex_document_samples(
    location: &'static str,
    name: &str,
    values: &[Complex64],
) -> Result<Vec<ComplexSample>, ResultDocumentError> {
    values
        .iter()
        .map(|value| {
            if value.re.is_finite() && value.im.is_finite() {
                Ok(ComplexSample::new(value.re, value.im))
            } else {
                Err(source_error(
                    location,
                    format!("'{name}' has a non-finite coefficient"),
                ))
            }
        })
        .collect()
}

fn projected_real_series(
    location: &'static str,
    descriptor: SignalDescriptor,
    name: &str,
    waveform: &[Value],
    point_count: usize,
) -> Result<ResultSignal, ResultDocumentError> {
    if waveform.is_empty() {
        return ResultSignal::new(
            descriptor,
            None,
            SeriesAvailability::NotProjected,
            SeriesValues::Real {
                samples: vec![None; point_count],
            },
        );
    }
    if waveform.len() != point_count {
        return Err(source_error(
            location,
            format!(
                "'{name}' has {} samples for {point_count} times",
                waveform.len()
            ),
        ));
    }
    ResultSignal::new(
        descriptor,
        None,
        SeriesAvailability::Available,
        SeriesValues::Real {
            samples: finite_samples(location, name, waveform)?,
        },
    )
}

fn complex_frequency_sweep<T>(
    location: &'static str,
    points: &[T],
    frequency: impl Fn(&T) -> Value,
    voltages: impl Fn(&T) -> (&Vec<String>, &Vec<Complex64>),
    currents: impl Fn(&T) -> (&Vec<String>, &Vec<Complex64>),
    qualifier: Option<SeriesQualifier>,
) -> Result<(ResultAxis, Vec<ResultSignal>), ResultDocumentError> {
    let first = points
        .first()
        .ok_or_else(|| source_error(location, "a frequency sweep needs at least one point"))?;
    let point_count = points.len();
    let (node_names, node_values) = voltages(first);
    let (branch_names, branch_values) = currents(first);
    require_named_columns(location, "node voltages", node_names, node_values.len())?;
    require_named_columns(
        location,
        "branch currents",
        branch_names,
        branch_values.len(),
    )?;
    for point in points {
        if voltages(point).0 != node_names || currents(point).0 != branch_names {
            return Err(source_error(
                location,
                "frequency points do not share one signal schema",
            ));
        }
    }

    let frequencies = points.iter().map(&frequency).collect::<Vec<_>>();
    let axis = ResultAxis::new(
        "frequency",
        "Frequency",
        ResultAxisKind::Frequency,
        SignalUnit::Hertz,
        AxisValues::Real {
            values: finite_axis(location, "frequency", &frequencies)?,
        },
    )?;

    let mut signals = Vec::with_capacity(node_names.len() + branch_names.len());
    for (index, name) in node_names.iter().enumerate() {
        let column = points
            .iter()
            .map(|point| {
                voltages(point).1.get(index).copied().ok_or_else(|| {
                    source_error(location, format!("node '{name}' is missing at a point"))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        signals.push(ResultSignal::new(
            voltage_descriptor(location, name, SignalValueType::Complex, point_count)?,
            qualifier.clone(),
            SeriesAvailability::Available,
            SeriesValues::Complex {
                samples: finite_complex_samples(location, name, &column)?,
            },
        )?);
    }
    for (index, name) in branch_names.iter().enumerate() {
        let column = points
            .iter()
            .map(|point| {
                currents(point).1.get(index).copied().ok_or_else(|| {
                    source_error(location, format!("branch '{name}' is missing at a point"))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        signals.push(ResultSignal::new(
            current_descriptor(location, name, SignalValueType::Complex, point_count)?,
            qualifier.clone(),
            SeriesAvailability::Available,
            SeriesValues::Complex {
                samples: finite_complex_samples(location, name, &column)?,
            },
        )?);
    }
    Ok((axis, signals))
}

fn distortion_response_signals(
    location: &'static str,
    responses: &[AcResult],
    qualifier: Option<SeriesQualifier>,
) -> Result<Vec<ResultSignal>, ResultDocumentError> {
    let (_, signals) = complex_frequency_sweep(
        location,
        responses,
        |response| response.frequency,
        |response| (&response.node_names, &response.voltages),
        |response| (&response.branch_names, &response.currents),
        qualifier,
    )?;
    Ok(signals)
}

fn harmonic_balance_series(
    location: &'static str,
    result: &HbResult,
) -> Result<(Vec<ResultAxis>, Vec<ResultSignal>), ResultDocumentError> {
    let point_count = result.harmonic_frequencies.len();
    if point_count == 0 {
        return Err(source_error(
            location,
            "an HB result needs harmonic frequencies",
        ));
    }
    let harmonic_axis = ResultAxis::new(
        "harmonic",
        "Harmonic",
        ResultAxisKind::HarmonicIndex,
        SignalUnit::Dimensionless,
        AxisValues::Integer {
            values: (0..point_count).map(|index| index as i64).collect(),
        },
    )?;
    let frequency_axis = ResultAxis::new(
        "frequency",
        "Frequency",
        ResultAxisKind::Frequency,
        SignalUnit::Hertz,
        AxisValues::Real {
            values: finite_axis(location, "harmonic frequency", &result.harmonic_frequencies)?,
        },
    )?;

    let mut signals =
        Vec::with_capacity(result.spectral_voltages.len() + result.mna_branch_currents.len());
    for spectrum in &result.spectral_voltages {
        check_harmonic_grid(
            location,
            &spectrum.node_name,
            &spectrum.frequencies,
            &result.harmonic_frequencies,
            spectrum.coefficients.len(),
        )?;
        signals.push(ResultSignal::new(
            voltage_descriptor(
                location,
                &spectrum.node_name,
                SignalValueType::Complex,
                point_count,
            )?,
            None,
            SeriesAvailability::Available,
            SeriesValues::Complex {
                samples: finite_complex_samples(
                    location,
                    &spectrum.node_name,
                    &spectrum.coefficients,
                )?,
            },
        )?);
    }
    for spectrum in &result.mna_branch_currents {
        check_harmonic_grid(
            location,
            &spectrum.device_name,
            &spectrum.frequencies,
            &result.harmonic_frequencies,
            spectrum.coefficients.len(),
        )?;
        signals.push(ResultSignal::new(
            current_descriptor(
                location,
                &spectrum.device_name,
                SignalValueType::Complex,
                point_count,
            )?,
            None,
            SeriesAvailability::Available,
            SeriesValues::Complex {
                samples: finite_complex_samples(
                    location,
                    &spectrum.device_name,
                    &spectrum.coefficients,
                )?,
            },
        )?);
    }
    Ok((vec![harmonic_axis, frequency_axis], signals))
}

fn check_harmonic_grid(
    location: &'static str,
    name: &str,
    declared: &[Value],
    expected: &[Value],
    coefficient_count: usize,
) -> Result<(), ResultDocumentError> {
    if coefficient_count != expected.len() {
        return Err(source_error(
            location,
            format!(
                "'{name}' has {coefficient_count} coefficients for {} harmonics",
                expected.len()
            ),
        ));
    }
    if !declared.is_empty() && declared != expected {
        return Err(source_error(
            location,
            format!("'{name}' declares a different harmonic frequency grid"),
        ));
    }
    Ok(())
}

fn transient_payload_of(
    builder: &AnalysisResultDocumentBuilder,
) -> Result<TransientPayload, ResultDocumentError> {
    match builder.payload_ref() {
        ResultPayload::Tran(payload) => Ok(payload.clone()),
        other => Err(source_error(
            "envelope result",
            format!(
                "the continued transient produced a {} payload",
                other.result_kind().tag()
            ),
        )),
    }
}
