//! Long-run transient controls: compression and resumable checkpoints.
//!
//! `CompressedTransientResult` carries an error-bounded reduction of the full
//! analog waveform inventory, so a multi-hour run stays addressable without
//! holding every timepoint.
//! `TransientCheckpoint` carries the netlist-fingerprinted state a resumed run
//! restarts from; the fingerprint is what stops a checkpoint being replayed
//! against a deck it was not produced from.

use super::*;

/// Memory-decimated transient analog waveforms with bounded interpolation error.
#[pyclass(name = "CompressedTransientResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyCompressedTransientResult {
    inner: rspice_core::engine::TransientResultCompressed,
}

const COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION: usize = 3;
type CompressionErrorPersistenceState =
    (String, String, usize, f64, f64, f64, Option<f64>, f64, f64);
type CompressionReportPersistenceState = (
    u32,
    String,
    String,
    bool,
    f64,
    f64,
    f64,
    usize,
    usize,
    Option<CompressionErrorPersistenceState>,
);

/// One descriptor-keyed channel: role tag, node index, owner name, device
/// parameter, unit tag, availability tag, per-sample values, per-sample
/// absence reasons. A sample is present in exactly one of the last two
/// vectors, which is how the validity mask survives a round trip.
type CompressedChannelPersistenceState = (
    String,
    usize,
    String,
    String,
    String,
    String,
    Vec<Option<f64>>,
    Vec<Option<String>>,
);
/// One digital event trace: node name and `(time, state, strength)` events.
type CompressedDigitalTracePersistenceState = (String, Vec<(f64, String, String)>);
/// One real event trace: node name and `(time, value)` events.
type CompressedRealTracePersistenceState = (String, Vec<(f64, f64)>);
/// Analysis identity, coordinate identity, and topology fingerprint.
type CompressedIdentityPersistenceState = (
    Option<(String, u32)>,
    Option<(Vec<u8>, u32, usize, String)>,
    Option<Vec<u8>>,
);
/// One `.FOUR` operand result: card index, output, physical type, authored
/// fundamental and harmonic count, then the spectrum itself.
type CompressedFourierPersistenceState = (
    usize,
    String,
    String,
    f64,
    usize,
    f64,
    f64,
    Option<f64>,
    Vec<(usize, f64, f64, f64)>,
);
/// One transient `.MEASURE` result, field for field.
type CompressedMeasurementPersistenceState = (
    String,
    Option<f64>,
    Option<f64>,
    Option<String>,
    bool,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    bool,
    Option<f64>,
);
type CompressedTransientAnalogState = (
    usize,
    Vec<f64>,
    Vec<CompressedChannelPersistenceState>,
    Vec<CompressedDigitalTracePersistenceState>,
    Vec<CompressedRealTracePersistenceState>,
    CompressedIdentityPersistenceState,
    Vec<CompressedFourierPersistenceState>,
    Vec<CompressedMeasurementPersistenceState>,
);

fn channel_role_tag(role: &rspice_core::engine::TransientChannelRole) -> &'static str {
    use rspice_core::engine::TransientChannelRole;
    match role {
        TransientChannelRole::NodeVoltage { .. } => "node-voltage",
        TransientChannelRole::BranchCurrent { .. } => "branch-current",
        TransientChannelRole::DeviceObservable { .. } => "device-observable",
        TransientChannelRole::DeviceStore { .. } => "device-store",
    }
}

fn channel_persistence_state(
    channel: &rspice_core::engine::TransientCompressedChannel,
) -> CompressedChannelPersistenceState {
    use rspice_core::engine::TransientChannelRole;
    let role = channel.descriptor.role();
    let (node_index, owner, parameter) = match role {
        TransientChannelRole::NodeVoltage { node_index, node } => {
            (*node_index, node.clone(), String::new())
        }
        TransientChannelRole::BranchCurrent { branch } => (0, branch.clone(), String::new()),
        TransientChannelRole::DeviceObservable { device, parameter } => {
            (0, device.clone(), parameter.clone())
        }
        TransientChannelRole::DeviceStore { store } => (0, store.clone(), String::new()),
    };
    let values = channel
        .samples
        .iter()
        .map(|sample| sample.value())
        .collect::<Vec<_>>();
    let absence = channel
        .samples
        .iter()
        .map(|sample| sample.absence().map(|reason| reason.as_str().to_string()))
        .collect::<Vec<_>>();
    (
        channel_role_tag(role).to_string(),
        node_index,
        owner,
        parameter,
        channel.descriptor.unit().as_str().to_string(),
        channel.availability.as_str().to_string(),
        values,
        absence,
    )
}

