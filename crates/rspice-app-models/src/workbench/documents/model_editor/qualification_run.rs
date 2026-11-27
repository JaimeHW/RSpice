//! Driving a cooperative qualification pass from the editor.
//!
//! The pass runs one vector at a time so the UI stays responsive, and its
//! results are committed to the draft only once a platform record is complete
//! and validated. A partial pass — cancelled, or stopped by a failure — never
//! reaches the draft, so the editor cannot show evidence for a suite that did
//! not finish.

use super::*;

pub(super) fn parse_frequency_axis(value: &str) -> Result<Vec<FiniteValue>, String> {
    let tokens = value
        .split(|character: char| character == ',' || character.is_whitespace())
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();
    if tokens.is_empty() {
        return Err("Frequency axis requires at least one frequency in hertz".to_owned());
    }
    tokens
        .into_iter()
        .enumerate()
        .map(|(index, token)| {
            FiniteValue::new(parse_finite(token, &format!("Frequency point {index}"))?)
                .map_err(|error| error.to_string())
        })
        .collect()
}

pub(super) fn parse_sample_index(value: &str, label: &str) -> Result<usize, String> {
    value
        .trim()
        .parse::<usize>()
        .map_err(|error| format!("{label} is invalid: {error}"))
}

pub(super) fn parse_finite(value: &str, label: &str) -> Result<f64, String> {
    let parsed = value
        .trim()
        .parse::<f64>()
        .map_err(|error| format!("{label} is invalid: {error}"))?;
    if parsed.is_finite() {
        Ok(parsed)
    } else {
        Err(format!("{label} must be finite"))
    }
}

pub(super) fn parse_non_negative(value: &str, label: &str) -> Result<f64, String> {
    let parsed = parse_finite(value, label)?;
    if parsed >= 0.0 {
        Ok(parsed)
    } else {
        Err(format!("{label} must be non-negative"))
    }
}

/// Begin an exact, cooperative qualification pass for every persisted suite
/// on the current runtime. Desktop and WebAssembly records rendezvous through
/// persisted project state; neither target fabricates the other target's run.
pub fn start_qualification_execution(parts: &mut ModelEditorParts<'_>) -> Result<(), String> {
    start_qualification_execution_for(parts, None)
}

/// Begin a cooperative rerun for one exact retained suite. The suite must
/// belong to the current project-owned source revision.
pub fn start_qualification_suite_execution(
    parts: &mut ModelEditorParts<'_>,
    suite_id: &str,
) -> Result<(), String> {
    if suite_id.trim().is_empty() {
        return Err("Qualification suite ID is required".to_owned());
    }
    start_qualification_execution_for(parts, Some(suite_id))
}

/// Rerun the complete suite that owns a selected vector. Platform runs and
/// evidence are suite-complete contracts, so a vector action never publishes
/// a misleading partial suite result.
pub fn start_qualification_vector_execution(
    parts: &mut ModelEditorParts<'_>,
    suite_id: &str,
    vector_id: &str,
) -> Result<(), String> {
    parts
        .editor
        .draft
        .as_ref()
        .ok_or_else(|| "No project-owned model candidate is open".to_owned())?
        .qualification
        .qualification_vector(suite_id, vector_id)
        .map_err(|error| error.to_string())?;
    start_qualification_suite_execution(parts, suite_id)
}

