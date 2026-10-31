//! Model qualification: which models are qualified, and against what.
//!
//! A model's gate is derived from the evidence that exists, per domain, and
//! never from an aggregate — so a model qualified on one platform and not
//! another reads as exactly that.

use std::collections::{BTreeMap, BTreeSet};

use rspice_app_state::workbench::app_state::model::Model;
use sha2::{Digest as _, Sha256};

use crate::state::model_library::{
    DeviceModel, ModelCorrelationState, ModelLibrary, ModelQualificationState,
    ModelSourceAuthority, ModelSourceEvidenceBinding, QualificationAnalysis, QualificationPlatform,
    QualificationVectorDispositionCause, short_digest,
};
use crate::workbench::documents::model_editor::{self, ModelEditorState};

/// A model's release verdict.
///
/// Readable across the surfaces because it is a fact about the project, not
/// about this page: a simulation plan cannot honestly sign off on a model the
/// gate has not cleared, and the alternative — a second verdict enum on the
/// reading side — is how two surfaces come to disagree about one model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::workbench) enum QualificationGate {
    Qualified,
    Review,
    Unqualified,
    Blocked,
    /// The model is one of the simulator's own compiled-in equation defaults,
    /// and the source-owned gate is not its gate.
    ///
    /// Off the ramp above rather than a shade of [`Self::Blocked`]: a built-in
    /// card has no project source to bind evidence to, so "not source-owned"
    /// is its design and not a finding against it. Deriving it from a failed
    /// source resolution is what once had a project nobody had touched yet
    /// reporting every foundation family as a red release-gate failure.
    EngineOwned,
}

impl QualificationGate {
    pub(in crate::workbench) const fn label(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Review => "review",
            Self::Unqualified => "unqualified",
            Self::Blocked => "blocked",
            Self::EngineOwned => "engine-owned",
        }
    }

    /// Whether the source-owned release gate governs this model at all.
    ///
    /// The counts that read as verdicts — qualified of how many, parity of how
    /// many — are ratios over the governed population, and totalling the
    /// exempt models into their denominators is the same false alarm the
    /// per-model badge used to carry.
    pub(in crate::workbench) const fn is_gate_subject(self) -> bool {
        !matches!(self, Self::EngineOwned)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::workbench) enum QualificationDomain {
    Dc,
    Ac,
    Transient,
    Noise,
}

impl QualificationDomain {
    /// Every domain a vector can be written against, in the order the matrix
    /// columns read.
    ///
    /// Exhaustive over [`QualificationAnalysis`] on purpose. A column set
    /// derived from whatever the corpus happens to hold would drop the empty
    /// domains silently, and "this family declares nothing here" is the one
    /// answer a reader checking coverage came for.
    ///
    /// There is no temperature column because there is no temperature domain:
    /// the analyses a vector may name are DC, AC/charge, transient and noise,
    /// and a temperature is a *condition* one of them is run at rather than an
    /// analysis of its own.
    pub(in crate::workbench) const ALL: [Self; 4] =
        [Self::Dc, Self::Ac, Self::Transient, Self::Noise];