fn rebuild_channel(
    state: CompressedChannelPersistenceState,
) -> PyResult<rspice_core::engine::TransientCompressedChannel> {
    use rspice_core::engine::{
        TransientChannelAvailability, TransientChannelDescriptor, TransientChannelRole,
        TransientChannelSample, TransientChannelUnit, TransientCompressedChannel,
        TransientSampleAbsence,
    };
    let (role_tag, node_index, owner, parameter, unit, availability, values, absence) = state;
    let role = match role_tag.as_str() {
        "node-voltage" => TransientChannelRole::NodeVoltage {
            node_index,
            node: owner,
        },
        "branch-current" => TransientChannelRole::BranchCurrent { branch: owner },
        "device-observable" => TransientChannelRole::DeviceObservable {
            device: owner,
            parameter,
        },
        "device-store" => TransientChannelRole::DeviceStore { store: owner },
        _ => {
            return Err(crate::errors::value_error(format!(
                "unsupported compressed-transient channel role '{role_tag}'"
            )));
        }
    };
    let unit = TransientChannelUnit::from_tag(&unit).map_err(crate::errors::value_error)?;
    let availability = TransientChannelAvailability::from_tag(&availability).ok_or_else(|| {
        crate::errors::value_error(format!(
            "unsupported compressed-transient channel availability '{availability}'"
        ))
    })?;
    if values.len() != absence.len() {
        return Err(crate::errors::value_error(format!(
            "compressed-transient channel pickle has {} values for {} validity entries",
            values.len(),
            absence.len()
        )));
    }
    let mut samples = Vec::with_capacity(values.len());
    for (index, (value, reason)) in values.into_iter().zip(absence).enumerate() {
        samples.push(match (value, reason) {
            (Some(value), None) => TransientChannelSample::Value(value),
            (None, Some(reason)) => TransientChannelSample::Absent(
                TransientSampleAbsence::from_tag(&reason).ok_or_else(|| {
                    crate::errors::value_error(format!(
                        "unsupported compressed-transient sample absence reason '{reason}'"
                    ))
                })?,
            ),
            _ => {
                return Err(crate::errors::value_error(format!(
                    "compressed-transient channel pickle sample {index} is neither a value nor a typed absence"
                )));
            }
        });
    }
    Ok(TransientCompressedChannel {
        descriptor: TransientChannelDescriptor::new(role, unit)
            .map_err(crate::errors::value_error)?,
        availability,
        samples,
    })
}

fn digital_trace_persistence_state(
    trace: &rspice_core::engine::DigitalTrace,
) -> CompressedDigitalTracePersistenceState {
    (
        trace.node_name.clone(),
        trace
            .points
            .iter()
            .map(|point| {
                (
                    point.time,
                    rspice_core::engine::digital_state_tag(point.value.state).to_string(),
                    rspice_core::engine::digital_strength_tag(point.value.strength).to_string(),
                )
            })
            .collect(),
    )
}

fn rebuild_digital_trace(
    state: CompressedDigitalTracePersistenceState,
) -> PyResult<rspice_core::engine::DigitalTrace> {
    let (node_name, points) = state;
    let mut rebuilt = Vec::with_capacity(points.len());
    for (time, state_tag, strength_tag) in points {
        let state = rspice_core::engine::digital_state_from_tag(&state_tag).ok_or_else(|| {
            crate::errors::value_error(format!(
                "unsupported compressed-transient digital state '{state_tag}'"
            ))
        })?;
        let strength =
            rspice_core::engine::digital_strength_from_tag(&strength_tag).ok_or_else(|| {
                crate::errors::value_error(format!(
                    "unsupported compressed-transient digital strength '{strength_tag}'"
                ))
            })?;
        rebuilt.push(rspice_core::engine::DigitalTracePoint {
            time,
            value: rspice_core::xspice::DigitalValue { state, strength },
        });
    }
    Ok(rspice_core::engine::DigitalTrace {
        node_name,
        points: rebuilt,
    })
}

fn identity_persistence_state(
    identity: &rspice_core::engine::TransientResultIdentity,
) -> CompressedIdentityPersistenceState {
    (
        identity
            .analysis
            .as_ref()
            .map(|analysis| (analysis.kind_tag.clone(), analysis.ordinal)),
        identity.coordinate.as_ref().map(|coordinate| {
            (
                coordinate.semantic.to_vec(),
                coordinate.occurrence,
                coordinate.ordinal,
                coordinate.label.clone(),
            )
        }),
        identity
            .topology_fingerprint
            .map(|fingerprint| fingerprint.to_vec()),
    )
}

