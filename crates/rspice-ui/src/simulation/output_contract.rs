//! Immutable saved-output preparation and result materialization.
//!
//! Project rows are compiled against the exact prepared analysis identity.
//! Dispatch therefore carries no reference to mutable workspace state, and
//! result receipts authenticate the contract that actually produced data.

use std::sync::Arc;

use crate::analysis::calculator::{self, CalcValue};
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SavedOutputId};
use crate::simulation::execution::{analysis_kind_tag, content_digest};
use crate::simulation::multi_run::{AnalysisRunType, AnalysisSpec, FrequencySweep};
use crate::state::{
    AnalysisResult, DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, SavedOutput, SavedOutputCompatibility,
    SavedOutputKind, SavedOutputMaterializationStatus, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputReceipt, SavedOutputStreaming, WaveformData,
};

const MAX_SELECTED_POINT_COUNT: usize = 10_000_000;

/// Static validation result for a candidate output contract. `RuntimeBound`
/// is not a placeholder: it records the precise evidence that cannot exist
/// until the solver has produced the retained source dataset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SavedOutputSemanticStatus {
    Valid { detail: String },
    RuntimeBound { reason: String },
    Invalid { reason: String },
}

/// Additional retained waveform/cache bytes attributable to one candidate
/// output across all enabled compatible prepared tasks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SavedOutputStorageEstimate {
    ExactBytes(u64),
    Indeterminate { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedOutputPreflightReport {
    semantic_status: SavedOutputSemanticStatus,
    storage_estimate: SavedOutputStorageEstimate,
    compatible_analysis_count: usize,
}

impl SavedOutputPreflightReport {
    pub(crate) fn invalid(reason: impl Into<String>) -> Self {
        let reason = reason.into();
        Self {
            semantic_status: SavedOutputSemanticStatus::Invalid {
                reason: reason.clone(),
            },
            storage_estimate: SavedOutputStorageEstimate::Indeterminate { reason },
            compatible_analysis_count: 0,
        }
    }

    pub const fn semantic_status(&self) -> &SavedOutputSemanticStatus {
        &self.semantic_status
    }

    pub const fn storage_estimate(&self) -> &SavedOutputStorageEstimate {
        &self.storage_estimate
    }

    pub const fn compatible_analysis_count(&self) -> usize {
        self.compatible_analysis_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::simulation) struct TransientSelectionGrid {
    pub start: f64,
    pub step: f64,
    pub stop: f64,
}

/// One output contract resolved for exactly one prepared analysis task.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::simulation) struct PreparedSavedOutput {
    output_id: SavedOutputId,
    output_revision: ObjectRevision,
    analysis_id: AnalysisInstanceId,
    kind: SavedOutputKind,
    name: String,
    source_expression: String,
    policy: SavedOutputPolicy,
    precision: SavedOutputPrecision,
    streaming: SavedOutputStreaming,
    selection_grid: Option<TransientSelectionGrid>,
    digest: ContentDigest,
}

impl PreparedSavedOutput {
    pub(in crate::simulation) fn prepare(
        output: &SavedOutput,
        analysis_id: AnalysisInstanceId,
        spec: &AnalysisSpec,
    ) -> Result<Option<Self>, String> {
        output.validate()?;
        let selected = match &output.compatible_analyses {
            SavedOutputCompatibility::OpTranAc => {
                matches!(
                    spec.run_type(),
                    AnalysisRunType::DcOp | AnalysisRunType::Transient | AnalysisRunType::Ac
                ) && output_kind_supports_run_type(output.kind, spec.run_type())
            }
            SavedOutputCompatibility::AllCompatibleAnalyses => {
                output_kind_supports_run_type(output.kind, spec.run_type())
            }
            SavedOutputCompatibility::SelectedAnalysis {
                analysis_id: selected,
            } => {
                if *selected != analysis_id {
                    false
                } else if !output_kind_supports_run_type(output.kind, spec.run_type()) {
                    return Err(format!(
                        "saved output '{}' selects analysis {analysis_id}, but {} outputs are incompatible with {}",
                        output.name,
                        output.kind.label(),
                        spec.run_type().display_name()
                    ));
                } else {
                    true
                }
            }
        };
        if !selected {
            return Ok(None);
        }
        if !output_kind_supports_run_type(output.kind, spec.run_type()) {
            return Err(format!(
                "saved output '{}' cannot be materialized by {}",
                output.name,
                spec.run_type().display_name()
            ));
        }
        validate_static_contract_semantics(output, spec)?;

        let selection_grid = match (output.save_policy, spec) {
            (
                SavedOutputPolicy::SelectedAndFinalPoints,
                AnalysisSpec::Transient {
                    stop_time,
                    step_time,
                    start_time,
                    ..
                },
            ) => Some(TransientSelectionGrid {
                start: *start_time,
                step: *step_time,
                stop: *stop_time,
            }),
            _ => None,
        };
        if let Some(grid) = selection_grid {
            validate_selection_grid(grid)?;
        }
        let digest = output_contract_digest(output, analysis_id, spec, selection_grid);
        Ok(Some(Self {
            output_id: output.id,
            output_revision: output.revision,
            analysis_id,
            kind: output.kind,
            name: output.name.clone(),
            source_expression: output.source_expression.clone(),
            policy: output.save_policy,
            precision: output.stored_precision,
            streaming: output.streaming,
            selection_grid,
            digest,
        }))
    }

    pub(in crate::simulation) const fn output_id(&self) -> SavedOutputId {
        self.output_id
    }

    pub(in crate::simulation) const fn output_revision(&self) -> ObjectRevision {
        self.output_revision
    }

    pub(in crate::simulation) const fn analysis_id(&self) -> AnalysisInstanceId {
        self.analysis_id
    }

    pub(in crate::simulation) const fn kind(&self) -> SavedOutputKind {
        self.kind
    }

    pub(in crate::simulation) fn name(&self) -> &str {
        &self.name
    }

    pub(in crate::simulation) fn source_expression(&self) -> &str {
        &self.source_expression
    }

    pub(in crate::simulation) const fn policy(&self) -> SavedOutputPolicy {
        self.policy
    }

    pub(in crate::simulation) const fn precision(&self) -> SavedOutputPrecision {
        self.precision
    }

    pub(in crate::simulation) const fn streaming(&self) -> SavedOutputStreaming {
        self.streaming
    }

    pub(in crate::simulation) const fn selection_grid(&self) -> Option<TransientSelectionGrid> {
        self.selection_grid
    }

    pub(in crate::simulation) const fn digest(&self) -> ContentDigest {
        self.digest
    }
}

pub(in crate::simulation) fn compile_saved_output_contracts<'a>(
    output: &SavedOutput,
    analyses: impl IntoIterator<Item = (AnalysisInstanceId, &'a AnalysisSpec)>,
) -> Result<Vec<PreparedSavedOutput>, String> {
    let mut contracts = Vec::new();
    for (analysis_id, spec) in analyses {
        if let Some(contract) = PreparedSavedOutput::prepare(output, analysis_id, spec)? {
            contracts.push(contract);
        }
    }
    if contracts.is_empty() {
        return Err(format!(
            "saved output '{}' has no compatible enabled analysis",
            output.name
        ));
    }
    Ok(contracts)
}

pub(in crate::simulation) fn preflight_saved_output<'a>(
    output: &SavedOutput,
    analyses: impl IntoIterator<Item = (AnalysisInstanceId, &'a AnalysisSpec)>,
) -> SavedOutputPreflightReport {
    let analyses = analyses.into_iter().collect::<Vec<_>>();
    let contracts = match compile_saved_output_contracts(output, analyses.iter().copied()) {
        Ok(contracts) => contracts,
        Err(reason) => return SavedOutputPreflightReport::invalid(reason),
    };

    let semantic_status = semantic_status(output, &contracts, &analyses);
    if let SavedOutputSemanticStatus::Invalid { reason } = &semantic_status {
        return SavedOutputPreflightReport::invalid(reason.clone());
    }
    let storage_estimate = storage_estimate(&contracts, &analyses);
    SavedOutputPreflightReport {
        semantic_status,
        storage_estimate,
        compatible_analysis_count: contracts.len(),
    }
}

pub(in crate::simulation) fn output_kind_supports_run_type(
    kind: SavedOutputKind,
    run_type: AnalysisRunType,
) -> bool {
    match kind {
        SavedOutputKind::RawVoltageOrCurrent | SavedOutputKind::DerivedExpression => matches!(
            run_type,
            AnalysisRunType::DcOp
                | AnalysisRunType::DcSweep
                | AnalysisRunType::Ac
                | AnalysisRunType::Transient
                | AnalysisRunType::Noise
                | AnalysisRunType::MonteCarlo
                | AnalysisRunType::Parametric
                | AnalysisRunType::Corner
                | AnalysisRunType::Reliability
                | AnalysisRunType::Optimization
                | AnalysisRunType::Soa
                | AnalysisRunType::SParameter
                | AnalysisRunType::Pac
                | AnalysisRunType::Pnoise
                | AnalysisRunType::Pxf
                | AnalysisRunType::Pss
                | AnalysisRunType::Qpss
                | AnalysisRunType::HarmonicBalance
                | AnalysisRunType::Envelope
                | AnalysisRunType::Fourier
                | AnalysisRunType::TransientNoise
        ),
        SavedOutputKind::DeviceOperatingPointQuantity => matches!(run_type, AnalysisRunType::DcOp),
        SavedOutputKind::NoiseContributor => matches!(
            run_type,
            AnalysisRunType::Noise
                | AnalysisRunType::Pnoise
                | AnalysisRunType::Qpnoise
                | AnalysisRunType::Hbnoise
                | AnalysisRunType::TransientNoise
        ),
        SavedOutputKind::RfPortQuantity => matches!(
            run_type,
            AnalysisRunType::SParameter | AnalysisRunType::Hbsp | AnalysisRunType::Psp
        ),
    }
}

fn validate_static_contract_semantics(
    output: &SavedOutput,
    spec: &AnalysisSpec,
) -> Result<(), String> {
    if output.kind != SavedOutputKind::RfPortQuantity {
        return Ok(());
    }
    let (output_port, input_port) = parse_rf_port(&output.source_expression)?;
    let port_count = match spec {
        AnalysisSpec::SParameter { ports, .. }
        | AnalysisSpec::Hbsp { ports, .. }
        | AnalysisSpec::Psp { ports, .. } => ports.len(),
        _ => {
            return Err(format!(
                "saved output '{}' requires an RF-port analysis",
                output.name
            ));
        }
    };
    if output_port > port_count || input_port > port_count {
        return Err(format!(
            "saved output '{}' references S({output_port},{input_port}), but {} has {port_count} configured port{}",
            output.name,
            spec.run_type().display_name(),
            if port_count == 1 { "" } else { "s" }
        ));
    }
    Ok(())
}

fn semantic_status(
    output: &SavedOutput,
    contracts: &[PreparedSavedOutput],
    analyses: &[(AnalysisInstanceId, &AnalysisSpec)],
) -> SavedOutputSemanticStatus {
    if output.kind == SavedOutputKind::RfPortQuantity {
        for contract in contracts {
            let Some((_, spec)) = analyses
                .iter()
                .find(|(analysis_id, _)| *analysis_id == contract.analysis_id)
            else {
                return SavedOutputSemanticStatus::Invalid {
                    reason: format!(
                        "prepared analysis {} is absent from the preflight input",
                        contract.analysis_id
                    ),
                };
            };
            if let Err(reason) = validate_static_contract_semantics(output, spec) {
                return SavedOutputSemanticStatus::Invalid { reason };
            }
        }
        return SavedOutputSemanticStatus::Valid {
            detail: "RF port indices resolve to configured ports in every compatible analysis"
                .to_owned(),
        };
    }

    let reason = match output.kind {
        SavedOutputKind::RawVoltageOrCurrent => {
            "probe grammar and analysis ownership are valid; node/branch existence is bound to the sealed executable netlist"
        }
        SavedOutputKind::DerivedExpression => {
            "expression grammar and analysis ownership are valid; referenced traces are bound to each retained solver result"
        }
        SavedOutputKind::DeviceOperatingPointQuantity => {
            "device-quantity grammar and DC operating-point ownership are valid; device existence is bound to the sealed executable netlist"
        }
        SavedOutputKind::NoiseContributor => {
            "noise contributor grammar and analysis ownership are valid; contributor existence is bound to the retained noise report"
        }
        SavedOutputKind::RfPortQuantity => unreachable!("handled above"),
    };
    SavedOutputSemanticStatus::RuntimeBound {
        reason: reason.to_owned(),
    }
}

fn storage_estimate(
    contracts: &[PreparedSavedOutput],
    analyses: &[(AnalysisInstanceId, &AnalysisSpec)],
) -> SavedOutputStorageEstimate {
    let mut total = 0_u64;
    for contract in contracts {
        if contract.policy == SavedOutputPolicy::OnDemandFromRetainedState {
            continue;
        }
        if contract.policy == SavedOutputPolicy::FailureDiagnosticsOnly {
            return SavedOutputStorageEstimate::Indeterminate {
                reason: format!(
                    "'{}' is retained only on failure, so its storage depends on the partial dataset available at the failure boundary",
                    contract.name
                ),
            };
        }
        let Some((_, spec)) = analyses
            .iter()
            .find(|(analysis_id, _)| *analysis_id == contract.analysis_id)
        else {
            return SavedOutputStorageEstimate::Indeterminate {
                reason: format!(
                    "prepared analysis {} is absent from the preflight input",
                    contract.analysis_id
                ),
            };
        };
        let sample_count = match deterministic_sample_count(contract, spec) {
            Ok(sample_count) => sample_count,
            Err(reason) => return SavedOutputStorageEstimate::Indeterminate { reason },
        };
        let source_values = if stores_complex_components(contract.kind, spec.run_type()) {
            4_u64
        } else {
            2_u64
        };
        let source_bytes = sample_count
            .checked_mul(source_values)
            .and_then(|values| values.checked_mul(std::mem::size_of::<f64>() as u64));
        let cache_bytes = if contract.precision
            == SavedOutputPrecision::DisplayCacheWithFullSourcePrecision
            || contract.streaming == SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
        {
            sample_count
                .min(DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES as u64)
                .checked_mul(2)
                .and_then(|values| values.checked_mul(std::mem::size_of::<f32>() as u64))
        } else {
            Some(0)
        };
        let Some(output_bytes) =
            source_bytes.and_then(|source| cache_bytes.and_then(|cache| source.checked_add(cache)))
        else {
            return SavedOutputStorageEstimate::Indeterminate {
                reason: "saved-output storage estimate exceeds the supported 64-bit byte range"
                    .to_owned(),
            };
        };
        let Some(next_total) = total.checked_add(output_bytes) else {
            return SavedOutputStorageEstimate::Indeterminate {
                reason: "aggregate saved-output storage estimate exceeds the supported 64-bit byte range"
                    .to_owned(),
            };
        };
        total = next_total;
    }
    SavedOutputStorageEstimate::ExactBytes(total)
}

fn deterministic_sample_count(
    contract: &PreparedSavedOutput,
    spec: &AnalysisSpec,
) -> Result<u64, String> {
    if contract.kind == SavedOutputKind::DeviceOperatingPointQuantity {
        return Ok(1);
    }
    if let Some(grid) = contract.selection_grid {
        let intervals = ((grid.stop - grid.start) / grid.step).ceil();
        if !intervals.is_finite() || intervals < 0.0 || intervals >= u64::MAX as f64 {
            return Err(format!(
                "'{}' selected-point grid exceeds the supported estimate range",
                contract.name
            ));
        }
        return Ok(intervals as u64 + 1);
    }
    let count = match spec {
        AnalysisSpec::DcOp => Some(1_usize),
        AnalysisSpec::DcSweep {
            start,
            stop,
            step,
            source2,
            start2,
            stop2,
            step2,
            ..
        } => {
            let primary = rspice_core::netlist::DcSweepSpec::linear(*start, *stop, *step)
                .points()
                .len();
            if source2.is_some() {
                match (start2, stop2, step2) {
                    (Some(start), Some(stop), Some(step)) => Some(
                        primary.saturating_mul(
                            rspice_core::netlist::DcSweepSpec::linear(*start, *stop, *step)
                                .points()
                                .len(),
                        ),
                    ),
                    _ => None,
                }
            } else {
                Some(primary)
            }
        }
        AnalysisSpec::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        }
        | AnalysisSpec::Disto {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ..
        }
        | AnalysisSpec::SParameter {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ..
        }
        | AnalysisSpec::Hbsp {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ..
        }
        | AnalysisSpec::Hbnoise {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ..
        }
        | AnalysisSpec::Psp {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ..
        }
        | AnalysisSpec::Qpac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ..
        }
        | AnalysisSpec::Qpnoise {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ..
        }
        | AnalysisSpec::Qpxf {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ..
        } => frequency_point_count(*start_freq, *stop_freq, *points_per_unit, *sweep),
        AnalysisSpec::AcData { frequencies, .. } => Some(frequencies.len()),
        AnalysisSpec::Noise {
            start_freq,
            stop_freq,
            points_per_decade,
            ..
        } => frequency_point_count(
            *start_freq,
            *stop_freq,
            *points_per_decade,
            FrequencySweep::Decade,
        ),
        AnalysisSpec::Reliability { target_years, .. } => Some(target_years.len()),
        _ => None,
    };
    count
        .and_then(|count| u64::try_from(count).ok())
        .ok_or_else(|| {
            format!(
                "'{}' uses {}, whose prepared point count is data-dependent or not bounded by its analysis specification",
                contract.name,
                spec.run_type().display_name()
            )
        })
}