pub(super) fn start_qualification_execution_for(
    parts: &mut ModelEditorParts<'_>,
    selected_suite_id: Option<&str>,
) -> Result<(), String> {
    if !parts.platform.project_lifecycle.is_open() {
        return Err("Qualification requires an open project".to_owned());
    }
    if parts.model.safe_mode.project_read_only() {
        return Err("Qualification cannot run while the project is read-only".to_owned());
    }
    if !validate_open_candidate(parts) {
        return Err("Qualification cannot run until the model candidate validates".to_owned());
    }
    let editor = &mut parts.editor;
    if editor.qualification_execution.is_some() {
        return Err("A model qualification run is already active".to_owned());
    }
    let draft = editor
        .draft
        .as_ref()
        .ok_or_else(|| "No project-owned model candidate is open".to_owned())?;
    if draft.definition_is_dirty() {
        return Err("Save the model definition before running qualification".to_owned());
    }
    draft
        .qualification
        .validate_for_model(&draft.model_name)
        .map_err(|error| format!("Qualification plan is invalid: {error}"))?;
    let source = ModelSourceEvidenceBinding::try_new_project_bound(
        draft.model_name.clone(),
        draft.source_id,
        draft.base_source_digest,
        draft.base_source_revision,
    )
    .map_err(|error| error.to_string())?;
    let exact_suites = draft
        .qualification
        .exact_suites_for_source(&source)
        .map_err(|error| format!("Qualification plan is invalid: {error}"))?;
    let selected_suites = if let Some(suite_id) = selected_suite_id {
        let suite = exact_suites
            .iter()
            .copied()
            .find(|suite| suite.id.eq_ignore_ascii_case(suite_id))
            .ok_or_else(|| {
                format!(
                    "Qualification suite '{suite_id}' is missing or is not bound to the current model source revision"
                )
            })?;
        vec![suite]
    } else {
        exact_suites
    };
    let first_suite = selected_suites.first().copied().ok_or_else(|| {
        "Add at least one executable qualification suite for the current model source revision before running"
            .to_owned()
    })?;
    let suite_ids = selected_suites
        .iter()
        .map(|suite| suite.id.clone())
        .collect::<Vec<_>>();
    let session = QualificationExecutionSession::try_new(first_suite, &source)
        .map_err(|error| error.to_string())?;
    let progress = session.progress();
    let platform = qualification_platform_label(progress.platform);
    editor.qualification_execution = Some(ModelQualificationExecution {
        suite_ids,
        suite_index: 0,
        source,
        session,
        progress,
        assembled_evidence: 0,
    });
    editor.qualification_execution_notice = Some(format!(
        "Running {platform} qualification. Completed vectors are retained only after each full suite finishes."
    ));
    Ok(())
}

/// Advance at most one qualification vector. Calling this once per frame keeps
/// desktop and browser shells responsive while preserving atomic run records.
pub fn advance_qualification_execution(parts: &mut ModelEditorParts<'_>) {
    let Some(mut execution) = parts.editor.qualification_execution.take() else {
        return;
    };

    match execution.session.step(&rspice_core::NoAbort) {
        Ok(QualificationExecutionStep::InProgress(progress)) => {
            execution.progress = progress;
            parts.editor.qualification_execution = Some(execution);
        }
        Ok(QualificationExecutionStep::Complete { progress, run }) => {
            execution.progress = progress;
            let suite_id = execution.current_suite_id().to_owned();
            let outcome = (|| -> Result<(), String> {
                let draft =
                    parts.editor.draft.as_mut().ok_or_else(|| {
                        "The model candidate closed during qualification".to_owned()
                    })?;
                draft
                    .qualification
                    .upsert_platform_run_atomically(run)
                    .map_err(|error| error.to_string())?;
                let evidence_id = qualification_evidence_id(&suite_id, &execution.source);
                match draft.qualification.assemble_and_upsert_evidence_atomically(
                    evidence_id,
                    &suite_id,
                    &execution.source,
                ) {
                    Ok(evidence) => {
                        execution.assembled_evidence += 1;
                        let covered_sections = draft
                            .qualification
                            .suites
                            .iter()
                            .find(|suite| suite.id.eq_ignore_ascii_case(&suite_id))
                            .into_iter()
                            .flat_map(|suite| suite.vectors.iter())
                            .filter_map(|vector| vector.model_section.as_deref())
                            .map(str::to_ascii_lowercase)
                            .collect::<BTreeSet<_>>();
                        if !covered_sections.is_empty() {
                            let evidence_digest = evidence
                                .content_digest()
                                .map_err(|error| error.to_string())?;
                            if evidence.passed {
                                for section_name in &covered_sections {
                                    draft
                                        .qualification
                                        .validate_exact_section_evidence_digest(
                                            &execution.source,
                                            section_name,
                                            evidence_digest,
                                        )
                                        .map_err(|error| error.to_string())?;
                                }
                            }
                            for section in &mut draft.metadata.sections {
                                if !covered_sections.contains(&section.name.to_ascii_lowercase()) {
                                    continue;
                                }
                                if evidence.passed {
                                    section.qualification = ModelSectionQualification::Qualified {
                                        evidence_digest: Some(evidence_digest.to_string()),
                                    };
                                } else {
                                    section.qualification = ModelSectionQualification::Failed {
                                        summary: format!(
                                            "Qualification suite {suite_id} failed one or more declared Desktop/WebAssembly vector gates"
                                        ),
                                    };
                                }
                            }
                        }
                    }
                    Err(error)
                        if error.code == QualificationErrorCode::EvidenceCoverageMismatch =>
                    {
                        // The other real runtime has not published its exact run yet.
                    }
                    Err(error) => return Err(error.to_string()),
                }
                Ok(())
            })();
            if let Err(error) = outcome {
                parts.editor.qualification_execution_notice = Some(format!(
                    "Qualification stopped without keeping partial results: {error}"
                ));
                return;
            }

            execution.suite_index += 1;
            if execution.suite_index < execution.suite_ids.len() {
                let next_id = execution.current_suite_id().to_owned();
                let next_suite = parts
                    .editor
                    .draft
                    .as_ref()
                    .and_then(|draft| {
                        draft
                            .qualification
                            .suites
                            .iter()
                            .find(|suite| suite.id.eq_ignore_ascii_case(&next_id))
                    })
                    .cloned();
                let Some(next_suite) = next_suite else {
                    parts.editor.qualification_execution_notice = Some(
                        "Qualification plan changed during execution; remaining suites were not run."
                            .to_owned(),
                    );
                    return;
                };
                match QualificationExecutionSession::try_new(&next_suite, &execution.source) {
                    Ok(session) => {
                        execution.progress = session.progress();
                        execution.session = session;
                        parts.editor.qualification_execution = Some(execution);
                    }
                    Err(error) => {
                        parts.editor.qualification_execution_notice = Some(format!(
                            "Qualification stopped before suite '{next_id}': {error}"
                        ));
                    }
                }
            } else {
                let platform = qualification_platform_label(execution.progress.platform);
                parts.editor.qualification_execution_notice = Some(format!(
                    "{platform} qualification completed for {} suite(s); {} Desktop/WebAssembly result record(s) were assembled. Save the qualification revision to keep the results.",
                    execution.total_suites(),
                    execution.assembled_evidence,
                ));
            }
        }
        Err(error) => {
            parts.editor.qualification_execution_notice = Some(format!(
                "Qualification stopped without publishing a partial suite: {error}"
            ));
        }
    }
}

