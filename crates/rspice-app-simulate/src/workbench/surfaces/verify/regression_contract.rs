//! What a locked baseline promises, and how a run is judged against it.
//!
//! A regression verdict is only meaningful if the baseline it cites cannot
//! have moved underneath it, so every check is anchored to a content digest of
//! the baseline run and the tolerance rule in force at comparison time.  A
//! target whose rule has been deleted, whose waveform is unusable, or whose
//! receipt no longer matches the contract is reported as a coverage gap — it
//! is never quietly scored as a pass.

use super::*;

pub(super) fn active_regression_specification_policy(
    model: &Model,
) -> crate::state::RegressionSpecificationPolicy {
    model
        .sim_setup
        .analysis_plan
        .as_ref()
        .and_then(|plan| model.workspace.content.plan_data(plan.id()))
        .map(|payload| payload.specification_policy.regression)
        .unwrap_or_default()
}

pub(super) const fn regression_requires_waveforms(
    policy: crate::state::RegressionSpecificationPolicy,
) -> bool {
    matches!(
        policy,
        crate::state::RegressionSpecificationPolicy::LimitAndWaveform
    )
}

pub(super) fn regression_coverage_issues_for_policy(
    baseline: &crate::state::SimulationRun,
    current: &crate::state::SimulationRun,
    rules: &[crate::state::RegressionToleranceRule],
    policy: crate::state::RegressionSpecificationPolicy,
) -> Vec<RegressionCoverageIssue> {
    let mut issues = regression_coverage_issues(baseline, current, rules);
    if !regression_requires_waveforms(policy) {
        issues.retain(|issue| {
            issue
                .target
                .as_ref()
                .is_none_or(|target| target.kind == crate::state::RegressionTargetKind::Measurement)
        });
    }
    issues
}

pub(super) fn orphaned_regression_targets(
    issues: &[RegressionCoverageIssue],
) -> Vec<crate::state::RegressionTargetSelector> {
    issues
        .iter()
        .filter(|issue| issue.detail == "persisted tolerance target is absent from both datasets")
        .filter_map(|issue| issue.target.clone())
        .collect()
}

pub(super) fn validate_regression_waveform_data(
    waveform: &crate::state::RetainedTrace,
) -> Result<(), String> {
    if waveform.x.len() != waveform.y.len() {
        return Err(format!(
            "axis/value length mismatch ({} x values, {} y values)",
            waveform.x.len(),
            waveform.y.len()
        ));
    }
    if waveform.x.is_empty() {
        return Err("waveform contains no samples".to_owned());
    }
    if !strictly_increasing_finite(waveform.x.as_slice()) {
        return Err("waveform axis is not finite and strictly increasing".to_owned());
    }
    if waveform.y.iter().any(|value| !value.is_finite()) {
        return Err("waveform contains a non-finite value".to_owned());
    }
    if let Some(complex) = &waveform.complex
        && (complex.real.len() != waveform.x.len()
            || complex.imag.len() != waveform.x.len()
            || complex.real.iter().any(|value| !value.is_finite())
            || complex.imag.iter().any(|value| !value.is_finite()))
    {
        return Err("complex waveform components are incomplete or non-finite".to_owned());
    }
    Ok(())
}

pub(super) fn regression_target_observations(
    run: &crate::state::SimulationRun,
) -> Vec<RegressionTargetObservation> {
    let mut observations = Vec::new();
    for analysis in &run.analyses {
        let Some(provenance) = analysis.provenance.as_ref() else {
            continue;
        };
        for (index, measurement) in analysis.measurements.iter().enumerate() {
            let occurrence = analysis.measurements[..index]
                .iter()
                .filter(|prior| prior.name.eq_ignore_ascii_case(&measurement.name))
                .count();
            let Ok(occurrence) = u32::try_from(occurrence) else {
                continue;
            };
            observations.push(RegressionTargetObservation {
                target: crate::state::RegressionTargetSelector {
                    source_domain: provenance.source_domain(),
                    source_instance_id: provenance.source_instance_id(),
                    kind: crate::state::RegressionTargetKind::Measurement,
                    name: measurement.name.to_ascii_lowercase(),
                    occurrence,
                },
                validation_error: match measurement.value {
                    Some(value) if value.is_finite() => None,
                    Some(_) => Some("measurement value is non-finite".to_owned()),
                    None => Some(format!(
                        "measurement has no numeric value{}",
                        measurement
                            .error
                            .as_deref()
                            .map_or_else(String::new, |error| format!(": {error}"))
                    )),
                },
            });
        }
        for (index, waveform) in analysis.waveforms.iter().enumerate() {
            let occurrence = analysis.waveforms[..index]
                .iter()
                .filter(|prior| prior.name.eq_ignore_ascii_case(&waveform.name))
                .count();
            let Ok(occurrence) = u32::try_from(occurrence) else {
                continue;
            };
            observations.push(RegressionTargetObservation {
                target: crate::state::RegressionTargetSelector {
                    source_domain: provenance.source_domain(),
                    source_instance_id: provenance.source_instance_id(),
                    kind: crate::state::RegressionTargetKind::Waveform,
                    name: waveform.name.to_ascii_lowercase(),
                    occurrence,
                },
                validation_error: validate_regression_waveform_data(waveform).err(),
            });
        }
    }
    observations
}

pub(super) fn regression_coverage_issues(
    baseline: &crate::state::SimulationRun,
    current: &crate::state::SimulationRun,
    rules: &[crate::state::RegressionToleranceRule],
) -> Vec<RegressionCoverageIssue> {
    let baseline_observations = regression_target_observations(baseline);
    let current_observations = regression_target_observations(current);
    let mut targets = baseline_observations
        .iter()
        .chain(&current_observations)
        .map(|observation| observation.target.clone())
        .chain(rules.iter().map(|rule| rule.target.clone()))
        .collect::<Vec<_>>();
    targets.sort_by(|left, right| {
        let domain = |domain| match domain {
            crate::state::AnalysisResultSourceDomain::SimulationPlan => 0,
            crate::state::AnalysisResultSourceDomain::ManualDeck => 1,
            crate::state::AnalysisResultSourceDomain::LegacyUnclassified => 2,
        };
        domain(left.source_domain)
            .cmp(&domain(right.source_domain))
            .then_with(|| {
                left.source_instance_id
                    .as_uuid()
                    .as_bytes()
                    .cmp(right.source_instance_id.as_uuid().as_bytes())
            })
            .then_with(|| left.name.cmp(&right.name))
            .then_with(|| left.occurrence.cmp(&right.occurrence))
            .then_with(|| {
                let kind = |kind| match kind {
                    crate::state::RegressionTargetKind::Measurement => 0,
                    crate::state::RegressionTargetKind::Waveform => 1,
                };
                kind(left.kind).cmp(&kind(right.kind))
            })
    });
    targets.dedup();

    let mut issues = Vec::new();
    for target in targets {
        let baseline_observation = baseline_observations
            .iter()
            .find(|observation| observation.target == target);
        let current_observation = current_observations
            .iter()
            .find(|observation| observation.target == target);
        let kind = match target.kind {
            crate::state::RegressionTargetKind::Measurement => "measurement",
            crate::state::RegressionTargetKind::Waveform => "waveform",
        };
        let detail = match (baseline_observation, current_observation) {
            (None, None) => {
                Some("persisted tolerance target is absent from both datasets".to_owned())
            }
            (None, Some(_)) => Some("target is present only in the candidate dataset".to_owned()),
            (Some(_), None) => Some("target is missing from the candidate dataset".to_owned()),
            (Some(baseline), Some(current)) => baseline
                .validation_error
                .as_ref()
                .map(|error| format!("baseline results are invalid: {error}"))
                .or_else(|| {
                    current
                        .validation_error
                        .as_ref()
                        .map(|error| format!("candidate results are invalid: {error}"))
                }),
        };
        if let Some(detail) = detail {
            issues.push(RegressionCoverageIssue {
                target: Some(target.clone()),
                label: format!("{kind} · {} [{}]", target.name, target.occurrence),
                detail,
            });
        }
    }

    let analysis_keys = |run: &crate::state::SimulationRun| {
        run.analyses
            .iter()
            .filter_map(|analysis| analysis.provenance.as_ref())
            .map(|provenance| (provenance.source_domain(), provenance.source_instance_id()))
            .collect::<Vec<_>>()
    };
    let baseline_analyses = analysis_keys(baseline);
    let current_analyses = analysis_keys(current);
    for (domain, id) in baseline_analyses
        .iter()
        .chain(&current_analyses)
        .copied()
        .collect::<Vec<_>>()
    {
        let in_baseline = baseline_analyses.contains(&(domain, id));
        let in_current = current_analyses.contains(&(domain, id));
        if in_baseline != in_current
            && !issues.iter().any(|issue| {
                issue.target.as_ref().is_some_and(|target| {
                    target.source_domain == domain && target.source_instance_id == id
                })
            })
        {
            issues.push(RegressionCoverageIssue {
                target: None,
                label: format!("analysis · {id}"),
                detail: if in_baseline {
                    "analysis is missing from the candidate dataset".to_owned()
                } else {
                    "analysis is present only in the candidate dataset".to_owned()
                },
            });
        }
    }
    issues
}

