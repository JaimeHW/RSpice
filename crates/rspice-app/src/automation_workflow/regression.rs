//! Deterministic, fail-closed waveform comparison for governed Automation runs.

use sha2::{Digest, Sha256};

use crate::product::ContentDigest;
use crate::state::{
    AnalysisResultSourceDomain, RegressionComparisonMethod, RegressionComparisonWindow,
    RegressionTargetKind, RegressionTargetSelector, RegressionToleranceRule, SimulationRun,
    WaveformData,
};

use super::ComparisonEvidence;

const MAX_RETAINED_ISSUES: usize = 64;
const MAX_RETAINED_DETAIL_BYTES: usize = 12 * 1024;
const MAX_RETAINED_ISSUE_BYTES: usize = 768;

#[derive(Clone)]
struct OwnedWaveformObservation<'run> {
    target: RegressionTargetSelector,
    waveform: &'run WaveformData,
}

/// Compare the complete union of baseline, candidate, and configured waveform
/// targets. Missing data, duplicate policy, invalid data, and absent policy are
/// all explicit release failures rather than silently skipped targets.
pub(crate) fn compare_governed_waveforms(
    baseline_name: &str,
    baseline: &SimulationRun,
    candidate: &SimulationRun,
    rules: &[RegressionToleranceRule],
) -> Result<ComparisonEvidence, String> {
    let baseline_observations = waveform_observations(baseline)?;
    let candidate_observations = waveform_observations(candidate)?;
    let mut targets = baseline_observations
        .iter()
        .map(|observation| observation.target.clone())
        .chain(
            candidate_observations
                .iter()
                .map(|observation| observation.target.clone()),
        )
        .chain(
            rules
                .iter()
                .filter(|rule| rule.target.kind == RegressionTargetKind::Waveform)
                .map(|rule| canonical_target(&rule.target)),
        )
        .collect::<Vec<_>>();
    targets.sort_by(compare_targets);
    targets.dedup();
    if targets.is_empty() {
        return Err(
            "The completed run, named baseline, and sealed policy contain no governed waveform targets."
                .to_owned(),
        );
    }

    let mut evaluated = 0_u64;
    let mut differing = 0_u64;
    let mut missing = 0_u64;
    let mut issues = Vec::new();
    for target in &targets {
        let baseline_observation = find_observation(&baseline_observations, target);
        let candidate_observation = find_observation(&candidate_observations, target);
        let matching_rules = rules
            .iter()
            .filter(|rule| {
                rule.target.kind == RegressionTargetKind::Waveform
                    && canonical_target(&rule.target) == *target
            })
            .collect::<Vec<_>>();
        let label = target_label(target);
        match (baseline_observation, candidate_observation) {
            (None, None) => {
                missing = missing.saturating_add(1);
                retain_issue(
                    &mut issues,
                    format!("{label}: configured target is absent from both datasets"),
                );
            }
            (None, Some(_)) => {
                missing = missing.saturating_add(1);
                retain_issue(
                    &mut issues,
                    format!("{label}: target is present only in the candidate dataset"),
                );
            }
            (Some(_), None) => {
                missing = missing.saturating_add(1);
                retain_issue(
                    &mut issues,
                    format!("{label}: target is missing from the candidate dataset"),
                );
            }
            (Some(reference), Some(current)) => {
                evaluated = evaluated.saturating_add(1);
                let verdict = match matching_rules.as_slice() {
                    [] => Err("persisted tolerance is not configured".to_owned()),
                    [rule] => evaluate_waveform(reference, current, rule),
                    _ => Err(format!(
                        "{} duplicate tolerance rules govern the same target",
                        matching_rules.len()
                    )),
                };
                if let Err(error) = verdict {
                    differing = differing.saturating_add(1);
                    retain_issue(&mut issues, format!("{label}: {error}"));
                }
            }
        }
    }

    let expected = u64::try_from(targets.len())
        .map_err(|_| "waveform target count exceeds the supported evidence range".to_owned())?;
    let issue_count = differing.saturating_add(missing);
    let omitted = usize::try_from(issue_count)
        .unwrap_or(usize::MAX)
        .saturating_sub(issues.len());
    let detail = if issues.is_empty() {
        format!("All {expected} governed waveform targets satisfied the sealed tolerance policy.")
    } else {
        let mut detail = issues.join("\n");
        if omitted != 0 {
            detail.push_str(&format!(
                "\n{omitted} additional comparison failures omitted."
            ));
        }
        detail
    };
    ComparisonEvidence::try_new_complete(
        baseline_name,
        expected,
        evaluated,
        differing,
        missing,
        regression_tolerance_digest(rules).to_string(),
        detail,
    )
    .map_err(|error| format!("Could not seal baseline comparison: {error}"))
}