fn frequency_point_count(
    start: f64,
    stop: f64,
    points_per_unit: usize,
    sweep: FrequencySweep,
) -> Option<usize> {
    if !start.is_finite()
        || !stop.is_finite()
        || start <= 0.0
        || stop < start
        || points_per_unit == 0
    {
        return None;
    }
    match sweep {
        FrequencySweep::Linear => Some(points_per_unit),
        FrequencySweep::Decade | FrequencySweep::Octave => {
            let units = if sweep == FrequencySweep::Decade {
                (stop / start).log10()
            } else {
                (stop / start).log2()
            };
            let requested = points_per_unit as f64 * units;
            (requested.is_finite() && requested <= usize::MAX as f64)
                .then_some((requested.round() as usize).max(2))
        }
    }
}

fn stores_complex_components(kind: SavedOutputKind, run_type: AnalysisRunType) -> bool {
    kind == SavedOutputKind::RfPortQuantity
        || kind == SavedOutputKind::RawVoltageOrCurrent
            && matches!(
                run_type,
                AnalysisRunType::Ac
                    | AnalysisRunType::Pac
                    | AnalysisRunType::Pxf
                    | AnalysisRunType::Pstb
                    | AnalysisRunType::Stb
                    | AnalysisRunType::SParameter
                    | AnalysisRunType::Hbsp
                    | AnalysisRunType::Psp
                    | AnalysisRunType::Qpac
                    | AnalysisRunType::Qpxf
            )
}

