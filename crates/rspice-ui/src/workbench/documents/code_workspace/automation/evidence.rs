//! Specification evidence and the run digest it is bound to.
//!
//! The digest covers every field the comparison reads, so evidence can never
//! be published against a run that differs from the one it was measured on.

use super::*;

pub(super) fn build_spec_evidence(
    specs: &[SpecEntry],
    run: &SimulationRun,
) -> Result<Vec<CheckEvidence>, String> {
    let required = specs
        .len()
        .checked_mul(run.analyses.len())
        .ok_or_else(|| "specification coverage count overflowed".to_owned())?;
    if required == 0 {
        return Err("specification coverage requires at least one task and one spec".to_owned());
    }
    let mut checks = Vec::with_capacity(required);
    for (task_index, analysis) in run.analyses.iter().enumerate() {
        let provenance = analysis.provenance.as_ref().ok_or_else(|| {
            format!(
                "analysis {:?} has no authenticated source identity",
                analysis.label
            )
        })?;
        for (spec_index, spec) in specs.iter().enumerate() {
            let matches = analysis
                .measurements
                .iter()
                .filter(|measurement| measurement.name.eq_ignore_ascii_case(&spec.measurement))
                .collect::<Vec<_>>();
            let (passed, detail) = match matches.as_slice() {
                [] => (
                    false,
                    format!(
                        "Task {} ({}) retained no measurement named {:?}; required {}.",
                        task_index + 1,
                        provenance.source_instance_id(),
                        spec.measurement,
                        specification_limits(spec)
                    ),
                ),
                [measurement] => {
                    let value = measurement.value.filter(|value| value.is_finite());
                    let passed = analysis.success
                        && measurement.passed
                        && measurement.error.is_none()
                        && value.is_some_and(|value| spec.passes(value));
                    let detail = value.map_or_else(
                        || {
                            format!(
                                "Task {} ({}) retained no verified finite value for {:?}: {}",
                                task_index + 1,
                                provenance.source_instance_id(),
                                spec.measurement,
                                measurement
                                    .error
                                    .as_deref()
                                    .unwrap_or("measurement result was incomplete")
                            )
                        },
                        |value| {
                            let status = measurement.error.as_deref().map_or_else(
                                || {
                                    if measurement.passed {
                                        "measurement evaluation passed".to_owned()
                                    } else {
                                        "measurement evaluation failed its declared goal".to_owned()
                                    }
                                },
                                |error| format!("measurement error: {error}"),
                            );
                            format!(
                                "Task {} ({}) measured {value:.12e}; required {}; {status}.",
                                task_index + 1,
                                provenance.source_instance_id(),
                                specification_limits(spec)
                            )
                        },
                    );
                    (passed, detail)
                }
                duplicates => (
                    false,
                    format!(
                        "Task {} ({}) retained {} ambiguous measurements named {:?}.",
                        task_index + 1,
                        provenance.source_instance_id(),
                        duplicates.len(),
                        spec.measurement
                    ),
                ),
            };
            checks.push(
                CheckEvidence::try_new(
                    format!(
                        "release-spec-{:05}-task-{}",
                        spec_index + 1,
                        provenance.source_instance_id()
                    ),
                    format!("{} / {}", analysis.label, spec.measurement),
                    if passed {
                        CheckOutcome::Passed
                    } else {
                        CheckOutcome::Failed
                    },
                    0,
                    detail,
                )
                .map_err(|error| format!("invalid task/spec evidence: {error}"))?,
            );
        }
    }
    Ok(checks)
}

pub(super) fn specification_limits(spec: &SpecEntry) -> String {
    match (spec.min, spec.max) {
        (Some(min), Some(max)) => format!("{min:.12e} to {max:.12e} {}", spec.unit),
        (Some(min), None) => format!(">= {min:.12e} {}", spec.unit),
        (None, Some(max)) => format!("<= {max:.12e} {}", spec.unit),
        (None, None) => "a tracked finite value".to_owned(),
    }
}

