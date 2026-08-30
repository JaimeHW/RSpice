//! Encoding the worker protocol on the wire, and refusing anything unsound.
//!
//! Every numeric array crosses the worker boundary as a transferable buffer
//! rather than inside the JSON metadata, so the metadata carries only lengths
//! and indices.  That split is why validation lives here: a payload is checked
//! against the ingress limits *before* any copy, buffer indices and lengths are
//! reconciled with the metadata that claims them, and a response that fails
//! either check is rejected rather than partially reconstructed.

// These result enums have one variant per analysis kind, and the analyses
// genuinely differ in payload size — a transient result carries waveform
// series where a DC operating point carries scalars. The value is built once
// per run, serialized, and dropped; it is never stored in a collection, so
// the size the lint measures is a transient stack cost on a path that is
// already doing IO. Boxing each variant's fields would mean a payload struct
// per analysis and, for the struct variants, a change to the JSON shape the
// worker protocol is pinned to.
#![allow(clippy::large_enum_variant)]

use super::*;

impl WorkerResponseTransport {
    pub(super) fn from_response(response: WorkerResponse) -> Result<Self, String> {
        validate_worker_response_before_transport(&response)?;
        let mut buffers = Vec::new();
        let response = WorkerResponseTransportMetadata {
            id: response.id,
            outcome: WorkerOutcomeTransport::from_outcome(response.outcome, &mut buffers),
        };
        validate_worker_transfer_buffers(&buffers)?;
        Ok(Self {
            protocol: WORKER_RESPONSE_TRANSPORT_PROTOCOL,
            response,
            buffers,
        })
    }

    pub(super) fn into_response(self) -> Result<WorkerResponse, String> {
        if self.protocol != WORKER_RESPONSE_TRANSPORT_PROTOCOL {
            return Err(format!(
                "unsupported worker response transport protocol {}",
                self.protocol
            ));
        }
        validate_worker_transfer_buffers(&self.buffers)?;

        Ok(WorkerResponse {
            id: self.response.id,
            outcome: self.response.outcome.into_outcome(&self.buffers)?,
        })
    }
}

pub(super) fn validate_worker_response_before_transport(
    response: &WorkerResponse,
) -> Result<(), String> {
    if let WorkerOutcome::Success(result) = &response.outcome
        && let WorkerSimulationResult::DcOp {
            configuration,
            mna_node_names,
            mna_branch_names,
            mna_solution,
            ..
        } = result.as_ref()
    {
        if let Some(previous_state) = configuration.previous_state.as_ref()
            && previous_state.solution.len() > MAX_WORKER_F64_VALUES
        {
            return Err(format!(
                "worker DC operating-point response contains {} previous-state MNA values, exceeding the {MAX_WORKER_F64_VALUES}-value limit",
                previous_state.solution.len()
            ));
        }
        if mna_solution.len() > MAX_WORKER_F64_VALUES {
            return Err(format!(
                "worker DC operating-point response contains {} MNA values, exceeding the {MAX_WORKER_F64_VALUES}-value limit",
                mna_solution.len()
            ));
        }
        return validate_worker_dc_op_state(
            configuration,
            mna_node_names,
            mna_branch_names,
            mna_solution,
        );
    }
    let WorkerOutcome::Success(result) = &response.outcome else {
        return Ok(());
    };
    if let WorkerSimulationResult::Hb {
        frequencies,
        waveforms,
        operating_point,
        ..
    } = result.as_ref()
    {
        let waveform_buffer_count = waveforms.iter().try_fold(0usize, |count, waveform| {
            count
                .checked_add(2 + usize::from(waveform.y_imag.is_some()))
                .ok_or_else(|| {
                    "retained HB response buffer count overflows this platform".to_owned()
                })
        })?;
        let transfer_buffer_count = operating_point
            .spectral_state()
            .len()
            .checked_add(operating_point.mna_branch_spectral_state().len())
            .ok_or_else(|| "retained HB response buffer count overflows this platform".to_owned())?
            .checked_mul(2)
            .and_then(|count| count.checked_add(waveform_buffer_count))
            .and_then(|count| count.checked_add(1))
            .ok_or_else(|| {
                "retained HB response buffer count overflows this platform".to_owned()
            })?;
        if transfer_buffer_count > MAX_WORKER_TRANSFER_BUFFERS {
            return Err(format!(
                "retained HB response requires {transfer_buffer_count} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit"
            ));
        }
        let mut numeric_values = frequencies.len();
        for waveform in waveforms {
            numeric_values = numeric_values
                .checked_add(waveform.x_values.len())
                .and_then(|count| count.checked_add(waveform.y_values.len()))
                .and_then(|count| count.checked_add(waveform.y_imag.as_ref().map_or(0, Vec::len)))
                .ok_or_else(|| "retained HB response size overflows this platform".to_owned())?;
        }
        for spectrum in operating_point.spectral_state() {
            numeric_values = numeric_values
                .checked_add(spectrum.len().checked_mul(2).ok_or_else(|| {
                    "retained HB response size overflows this platform".to_owned()
                })?)
                .ok_or_else(|| "retained HB response size overflows this platform".to_owned())?;
        }
        for spectrum in operating_point.mna_branch_spectral_state() {
            numeric_values = numeric_values
                .checked_add(spectrum.len().checked_mul(2).ok_or_else(|| {
                    "retained HB response size overflows this platform".to_owned()
                })?)
                .ok_or_else(|| "retained HB response size overflows this platform".to_owned())?;
        }
        if numeric_values > MAX_WORKER_F64_VALUES {
            return Err(format!(
                "retained HB response contains {numeric_values} numerical values, exceeding the {MAX_WORKER_F64_VALUES}-value limit"
            ));
        }
        return Ok(());
    }
    if let WorkerSimulationResult::Pstb {
        modes,
        mode_indices,
        waveforms,
        ..
    } = result.as_ref()
    {
        validate_worker_pstb_result(result)?;
        let waveform_buffer_count = waveforms.iter().try_fold(0usize, |count, waveform| {
            count
                .checked_add(2 + usize::from(waveform.y_imag.is_some()))
                .ok_or_else(|| {
                    "PSTB worker response buffer count overflows this platform".to_owned()
                })
        })?;
        let transfer_buffer_count = waveform_buffer_count.checked_add(6).ok_or_else(|| {
            "PSTB worker response buffer count overflows this platform".to_owned()
        })?;
        if transfer_buffer_count > MAX_WORKER_TRANSFER_BUFFERS {
            return Err(format!(
                "PSTB worker response requires {transfer_buffer_count} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit"
            ));
        }
        let mut numeric_values = modes
            .len()
            .checked_mul(5)
            .and_then(|count| count.checked_add(mode_indices.len()))
            .ok_or_else(|| "PSTB worker response size overflows this platform".to_owned())?;
        for waveform in waveforms {
            numeric_values = numeric_values
                .checked_add(waveform.x_values.len())
                .and_then(|count| count.checked_add(waveform.y_values.len()))
                .and_then(|count| count.checked_add(waveform.y_imag.as_ref().map_or(0, Vec::len)))
                .ok_or_else(|| "PSTB worker response size overflows this platform".to_owned())?;
        }
        if numeric_values > MAX_WORKER_F64_VALUES {
            return Err(format!(
                "PSTB worker response contains {numeric_values} numerical values, exceeding the {MAX_WORKER_F64_VALUES}-value limit"
            ));
        }
        return Ok(());
    }
    let WorkerSimulationResult::Pss {
        operating_point, ..
    } = result.as_ref()
    else {
        return Ok(());
    };
    let analysis = operating_point.analysis();
    let validation = if let Some(identity) = operating_point.producer_identity() {
        rspice_core::engine::PssOperatingPoint::try_from_authenticated_parts(
            identity.clone(),
            operating_point.config().clone(),
            analysis.clone(),
            operating_point.shooting_state_basis().to_vec(),
            operating_point.shooting_state().to_vec(),
        )
    } else {
        rspice_core::engine::PssOperatingPoint::try_from_parts(
            operating_point.config().clone(),
            analysis.clone(),
            operating_point.shooting_state().to_vec(),
        )
    };
    validation.map_err(|error| format!("invalid retained PSS worker response: {error}"))?;
    let transfer_buffer_count = analysis
        .result
        .waveforms
        .len()
        .checked_add(analysis.monodromy.len())
        .and_then(|count| count.checked_add(6))
        .ok_or_else(|| "retained PSS response buffer count overflows this platform".to_owned())?;
    if transfer_buffer_count > MAX_WORKER_TRANSFER_BUFFERS {
        return Err(format!(
            "retained PSS response requires {transfer_buffer_count} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit"
        ));
    }
    let mut numeric_values = analysis.result.time.len();
    for waveform in &analysis.result.waveforms {
        numeric_values = numeric_values
            .checked_add(waveform.values.len())
            .ok_or_else(|| "retained PSS response size overflows this platform".to_owned())?;
    }
    for row in &analysis.monodromy {
        numeric_values = numeric_values
            .checked_add(row.len())
            .ok_or_else(|| "retained PSS response size overflows this platform".to_owned())?;
    }
    numeric_values = numeric_values
        .checked_add(
            analysis
                .result
                .floquet_multipliers
                .len()
                .checked_mul(2)
                .ok_or_else(|| "retained PSS response size overflows this platform".to_owned())?,
        )
        .and_then(|count| {
            analysis
                .floquet_multipliers
                .len()
                .checked_mul(2)
                .and_then(|values| count.checked_add(values))
        })
        .and_then(|count| count.checked_add(operating_point.shooting_state().len()))
        .ok_or_else(|| "retained PSS response size overflows this platform".to_owned())?;
    if numeric_values > MAX_WORKER_F64_VALUES {
        return Err(format!(
            "retained PSS response contains {numeric_values} unique numerical values, exceeding the {MAX_WORKER_F64_VALUES}-value limit"
        ));
    }
    Ok(())
}