/// Apply immutable contracts to either a successful result or a failed result
/// carrying partial diagnostic waveforms. Existing engine outputs are retained
/// as the source state for deferred evaluation and exact downstream exports.
pub(in crate::simulation) fn materialize_saved_outputs(
    analysis: &mut AnalysisResult,
    contracts: &[PreparedSavedOutput],
) {
    if contracts.is_empty() {
        return;
    }
    let source_waveforms = analysis.waveforms.clone();
    let mut receipts = Vec::with_capacity(contracts.len());
    for contract in contracts {
        let status = if contract.policy == SavedOutputPolicy::OnDemandFromRetainedState {
            SavedOutputMaterializationStatus::Deferred
        } else if contract.policy == SavedOutputPolicy::FailureDiagnosticsOnly && analysis.success {
            SavedOutputMaterializationStatus::SuppressedOnSuccess
        } else {
            match resolve_contract_waveform(contract, analysis, &source_waveforms) {
                Ok(mut waveform) => {
                    if contract.policy == SavedOutputPolicy::SelectedAndFinalPoints
                        && let Some(grid) = contract.selection_grid
                    {
                        match resample_selected_and_final(&waveform, grid) {
                            Ok(selected) => waveform = selected,
                            Err(error) => {
                                receipts.push(receipt(
                                    contract,
                                    SavedOutputMaterializationStatus::Unavailable { reason: error },
                                ));
                                continue;
                            }
                        }
                    }
                    if contract.precision
                        == SavedOutputPrecision::DisplayCacheWithFullSourcePrecision
                        || contract.streaming
                            == SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
                    {
                        waveform.rebuild_display_cache(DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES);
                    }
                    let sample_count = u64::try_from(waveform.x.len()).unwrap_or(u64::MAX);
                    let waveform_name = waveform.name.clone();
                    if let Some(existing) = analysis
                        .waveforms
                        .iter_mut()
                        .find(|existing| existing.name == waveform_name)
                    {
                        if existing.x != waveform.x
                            || existing.y != waveform.y
                            || existing.complex != waveform.complex
                        {
                            receipts.push(receipt(
                                contract,
                                SavedOutputMaterializationStatus::Unavailable {
                                    reason: format!(
                                        "saved-output name '{}' collides with a different retained waveform",
                                        waveform_name
                                    ),
                                },
                            ));
                            continue;
                        }
                        existing.display_cache = waveform.display_cache;
                    } else {
                        analysis.waveforms.push(waveform);
                    }
                    SavedOutputMaterializationStatus::Materialized {
                        waveform_name,
                        sample_count,
                    }
                }
                Err(reason) => SavedOutputMaterializationStatus::Unavailable { reason },
            }
        };
        receipts.push(receipt(contract, status));
    }
    analysis.saved_output_receipts.extend(receipts);
}