/// Cancel between vectors and discard all in-progress outcomes. Completed
/// suites already committed to the draft remain available for explicit save.
pub fn cancel_qualification_execution(parts: &mut ModelEditorParts<'_>) -> bool {
    let Some(mut execution) = parts.editor.qualification_execution.take() else {
        return false;
    };
    execution.session.cancel();
    parts.editor.qualification_execution_notice = Some(format!(
        "{} qualification cancelled before the next vector; no partial suite was published.",
        qualification_platform_label(execution.progress.platform),
    ));
    true
}

/// Apply a promotion transaction to the open candidate only when the project
/// remains writable. The subsequent explicit save publishes the immutable
/// release through the normal project revision history.
pub fn promote_open_candidate(parts: &mut ModelEditorParts<'_>, candidate_id: &str) -> bool {
    if !parts.platform.project_lifecycle.is_open() {
        parts.editor.promotion_error = Some("Model promotion requires an open project".to_owned());
        return false;
    }
    if parts.model.safe_mode.project_read_only() {
        parts.editor.promotion_error =
            Some("Model promotion is unavailable while the project is read-only".to_owned());
        return false;
    }
    if !validate_open_candidate(parts) {
        parts.editor.promotion_error = Some(
            "Model promotion requires a valid candidate bound to the current project revision"
                .to_owned(),
        );
        return false;
    }
    let Some((library_name, model_name)) = parts
        .editor
        .draft
        .as_ref()
        .map(|draft| (draft.library_name.clone(), draft.model_name.clone()))
    else {
        parts.editor.promotion_error = Some("No project-owned model candidate is open".to_owned());
        return false;
    };
    let Some(library) = parts
        .model
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .find(|library| library.name.eq_ignore_ascii_case(&library_name))
    else {
        parts.editor.promotion_error = Some(format!(
            "Model promotion cannot resolve project library '{library_name}'"
        ));
        return false;
    };
    let correlation = library
        .model_correlation
        .get(&model_name)
        .cloned()
        .unwrap_or_default();
    parts.editor.commit_promotion(candidate_id, &correlation)
}

pub(super) fn qualification_evidence_id(
    suite_id: &str,
    source: &ModelSourceEvidenceBinding,
) -> String {
    format!(
        "{}-r{}-{}",
        suite_id,
        source.source_revision.get(),
        source.source_digest
    )
}

#[must_use]
pub const fn qualification_platform_label(platform: QualificationPlatform) -> &'static str {
    match platform {
        QualificationPlatform::Desktop => "Desktop",
        QualificationPlatform::WebAssembly => "WebAssembly",
    }
}