impl RegressionCheck {
    pub(super) fn delta(&self) -> f64 {
        self.current - self.baseline
    }

    pub(super) fn changed(&self) -> bool {
        self.current.to_bits() != self.baseline.to_bits()
    }
}

pub(super) fn derive_regression_checks(
    baseline: &crate::state::SimulationRun,
    current: &crate::state::SimulationRun,
) -> Vec<RegressionCheck> {
    let mut checks = Vec::new();
    for baseline_analysis in baseline
        .analyses
        .iter()
        .filter(|analysis| verified_analysis(analysis))
    {
        let baseline_provenance = baseline_analysis
            .provenance
            .as_ref()
            .expect("verified analyses retain provenance");
        let Some(current_analysis) = current.analyses.iter().find(|analysis| {
            verified_analysis(analysis)
                && analysis.provenance.as_ref().is_some_and(|provenance| {
                    provenance.source_domain() == baseline_provenance.source_domain()
                        && provenance.source_instance_id()
                            == baseline_provenance.source_instance_id()
                })
        }) else {
            continue;
        };
        let current_provenance = current_analysis
            .provenance
            .as_ref()
            .expect("verified analyses retain provenance");
        for (index, baseline_measurement) in baseline_analysis.measurements.iter().enumerate() {
            let name = baseline_measurement.name.to_ascii_lowercase();
            let occurrence = baseline_analysis.measurements[..index]
                .iter()
                .filter(|measurement| measurement.name.eq_ignore_ascii_case(&name))
                .count();
            let Some(current_measurement) = current_analysis
                .measurements
                .iter()
                .filter(|measurement| measurement.name.eq_ignore_ascii_case(&name))
                .nth(occurrence)
            else {
                continue;
            };
            let Some(baseline_value) = baseline_measurement.value.filter(|value| value.is_finite())
            else {
                continue;
            };
            let Some(current_value) = current_measurement.value.filter(|value| value.is_finite())
            else {
                continue;
            };
            let Ok(occurrence) = u32::try_from(occurrence) else {
                continue;
            };
            // The unit the producer stated the value in, the current run's
            // first; a measurement that states none is shown bare.
            let unit = [current_measurement, baseline_measurement]
                .into_iter()
                .find_map(|measurement| {
                    measurement
                        .units
                        .as_ref()
                        .and_then(|units| units.value.symbol())
                })
                .unwrap_or_default()
                .to_owned();
            checks.push(RegressionCheck {
                target: crate::state::RegressionTargetSelector {
                    source_domain: baseline_provenance.source_domain(),
                    source_instance_id: baseline_provenance.source_instance_id(),
                    kind: crate::state::RegressionTargetKind::Measurement,
                    name: name.clone(),
                    occurrence,
                },
                name,
                source_identity: format!(
                    "{} · revision {} → {}",
                    baseline_provenance.source_instance_id(),
                    baseline_provenance.source_revision().get(),
                    current_provenance.source_revision().get()
                ),
                baseline: baseline_value,
                current: current_value,
                unit,
            });
        }
    }
    checks.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| left.source_identity.cmp(&right.source_identity))
    });
    checks
}

pub(super) fn regression_run_pair<'m>(
    model: &'m Model,
    simulate: &SimulateState,
) -> Option<(
    &'m crate::state::SimulationRun,
    &'m crate::state::SimulationRun,
)> {
    let plan_id = model
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::id)?;
    let current = model.simulation.active_run()?;
    validate_regression_run_for_plan(current, plan_id).ok()?;
    let selected = active_regression_baseline(model, simulate).and_then(|id| {
        model
            .simulation
            .retained
            .runs
            .iter()
            .find(|run| run.run_id == id)
    });
    let baseline = selected?;
    (baseline.run_id != current.run_id
        && validate_regression_run_for_plan(baseline, plan_id).is_ok())
    .then_some((baseline, current))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RegressionRunSeal {
    pub(super) content_digest: crate::product::ContentDigest,
    pub(super) authority_digest: crate::product::ContentDigest,
}