/// Materialize one deferred receipt against its retained source analysis.
/// The receipt's immutable digest and source text are reused; live project
/// rows are never consulted.
pub fn materialize_deferred_saved_output(
    analysis: &mut AnalysisResult,
    receipt_index: usize,
) -> Result<(), String> {
    let receipt = analysis
        .saved_output_receipts
        .get(receipt_index)
        .cloned()
        .ok_or_else(|| "saved-output receipt no longer exists".to_owned())?;
    if receipt.status != SavedOutputMaterializationStatus::Deferred {
        return Err("saved-output receipt is not deferred".to_owned());
    }
    let contract = PreparedSavedOutput {
        output_id: receipt.output_id,
        output_revision: receipt.output_revision,
        analysis_id: receipt.analysis_id,
        kind: receipt.output_kind,
        name: receipt.name.clone(),
        source_expression: receipt.source_expression.clone(),
        policy: receipt.save_policy,
        precision: receipt.stored_precision,
        streaming: receipt.streaming,
        selection_grid: None,
        digest: receipt.contract_digest,
    };
    let source_waveforms = analysis.waveforms.clone();
    let mut waveform = resolve_contract_waveform(&contract, analysis, &source_waveforms)?;
    if contract.precision == SavedOutputPrecision::DisplayCacheWithFullSourcePrecision
        || contract.streaming == SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
    {
        waveform.rebuild_display_cache(DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES);
    }
    let sample_count = u64::try_from(waveform.x.len()).unwrap_or(u64::MAX);
    let waveform_name = waveform.name.clone();
    analysis.waveforms.push(waveform);
    analysis.saved_output_receipts[receipt_index].status =
        SavedOutputMaterializationStatus::Materialized {
            waveform_name,
            sample_count,
        };
    Ok(())
}