fn waveform_observations(run: &SimulationRun) -> Result<Vec<OwnedWaveformObservation<'_>>, String> {
    let mut observations = Vec::new();
    for analysis in &run.analyses {
        let provenance = analysis.provenance.as_ref().ok_or_else(|| {
            format!(
                "analysis {:?} has no authenticated source identity",
                analysis.label
            )
        })?;
        for (index, waveform) in analysis.waveforms.iter().enumerate() {
            let occurrence = analysis.waveforms[..index]
                .iter()
                .filter(|prior| prior.name.eq_ignore_ascii_case(&waveform.name))
                .count();
            let occurrence = u32::try_from(occurrence).map_err(|_| {
                format!(
                    "analysis {:?} contains too many waveform occurrences named {:?}",
                    analysis.label, waveform.name
                )
            })?;
            observations.push(OwnedWaveformObservation {
                target: RegressionTargetSelector {
                    source_domain: provenance.source_domain(),
                    source_instance_id: provenance.source_instance_id(),
                    kind: RegressionTargetKind::Waveform,
                    name: waveform.name.to_ascii_lowercase(),
                    occurrence,
                },
                waveform,
            });
        }
    }
    Ok(observations)
}

fn find_observation<'run>(
    observations: &'run [OwnedWaveformObservation<'run>],
    target: &RegressionTargetSelector,
) -> Option<&'run WaveformData> {
    observations
        .iter()
        .find(|observation| observation.target == *target)
        .map(|observation| observation.waveform)
}

fn canonical_target(target: &RegressionTargetSelector) -> RegressionTargetSelector {
    let mut canonical = target.clone();
    canonical.name.make_ascii_lowercase();
    canonical
}

fn compare_targets(
    left: &RegressionTargetSelector,
    right: &RegressionTargetSelector,
) -> std::cmp::Ordering {
    domain_tag(left.source_domain)
        .cmp(&domain_tag(right.source_domain))
        .then_with(|| {
            left.source_instance_id
                .as_uuid()
                .as_bytes()
                .cmp(right.source_instance_id.as_uuid().as_bytes())
        })
        .then_with(|| kind_tag(left.kind).cmp(&kind_tag(right.kind)))
        .then_with(|| left.name.cmp(&right.name))
        .then_with(|| left.occurrence.cmp(&right.occurrence))
}

fn target_label(target: &RegressionTargetSelector) -> String {
    format!(
        "waveform::{:?}::{}::{}[{}]",
        target.source_domain, target.source_instance_id, target.name, target.occurrence
    )
}

fn retain_issue(issues: &mut Vec<String>, issue: String) {
    if issues.len() >= MAX_RETAINED_ISSUES {
        return;
    }
    let mut issue = issue;
    if issue.len() > MAX_RETAINED_ISSUE_BYTES {
        let mut boundary = MAX_RETAINED_ISSUE_BYTES;
        while !issue.is_char_boundary(boundary) {
            boundary -= 1;
        }
        issue.truncate(boundary);
        issue.push_str("...");
    }
    let retained_bytes = issues
        .iter()
        .map(String::len)
        .sum::<usize>()
        .saturating_add(issues.len());
    if retained_bytes.saturating_add(issue.len()) <= MAX_RETAINED_DETAIL_BYTES {
        issues.push(issue);
    }
}

