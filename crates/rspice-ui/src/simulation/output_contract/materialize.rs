//! Atomic adoption of the waveforms one immutable output contract produces.
//! Resolution and collision checks finish before any member is adopted. The
//! same path serves immediate outputs, retained-source evaluation, and families.

use std::collections::HashMap;

use super::dc_family::Sources;
use super::*;
use crate::state::SavedOutputDcMember;

enum ResolvedOutput {
    Single(WaveformData),
    DcFamily(Vec<WaveformData>),
}

impl ResolvedOutput {
    fn waveforms_mut(&mut self) -> &mut [WaveformData] {
        match self {
            Self::Single(waveform) => std::slice::from_mut(waveform),
            Self::DcFamily(waveforms) => waveforms,
        }
    }

    fn status(&self) -> SavedOutputMaterializationStatus {
        match self {
            Self::Single(waveform) => SavedOutputMaterializationStatus::Materialized {
                waveform_name: waveform.name.clone(),
                sample_count: waveform.x.len() as u64,
            },
            Self::DcFamily(waveforms) => SavedOutputMaterializationStatus::MaterializedDcFamily {
                members: waveforms
                    .iter()
                    .enumerate()
                    .map(|(member, waveform)| SavedOutputDcMember {
                        member,
                        waveform_name: waveform.name.clone(),
                        sample_count: waveform.x.len() as u64,
                    })
                    .collect(),
            },
        }
    }

    fn into_waveforms(self) -> impl Iterator<Item = WaveformData> {
        let (single, family) = match self {
            Self::Single(waveform) => (Some(waveform), Vec::new()),
            Self::DcFamily(waveforms) => (None, waveforms),
        };
        single.into_iter().chain(family)
    }
}

fn resolve_output(
    contract: &PreparedSavedOutput,
    analysis: &AnalysisResult,
    source: &[WaveformData],
    family: Option<&Result<Sources<'_>, String>>,
    bindings: Option<&crate::state::SavedOutputSourceBindings>,
) -> Result<ResolvedOutput, String> {
    if let Some(family) = family {
        family
            .as_ref()
            .map_err(Clone::clone)?
            .resolve(
                contract,
                bindings.ok_or_else(|| "DC output has no source bindings".to_owned())?,
            )
            .map(ResolvedOutput::DcFamily)
    } else if let Some(bindings) = bindings {
        super::bindings::resolve(contract, analysis, source, bindings).map(ResolvedOutput::Single)
    } else {
        resolve_contract_waveform(contract, analysis, source).map(ResolvedOutput::Single)
    }
}

fn same_samples(left: &WaveformData, right: &WaveformData) -> bool {
    left.x == right.x
        && left.y == right.y
        && left.complex == right.complex
        && left.unit == right.unit
}

fn prepare_output(
    contract: &PreparedSavedOutput,
    analysis: &AnalysisResult,
    source: &[WaveformData],
    preserve_engine: bool,
    family: Option<&Result<Sources<'_>, String>>,
    bindings: Option<&crate::state::SavedOutputSourceBindings>,
    destinations: &HashMap<String, usize>,
) -> Result<ResolvedOutput, String> {
    let mut output = resolve_output(contract, analysis, source, family, bindings)?;
    for waveform in output.waveforms_mut() {
        if contract.policy == SavedOutputPolicy::SelectedAndFinalPoints
            && let Some(grid) = contract.selection_grid
        {
            *waveform = resample_selected_and_final(waveform, grid)?;
        }
        if let Some(existing) = destinations
            .get(&waveform.name)
            .map(|index| &analysis.waveforms[*index])
            && !same_samples(existing, waveform)
        {
            if preserve_engine
                && contract.kind == SavedOutputKind::RawVoltageOrCurrent
                && existing.unit == waveform.unit
                && waveform_matches_requested(existing, &contract.source_expression)
            {
                // Save-all retains the exact engine superset of a sampled raw probe.
                *waveform = existing.clone();
            } else {
                return Err(format!(
                    "saved-output name '{}' collides with a different retained waveform",
                    waveform.name
                ));
            }
        }
        waveform.visible = contract.display_intent == crate::state::SavedOutputDisplayIntent::Plot;
        if contract.precision == SavedOutputPrecision::DisplayCacheWithFullSourcePrecision
            || contract.streaming == SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
        {
            waveform.rebuild_display_cache(DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES);
        }
    }
    Ok(output)
}