impl ModelEditorState {
    pub(super) fn build_qualification_suite(&self) -> Result<QualificationSuite, String> {
        let draft = self
            .draft
            .as_ref()
            .ok_or_else(|| "No project-owned model candidate is open".to_owned())?;
        if draft.definition_is_dirty() {
            return Err(
                "Save the model definition before binding a qualification vector to it".to_owned(),
            );
        }
        let fields = &self.qualification_authoring;
        let source = ModelSourceEvidenceBinding::try_new_project_bound(
            draft.model_name.clone(),
            draft.source_id,
            draft.base_source_digest,
            draft.base_source_revision,
        )
        .map_err(|error| error.to_string())?;
        let definition = draft.definition().map_err(|diagnostics| {
            diagnostics
                .into_iter()
                .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
                .collect::<Vec<_>>()
                .join("; ")
        })?;
        let model_source = definition
            .canonical_source()
            .map_err(|error| error.to_string())?
            .into_bytes();
        if fields.model_section.trim() != fields.model_section {
            return Err("Model section must not contain outer whitespace".to_owned());
        }
        let model_section =
            (!fields.model_section.is_empty()).then(|| fields.model_section.clone());
        let execution_model_source = definition
            .qualification_model_source(model_section.as_deref())
            .map_err(|error| error.to_string())?
            .into_bytes();
        // SPICE reserves the first physical line as a deck title. Keep the
        // selected model card after an explicit title so a section card that
        // begins directly with `.model` is parsed rather than discarded.
        let mut executable_input = b"RSpice model qualification\n".to_vec();
        executable_input.extend_from_slice(&execution_model_source);
        if !executable_input.ends_with(b"\n") {
            executable_input.push(b'\n');
        }
        executable_input.extend_from_slice(fields.executable_input.as_bytes());
        let analysis = match fields.analysis {
            QualificationAuthoringAnalysis::DcOperatingPoint => {
                QualificationAnalysis::DcOperatingPoint
            }
            QualificationAuthoringAnalysis::DcSweep => QualificationAnalysis::DcSweep {
                source: fields.sweep_source.clone(),
                start: FiniteValue::new(parse_finite(&fields.sweep_start, "Sweep start")?)
                    .map_err(|error| error.to_string())?,
                stop: FiniteValue::new(parse_finite(&fields.sweep_stop, "Sweep stop")?)
                    .map_err(|error| error.to_string())?,
                step: FiniteValue::new(parse_finite(&fields.sweep_step, "Sweep step")?)
                    .map_err(|error| error.to_string())?,
            },
            QualificationAuthoringAnalysis::AcSweep => QualificationAnalysis::AcSweep {
                frequencies: parse_frequency_axis(&fields.frequencies)?,
            },
            QualificationAuthoringAnalysis::Noise => QualificationAnalysis::Noise {
                output_node: fields.noise_output_node.clone(),
                output_reference: (!fields.noise_output_reference.is_empty())
                    .then(|| fields.noise_output_reference.clone()),
                input_source: fields.noise_input_source.clone(),
                frequencies: parse_frequency_axis(&fields.frequencies)?,
                temperature_kelvin: FiniteValue::new(parse_finite(
                    &fields.noise_temperature_kelvin,
                    "Noise temperature",
                )?)
                .map_err(|error| error.to_string())?,
            },
            QualificationAuthoringAnalysis::Transient => QualificationAnalysis::Transient {
                stop_time: FiniteValue::new(parse_finite(
                    &fields.transient_stop_time,
                    "Transient stop time",
                )?)
                .map_err(|error| error.to_string())?,
                max_step: FiniteValue::new(parse_finite(
                    &fields.transient_max_step,
                    "Transient maximum step",
                )?)
                .map_err(|error| error.to_string())?,
            },
        };
        let probe = match fields.probe {
            QualificationAuthoringProbe::NodeVoltage => QualificationProbe::NodeVoltage {
                node: fields.probe_target.clone(),
            },
            QualificationAuthoringProbe::BranchCurrent => QualificationProbe::BranchCurrent {
                branch: fields.probe_target.clone(),
            },
            QualificationAuthoringProbe::DcObservable => QualificationProbe::DcObservable {
                expression: fields.probe_target.clone(),
            },
            QualificationAuthoringProbe::SweepValue => QualificationProbe::SweepValue,
            QualificationAuthoringProbe::AcNodeVoltageMagnitude => {
                QualificationProbe::AcNodeVoltageMagnitude {
                    node: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcNodeVoltagePhaseDegrees => {
                QualificationProbe::AcNodeVoltagePhaseDegrees {
                    node: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcNodeVoltageReal => {
                QualificationProbe::AcNodeVoltageReal {
                    node: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcNodeVoltageImaginary => {
                QualificationProbe::AcNodeVoltageImaginary {
                    node: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcBranchCurrentMagnitude => {
                QualificationProbe::AcBranchCurrentMagnitude {
                    branch: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcBranchCurrentPhaseDegrees => {
                QualificationProbe::AcBranchCurrentPhaseDegrees {
                    branch: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcBranchCurrentReal => {
                QualificationProbe::AcBranchCurrentReal {
                    branch: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcBranchCurrentImaginary => {
                QualificationProbe::AcBranchCurrentImaginary {
                    branch: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcEffectiveCapacitance => {
                QualificationProbe::AcEffectiveCapacitance {
                    branch: fields.probe_target.clone(),
                    excitation_magnitude: FiniteValue::new(parse_finite(
                        &fields.excitation_magnitude,
                        "AC excitation magnitude",
                    )?)
                    .map_err(|error| error.to_string())?,
                }
            }
            QualificationAuthoringProbe::FrequencyValue => QualificationProbe::FrequencyValue,
            QualificationAuthoringProbe::NoiseOutputDensity => {
                QualificationProbe::NoiseOutputDensity
            }
            QualificationAuthoringProbe::NoiseInputReferredDensity => {
                QualificationProbe::NoiseInputReferredDensity
            }
            QualificationAuthoringProbe::NoiseOutputAmplitude => {
                QualificationProbe::NoiseOutputAmplitude
            }
            QualificationAuthoringProbe::NoiseInputReferredAmplitude => {
                QualificationProbe::NoiseInputReferredAmplitude
            }
            QualificationAuthoringProbe::TransientNodeVoltage => {
                QualificationProbe::TransientNodeVoltage {
                    node: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::TransientBranchCurrent => {
                QualificationProbe::TransientBranchCurrent {
                    branch: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::TimeValue => QualificationProbe::TimeValue,
        };
        let sample = match fields.sample {
            QualificationAuthoringSample::OperatingPoint => QualificationSample::OperatingPoint,
            QualificationAuthoringSample::FirstSweepPoint => QualificationSample::FirstSweepPoint,
            QualificationAuthoringSample::LastSweepPoint => QualificationSample::LastSweepPoint,
            QualificationAuthoringSample::SweepPoint => QualificationSample::SweepPoint {
                index: fields
                    .sample_index
                    .trim()
                    .parse::<usize>()
                    .map_err(|error| format!("Sweep point index is invalid: {error}"))?,
            },
            QualificationAuthoringSample::FirstFrequencyPoint => {
                QualificationSample::FirstFrequencyPoint
            }
            QualificationAuthoringSample::LastFrequencyPoint => {
                QualificationSample::LastFrequencyPoint
            }
            QualificationAuthoringSample::FrequencyPoint => QualificationSample::FrequencyPoint {
                index: parse_sample_index(&fields.sample_index, "Frequency point index")?,
            },
            QualificationAuthoringSample::FirstTimePoint => QualificationSample::FirstTimePoint,
            QualificationAuthoringSample::LastTimePoint => QualificationSample::LastTimePoint,
            QualificationAuthoringSample::TimePoint => QualificationSample::TimePoint {
                index: parse_sample_index(&fields.sample_index, "Time point index")?,
            },
        };
        let output = QualificationOutputDefinition::try_new(fields.quantity.clone(), probe, sample)
            .map_err(|error| error.to_string())?;
        let reference = QualificationReference::try_new(
            fields.quantity.clone(),
            parse_finite(&fields.expected, "Expected value")?,
            parse_non_negative(&fields.absolute_tolerance, "Absolute tolerance")?,
            parse_non_negative(&fields.relative_tolerance, "Relative tolerance")?,
        )
        .map_err(|error| error.to_string())?;
        let vector = QualificationVector::try_new(
            rspice_model_library::qualification::QualificationVectorInput {
                id: fields.vector_id.clone(),
                name: fields.vector_name.clone(),
                source,
                model_source,
                model_section,
                execution_model_source,
                executable_input,
                analysis,
                outputs: vec![output],
                references: vec![reference],
            },
        )
        .map_err(|error| error.to_string())?;
        QualificationSuite::try_new(
            fields.suite_id.clone(),
            fields.suite_name.clone(),
            ObjectRevision::INITIAL,
            vec![vector],
        )
        .map_err(|error| error.to_string())
    }
}