fn receipt(
    contract: &PreparedSavedOutput,
    status: SavedOutputMaterializationStatus,
) -> SavedOutputReceipt {
    SavedOutputReceipt {
        output_id: contract.output_id,
        output_revision: contract.output_revision,
        analysis_id: contract.analysis_id,
        contract_digest: contract.digest,
        name: contract.name.clone(),
        source_expression: contract.source_expression.clone(),
        output_kind: contract.kind,
        save_policy: contract.policy,
        stored_precision: contract.precision,
        streaming: contract.streaming,
        status,
    }
}

fn resolve_contract_waveform(
    contract: &PreparedSavedOutput,
    analysis: &AnalysisResult,
    waveforms: &[WaveformData],
) -> Result<WaveformData, String> {
    match contract.kind {
        SavedOutputKind::RawVoltageOrCurrent => {
            resolve_raw_probe(&contract.source_expression, waveforms, &contract.name)
        }
        SavedOutputKind::DerivedExpression => {
            resolve_derived_expression(&contract.source_expression, waveforms, &contract.name)
        }
        SavedOutputKind::DeviceOperatingPointQuantity => {
            resolve_device_quantity(&contract.source_expression, analysis, &contract.name)
        }
        SavedOutputKind::NoiseContributor => {
            let source = format!("noise({})", contract.source_expression.trim());
            clone_named_waveform(waveforms, &source, &contract.name).or_else(|_| {
                clone_named_waveform(waveforms, &contract.source_expression, &contract.name)
            })
        }
        SavedOutputKind::RfPortQuantity => {
            let (output, input) = parse_rf_port(&contract.source_expression)?;
            let compact = format!("S{output}{input}");
            clone_named_waveform(waveforms, &compact, &contract.name).or_else(|_| {
                clone_named_waveform(waveforms, &contract.source_expression, &contract.name)
            })
        }
    }
}

fn resolve_raw_probe(
    expression: &str,
    waveforms: &[WaveformData],
    output_name: &str,
) -> Result<WaveformData, String> {
    let (function, arguments) = parse_probe(expression)?;
    if function.eq_ignore_ascii_case("V") && arguments.len() == 2 {
        let positive = find_waveform(waveforms, &format!("V({})", arguments[0]))
            .or_else(|| find_waveform(waveforms, &arguments[0]))
            .ok_or_else(|| format!("positive probe '{}' is absent", arguments[0]))?;
        let negative = find_waveform(waveforms, &format!("V({})", arguments[1]))
            .or_else(|| find_waveform(waveforms, &arguments[1]))
            .ok_or_else(|| format!("negative probe '{}' is absent", arguments[1]))?;
        return subtract_waveforms(positive, negative, output_name);
    }
    let source = find_waveform(waveforms, expression)
        .or_else(|| {
            arguments
                .first()
                .and_then(|argument| find_waveform(waveforms, argument))
        })
        .ok_or_else(|| format!("source probe '{expression}' is absent"))?;
    Ok(clone_with_name(source, output_name))
}

fn resolve_derived_expression(
    expression: &str,
    waveforms: &[WaveformData],
    output_name: &str,
) -> Result<WaveformData, String> {
    let parsed = calculator::parser::Parser::new(expression)
        .try_parse()
        .map_err(|error| format!("expression parse failed: {error}"))?;
    let context = calculator::WaveformsContext::new(waveforms);
    match calculator::evaluator::evaluate(&parsed, &context)
        .map_err(|error| format!("expression evaluation failed: {error}"))?
    {
        CalcValue::Waveform(x, y) if !x.is_empty() && x.len() == y.len() => {
            Ok(WaveformData::new(output_name, x, y, "#f5b700"))
        }
        CalcValue::Waveform(..) => Err("expression produced no aligned samples".to_owned()),
        CalcValue::Scalar(value) => {
            let source = waveforms
                .first()
                .ok_or_else(|| "scalar expression has no retained axis".to_owned())?;
            Ok(WaveformData::new(
                output_name,
                Arc::clone(&source.x),
                vec![value; source.x.len()],
                "#f5b700",
            ))
        }
    }
}