    pub(in crate::workbench) const fn label(self) -> &'static str {
        match self {
            Self::Dc => "DC operating curves",
            Self::Ac => "AC / charge",
            Self::Transient => "Transient",
            Self::Noise => "Noise",
        }
    }

    /// The domain's name as a matrix column heading.
    ///
    /// Shorter than [`Self::label`] because a column heading has a column's
    /// width, and not an abbreviation because the heading is what says which
    /// analyses the cells under it were counted from.
    pub(in crate::workbench) const fn column_label(self) -> &'static str {
        match self {
            Self::Dc => "DC",
            Self::Ac => "AC · charge",
            Self::Transient => "Transient",
            Self::Noise => "Noise",
        }
    }

    const fn from_analysis(analysis: &QualificationAnalysis) -> Self {
        match analysis {
            QualificationAnalysis::DcOperatingPoint | QualificationAnalysis::DcSweep { .. } => {
                Self::Dc
            }
            QualificationAnalysis::AcSweep { .. } => Self::Ac,
            QualificationAnalysis::Transient { .. } => Self::Transient,
            QualificationAnalysis::Noise { .. } => Self::Noise,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(in crate::workbench) struct QualificationDomainAccumulator {
    pub(in crate::workbench) vectors: usize,
    pub(in crate::workbench) references: usize,
    pub(in crate::workbench) quantities: BTreeSet<String>,
    pub(in crate::workbench) tolerance_contracts: BTreeMap<(u64, u64), String>,
    pub(in crate::workbench) evidenced_vectors: usize,
    pub(in crate::workbench) passing_vectors: usize,
    pub(in crate::workbench) open_dispositions: usize,
}

#[derive(Debug, Clone)]
pub(in crate::workbench) struct QualificationDomainSummary {
    pub(in crate::workbench) domain: QualificationDomain,
    pub(in crate::workbench) vectors: usize,
    /// Vectors in this domain that a retained evidence record actually
    /// describes. Carried as a count beside [`Self::passing_vectors`] rather
    /// than folded into the prose below it, because a matrix cell has to tell
    /// "ran and failed" from "never ran" and the two colour differently.
    pub(in crate::workbench) evidenced_vectors: usize,
    pub(in crate::workbench) passing_vectors: usize,
    pub(in crate::workbench) open_dispositions: usize,
    pub(in crate::workbench) reference_coverage: String,
    pub(in crate::workbench) tolerance: String,
    pub(in crate::workbench) disposition: String,
}

/// One model's qualification record, reduced to the facts a gate is made of.
///
/// The fields marked `pub(crate)` are the ones the Simulation Studio's Models
/// page reads to report the gate. They are widened deliberately and narrowly:
/// a second surface may read this summary, but only this module may build one,
/// so there is still exactly one derivation of every number on it.
#[derive(Debug, Clone)]
pub(crate) struct QualificationModelSummary {
    pub(in crate::workbench) key: String,
    pub(crate) library: String,
    pub(crate) model: String,
    pub(in crate::workbench) source_revision: String,
    pub(crate) source_error: Option<String>,
    pub(in crate::workbench) suites: usize,
    pub(crate) vectors: usize,
    pub(crate) evidenced_vectors: usize,
    pub(crate) passing_vectors: usize,
    pub(in crate::workbench) references: usize,
    pub(in crate::workbench) desktop_passing: usize,
    pub(in crate::workbench) wasm_passing: usize,
    pub(in crate::workbench) parity_suites: usize,
    pub(in crate::workbench) evidence_digest: Option<String>,
    pub(crate) open_dispositions: usize,
    /// The two causes an open disposition can have, counted apart.
    ///
    /// The retained record carries a cause and a required action and nothing
    /// else — no severity, no age, no measured miss. So "worst open" is not a
    /// fact this state holds, and the honest split of a pending count is the
    /// one the state does hold: a vector that ran and failed, against a vector
    /// whose evidence no longer describes the current source.
    pub(in crate::workbench) open_failed: usize,
    pub(in crate::workbench) open_stale: usize,
    pub(in crate::workbench) releases: usize,
    pub(in crate::workbench) comparison_available: bool,
    pub(in crate::workbench) correlation_status: String,
    pub(in crate::workbench) correlation_evidence_digest: Option<String>,
    pub(crate) gate: QualificationGate,
    pub(in crate::workbench) domains: Vec<QualificationDomainSummary>,
}

impl QualificationModelSummary {
    /// What this family retains in one domain, or nothing if it declares no
    /// vector there.
    ///
    /// `None` and a summary of zero vectors are different facts and only the
    /// first can happen: [`qualification_domain_summaries`] builds a row per
    /// domain a vector was actually counted into, so an absent domain is one
    /// the family never wrote a vector against.
    pub(in crate::workbench) fn domain(
        &self,
        domain: QualificationDomain,
    ) -> Option<&QualificationDomainSummary> {
        self.domains.iter().find(|summary| summary.domain == domain)
    }
}

pub(crate) fn qualification_summaries(
    model: &Model,
    editor: &ModelEditorState,
) -> Vec<QualificationModelSummary> {
    let mut summaries = Vec::new();
    for library in model.model_library_manager.libraries_sorted() {
        // Authenticating the closure digests every retained byte, and it
        // depends on the library alone. Doing it per model — which is what
        // resolving each model separately does — made this whole function cost
        // one hash of the library per model, every frame the page was open.
        let closure = model_editor::verify_project_library_closure(library, &library.name);
        let mut models = library.models.values().collect::<Vec<_>>();
        models.sort_by(|left, right| left.name.cmp(&right.name));
        for model in models {
            summaries.push(qualification_model_summary(
                editor,
                library,
                model,
                closure.as_ref(),
            ));
        }
    }
    summaries
}

/// One model's release gate, as another surface may read it.
///
/// [`QualificationModelSummary`] is this page's own working record — the
/// per-domain vector counts, the desktop/wasm parity split, the evidence
/// digests — and it moves whenever the qualification page does. This is the
/// narrow projection of it, so a field added there does not silently become
/// another surface's dependency.
#[derive(Debug, Clone)]
pub(in crate::workbench) struct ModelGateFact {
    pub(in crate::workbench) library: String,
    pub(in crate::workbench) model: String,
    pub(in crate::workbench) vectors: usize,
    pub(in crate::workbench) evidenced_vectors: usize,
    pub(in crate::workbench) passing_vectors: usize,
    pub(in crate::workbench) open_dispositions: usize,
    /// This page could not read the model's gate at all, so its counts say
    /// nothing either way and a reader must not total them as passing.
    pub(in crate::workbench) unreadable: bool,
    pub(in crate::workbench) gate: QualificationGate,
}

pub(in crate::workbench) fn qualification_model_summary(
    editor: &ModelEditorState,
    library: &ModelLibrary,
    model: &DeviceModel,
    closure: Result<&model_editor::VerifiedProjectClosure<'_>, &String>,
) -> QualificationModelSummary {
    let key = model_key(&library.name, &model.name);
    let open_draft = editor.draft.as_ref().filter(|draft| {
        draft.library_name.eq_ignore_ascii_case(&library.name)
            && draft.model_name.eq_ignore_ascii_case(&model.name)
    });
    // Asked before the source is resolved, not after it fails to be. An
    // engine-owned library has no project source, so resolving one can only
    // manufacture a "not source-owned" error — and that manufactured error was
    // the whole of the evidence behind sixteen red gates on a fresh project.
    let engine_owned = library.source_authority == ModelSourceAuthority::BuiltIn;
    let (source_revision, source_error, state, source) = if engine_owned {
        (
            "engine-owned".to_owned(),
            None,
            ModelQualificationState::default(),
            None,
        )
    } else {
        let resolved = closure.map_or_else(
            |error| Err(error.clone()),
            |closure| {
                model_editor::resolve_project_model_in_closure(
                    library,
                    &library.name,
                    &model.name,
                    closure,
                )
            },
        );
        match resolved {
            Ok(resolved) => {
                let qualification = open_draft.map_or_else(
                    || resolved.qualification.clone(),
                    |draft| draft.qualification.clone(),
                );
                let source_id =
                    open_draft.map_or_else(|| resolved.source_id, |draft| draft.source_id);
                let source_digest =
                    open_draft.map_or(resolved.model_digest, |draft| draft.base_source_digest);
                let source_revision =
                    open_draft.map_or(resolved.model_revision, |draft| draft.base_source_revision);
                let source = ModelSourceEvidenceBinding::try_new_project_bound(
                    &model.name,
                    source_id,
                    source_digest,
                    source_revision,
                );
                match source {
                    Ok(source) => (
                        if open_draft.is_some_and(|draft| draft.qualification_is_dirty()) {
                            format!(
                                "{}@{} · working qualification",
                                model.name,
                                source_revision.get()
                            )
                        } else {
                            format!("{}@{}", model.name, source_revision.get())
                        },
                        None,
                        qualification,
                        Some(source),
                    ),
                    Err(error) => (
                        "invalid source identity".to_owned(),
                        Some(error.to_string()),
                        qualification,
                        None,
                    ),
                }
            }
            Err(error) => (
                "not source-owned".to_owned(),
                Some(error),
                library
                    .model_qualification
                    .get(&model.name)
                    .cloned()
                    .unwrap_or_default(),
                None,
            ),
        }
    };

    let mut summary = summarize_qualification_state(
        key,
        &library.name,
        model,
        source_revision,
        source_error,
        engine_owned,
        &state,
        source.as_ref(),
    );
    apply_correlation_qualification_contract(
        &mut summary,
        library.model_correlation.get(&model.name),
        source.as_ref(),
    );
    summary
}

pub(in crate::workbench) fn summarize_qualification_state(
    key: String,
    library_name: &str,
    model: &DeviceModel,
    source_revision: String,
    mut source_error: Option<String>,
    engine_owned: bool,
    state: &ModelQualificationState,
    source: Option<&ModelSourceEvidenceBinding>,
) -> QualificationModelSummary {
    // The retained-state check reports a defect in *project* state. An
    // engine-owned model holds none, so running it here could only put a
    // finding on a model the gate does not govern.
    if !engine_owned
        && source_error.is_none()
        && let Err(error) = state.validate_for_model(&model.name)
    {
        source_error = Some(format!("Retained qualification state is invalid: {error}"));
    }
    let exact_suites = source
        .and_then(|source| state.exact_suites_for_source(source).ok())
        .unwrap_or_default();
    let mut vectors = 0usize;
    let mut evidenced_vectors = 0usize;
    let mut passing_vectors = 0usize;
    let mut references = 0usize;
    let mut desktop_passing = 0usize;
    let mut wasm_passing = 0usize;
    let mut parity_suites = 0usize;
    let mut evidence_members = Vec::<(String, u64, crate::product::ContentDigest)>::new();
    let mut all_suites_have_passing_evidence = !exact_suites.is_empty();
    let mut domain_accumulators =
        BTreeMap::<QualificationDomain, QualificationDomainAccumulator>::new();

    for suite in &exact_suites {
        vectors += suite.vectors.len();
        let evidence = source.and_then(|source| {
            state.evidence.iter().find(|evidence| {
                evidence.source == *source
                    && evidence.suite_id.eq_ignore_ascii_case(&suite.id)
                    && evidence.suite_revision == suite.revision
            })
        });
        for vector in &suite.vectors {
            references += vector.references.len();
            let domain = QualificationDomain::from_analysis(&vector.analysis);
            let accumulator = domain_accumulators.entry(domain).or_default();
            accumulator.vectors += 1;
            accumulator.references += vector.references.len();
            for reference in &vector.references {
                accumulator.quantities.insert(reference.quantity.clone());
                let absolute = reference.absolute_tolerance.get();
                let relative = reference.relative_tolerance.get();
                accumulator
                    .tolerance_contracts
                    .entry(qualification_tolerance_key(absolute, relative))
                    .or_insert_with(|| qualification_tolerance_label(absolute, relative));
            }
            if let Some(outcome) = evidence.and_then(|evidence| {
                evidence
                    .vector_outcomes
                    .iter()
                    .find(|outcome| outcome.vector_id.eq_ignore_ascii_case(&vector.id))
            }) {
                accumulator.evidenced_vectors += 1;
                accumulator.passing_vectors += usize::from(outcome.passed);
            }
            accumulator.open_dispositions += source.map_or(0, |source| {
                state
                    .vector_dispositions
                    .iter()
                    .filter(|disposition| {
                        disposition.is_open()
                            && disposition.vector.source == *source
                            && disposition.vector.suite_id.eq_ignore_ascii_case(&suite.id)
                            && disposition.vector.suite_revision == suite.revision
                            && disposition
                                .vector
                                .vector_id
                                .eq_ignore_ascii_case(&vector.id)
                    })
                    .count()
            });
        }

        if let Some(evidence) = evidence {
            evidenced_vectors += evidence.vector_outcomes.len();
            passing_vectors += evidence
                .vector_outcomes
                .iter()
                .filter(|outcome| outcome.passed)
                .count();
            all_suites_have_passing_evidence &= evidence.passed;
            if let Ok(digest) = evidence.content_digest() {
                evidence_members.push((suite.id.clone(), suite.revision.get(), digest));
            }
        } else {
            all_suites_have_passing_evidence = false;
        }

        let platform_run = |platform| {
            source.and_then(|source| {
                state.platform_runs.iter().find(|run| {
                    run.platform == platform
                        && run.source == *source
                        && run.suite_id.eq_ignore_ascii_case(&suite.id)
                        && run.suite_revision == suite.revision
                })
            })
        };
        let desktop = platform_run(QualificationPlatform::Desktop);
        let wasm = platform_run(QualificationPlatform::WebAssembly);
        desktop_passing += desktop.map_or(0, |run| {
            run.vector_outcomes
                .iter()
                .filter(|outcome| outcome.outcome.passed)
                .count()
        });
        wasm_passing += wasm.map_or(0, |run| {
            run.vector_outcomes
                .iter()
                .filter(|outcome| outcome.outcome.passed)
                .count()
        });
        parity_suites += usize::from(
            desktop.is_some_and(|run| run.passed) && wasm.is_some_and(|run| run.passed),
        );
    }

    let open = source.map_or_else(Vec::new, |source| {
        state
            .vector_dispositions
            .iter()
            .filter(|disposition| disposition.is_open() && disposition.vector.source == *source)
            .collect()
    });
    let open_dispositions = open.len();
    let open_failed = open
        .iter()
        .filter(|disposition| disposition.cause == QualificationVectorDispositionCause::Failed)
        .count();
    let open_stale = open_dispositions - open_failed;
    let gate = if engine_owned {
        // Decided before the ramp below, and never by it: with no source to
        // bind evidence to there are no suites, so every branch under this one
        // would read the absence of project evidence as the model's failure to
        // produce it.
        QualificationGate::EngineOwned
    } else if source_error.is_some() {
        QualificationGate::Blocked
    } else if exact_suites.is_empty() {
        QualificationGate::Unqualified
    } else if all_suites_have_passing_evidence
        && parity_suites == exact_suites.len()
        && open_dispositions == 0
    {
        QualificationGate::Qualified
    } else {
        QualificationGate::Review
    };
    let evidence_digest = qualification_evidence_contract_digest(&mut evidence_members);
    let domains = qualification_domain_summaries(domain_accumulators);

    QualificationModelSummary {
        key,
        library: library_name.to_owned(),
        model: model.name.clone(),
        source_revision,
        source_error,
        suites: exact_suites.len(),
        vectors,
        evidenced_vectors,
        passing_vectors,
        references,
        desktop_passing,
        wasm_passing,
        parity_suites,
        evidence_digest,
        open_dispositions,
        open_failed,
        open_stale,
        releases: state.releases.len(),
        comparison_available: !state.candidates.is_empty() && !state.releases.is_empty(),
        correlation_status: "not configured".to_owned(),
        correlation_evidence_digest: None,
        gate,
        domains,
    }
}

pub(in crate::workbench) fn apply_correlation_qualification_contract(
    summary: &mut QualificationModelSummary,
    correlation: Option<&ModelCorrelationState>,
    source: Option<&ModelSourceEvidenceBinding>,
) {
    // Every refusal below reads a missing source identity as a defect, which
    // for an engine-owned model is its design. The gate is not this model's
    // gate, so nothing configured against it may move that verdict.
    if summary.gate == QualificationGate::EngineOwned {
        return;
    }
    let Some(correlation) = correlation.filter(|state| !state.suites.is_empty()) else {
        return;
    };
    if let Err(error) = correlation.validate_for_model(&summary.model) {
        summary.correlation_status = format!("invalid retained state: {error}");
        summary.gate = QualificationGate::Blocked;
        return;
    }
    let Some(source) = source else {
        summary.correlation_status = "source identity unavailable".to_owned();
        summary.gate = QualificationGate::Blocked;
        return;
    };
    let suites = correlation
        .suite_lineages()
        .into_iter()
        .filter(|suite| suite.source == *source)
        .collect::<Vec<_>>();
    if suites.is_empty() {
        summary.correlation_status =
            "configured correlation is stale for this model revision".to_owned();
        if summary.gate != QualificationGate::Blocked {
            summary.gate = QualificationGate::Review;
        }
        return;
    }

    let mut approved = 0usize;
    let mut evidence_members = Vec::new();
    for suite in &suites {
        let evidence = correlation
            .evidence
            .iter()
            .filter(|evidence| {
                evidence.suite_id.eq_ignore_ascii_case(&suite.id)
                    && evidence.suite_revision == suite.revision
                    && evidence.source == *source
                    && evidence.validate_current(suite).is_ok()
            })
            .max_by_key(|evidence| (evidence.reviewed_at_unix_ms, evidence.id.as_str()));
        if let Some(evidence) = evidence.filter(|evidence| evidence.approved()) {
            approved += 1;
            if let Ok(digest) = evidence.content_digest() {
                evidence_members.push((suite.id.clone(), suite.revision.get(), digest));
            }
        }
    }
    summary.correlation_evidence_digest =
        qualification_evidence_contract_digest(&mut evidence_members);
    summary.correlation_status = if approved == suites.len() {
        format!(
            "{} current suite{} approved",
            approved,
            if approved == 1 { "" } else { "s" }
        )
    } else {
        format!(
            "{approved}/{} current suite approvals retained",
            suites.len()
        )
    };
    if approved != suites.len() && summary.gate != QualificationGate::Blocked {
        summary.gate = QualificationGate::Review;
    }
}

pub(in crate::workbench) fn qualification_domain_summaries(
    accumulators: BTreeMap<QualificationDomain, QualificationDomainAccumulator>,
) -> Vec<QualificationDomainSummary> {
    accumulators
        .into_iter()
        .map(|(domain, accumulated)| {
            let reference_coverage = if accumulated.references == 0 {
                "No retained references".to_owned()
            } else {
                let quantity_label = if accumulated.quantities.len() == 1 {
                    "quantity"
                } else {
                    "quantities"
                };
                format!(
                    "{} refs · {} {quantity_label}",
                    accumulated.references,
                    accumulated.quantities.len()
                )
            };
            let tolerance = match accumulated.tolerance_contracts.len() {
                0 => "not declared".to_owned(),
                1 => accumulated
                    .tolerance_contracts
                    .values()
                    .next()
                    .cloned()
                    .expect("one retained tolerance contract"),
                count => format!("{count} declared contracts · varies"),
            };
            let disposition = if accumulated.open_dispositions > 0 {
                format!("{} open", accumulated.open_dispositions)
            } else if accumulated.evidenced_vectors < accumulated.vectors {
                format!(
                    "{} without results",
                    accumulated.vectors - accumulated.evidenced_vectors
                )
            } else if accumulated.passing_vectors == accumulated.vectors {
                "accepted".to_owned()
            } else {
                format!(
                    "{} review",
                    accumulated.vectors - accumulated.passing_vectors
                )
            };
            QualificationDomainSummary {
                domain,
                vectors: accumulated.vectors,
                evidenced_vectors: accumulated.evidenced_vectors,
                passing_vectors: accumulated.passing_vectors,
                open_dispositions: accumulated.open_dispositions,
                reference_coverage,
                tolerance,
                disposition,
            }
        })
        .collect()
}

pub(in crate::workbench) fn qualification_tolerance_key(
    absolute: f64,
    relative: f64,
) -> (u64, u64) {
    let canonical_bits = |value: f64| {
        if value == 0.0 {
            0.0_f64.to_bits()
        } else {
            value.to_bits()
        }
    };
    (canonical_bits(absolute), canonical_bits(relative))
}

pub(in crate::workbench) fn qualification_tolerance_label(absolute: f64, relative: f64) -> String {
    match (absolute > 0.0, relative > 0.0) {
        (false, false) => "exact".to_owned(),
        (true, false) => format!("{absolute:.3e} absolute"),
        (false, true) => format!("{:.4}% relative", relative * 100.0),
        (true, true) => format!("{absolute:.3e} abs · {:.4}% rel", relative * 100.0),
    }
}

pub(in crate::workbench) fn qualification_evidence_contract_digest(
    members: &mut Vec<(String, u64, crate::product::ContentDigest)>,
) -> Option<String> {
    if members.is_empty() {
        return None;
    }
    members.sort_by(|left, right| {
        left.0
            .to_ascii_lowercase()
            .cmp(&right.0.to_ascii_lowercase())
            .then_with(|| left.1.cmp(&right.1))
            .then_with(|| left.2.cmp(&right.2))
    });
    if let [(suite, revision, digest)] = members.as_slice() {
        return Some(format!(
            "{suite}@{revision} · {}",
            short_digest(&digest.to_string())
        ));
    }

    let mut hasher = Sha256::new();
    hasher.update(b"rspice:model-qualification-evidence-set:v1\0");
    let member_count = members.len();
    for (suite, revision, digest) in members.iter() {
        hasher.update((suite.len() as u64).to_le_bytes());
        hasher.update(suite.as_bytes());
        hasher.update(revision.to_le_bytes());
        hasher.update(digest.as_bytes());
    }
    let digest = crate::product::ContentDigest::from_bytes(hasher.finalize().into());
    Some(format!(
        "{} suites · {}",
        member_count,
        short_digest(&digest.to_string())
    ))
}

fn model_key(library: &str, item: &str) -> String {
    format!("{library}\u{1f}{item}")
}