pub(super) fn validate_worker_transfer_buffers(buffers: &[Vec<f64>]) -> Result<(), String> {
    if buffers.len() > MAX_WORKER_TRANSFER_BUFFERS {
        return Err(format!(
            "worker response contains {} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit",
            buffers.len()
        ));
    }
    let numeric_values = buffers.iter().try_fold(0usize, |total, values| {
        total
            .checked_add(values.len())
            .ok_or_else(|| "worker response numeric size overflows this platform".to_owned())
    })?;
    if numeric_values > MAX_WORKER_F64_VALUES {
        return Err(format!(
            "worker response contains {numeric_values} numerical values, exceeding the {MAX_WORKER_F64_VALUES}-value limit"
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerResponseTransportMetadata {
    pub id: u64,
    pub outcome: WorkerOutcomeTransport,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerOutcomeTransport {
    Success(WorkerSimulationResultTransport),
    Failure(WorkerSimulationError),
}

impl WorkerOutcomeTransport {
    pub(super) fn from_outcome(outcome: WorkerOutcome, buffers: &mut Vec<Vec<f64>>) -> Self {
        match outcome {
            WorkerOutcome::Success(result) => Self::Success(
                WorkerSimulationResultTransport::from_result(*result, buffers),
            ),
            WorkerOutcome::Failure(error) => Self::Failure(error),
        }
    }

    pub(super) fn into_outcome(self, buffers: &[Vec<f64>]) -> Result<WorkerOutcome, String> {
        match self {
            Self::Success(result) => Ok(WorkerOutcome::Success(Box::new(
                result.into_result(buffers)?,
            ))),
            Self::Failure(error) => Ok(WorkerOutcome::Failure(error)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerF64Series {
    Inline(Vec<f64>),
    Buffer { buffer: usize, len: usize },
}

pub(super) const MAX_WORKER_F64_VALUES: usize = 16_777_216;
pub(super) const MAX_WORKER_TRANSFER_BUFFERS: usize = 65_536;

pub(super) fn checked_worker_request_numeric_total(
    current_total: usize,
    buffer_index: usize,
    buffer_len: usize,
) -> Result<usize, String> {
    if buffer_len > MAX_WORKER_F64_VALUES {
        return Err(format!(
            "worker request transport buffer {buffer_index} contains {buffer_len} values, exceeding the {MAX_WORKER_F64_VALUES}-value limit"
        ));
    }
    let total = current_total
        .checked_add(buffer_len)
        .ok_or_else(|| "worker request numeric size overflows this platform".to_owned())?;
    if total > MAX_WORKER_F64_VALUES {
        return Err(format!(
            "worker request contains more than {MAX_WORKER_F64_VALUES} numerical values"
        ));
    }
    Ok(total)
}

#[cfg(any(target_arch = "wasm32", test))]
pub(super) fn validate_worker_request_transfer_buffers(buffers: &[Vec<f64>]) -> Result<(), String> {
    validate_worker_request_transfer_buffer_lengths(buffers.iter().map(Vec::len))
}

#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn validate_worker_request_transfer_buffer_lengths(
    lengths: impl IntoIterator<Item = usize>,
) -> Result<(), String> {
    let lengths = lengths.into_iter();
    let (lower_bound, upper_bound) = lengths.size_hint();
    let declared_count = upper_bound.unwrap_or(lower_bound);
    if declared_count > MAX_WORKER_TRANSFER_BUFFERS {
        return Err(format!(
            "worker request contains {declared_count} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit"
        ));
    }
    let mut total = 0usize;
    for (index, len) in lengths.enumerate() {
        if index >= MAX_WORKER_TRANSFER_BUFFERS {
            return Err(format!(
                "worker request contains more than {MAX_WORKER_TRANSFER_BUFFERS} transfer buffers"
            ));
        }
        total = checked_worker_request_numeric_total(total, index, len)?;
    }
    Ok(())
}

impl WorkerF64Series {
    pub(super) fn from_vec(values: Vec<f64>, buffers: &mut Vec<Vec<f64>>) -> Self {
        let len = values.len();
        let buffer = buffers.len();
        buffers.push(values);
        Self::Buffer { buffer, len }
    }

    pub(super) fn into_vec(self, buffers: &[Vec<f64>]) -> Result<Vec<f64>, String> {
        match self {
            Self::Inline(values) => {
                if values.len() > MAX_WORKER_F64_VALUES {
                    return Err(format!(
                        "inline worker series contains {} values, exceeding the {MAX_WORKER_F64_VALUES}-value limit",
                        values.len()
                    ));
                }
                Ok(values)
            }
            Self::Buffer { buffer, len } => {
                if len > MAX_WORKER_F64_VALUES {
                    return Err(format!(
                        "transferable buffer {buffer} declares {len} values, exceeding the {MAX_WORKER_F64_VALUES}-value limit"
                    ));
                }
                let values = buffers
                    .get(buffer)
                    .ok_or_else(|| format!("missing transferable buffer {buffer}"))?;
                if values.len() != len {
                    return Err(format!(
                        "transferable buffer {buffer} has length {}, expected {len}",
                        values.len()
                    ));
                }
                Ok(values.clone())
            }
        }
    }

    pub(super) fn len(&self) -> usize {
        match self {
            Self::Inline(values) => values.len(),
            Self::Buffer { len, .. } => *len,
        }
    }
}

impl WorkerOpPreviousStateTransport {
    pub(super) fn from_previous_state(
        previous_state: crate::simulation::dialog::OpPreviousState,
        buffers: &mut Vec<Vec<f64>>,
    ) -> Result<Self, String> {
        validate_worker_op_previous_state(
            &previous_state.node_names,
            &previous_state.branch_names,
            &previous_state.solution,
        )?;
        if previous_state.solution.len() > MAX_WORKER_F64_VALUES {
            return Err(format!(
                "OP previous-state solution contains {} values, exceeding the {MAX_WORKER_F64_VALUES}-value limit",
                previous_state.solution.len()
            ));
        }
        Ok(Self::from_validated_previous_state(previous_state, buffers))
    }

    pub(super) fn from_validated_previous_state(
        previous_state: crate::simulation::dialog::OpPreviousState,
        buffers: &mut Vec<Vec<f64>>,
    ) -> Self {
        let crate::simulation::dialog::OpPreviousState {
            source_content_digest,
            producer_snapshot_digest,
            producer_result_digest,
            node_names,
            branch_names,
            solution,
        } = previous_state;
        let solution_digest = crate::simulation::execution::f64_sequence_digest(
            "rspice.worker-op-previous-state/v1",
            &solution,
        );
        Self {
            source_content_digest,
            producer_snapshot_digest,
            producer_result_digest,
            node_names,
            branch_names,
            solution: WorkerF64Series::from_vec(solution, buffers),
            solution_digest,
        }
    }

    pub(super) fn into_previous_state(
        self,
        buffers: &[Vec<f64>],
    ) -> Result<crate::simulation::dialog::OpPreviousState, String> {
        if !matches!(self.solution, WorkerF64Series::Buffer { .. }) {
            return Err(
                "worker OP previous-state solution must use a transferable Float64 buffer"
                    .to_owned(),
            );
        }
        let solution = self.solution.into_vec(buffers)?;
        let actual_digest = crate::simulation::execution::f64_sequence_digest(
            "rspice.worker-op-previous-state/v1",
            &solution,
        );
        if actual_digest != self.solution_digest {
            return Err(format!(
                "worker OP previous-state solution digest is {actual_digest}, expected {}",
                self.solution_digest
            ));
        }
        validate_worker_op_previous_state(&self.node_names, &self.branch_names, &solution)?;
        Ok(crate::simulation::dialog::OpPreviousState {
            source_content_digest: self.source_content_digest,
            producer_snapshot_digest: self.producer_snapshot_digest,
            producer_result_digest: self.producer_result_digest,
            node_names: self.node_names,
            branch_names: self.branch_names,
            solution,
        })
    }
}

pub(super) fn validate_worker_op_previous_state(
    node_names: &[String],
    branch_names: &[String],
    solution: &[f64],
) -> Result<(), String> {
    rspice_core::engine::PssDcOperatingPointSeed::try_new(
        node_names.to_vec(),
        branch_names.to_vec(),
        solution.to_vec(),
    )
    .map(|_| ())
    .map_err(|error| format!("invalid worker OP previous-state payload: {error}"))
}

#[cfg(test)]
#[test]
pub(super) fn worker_transport_extracts_every_retained_pss_numeric_array_from_metadata() {
    let response = WorkerResponse {
        id: 78,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::Pss {
            measurements: Vec::new(),
            operating_point: tests::retained_pss_operating_point(),
        })),
    };
    let transport = WorkerResponseTransport::from_response(response.clone()).unwrap();
    let metadata = serde_json::to_string(&transport.response).unwrap();
    assert!(
        metadata.len() < 4_096,
        "PSS samples leaked into worker metadata"
    );
    assert!(!metadata.contains("\"Inline\""));
    assert_eq!(transport.buffers.len(), 8);

    let WorkerOutcomeTransport::Success(WorkerSimulationResultTransport::Pss {
        operating_point: periodic,
        ..
    }) = &transport.response.outcome
    else {
        panic!("expected retained PSS transport metadata")
    };
    assert!(matches!(
        periodic.result_time,
        WorkerF64Series::Buffer { .. }
    ));
    assert!(matches!(
        periodic.result_waveforms[0].values,
        WorkerF64Series::Buffer { .. }
    ));
    assert!(matches!(
        periodic.analysis_monodromy[0],
        WorkerF64Series::Buffer { .. }
    ));
    assert!(matches!(
        periodic.shooting_state,
        WorkerF64Series::Buffer { .. }
    ));

    assert_eq!(transport.into_response().unwrap(), response);
}

#[cfg(test)]
#[test]
pub(super) fn worker_transport_extracts_and_authenticates_dc_op_mna_solution() {
    let configuration = tests::nondefault_op_config();
    let response = WorkerResponse {
        id: 79,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::DcOp {
            configuration,
            validated_startup_directives: 0,
            mna_node_names: vec!["out".to_owned()],
            mna_branch_names: vec!["V1".to_owned()],
            mna_solution: vec![1.25, -0.001],
            node_voltages: HashMap::from([("out".to_owned(), 1.25)]),
            branch_currents: HashMap::from([("V1".to_owned(), -0.001)]),
            device_report: None,
        })),
    };
    let mut transport = WorkerResponseTransport::from_response(response.clone()).unwrap();
    let metadata = serde_json::to_string(&transport.response).unwrap();
    assert!(!metadata.contains("\"Inline\""));
    assert!(
        metadata.contains("\"mna_solution\":{\"Buffer\""),
        "MNA state must be represented only by a transferable buffer reference"
    );
    assert!(metadata.contains("\"previous_state\":{"));
    assert!(!metadata.contains("\"solution\":[1.25"));
    assert_eq!(
        transport.buffers,
        vec![vec![1.25, -0.001], vec![1.25, -0.001]]
    );
    assert_eq!(transport.clone().into_response().unwrap(), response);

    let mut tampered = transport.clone();
    tampered.buffers[1][0] = 1.5;
    assert!(
        tampered
            .into_response()
            .unwrap_err()
            .contains("payload digest")
    );

    let mut nonfinite = transport.clone();
    nonfinite.buffers[1][0] = f64::NAN;
    assert!(nonfinite.into_response().is_err());

    let mut previous_state_tamper = transport.clone();
    previous_state_tamper.buffers[0][0] = 1.5;
    assert!(
        previous_state_tamper
            .into_response()
            .unwrap_err()
            .contains("previous-state solution digest")
    );

    let WorkerOutcomeTransport::Success(WorkerSimulationResultTransport::DcOp {
        mna_solution, ..
    }) = &mut transport.response.outcome
    else {
        panic!("expected DC OP transport")
    };
    *mna_solution = WorkerF64Series::Buffer {
        buffer: 1,
        len: MAX_WORKER_F64_VALUES + 1,
    };
    assert!(transport.into_response().unwrap_err().contains("exceeding"));
}

#[cfg(test)]
#[test]
pub(super) fn worker_request_ingress_limits_are_checked_before_copy() {
    assert_eq!(checked_worker_request_numeric_total(10, 0, 20).unwrap(), 30);
    assert!(
        checked_worker_request_numeric_total(0, 0, MAX_WORKER_F64_VALUES + 1)
            .unwrap_err()
            .contains("buffer 0")
    );
    assert!(
        checked_worker_request_numeric_total(MAX_WORKER_F64_VALUES, 1, 1)
            .unwrap_err()
            .contains("more than")
    );
    assert!(
        validate_worker_request_transfer_buffer_lengths([MAX_WORKER_F64_VALUES, 1])
            .unwrap_err()
            .contains("more than")
    );
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerPeriodicWaveformTransport {
    node_name: String,
    values: WorkerF64Series,
}

/// Scalar metadata plus transferable numerical arrays for a retained PSS
/// operating point. No orbit, monodromy, Floquet, or shooting-state array is
/// serialized into the browser worker's JSON response metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerPssOperatingPointTransport {
    config: rspice_core::analysis::PssConfig,
    #[serde(default)]
    producer_identity: Option<rspice_core::engine::PssOperatingPointIdentity>,
    result_period: f64,
    result_frequency: f64,
    result_iterations: usize,
    result_residual_norm: f64,
    result_time: WorkerF64Series,
    result_waveforms: Vec<WorkerPeriodicWaveformTransport>,
    result_period_detected: bool,
    result_floquet_real: WorkerF64Series,
    result_floquet_imag: WorkerF64Series,
    #[serde(default)]
    result_floquet_evidence: rspice_core::analysis::FloquetSpectrumEvidence,
    #[serde(default)]
    result_floquet_orbit_kind: rspice_core::analysis::FloquetOrbitKind,
    #[serde(default)]
    result_trivial_floquet_multiplier_index: Option<usize>,
    analysis_iterations: usize,
    analysis_final_residual: f64,
    analysis_period: f64,
    analysis_monodromy: Vec<WorkerF64Series>,
    analysis_floquet_real: WorkerF64Series,
    analysis_floquet_imag: WorkerF64Series,
    analysis_is_stable: bool,
    #[serde(default = "indeterminate_floquet_verdict")]
    analysis_floquet_verdict: rspice_core::analysis::FloquetStabilityVerdict,
    #[serde(default)]
    analysis_floquet_authenticated: bool,
    #[serde(default)]
    shooting_state_basis: Vec<String>,
    shooting_state: WorkerF64Series,
}

fn indeterminate_floquet_verdict() -> rspice_core::analysis::FloquetStabilityVerdict {
    rspice_core::analysis::FloquetStabilityVerdict::Indeterminate
}

fn pss_floquet_contract_is_authenticated(
    result: &rspice_core::analysis::pss::PssResult,
    monodromy_order: usize,
) -> bool {
    if !result.has_consistent_floquet_contract() {
        return false;
    }
    match &result.floquet_evidence {
        rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate } => {
            certificate.is_valid()
                && monodromy_order > 0
                && certificate.problem_order == monodromy_order
                && certificate.problem_order == result.floquet_multipliers.len()
        }
        rspice_core::analysis::FloquetSpectrumEvidence::NoDynamicModes => {
            monodromy_order == 0
                && result.floquet_multipliers.is_empty()
                && result.floquet_orbit_kind == rspice_core::analysis::FloquetOrbitKind::Driven
        }
        _ => false,
    }
}

impl WorkerPssOperatingPointTransport {
    pub(super) fn from_operating_point(
        operating_point: rspice_core::engine::PssOperatingPoint,
        buffers: &mut Vec<Vec<f64>>,
    ) -> Self {
        let config = operating_point.config().clone();
        let analysis = operating_point.analysis();
        let result = &analysis.result;
        let analysis_floquet_verdict = result.stability_verdict();
        let analysis_floquet_authenticated =
            pss_floquet_contract_is_authenticated(result, analysis.monodromy.len());
        let result_waveforms = result
            .node_names
            .iter()
            .cloned()
            .zip(result.waveforms.iter())
            .map(|(node_name, waveform)| WorkerPeriodicWaveformTransport {
                node_name,
                values: WorkerF64Series::from_vec(waveform.values.clone(), buffers),
            })
            .collect();
        let (result_floquet_real, result_floquet_imag): (Vec<_>, Vec<_>) = result
            .floquet_multipliers
            .iter()
            .map(|value| (value.re, value.im))
            .unzip();
        let (analysis_floquet_real, analysis_floquet_imag): (Vec<_>, Vec<_>) = analysis
            .floquet_multipliers
            .iter()
            .map(|value| (value.re, value.im))
            .unzip();
        Self {
            config,
            producer_identity: operating_point.producer_identity().cloned(),
            result_period: result.period,
            result_frequency: result.frequency,
            result_iterations: result.iterations,
            result_residual_norm: result.residual_norm,
            result_time: WorkerF64Series::from_vec(result.time.clone(), buffers),
            result_waveforms,
            result_period_detected: result.period_detected,
            result_floquet_real: WorkerF64Series::from_vec(result_floquet_real, buffers),
            result_floquet_imag: WorkerF64Series::from_vec(result_floquet_imag, buffers),
            result_floquet_evidence: result.floquet_evidence.clone(),
            result_floquet_orbit_kind: result.floquet_orbit_kind,
            result_trivial_floquet_multiplier_index: result.trivial_floquet_multiplier_index,
            analysis_iterations: analysis.iterations,
            analysis_final_residual: analysis.final_residual,
            analysis_period: analysis.period,
            analysis_monodromy: analysis
                .monodromy
                .iter()
                .cloned()
                .map(|row| WorkerF64Series::from_vec(row, buffers))
                .collect(),
            analysis_floquet_real: WorkerF64Series::from_vec(analysis_floquet_real, buffers),
            analysis_floquet_imag: WorkerF64Series::from_vec(analysis_floquet_imag, buffers),
            analysis_is_stable: analysis.is_stable,
            analysis_floquet_verdict,
            analysis_floquet_authenticated,
            shooting_state_basis: operating_point.shooting_state_basis().to_vec(),
            shooting_state: WorkerF64Series::from_vec(
                operating_point.shooting_state().to_vec(),
                buffers,
            ),
        }
    }

    pub(super) fn into_operating_point(
        self,
        buffers: &[Vec<f64>],
    ) -> Result<rspice_core::engine::PssOperatingPoint, String> {
        if self.result_waveforms.len() > 65_536 || self.analysis_monodromy.len() > 65_536 {
            return Err("retained PSS worker metadata exceeds structural limits".to_owned());
        }
        let mut node_names = Vec::with_capacity(self.result_waveforms.len());
        let mut waveforms = Vec::with_capacity(self.result_waveforms.len());
        for waveform in self.result_waveforms {
            node_names.push(waveform.node_name);
            waveforms.push(rspice_core::analysis::pss::PeriodicWaveform::from_values(
                waveform.values.into_vec(buffers)?,
            ));
        }
        let result_floquet_multipliers = worker_join_complex(
            "PSS result Floquet",
            self.result_floquet_real.into_vec(buffers)?,
            self.result_floquet_imag.into_vec(buffers)?,
        )?;
        let analysis_floquet_multipliers = worker_join_complex(
            "PSS analysis Floquet",
            self.analysis_floquet_real.into_vec(buffers)?,
            self.analysis_floquet_imag.into_vec(buffers)?,
        )?;
        let monodromy = self
            .analysis_monodromy
            .into_iter()
            .map(|row| row.into_vec(buffers))
            .collect::<Result<Vec<_>, _>>()?;
        let shooting_state = self.shooting_state.into_vec(buffers)?;
        let result = rspice_core::analysis::pss::PssResult {
            period: self.result_period,
            frequency: self.result_frequency,
            iterations: self.result_iterations,
            residual_norm: self.result_residual_norm,
            time: self.result_time.into_vec(buffers)?,
            waveforms,
            node_names,
            period_detected: self.result_period_detected,
            floquet_multipliers: result_floquet_multipliers,
            floquet_evidence: self.result_floquet_evidence,
            floquet_orbit_kind: self.result_floquet_orbit_kind,
            trivial_floquet_multiplier_index: self.result_trivial_floquet_multiplier_index,
        };
        let computed_verdict = result.stability_verdict();
        let computed_authenticated =
            pss_floquet_contract_is_authenticated(&result, monodromy.len());
        if self.analysis_floquet_verdict != computed_verdict {
            return Err(
                "retained PSS worker compatibility verdict does not match its Floquet contract"
                    .to_owned(),
            );
        }
        if self.analysis_floquet_authenticated != computed_authenticated || !computed_authenticated
        {
            return Err(
                "retained PSS worker payload lacks authenticated Floquet evidence".to_owned(),
            );
        }
        let analysis = rspice_core::engine::PssAnalysisResult {
            result,
            iterations: self.analysis_iterations,
            final_residual: self.analysis_final_residual,
            period: self.analysis_period,
            monodromy,
            floquet_multipliers: analysis_floquet_multipliers,
            is_stable: self.analysis_is_stable,
        };
        let operating_point = if let Some(producer_identity) = self.producer_identity {
            rspice_core::engine::PssOperatingPoint::try_from_authenticated_parts(
                producer_identity,
                self.config,
                analysis,
                self.shooting_state_basis,
                shooting_state,
            )
        } else {
            rspice_core::engine::PssOperatingPoint::try_from_parts(
                self.config,
                analysis,
                shooting_state,
            )
        };
        operating_point.map_err(|error| format!("invalid retained PSS worker payload: {error}"))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerHbSpectrumTransport {
    node_name: String,
    real: WorkerF64Series,
    imaginary: WorkerF64Series,
    real_digest: crate::product::ContentDigest,
    imaginary_digest: crate::product::ContentDigest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerHbBranchSpectrumTransport {
    branch_name: String,
    real: WorkerF64Series,
    imaginary: WorkerF64Series,
    real_digest: crate::product::ContentDigest,
    imaginary_digest: crate::product::ContentDigest,
}

/// Scalar HB basis metadata plus transferable complex spectral rows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerHbOperatingPointTransport {
    config: rspice_core::analysis::HbConfig,
    #[serde(default)]
    producer_identity: Option<rspice_core::engine::HbOperatingPointIdentity>,
    spectra: Vec<WorkerHbSpectrumTransport>,
    mna_branch_spectra: Vec<WorkerHbBranchSpectrumTransport>,
    iterations: usize,
    residual_norm: f64,
    state_digest: crate::product::ContentDigest,
}

impl WorkerHbOperatingPointTransport {
    pub(super) fn from_operating_point(
        operating_point: rspice_core::engine::HbOperatingPoint,
        buffers: &mut Vec<Vec<f64>>,
    ) -> Self {
        let spectra = operating_point
            .node_names()
            .iter()
            .cloned()
            .zip(operating_point.spectral_state())
            .map(|(node_name, coefficients)| {
                let (real, imaginary): (Vec<_>, Vec<_>) = coefficients
                    .iter()
                    .map(|value| (value.re, value.im))
                    .unzip();
                WorkerHbSpectrumTransport {
                    node_name,
                    real_digest: crate::simulation::execution::f64_sequence_digest(
                        "rspice.worker-hb-spectrum-real/v1",
                        &real,
                    ),
                    imaginary_digest: crate::simulation::execution::f64_sequence_digest(
                        "rspice.worker-hb-spectrum-imaginary/v1",
                        &imaginary,
                    ),
                    real: WorkerF64Series::from_vec(real, buffers),
                    imaginary: WorkerF64Series::from_vec(imaginary, buffers),
                }
            })
            .collect();
        let mna_branch_spectra = operating_point
            .mna_branch_names()
            .iter()
            .cloned()
            .zip(operating_point.mna_branch_spectral_state())
            .map(|(branch_name, coefficients)| {
                let (real, imaginary): (Vec<_>, Vec<_>) = coefficients
                    .iter()
                    .map(|value| (value.re, value.im))
                    .unzip();
                WorkerHbBranchSpectrumTransport {
                    branch_name,
                    real_digest: crate::simulation::execution::f64_sequence_digest(
                        "rspice.worker-hb-branch-spectrum-real/v1",
                        &real,
                    ),
                    imaginary_digest: crate::simulation::execution::f64_sequence_digest(
                        "rspice.worker-hb-branch-spectrum-imaginary/v1",
                        &imaginary,
                    ),
                    real: WorkerF64Series::from_vec(real, buffers),
                    imaginary: WorkerF64Series::from_vec(imaginary, buffers),
                }
            })
            .collect();
        Self {
            config: operating_point.config().clone(),
            producer_identity: operating_point.producer_identity().cloned(),
            spectra,
            mna_branch_spectra,
            iterations: operating_point.iterations(),
            residual_norm: operating_point.residual_norm(),
            state_digest: crate::simulation::execution::hb_operating_point_digest(&operating_point),
        }
    }

    pub(super) fn into_operating_point(
        self,
        buffers: &[Vec<f64>],
    ) -> Result<rspice_core::engine::HbOperatingPoint, String> {
        if self
            .spectra
            .len()
            .checked_add(self.mna_branch_spectra.len())
            .is_none_or(|rows| rows > 65_536)
        {
            return Err("retained HB worker metadata exceeds structural limits".to_owned());
        }
        let mut node_names = Vec::with_capacity(self.spectra.len());
        let mut spectral_state = Vec::with_capacity(self.spectra.len());
        for spectrum in self.spectra {
            node_names.push(spectrum.node_name);
            let real = spectrum.real.into_vec(buffers)?;
            let imaginary = spectrum.imaginary.into_vec(buffers)?;
            let actual_real_digest = crate::simulation::execution::f64_sequence_digest(
                "rspice.worker-hb-spectrum-real/v1",
                &real,
            );
            let actual_imaginary_digest = crate::simulation::execution::f64_sequence_digest(
                "rspice.worker-hb-spectrum-imaginary/v1",
                &imaginary,
            );
            if actual_real_digest != spectrum.real_digest
                || actual_imaginary_digest != spectrum.imaginary_digest
            {
                return Err("retained HB worker spectral payload digest mismatch".to_owned());
            }
            spectral_state.push(worker_join_complex("HB spectral row", real, imaginary)?);
        }
        let mut mna_branch_names = Vec::with_capacity(self.mna_branch_spectra.len());
        let mut mna_branch_spectral_state = Vec::with_capacity(self.mna_branch_spectra.len());
        for spectrum in self.mna_branch_spectra {
            mna_branch_names.push(spectrum.branch_name);
            let real = spectrum.real.into_vec(buffers)?;
            let imaginary = spectrum.imaginary.into_vec(buffers)?;
            let actual_real_digest = crate::simulation::execution::f64_sequence_digest(
                "rspice.worker-hb-branch-spectrum-real/v1",
                &real,
            );
            let actual_imaginary_digest = crate::simulation::execution::f64_sequence_digest(
                "rspice.worker-hb-branch-spectrum-imaginary/v1",
                &imaginary,
            );
            if actual_real_digest != spectrum.real_digest
                || actual_imaginary_digest != spectrum.imaginary_digest
            {
                return Err(
                    "retained HB worker MNA branch spectral payload digest mismatch".to_owned(),
                );
            }
            mna_branch_spectral_state.push(worker_join_complex(
                "HB MNA branch spectral row",
                real,
                imaginary,
            )?);
        }
        let operating_point = if let Some(producer_identity) = self.producer_identity {
            rspice_core::engine::HbOperatingPoint::try_from_authenticated_parts_with_mna_branches(
                producer_identity,
                self.config,
                node_names,
                spectral_state,
                mna_branch_names,
                mna_branch_spectral_state,
                self.iterations,
                self.residual_norm,
            )
        } else {
            rspice_core::engine::HbOperatingPoint::try_from_parts_with_mna_branches(
                self.config,
                node_names,
                spectral_state,
                mna_branch_names,
                mna_branch_spectral_state,
                self.iterations,
                self.residual_norm,
            )
        }
        .map_err(|error| format!("invalid retained HB worker payload: {error}"))?;
        let actual_state_digest =
            crate::simulation::execution::hb_operating_point_digest(&operating_point);
        if actual_state_digest != self.state_digest {
            return Err(
                "retained HB worker state identity or configuration digest mismatch".to_owned(),
            );
        }
        Ok(operating_point)
    }
}

pub(super) fn worker_join_complex(
    label: &str,
    real: Vec<f64>,
    imaginary: Vec<f64>,
) -> Result<Vec<num_complex::Complex64>, String> {
    if real.len() != imaginary.len() {
        return Err(format!(
            "{label} real/imaginary lengths differ ({} versus {})",
            real.len(),
            imaginary.len()
        ));
    }
    Ok(real
        .into_iter()
        .zip(imaginary)
        .map(|(re, im)| num_complex::Complex64::new(re, im))
        .collect())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerPstbModesTransport {
    multiplier_real: WorkerF64Series,
    multiplier_imaginary: WorkerF64Series,
    exponent_real: WorkerF64Series,
    exponent_imaginary: WorkerF64Series,
    probe_participation: WorkerF64Series,
    is_unstable: Vec<bool>,
    is_trivial: Vec<bool>,
    subharmonic_order: Vec<Option<usize>>,
}

impl WorkerPstbModesTransport {
    fn from_modes(modes: Vec<WorkerPstbFloquetMode>, buffers: &mut Vec<Vec<f64>>) -> Self {
        let mut multiplier_real = Vec::with_capacity(modes.len());
        let mut multiplier_imaginary = Vec::with_capacity(modes.len());
        let mut exponent_real = Vec::with_capacity(modes.len());
        let mut exponent_imaginary = Vec::with_capacity(modes.len());
        let mut probe_participation = Vec::with_capacity(modes.len());
        let mut is_unstable = Vec::with_capacity(modes.len());
        let mut is_trivial = Vec::with_capacity(modes.len());
        let mut subharmonic_order = Vec::with_capacity(modes.len());
        for mode in modes {
            multiplier_real.push(mode.multiplier.0);
            multiplier_imaginary.push(mode.multiplier.1);
            exponent_real.push(mode.exponent.0);
            exponent_imaginary.push(mode.exponent.1);
            probe_participation.push(mode.probe_participation);
            is_unstable.push(mode.is_unstable);
            is_trivial.push(mode.is_trivial);
            subharmonic_order.push(mode.subharmonic_order);
        }
        Self {
            multiplier_real: WorkerF64Series::from_vec(multiplier_real, buffers),
            multiplier_imaginary: WorkerF64Series::from_vec(multiplier_imaginary, buffers),
            exponent_real: WorkerF64Series::from_vec(exponent_real, buffers),
            exponent_imaginary: WorkerF64Series::from_vec(exponent_imaginary, buffers),
            probe_participation: WorkerF64Series::from_vec(probe_participation, buffers),
            is_unstable,
            is_trivial,
            subharmonic_order,
        }
    }

    fn into_modes(self, buffers: &[Vec<f64>]) -> Result<Vec<WorkerPstbFloquetMode>, String> {
        let len = self.multiplier_real.len();
        if len > MAX_WORKER_F64_VALUES
            || self.multiplier_imaginary.len() != len
            || self.exponent_real.len() != len
            || self.exponent_imaginary.len() != len
            || self.probe_participation.len() != len
            || self.is_unstable.len() != len
            || self.is_trivial.len() != len
            || self.subharmonic_order.len() != len
        {
            return Err("PSTB mode columns have inconsistent cardinality".to_owned());
        }
        let multiplier_real = self.multiplier_real.into_vec(buffers)?;
        let multiplier_imaginary = self.multiplier_imaginary.into_vec(buffers)?;
        let exponent_real = self.exponent_real.into_vec(buffers)?;
        let exponent_imaginary = self.exponent_imaginary.into_vec(buffers)?;
        let probe_participation = self.probe_participation.into_vec(buffers)?;
        Ok((0..len)
            .map(|index| WorkerPstbFloquetMode {
                multiplier: (multiplier_real[index], multiplier_imaginary[index]),
                exponent: (exponent_real[index], exponent_imaginary[index]),
                probe_participation: probe_participation[index],
                is_unstable: self.is_unstable[index],
                is_trivial: self.is_trivial[index],
                subharmonic_order: self.subharmonic_order[index],
            })
            .collect())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerSimulationResultTransport {
    Inline(WorkerSimulationResult),
    DcOp {
        configuration: WorkerOpConfigTransport,
        validated_startup_directives: usize,
        mna_node_names: Vec<String>,
        mna_branch_names: Vec<String>,
        mna_solution: WorkerF64Series,
        mna_solution_digest: crate::product::ContentDigest,
        node_voltages: HashMap<String, f64>,
        branch_currents: HashMap<String, f64>,
        device_report: Option<WorkerDeviceOpReport>,
    },
    DcSweep {
        sweep_var: String,
        sweep_values: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        measurements: Vec<WorkerMeasurement>,
    },
    Transient {
        time: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        measurements: Vec<WorkerMeasurement>,
        /// Event histories ride the JSON envelope rather than the binary
        /// buffer channel: they are short, and their times are the datum, not
        /// a resampling of `time`.
        #[serde(default)]
        events: WorkerEventHistory,
    },
    Pss {
        measurements: Vec<WorkerMeasurement>,
        operating_point: WorkerPssOperatingPointTransport,
    },
    Pstb {
        period: f64,
        fundamental_frequency: f64,
        stability_threshold: f64,
        probe_instance: String,
        detect_subharmonics: bool,
        modes: WorkerPstbModesTransport,
        floquet_evidence: rspice_core::analysis::FloquetSpectrumEvidence,
        orbit_kind: rspice_core::analysis::FloquetOrbitKind,
        trivial_multiplier_index: Option<usize>,
        stability_verdict: rspice_core::analysis::FloquetStabilityVerdict,
        stability_classification: WorkerPstbStabilityClassification,
        min_stability_margin_db: Option<f64>,
        max_multiplier_magnitude: f64,
        num_unstable: usize,
        subharmonics: Vec<usize>,
        converged: bool,
        iterations: usize,
        mode_indices: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
    },
    Hb {
        frequencies: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        measurements: Vec<WorkerMeasurement>,
        operating_point: WorkerHbOperatingPointTransport,
    },
    Ac {
        frequencies: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        measurements: Vec<WorkerMeasurement>,
    },
    Noise {
        frequencies: WorkerF64Series,
        output_noise: WorkerF64Series,
        input_noise: Option<WorkerF64Series>,
        contributors: HashMap<String, WorkerF64Series>,
        #[serde(default)]
        summary: Option<WorkerNoiseSummary>,
        #[serde(default)]
        measurements: Vec<WorkerMeasurement>,
    },
    Parametric {
        target: String,
        sweep_values: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        num_failures: usize,
        #[serde(default)]
        member_measurements: Vec<crate::state::FamilyMemberMeasurements>,
    },
    Corner {
        x_values: WorkerF64Series,
        x_label: String,
        x_unit: String,
        temperatures_c: WorkerF64Series,
        corner_labels: Vec<String>,
        waveforms: Vec<WorkerWaveformTransport>,
        num_failures: usize,
        #[serde(default)]
        member_measurements: Vec<crate::state::FamilyMemberMeasurements>,
    },
    Reliability {
        years: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        device_results: Vec<WorkerReliabilityResult>,
    },
    Optimization {
        iterations: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        best_cost: f64,
        best_variables: HashMap<String, f64>,
        converged: bool,
    },
    Soa {
        time: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        violations: Vec<WorkerSoAViolation>,
        evaluations: Vec<WorkerSoAEvaluation>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkerOpConfigTransport {
    config: crate::simulation::dialog::OpConfig,
    #[serde(default)]
    previous_state: Option<WorkerOpPreviousStateTransport>,
}

impl WorkerOpConfigTransport {
    pub(super) fn from_config(
        mut config: crate::simulation::dialog::OpConfig,
        buffers: &mut Vec<Vec<f64>>,
    ) -> Self {
        debug_assert!(config.validate_for_execution().is_ok());
        let previous_state = config.previous_state.take().map(|state| {
            WorkerOpPreviousStateTransport::from_validated_previous_state(state, buffers)
        });
        Self {
            config,
            previous_state,
        }
    }

    pub(super) fn into_config(
        mut self,
        buffers: &[Vec<f64>],
    ) -> Result<crate::simulation::dialog::OpConfig, String> {
        if self.config.previous_state.is_some() {
            return Err(
                "worker DC operating-point response carries a duplicate inline previous-state solution"
                    .to_owned(),
            );
        }
        self.config.previous_state = self
            .previous_state
            .map(|state| state.into_previous_state(buffers))
            .transpose()?;
        self.config.validate_for_execution()?;
        Ok(self.config)
    }
}

impl WorkerSimulationResultTransport {
    pub(super) fn from_result(result: WorkerSimulationResult, buffers: &mut Vec<Vec<f64>>) -> Self {
        match result {
            WorkerSimulationResult::DcOp {
                configuration,
                validated_startup_directives,
                mna_node_names,
                mna_branch_names,
                mna_solution,
                node_voltages,
                branch_currents,
                device_report,
            } => Self::DcOp {
                configuration: WorkerOpConfigTransport::from_config(configuration, buffers),
                validated_startup_directives,
                mna_node_names,
                mna_branch_names,
                mna_solution_digest: crate::simulation::execution::f64_sequence_digest(
                    "rspice.worker-dc-op-mna/v1",
                    &mna_solution,
                ),
                mna_solution: WorkerF64Series::from_vec(mna_solution, buffers),
                node_voltages,
                branch_currents,
                device_report,
            },
            WorkerSimulationResult::DcSweep {
                sweep_var,
                sweep_values,
                waveforms,
                measurements,
            } => Self::DcSweep {
                sweep_var,
                sweep_values: WorkerF64Series::from_vec(sweep_values, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                measurements,
            },
            WorkerSimulationResult::Transient {
                time,
                waveforms,
                measurements,
                events,
            } => Self::Transient {
                time: WorkerF64Series::from_vec(time, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                measurements,
                events,
            },
            WorkerSimulationResult::Pss {
                measurements,
                operating_point,
            } => Self::Pss {
                measurements,
                operating_point: WorkerPssOperatingPointTransport::from_operating_point(
                    operating_point,
                    buffers,
                ),
            },
            WorkerSimulationResult::Pstb {
                period,
                fundamental_frequency,
                stability_threshold,
                probe_instance,
                detect_subharmonics,
                modes,
                floquet_evidence,
                orbit_kind,
                trivial_multiplier_index,
                stability_verdict,
                stability_classification,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                subharmonics,
                converged,
                iterations,
                mode_indices,
                waveforms,
            } => Self::Pstb {
                period,
                fundamental_frequency,
                stability_threshold,
                probe_instance,
                detect_subharmonics,
                modes: WorkerPstbModesTransport::from_modes(modes, buffers),
                floquet_evidence,
                orbit_kind,
                trivial_multiplier_index,
                stability_verdict,
                stability_classification,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                subharmonics,
                converged,
                iterations,
                mode_indices: WorkerF64Series::from_vec(mode_indices, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
            },
            WorkerSimulationResult::Hb {
                frequencies,
                waveforms,
                measurements,
                operating_point,
            } => Self::Hb {
                frequencies: WorkerF64Series::from_vec(frequencies, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                measurements,
                operating_point: WorkerHbOperatingPointTransport::from_operating_point(
                    operating_point,
                    buffers,
                ),
            },
            WorkerSimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
            } => Self::Ac {
                frequencies: WorkerF64Series::from_vec(frequencies, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                measurements,
            },
            WorkerSimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
                measurements,
            } => Self::Noise {
                frequencies: WorkerF64Series::from_vec(frequencies, buffers),
                output_noise: WorkerF64Series::from_vec(output_noise, buffers),
                input_noise: input_noise.map(|values| WorkerF64Series::from_vec(values, buffers)),
                contributors: contributors
                    .into_iter()
                    .map(|(name, values)| (name, WorkerF64Series::from_vec(values, buffers)))
                    .collect(),
                summary,
                measurements,
            },
            WorkerSimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
                member_measurements,
            } => Self::Parametric {
                target,
                sweep_values: WorkerF64Series::from_vec(sweep_values, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                num_failures,
                member_measurements,
            },
            WorkerSimulationResult::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms,
                num_failures,
                member_measurements,
            } => Self::Corner {
                x_values: WorkerF64Series::from_vec(x_values, buffers),
                x_label,
                x_unit,
                temperatures_c: WorkerF64Series::from_vec(temperatures_c, buffers),
                corner_labels,
                waveforms: transport_waveforms(waveforms, buffers),
                num_failures,
                member_measurements,
            },
            WorkerSimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => Self::Reliability {
                years: WorkerF64Series::from_vec(years, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                device_results,
            },
            WorkerSimulationResult::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                converged,
            } => Self::Optimization {
                iterations: WorkerF64Series::from_vec(iterations, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                best_cost,
                best_variables,
                converged,
            },
            WorkerSimulationResult::Soa {
                time,
                waveforms,
                violations,
                evaluations,
            } => Self::Soa {
                time: WorkerF64Series::from_vec(time, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                violations,
                evaluations,
            },
            other => Self::Inline(other),
        }
    }

    pub(super) fn into_result(
        self,
        buffers: &[Vec<f64>],
    ) -> Result<WorkerSimulationResult, String> {
        match self {
            Self::Inline(result) => {
                if matches!(result, WorkerSimulationResult::Pstb { .. }) {
                    return Err(
                        "PSTB worker result must use the dedicated transfer-buffer transport"
                            .to_owned(),
                    );
                }
                Ok(result)
            }
            Self::DcOp {
                configuration,
                validated_startup_directives,
                mna_node_names,
                mna_branch_names,
                mna_solution,
                mna_solution_digest,
                node_voltages,
                branch_currents,
                device_report,
            } => {
                let configuration = configuration.into_config(buffers)?;
                let mna_solution = mna_solution.into_vec(buffers)?;
                let actual_digest = crate::simulation::execution::f64_sequence_digest(
                    "rspice.worker-dc-op-mna/v1",
                    &mna_solution,
                );
                if actual_digest != mna_solution_digest {
                    return Err(format!(
                        "worker DC operating-point MNA payload digest is {actual_digest}, expected {mna_solution_digest}"
                    ));
                }
                validate_worker_dc_op_state(
                    &configuration,
                    &mna_node_names,
                    &mna_branch_names,
                    &mna_solution,
                )?;
                Ok(WorkerSimulationResult::DcOp {
                    configuration,
                    validated_startup_directives,
                    mna_node_names,
                    mna_branch_names,
                    mna_solution,
                    node_voltages,
                    branch_currents,
                    device_report,
                })
            }
            Self::DcSweep {
                sweep_var,
                sweep_values,
                waveforms,
                measurements,
            } => Ok(WorkerSimulationResult::DcSweep {
                sweep_var,
                sweep_values: sweep_values.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                measurements,
            }),
            Self::Transient {
                time,
                waveforms,
                measurements,
                events,
            } => Ok(WorkerSimulationResult::Transient {
                time: time.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                measurements,
                events,
            }),
            Self::Pss {
                measurements,
                operating_point,
            } => Ok(WorkerSimulationResult::Pss {
                measurements,
                operating_point: operating_point.into_operating_point(buffers)?,
            }),
            Self::Pstb {
                period,
                fundamental_frequency,
                stability_threshold,
                probe_instance,
                detect_subharmonics,
                modes,
                floquet_evidence,
                orbit_kind,
                trivial_multiplier_index,
                stability_verdict,
                stability_classification,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                subharmonics,
                converged,
                iterations,
                mode_indices,
                waveforms,
            } => {
                let result = WorkerSimulationResult::Pstb {
                    period,
                    fundamental_frequency,
                    stability_threshold,
                    probe_instance,
                    detect_subharmonics,
                    modes: modes.into_modes(buffers)?,
                    floquet_evidence,
                    orbit_kind,
                    trivial_multiplier_index,
                    stability_verdict,
                    stability_classification,
                    min_stability_margin_db,
                    max_multiplier_magnitude,
                    num_unstable,
                    subharmonics,
                    converged,
                    iterations,
                    mode_indices: mode_indices.into_vec(buffers)?,
                    waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                };
                validate_worker_pstb_result(&result)?;
                Ok(result)
            }
            Self::Hb {
                frequencies,
                waveforms,
                measurements,
                operating_point,
            } => Ok(WorkerSimulationResult::Hb {
                frequencies: frequencies.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                measurements,
                operating_point: operating_point.into_operating_point(buffers)?,
            }),
            Self::Ac {
                frequencies,
                waveforms,
                measurements,
            } => Ok(WorkerSimulationResult::Ac {
                frequencies: frequencies.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                measurements,
            }),
            Self::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
                measurements,
            } => Ok(WorkerSimulationResult::Noise {
                frequencies: frequencies.into_vec(buffers)?,
                output_noise: output_noise.into_vec(buffers)?,
                input_noise: input_noise
                    .map(|values| values.into_vec(buffers))
                    .transpose()?,
                contributors: contributors
                    .into_iter()
                    .map(|(name, values)| values.into_vec(buffers).map(|values| (name, values)))
                    .collect::<Result<_, _>>()?,
                summary,
                measurements,
            }),
            Self::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
                member_measurements,
            } => Ok(WorkerSimulationResult::Parametric {
                target,
                sweep_values: sweep_values.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                num_failures,
                member_measurements,
            }),
            Self::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms,
                num_failures,
                member_measurements,
            } => Ok(WorkerSimulationResult::Corner {
                x_values: x_values.into_vec(buffers)?,
                x_label,
                x_unit,
                temperatures_c: temperatures_c.into_vec(buffers)?,
                corner_labels,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                num_failures,
                member_measurements,
            }),
            Self::Reliability {
                years,
                waveforms,
                device_results,
            } => Ok(WorkerSimulationResult::Reliability {
                years: years.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                device_results,
            }),
            Self::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                converged,
            } => Ok(WorkerSimulationResult::Optimization {
                iterations: iterations.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                best_cost,
                best_variables,
                converged,
            }),
            Self::Soa {
                time,
                waveforms,
                violations,
                evaluations,
            } => Ok(WorkerSimulationResult::Soa {
                time: time.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                violations,
                evaluations,
            }),
        }
    }
}

pub(super) fn validate_worker_dc_op_state(
    configuration: &crate::simulation::dialog::OpConfig,
    node_names: &[String],
    branch_names: &[String],
    solution: &[f64],
) -> Result<(), String> {
    configuration.validate_for_execution()?;
    rspice_core::engine::PssDcOperatingPointSeed::try_new(
        node_names.to_vec(),
        branch_names.to_vec(),
        solution.to_vec(),
    )
    .map(|_| ())
    .map_err(|error| format!("worker DC operating-point state is invalid: {error}"))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerWaveformTransport {
    pub name: String,
    pub x_values: WorkerF64Series,
    pub y_values: WorkerF64Series,
    pub y_unit: String,
    pub is_complex: bool,
    pub y_imag: Option<WorkerF64Series>,
}

impl WorkerWaveformTransport {
    pub(super) fn from_waveform(waveform: WorkerWaveform, buffers: &mut Vec<Vec<f64>>) -> Self {
        Self {
            name: waveform.name,
            x_values: WorkerF64Series::from_vec(waveform.x_values, buffers),
            y_values: WorkerF64Series::from_vec(waveform.y_values, buffers),
            y_unit: waveform.y_unit,
            is_complex: waveform.is_complex,
            y_imag: waveform
                .y_imag
                .map(|values| WorkerF64Series::from_vec(values, buffers)),
        }
    }

    pub(super) fn into_waveform(self, buffers: &[Vec<f64>]) -> Result<WorkerWaveform, String> {
        let x_len = self.x_values.len();
        let y_len = self.y_values.len();
        let imag_len = self.y_imag.as_ref().map(WorkerF64Series::len);

        if x_len != y_len {
            return Err(format!(
                "waveform {} x/y length mismatch: x length {x_len}, y length {y_len}",
                self.name
            ));
        }
        match (self.is_complex, imag_len) {
            (true, Some(len)) if len == y_len => {}
            (true, Some(len)) => {
                return Err(format!(
                    "complex waveform {} imaginary length {len} does not match y length {y_len}",
                    self.name
                ));
            }
            (true, None) => {
                return Err(format!(
                    "complex waveform {} is missing an imaginary buffer",
                    self.name
                ));
            }
            (false, Some(_)) => {
                return Err(format!(
                    "non-complex waveform {} must not include an imaginary buffer",
                    self.name
                ));
            }
            (false, None) => {}
        }

        Ok(WorkerWaveform {
            name: self.name,
            x_values: self.x_values.into_vec(buffers)?,
            y_values: self.y_values.into_vec(buffers)?,
            y_unit: self.y_unit,
            is_complex: self.is_complex,
            y_imag: self
                .y_imag
                .map(|values| values.into_vec(buffers))
                .transpose()?,
        })
    }
}

pub(super) fn transport_waveforms(
    waveforms: Vec<WorkerWaveform>,
    buffers: &mut Vec<Vec<f64>>,
) -> Vec<WorkerWaveformTransport> {
    waveforms
        .into_iter()
        .map(|waveform| WorkerWaveformTransport::from_waveform(waveform, buffers))
        .collect()
}

pub(super) fn worker_waveforms_from_transport(
    waveforms: Vec<WorkerWaveformTransport>,
    buffers: &[Vec<f64>],
) -> Result<Vec<WorkerWaveform>, String> {
    waveforms
        .into_iter()
        .map(|waveform| waveform.into_waveform(buffers))
        .collect()
}

#[cfg(test)]
mod hb_state_contract_tests {
    use super::*;

    fn transport() -> (WorkerHbOperatingPointTransport, Vec<Vec<f64>>) {
        let mut buffers = Vec::new();
        let transport = WorkerHbOperatingPointTransport::from_operating_point(
            super::super::tests::retained_hb_operating_point(),
            &mut buffers,
        );
        (transport, buffers)
    }

    #[test]
    fn retained_hb_transport_authenticates_branch_identity_and_configuration() {
        let (mut identity, buffers) = transport();
        identity.mna_branch_spectra[0].branch_name = "VDRIFT".to_owned();
        assert!(
            identity
                .into_operating_point(&buffers)
                .unwrap_err()
                .contains("identity or configuration digest mismatch")
        );

        let (mut config, buffers) = transport();
        config.config.tolerance *= 10.0;
        assert!(
            config
                .into_operating_point(&buffers)
                .unwrap_err()
                .contains("identity or configuration digest mismatch")
        );
    }

    #[test]
    fn current_zero_branch_state_still_round_trips_without_invented_branches() {
        let config = rspice_core::analysis::HbConfig::new(1.0).with_harmonics(1);
        let operating_point = rspice_core::engine::HbOperatingPoint::try_from_parts(
            config,
            vec!["out".to_owned()],
            vec![vec![
                num_complex::Complex64::new(0.5, 0.0),
                num_complex::Complex64::new(0.1, -0.2),
            ]],
            2,
            1.0e-9,
        )
        .unwrap();
        let mut buffers = Vec::new();
        let transport = WorkerHbOperatingPointTransport::from_operating_point(
            operating_point.clone(),
            &mut buffers,
        );
        assert!(transport.mna_branch_spectra.is_empty());
        assert_eq!(
            transport.into_operating_point(&buffers).unwrap(),
            operating_point
        );
    }
}

#[cfg(test)]
mod pss_floquet_contract_tests {
    use super::*;

    fn transport() -> (WorkerPssOperatingPointTransport, Vec<Vec<f64>>) {
        let mut buffers = Vec::new();
        let transport = WorkerPssOperatingPointTransport::from_operating_point(
            super::super::tests::retained_pss_operating_point(),
            &mut buffers,
        );
        (transport, buffers)
    }

    fn authenticated_transport() -> (
        rspice_core::engine::PssOperatingPoint,
        WorkerPssOperatingPointTransport,
        Vec<Vec<f64>>,
    ) {
        let netlist = rspice_core::netlist::Netlist::parse(
            "* authenticated worker PSS fixture\n\
             V1 in 0 DC 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             C1 out 0 1p\n\
             .end\n",
        )
        .unwrap();
        let config = rspice_core::analysis::PssConfig::new(1.0e6)
            .with_harmonics(4)
            .with_points_per_period(32)
            .with_tstab_periods(0);
        let operating_point = rspice_core::engine::Engine::default()
            .run_pss_operating_point_with_abort(
                &netlist,
                config,
                &rspice_core::abort_signal::NoAbort,
            )
            .unwrap();
        let mut buffers = Vec::new();
        let transport = WorkerPssOperatingPointTransport::from_operating_point(
            operating_point.clone(),
            &mut buffers,
        );
        (operating_point, transport, buffers)
    }

    #[test]
    fn retained_pss_transport_round_trips_authenticated_floquet_contract() {
        let (transport, buffers) = transport();
        assert!(transport.analysis_floquet_authenticated);
        assert_eq!(
            transport.analysis_floquet_verdict,
            rspice_core::analysis::FloquetStabilityVerdict::Stable
        );

        let restored = transport.into_operating_point(&buffers).unwrap();
        assert_eq!(
            restored,
            super::super::tests::retained_pss_operating_point()
        );
    }

    #[test]
    fn retained_pss_worker_transport_preserves_identity_and_rejects_numeric_tamper() {
        let (operating_point, transport, buffers) = authenticated_transport();
        assert!(transport.producer_identity.is_some());
        assert_eq!(transport.shooting_state_basis, ["C:C1"]);

        let restored = transport.clone().into_operating_point(&buffers).unwrap();
        assert_eq!(
            restored.producer_identity(),
            operating_point.producer_identity()
        );
        assert_eq!(restored.shooting_state_basis(), ["C:C1"]);

        let mut tampered = buffers;
        tampered.last_mut().unwrap()[0] += 0.25;
        let error = transport.into_operating_point(&tampered).unwrap_err();
        assert!(
            error.contains("numerical payload does not match"),
            "retained shooting-state tamper should fail core authentication: {error}"
        );
    }

    #[test]
    fn retained_pss_transport_missing_evidence_stays_legacy_and_is_rejected() {
        let (transport, buffers) = transport();
        let mut encoded = serde_json::to_value(transport).unwrap();
        let object = encoded.as_object_mut().unwrap();
        object.remove("result_floquet_evidence");
        object.remove("result_floquet_orbit_kind");
        object.remove("result_trivial_floquet_multiplier_index");
        object.remove("analysis_floquet_verdict");
        object.remove("analysis_floquet_authenticated");
        let restored: WorkerPssOperatingPointTransport = serde_json::from_value(encoded).unwrap();
        assert_eq!(
            restored.result_floquet_evidence,
            rspice_core::analysis::FloquetSpectrumEvidence::LegacyUnknown
        );
        assert!(!restored.analysis_floquet_authenticated);
        assert_eq!(
            restored.analysis_floquet_verdict,
            rspice_core::analysis::FloquetStabilityVerdict::Indeterminate
        );
        assert!(restored.into_operating_point(&buffers).is_err());
    }

    #[test]
    fn retained_pss_transport_parses_pre_identity_metadata_as_untrusted_legacy_state() {
        let (transport, buffers) = transport();
        let mut encoded = serde_json::to_value(transport).unwrap();
        let object = encoded.as_object_mut().unwrap();
        object.remove("producer_identity");
        object.remove("shooting_state_basis");

        let restored: WorkerPssOperatingPointTransport = serde_json::from_value(encoded).unwrap();
        let operating_point = restored.into_operating_point(&buffers).unwrap();
        assert!(operating_point.producer_identity().is_none());
        assert!(operating_point.shooting_state_basis().is_empty());
    }

    #[test]
    fn retained_pss_transport_rejects_noncanonical_certificate_and_compatibility_drift() {
        let (mut inflated, buffers) = transport();
        let rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate } =
            &mut inflated.result_floquet_evidence
        else {
            panic!("fixture must carry qualified evidence")
        };
        certificate.qualification_tolerance = 1.0;
        assert!(inflated.into_operating_point(&buffers).is_err());

        let (mut roots, buffers) = transport();
        roots.analysis_floquet_real = WorkerF64Series::Inline(vec![0.8]);
        assert!(roots.into_operating_point(&buffers).is_err());

        let (mut stable, buffers) = transport();
        stable.analysis_is_stable = false;
        assert!(stable.into_operating_point(&buffers).is_err());

        let (mut orbit, buffers) = transport();
        orbit.result_floquet_orbit_kind = rspice_core::analysis::FloquetOrbitKind::Autonomous;
        assert!(orbit.into_operating_point(&buffers).is_err());

        let (mut trivial, buffers) = transport();
        trivial.result_trivial_floquet_multiplier_index = Some(usize::MAX);
        assert!(trivial.into_operating_point(&buffers).is_err());
    }

    #[test]
    fn zero_order_autonomous_contract_is_not_authenticated() {
        let mut result = rspice_core::analysis::pss::PssResult::new(1.0, 0, 0);
        result.set_floquet_spectrum(
            Vec::new(),
            rspice_core::analysis::FloquetSpectrumEvidence::NoDynamicModes,
            rspice_core::analysis::FloquetOrbitKind::Autonomous,
        );
        assert!(!pss_floquet_contract_is_authenticated(&result, 0));
    }
}

#[cfg(test)]
mod pstb_floquet_contract_tests {
    use super::*;

    fn response_and_worker() -> (WorkerResponse, WorkerSimulationResult) {
        let worker =
            WorkerSimulationResult::try_from(super::super::tests::authenticated_pstb_result())
                .unwrap();
        (
            WorkerResponse {
                id: 73,
                outcome: WorkerOutcome::Success(Box::new(worker.clone())),
            },
            worker,
        )
    }

    fn transport() -> WorkerResponseTransport {
        let (response, _) = response_and_worker();
        WorkerResponseTransport::from_response(response).unwrap()
    }

    fn pstb_payload_mut(
        transport: &mut WorkerResponseTransport,
    ) -> &mut WorkerSimulationResultTransport {
        let WorkerOutcomeTransport::Success(payload) = &mut transport.response.outcome else {
            panic!("fixture must be a successful PSTB response")
        };
        payload
    }

    #[test]
    fn pstb_transport_round_trips_complete_spectrum_separately_from_display_projection() {
        let (response, expected) = response_and_worker();
        let transport = WorkerResponseTransport::from_response(response).unwrap();
        let WorkerOutcomeTransport::Success(WorkerSimulationResultTransport::Pstb {
            modes,
            mode_indices,
            ..
        }) = &transport.response.outcome
        else {
            panic!("PSTB must use its dedicated transport variant")
        };
        assert_eq!(modes.multiplier_real.len(), 2);
        assert_eq!(modes.exponent_imaginary.len(), 2);
        assert_eq!(mode_indices.len(), 1);
        assert!(transport.buffers.len() >= 6);

        let restored = transport.into_response().unwrap();
        let WorkerOutcome::Success(restored) = restored.outcome else {
            panic!("PSTB response must remain successful")
        };
        assert_eq!(*restored, expected);
    }

    #[test]
    fn pstb_transport_rejects_truncated_mode_column() {
        let mut transport = transport();
        let WorkerSimulationResultTransport::Pstb { modes, .. } = pstb_payload_mut(&mut transport)
        else {
            panic!("fixture must be PSTB")
        };
        modes.multiplier_imaginary = WorkerF64Series::Inline(vec![0.0]);
        assert!(transport.into_response().is_err());
    }

    #[test]
    fn pstb_transport_rejects_inline_bypass_of_dedicated_numeric_buffers() {
        let (_, worker) = response_and_worker();
        let transport = WorkerResponseTransport {
            protocol: WORKER_RESPONSE_TRANSPORT_PROTOCOL,
            response: WorkerResponseTransportMetadata {
                id: 73,
                outcome: WorkerOutcomeTransport::Success(WorkerSimulationResultTransport::Inline(
                    worker,
                )),
            },
            buffers: Vec::new(),
        };
        assert!(transport.into_response().is_err());
    }

    #[test]
    fn pstb_transport_rejects_forged_or_mismatched_floquet_evidence() {
        let mut inflated = transport();
        let WorkerSimulationResultTransport::Pstb {
            floquet_evidence, ..
        } = pstb_payload_mut(&mut inflated)
        else {
            panic!("fixture must be PSTB")
        };
        let rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate } =
            floquet_evidence
        else {
            panic!("fixture must be qualified")
        };
        certificate.qualification_tolerance = 1.0;
        assert!(inflated.into_response().is_err());

        let mut mismatched = transport();
        let WorkerSimulationResultTransport::Pstb {
            floquet_evidence, ..
        } = pstb_payload_mut(&mut mismatched)
        else {
            panic!("fixture must be PSTB")
        };
        *floquet_evidence = rspice_core::analysis::FloquetSpectrumEvidence::NoDynamicModes;
        assert!(mismatched.into_response().is_err());
    }

    #[test]
    fn pstb_transport_rejects_forged_provenance_and_aggregate_metadata() {
        let mut blank_probe = transport();
        let WorkerSimulationResultTransport::Pstb { probe_instance, .. } =
            pstb_payload_mut(&mut blank_probe)
        else {
            panic!("fixture must be PSTB")
        };
        probe_instance.clear();
        assert!(blank_probe.into_response().is_err());

        let mut threshold = transport();
        let WorkerSimulationResultTransport::Pstb {
            stability_threshold,
            ..
        } = pstb_payload_mut(&mut threshold)
        else {
            panic!("fixture must be PSTB")
        };
        *stability_threshold = 0.5;
        assert!(threshold.into_response().is_err());

        let mut policy = transport();
        let WorkerSimulationResultTransport::Pstb {
            detect_subharmonics,
            modes,
            ..
        } = pstb_payload_mut(&mut policy)
        else {
            panic!("fixture must be PSTB")
        };
        *detect_subharmonics = false;
        modes.subharmonic_order[0] = Some(2);
        assert!(policy.into_response().is_err());

        let mut count = transport();
        let WorkerSimulationResultTransport::Pstb { num_unstable, .. } =
            pstb_payload_mut(&mut count)
        else {
            panic!("fixture must be PSTB")
        };
        *num_unstable = 1;
        assert!(count.into_response().is_err());

        let (response, _) = response_and_worker();
        let WorkerOutcome::Success(worker) = response.outcome else {
            panic!("fixture must be successful")
        };
        let mut encoded = serde_json::to_value(&*worker).unwrap();
        encoded
            .get_mut("Pstb")
            .and_then(serde_json::Value::as_object_mut)
            .unwrap()
            .remove("floquet_evidence");
        assert!(serde_json::from_value::<WorkerSimulationResult>(encoded).is_err());
    }
}