pub(super) fn digest_bytes(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

pub(super) fn digest_text(hasher: &mut Sha256, value: &str) {
    digest_bytes(hasher, value.as_bytes());
}

pub(super) fn digest_optional_text(hasher: &mut Sha256, value: Option<&str>) {
    hasher.update([u8::from(value.is_some())]);
    if let Some(value) = value {
        digest_text(hasher, value);
    }
}

pub(super) fn digest_optional_f64(hasher: &mut Sha256, value: Option<f64>) {
    hasher.update([u8::from(value.is_some())]);
    if let Some(value) = value {
        hasher.update(value.to_bits().to_le_bytes());
    }
}

pub(super) fn regression_authority_digest(
    receipt: &crate::state::PreparedRunReceipt,
) -> crate::product::ContentDigest {
    let mut digest = Sha256::new();
    digest.update(b"rspice-regression-run-authority-v1\0");
    digest.update([match receipt.source_domain() {
        crate::state::AnalysisResultSourceDomain::SimulationPlan => 0,
        crate::state::AnalysisResultSourceDomain::ManualDeck => 1,
        crate::state::AnalysisResultSourceDomain::LegacyUnclassified => 2,
    }]);
    match receipt.simulation_plan_id() {
        Some(id) => {
            digest.update([1]);
            digest.update(id.as_uuid().as_bytes());
        }
        None => digest.update([0]),
    }
    digest.update(receipt.project_revision().get().to_le_bytes());
    digest.update(receipt.prepared_snapshot_digest().as_bytes());
    digest.update(receipt.source_content_digest().as_bytes());
    digest.update(receipt.source_check_receipt().digest().as_bytes());
    digest.update((receipt.tasks().len() as u64).to_le_bytes());
    for task in receipt.tasks() {
        digest.update(task.instance_id().as_uuid().as_bytes());
        digest.update(task.source_revision().get().to_le_bytes());
        digest.update([task.analysis_kind_tag()]);
        digest.update(task.config_digest().as_bytes());
        digest.update((task.dependencies().len() as u64).to_le_bytes());
        for dependency in task.dependencies() {
            digest.update(dependency.as_uuid().as_bytes());
        }
    }
    crate::product::ContentDigest::from_bytes(digest.finalize().into())
}

pub(super) fn regression_content_digest(
    run: &crate::state::SimulationRun,
) -> crate::product::ContentDigest {
    let mut digest = Sha256::new();
    digest.update(b"rspice-regression-result-content-v1\0");
    digest.update((run.analyses.len() as u64).to_le_bytes());
    for analysis in &run.analyses {
        digest.update(analysis.id.to_le_bytes());
        digest.update([u8::from(analysis.success)]);
        digest_text(&mut digest, &analysis.label);
        digest_optional_text(&mut digest, analysis.error_message.as_deref());
        if let Some(provenance) = &analysis.provenance {
            digest.update([1]);
            digest.update([match provenance.source_domain() {
                crate::state::AnalysisResultSourceDomain::SimulationPlan => 0,
                crate::state::AnalysisResultSourceDomain::ManualDeck => 1,
                crate::state::AnalysisResultSourceDomain::LegacyUnclassified => 2,
            }]);
            digest.update(provenance.source_instance_id().as_uuid().as_bytes());
            digest.update(provenance.source_revision().get().to_le_bytes());
            digest.update(provenance.prepared_snapshot_digest().as_bytes());
        } else {
            digest.update([0]);
        }
        digest.update((analysis.measurements.len() as u64).to_le_bytes());
        for measurement in &analysis.measurements {
            digest_text(&mut digest, &measurement.name);
            digest_optional_f64(&mut digest, measurement.value);
            digest_optional_text(&mut digest, measurement.error.as_deref());
            digest.update([u8::from(measurement.passed)]);
            digest_optional_f64(&mut digest, measurement.expected);
            digest_optional_f64(&mut digest, measurement.tolerance);
        }
        digest.update((analysis.waveforms.len() as u64).to_le_bytes());
        for waveform in &analysis.waveforms {
            digest_text(&mut digest, &waveform.name);
            digest.update((waveform.x.len() as u64).to_le_bytes());
            for value in waveform.x.iter() {
                digest.update(value.to_bits().to_le_bytes());
            }
            digest.update((waveform.y.len() as u64).to_le_bytes());
            for value in waveform.y.iter() {
                digest.update(value.to_bits().to_le_bytes());
            }
            match &waveform.complex {
                Some(complex) => {
                    digest.update([1]);
                    digest_text(&mut digest, &complex.source_name);
                    digest.update((complex.real.len() as u64).to_le_bytes());
                    for value in complex.real.iter() {
                        digest.update(value.to_bits().to_le_bytes());
                    }
                    digest.update((complex.imag.len() as u64).to_le_bytes());
                    for value in complex.imag.iter() {
                        digest.update(value.to_bits().to_le_bytes());
                    }
                }
                None => digest.update([0]),
            }
        }
    }
    crate::product::ContentDigest::from_bytes(digest.finalize().into())
}

pub(super) fn validate_regression_run(
    run: &crate::state::SimulationRun,
) -> Result<&crate::state::PreparedRunReceipt, String> {
    let receipt = run
        .prepared_receipt()
        .ok_or_else(|| "dataset has no current prepared-run authority".to_owned())?;
    run.validate_provenance()?;
    if run.analyses.len() != receipt.tasks().len() {
        return Err(format!(
            "dataset is incomplete: {} of {} tasks produced results",
            run.analyses.len(),
            receipt.tasks().len()
        ));
    }
    if let Some((index, analysis)) = run
        .analyses
        .iter()
        .enumerate()
        .find(|(_, analysis)| !analysis.success)
    {
        return Err(format!(
            "dataset task {} did not complete successfully: {}",
            index + 1,
            analysis
                .error_message
                .as_deref()
                .unwrap_or("no failure detail retained")
        ));
    }
    Ok(receipt)
}

/// Validate both result completeness and the plan authority required by a
/// golden-regression contract. A complete manual-deck run or another plan's
/// run is still valid engineering data, but it is not a candidate or baseline
/// for this plan.
pub(super) fn validate_regression_run_for_plan(
    run: &crate::state::SimulationRun,
    plan_id: crate::product::SimulationPlanId,
) -> Result<&crate::state::PreparedRunReceipt, String> {
    let receipt = validate_regression_run(run)?;
    if receipt.source_domain() != crate::state::AnalysisResultSourceDomain::SimulationPlan
        || receipt.simulation_plan_id() != Some(plan_id)
    {
        return Err(format!(
            "dataset authority belongs to {}, not active simulation plan {plan_id}",
            receipt.simulation_plan_id().map_or_else(
                || match receipt.source_domain() {
                    crate::state::AnalysisResultSourceDomain::SimulationPlan => {
                        "an unidentified simulation plan".to_owned()
                    }
                    crate::state::AnalysisResultSourceDomain::ManualDeck => {
                        "a manual deck".to_owned()
                    }
                    crate::state::AnalysisResultSourceDomain::LegacyUnclassified => {
                        "an unclassified legacy source".to_owned()
                    }
                },
                |owner| format!("simulation plan {owner}")
            )
        ));
    }
    Ok(receipt)
}

pub(super) fn regression_run_seal(
    run: &crate::state::SimulationRun,
) -> Result<RegressionRunSeal, String> {
    let receipt = validate_regression_run(run)?;
    Ok(RegressionRunSeal {
        content_digest: regression_content_digest(run),
        authority_digest: regression_authority_digest(receipt),
    })
}

pub(super) fn active_regression_baseline(
    model: &Model,
    simulate: &SimulateState,
) -> Option<crate::product::RunId> {
    let plan_id = model
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::id)?;
    model
        .workspace
        .content
        .plan_data(plan_id)
        .and_then(|payload| payload.regression_baseline_run)
        .or(simulate.verification.regression_baseline_run)
}

/// Whether the active plan can take `run_id` as its regression baseline
/// against the active run, and that plan when it can: the run is retained,
/// it and the active run are complete runs of the plan, they are two runs,
/// and they share a measurement or a waveform to compare. The baseline
/// picker offers exactly the runs this accepts; a commit refuses the rest
/// with its reason.
pub(super) fn regression_baseline_eligibility(
    model: &Model,
    run_id: crate::product::RunId,
) -> Result<crate::product::SimulationPlanId, String> {
    let plan_id = model
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::id)
        .ok_or_else(|| "the active simulation plan is unavailable".to_owned())?;
    let selected = model
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.run_id == run_id)
        .ok_or_else(|| format!("retained run {run_id} no longer exists"))?;
    validate_regression_run_for_plan(selected, plan_id)
        .map_err(|error| format!("retained run {run_id} is not an eligible baseline: {error}"))?;
    let candidate = model
        .simulation
        .active_run()
        .ok_or_else(|| "no active candidate dataset is selected".to_owned())?;
    validate_regression_run_for_plan(candidate, plan_id)
        .map_err(|error| format!("active candidate dataset is not complete: {error}"))?;
    if candidate.run_id == run_id {
        return Err("the active candidate cannot also be its own baseline".to_owned());
    }
    if derive_regression_checks(selected, candidate).is_empty()
        && regression_waveform_pairs(selected, candidate).is_empty()
    {
        return Err(
            "baseline and candidate contain no common valid measurement or waveform target"
                .to_owned(),
        );
    }
    Ok(plan_id)
}

pub(super) fn commit_regression_baseline(
    cx: &mut Cx<'_, SimulateFeature>,
    run_id: crate::product::RunId,
) -> Result<(), String> {
    let plan_id = regression_baseline_eligibility(cx.model, run_id)?;

    let mut workspace = cx.model.workspace.clone();
    let mut setup = cx.model.sim_setup.clone();
    let mut simulation = cx.model.simulation.clone();
    workspace
        .content
        .ensure_active_plan_data(plan_id)
        .regression_baseline_run = Some(run_id);
    if !simulation.set_run_retention(run_id, crate::state::RunRetention::GoldenBaseline) {
        return Err(format!(
            "retained run {run_id} disappeared before its baseline classification could be committed"
        ));
    }
    setup
        .commit_active_plan_configuration_change(format!("chose run {run_id} as the reference"))
        .map_err(|error| error.to_string())?;
    cx.model.workspace = workspace;
    cx.model.sim_setup = setup;
    cx.model.simulation = simulation;
    invalidate_plan_bound_preflight(cx);
    cx.state.verification.regression_baseline_run = Some(run_id);
    cx.state.verification.reference_edit = None;
    Ok(())
}