fn resolve_device_quantity(
    expression: &str,
    analysis: &AnalysisResult,
    output_name: &str,
) -> Result<WaveformData, String> {
    let body = expression
        .trim()
        .strip_prefix('@')
        .ok_or_else(|| "device quantity must begin with '@'".to_owned())?;
    let open = body
        .find('[')
        .ok_or_else(|| "device quantity is missing '['".to_owned())?;
    let device = &body[..open];
    let quantity = body[open + 1..]
        .strip_suffix(']')
        .ok_or_else(|| "device quantity is missing ']'".to_owned())?;
    let report = analysis
        .device_op
        .as_ref()
        .ok_or_else(|| "analysis retained no device operating-point report".to_owned())?;
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case(device))
        .ok_or_else(|| format!("device '{device}' is absent from the operating-point report"))?;
    let value = entry
        .params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(quantity))
        .map(|(_, value)| *value)
        .ok_or_else(|| format!("device '{device}' has no '{quantity}' quantity"))?;
    Ok(WaveformData::new(
        output_name,
        vec![0.0],
        vec![value],
        "#f5b700",
    ))
}

fn parse_probe(expression: &str) -> Result<(String, Vec<String>), String> {
    let expression = expression.trim();
    let open = expression
        .find('(')
        .ok_or_else(|| "probe is missing '('".to_owned())?;
    let inner = expression[open + 1..]
        .strip_suffix(')')
        .ok_or_else(|| "probe is missing ')'".to_owned())?;
    Ok((
        expression[..open].to_owned(),
        inner
            .split(',')
            .map(|value| value.trim().to_owned())
            .collect(),
    ))
}

fn parse_rf_port(expression: &str) -> Result<(usize, usize), String> {
    let (function, arguments) = parse_probe(expression)?;
    if !function.eq_ignore_ascii_case("S") || arguments.len() != 2 {
        return Err("RF quantity must use S(output, input)".to_owned());
    }
    let output = arguments[0]
        .parse::<usize>()
        .map_err(|_| "RF output port must be a positive integer".to_owned())?;
    let input = arguments[1]
        .parse::<usize>()
        .map_err(|_| "RF input port must be a positive integer".to_owned())?;
    if output == 0 || input == 0 {
        return Err("RF ports are one-based".to_owned());
    }
    Ok((output, input))
}

fn clone_named_waveform(
    waveforms: &[WaveformData],
    source: &str,
    output_name: &str,
) -> Result<WaveformData, String> {
    find_waveform(waveforms, source)
        .map(|waveform| clone_with_name(waveform, output_name))
        .ok_or_else(|| format!("source waveform '{source}' is absent"))
}

fn find_waveform<'a>(waveforms: &'a [WaveformData], requested: &str) -> Option<&'a WaveformData> {
    let requested = requested.trim();
    waveforms.iter().find(|waveform| {
        if waveform.name.eq_ignore_ascii_case(requested)
            || waveform
                .name
                .trim_matches('|')
                .eq_ignore_ascii_case(requested)
        {
            return true;
        }
        let source_name = waveform
            .complex
            .as_ref()
            .map(|complex| complex.source_name.as_str());
        if source_name.is_some_and(|source| source.eq_ignore_ascii_case(requested)) {
            return true;
        }
        let waveform_inner = waveform
            .name
            .trim_matches('|')
            .strip_prefix("V(")
            .or_else(|| waveform.name.trim_matches('|').strip_prefix("I("))
            .and_then(|value| value.strip_suffix(')'))
            .unwrap_or(waveform.name.trim_matches('|'));
        let requested_inner = requested
            .strip_prefix("V(")
            .or_else(|| requested.strip_prefix("I("))
            .and_then(|value| value.strip_suffix(')'))
            .unwrap_or(requested);
        waveform_inner.eq_ignore_ascii_case(requested_inner)
    })
}

fn clone_with_name(source: &WaveformData, name: &str) -> WaveformData {
    let mut waveform = source.clone();
    waveform.name = name.to_owned();
    waveform.display_cache = None;
    waveform
}

fn subtract_waveforms(
    positive: &WaveformData,
    negative: &WaveformData,
    name: &str,
) -> Result<WaveformData, String> {
    if positive.x.as_ref() != negative.x.as_ref() || positive.y.len() != negative.y.len() {
        return Err("differential probe sources do not share an exact axis".to_owned());
    }
    let y = positive
        .y
        .iter()
        .zip(negative.y.iter())
        .map(|(positive, negative)| positive - negative)
        .collect::<Vec<_>>();
    let mut result = WaveformData::new(name, Arc::clone(&positive.x), y, "#f5b700");
    if let (Some(positive), Some(negative)) = (&positive.complex, &negative.complex)
        && positive.real.len() == negative.real.len()
        && positive.imag.len() == negative.imag.len()
    {
        result = result.with_complex_components(
            name,
            positive
                .real
                .iter()
                .zip(negative.real.iter())
                .map(|(positive, negative)| positive - negative)
                .collect::<Vec<_>>(),
            positive
                .imag
                .iter()
                .zip(negative.imag.iter())
                .map(|(positive, negative)| positive - negative)
                .collect::<Vec<_>>(),
        );
    }
    Ok(result)
}

fn validate_selection_grid(grid: TransientSelectionGrid) -> Result<(), String> {
    if !grid.start.is_finite()
        || !grid.step.is_finite()
        || !grid.stop.is_finite()
        || grid.start < 0.0
        || grid.step <= 0.0
        || grid.stop < grid.start
    {
        return Err("selected-point transient grid is invalid".to_owned());
    }
    let count = ((grid.stop - grid.start) / grid.step).floor() + 2.0;
    if !count.is_finite() || count > MAX_SELECTED_POINT_COUNT as f64 {
        return Err(format!(
            "selected-point grid exceeds the {MAX_SELECTED_POINT_COUNT}-sample safety limit"
        ));
    }
    Ok(())
}

