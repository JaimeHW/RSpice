//! Atomic adoption of the waveforms one immutable output contract produces.
//! Resolution and collision checks finish before any member is adopted. The
//! same path serves immediate outputs, retained-source evaluation, and families.

use std::collections::HashMap;

use super::dc_family::Sources;
use super::*;
use rspice_results::saved_output::SavedOutputDcMember;

enum ResolvedOutput<W> {
    Single(W),
    DcFamily(Vec<W>),
}

impl<W: OutputWaveform> ResolvedOutput<W> {
    fn waveforms_mut(&mut self) -> &mut [W] {
        match self {
            Self::Single(waveform) => std::slice::from_mut(waveform),
            Self::DcFamily(waveforms) => waveforms,
        }
    }

    fn status(&self) -> SavedOutputMaterializationStatus {
        match self {
            Self::Single(waveform) => SavedOutputMaterializationStatus::Materialized {
                waveform_name: waveform.as_ref().name.clone(),
                sample_count: waveform.as_ref().x.len() as u64,
            },
            Self::DcFamily(waveforms) => SavedOutputMaterializationStatus::MaterializedDcFamily {
                members: waveforms
                    .iter()
                    .enumerate()
                    .map(|(member, waveform)| SavedOutputDcMember {
                        member,
                        waveform_name: waveform.as_ref().name.clone(),
                        sample_count: waveform.as_ref().x.len() as u64,
                    })
                    .collect(),
            },
        }
    }

    fn into_waveforms(self) -> impl Iterator<Item = W> {
        let (single, family) = match self {
            Self::Single(waveform) => (Some(waveform), Vec::new()),
            Self::DcFamily(waveforms) => (None, waveforms),
        };
        single.into_iter().chain(family)
    }
}