fn adopt_output(
    analysis: &mut AnalysisResult,
    output: ResolvedOutput,
    destinations: &mut HashMap<String, usize>,
) -> SavedOutputMaterializationStatus {
    let status = output.status();
    for waveform in output.into_waveforms() {
        if let Some(index) = destinations.get(&waveform.name) {
            let existing = &mut analysis.waveforms[*index];
            existing.display_cache = waveform.display_cache;
            existing.visible = waveform.visible;
        } else {
            destinations.insert(waveform.name.clone(), analysis.waveforms.len());
            analysis.waveforms.push(waveform);
        }
    }
    status
}

fn destinations(analysis: &AnalysisResult) -> HashMap<String, usize> {
    analysis
        .waveforms
        .iter()
        .enumerate()
        .map(|(index, waveform)| (waveform.name.clone(), index))
        .collect()
}

/// Operating points retain scalar voltages/currents in their table. Expose that
/// solved basis to the same output resolver using the single-point OP axis.
/// Authored outputs are projections of this basis, never additional circuit
/// nodes or branches. Reusing their labels here would let a previous output
/// named `V(absent)` invent a physical node during deferred evaluation.
fn source_waveforms(analysis: &AnalysisResult) -> Vec<WaveformData> {
    let Some(op) = &analysis.dc_op else {
        return analysis.waveforms.clone();
    };
    let axis = Arc::new(vec![0.0]);
    op.node_voltages
        .iter()
        .chain(&op.branch_currents)
        .map(|value| {
            WaveformData::new(&value.name, Arc::clone(&axis), vec![value.value], "#f5b700")
                .with_unit(&value.unit)
        })
        .collect()
}

pub(in crate::simulation) fn materialize_saved_outputs(
    analysis: &mut AnalysisResult,
    contracts: &[PreparedSavedOutput],
) {
    materialize_with_engine_policy(analysis, contracts, false);
}

pub(in crate::simulation) fn materialize_saved_outputs_preserving_engine(
    analysis: &mut AnalysisResult,
    contracts: &[PreparedSavedOutput],
) {
    materialize_with_engine_policy(analysis, contracts, true);
}

fn materialize_with_engine_policy(
    analysis: &mut AnalysisResult,
    contracts: &[PreparedSavedOutput],
    preserve_engine: bool,
) {
    if contracts.is_empty() {
        return;
    }
    let source = source_waveforms(analysis);
    let family = Sources::new(analysis, &source);
    let mut destinations = destinations(analysis);
    for contract in contracts {
        let captured = bindings::capture(contract, analysis, &source, family.as_ref());
        let status = if let Err(reason) = &captured {
            SavedOutputMaterializationStatus::Unavailable {
                reason: reason.clone(),
            }
        } else if contract.policy == SavedOutputPolicy::OnDemandFromRetainedState {
            SavedOutputMaterializationStatus::Deferred
        } else if contract.policy == SavedOutputPolicy::FailureDiagnosticsOnly && analysis.success {
            SavedOutputMaterializationStatus::SuppressedOnSuccess
        } else {
            match prepare_output(
                contract,
                analysis,
                &source,
                preserve_engine,
                family.as_ref(),
                captured.as_ref().ok().and_then(Option::as_ref),
                &destinations,
            ) {
                Ok(output) => adopt_output(analysis, output, &mut destinations),
                Err(reason) => SavedOutputMaterializationStatus::Unavailable { reason },
            }
        };
        analysis
            .saved_output_receipts
            .push(receipt(contract, status, captured.ok().flatten()));
    }
}

pub(in crate::simulation) fn apply_saved_output_policy(
    analysis: &mut AnalysisResult,
    policy: crate::simulation::execution::SavePolicy,
    contracts: &[PreparedSavedOutput],
) {
    if !matches!(
        policy,
        crate::simulation::execution::SavePolicy::PlanOwned { .. }
    ) {
        return;
    }
    if policy.output_selection_mode() == crate::state::OutputSelectionMode::SaveAll {
        for waveform in &mut analysis.waveforms {
            waveform.visible = false;
        }
        materialize_saved_outputs_preserving_engine(analysis, contracts);
    } else {
        retain_plan_saved_outputs(analysis, contracts);
    }
}