fn rebuild_identity(
    state: CompressedIdentityPersistenceState,
) -> PyResult<rspice_core::engine::TransientResultIdentity> {
    let (analysis, coordinate, topology) = state;
    let analysis = analysis
        .map(|(kind_tag, ordinal)| {
            rspice_core::engine::TransientAnalysisIdentity::new(kind_tag, ordinal)
                .map_err(crate::errors::value_error)
        })
        .transpose()?;
    let coordinate = coordinate
        .map(|(semantic, occurrence, ordinal, label)| {
            let semantic: [u8; 16] = semantic.try_into().map_err(|_| {
                crate::errors::value_error(
                    "compressed-transient coordinate identity requires a 16-byte semantic digest",
                )
            })?;
            rspice_core::engine::TransientCoordinateIdentity::new(
                semantic, occurrence, ordinal, label,
            )
            .map_err(crate::errors::value_error)
        })
        .transpose()?;
    let topology_fingerprint = topology
        .map(|bytes| {
            let bytes: [u8; 32] = bytes.try_into().map_err(|_| {
                crate::errors::value_error(
                    "compressed-transient topology fingerprint requires 32 bytes",
                )
            })?;
            Ok::<_, PyErr>(bytes)
        })
        .transpose()?;
    Ok(rspice_core::engine::TransientResultIdentity {
        analysis,
        coordinate,
        topology_fingerprint,
    })
}

fn fourier_persistence_state(
    result: &rspice_core::engine::TransientFourierResult,
) -> CompressedFourierPersistenceState {
    (
        result.card_index,
        result.output.clone(),
        result.physical_type.to_string(),
        result.fundamental,
        result.harmonic_count,
        result.spectrum.fundamental_freq,
        result.spectrum.dc_component,
        result.spectrum.thd,
        result
            .spectrum
            .harmonics
            .iter()
            .map(|harmonic| {
                (
                    harmonic.harmonic_number,
                    harmonic.frequency,
                    harmonic.magnitude,
                    harmonic.phase,
                )
            })
            .collect(),
    )
}

fn rebuild_fourier(
    state: CompressedFourierPersistenceState,
) -> PyResult<rspice_core::engine::TransientFourierResult> {
    let (
        card_index,
        output,
        physical_type,
        fundamental,
        harmonic_count,
        fundamental_freq,
        dc_component,
        thd,
        harmonics,
    ) = state;
    let physical_type = match physical_type.as_str() {
        "voltage" => "voltage",
        "current" => "current",
        "parameter" => "parameter",
        _ => {
            return Err(crate::errors::value_error(format!(
                "unsupported compressed-transient Fourier physical type '{physical_type}'"
            )));
        }
    };
    Ok(rspice_core::engine::TransientFourierResult {
        card_index,
        output,
        physical_type,
        fundamental,
        harmonic_count,
        spectrum: rspice_core::analysis::FourierResult {
            fundamental_freq,
            dc_component,
            harmonics: harmonics
                .into_iter()
                .map(|(harmonic_number, frequency, magnitude, phase)| {
                    rspice_core::analysis::HarmonicComponent {
                        harmonic_number,
                        frequency,
                        magnitude,
                        phase,
                    }
                })
                .collect(),
            thd,
        },
    })
}

fn measurement_persistence_state(
    result: &rspice_core::MeasureResult,
) -> CompressedMeasurementPersistenceState {
    (
        result.name.clone(),
        result.value,
        result.raw_value,
        result.error.clone(),
        result.passed,
        result.expected,
        result.tolerance,
        result.failure_limit,
        result.failure_limit_exceeded,
        result.event_axis,
    )
}

fn rebuild_measurement(state: CompressedMeasurementPersistenceState) -> rspice_core::MeasureResult {
    let (
        name,
        value,
        raw_value,
        error,
        passed,
        expected,
        tolerance,
        failure_limit,
        failure_limit_exceeded,
        event_axis,
    ) = state;
    rspice_core::MeasureResult {
        name,
        value,
        raw_value,
        error,
        passed,
        expected,
        tolerance,
        failure_limit,
        failure_limit_exceeded,
        event_axis,
    }
}

fn compression_report_persistence_state(
    report: &rspice_core::engine::TransientCompressionReport,
) -> CompressionReportPersistenceState {
    let worst = report.worst_observed.as_ref().map(|observation| {
        (
            observation.signal.kind.as_str().to_string(),
            observation.signal.canonical_name.clone(),
            observation.input_sample_index,
            observation.time,
            observation.actual_value,
            observation.absolute_error,
            observation.relative_error,
            observation.allowed_tolerance,
            observation.tolerance_utilization,
        )
    });
    (
        report.schema_version,
        report.algorithm.as_str().to_string(),
        report.sample_domain.as_str().to_string(),
        report.applied_policy.enabled,
        report.applied_policy.absolute_tolerance,
        report.applied_policy.relative_tolerance,
        report.applied_policy.maximum_retained_interval,
        report.input_points,
        report.retained_points,
        worst,
    )
}