fn resolve_output<W: OutputWaveform>(
    contract: &PreparedSavedOutput,
    analysis: &AnalysisResult<W>,
    source: &[W],
    family: Option<&Result<Sources<'_, W>, String>>,
    bindings: Option<&rspice_results::saved_output::SavedOutputSourceBindings>,
) -> Result<ResolvedOutput<W>, String> {
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

fn same_samples<W: OutputWaveform>(left: &W, right: &W) -> bool {
    left.as_ref().x == right.as_ref().x
        && left.as_ref().y == right.as_ref().y
        && left.as_ref().complex == right.as_ref().complex
        && left.as_ref().unit == right.as_ref().unit
}

fn prepare_output<W: OutputWaveform>(
    contract: &PreparedSavedOutput,
    analysis: &AnalysisResult<W>,
    source: &[W],
    preserve_engine: bool,
    family: Option<&Result<Sources<'_, W>, String>>,
    bindings: Option<&rspice_results::saved_output::SavedOutputSourceBindings>,
    destinations: &HashMap<String, usize>,
) -> Result<ResolvedOutput<W>, String> {
    let mut output = resolve_output(contract, analysis, source, family, bindings)?;
    for waveform in output.waveforms_mut() {
        if contract.policy() == SavedOutputPolicy::SelectedAndFinalPoints
            && let Some(grid) = contract.selection_grid()
        {
            *waveform = resample_selected_and_final(waveform, grid)?;
        }
        if let Some(existing) = destinations
            .get(&waveform.as_ref().name)
            .map(|index| &analysis.waveforms[*index])
            && !same_samples(existing, waveform)
        {
            if preserve_engine
                && contract.kind() == SavedOutputKind::RawVoltageOrCurrent
                && existing.as_ref().unit == waveform.as_ref().unit
                && waveform_matches_requested(existing, contract.source_expression())
            {
                // Save-all retains the exact engine superset of a sampled raw probe.
                *waveform = existing.clone();
            } else {
                return Err(format!(
                    "saved-output name '{}' collides with a different retained waveform",
                    waveform.as_ref().name
                ));
            }
        }
        waveform.set_visible(
            contract.display_intent()
                == rspice_results::saved_output::SavedOutputDisplayIntent::Plot,
        );
        if contract.precision() == SavedOutputPrecision::DisplayCacheWithFullSourcePrecision
            || contract.streaming() == SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
        {
            waveform.rebuild_output_display_cache();
        }
    }
    Ok(output)
}

fn adopt_output<W: OutputWaveform>(
    analysis: &mut AnalysisResult<W>,
    output: ResolvedOutput<W>,
    destinations: &mut HashMap<String, usize>,
) -> SavedOutputMaterializationStatus {
    let status = output.status();
    for waveform in output.into_waveforms() {
        if let Some(index) = destinations.get(&waveform.as_ref().name) {
            let existing = &mut analysis.waveforms[*index];
            existing.adopt_presentation(waveform);
        } else {
            destinations.insert(waveform.as_ref().name.clone(), analysis.waveforms.len());
            analysis.waveforms.push(waveform);
        }
    }
    status
}

fn destinations<W: OutputWaveform>(analysis: &AnalysisResult<W>) -> HashMap<String, usize> {
    analysis
        .waveforms
        .iter()
        .enumerate()
        .map(|(index, waveform)| (waveform.as_ref().name.clone(), index))
        .collect()
}

/// Operating points retain scalar voltages/currents in their table. Expose that
/// solved basis to the same output resolver using the single-point OP axis.
/// Authored outputs are projections of this basis, never additional circuit
/// nodes or branches. Reusing their labels here would let a previous output
/// named `V(absent)` invent a physical node during deferred evaluation.
fn source_waveforms<W: OutputWaveform>(analysis: &AnalysisResult<W>) -> Result<Vec<W>, String> {
    if let Some(basis) = analysis
        .result_payload
        .as_ref()
        .map(|payload| {
            payload
                .retained_waveform_basis()
                .map(|basis| basis.map(|waves| waves.into_iter().map(W::from_retained).collect()))
        })
        .transpose()?
        .flatten()
    {
        return Ok(basis);
    }
    let Some(op) = &analysis.dc_op else {
        return Ok(analysis.waveforms.clone());
    };
    let axis = Arc::new(vec![0.0]);
    Ok(op
        .node_voltages
        .iter()
        .chain(&op.branch_currents)
        .map(|value| {
            W::from_retained(
                RetainedWaveform::new(&value.name, Arc::clone(&axis), vec![value.value])
                    .with_unit(&value.unit),
            )
        })
        .collect())
}

pub fn materialize_saved_outputs<W: OutputWaveform>(
    analysis: &mut AnalysisResult<W>,
    contracts: &[PreparedSavedOutput],
) {
    materialize_with_engine_policy(analysis, contracts, false);
}

pub fn materialize_saved_outputs_preserving_engine<W: OutputWaveform>(
    analysis: &mut AnalysisResult<W>,
    contracts: &[PreparedSavedOutput],
) {
    materialize_with_engine_policy(analysis, contracts, true);
}

fn materialize_with_engine_policy<W: OutputWaveform>(
    analysis: &mut AnalysisResult<W>,
    contracts: &[PreparedSavedOutput],
    preserve_engine: bool,
) {
    if contracts.is_empty() {
        return;
    }
    let source = match source_waveforms(analysis) {
        Ok(source) => source,
        Err(reason) => {
            for contract in contracts {
                analysis.saved_output_receipts.push(contract.receipt(
                    SavedOutputMaterializationStatus::Unavailable {
                        reason: reason.clone(),
                    },
                    None,
                ));
            }
            return;
        }
    };
    let family = Sources::new(analysis, &source);
    let mut destinations = destinations(analysis);
    for contract in contracts {
        let captured = bindings::capture(contract, analysis, &source, family.as_ref());
        let status = if let Err(reason) = &captured {
            SavedOutputMaterializationStatus::Unavailable {
                reason: reason.clone(),
            }
        } else if contract.policy() == SavedOutputPolicy::OnDemandFromRetainedState {
            SavedOutputMaterializationStatus::Deferred
        } else if contract.policy() == SavedOutputPolicy::FailureDiagnosticsOnly && analysis.success
        {
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
            .push(contract.receipt(status, captured.ok().flatten()));
    }
}

pub fn apply_saved_output_policy<W: OutputWaveform>(
    analysis: &mut AnalysisResult<W>,
    policy: crate::execution::SavePolicy,
    contracts: &[PreparedSavedOutput],
) {
    if !matches!(policy, crate::execution::SavePolicy::PlanOwned { .. }) {
        return;
    }
    // Automatic selection must not discard an analysis's native results just
    // because it has no ordinary node-waveform output contract (for example,
    // Monte Carlo statistics, a noise spectrum or an FFT).
    if policy.output_selection_mode()
        == rspice_simulation_contract::output_policy::OutputSelectionMode::Automatic
        && contracts.is_empty()
    {
        return;
    }
    if policy.output_selection_mode()
        == rspice_simulation_contract::output_policy::OutputSelectionMode::SaveAll
    {
        for waveform in &mut analysis.waveforms {
            waveform.set_visible(false);
        }
        materialize_saved_outputs_preserving_engine(analysis, contracts);
    } else {
        retain_plan_saved_outputs(analysis, contracts);
    }
}

pub fn retain_plan_saved_outputs<W: OutputWaveform>(
    analysis: &mut AnalysisResult<W>,
    contracts: &[PreparedSavedOutput],
) {
    let receipt_start = analysis.saved_output_receipts.len();
    materialize_saved_outputs(analysis, contracts);
    let names = analysis.saved_output_receipts[receipt_start..]
        .iter()
        .flat_map(|receipt| receipt.status.materialized_waveforms())
        .map(|(name, _)| name.to_owned())
        .collect::<HashSet<_>>();
    if !matches!(
        analysis.result_payload,
        Some(
            rspice_results::analysis_payload::AnalysisResultPayload::Qpss { .. }
                | rspice_results::analysis_payload::AnalysisResultPayload::Qpac { .. }
                | rspice_results::analysis_payload::AnalysisResultPayload::Qpxf { .. }
                | rspice_results::analysis_payload::AnalysisResultPayload::Qpnoise { .. }
        )
    ) && contracts
        .iter()
        .any(|contract| contract.policy() == SavedOutputPolicy::OnDemandFromRetainedState)
    {
        // Source curves remain available for deferred evaluation, but are not
        // authored plot outputs in an explicit-only plan.
        for waveform in &mut analysis.waveforms {
            if !names.contains(&waveform.as_ref().name) {
                waveform.set_visible(false);
            }
        }
        return;
    }
    if let Some(rspice_results::analysis_payload::AnalysisResultPayload::DcSweep { evidence }) =
        &mut analysis.result_payload
    {
        Arc::make_mut(evidence).retain_curves(&names);
    }
    analysis
        .waveforms
        .retain(|waveform| names.contains(&waveform.as_ref().name));
}

pub fn materialize_live_saved_outputs<W: OutputWaveform>(
    source_analysis: &AnalysisResult<W>,
    contracts: &[PreparedSavedOutput],
    maximum_samples: usize,
) -> Vec<W> {
    let mut live = contracts
        .iter()
        .filter(|contract| {
            contract.streaming() == SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
                && contract.display_intent()
                    == rspice_results::saved_output::SavedOutputDisplayIntent::Plot
        })
        .peekable();
    if live.peek().is_none() {
        return Vec::new();
    }
    let mut outputs = Vec::new();
    let Ok(source) = source_waveforms(source_analysis) else {
        return Vec::new();
    };
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
            waveform.set_visible(true);
            if let Ok(preview) = waveform.into_output_preview(maximum_samples) {
                outputs.push(preview);
            }
        }
    }
    outputs
}

/// Adopt the entire deferred output only after every member and receipt validate.
pub fn materialize_deferred_saved_output<W: OutputWaveform>(
    analysis: &mut AnalysisResult<W>,
    receipt_index: usize,
) -> Result<(), String> {
    let receipt = analysis
        .saved_output_receipts
        .get(receipt_index)
        .ok_or_else(|| "saved-output receipt no longer exists".to_owned())?;
    let contract = PreparedSavedOutput::from_deferred_receipt(receipt)?;
    let source = source_waveforms(analysis)?;
    let family = Sources::new(analysis, &source);
    let bindings = if receipt.source_bindings.is_some() {
        receipt.source_bindings.clone()
    } else if matches!(
        contract.kind(),
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
