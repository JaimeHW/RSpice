//! Assembling a release candidate from the editor's drafts.
//!
//! Every field a candidate needs is parsed and checked here before the
//! candidate exists, so a half-filled promotion form produces an error rather
//! than a candidate that later fails the gate. The comparison rows are built
//! from the same parsed values, which is what makes the diff a reviewer sees
//! and the record that gets promoted the same thing.

use super::*;

pub(super) fn qualification_evidence_summary(
    evidence: &crate::state::model_library::QualificationEvidence,
) -> String {
    let desktop = evidence.platform_passed(QualificationPlatform::Desktop);
    let webassembly = evidence.platform_passed(QualificationPlatform::WebAssembly);
    format!(
        "{} vectors · Desktop {} · WebAssembly {}",
        evidence.vector_outcomes.len(),
        if desktop { "pass" } else { "fail" },
        if webassembly { "pass" } else { "fail" },
    )
}

pub(super) fn append_typed_metadata_comparison_rows(
    rows: &mut Vec<ModelReleaseComparisonRow>,
    released: &ModelDefinitionMetadata,
    candidate: &ModelDefinitionMetadata,
) -> Result<(), String> {
    append_named_contract_rows(
        rows,
        "Parameter contract",
        &released.parameters,
        &candidate.parameters,
        |value| &value.name,
        |value| {
            format!(
                "{:?} · value {:?} · unit {} · bounds {:?} · source {:?}",
                value.data_type,
                value.value,
                value.unit.as_deref().unwrap_or("unitless"),
                value.bounds,
                value.source
            )
        },
    );
    append_named_contract_rows(
        rows,
        "Section",
        &released.sections,
        &candidate.sections,
        |value| &value.name,
        |value| {
            format!(
                "parent {} · {} overrides · {} files · {:?}",
                value.parent.as_deref().unwrap_or("base"),
                value.overrides.len(),
                value.model_files.len(),
                value.qualification
            )
        },
    );
    append_named_contract_rows(
        rows,
        "Statistical variable",
        &released.statistics.variables,
        &candidate.statistics.variables,
        |value| &value.name,
        |value| {
            format!(
                "parameter {} · {:?} · group {} · {:?}",
                value.parameter,
                value.distribution,
                value.correlation_group.as_deref().unwrap_or("independent"),
                value.hierarchy
            )
        },
    );
    append_named_contract_rows(
        rows,
        "Correlation matrix",
        &released.statistics.correlation_matrices,
        &candidate.statistics.correlation_matrices,
        |value| &value.group,
        |value| {
            format!(
                "{} variables · exact coefficients {:?}",
                value.variables.len(),
                value.coefficients
            )
        },
    );
    append_named_contract_rows(
        rows,
        "Temperature law",
        &released.temperature_laws,
        &candidate.temperature_laws,
        |value| &value.quantity,
        |value| {
            format!(
                "{:?} · reference {} °C · range {}…{} °C · {:?}",
                value.representation,
                value.reference_temperature_c.get(),
                value.valid_range.minimum_c.get(),
                value.valid_range.maximum_c.get(),
                value.extrapolation
            )
        },
    );
    Ok(())
}