fn resample_selected_and_final(
    waveform: &WaveformData,
    grid: TransientSelectionGrid,
) -> Result<WaveformData, String> {
    validate_selection_grid(grid)?;
    if waveform.x.is_empty() || waveform.x.len() != waveform.y.len() {
        return Err("source waveform has no aligned samples".to_owned());
    }
    if waveform
        .x
        .windows(2)
        .any(|window| !window[0].is_finite() || window[1] <= window[0])
    {
        return Err("source waveform axis is not strictly increasing".to_owned());
    }
    let first = waveform.x[0];
    let last = *waveform.x.last().expect("non-empty checked");
    let start = grid.start.max(first);
    let stop = grid.stop.min(last);
    if stop < start {
        return Err("selected-point grid does not overlap the source axis".to_owned());
    }
    let mut x = Vec::new();
    let mut cursor = start;
    while cursor < stop {
        x.push(cursor);
        cursor = start + grid.step * x.len() as f64;
    }
    if x.last()
        .is_none_or(|value| value.to_bits() != stop.to_bits())
    {
        x.push(stop);
    }
    let y = x
        .iter()
        .map(|point| interpolate(&waveform.x, &waveform.y, *point))
        .collect::<Result<Vec<_>, _>>()?;
    let mut result = WaveformData::new(&waveform.name, x.clone(), y, waveform.color.clone());
    result.visible = waveform.visible;
    if let Some(complex) = &waveform.complex {
        let real = x
            .iter()
            .map(|point| interpolate(&waveform.x, &complex.real, *point))
            .collect::<Result<Vec<_>, _>>()?;
        let imag = x
            .iter()
            .map(|point| interpolate(&waveform.x, &complex.imag, *point))
            .collect::<Result<Vec<_>, _>>()?;
        result = result.with_complex_components(&complex.source_name, real, imag);
    }
    Ok(result)
}

fn interpolate(axis: &[f64], values: &[f64], point: f64) -> Result<f64, String> {
    if axis.len() != values.len() || axis.is_empty() {
        return Err("interpolation source is unaligned".to_owned());
    }
    match axis.binary_search_by(|candidate| candidate.total_cmp(&point)) {
        Ok(index) => Ok(values[index]),
        Err(0) => Ok(values[0]),
        Err(index) if index >= axis.len() => Ok(values[values.len() - 1]),
        Err(index) => {
            let left = index - 1;
            let scale = (point - axis[left]) / (axis[index] - axis[left]);
            Ok(values[left] + scale * (values[index] - values[left]))
        }
    }
}

fn output_contract_digest(
    output: &SavedOutput,
    analysis_id: AnalysisInstanceId,
    spec: &AnalysisSpec,
    grid: Option<TransientSelectionGrid>,
) -> ContentDigest {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(output.id.as_uuid().as_bytes());
    bytes.extend_from_slice(&output.revision.get().to_be_bytes());
    bytes.extend_from_slice(analysis_id.as_uuid().as_bytes());
    bytes.push(analysis_kind_tag(spec));
    bytes.push(output_kind_tag(output.kind));
    append_string(&mut bytes, &output.name);
    append_string(&mut bytes, &output.source_expression);
    bytes.push(policy_tag(output.save_policy));
    bytes.push(precision_tag(output.stored_precision));
    bytes.push(streaming_tag(output.streaming));
    if let Some(grid) = grid {
        bytes.push(1);
        bytes.extend_from_slice(&grid.start.to_bits().to_be_bytes());
        bytes.extend_from_slice(&grid.step.to_bits().to_be_bytes());
        bytes.extend_from_slice(&grid.stop.to_bits().to_be_bytes());
    } else {
        bytes.push(0);
    }
    content_digest("rspice.prepared-saved-output/v1", &bytes)
}

fn append_string(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value.as_bytes());
}

pub(in crate::simulation) const fn output_kind_tag(kind: SavedOutputKind) -> u8 {
    match kind {
        SavedOutputKind::RawVoltageOrCurrent => 0,
        SavedOutputKind::DerivedExpression => 1,
        SavedOutputKind::DeviceOperatingPointQuantity => 2,
        SavedOutputKind::NoiseContributor => 3,
        SavedOutputKind::RfPortQuantity => 4,
    }
}

pub(in crate::simulation) const fn policy_tag(policy: SavedOutputPolicy) -> u8 {
    match policy {
        SavedOutputPolicy::EveryAcceptedPoint => 0,
        SavedOutputPolicy::SelectedAndFinalPoints => 1,
        SavedOutputPolicy::OnDemandFromRetainedState => 2,
        SavedOutputPolicy::FailureDiagnosticsOnly => 3,
    }
}

pub(in crate::simulation) const fn precision_tag(precision: SavedOutputPrecision) -> u8 {
    match precision {
        SavedOutputPrecision::FullSourcePrecision => 0,
        SavedOutputPrecision::DisplayCacheWithFullSourcePrecision => 1,
    }
}