pub(super) fn format_regression_window(
    window: Option<crate::state::RegressionComparisonWindow>,
) -> String {
    window.map_or_else(
        || "full domain".to_owned(),
        |window| {
            format!(
                "{} … {}",
                crate::simulation::dialog::options::format_si_value(window.start),
                crate::simulation::dialog::options::format_si_value(window.end)
            )
        },
    )
}

/// A tolerance rule in the words its Reference cell states it, and a reader
/// enters it: `abs 10m + rel 2 %` allows the sum of an absolute and a
/// relative change, `max(abs 10m, rel 2 %)` the larger of the two at each
/// point, and a waveform's rule adds the time skew it allows and the window it
/// is compared over when it sets them: `abs 20m + rel 2 % · skew 20n · 0 … 3u`.
pub(super) fn format_tolerance_rule(rule: &crate::state::RegressionToleranceRule) -> String {
    let relative = format!("{:.6}", rule.relative_tolerance * 100.0);
    let relative = format!(
        "rel {} %",
        relative.trim_end_matches('0').trim_end_matches('.')
    );
    let absolute = format!(
        "abs {}",
        crate::simulation::dialog::options::format_si_value(rule.absolute_tolerance)
    );
    let mut text = match rule.method {
        crate::state::RegressionComparisonMethod::PointwiseRelative => {
            format!("max({absolute}, {relative})")
        }
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope => {
            if rule.relative_tolerance == 0.0 {
                absolute
            } else if rule.absolute_tolerance == 0.0 {
                relative
            } else {
                format!("{absolute} + {relative}")
            }
        }
    };
    if rule.target.kind == crate::state::RegressionTargetKind::Waveform {
        if rule.time_skew_allowance > 0.0 {
            text.push_str(&format!(
                " \u{00b7} skew {}",
                crate::simulation::dialog::options::format_si_value(rule.time_skew_allowance)
            ));
        }
        if rule.comparison_window.is_some() {
            text.push_str(&format!(
                " \u{00b7} {}",
                format_regression_window(rule.comparison_window)
            ));
        }
    }
    text
}

pub(super) fn parse_regression_window(
    text: &str,
) -> Result<Option<crate::state::RegressionComparisonWindow>, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("full domain") {
        return Ok(None);
    }
    let bounds = trimmed
        .split_once('…')
        .or_else(|| trimmed.split_once(".."))
        .ok_or_else(|| "comparison window must be 'start … end' or 'full domain'".to_owned())?;
    let parse = |value: &str| {
        crate::simulation::dialog::options::parse_si_value(value.trim())
            .map_err(|error| format!("invalid comparison-window bound: {error}"))
    };
    let window = crate::state::RegressionComparisonWindow {
        start: parse(bounds.0)?,
        end: parse(bounds.1)?,
    };
    if window.start > window.end {
        return Err("comparison-window start must not exceed its end".to_owned());
    }
    Ok(Some(window))
}

/// `text` without `word` in front of it, ignoring case.
fn strip_word<'t>(text: &'t str, word: &str) -> Option<&'t str> {
    text.get(..word.len())
        .filter(|head| head.eq_ignore_ascii_case(word))
        .map(|_| &text[word.len()..])
}

/// The terms of a sum, split on each `+` that is not an exponent's sign.
fn sum_terms(text: &str) -> Vec<&str> {
    let bytes = text.as_bytes();
    let mut terms = Vec::new();
    let mut start = 0;
    for (index, byte) in bytes.iter().enumerate() {
        let exponent = index >= 2
            && matches!(bytes[index - 1], b'e' | b'E')
            && bytes[index - 2].is_ascii_digit();
        if *byte == b'+' && !exponent {
            terms.push(&text[start..index]);
            start = index + 1;
        }
    }
    terms.push(&text[start..]);
    terms
}

/// The rule a reader entered in a Reference tolerance cell, in the words
/// [`format_tolerance_rule`] states one in; `None` for an empty cell, which
/// leaves the target without a rule. A bare number is an absolute allowance
/// and a number with `%` a relative one, so `10m + 2 %` reads as
/// `abs 10m + rel 2 %`.
pub(super) fn parse_tolerance_rule(
    text: &str,
    target: &crate::state::RegressionTargetSelector,
) -> Result<Option<crate::state::RegressionToleranceRule>, String> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    let nonnegative = |text: &str, label: &str| {
        let value = crate::simulation::dialog::options::parse_si_value(text.trim())
            .map_err(|error| format!("the {label} is not a value: {error}"))?;
        if !value.is_finite() || value < 0.0 {
            Err(format!("the {label} must be a finite value of at least 0"))
        } else {
            Ok(value)
        }
    };
    let mut parts = text.split('\u{00b7}').map(str::trim);
    let allowance = parts.next().unwrap_or_default();
    let mut rule = crate::state::RegressionToleranceRule {
        target: target.clone(),
        method: crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope,
        absolute_tolerance: 0.0,
        relative_tolerance: 0.0,
        time_skew_allowance: 0.0,
        comparison_window: None,
    };
    let terms = match strip_word(allowance, "max(").and_then(|inner| inner.strip_suffix(')')) {
        Some(inner) => {
            rule.method = crate::state::RegressionComparisonMethod::PointwiseRelative;
            inner.split(',').collect()
        }
        None => sum_terms(allowance),
    };
    let (mut absolute, mut relative) = (None, None);
    for term in terms {
        let term = term.trim();
        if term.is_empty() {
            return Err("a term of the tolerance is empty".to_owned());
        }
        let (is_relative, value) = if let Some(value) = strip_word(term, "rel") {
            (true, value.trim())
        } else if let Some(value) = strip_word(term, "abs") {
            (false, value.trim())
        } else {
            (term.ends_with('%'), term)
        };
        if is_relative {
            let value = value.strip_suffix('%').unwrap_or(value);
            let percent = nonnegative(value, "relative tolerance")?;
            if relative.replace(percent / 100.0).is_some() {
                return Err("the relative tolerance is given twice".to_owned());
            }
        } else {
            if value.ends_with('%') {
                return Err("an absolute tolerance is not a percentage".to_owned());
            }
            if absolute
                .replace(nonnegative(value, "absolute tolerance")?)
                .is_some()
            {
                return Err("the absolute tolerance is given twice".to_owned());
            }
        }
    }
    rule.absolute_tolerance = absolute.unwrap_or(0.0);
    rule.relative_tolerance = relative.unwrap_or(0.0);
    for part in parts {
        if target.kind == crate::state::RegressionTargetKind::Measurement {
            return Err("a measurement's tolerance has no time skew or window".to_owned());
        }
        match strip_word(part, "skew") {
            Some(skew) => rule.time_skew_allowance = nonnegative(skew, "time skew")?,
            None => rule.comparison_window = parse_regression_window(part)?,
        }
    }
    rule.validate()?;
    Ok(Some(rule))
}

/// Make `rule` the tolerance the active plan holds for `target`, or leave the
/// target without one when `rule` is `None`: one plan commit, and the line
/// its receipt states; `None` when the plan already held exactly that.
pub(super) fn commit_regression_tolerance(
    cx: &mut Cx<'_, SimulateFeature>,
    target: &crate::state::RegressionTargetSelector,
    rule: Option<crate::state::RegressionToleranceRule>,
    name: &str,
) -> Result<Option<String>, String> {
    let plan = cx
        .model
        .sim_setup
        .analysis_plan
        .as_ref()
        .ok_or_else(|| "the active simulation plan is unavailable".to_owned())?;
    if target.source_domain == crate::state::AnalysisResultSourceDomain::SimulationPlan
        && plan.instance(target.source_instance_id).is_none()
    {
        return Err(format!(
            "its analysis {} is no longer in the plan",
            target.source_instance_id
        ));
    }
    let plan_id = plan.id();
    let detail = rule.as_ref().map_or_else(
        || format!("removed the tolerance of {name}"),
        |rule| {
            format!(
                "set the tolerance of {name} to {}",
                format_tolerance_rule(rule)
            )
        },
    );
    let mut workspace = cx.model.workspace.clone();
    let rules = &mut workspace
        .content
        .ensure_active_plan_data(plan_id)
        .regression_tolerances;
    match (rules.iter().position(|held| held.target == *target), rule) {
        (Some(index), Some(rule)) if rules[index] == rule => return Ok(None),
        (Some(index), Some(rule)) => rules[index] = rule,
        (Some(index), None) => {
            rules.remove(index);
        }
        (None, Some(rule)) => rules.push(rule),
        (None, None) => return Ok(None),
    }
    workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let mut setup = cx.model.sim_setup.clone();
    let receipt = setup
        .commit_active_plan_configuration_change(detail)
        .map_err(|error| error.to_string())?;
    cx.model.workspace = workspace;
    cx.model.sim_setup = setup;
    invalidate_plan_bound_preflight(cx);
    Ok(Some(receipt.status_line()))
}