fn rebuild_compression_report(
    state: CompressionReportPersistenceState,
) -> PyResult<rspice_core::engine::TransientCompressionReport> {
    let (
        schema_version,
        algorithm,
        sample_domain,
        enabled,
        absolute_tolerance,
        relative_tolerance,
        maximum_retained_interval,
        input_points,
        retained_points,
        worst,
    ) = state;
    if schema_version != rspice_core::engine::TRANSIENT_COMPRESSION_REPORT_VERSION {
        return Err(crate::errors::value_error(format!(
            "unsupported compressed-transient compression-report version {schema_version}"
        )));
    }
    let algorithm = match algorithm.as_str() {
        "multi-channel-rdp-linear-v1" => {
            rspice_core::engine::TransientCompressionAlgorithm::MultiChannelRdpLinearV1
        }
        _ => {
            return Err(crate::errors::value_error(format!(
                "unsupported compressed-transient compression algorithm '{algorithm}'"
            )));
        }
    };
    let sample_domain = match sample_domain.as_str() {
        "accepted-input-samples" => {
            rspice_core::engine::TransientCompressionSampleDomain::AcceptedInputSamples
        }
        _ => {
            return Err(crate::errors::value_error(format!(
                "unsupported compressed-transient compression sample domain '{sample_domain}'"
            )));
        }
    };
    let worst_observed = worst
        .map(
            |(
                signal_kind,
                canonical_name,
                input_sample_index,
                time,
                actual_value,
                absolute_error,
                relative_error,
                allowed_tolerance,
                tolerance_utilization,
            )| {
                let Some(kind) =
                    rspice_core::engine::TransientCompressionSignalKind::from_tag(&signal_kind)
                else {
                    return Err(crate::errors::value_error(format!(
                        "unsupported compressed-transient compression signal kind '{signal_kind}'"
                    )));
                };
                Ok(rspice_core::engine::TransientCompressionErrorObservation {
                    signal: rspice_core::engine::TransientCompressionSignal::new(
                        kind,
                        canonical_name,
                    )
                    .map_err(crate::errors::value_error)?,
                    input_sample_index,
                    time,
                    actual_value,
                    absolute_error,
                    relative_error,
                    allowed_tolerance,
                    tolerance_utilization,
                })
            },
        )
        .transpose()?;
    Ok(rspice_core::engine::TransientCompressionReport {
        schema_version,
        algorithm,
        sample_domain,
        applied_policy: rspice_core::engine::TransientCompressionPolicy {
            enabled,
            absolute_tolerance,
            relative_tolerance,
            maximum_retained_interval,
        },
        input_points,
        retained_points,
        worst_observed,
    })
}

impl PyCompressedTransientResult {
    pub fn new(inner: rspice_core::engine::TransientResultCompressed) -> Self {
        Self { inner }
    }

    fn node_index(&self, node: &NodeIdentifier) -> PyResult<Option<usize>> {
        let num_nodes = self.inner.num_nodes();
        match node {
            NodeIdentifier::Index(0) => Ok(None),
            NodeIdentifier::Index(index) if *index <= num_nodes => Ok(Some(index - 1)),
            NodeIdentifier::Index(index) => Err(invalid_node_index_error(*index, num_nodes).into()),
            NodeIdentifier::Name(name) if is_ground_name(name) => Ok(None),
            NodeIdentifier::Name(name) => self
                .inner
                .node_names()
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
                .map(Some)
                .ok_or_else(|| unknown_node_name_error(name).into()),
        }
    }

    /// Dense retained samples of one channel, refusing to invent a number for
    /// a sample the producing run recorded as absent.
    fn dense_channel_values(
        &self,
        channel: &rspice_core::engine::TransientCompressedChannel,
        label: &str,
    ) -> PyResult<Vec<f64>> {
        if channel.availability != rspice_core::engine::TransientChannelAvailability::Available {
            return Err(crate::errors::key_error(format!(
                "{label} was not recorded; add it to .SAVE"
            )));
        }
        channel.dense_values().ok_or_else(|| {
            crate::errors::value_error(format!(
                "{label} has samples the producing run did not record as numbers; read `channel_absence` for the reason at each retained point"
            ))
        })
    }

    fn channel(
        &self,
        canonical_name: &str,
    ) -> PyResult<&rspice_core::engine::TransientCompressedChannel> {
        self.inner.channel_named(canonical_name).ok_or_else(|| {
            crate::errors::key_error(format!(
                "unknown compressed transient channel '{canonical_name}'"
            ))
        })
    }

    fn branch_current_values(&self, name: &str) -> PyResult<Vec<f64>> {
        let channel = self
            .inner
            .branch_current_channel(name)
            .ok_or_else(|| PyErr::from(unknown_branch_name_error(name)))?;
        self.dense_channel_values(channel, &format!("branch-current waveform '{name}'"))
    }
}