pub(in crate::simulation) const fn streaming_tag(streaming: SavedOutputStreaming) -> u8 {
    match streaming {
        SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation => 0,
        SavedOutputStreaming::StoreOnly => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AnalysisType;

    fn output(policy: SavedOutputPolicy, precision: SavedOutputPrecision) -> SavedOutput {
        SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            "output_voltage",
            "V(out)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            policy,
            precision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("valid output")
    }

    fn transient_spec() -> AnalysisSpec {
        AnalysisSpec::Transient {
            stop_time: 1.0,
            step_time: 0.25,
            start_time: 0.0,
            max_timestep: Some(0.1),
            uic: false,
        }
    }

    #[test]
    fn every_accepted_materializes_exact_source_and_receipt() {
        let contract = PreparedSavedOutput::prepare(
            &output(
                SavedOutputPolicy::EveryAcceptedPoint,
                SavedOutputPrecision::FullSourcePrecision,
            ),
            AnalysisInstanceId::new(),
            &transient_spec(),
        )
        .expect("prepare")
        .expect("applies");
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "out",
                    vec![0.0, 0.1, 0.4, 1.0],
                    vec![0.0, 1.0, 4.0, 10.0],
                    "#fff",
                ),
            ]);
        materialize_saved_outputs(&mut analysis, &[contract]);
        assert_eq!(analysis.waveforms[1].name, "output_voltage");
        assert_eq!(
            analysis.waveforms[1].x.as_ref(),
            analysis.waveforms[0].x.as_ref()
        );
        assert!(matches!(
            analysis.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Materialized {
                sample_count: 4,
                ..
            }
        ));
    }

    #[test]
    fn selected_and_final_uses_configured_grid_and_exact_final() {
        let contract = PreparedSavedOutput::prepare(
            &output(
                SavedOutputPolicy::SelectedAndFinalPoints,
                SavedOutputPrecision::FullSourcePrecision,
            ),
            AnalysisInstanceId::new(),
            &transient_spec(),
        )
        .expect("prepare")
        .expect("applies");
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "out",
                    vec![0.0, 0.1, 0.4, 0.8, 1.0],
                    vec![0.0, 1.0, 4.0, 8.0, 10.0],
                    "#fff",
                ),
            ]);
        materialize_saved_outputs(&mut analysis, &[contract]);
        assert_eq!(
            analysis.waveforms[1].x.as_ref(),
            &[0.0, 0.25, 0.5, 0.75, 1.0]
        );
        assert_eq!(
            analysis.waveforms[1].y.as_ref(),
            &[0.0, 2.5, 5.0, 7.5, 10.0]
        );
    }

    #[test]
    fn on_demand_is_deferred_then_materializes_from_retained_source() {
        let contract = PreparedSavedOutput::prepare(
            &output(
                SavedOutputPolicy::OnDemandFromRetainedState,
                SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
            ),
            AnalysisInstanceId::new(),
            &transient_spec(),
        )
        .expect("prepare")
        .expect("applies");
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
            ]);
        materialize_saved_outputs(&mut analysis, &[contract]);
        assert_eq!(analysis.waveforms.len(), 1);
        assert_eq!(
            analysis.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Deferred
        );
        materialize_deferred_saved_output(&mut analysis, 0).expect("deferred materializes");
        assert_eq!(analysis.waveforms.len(), 2);
        assert!(analysis.waveforms[1].display_cache.is_some());
    }

    #[test]
    fn failure_only_is_suppressed_on_success_and_materializes_partial_failure_data() {
        let contract = PreparedSavedOutput::prepare(
            &output(
                SavedOutputPolicy::FailureDiagnosticsOnly,
                SavedOutputPrecision::FullSourcePrecision,
            ),
            AnalysisInstanceId::new(),
            &transient_spec(),
        )
        .expect("prepare")
        .expect("applies");
        let source = WaveformData::new("out", vec![0.0, 0.1], vec![0.0, 1.0], "#fff");
        let mut success = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![source.clone()]);
        materialize_saved_outputs(&mut success, std::slice::from_ref(&contract));
        assert_eq!(
            success.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::SuppressedOnSuccess
        );

        let mut failed = AnalysisResult::failed(1, AnalysisType::Transient, "TRAN", "failed")
            .with_waveforms(vec![source]);
        materialize_saved_outputs(&mut failed, &[contract]);
        assert!(matches!(
            failed.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Materialized {
                sample_count: 2,
                ..
            }
        ));
    }

    #[test]
    fn preflight_exactly_estimates_fixed_ac_storage() {
        let output = output(
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
        );
        let analysis_id = AnalysisInstanceId::new();
        let spec = AnalysisSpec::Ac {
            start_freq: 1.0,
            stop_freq: 1_000.0,
            points_per_unit: 10,
            sweep: FrequencySweep::Decade,
        };
        let report = preflight_saved_output(&output, [(analysis_id, &spec)]);
        assert_eq!(report.compatible_analysis_count(), 1);
        assert_eq!(
            report.storage_estimate(),
            &SavedOutputStorageEstimate::ExactBytes(30 * 4 * 8)
        );
        assert!(matches!(
            report.semantic_status(),
            SavedOutputSemanticStatus::RuntimeBound { .. }
        ));
    }

    #[test]
    fn preflight_marks_adaptive_transient_capture_indeterminate() {
        let output = output(
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
        );
        let spec = transient_spec();
        let report = preflight_saved_output(&output, [(AnalysisInstanceId::new(), &spec)]);
        assert!(matches!(
            report.storage_estimate(),
            SavedOutputStorageEstimate::Indeterminate { reason }
                if reason.contains("data-dependent")
        ));
    }

    #[test]
    fn preflight_rejects_rf_port_outside_prepared_port_set() {
        let output = SavedOutput::new(
            SavedOutputKind::RfPortQuantity,
            "forward_gain",
            "S(3,1)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("syntactically valid output");
        let spec = AnalysisSpec::SParameter {
            start_freq: 1.0e6,
            stop_freq: 1.0e9,
            points_per_unit: 10,
            sweep: FrequencySweep::Decade,
            z0: 50.0,
            ports: vec![
                crate::simulation::multi_run::SpPort {
                    node_pos: "in".to_owned(),
                    node_neg: "0".to_owned(),
                    z0: None,
                },
                crate::simulation::multi_run::SpPort {
                    node_pos: "out".to_owned(),
                    node_neg: "0".to_owned(),
                    z0: None,
                },
            ],
        };
        let report = preflight_saved_output(&output, [(AnalysisInstanceId::new(), &spec)]);
        assert!(matches!(
            report.semantic_status(),
            SavedOutputSemanticStatus::Invalid { reason } if reason.contains("2 configured ports")
        ));
    }
}