pub(super) fn remove_orphaned_regression_rules(
    cx: &mut Cx<'_, SimulateFeature>,
    orphaned: &[crate::state::RegressionTargetSelector],
) -> Result<usize, String> {
    if orphaned.is_empty() {
        return Err("no orphaned regression tolerance is available to remove".to_owned());
    }
    let plan_id = cx
        .model
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::id)
        .ok_or_else(|| "the active simulation plan is unavailable".to_owned())?;
    let mut workspace = cx.model.workspace.clone();
    let payload = workspace.content.ensure_active_plan_data(plan_id);
    let prior = payload.regression_tolerances.len();
    payload
        .regression_tolerances
        .retain(|rule| !orphaned.contains(&rule.target));
    let removed = prior - payload.regression_tolerances.len();
    if removed == 0 {
        return Err("the orphaned tolerance contract changed before removal".to_owned());
    }
    workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let mut setup = cx.model.sim_setup.clone();
    let receipt = setup
        .commit_active_plan_configuration_change(if removed == 1 {
            "deleted a tolerance rule whose target neither run holds"
        } else {
            "deleted the tolerance rules whose targets neither run holds"
        })
        .map_err(|error| error.to_string())?;
    cx.model.workspace = workspace;
    cx.model.sim_setup = setup;
    invalidate_plan_bound_preflight(cx);
    cx.model.record_plan_receipt(receipt.status_line());
    cx.state.verification.reference_edit = None;
    Ok(removed)
}

/// One run compared against its reference under the plan's tolerances: every
/// measurement and waveform the two share, each one's verdict, and what could
/// not be compared.
pub(super) struct RegressionComparison<'a> {
    pub(super) checks: Vec<RegressionCheck>,
    pub(super) check_verdicts: Vec<RegressionVerdict>,
    pub(super) waveforms: Vec<RegressionWaveformPair<'a>>,
    pub(super) waveform_verdicts: Vec<RegressionVerdict>,
    pub(super) coverage_issues: Vec<RegressionCoverageIssue>,
}

/// Compare `current` against `baseline` under `rules`; the waveforms only
/// when the plan's policy compares them.
pub(super) fn compare_regression_runs<'a>(
    baseline: &'a crate::state::SimulationRun,
    current: &'a crate::state::SimulationRun,
    rules: &[crate::state::RegressionToleranceRule],
    policy: crate::state::RegressionSpecificationPolicy,
) -> RegressionComparison<'a> {
    let checks = derive_regression_checks(baseline, current);
    let waveforms = if regression_requires_waveforms(policy) {
        regression_waveform_pairs(baseline, current)
    } else {
        Vec::new()
    };
    let check_verdicts = checks
        .iter()
        .map(|check| evaluate_regression_check(check, regression_rule(rules, &check.target)))
        .collect();
    let waveform_verdicts = waveforms
        .iter()
        .map(|pair| evaluate_regression_waveform(pair, regression_rule(rules, &pair.target)))
        .collect();
    RegressionComparison {
        checks,
        check_verdicts,
        waveform_verdicts,
        waveforms,
        coverage_issues: regression_coverage_issues_for_policy(baseline, current, rules, policy),
    }
}

impl RegressionComparison<'_> {
    /// The receipt of this comparison: the plan revision and tolerances it
    /// was made under, both runs sealed by their content and authority, and
    /// how it came out.
    pub(super) fn receipt(
        &self,
        plan_id: crate::product::SimulationPlanId,
        plan_revision: crate::product::ObjectRevision,
        rules: &[crate::state::RegressionToleranceRule],
        baseline: &crate::state::SimulationRun,
        current: &crate::state::SimulationRun,
    ) -> Result<super::super::super::state::RegressionComparisonReceipt, String> {
        let baseline_seal = regression_run_seal(baseline)?;
        let candidate_seal = regression_run_seal(current)?;
        let count = |verdicts: &[RegressionVerdict], keep: fn(&RegressionVerdict) -> bool| {
            verdicts.iter().filter(|verdict| keep(verdict)).count()
        };
        let both = |keep: fn(&RegressionVerdict) -> bool| {
            count(&self.check_verdicts, keep) + count(&self.waveform_verdicts, keep)
        };
        Ok(super::super::super::state::RegressionComparisonReceipt {
            plan_id,
            plan_revision,
            tolerance_digest: regression_tolerance_digest(rules),
            baseline_run: baseline.run_id,
            candidate_run: current.run_id,
            baseline_dataset: baseline.dataset_id,
            candidate_dataset: current.dataset_id,
            baseline_content_digest: baseline_seal.content_digest,
            candidate_content_digest: candidate_seal.content_digest,
            baseline_authority_digest: baseline_seal.authority_digest,
            candidate_authority_digest: candidate_seal.authority_digest,
            aligned_checks: self.checks.len(),
            aligned_waveforms: self.waveforms.len(),
            changed_checks: self.checks.iter().filter(|check| check.changed()).count(),
            passed_checks: count(&self.check_verdicts, RegressionVerdict::passed),
            failed_checks: count(&self.check_verdicts, RegressionVerdict::failed),
            passed_waveforms: count(&self.waveform_verdicts, RegressionVerdict::passed),
            failed_waveforms: count(&self.waveform_verdicts, RegressionVerdict::failed),
            unconfigured_targets: both(|verdict| {
                matches!(verdict, RegressionVerdict::NotConfigured)
            }),
            unevaluated_targets: both(|verdict| {
                matches!(verdict, RegressionVerdict::NotEvaluated(_))
            }) + self.coverage_issues.len(),
        })
    }

    /// The comparison as the cases the CI documents list, each quoting the
    /// tolerance it was judged by.
    pub(super) fn export_cases(
        &self,
        rules: &[crate::state::RegressionToleranceRule],
    ) -> Vec<RegressionExportCase> {
        regression_export_cases(
            &self.checks,
            &self.check_verdicts,
            &self.waveforms,
            &self.waveform_verdicts,
            &self.coverage_issues,
            rules,
        )
    }
}

#[derive(Debug, Clone)]
pub(super) struct RegressionWaveformPair<'a> {
    pub(super) target: crate::state::RegressionTargetSelector,
    pub(super) baseline: &'a crate::state::RetainedTrace,
    pub(super) current: &'a crate::state::RetainedTrace,
    /// The analysis both waveforms came out of, which is what names the
    /// quantity their shared sweep axis is in.
    pub(super) analysis_type: crate::state::AnalysisType,
}