pub(super) fn simulation_run_digest(run: &SimulationRun) -> Result<ContentDigest, String> {
    validate_run_shape(run)?;
    let receipt = run
        .prepared_receipt()
        .ok_or_else(|| "run lost its prepared-run authority before evidence sealing".to_owned())?;
    let mut digest = Sha256::new();
    digest.update(b"rspice-automation-run-evidence-v1\0");
    digest.update(run.run_id.as_uuid().as_bytes());
    digest.update(run.dataset_id.as_uuid().as_bytes());
    hash_u64(&mut digest, run.id);
    hash_f64(&mut digest, run.timestamp);
    hash_f64(&mut digest, run.elapsed_time);
    digest.update([u8::from(run.success)]);
    digest.update([match receipt.source_domain() {
        AnalysisResultSourceDomain::SimulationPlan => 0,
        AnalysisResultSourceDomain::ManualDeck => 1,
        AnalysisResultSourceDomain::LegacyUnclassified => 2,
    }]);
    match receipt.simulation_plan_id() {
        Some(plan_id) => {
            digest.update([1]);
            digest.update(plan_id.as_uuid().as_bytes());
        }
        None => digest.update([0]),
    }
    hash_u64(&mut digest, receipt.project_revision().get());
    digest.update(receipt.prepared_snapshot_digest().as_bytes());
    digest.update(receipt.source_content_digest().as_bytes());
    digest.update(receipt.source_check_receipt().digest().as_bytes());
    hash_u64(&mut digest, receipt.tasks().len() as u64);
    for task in receipt.tasks() {
        digest.update(task.instance_id().as_uuid().as_bytes());
        hash_u64(&mut digest, task.source_revision().get());
        digest.update([task.analysis_kind_tag()]);
        digest.update(task.config_digest().as_bytes());
        hash_u64(&mut digest, task.dependencies().len() as u64);
        for dependency in task.dependencies() {
            digest.update(dependency.as_uuid().as_bytes());
        }
    }
    hash_u64(&mut digest, run.analyses.len() as u64);
    for analysis in &run.analyses {
        hash_u64(&mut digest, analysis.id);
        hash_text(&mut digest, &analysis.label);
        hash_f64(&mut digest, analysis.timestamp);
        digest.update([u8::from(analysis.success)]);
        hash_optional_text(&mut digest, analysis.error_message.as_deref());
        let provenance = analysis
            .provenance
            .as_ref()
            .ok_or_else(|| format!("analysis {:?} has no provenance", analysis.label))?;
        digest.update(provenance.source_instance_id().as_uuid().as_bytes());
        hash_u64(&mut digest, provenance.source_revision().get());
        digest.update(provenance.prepared_snapshot_digest().as_bytes());
        hash_u64(&mut digest, analysis.measurements.len() as u64);
        for measurement in &analysis.measurements {
            hash_text(&mut digest, &measurement.name);
            hash_optional_f64(&mut digest, measurement.value);
            hash_optional_text(&mut digest, measurement.error.as_deref());
            digest.update([u8::from(measurement.passed)]);
            hash_optional_f64(&mut digest, measurement.expected);
            hash_optional_f64(&mut digest, measurement.tolerance);
        }
        hash_u64(&mut digest, analysis.waveforms.len() as u64);
        for waveform in &analysis.waveforms {
            hash_text(&mut digest, &waveform.name);
            hash_float_slice(&mut digest, &waveform.x);
            hash_float_slice(&mut digest, &waveform.y);
            match &waveform.complex {
                Some(complex) => {
                    digest.update([1]);
                    hash_text(&mut digest, &complex.source_name);
                    hash_float_slice(&mut digest, &complex.real);
                    hash_float_slice(&mut digest, &complex.imag);
                }
                None => digest.update([0]),
            }
        }
    }
    Ok(ContentDigest::from_bytes(digest.finalize().into()))
}

pub(super) fn hash_u64(digest: &mut Sha256, value: u64) {
    digest.update(value.to_le_bytes());
}

pub(super) fn hash_f64(digest: &mut Sha256, value: f64) {
    digest.update(value.to_bits().to_le_bytes());
}

pub(super) fn hash_text(digest: &mut Sha256, value: &str) {
    hash_u64(digest, value.len() as u64);
    digest.update(value.as_bytes());
}

pub(super) fn hash_optional_text(digest: &mut Sha256, value: Option<&str>) {
    match value {
        Some(value) => {
            digest.update([1]);
            hash_text(digest, value);
        }
        None => digest.update([0]),
    }
}

pub(super) fn hash_optional_f64(digest: &mut Sha256, value: Option<f64>) {
    match value {
        Some(value) => {
            digest.update([1]);
            hash_f64(digest, value);
        }
        None => digest.update([0]),
    }
}

pub(super) fn hash_float_slice(digest: &mut Sha256, values: &[f64]) {
    hash_u64(digest, values.len() as u64);
    for value in values {
        hash_f64(digest, *value);
    }
}