pub(in crate::simulation) fn retain_plan_saved_outputs(
    analysis: &mut AnalysisResult,
    contracts: &[PreparedSavedOutput],
) {
    let receipt_start = analysis.saved_output_receipts.len();
    materialize_saved_outputs(analysis, contracts);
    let names = analysis.saved_output_receipts[receipt_start..]
        .iter()
        .flat_map(|receipt| receipt.status.materialized_waveforms())
        .map(|(name, _)| name.to_owned())
        .collect::<HashSet<_>>();
    if contracts
        .iter()
        .any(|contract| contract.policy == SavedOutputPolicy::OnDemandFromRetainedState)
    {
        // Source curves remain available for deferred evaluation, but are not
        // authored plot outputs in an explicit-only plan.
        for waveform in &mut analysis.waveforms {
            if !names.contains(&waveform.name) {
                waveform.visible = false;
            }
        }
        return;
    }
    if let Some(crate::state::AnalysisResultPayload::DcSweep { evidence }) =
        &mut analysis.result_payload
    {
        Arc::make_mut(evidence).retain_curves(&names);
    }
    analysis
        .waveforms
        .retain(|waveform| names.contains(&waveform.name));
}

pub(in crate::simulation) fn materialize_live_saved_outputs(
    source_analysis: &AnalysisResult,
    contracts: &[PreparedSavedOutput],
) -> Vec<WaveformData> {
    let mut live = contracts
        .iter()
        .filter(|contract| {
            contract.streaming() == SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
                && contract.display_intent == crate::state::SavedOutputDisplayIntent::Plot
        })
        .peekable();
    if live.peek().is_none() {
        return Vec::new();
    }
    let mut outputs = Vec::new();
    let source = source_waveforms(source_analysis);
    let family = Sources::new(source_analysis, &source);
    for contract in live {
        let Ok(bindings) = bindings::capture(contract, source_analysis, &source, family.as_ref())
        else {
            continue;
        };
        let Ok(output) = resolve_output(
            contract,
            source_analysis,
            &source,
            family.as_ref(),
            bindings.as_ref(),
        ) else {
            continue;
        };
        for mut waveform in output.into_waveforms() {
            waveform.visible = true;
            if let Ok(preview) =
                waveform.into_bounded_preview(DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES)
            {
                outputs.push(preview);
            }
        }
    }
    outputs
}

/// Adopt the entire deferred output only after every member and receipt validate.
pub(crate) fn materialize_deferred_saved_output(
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
        name: receipt.name,
        source_expression: receipt.source_expression,
        complex_policy: receipt.complex_policy,
        policy: receipt.save_policy,
        precision: receipt.stored_precision,
        streaming: receipt.streaming,
        display_intent: receipt.display_intent,
        selection_grid: None,
        candidates: None,
        digest: receipt.contract_digest,
    };
    let source = source_waveforms(analysis);
    let family = Sources::new(analysis, &source);
    let bindings = if receipt.source_bindings.is_some() {
        receipt.source_bindings
    } else if matches!(
        contract.kind,
        SavedOutputKind::RawVoltageOrCurrent | SavedOutputKind::DerivedExpression
    ) {
        if analysis.dc_op.is_none() && family.is_none() {
            return Err("this historical output has no physical source bindings; rerun its original deck to evaluate it".to_owned());
        }
        bindings::capture(&contract, analysis, &source, family.as_ref())?
    } else {
        None
    };
    let mut destinations = destinations(analysis);
    let output = prepare_output(
        &contract,
        analysis,
        &source,
        false,
        family.as_ref(),
        bindings.as_ref(),
        &destinations,
    )?;
    let mut candidate = analysis.clone();
    candidate.saved_output_receipts[receipt_index].status =
        adopt_output(&mut candidate, output, &mut destinations);
    candidate.validate_retained_evidence()?;
    *analysis = candidate;
    Ok(())
}