pub(super) fn regression_waveform_pairs<'a>(
    baseline: &'a crate::state::SimulationRun,
    current: &'a crate::state::SimulationRun,
) -> Vec<RegressionWaveformPair<'a>> {
    let mut aligned = Vec::new();
    for baseline_analysis in baseline
        .analyses
        .iter()
        .filter(|analysis| verified_analysis(analysis))
    {
        let baseline_provenance = baseline_analysis
            .provenance
            .as_ref()
            .expect("verified analyses retain provenance");
        let Some(current_analysis) = current.analyses.iter().find(|analysis| {
            verified_analysis(analysis)
                && analysis.provenance.as_ref().is_some_and(|provenance| {
                    provenance.source_domain() == baseline_provenance.source_domain()
                        && provenance.source_instance_id()
                            == baseline_provenance.source_instance_id()
                })
        }) else {
            continue;
        };
        for (index, baseline_waveform) in baseline_analysis.waveforms.iter().enumerate() {
            let occurrence = baseline_analysis.waveforms[..index]
                .iter()
                .filter(|waveform| waveform.name.eq_ignore_ascii_case(&baseline_waveform.name))
                .count();
            if let Some(current_waveform) = current_analysis
                .waveforms
                .iter()
                .filter(|waveform| waveform.name.eq_ignore_ascii_case(&baseline_waveform.name))
                .nth(occurrence)
                .filter(|waveform| {
                    validate_regression_waveform_data(baseline_waveform).is_ok()
                        && validate_regression_waveform_data(waveform).is_ok()
                })
            {
                let Ok(occurrence) = u32::try_from(occurrence) else {
                    continue;
                };
                aligned.push(RegressionWaveformPair {
                    target: crate::state::RegressionTargetSelector {
                        source_domain: baseline_provenance.source_domain(),
                        source_instance_id: baseline_provenance.source_instance_id(),
                        kind: crate::state::RegressionTargetKind::Waveform,
                        name: baseline_waveform.name.to_ascii_lowercase(),
                        occurrence,
                    },
                    baseline: baseline_waveform,
                    current: current_waveform,
                    analysis_type: baseline_analysis.analysis_type,
                });
            }
        }
    }
    aligned
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum RegressionVerdict {
    Pass {
        worst_delta: f64,
        allowed_delta: f64,
    },
    Fail {
        worst_delta: f64,
        allowed_delta: f64,
        detail: String,
    },
    NotConfigured,
    NotEvaluated(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RegressionExportDisposition {
    Pass,
    Failure,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RegressionExportCase {
    pub(super) name: String,
    pub(super) detail: String,
    pub(super) disposition: RegressionExportDisposition,
}

pub(super) fn regression_export_cases(
    checks: &[RegressionCheck],
    check_verdicts: &[RegressionVerdict],
    waveforms: &[RegressionWaveformPair<'_>],
    waveform_verdicts: &[RegressionVerdict],
    coverage_issues: &[RegressionCoverageIssue],
    rules: &[crate::state::RegressionToleranceRule],
) -> Vec<RegressionExportCase> {
    let source_domain = |domain| match domain {
        crate::state::AnalysisResultSourceDomain::SimulationPlan => "simulation_plan",
        crate::state::AnalysisResultSourceDomain::ManualDeck => "manual_deck",
        crate::state::AnalysisResultSourceDomain::LegacyUnclassified => "legacy_unclassified",
    };
    let verdict_case = |name: String,
                        target: &crate::state::RegressionTargetSelector,
                        verdict: &RegressionVerdict| {
        let (disposition, mut detail) = match verdict {
            RegressionVerdict::Pass {
                worst_delta,
                allowed_delta,
            } => (
                RegressionExportDisposition::Pass,
                format!("worst_delta={worst_delta:.17e}; allowed_delta={allowed_delta:.17e}"),
            ),
            RegressionVerdict::Fail {
                worst_delta,
                allowed_delta,
                detail,
            } => (
                RegressionExportDisposition::Failure,
                format!(
                    "{detail}; worst_delta={worst_delta:.17e}; allowed_delta={allowed_delta:.17e}"
                ),
            ),
            RegressionVerdict::NotConfigured => (
                RegressionExportDisposition::Blocked,
                "persisted tolerance is not configured".to_owned(),
            ),
            RegressionVerdict::NotEvaluated(detail) => {
                (RegressionExportDisposition::Blocked, detail.clone())
            }
        };
        if let Some(rule) = regression_rule(rules, target) {
            detail.push_str(&format!("; tolerance={}", format_tolerance_rule(rule)));
        }
        RegressionExportCase {
            name,
            detail,
            disposition,
        }
    };
    let mut cases = checks
        .iter()
        .zip(check_verdicts)
        .map(|(check, verdict)| {
            verdict_case(
                format!(
                    "measurement::{}::{}::{}[{}]",
                    source_domain(check.target.source_domain),
                    check.target.source_instance_id,
                    check.name,
                    check.target.occurrence
                ),
                &check.target,
                verdict,
            )
        })
        .chain(
            waveforms
                .iter()
                .zip(waveform_verdicts)
                .map(|(pair, verdict)| {
                    verdict_case(
                        format!(
                            "waveform::{}::{}::{}[{}]",
                            source_domain(pair.target.source_domain),
                            pair.target.source_instance_id,
                            pair.target.name,
                            pair.target.occurrence
                        ),
                        &pair.target,
                        verdict,
                    )
                }),
        )
        .collect::<Vec<_>>();
    cases.extend(coverage_issues.iter().map(|issue| RegressionExportCase {
        name: format!("coverage::{}", issue.label),
        detail: issue.detail.clone(),
        disposition: RegressionExportDisposition::Blocked,
    }));
    if cases.is_empty() {
        cases.push(RegressionExportCase {
            name: "coverage::no_comparable_targets".to_owned(),
            detail: "no comparable governed measurement or waveform target".to_owned(),
            disposition: RegressionExportDisposition::Blocked,
        });
    }
    cases
}

pub(super) fn xml_escape(value: &str) -> Result<String, String> {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        let legal_xml_1_0 = matches!(character, '\u{9}' | '\u{a}' | '\u{d}')
            || matches!(character as u32, 0x20..=0xd7ff | 0xe000..=0xfffd | 0x1_0000..=0x10_ffff);
        if !legal_xml_1_0 {
            return Err(format!(
                "XML 1.0 cannot represent U+{:04X}",
                character as u32
            ));
        }
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            character => escaped.push(character),
        }
    }
    Ok(escaped)
}

pub(super) fn tap_text(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character == '\n' || character == '\r' || character.is_control() {
                ' '
            } else {
                character
            }
        })
        .collect()
}

