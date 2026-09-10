//! Immutable saved-output preparation and result materialization.
//!
//! Project rows are compiled against the exact prepared analysis identity.
//! Dispatch therefore carries no reference to mutable workspace state, and
//! result receipts authenticate the contract that actually produced data.

use std::collections::HashSet;
use std::sync::Arc;

use crate::analysis::calculator::{self, CalcValue};
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SavedOutputId};
use crate::simulation::config::NoiseSweepType;
use crate::simulation::execution::{analysis_kind_tag, content_digest};
use crate::simulation::multi_run::{AnalysisRunType, AnalysisSpec, FrequencySweep};
use crate::state::{
    AnalysisResult, DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, SavedOutput, SavedOutputCompatibility,
    SavedOutputKind, SavedOutputMaterializationStatus, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputReceipt, SavedOutputStreaming, WaveformData,
};

const MAX_SELECTED_POINT_COUNT: usize = 10_000_000;

mod bindings;
mod dc_family;
mod materialize;
mod probe;
pub(crate) use materialize::materialize_deferred_saved_output;
#[cfg(test)]
use materialize::materialize_saved_outputs;
pub(in crate::simulation) use materialize::{
    apply_saved_output_policy, materialize_live_saved_outputs, retain_plan_saved_outputs,
};
use probe::resolve_raw_probe;

#[cfg(test)]
mod binding_tests;
#[cfg(test)]
mod complex_tests;
#[cfg(test)]
mod dc_family_tests;
#[cfg(test)]
mod durable_binding_tests;
#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod probe_tests;

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
    /// The same bound, divided between the analyses that produce it.
    ///
    /// A caller pricing this output over a run set needs the parts: two
    /// analyses producing one output do not have to run at the same number of
    /// points, and multiplying the sum by the whole matrix prices a
    /// nominal-only analysis as if it crossed every corner.
    bytes_by_analysis: Vec<(AnalysisInstanceId, u64)>,
    compatible_analysis_count: usize,
    retained_engine_source_analysis_ids: Vec<AnalysisInstanceId>,
}

impl SavedOutputPreflightReport {
    pub(crate) fn invalid(reason: impl Into<String>) -> Self {
        let reason = reason.into();
        Self {
            semantic_status: SavedOutputSemanticStatus::Invalid {
                reason: reason.clone(),
            },
            storage_estimate: SavedOutputStorageEstimate::Indeterminate { reason },
            bytes_by_analysis: Vec::new(),
            compatible_analysis_count: 0,
            retained_engine_source_analysis_ids: Vec::new(),
        }
    }

    /// A report that bounds one output at exactly `bytes`, for tests that
    /// exercise arithmetic over reports rather than the preflight that
    /// produces them.
    #[cfg(test)]
    pub(crate) fn exact_for_test(bytes: u64) -> Self {
        Self {
            semantic_status: SavedOutputSemanticStatus::Valid {
                detail: "test".to_owned(),
            },
            storage_estimate: SavedOutputStorageEstimate::ExactBytes(bytes),
            // No analysis to attribute it to, so a ledger prices it at the
            // caller's default participation. That is what these fixtures
            // mean: one output, one bound, over the whole declared space.
            bytes_by_analysis: Vec::new(),
            compatible_analysis_count: 1,
            retained_engine_source_analysis_ids: Vec::new(),
        }
    }

    /// [`Self::exact_for_test`] with the bound attributed to one analysis, for
    /// a test that prices the same report over different participations.
    #[cfg(test)]
    pub(crate) fn exact_for_analysis_test(analysis: AnalysisInstanceId, bytes: u64) -> Self {
        Self {
            bytes_by_analysis: vec![(analysis, bytes)],
            ..Self::exact_for_test(bytes)
        }
    }

    pub const fn semantic_status(&self) -> &SavedOutputSemanticStatus {
        &self.semantic_status
    }

    pub const fn storage_estimate(&self) -> &SavedOutputStorageEstimate {
        &self.storage_estimate
    }

    /// [`Self::storage_estimate`], divided between the analyses producing it.
    ///
    /// Empty when the estimate is indeterminate, and empty for a fixture
    /// report that names no analysis. The entries always sum to the scalar.
    pub fn bytes_by_analysis(&self) -> &[(AnalysisInstanceId, u64)] {
        &self.bytes_by_analysis
    }

    pub const fn compatible_analysis_count(&self) -> usize {
        self.compatible_analysis_count
    }