fn validate_waveform(waveform: &WaveformData) -> Result<(), String> {
    if waveform.x.len() != waveform.y.len() {
        return Err(format!(
            "axis/value length mismatch ({} x values, {} y values)",
            waveform.x.len(),
            waveform.y.len()
        ));
    }
    if !strictly_increasing_finite(&waveform.x) {
        return Err("waveform axis is empty, non-finite, or not strictly increasing".to_owned());
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

fn evaluate_waveform(
    baseline: &WaveformData,
    candidate: &WaveformData,
    rule: &RegressionToleranceRule,
) -> Result<(), String> {
    rule.validate()
        .map_err(|error| format!("invalid persisted tolerance: {error}"))?;
    if rule.target.kind != RegressionTargetKind::Waveform {
        return Err("persisted tolerance does not govern a waveform".to_owned());
    }
    validate_waveform(baseline)
        .map_err(|error| format!("baseline evidence is invalid: {error}"))?;
    validate_waveform(candidate)
        .map_err(|error| format!("candidate evidence is invalid: {error}"))?;
    evaluate_series(
        &baseline.x,
        &baseline.y,
        &candidate.x,
        &candidate.y,
        rule,
        "display value",
    )?;
    match (&baseline.complex, &candidate.complex) {
        (None, None) => Ok(()),
        (Some(_), None) | (None, Some(_)) => {
            Err("baseline and candidate complex-component coverage differs".to_owned())
        }
        (Some(reference), Some(current)) => {
            if reference.source_name != current.source_name {
                return Err(format!(
                    "complex source identity changed from {:?} to {:?}",
                    reference.source_name, current.source_name
                ));
            }
            evaluate_series(
                &baseline.x,
                &reference.real,
                &candidate.x,
                &current.real,
                rule,
                "real component",
            )?;
            evaluate_series(
                &baseline.x,
                &reference.imag,
                &candidate.x,
                &current.imag,
                rule,
                "imaginary component",
            )
        }
    }
}

fn evaluate_series(
    baseline_x: &[f64],
    baseline_y: &[f64],
    candidate_x: &[f64],
    candidate_y: &[f64],
    rule: &RegressionToleranceRule,
    component: &str,
) -> Result<(), String> {
    let baseline_samples = evaluation_samples(baseline_x, baseline_y, rule.comparison_window)
        .map_err(|error| format!("baseline {component} {error}"))?;
    for (x, reference) in baseline_samples {
        let Some((delta, _)) = minimum_interpolated_delta(
            candidate_x,
            candidate_y,
            x,
            reference,
            rule.time_skew_allowance,
        ) else {
            return Err(format!(
                "{component} has no candidate coverage near x={x:.6e}"
            ));
        };
        let allowed = permitted_delta(reference, rule);
        if delta > allowed {
            return Err(format!(
                "{component} delta {delta:.6e} exceeds {allowed:.6e} near x={x:.6e}"
            ));
        }
    }
    let candidate_samples = evaluation_samples(candidate_x, candidate_y, rule.comparison_window)
        .map_err(|error| format!("candidate {component} {error}"))?;
    for (x, current) in candidate_samples {
        let Some((delta, matched_reference)) = minimum_interpolated_delta(
            baseline_x,
            baseline_y,
            x,
            current,
            rule.time_skew_allowance,
        ) else {
            return Err(format!(
                "{component} has no baseline coverage near x={x:.6e}"
            ));
        };
        let allowed = permitted_delta(matched_reference, rule);
        if delta > allowed {
            return Err(format!(
                "{component} candidate excursion {delta:.6e} exceeds {allowed:.6e} near x={x:.6e}"
            ));
        }
    }
    Ok(())
}

fn strictly_increasing_finite(values: &[f64]) -> bool {
    !values.is_empty()
        && values.iter().all(|value| value.is_finite())
        && values.windows(2).all(|pair| pair[0] < pair[1])
}

fn permitted_delta(baseline: f64, rule: &RegressionToleranceRule) -> f64 {
    let relative = rule.relative_tolerance * baseline.abs();
    match rule.method {
        RegressionComparisonMethod::AbsoluteRelativeEnvelope => rule.absolute_tolerance + relative,
        RegressionComparisonMethod::PointwiseRelative => rule.absolute_tolerance.max(relative),
    }
}

fn minimum_interpolated_delta(
    x: &[f64],
    y: &[f64],
    reference_x: f64,
    reference_y: f64,
    skew: f64,
) -> Option<(f64, f64)> {
    let left = reference_x - skew;
    let right = reference_x + skew;
    if left > right {
        return None;
    }
    if x.len() == 1 {
        return (x[0] >= left && x[0] <= right).then(|| ((y[0] - reference_y).abs(), y[0]));
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
        for candidate_x in [segment_left, segment_right] {
            let matched = interpolate(candidate_x);
            let delta = (matched - reference_y).abs();
            if best.is_none_or(|(current, _)| delta < current) {
                best = Some((delta, matched));
            }
        }
        let dy = y[index + 1] - y[index];
        if dy != 0.0 {
            let crossing = x[index] + (reference_y - y[index]) * span / dy;
            if crossing >= segment_left && crossing <= segment_right {
                return Some((0.0, reference_y));
            }
        }
    }
    best
}

fn evaluation_samples(
    x: &[f64],
    y: &[f64],
    window: Option<RegressionComparisonWindow>,
) -> Result<Vec<(f64, f64)>, String> {
    let Some(window) = window else {
        return Ok(x.iter().copied().zip(y.iter().copied()).collect());
    };
    let mut sample_x = vec![window.start, window.end];
    sample_x.extend(
        x.iter()
            .copied()
            .filter(|sample| *sample > window.start && *sample < window.end),
    );
    sample_x.sort_by(f64::total_cmp);
    sample_x.dedup_by(|left, right| left.to_bits() == right.to_bits());
    sample_x
        .into_iter()
        .map(|sample| {
            interpolate_at(x, y, sample)
                .map(|value| (sample, value))
                .ok_or_else(|| {
                    format!(
                        "comparison window boundary x={sample:.6e} is outside retained waveform coverage"
                    )
                })
        })
        .collect()
}

fn interpolate_at(x: &[f64], y: &[f64], at: f64) -> Option<f64> {
    let first = *x.first()?;
    let last = *x.last()?;
    if !at.is_finite() || at < first || at > last {
        return None;
    }
    match x.binary_search_by(|probe| probe.total_cmp(&at)) {
        Ok(index) => y.get(index).copied(),
        Err(upper) if upper > 0 && upper < x.len() => {
            let lower = upper - 1;
            let span = x[upper] - x[lower];
            Some(y[lower] + (y[upper] - y[lower]) * ((at - x[lower]) / span))
        }
        Err(_) => None,
    }
}

pub(crate) fn regression_tolerance_digest(rules: &[RegressionToleranceRule]) -> ContentDigest {
    let mut ordered = rules.iter().collect::<Vec<_>>();
    ordered.sort_by(|left, right| compare_targets(&left.target, &right.target));
    let mut digest = Sha256::new();
    digest.update(b"rspice-regression-tolerance-v1\0");
    digest.update((ordered.len() as u64).to_le_bytes());
    let canonical_bits = |value: f64| if value == 0.0 { 0_u64 } else { value.to_bits() };
    for rule in ordered {
        digest.update([domain_tag(rule.target.source_domain)]);
        digest.update(rule.target.source_instance_id.as_uuid().as_bytes());
        digest.update([kind_tag(rule.target.kind)]);
        digest.update((rule.target.name.len() as u64).to_le_bytes());
        digest.update(rule.target.name.as_bytes());
        digest.update(rule.target.occurrence.to_le_bytes());
        digest.update([match rule.method {
            RegressionComparisonMethod::AbsoluteRelativeEnvelope => 0,
            RegressionComparisonMethod::PointwiseRelative => 1,
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
    ContentDigest::from_bytes(digest.finalize().into())
}

const fn domain_tag(domain: AnalysisResultSourceDomain) -> u8 {
    match domain {
        AnalysisResultSourceDomain::SimulationPlan => 0,
        AnalysisResultSourceDomain::ManualDeck => 1,
        AnalysisResultSourceDomain::LegacyUnclassified => 2,
    }
}

const fn kind_tag(kind: RegressionTargetKind) -> u8 {
    match kind {
        RegressionTargetKind::Measurement => 0,
        RegressionTargetKind::Waveform => 1,
    }
}

#[cfg(test)]
mod tests {
    use crate::product::{AnalysisInstanceId, ObjectRevision};
    use crate::state::{AnalysisResult, AnalysisResultProvenance, AnalysisType};

    use super::*;

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    fn waveform(name: &str, y: Vec<f64>) -> WaveformData {
        WaveformData::new(name, vec![0.0, 1.0], y, "#ffffff")
    }

    fn run(instance_id: AnalysisInstanceId, waveforms: Vec<WaveformData>) -> SimulationRun {
        let provenance = AnalysisResultProvenance::new(
            instance_id,
            ObjectRevision::INITIAL,
            digest(1),
            Vec::new(),
        )
        .expect("valid provenance");
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_provenance(provenance);
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis.with_waveforms(waveforms));
        run
    }

    fn rule(
        instance_id: AnalysisInstanceId,
        name: &str,
        occurrence: u32,
        absolute_tolerance: f64,
    ) -> RegressionToleranceRule {
        RegressionToleranceRule {
            target: RegressionTargetSelector {
                source_domain: AnalysisResultSourceDomain::SimulationPlan,
                source_instance_id: instance_id,
                kind: RegressionTargetKind::Waveform,
                name: name.to_ascii_lowercase(),
                occurrence,
            },
            method: RegressionComparisonMethod::AbsoluteRelativeEnvelope,
            absolute_tolerance,
            relative_tolerance: 0.0,
            time_skew_allowance: 0.0,
            comparison_window: None,
        }
    }

    #[test]
    fn comparison_applies_the_sealed_tolerance_bidirectionally() {
        let id = AnalysisInstanceId::new();
        let baseline = run(id, vec![waveform("V(out)", vec![1.0, 2.0])]);
        let within = run(id, vec![waveform("V(out)", vec![1.05, 1.95])]);
        let outside = run(id, vec![waveform("V(out)", vec![1.2, 2.0])]);
        let rules = vec![rule(id, "V(out)", 0, 0.1)];

        let passed = compare_governed_waveforms("main", &baseline, &within, &rules)
            .expect("comparison evidence");
        assert!(passed.passed());
        assert_eq!(passed.evaluated_waveform_count(), 1);

        let failed = compare_governed_waveforms("main", &baseline, &outside, &rules)
            .expect("failed comparison remains evidence");
        assert!(!failed.passed());
        assert_eq!(failed.differing_waveform_count(), 1);
        assert!(failed.detail().contains("exceeds"));
    }

    #[test]
    fn missing_and_unconfigured_targets_fail_closed() {
        let id = AnalysisInstanceId::new();
        let baseline = run(id, vec![waveform("V(out)", vec![1.0, 2.0])]);
        let absent = run(id, Vec::new());
        let matching = run(id, vec![waveform("V(out)", vec![1.0, 2.0])]);
        let rules = vec![rule(id, "V(out)", 0, 0.0)];

        let missing = compare_governed_waveforms("main", &baseline, &absent, &rules)
            .expect("missing coverage remains evidence");
        assert_eq!(missing.waveform_count(), 1);
        assert_eq!(missing.evaluated_waveform_count(), 0);
        assert_eq!(missing.missing_waveform_count(), 1);
        assert!(!missing.passed());

        let unconfigured = compare_governed_waveforms("main", &baseline, &matching, &[])
            .expect("unconfigured coverage remains evidence");
        assert_eq!(unconfigured.differing_waveform_count(), 1);
        assert!(unconfigured.detail().contains("not configured"));
    }

    #[test]
    fn duplicate_names_are_bijective_by_occurrence() {
        let id = AnalysisInstanceId::new();
        let baseline = run(
            id,
            vec![
                waveform("V(out)", vec![1.0, 2.0]),
                waveform("v(OUT)", vec![3.0, 4.0]),
            ],
        );
        let candidate = run(id, vec![waveform("V(out)", vec![1.0, 2.0])]);
        let rules = vec![rule(id, "v(out)", 0, 0.0), rule(id, "v(out)", 1, 0.0)];

        let evidence = compare_governed_waveforms("main", &baseline, &candidate, &rules)
            .expect("coverage evidence");
        assert_eq!(evidence.waveform_count(), 2);
        assert_eq!(evidence.evaluated_waveform_count(), 1);
        assert_eq!(evidence.missing_waveform_count(), 1);
        assert!(!evidence.passed());
    }

    #[test]
    fn tolerance_digest_is_order_independent_but_semantic() {
        let first = AnalysisInstanceId::new();
        let second = AnalysisInstanceId::new();
        let a = rule(first, "v(a)", 0, 0.1);
        let b = rule(second, "v(b)", 0, 0.2);
        assert_eq!(
            regression_tolerance_digest(&[a.clone(), b.clone()]),
            regression_tolerance_digest(&[b.clone(), a.clone()])
        );
        let mut changed = b.clone();
        changed.absolute_tolerance = 0.3;
        assert_ne!(
            regression_tolerance_digest(&[a.clone(), changed]),
            regression_tolerance_digest(&[a, b])
        );
    }
}