pub(super) fn regression_ci_documents(
    receipt: &super::super::super::state::RegressionComparisonReceipt,
    cases: &[RegressionExportCase],
) -> Result<(String, String), String> {
    if cases.is_empty() {
        return Err(
            "The CI export needs at least one passing, failing, or blocked case".to_owned(),
        );
    }
    let failures = cases
        .iter()
        .filter(|case| case.disposition == RegressionExportDisposition::Failure)
        .count();
    let blocked = cases
        .iter()
        .filter(|case| case.disposition == RegressionExportDisposition::Blocked)
        .count();
    let properties = [
        ("plan_id", receipt.plan_id.to_string()),
        ("plan_revision", receipt.plan_revision.get().to_string()),
        ("tolerance_digest", receipt.tolerance_digest.to_string()),
        ("baseline_run", receipt.baseline_run.to_string()),
        ("candidate_run", receipt.candidate_run.to_string()),
        ("baseline_dataset", receipt.baseline_dataset.to_string()),
        ("candidate_dataset", receipt.candidate_dataset.to_string()),
        (
            "baseline_content_digest",
            receipt.baseline_content_digest.to_string(),
        ),
        (
            "candidate_content_digest",
            receipt.candidate_content_digest.to_string(),
        ),
        (
            "baseline_authority_digest",
            receipt.baseline_authority_digest.to_string(),
        ),
        (
            "candidate_authority_digest",
            receipt.candidate_authority_digest.to_string(),
        ),
    ];
    let mut junit = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<testsuite name=\"RSpice Reference Comparison\" tests=\"{}\" failures=\"{failures}\" errors=\"{blocked}\">\n  <properties>\n",
        cases.len()
    );
    for (name, value) in &properties {
        junit.push_str(&format!(
            "    <property name=\"{}\" value=\"{}\"/>\n",
            xml_escape(name).map_err(|error| format!("invalid JUnit property name: {error}"))?,
            xml_escape(value)
                .map_err(|error| format!("invalid JUnit property '{name}': {error}"))?
        ));
    }
    junit.push_str("  </properties>\n");
    for case in cases {
        junit.push_str(&format!(
            "  <testcase classname=\"rspice.reference\" name=\"{}\">",
            xml_escape(&case.name)
                .map_err(|error| format!("invalid JUnit case name '{}': {error}", case.name))?
        ));
        match case.disposition {
            RegressionExportDisposition::Pass => {}
            RegressionExportDisposition::Failure => junit.push_str(&format!(
                "<failure message=\"regression tolerance exceeded\">{}</failure>",
                xml_escape(&case.detail).map_err(|error| format!(
                    "invalid JUnit case detail '{}': {error}",
                    case.name
                ))?
            )),
            RegressionExportDisposition::Blocked => junit.push_str(&format!(
                "<error message=\"regression evaluation blocked\">{}</error>",
                xml_escape(&case.detail).map_err(|error| format!(
                    "invalid JUnit case detail '{}': {error}",
                    case.name
                ))?
            )),
        }
        junit.push_str("</testcase>\n");
    }
    junit.push_str("</testsuite>\n");

    let mut tap = format!(
        "TAP version 13\n# plan_id={} plan_revision={}\n# tolerance_digest={}\n# baseline_run={} baseline_dataset={} baseline_content_digest={} baseline_authority_digest={}\n# candidate_run={} candidate_dataset={} candidate_content_digest={} candidate_authority_digest={}\n1..{}\n",
        receipt.plan_id,
        receipt.plan_revision.get(),
        receipt.tolerance_digest,
        receipt.baseline_run,
        receipt.baseline_dataset,
        receipt.baseline_content_digest,
        receipt.baseline_authority_digest,
        receipt.candidate_run,
        receipt.candidate_dataset,
        receipt.candidate_content_digest,
        receipt.candidate_authority_digest,
        cases.len()
    );
    for (index, case) in cases.iter().enumerate() {
        let ok = case.disposition == RegressionExportDisposition::Pass;
        tap.push_str(&format!(
            "{} {} - {}\n# {}\n",
            if ok { "ok" } else { "not ok" },
            index + 1,
            tap_text(&case.name),
            tap_text(&case.detail)
        ));
    }
    Ok((junit, tap))
}

pub(super) fn export_regression_ci(cx: &mut Cx<'_, SimulateFeature>, junit: &str, tap: &str) {
    use crate::workbench::workflows::export_workflow::SaveDialogConfig;

    let result: Result<(), String> = (|| {
        let Some(path) = cx.handles.export_io.show_save_dialog(SaveDialogConfig {
            title: "Export the Reference Comparison as JUnit and TAP",
            default_name: "rspice-reference-ci.zip",
            filter_name: "ZIP archive",
            filter_extensions: &["zip"],
        })?
        else {
            return Ok(());
        };
        let package = crate::workbench::workflows::export_workflow::deterministic_stored_zip(&[
            ("rspice-reference.xml", junit.as_bytes()),
            ("rspice-reference.tap", tap.as_bytes()),
        ])
        .map_err(|error| error.to_string())?;
        let destination = cx.handles.export_io.observe_destination(&path)?;
        cx.handles
            .export_io
            .write_bytes_file_observed(&destination, &package, "application/zip")
    })();
    cx.state.verification.action_problem = result
        .err()
        .map(|error| format!("JUnit / TAP export failed: {error}"));
}

impl RegressionVerdict {
    pub(super) fn passed(&self) -> bool {
        matches!(self, Self::Pass { .. })
    }

    pub(super) fn failed(&self) -> bool {
        matches!(self, Self::Fail { .. })
    }
}

pub(super) fn regression_rule<'a>(
    rules: &'a [crate::state::RegressionToleranceRule],
    target: &crate::state::RegressionTargetSelector,
) -> Option<&'a crate::state::RegressionToleranceRule> {
    rules.iter().find(|rule| rule.target == *target)
}

pub(super) fn regression_tolerance_digest(
    rules: &[crate::state::RegressionToleranceRule],
) -> crate::product::ContentDigest {
    let mut ordered = rules.iter().collect::<Vec<_>>();
    ordered.sort_by(|left, right| {
        let domain = |domain| match domain {
            crate::state::AnalysisResultSourceDomain::SimulationPlan => 0_u8,
            crate::state::AnalysisResultSourceDomain::ManualDeck => 1,
            crate::state::AnalysisResultSourceDomain::LegacyUnclassified => 2,
        };
        let kind = |kind| match kind {
            crate::state::RegressionTargetKind::Measurement => 0_u8,
            crate::state::RegressionTargetKind::Waveform => 1,
        };
        domain(left.target.source_domain)
            .cmp(&domain(right.target.source_domain))
            .then_with(|| {
                left.target
                    .source_instance_id
                    .as_uuid()
                    .as_bytes()
                    .cmp(right.target.source_instance_id.as_uuid().as_bytes())
            })
            .then_with(|| kind(left.target.kind).cmp(&kind(right.target.kind)))
            .then_with(|| left.target.name.cmp(&right.target.name))
            .then_with(|| left.target.occurrence.cmp(&right.target.occurrence))
    });
    let mut digest = Sha256::new();
    digest.update(b"rspice-regression-tolerance-v1\0");
    digest.update((ordered.len() as u64).to_le_bytes());
    let canonical_bits = |value: f64| if value == 0.0 { 0_u64 } else { value.to_bits() };
    for rule in ordered {
        digest.update([match rule.target.source_domain {
            crate::state::AnalysisResultSourceDomain::SimulationPlan => 0,
            crate::state::AnalysisResultSourceDomain::ManualDeck => 1,
            crate::state::AnalysisResultSourceDomain::LegacyUnclassified => 2,
        }]);
        digest.update(rule.target.source_instance_id.as_uuid().as_bytes());
        digest.update([match rule.target.kind {
            crate::state::RegressionTargetKind::Measurement => 0,
            crate::state::RegressionTargetKind::Waveform => 1,
        }]);
        digest.update((rule.target.name.len() as u64).to_le_bytes());
        digest.update(rule.target.name.as_bytes());
        digest.update(rule.target.occurrence.to_le_bytes());
        digest.update([match rule.method {
            crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope => 0,
            crate::state::RegressionComparisonMethod::PointwiseRelative => 1,
        }]);
        digest.update(canonical_bits(rule.absolute_tolerance).to_le_bytes());
        digest.update(canonical_bits(rule.relative_tolerance).to_le_bytes());
        digest.update(canonical_bits(rule.time_skew_allowance).to_le_bytes());
        match rule.comparison_window {
            Some(window) => {
                digest.update([1]);
                digest.update(canonical_bits(window.start).to_le_bytes());
                digest.update(canonical_bits(window.end).to_le_bytes());
            }
            None => digest.update([0]),
        }
    }
    crate::product::ContentDigest::from_bytes(digest.finalize().into())
}

pub(super) fn regression_receipt_matches_contract(
    receipt: &super::super::super::state::RegressionComparisonReceipt,
    plan_id: crate::product::SimulationPlanId,
    plan_revision: crate::product::ObjectRevision,
    tolerance_digest: crate::product::ContentDigest,
    baseline: &crate::state::SimulationRun,
    candidate: &crate::state::SimulationRun,
) -> bool {
    let Ok(baseline_seal) = regression_run_seal(baseline) else {
        return false;
    };
    let Ok(candidate_seal) = regression_run_seal(candidate) else {
        return false;
    };
    receipt.plan_id == plan_id
        && receipt.plan_revision == plan_revision
        && receipt.tolerance_digest == tolerance_digest
        && receipt.baseline_run == baseline.run_id
        && receipt.candidate_run == candidate.run_id
        && receipt.baseline_dataset == baseline.dataset_id
        && receipt.candidate_dataset == candidate.dataset_id
        && receipt.baseline_content_digest == baseline_seal.content_digest
        && receipt.candidate_content_digest == candidate_seal.content_digest
        && receipt.baseline_authority_digest == baseline_seal.authority_digest
        && receipt.candidate_authority_digest == candidate_seal.authority_digest
}