#[pymethods]
impl PyCompressedTransientResult {
    #[getter]
    fn time<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.time.to_pyarray(py)
    }

    /// Exact accepted integration intervals at the retained points.
    #[getter]
    fn step_sizes<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.step_sizes.to_pyarray(py)
    }

    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.inner.node_names()
    }

    #[getter]
    fn num_nodes(&self) -> usize {
        self.inner.num_nodes()
    }

    #[getter]
    fn num_points(&self) -> usize {
        self.inner.time.len()
    }

    #[getter]
    fn input_points(&self) -> usize {
        self.inner.input_points
    }

    #[getter]
    fn compression_ratio(&self) -> f64 {
        self.inner.compression_ratio
    }

    /// Version of the persisted compression evidence contract.
    #[getter]
    fn compression_report_version(&self) -> u32 {
        self.inner.compression_report.schema_version
    }

    /// Stable identifier of the compression algorithm.
    #[getter]
    fn compression_algorithm(&self) -> &'static str {
        self.inner.compression_report.algorithm.as_str()
    }

    /// Stable identifier of the input-sample domain used for verification.
    #[getter]
    fn compression_sample_domain(&self) -> &'static str {
        self.inner.compression_report.sample_domain.as_str()
    }

    /// Whether waveform decimation was enabled for this result.
    #[getter]
    fn compression_enabled(&self) -> bool {
        self.inner.compression_report.applied_policy.enabled
    }

    /// Applied absolute interpolation tolerance in each signal's native unit.
    #[getter]
    fn compression_absolute_tolerance(&self) -> f64 {
        self.inner
            .compression_report
            .applied_policy
            .absolute_tolerance
    }

    /// Applied relative interpolation tolerance.
    #[getter]
    fn compression_relative_tolerance(&self) -> f64 {
        self.inner
            .compression_report
            .applied_policy
            .relative_tolerance
    }

    /// Applied maximum interval between retained points in seconds.
    #[getter]
    fn compression_maximum_interval(&self) -> f64 {
        self.inner
            .compression_report
            .applied_policy
            .maximum_retained_interval
    }

    /// Stable kind of the signal with the highest tolerance utilization.
    #[getter]
    fn worst_compression_error_signal_kind(&self) -> Option<&'static str> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.signal.kind.as_str())
    }

    /// Canonical identity of the signal with the highest tolerance utilization.
    #[getter]
    fn worst_compression_error_signal(&self) -> Option<String> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.signal.canonical_name.clone())
    }

    /// Original accepted-grid index of the worst reconstructed sample.
    #[getter]
    fn worst_compression_error_input_sample_index(&self) -> Option<usize> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.input_sample_index)
    }

    /// Source-sample time of the worst final-grid reconstruction error.
    #[getter]
    fn worst_compression_error_time(&self) -> Option<f64> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.time)
    }

    /// Original value at the worst-utilization observation.
    #[getter]
    fn worst_compression_actual_value(&self) -> Option<f64> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.actual_value)
    }

    /// Absolute reconstruction error at the worst-utilization observation.
    #[getter]
    fn worst_compression_absolute_error(&self) -> Option<f64> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.absolute_error)
    }

    /// Relative error at the worst observation; absent when the actual value
    /// is zero or the ratio cannot be represented finitely.
    #[getter]
    fn worst_compression_relative_error(&self) -> Option<f64> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .and_then(|observation| observation.relative_error)
    }

    /// Allowed absolute-plus-relative tolerance at the worst observation.
    #[getter]
    fn worst_compression_allowed_tolerance(&self) -> Option<f64> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.allowed_tolerance)
    }

    /// Fraction of the declared tolerance consumed at the worst observation.
    #[getter]
    fn worst_compression_tolerance_utilization(&self) -> Option<f64> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.tolerance_utilization)
    }

    /// Canonical branch names aligned with retained branch-current waveforms.
    #[getter]
    fn branch_names(&self) -> Vec<String> {
        self.inner.branch_names()
    }

    /// Canonical names of every descriptor-keyed channel in this container.
    #[getter]
    fn channel_names(&self) -> Vec<String> {
        self.inner
            .channels
            .iter()
            .map(|channel| channel.descriptor.canonical_name().to_string())
            .collect()
    }

    /// Physical unit of one channel, by canonical name.
    fn channel_unit(&self, name: &str) -> PyResult<String> {
        self.channel(name)
            .map(|channel| channel.descriptor.unit().as_str().to_string())
    }

    /// Whether a channel was retained, and if not, why.
    fn channel_availability(&self, name: &str) -> PyResult<&'static str> {
        self.channel(name)
            .map(|channel| channel.availability.as_str())
    }

    /// Per-retained-point absence reasons for one channel. An entry is `None`
    /// where the channel has a number and a reason string where it does not,
    /// so a caller never has to read a placeholder as data.
    fn channel_absence(&self, name: &str) -> PyResult<Vec<Option<&'static str>>> {
        self.channel(name).map(|channel| {
            channel
                .samples
                .iter()
                .map(|sample| sample.absence().map(|reason| reason.as_str()))
                .collect()
        })
    }

    /// XSPICE digital event node names carried exactly through compression.
    #[getter]
    fn digital_trace_names(&self) -> Vec<String> {
        self.inner
            .digital_traces
            .iter()
            .map(|trace| trace.node_name.clone())
            .collect()
    }

    /// Committed `(time, state, strength)` events for one digital node.
    fn digital_trace(&self, name: &str) -> PyResult<Vec<(f64, &'static str, &'static str)>> {
        self.inner
            .digital_traces
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case(name))
            .map(|trace| {
                trace
                    .points
                    .iter()
                    .map(|point| {
                        (
                            point.time,
                            rspice_core::engine::digital_state_tag(point.value.state),
                            rspice_core::engine::digital_strength_tag(point.value.strength),
                        )
                    })
                    .collect()
            })
            .ok_or_else(|| {
                crate::errors::key_error(format!("unknown XSPICE digital event node '{name}'"))
            })
    }

    /// XSPICE real event node names carried exactly through compression.
    #[getter]
    fn real_trace_names(&self) -> Vec<String> {
        self.inner
            .real_traces
            .iter()
            .map(|trace| trace.node_name.clone())
            .collect()
    }

    /// Committed `(time, value)` events for one real event node.
    fn real_trace(&self, name: &str) -> PyResult<Vec<(f64, f64)>> {
        self.inner
            .real_traces
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case(name))
            .map(|trace| {
                trace
                    .points
                    .iter()
                    .map(|point| (point.time, point.value))
                    .collect()
            })
            .ok_or_else(|| {
                crate::errors::key_error(format!("unknown XSPICE real event node '{name}'"))
            })
    }

    /// `.FOUR` spectra computed on the exact accepted trajectory.
    #[getter]
    fn fourier_results(&self) -> Vec<PyFourierResult> {
        self.inner
            .post_results
            .fourier
            .iter()
            .map(|entry| {
                PyFourierResult::from_core_with_provenance(
                    &entry.spectrum,
                    entry.output.clone(),
                    format!("four-{:03}", entry.card_index + 1),
                    None,
                    None,
                )
            })
            .collect()
    }

    /// Transient `.MEASURE` results computed on the exact accepted trajectory.
    #[getter]
    fn measurements(&self) -> Vec<PyMeasurement> {
        self.inner
            .post_results
            .measurements
            .iter()
            .map(|result| PyMeasurement::from_core(result, "TRAN"))
            .collect()
    }

    /// Stable tag of the authored analysis card this result came from.
    #[getter]
    fn analysis_id(&self) -> Option<String> {
        self.inner
            .identity
            .analysis
            .as_ref()
            .map(|analysis| analysis.tag())
    }

    /// Label of the shared-deck coordinate this result was produced at.
    #[getter]
    fn coordinate_label(&self) -> Option<String> {
        self.inner
            .identity
            .coordinate
            .as_ref()
            .map(|coordinate| coordinate.label.clone())
    }

    /// Lower-case hexadecimal topology fingerprint of the solved circuit.
    #[getter]
    fn topology_fingerprint(&self) -> Option<String> {
        self.inner.identity.topology_fingerprint.map(|bytes| {
            bytes
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        })
    }

    fn branch_current_waveform<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self.branch_current_values(name)?.to_pyarray(py))
    }

    fn branch_current_at(&self, name: &str, time: f64) -> PyResult<f64> {
        if !time.is_finite() {
            return Err(crate::errors::value_error("time must be finite"));
        }
        self.branch_current_values(name)?;
        self.inner
            .interpolate_branch_current_named(name, time)
            .ok_or_else(|| {
                crate::errors::value_error(format!(
                    "compressed branch-current waveform '{name}' cannot be interpolated"
                ))
            })
    }

    /// Device operating-point traces requested with `.SAVE @device[param]`.
    #[getter]
    fn device_parameter_names(&self) -> Vec<String> {
        self.inner
            .channels
            .iter()
            .filter_map(|channel| match channel.descriptor.role() {
                rspice_core::engine::TransientChannelRole::DeviceObservable {
                    device,
                    parameter,
                } => Some(format!("@{device}[{parameter}]")),
                _ => None,
            })
            .collect()
    }

    fn device_parameter_waveform<'py>(
        &self,
        py: Python<'py>,
        device: &str,
        parameter: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let channel = self.inner.device_op_channel(device, parameter).ok_or_else(|| {
            crate::errors::key_error(format!(
                "device operating-point trace '@{device}[{parameter}]' was not recorded; add it to .SAVE"
            ))
        })?;
        Ok(self
            .dense_channel_values(
                channel,
                &format!("device operating-point trace '@{device}[{parameter}]'"),
            )?
            .to_pyarray(py))
    }

    fn device_parameter_at(&self, device: &str, parameter: &str, time: f64) -> PyResult<f64> {
        if !time.is_finite() {
            return Err(crate::errors::value_error("time must be finite"));
        }
        self.inner
            .interpolate_device_op_named(device, parameter, time)
            .ok_or_else(|| {
                crate::errors::key_error(format!(
                    "device operating-point trace '@{device}[{parameter}]' was not recorded; add it to .SAVE"
                ))
            })
    }

    /// Canonical typed device-store trace names.
    #[getter]
    fn store_names(&self) -> Vec<String> {
        self.inner
            .channels
            .iter()
            .filter_map(|channel| match channel.descriptor.role() {
                rspice_core::engine::TransientChannelRole::DeviceStore { store } => {
                    Some(store.clone())
                }
                _ => None,
            })
            .collect()
    }

    fn store_waveform<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let channel = self.inner.store_channel(name).ok_or_else(|| {
            crate::errors::key_error(format!("unknown device-store trace '{name}'"))
        })?;
        Ok(self
            .dense_channel_values(channel, &format!("device-store trace '{name}'"))?
            .to_pyarray(py))
    }

    fn store_at(&self, name: &str, time: f64) -> PyResult<f64> {
        if !time.is_finite() {
            return Err(crate::errors::value_error("time must be finite"));
        }
        self.inner
            .interpolate_store_named(name, time)
            .ok_or_else(|| crate::errors::key_error(format!("unknown device-store trace '{name}'")))
    }

    /// Typed `.FFT` products computed before waveform decimation.
    #[getter]
    fn fft_results(&self) -> Vec<PyTransientFftResult> {
        self.inner
            .post_results
            .fft
            .iter()
            .map(PyTransientFftResult::from)
            .collect()
    }

    fn fft(&self, index: usize) -> PyResult<PyTransientFftResult> {
        self.inner
            .post_results
            .fft
            .get(index)
            .map(PyTransientFftResult::from)
            .ok_or_else(|| {
                crate::errors::index_error(format!(
                    "FFT result index {index} out of range (0..{})",
                    self.inner.post_results.fft.len()
                ))
            })
    }

    fn voltage_waveform<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = match self.node_index(&node)? {
            Some(index) => {
                let channel = self.inner.node_voltage_channel(index).ok_or_else(|| {
                    crate::errors::value_error("malformed compressed transient voltage inventory")
                })?;
                self.dense_channel_values(channel, "requested node voltage")?
            }
            None => vec![0.0; self.inner.time.len()],
        };
        if values.len() != self.inner.time.len() {
            return Err(crate::errors::value_error(
                "malformed compressed transient waveform length",
            ));
        }
        Ok(values.to_pyarray(py))
    }

    fn voltage_at(&self, node: NodeIdentifier, time: f64) -> PyResult<f64> {
        if !time.is_finite() {
            return Err(crate::errors::value_error("time must be finite"));
        }
        match self.node_index(&node)? {
            Some(index) => {
                let channel = self.inner.node_voltage_channel(index).ok_or_else(|| {
                    crate::errors::value_error("malformed compressed transient voltage inventory")
                })?;
                if channel.availability
                    != rspice_core::engine::TransientChannelAvailability::Available
                {
                    return Err(crate::errors::key_error(
                        "requested node voltage was not recorded; add it to .SAVE",
                    ));
                }
                self.inner.interpolate(index, time).ok_or_else(|| {
                    crate::errors::value_error(
                        "compressed transient waveform cannot be interpolated",
                    )
                })
            }
            None => Ok(0.0),
        }
    }

    #[allow(clippy::type_complexity)]
    fn resample<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
        num_points: usize,
    ) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
        if num_points < 2 {
            return Err(crate::errors::value_error("num_points must be at least 2"));
        }
        match self.node_index(&node)? {
            Some(index) => self
                .inner
                .node_voltage_channel(index)
                .filter(|channel| {
                    channel.availability
                        == rspice_core::engine::TransientChannelAvailability::Available
                        || self.inner.time.is_empty()
                })
                .ok_or_else(|| {
                    crate::errors::key_error(
                        "requested node voltage was not recorded; add it to .SAVE",
                    )
                })
                .and_then(|_| {
                    self.inner.resample(index, num_points).ok_or_else(|| {
                        crate::errors::value_error("compressed waveform cannot be resampled")
                    })
                })
                .map(|(time, values)| (time.to_pyarray(py), values.to_pyarray(py))),
            None => {
                let Some((&start, &stop)) = self.inner.time.first().zip(self.inner.time.last())
                else {
                    return Err(crate::errors::value_error(
                        "empty compressed transient has no time domain to resample",
                    ));
                };
                let step = (stop - start) / (num_points - 1) as f64;
                let time = (0..num_points)
                    .map(|index| start + index as f64 * step)
                    .collect::<Vec<_>>();
                Ok((time.to_pyarray(py), vec![0.0; num_points].to_pyarray(py)))
            }
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "CompressedTransientResult(nodes={}, stored_points={}, input_points={}, ratio={:.2}x)",
            self.inner.num_nodes(),
            self.inner.time.len(),
            self.inner.input_points,
            self.inner.compression_ratio
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    ///
    #[staticmethod]
    #[pyo3(signature = (time, compression_ratio, input_points, fft_state=None, analog_state=None, compression_state=None))]
    fn _unpickle(
        time: Vec<f64>,
        compression_ratio: f64,
        input_points: usize,
        fft_state: Option<TransientFftPersistenceState>,
        analog_state: Option<CompressedTransientAnalogState>,
        compression_state: Option<CompressionReportPersistenceState>,
    ) -> PyResult<Self> {
        let Some((
            version,
            step_sizes,
            channels,
            digital_traces,
            real_traces,
            identity,
            fourier,
            measurements,
        )) = analog_state
        else {
            return Err(crate::errors::value_error(
                "legacy compressed-transient pickle predates lossless analog inventory persistence; rerun the analysis",
            ));
        };
        if version < COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION {
            return Err(crate::errors::value_error(format!(
                "compressed-transient analog pickle state version {version} predates the descriptor-indexed channel container with per-sample validity, event traces, parent identity, and post-results; rerun the analysis"
            )));
        }
        if version != COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION {
            return Err(crate::errors::value_error(format!(
                "unsupported compressed-transient analog pickle state version {version}"
            )));
        }
        let Some(compression_report) = compression_state else {
            return Err(crate::errors::value_error(
                "compressed-transient pickle is missing its required compression error certificate; rerun the analysis",
            ));
        };
        let inner = rspice_core::engine::TransientResultCompressed {
            time,
            step_sizes,
            channels: channels
                .into_iter()
                .map(rebuild_channel)
                .collect::<PyResult<Vec<_>>>()?,
            digital_traces: digital_traces
                .into_iter()
                .map(rebuild_digital_trace)
                .collect::<PyResult<Vec<_>>>()?,
            real_traces: real_traces
                .into_iter()
                .map(|(node_name, points)| rspice_core::engine::RealTrace {
                    node_name,
                    points: points
                        .into_iter()
                        .map(|(time, value)| rspice_core::engine::RealTracePoint { time, value })
                        .collect(),
                })
                .collect(),
            post_results: rspice_core::engine::TransientPostResults {
                fft: rebuild_transient_fft_results(fft_state)?,
                fourier: fourier
                    .into_iter()
                    .map(rebuild_fourier)
                    .collect::<PyResult<Vec<_>>>()?,
                measurements: measurements.into_iter().map(rebuild_measurement).collect(),
            },
            identity: rebuild_identity(identity)?,
            compression_ratio,
            input_points,
            compression_report: rebuild_compression_report(compression_report)?,
        };
        inner.validate().map_err(crate::errors::value_error)?;
        Ok(Self::new(inner))
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            Vec<f64>,
            f64,
            usize,
            TransientFftPersistenceState,
            CompressedTransientAnalogState,
            CompressionReportPersistenceState,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.inner.time.clone(),
                self.inner.compression_ratio,
                self.inner.input_points,
                transient_fft_persistence_state(&self.inner.post_results.fft)?,
                (
                    COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION,
                    self.inner.step_sizes.clone(),
                    self.inner
                        .channels
                        .iter()
                        .map(channel_persistence_state)
                        .collect(),
                    self.inner
                        .digital_traces
                        .iter()
                        .map(digital_trace_persistence_state)
                        .collect(),
                    self.inner
                        .real_traces
                        .iter()
                        .map(|trace| {
                            (
                                trace.node_name.clone(),
                                trace
                                    .points
                                    .iter()
                                    .map(|point| (point.time, point.value))
                                    .collect(),
                            )
                        })
                        .collect(),
                    identity_persistence_state(&self.inner.identity),
                    self.inner
                        .post_results
                        .fourier
                        .iter()
                        .map(fourier_persistence_state)
                        .collect(),
                    self.inner
                        .post_results
                        .measurements
                        .iter()
                        .map(measurement_persistence_state)
                        .collect(),
                ),
                compression_report_persistence_state(&self.inner.compression_report),
            ),
        ))
    }
}