    /// Prepared analyses whose complete engine waveform state must survive so
    /// this deferred output can be evaluated exactly after the run.
    pub fn retained_engine_source_analysis_ids(&self) -> &[AnalysisInstanceId] {
        &self.retained_engine_source_analysis_ids
    }
}

/// Conservative logical-byte ceiling for complete engine waveform source
/// state retained by deferred outputs.
///
/// The core resource contract bounds scalar result values for one analysis.
/// Counting every permitted value as f64 is the stable cross-platform upper
/// bound consumed by preflight. Several deferred outputs attached to the same
/// prepared analysis share this state and must count it exactly once.
pub(crate) fn retained_engine_source_upper_bound_bytes(analysis_count: usize) -> u64 {
    let values =
        u64::try_from(rspice_core::ResourceLimits::default().max_result_values).unwrap_or(u64::MAX);
    let per_analysis = values.saturating_mul(std::mem::size_of::<f64>() as u64);
    per_analysis.saturating_mul(u64::try_from(analysis_count).unwrap_or(u64::MAX))
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
    complex_policy: crate::state::ComplexExpressionPolicy,
    policy: SavedOutputPolicy,
    precision: SavedOutputPrecision,
    streaming: SavedOutputStreaming,
    display_intent: crate::state::SavedOutputDisplayIntent,
    selection_grid: Option<TransientSelectionGrid>,
    candidates: Option<Arc<bindings::Candidates>>,
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
            complex_policy: output.complex_policy,
            policy: output.save_policy,
            precision: output.stored_precision,
            streaming: output.streaming,
            display_intent: output.display_intent,
            selection_grid,
            candidates: None,
            digest,
        }))
    }

    /// Recompile this immutable contract for a deterministically derived
    /// analysis identity. PVT expansion happens after plan outputs have been
    /// prepared, so copying the old digest or identity would make the
    /// expanded task fail authentication (or, worse, retain data under the
    /// wrong analysis). The reconstructed authored contract deliberately uses
    /// `AllCompatibleAnalyses`: selection was already proven when the original
    /// prepared contract was created, and every derived task has the same OP
    /// run type.
    pub(in crate::simulation) fn rebind_analysis(
        &self,
        analysis_id: AnalysisInstanceId,
        spec: &AnalysisSpec,
    ) -> Result<Self, String> {
        let output = SavedOutput {
            id: self.output_id,
            revision: self.output_revision,
            origin: crate::state::SavedOutputOrigin::Plan,
            display_intent: self.display_intent,
            kind: self.kind,
            name: self.name.clone(),
            source_expression: self.source_expression.clone(),
            complex_policy: self.complex_policy,
            compatible_analyses: SavedOutputCompatibility::AllCompatibleAnalyses,
            save_policy: self.policy,
            stored_precision: self.precision,
            streaming: self.streaming,
        };
        Self::prepare(&output, analysis_id, spec)?.ok_or_else(|| {
            format!(
                "saved output '{}' is incompatible with derived analysis {analysis_id}",
                self.name
            )
        })
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
    let (storage_estimate, bytes_by_analysis) = storage_estimate(&contracts, &analyses);
    let retained_engine_source_analysis_ids =
        if output.save_policy == SavedOutputPolicy::OnDemandFromRetainedState {
            contracts
                .iter()
                .map(PreparedSavedOutput::analysis_id)
                .collect()
        } else {
            Vec::new()
        };
    SavedOutputPreflightReport {
        semantic_status,
        storage_estimate,
        bytes_by_analysis,
        compatible_analysis_count: contracts.len(),
        retained_engine_source_analysis_ids,
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
        // SP's configured ports are a fallback. Authored hierarchical ports
        // may replace them, so only the solved circuit can establish this bound.
        AnalysisSpec::SParameter { .. } => return Ok(()),
        AnalysisSpec::Hbsp { ports, .. } | AnalysisSpec::Psp { ports, .. } => ports.len(),
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
        let mut requires_elaboration = false;
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
            requires_elaboration |= matches!(spec, AnalysisSpec::SParameter { .. });
        }
        if requires_elaboration {
            return SavedOutputSemanticStatus::RuntimeBound {
                reason: "RF port indices are bound to the elaborated circuit and its retained scattering traces".to_owned(),
            };
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

/// One output's bounded cost, and how it divides between the analyses that
/// produce it.
///
/// The scalar is the sum of the parts, so a reader can keep using it — but a
/// caller that prices the output over a run set needs the parts, because the
/// analyses producing it do not all run at the same number of points.
fn storage_estimate(
    contracts: &[PreparedSavedOutput],
    analyses: &[(AnalysisInstanceId, &AnalysisSpec)],
) -> (SavedOutputStorageEstimate, Vec<(AnalysisInstanceId, u64)>) {
    let mut by_analysis: Vec<(AnalysisInstanceId, u64)> = Vec::new();
    let mut total = 0_u64;
    for contract in contracts {
        if contract.policy == SavedOutputPolicy::OnDemandFromRetainedState {
            continue;
        }
        if contract.policy == SavedOutputPolicy::FailureDiagnosticsOnly {
            return (
                indeterminate(format!(
                    "'{}' is retained only on failure, so its storage depends on the partial dataset available at the failure boundary",
                    contract.name
                )),
                Vec::new(),
            );
        }
        let Some((_, spec)) = analyses
            .iter()
            .find(|(analysis_id, _)| *analysis_id == contract.analysis_id)
        else {
            return (
                indeterminate(format!(
                    "prepared analysis {} is absent from the preflight input",
                    contract.analysis_id
                )),
                Vec::new(),
            );
        };
        let sample_count = match deterministic_sample_count(contract, spec) {
            Ok(sample_count) => sample_count,
            Err(reason) => return (indeterminate(reason), Vec::new()),
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
            return (
                indeterminate(
                    "saved-output storage estimate exceeds the supported 64-bit byte range",
                ),
                Vec::new(),
            );
        };
        let Some(next_total) = total.checked_add(output_bytes) else {
            return (
                indeterminate(
                    "aggregate saved-output storage estimate exceeds the supported 64-bit byte range",
                ),
                Vec::new(),
            );
        };
        total = next_total;
        match by_analysis
            .iter_mut()
            .find(|(analysis_id, _)| *analysis_id == contract.analysis_id)
        {
            // One analysis can produce several contracts for one output, and
            // it runs all of them at the same points, so they fold into one
            // line rather than becoming two entries priced separately.
            Some((_, bytes)) => *bytes = bytes.saturating_add(output_bytes),
            None => by_analysis.push((contract.analysis_id, output_bytes)),
        }
    }
    (SavedOutputStorageEstimate::ExactBytes(total), by_analysis)
}

fn indeterminate(reason: impl Into<String>) -> SavedOutputStorageEstimate {
    SavedOutputStorageEstimate::Indeterminate {
        reason: reason.into(),
    }
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
        AnalysisSpec::DcOp { .. } => Some(1_usize),
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
            sweep,
            explicit_frequencies,
            ..
        } => explicit_frequencies.as_ref().map(Vec::len).or_else(|| {
            let sweep = match sweep {
                NoiseSweepType::Decade => FrequencySweep::Decade,
                NoiseSweepType::Octave => FrequencySweep::Octave,
                NoiseSweepType::Linear => FrequencySweep::Linear,
                NoiseSweepType::ExplicitFrequencyList | NoiseSweepType::Unsupported(_) => {
                    return None;
                }
            };
            frequency_point_count(*start_freq, *stop_freq, *points_per_decade, sweep)
        }),
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

fn receipt(
    contract: &PreparedSavedOutput,
    status: SavedOutputMaterializationStatus,
    source_bindings: Option<crate::state::SavedOutputSourceBindings>,
) -> SavedOutputReceipt {
    SavedOutputReceipt {
        output_id: contract.output_id,
        output_revision: contract.output_revision,
        analysis_id: contract.analysis_id,
        contract_digest: contract.digest,
        name: contract.name.clone(),
        source_expression: contract.source_expression.clone(),
        complex_policy: contract.complex_policy,
        output_kind: contract.kind,
        save_policy: contract.policy,
        stored_precision: contract.precision,
        streaming: contract.streaming,
        display_intent: contract.display_intent,
        source_bindings,
        status,
    }
}

fn resolve_contract_waveform(
    contract: &PreparedSavedOutput,
    analysis: &AnalysisResult,
    waveforms: &[WaveformData],
) -> Result<WaveformData, String> {
    match contract.kind {
        SavedOutputKind::RawVoltageOrCurrent => resolve_raw_probe(
            &contract.source_expression,
            waveforms,
            &contract.name,
            analysis.analysis_type.uses_complex_bode_projection(),
        ),
        SavedOutputKind::DerivedExpression => resolve_derived_expression(
            &contract.source_expression,
            waveforms,
            &contract.name,
            contract.complex_policy,
        ),
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
            let separated = format!("S{output}_{input}");
            clone_named_waveform(waveforms, &separated, &contract.name)
                .or_else(|error| {
                    if output <= 9 && input <= 9 {
                        clone_named_waveform(
                            waveforms,
                            &format!("S{output}{input}"),
                            &contract.name,
                        )
                    } else {
                        Err(error)
                    }
                })
                .or_else(|_| {
                    clone_named_waveform(waveforms, &contract.source_expression, &contract.name)
                })
        }
    }
}

fn resolve_derived_expression(
    expression: &str,
    waveforms: &[WaveformData],
    output_name: &str,
    complex_policy: crate::state::ComplexExpressionPolicy,
) -> Result<WaveformData, String> {
    resolve_derived_with(
        expression,
        output_name,
        &calculator::WaveformsContext::with_policy(waveforms, complex_policy),
        waveforms.first(),
    )
}

fn resolve_derived_with(
    expression: &str,
    output_name: &str,
    context: &impl calculator::EvaluationContext,
    axis_source: Option<&WaveformData>,
) -> Result<WaveformData, String> {
    let parsed = calculator::parser::Parser::new(expression)
        .try_parse()
        .map_err(|error| format!("expression parse failed: {error}"))?;
    let value = calculator::evaluator::evaluate(&parsed, context)
        .map_err(|error| format!("expression evaluation failed: {error}"))?;
    calculator::evaluated_waveform(
        value,
        output_name,
        axis_source.map(|source| source.x.as_slice()),
    )
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
        expression[..open].trim().to_owned(),
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
    find_literal_waveform(waveforms, requested).or_else(|| {
        let (current, node) = probe_identity(requested);
        let engine = crate::state::ProbeTarget::engine_alias(node)?;
        find_literal_waveform(
            waveforms,
            &format!("{}({engine})", if current { "I" } else { "V" }),
        )
    })
}

fn find_literal_waveform<'a>(
    waveforms: &'a [WaveformData],
    requested: &str,
) -> Option<&'a WaveformData> {
    // Exact authored names win before compatibility with bare engine nodes.
    // A node named V1 and branch I(V1) are different physical quantities.
    waveforms
        .iter()
        .find(|waveform| waveform.name.eq_ignore_ascii_case(requested))
        .or_else(|| {
            waveforms.iter().find(|waveform| {
                let source = waveform
                    .complex
                    .as_ref()
                    .map_or(waveform.name.as_str(), |complex| {
                        complex.source_name.as_str()
                    });
                let (current, node) = probe_identity(source);
                let (requested_current, requested_node) = probe_identity(requested);
                current == requested_current && node.eq_ignore_ascii_case(requested_node)
            })
        })
}