pub(super) fn permitted_delta(baseline: f64, rule: &crate::state::RegressionToleranceRule) -> f64 {
    let relative = rule.relative_tolerance * baseline.abs();
    match rule.method {
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope => {
            rule.absolute_tolerance + relative
        }
        crate::state::RegressionComparisonMethod::PointwiseRelative => {
            rule.absolute_tolerance.max(relative)
        }
    }
}

pub(super) fn evaluate_regression_check(
    check: &RegressionCheck,
    rule: Option<&crate::state::RegressionToleranceRule>,
) -> RegressionVerdict {
    let Some(rule) = rule else {
        return RegressionVerdict::NotConfigured;
    };
    if let Err(error) = rule.validate() {
        return RegressionVerdict::NotEvaluated(error);
    }
    let delta = check.delta().abs();
    let allowed = permitted_delta(check.baseline, rule);
    if delta <= allowed {
        RegressionVerdict::Pass {
            worst_delta: delta,
            allowed_delta: allowed,
        }
    } else {
        RegressionVerdict::Fail {
            worst_delta: delta,
            allowed_delta: allowed,
            detail: format!("absolute delta {delta:.6e} exceeds {allowed:.6e}"),
        }
    }
}

pub(super) fn strictly_increasing_finite(values: &[f64]) -> bool {
    !values.is_empty()
        && values.iter().all(|value| value.is_finite())
        && values.windows(2).all(|pair| pair[0] < pair[1])
}

pub(super) fn minimum_interpolated_delta(
    x: &[f64],
    y: &[f64],
    baseline_x: f64,
    baseline_y: f64,
    skew: f64,
    _window: Option<crate::state::RegressionComparisonWindow>,
) -> Option<(f64, f64)> {
    let left = baseline_x - skew;
    let right = baseline_x + skew;
    if left > right {
        return None;
    }
    if x.len() == 1 {
        return (x[0] >= left && x[0] <= right).then(|| ((y[0] - baseline_y).abs(), y[0]));
    }
    let mut best: Option<(f64, f64)> = None;
    for index in 0..x.len() - 1 {
        let segment_left = left.max(x[index]);
        let segment_right = right.min(x[index + 1]);
        if segment_left > segment_right {
            continue;
        }
        let span = x[index + 1] - x[index];
        let interpolate = |at: f64| y[index] + (y[index + 1] - y[index]) * ((at - x[index]) / span);
        for candidate in [segment_left, segment_right] {
            let matched = interpolate(candidate);
            let delta = (matched - baseline_y).abs();
            if best.is_none_or(|(current, _)| delta < current) {
                best = Some((delta, matched));
            }
        }
        let dy = y[index + 1] - y[index];
        if dy != 0.0 {
            let crossing = x[index] + (baseline_y - y[index]) * span / dy;
            if crossing >= segment_left && crossing <= segment_right {
                return Some((0.0, baseline_y));
            }
        }
    }
    best
}

pub(super) fn interpolate_waveform_at(
    waveform: &crate::state::RetainedTrace,
    at: f64,
) -> Option<f64> {
    let first = *waveform.x.first()?;
    let last = *waveform.x.last()?;
    if !at.is_finite() || at < first || at > last {
        return None;
    }
    match waveform.x.binary_search_by(|probe| probe.total_cmp(&at)) {
        Ok(index) => waveform.y.get(index).copied(),
        Err(upper) if upper > 0 && upper < waveform.x.len() => {
            let lower = upper - 1;
            let span = waveform.x[upper] - waveform.x[lower];
            Some(
                waveform.y[lower]
                    + (waveform.y[upper] - waveform.y[lower]) * ((at - waveform.x[lower]) / span),
            )
        }
        Err(_) => None,
    }
}

pub(super) fn waveform_evaluation_samples(
    waveform: &crate::state::RetainedTrace,
    window: Option<crate::state::RegressionComparisonWindow>,
) -> Result<Vec<(f64, f64)>, String> {
    let Some(window) = window else {
        return Ok(waveform
            .x
            .iter()
            .copied()
            .zip(waveform.y.iter().copied())
            .collect());
    };
    let mut x = vec![window.start, window.end];
    x.extend(
        waveform
            .x
            .iter()
            .copied()
            .filter(|sample| *sample > window.start && *sample < window.end),
    );
    x.sort_by(f64::total_cmp);
    x.dedup_by(|left, right| left.to_bits() == right.to_bits());
    x.into_iter()
        .map(|sample| {
            interpolate_waveform_at(waveform, sample)
                .map(|value| (sample, value))
                .ok_or_else(|| format!("comparison window boundary x={sample:.6e} is outside retained waveform coverage"))
        })
        .collect()
}

pub(super) fn evaluate_regression_waveform(
    pair: &RegressionWaveformPair<'_>,
    rule: Option<&crate::state::RegressionToleranceRule>,
) -> RegressionVerdict {
    let Some(rule) = rule else {
        return RegressionVerdict::NotConfigured;
    };
    if let Err(error) = rule.validate() {
        return RegressionVerdict::NotEvaluated(error);
    }
    if let Err(error) = validate_regression_waveform_data(pair.baseline) {
        return RegressionVerdict::NotEvaluated(format!("baseline {error}"));
    }
    if let Err(error) = validate_regression_waveform_data(pair.current) {
        return RegressionVerdict::NotEvaluated(format!("candidate {error}"));
    }
    let baseline = match waveform_evaluation_samples(pair.baseline, rule.comparison_window) {
        Ok(samples) => samples,
        Err(error) => return RegressionVerdict::NotEvaluated(format!("baseline {error}")),
    };
    let mut worst_delta = 0.0_f64;
    let mut allowed_at_worst = 0.0_f64;
    for (baseline_x, baseline_y) in baseline {
        let Some((delta, _)) = minimum_interpolated_delta(
            pair.current.x.as_slice(),
            pair.current.y.as_slice(),
            baseline_x,
            baseline_y,
            rule.time_skew_allowance,
            rule.comparison_window,
        ) else {
            return RegressionVerdict::Fail {
                worst_delta: f64::INFINITY,
                allowed_delta: permitted_delta(baseline_y, rule),
                detail: format!("no candidate coverage near x={baseline_x:.6e}"),
            };
        };
        let allowed = permitted_delta(baseline_y, rule);
        if delta >= worst_delta {
            worst_delta = delta;
            allowed_at_worst = allowed;
        }
        if delta > allowed {
            return RegressionVerdict::Fail {
                worst_delta: delta,
                allowed_delta: allowed,
                detail: format!("waveform envelope exceeded near x={baseline_x:.6e}"),
            };
        }
    }
    let candidate = match waveform_evaluation_samples(pair.current, rule.comparison_window) {
        Ok(samples) => samples,
        Err(error) => return RegressionVerdict::NotEvaluated(format!("candidate {error}")),
    };
    for (candidate_x, candidate_y) in candidate {
        let Some((delta, matched_baseline)) = minimum_interpolated_delta(
            pair.baseline.x.as_slice(),
            pair.baseline.y.as_slice(),
            candidate_x,
            candidate_y,
            rule.time_skew_allowance,
            rule.comparison_window,
        ) else {
            return RegressionVerdict::Fail {
                worst_delta: f64::INFINITY,
                allowed_delta: permitted_delta(candidate_y, rule),
                detail: format!("no baseline coverage near x={candidate_x:.6e}"),
            };
        };
        let allowed = permitted_delta(matched_baseline, rule);
        if delta >= worst_delta {
            worst_delta = delta;
            allowed_at_worst = allowed;
        }
        if delta > allowed {
            return RegressionVerdict::Fail {
                worst_delta: delta,
                allowed_delta: allowed,
                detail: format!("candidate excursion outside envelope near x={candidate_x:.6e}"),
            };
        }
    }
    RegressionVerdict::Pass {
        worst_delta,
        allowed_delta: allowed_at_worst,
    }
}