/// Versioned transient integrator state for fingerprint-validated continuation.
#[pyclass(name = "TransientCheckpoint", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyTransientCheckpoint {
    pub(crate) inner: rspice_core::engine::TransientCheckpoint,
}

impl PyTransientCheckpoint {
    pub fn new(inner: rspice_core::engine::TransientCheckpoint) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl PyTransientCheckpoint {
    #[staticmethod]
    fn load(path: PathBuf) -> PyResult<Self> {
        rspice_core::engine::TransientCheckpoint::load(&path)
            .map(Self::new)
            .map_err(crate::errors::value_error)
    }

    fn save(&self, path: PathBuf) -> PyResult<()> {
        self.inner.save(&path).map_err(crate::errors::value_error)
    }

    #[getter]
    fn time(&self) -> f64 {
        self.inner.time
    }

    #[getter]
    fn solution<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.solution.to_pyarray(py)
    }

    #[getter]
    fn netlist_fingerprint(&self) -> u64 {
        self.inner.netlist_fingerprint
    }

    fn __repr__(&self) -> String {
        format!(
            "TransientCheckpoint(time={:.6e}, state_size={}, fingerprint={:#018x})",
            self.inner.time,
            self.inner.solution.len(),
            self.inner.netlist_fingerprint
        )
    }
}