fn probe_identity(name: &str) -> (bool, &str) {
    let name = name.trim_matches('|');
    if let Some(inner) = name.get(2..).and_then(|inner| inner.strip_suffix(')')) {
        if name
            .get(..2)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("I("))
        {
            return (true, inner);
        }
        if name
            .get(..2)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V("))
        {
            return (false, inner);
        }
    }
    (false, name)
}

fn waveform_matches_requested(waveform: &WaveformData, requested: &str) -> bool {
    find_waveform(std::slice::from_ref(waveform), requested).is_some()
}

fn clone_with_name(source: &WaveformData, name: &str) -> WaveformData {
    let mut waveform = source.clone();
    waveform.name = name.to_owned();
    waveform.display_cache = None;
    waveform
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
    result.unit = waveform.unit.clone();
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
    bytes.push(match output.display_intent {
        crate::state::SavedOutputDisplayIntent::Plot => 0,
        crate::state::SavedOutputDisplayIntent::DataBrowserOnly => 1,
    });
    if let Some(grid) = grid {
        bytes.push(1);
        bytes.extend_from_slice(&grid.start.to_bits().to_be_bytes());
        bytes.extend_from_slice(&grid.step.to_bits().to_be_bytes());
        bytes.extend_from_slice(&grid.stop.to_bits().to_be_bytes());
    } else {
        bytes.push(0);
    }
    // Historical contracts retain their exact digest. The new domain binds
    // rectangular evaluation without reinterpreting an old receipt.
    let domain = if output.complex_policy.is_legacy() {
        "rspice.prepared-saved-output/v1"
    } else {
        "rspice.prepared-saved-output/rectangular-v2"
    };
    content_digest(domain, &bytes)
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

    #[test]
    fn raw_probes_keep_voltage_and_current_namespaces_separate() {
        for voltage_name in ["V(V1)", "V1", "|V(V1)|"] {
            let voltage = WaveformData::new(voltage_name, vec![0.0, 1.0], vec![0.0, 1.0], "#fff");
            let current = WaveformData::new("I(V1)", vec![0.0, 1.0], vec![0.0, -0.001], "#fff");
            for traces in [
                vec![current.clone(), voltage.clone()],
                vec![voltage.clone(), current.clone()],
            ] {
                assert_eq!(
                    resolve_raw_probe("v(v1)", &traces, "Voltage", false)
                        .unwrap()
                        .y
                        .as_slice(),
                    &[0.0, 1.0]
                );
                assert_eq!(
                    resolve_raw_probe("i(v1)", &traces, "Current", false)
                        .unwrap()
                        .y
                        .as_slice(),
                    &[0.0, -0.001]
                );
            }
            assert!(resolve_raw_probe("I(V1)", &[voltage], "missing current", false).is_err());
            assert!(resolve_raw_probe("V(V1)", &[current], "missing voltage", false).is_err());
        }
    }

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
        for (actual, expected) in analysis.waveforms[1]
            .y
            .iter()
            .zip([0.0_f64, 2.5, 5.0, 7.5, 10.0])
        {
            let tolerance = 4.0 * f64::EPSILON * expected.abs().max(1.0);
            assert!(
                (actual - expected).abs() <= tolerance,
                "interpolated {actual:.17e} differs from {expected:.17e} by more than {tolerance:.3e}"
            );
        }
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
    fn deferred_materialization_is_atomic_when_the_output_name_collides() {
        let contract = PreparedSavedOutput::prepare(
            &output(
                SavedOutputPolicy::OnDemandFromRetainedState,
                SavedOutputPrecision::FullSourcePrecision,
            ),
            AnalysisInstanceId::new(),
            &transient_spec(),
        )
        .expect("prepare")
        .expect("applies");
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
                WaveformData::new("output_voltage", vec![0.0, 1.0], vec![9.0, 9.0], "#f00"),
            ]);
        materialize_saved_outputs(&mut analysis, &[contract]);
        let before_waveforms = analysis.waveforms.clone();

        let error = materialize_deferred_saved_output(&mut analysis, 0)
            .expect_err("different retained waveform must block materialization");

        assert!(error.contains("collides"));
        assert_eq!(analysis.waveforms, before_waveforms);
        assert_eq!(
            analysis.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Deferred
        );
    }

    #[test]
    fn live_output_projection_is_bounded_and_does_not_mutate_source_precision() {
        let live_output = SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            "output_voltage",
            "V(out)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
            SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
        )
        .expect("valid live output");
        let contract = PreparedSavedOutput::prepare(
            &live_output,
            AnalysisInstanceId::new(),
            &transient_spec(),
        )
        .expect("prepare")
        .expect("applies");
        let x = (0..10_000).map(|index| index as f64).collect::<Vec<_>>();
        let y = x
            .iter()
            .map(|value| (value / 17.0).sin())
            .collect::<Vec<_>>();
        let source = AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![WaveformData::new("out", x.clone(), y, "#fff")]);

        let projected = materialize_live_saved_outputs(&source, &[contract]);

        assert_eq!(source.waveforms[0].x.len(), 10_000);
        assert_eq!(projected.len(), 1);
        assert_eq!(projected[0].name, "output_voltage");
        assert!(projected[0].x.len() <= DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES);
        assert!(projected[0].display_cache.is_some());
        assert!(source.saved_output_receipts.is_empty());
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
    fn preflight_exactly_bounds_default_schematic_probe_grid() {
        let output = output(
            SavedOutputPolicy::SelectedAndFinalPoints,
            SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
        );
        let spec = transient_spec();
        let report = preflight_saved_output(&output, [(AnalysisInstanceId::new(), &spec)]);

        // 0 through 1 at 0.25 is five retained samples.  A real voltage
        // waveform stores an f64 x/y pair and the requested display cache
        // stores an f32 x/y pair for each sample.
        assert_eq!(
            report.storage_estimate(),
            &SavedOutputStorageEstimate::ExactBytes(5 * ((2 * 8) + (2 * 4)))
        );
    }

    #[test]
    fn plan_retention_discards_unselected_engine_waveforms() {
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
                WaveformData::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
                WaveformData::new("internal", vec![0.0, 1.0], vec![4.0, 5.0], "#aaa"),
            ]);

        retain_plan_saved_outputs(&mut analysis, &[contract]);

        assert_eq!(analysis.waveforms.len(), 1);
        assert_eq!(analysis.waveforms[0].name, "output_voltage");
        assert!(matches!(
            analysis.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Materialized { .. }
        ));
    }

    #[test]
    fn retained_output_keeps_save_and_initial_display_intent_separate() {
        let saved = output(
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
        )
        .with_display_intent(crate::state::SavedOutputDisplayIntent::DataBrowserOnly);
        let contract =
            PreparedSavedOutput::prepare(&saved, AnalysisInstanceId::new(), &transient_spec())
                .expect("prepare")
                .expect("applies");
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
            ]);

        retain_plan_saved_outputs(&mut analysis, &[contract]);

        assert_eq!(analysis.waveforms.len(), 1);
        assert!(!analysis.waveforms[0].visible);
        assert_eq!(
            analysis.saved_output_receipts[0].display_intent,
            crate::state::SavedOutputDisplayIntent::DataBrowserOnly
        );
    }

    #[test]
    fn empty_plan_output_registry_retains_no_engine_waveforms() {
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
            ]);

        retain_plan_saved_outputs(&mut analysis, &[]);

        assert!(analysis.waveforms.is_empty());
        assert!(analysis.saved_output_receipts.is_empty());
    }

    #[test]
    fn on_demand_retention_keeps_source_and_reports_plan_level_source_ownership() {
        let deferred = output(
            SavedOutputPolicy::OnDemandFromRetainedState,
            SavedOutputPrecision::FullSourcePrecision,
        );
        let analysis_id = AnalysisInstanceId::new();
        let contract = PreparedSavedOutput::prepare(&deferred, analysis_id, &transient_spec())
            .expect("prepare")
            .expect("applies");
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
            ]);

        retain_plan_saved_outputs(&mut analysis, &[contract]);
        let report = preflight_saved_output(&deferred, [(analysis_id, &transient_spec())]);

        assert_eq!(analysis.waveforms.len(), 1);
        assert_eq!(
            analysis.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Deferred
        );
        assert_eq!(
            report.storage_estimate(),
            &SavedOutputStorageEstimate::ExactBytes(0)
        );
        assert_eq!(report.retained_engine_source_analysis_ids(), &[analysis_id]);
        assert_eq!(
            retained_engine_source_upper_bound_bytes(1),
            25_000_000 * std::mem::size_of::<f64>() as u64
        );
    }

    #[test]
    fn sp_port_bounds_and_multi_digit_indices_bind_to_the_retained_circuit() {
        let output = SavedOutput::new(
            SavedOutputKind::RfPortQuantity,
            "forward_gain",
            "S(10,1)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("syntactically valid output");
        let spec = AnalysisSpec::SParameter {
            do_noise: false,
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
            SavedOutputSemanticStatus::RuntimeBound { reason } if reason.contains("elaborated circuit")
        ));
        let contract = PreparedSavedOutput::prepare(&output, AnalysisInstanceId::new(), &spec)
            .unwrap()
            .unwrap();
        let source = WaveformData::new("S10_1", vec![1e6], vec![0.5], "#fff")
            .with_complex_components("S10_1", vec![0.5], vec![-0.1]);
        let mut analysis =
            AnalysisResult::new(1, AnalysisType::SParameter, "SP").with_waveforms(vec![source]);
        materialize_saved_outputs(&mut analysis, std::slice::from_ref(&contract));
        assert!(matches!(
            analysis.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Materialized {
                sample_count: 1,
                ..
            }
        ));
        assert_eq!(analysis.waveforms[1].name, "forward_gain");
        assert_eq!(
            analysis.waveforms[1]
                .complex
                .as_ref()
                .unwrap()
                .imag
                .as_ref(),
            &[-0.1]
        );
        let mut missing = AnalysisResult::new(1, AnalysisType::SParameter, "SP");
        materialize_saved_outputs(&mut missing, &[contract]);
        assert!(matches!(
            missing.saved_output_receipts[0].status,
            SavedOutputMaterializationStatus::Unavailable { .. }
        ));
    }
}