pub(super) fn append_named_contract_rows<T, Key, Summary>(
    rows: &mut Vec<ModelReleaseComparisonRow>,
    category: &str,
    released: &[T],
    candidate: &[T],
    key: Key,
    summary: Summary,
) where
    T: PartialEq,
    Key: Fn(&T) -> &str,
    Summary: Fn(&T) -> String,
{
    let released_by_name = released
        .iter()
        .map(|value| (key(value).to_ascii_lowercase(), value))
        .collect::<BTreeMap<_, _>>();
    let candidate_by_name = candidate
        .iter()
        .map(|value| (key(value).to_ascii_lowercase(), value))
        .collect::<BTreeMap<_, _>>();
    let names = released_by_name
        .keys()
        .chain(candidate_by_name.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut changed = false;
    for name in names {
        let before = released_by_name.get(&name).copied();
        let after = candidate_by_name.get(&name).copied();
        if before == after {
            continue;
        }
        changed = true;
        rows.push(ModelReleaseComparisonRow {
            item: format!("{category}: {name}"),
            released: before.map_or_else(|| "—".to_owned(), &summary),
            candidate: after.map_or_else(|| "—".to_owned(), &summary),
            effect: match (before, after) {
                (None, Some(_)) => format!("{category} added"),
                (Some(_), None) => format!("{category} removed"),
                _ => format!("{category} changed"),
            },
            disposition: ModelComparisonDisposition::Review,
        });
    }
    if !changed {
        rows.push(ModelReleaseComparisonRow {
            item: format!("{category}s"),
            released: released.len().to_string(),
            candidate: candidate.len().to_string(),
            effect: "exact contracts unchanged".to_owned(),
            disposition: ModelComparisonDisposition::Unchanged,
        });
    }
}

pub(super) fn append_numerical_evidence_rows(
    rows: &mut Vec<ModelReleaseComparisonRow>,
    released: &crate::state::model_library::QualificationEvidence,
    candidate: &crate::state::model_library::QualificationEvidence,
) {
    let released_values = evidence_numerical_values(released);
    let candidate_values = evidence_numerical_values(candidate);
    let keys = released_values
        .keys()
        .chain(candidate_values.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    for key in keys {
        let before = released_values.get(&key);
        let after = candidate_values.get(&key);
        let (effect, disposition) = match (before, after) {
            (Some(before), Some(after)) if before == after => (
                "exact result unchanged".to_owned(),
                ModelComparisonDisposition::Unchanged,
            ),
            (Some(before), Some(after)) => {
                let delta = after.observed - before.observed;
                let disposition = if before.passed && !after.passed {
                    ModelComparisonDisposition::Blocking
                } else if !before.passed && after.passed {
                    ModelComparisonDisposition::Improved
                } else {
                    ModelComparisonDisposition::Review
                };
                (format!("observed delta {delta:+.9e}"), disposition)
            }
            (None, Some(_)) => (
                "numerical result added".to_owned(),
                ModelComparisonDisposition::Review,
            ),
            (Some(_), None) => (
                "numerical result removed".to_owned(),
                ModelComparisonDisposition::Blocking,
            ),
            (None, None) => continue,
        };
        rows.push(ModelReleaseComparisonRow {
            item: format!("Result: {key}"),
            released: before.map_or_else(|| "—".to_owned(), NumericalEvidenceValue::summary),
            candidate: after.map_or_else(|| "—".to_owned(), NumericalEvidenceValue::summary),
            effect,
            disposition,
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct NumericalEvidenceValue {
    observed: f64,
    relative_error: f64,
    relative_tolerance: f64,
    passed: bool,
}

impl NumericalEvidenceValue {
    fn summary(&self) -> String {
        format!(
            "observed {:.9e} · rel {:.6}% / {:.6}% · {}",
            self.observed,
            self.relative_error * 100.0,
            self.relative_tolerance * 100.0,
            if self.passed { "pass" } else { "fail" }
        )
    }
}

pub(super) fn evidence_numerical_values(
    evidence: &crate::state::model_library::QualificationEvidence,
) -> BTreeMap<String, NumericalEvidenceValue> {
    let mut values = BTreeMap::new();
    for vector in &evidence.vector_outcomes {
        for platform in &vector.platforms {
            for reference in &platform.references {
                values.insert(
                    format!(
                        "{} / {:?} / {}",
                        vector.vector_id, platform.platform, reference.quantity
                    ),
                    NumericalEvidenceValue {
                        observed: reference.observed_value.get(),
                        relative_error: reference.relative_error.get(),
                        relative_tolerance: reference.relative_tolerance.get(),
                        passed: reference.passed,
                    },
                );
            }
        }
    }
    values
}

pub(super) fn build_release_candidate(
    draft: &ModelEditorDraft,
    fields: &PromotionCandidateDraft,
) -> Result<ModelReleaseCandidate, String> {
    let suite = draft
        .qualification
        .suites
        .iter()
        .find(|suite| suite.id.eq_ignore_ascii_case(fields.suite_id.trim()))
        .ok_or_else(|| {
            format!(
                "Qualification suite '{}' does not exist",
                fields.suite_id.trim()
            )
        })?;
    let evidence = draft
        .qualification
        .evidence
        .iter()
        .find(|evidence| evidence.id.eq_ignore_ascii_case(fields.evidence_id.trim()))
        .ok_or_else(|| {
            format!(
                "Qualification evidence '{}' does not exist",
                fields.evidence_id.trim()
            )
        })?;
    if !evidence
        .source
        .model_id
        .eq_ignore_ascii_case(&draft.model_name)
        || evidence.source.source_id != Some(draft.source_id)
        || evidence.source.source_digest != draft.base_source_digest
        || evidence.source.source_revision != draft.base_source_revision
    {
        return Err("Selected evidence is not bound to the open model source revision".to_owned());
    }
    if !evidence.suite_id.eq_ignore_ascii_case(&suite.id)
        || evidence.suite_revision != suite.revision
    {
        return Err("Selected evidence is not bound to the selected suite revision".to_owned());
    }

    let documentation = DocumentationSet::try_new(vec![
        DocumentationDeclaration {
            kind: RequiredDocumentation::ModelDescription,
            document: document_reference(
                "Model description",
                &fields.model_description_id,
                &fields.model_description_digest,
            )?,
        },
        DocumentationDeclaration {
            kind: RequiredDocumentation::ParameterReference,
            document: document_reference(
                "Parameter reference",
                &fields.parameter_reference_id,
                &fields.parameter_reference_digest,
            )?,
        },
        DocumentationDeclaration {
            kind: RequiredDocumentation::QualificationReport,
            document: document_reference(
                "Qualification report",
                &fields.qualification_report_id,
                &fields.qualification_report_digest,
            )?,
        },
    ])
    .map_err(|error| error.to_string())?;
    let license = LicenseDeclaration {
        license_id: required_authoring_text("License ID", &fields.license_id)?,
        expression: required_authoring_text("License expression", &fields.license_expression)?,
        scope: fields.license_scope,
        commercial_use_allowed: fields.commercial_use_allowed,
        redistribution_allowed: fields.redistribution_allowed,
        reviewed: fields.license_reviewed,
        notice: document_reference(
            "License notice",
            &fields.license_notice_id,
            &fields.license_notice_digest,
        )?,
    };
    license
        .validate_for_release()
        .map_err(|error| error.to_string())?;

    let migration_plan = optional_document_reference(
        "Migration plan",
        &fields.migration_plan_id,
        &fields.migration_plan_digest,
    )?;
    let affected_consumer_ids = fields
        .affected_consumer_ids
        .split([',', ';', '\n', '\r'])
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect();
    let consumer_impact = ConsumerImpactAssessment::try_new(
        fields.consumer_change,
        required_authoring_text("Consumer impact summary", &fields.consumer_summary)?,
        affected_consumer_ids,
        migration_plan,
        fields.consumer_reviewed,
    )
    .map_err(|error| error.to_string())?;

    let compatibility = CompatibilityAssessment::try_new(
        vec![
            PlatformCompatibilityEvidence {
                platform: QualificationPlatform::Desktop,
                disposition: fields.desktop_compatibility,
                evidence: document_reference(
                    "Desktop compatibility evidence",
                    &fields.desktop_evidence_id,
                    &fields.desktop_evidence_digest,
                )?,
            },
            PlatformCompatibilityEvidence {
                platform: QualificationPlatform::WebAssembly,
                disposition: fields.webassembly_compatibility,
                evidence: document_reference(
                    "WebAssembly compatibility evidence",
                    &fields.webassembly_evidence_id,
                    &fields.webassembly_evidence_digest,
                )?,
            },
        ],
        fields.existing_projects_compatibility,
        fields.compatibility_reviewed,
    )
    .map_err(|error| error.to_string())?;
    compatibility
        .validate_for_release(&consumer_impact)
        .map_err(|error| error.to_string())?;

    let approvals = vec![
        PromotionApproval {
            role: PromotionApprovalRole::ModelOwner,
            approver_id: required_authoring_text("Model owner approver", &fields.model_owner_id)?,
            decision: fields.model_owner_decision,
            decision_revision: parse_object_revision(
                "Model owner decision revision",
                &fields.model_owner_decision_revision,
            )?,
        },
        PromotionApproval {
            role: PromotionApprovalRole::QualificationApprover,
            approver_id: required_authoring_text(
                "Qualification approver",
                &fields.qualification_approver_id,
            )?,
            decision: fields.qualification_approver_decision,
            decision_revision: parse_object_revision(
                "Qualification approver decision revision",
                &fields.qualification_approver_decision_revision,
            )?,
        },
    ];
    let identity = ReleaseCandidateIdentity {
        id: required_authoring_text("Candidate ID", &fields.candidate_id)?,
        model_id: draft.model_name.clone(),
        version: required_authoring_text("Candidate version", &fields.candidate_version)?,
    };
    let mut candidate = ModelReleaseCandidate::try_new(
        identity,
        evidence.source.clone(),
        suite,
        evidence,
        documentation,
        Some(license),
        Some(consumer_impact),
        Some(compatibility),
        approvals,
    )
    .map_err(|error| error.to_string())?;
    let definition = draft.definition().map_err(|diagnostics| {
        diagnostics
            .into_iter()
            .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
            .collect::<Vec<_>>()
            .join("; ")
    })?;
    candidate.definition_source = definition
        .canonical_source()
        .map_err(|error| error.to_string())?
        .into_bytes();
    candidate.definition_metadata = Some(definition.metadata);
    candidate
        .validate_bound(suite, evidence)
        .map_err(|error| error.to_string())?;
    Ok(candidate)
}

pub(super) fn required_authoring_text(label: &str, value: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(format!("{label} is required"))
    } else if trimmed != value {
        Err(format!("{label} must not contain outer whitespace"))
    } else {
        Ok(trimmed.to_owned())
    }
}

pub(super) fn parameter_value_text(value: &ParameterValue) -> String {
    match value {
        ParameterValue::Numeric(value) => value.to_string(),
        ParameterValue::String(value) => value.clone(),
    }
}

pub(super) fn parse_content_digest(label: &str, value: &str) -> Result<ContentDigest, String> {
    value
        .trim()
        .parse::<ContentDigest>()
        .map_err(|error| format!("{label} digest is invalid: {error}"))
}

pub(super) fn document_reference(
    label: &str,
    id: &str,
    digest: &str,
) -> Result<DocumentReference, String> {
    DocumentReference::try_new(
        required_authoring_text(&format!("{label} ID"), id)?,
        parse_content_digest(label, digest)?,
    )
    .map_err(|error| error.to_string())
}

pub(super) fn optional_document_reference(
    label: &str,
    id: &str,
    digest: &str,
) -> Result<Option<DocumentReference>, String> {
    if id.trim().is_empty() && digest.trim().is_empty() {
        return Ok(None);
    }
    if id.trim().is_empty() || digest.trim().is_empty() {
        return Err(format!("{label} ID and digest must be supplied together"));
    }
    document_reference(label, id, digest).map(Some)
}

pub(super) fn parse_object_revision(label: &str, value: &str) -> Result<ObjectRevision, String> {
    let parsed = value
        .trim()
        .parse::<u64>()
        .map_err(|error| format!("{label} must be a positive integer: {error}"))?;
    ObjectRevision::new(parsed).map_err(|error| format!("{label} is invalid: {error}"))
}

pub(super) fn synchronize_parameter_overrides(
    metadata: &mut ModelDefinitionMetadata,
    parameters: &[ParameterDefinition],
) {
    for parameter in parameters {
        let ParameterSource::Overridden { section } = &parameter.source else {
            continue;
        };
        let Some(section) = metadata
            .sections
            .iter_mut()
            .find(|candidate| candidate.name.eq_ignore_ascii_case(section))
        else {
            continue;
        };
        let Some(key) = section
            .overrides
            .keys()
            .find(|name| name.eq_ignore_ascii_case(&parameter.name))
            .cloned()
        else {
            continue;
        };
        section.overrides.insert(key, parameter.value.clone());
    }
}

pub(super) fn parse_optional_finite_bound(
    value: &str,
    field: String,
    diagnostics: &mut Vec<ModelFieldDiagnostic>,
) -> Option<FiniteF64> {
    if value.is_empty() {
        return None;
    }
    match value.parse::<f64>() {
        Ok(value) => match FiniteF64::new(value) {
            Ok(value) => Some(value),
            Err(_) => {
                diagnostics.push(ModelFieldDiagnostic {
                    field,
                    message: "Bound must be finite".to_owned(),
                });
                None
            }
        },
        Err(error) => {
            diagnostics.push(ModelFieldDiagnostic {
                field,
                message: format!("Invalid numeric bound: {error}"),
            });
            None
        }
    }
}

pub(super) fn parameter_definitions(
    metadata: &ModelDefinitionMetadata,
) -> BTreeMap<String, &ParameterDefinition> {
    metadata
        .parameters
        .iter()
        .map(|parameter| (parameter.name.to_ascii_lowercase(), parameter))
        .collect()
}
