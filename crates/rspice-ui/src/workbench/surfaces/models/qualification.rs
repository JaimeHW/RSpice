//! The qualification page: which models are qualified, and against what.
//!
//! A model's gate is derived from the evidence that exists, per domain, and
//! never from an aggregate — so a model qualified on one platform and not
//! another reads as exactly that. Every blocked action states the reason it is
//! blocked rather than being hidden, because "no button" and "not qualified"
//! are different facts to an engineer looking for why.

use super::*;

/// A model's release verdict.
///
/// Readable across the surfaces because it is a fact about the project, not
/// about this page: a simulation plan cannot honestly sign off on a model the
/// gate has not cleared, and the alternative — a second verdict enum on the
/// reading side — is how two surfaces come to disagree about one model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::workbench::surfaces) enum QualificationGate {
    Qualified,
    Review,
    Unqualified,
    Blocked,
}

impl QualificationGate {
    pub(in crate::workbench::surfaces) const fn label(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Review => "review",
            Self::Unqualified => "unqualified",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum QualificationDomain {
    Dc,
    Ac,
    Transient,
    Noise,
}

impl QualificationDomain {
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Dc => "DC operating curves",
            Self::Ac => "AC / charge",
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
pub(super) struct QualificationDomainAccumulator {
    pub(super) vectors: usize,
    pub(super) references: usize,
    pub(super) quantities: BTreeSet<String>,
    pub(super) tolerance_contracts: BTreeMap<(u64, u64), String>,
    pub(super) evidenced_vectors: usize,
    pub(super) passing_vectors: usize,
    pub(super) open_dispositions: usize,
}

#[derive(Debug, Clone)]
pub(super) struct QualificationDomainSummary {
    pub(super) domain: QualificationDomain,
    pub(super) vectors: usize,
    pub(super) reference_coverage: String,
    pub(super) tolerance: String,
    pub(super) disposition: String,
}

/// One model's qualification record, reduced to the facts a gate is made of.
///
/// The fields marked `pub(crate)` are the ones the Simulation Studio's Models
/// page reads to report the gate. They are widened deliberately and narrowly:
/// a second surface may read this summary, but only this module may build one,
/// so there is still exactly one derivation of every number on it.
#[derive(Debug, Clone)]
pub(crate) struct QualificationModelSummary {
    pub(super) key: String,
    pub(crate) library: String,
    pub(crate) model: String,
    pub(super) source_revision: String,
    pub(crate) source_error: Option<String>,
    pub(super) suites: usize,
    pub(crate) vectors: usize,
    pub(crate) evidenced_vectors: usize,
    pub(crate) passing_vectors: usize,
    pub(super) references: usize,
    pub(super) desktop_passing: usize,
    pub(super) wasm_passing: usize,
    pub(super) parity_suites: usize,
    pub(super) evidence_digest: Option<String>,
    pub(crate) open_dispositions: usize,
    pub(super) releases: usize,
    pub(super) comparison_available: bool,
    pub(super) correlation_status: String,
    pub(super) correlation_evidence_digest: Option<String>,
    pub(crate) gate: QualificationGate,
    pub(super) domains: Vec<QualificationDomainSummary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum QualificationPageAction {
    ReviewVectors,
    ReviewReleaseBinding,
    RunSuite,
    CompareRelease,
    OpenCorrelation,
}

pub(crate) fn qualification_summaries(app: &RSpiceApp) -> Vec<QualificationModelSummary> {
    let mut summaries = Vec::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        // Authenticating the closure digests every retained byte, and it
        // depends on the library alone. Doing it per model — which is what
        // resolving each model separately does — made this whole function cost
        // one hash of the library per model, every frame the page was open.
        let closure = model_editor::verify_project_library_closure(library, &library.name);
        let mut models = library.models.values().collect::<Vec<_>>();
        models.sort_by(|left, right| left.name.cmp(&right.name));
        for model in models {
            summaries.push(qualification_model_summary(
                app,
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
pub(in crate::workbench::surfaces) struct ModelGateFact {
    pub(in crate::workbench::surfaces) library: String,
    pub(in crate::workbench::surfaces) model: String,
    pub(in crate::workbench::surfaces) vectors: usize,
    pub(in crate::workbench::surfaces) evidenced_vectors: usize,
    pub(in crate::workbench::surfaces) passing_vectors: usize,
    pub(in crate::workbench::surfaces) open_dispositions: usize,
    /// This page could not read the model's gate at all, so its counts say
    /// nothing either way and a reader must not total them as passing.
    pub(in crate::workbench::surfaces) unreadable: bool,
    pub(in crate::workbench::surfaces) gate: QualificationGate,
}

/// Every model in the closure, projected to what a reading surface may know.
pub(in crate::workbench::surfaces) fn model_gate_facts(app: &RSpiceApp) -> Vec<ModelGateFact> {
    qualification_summaries(app)
        .into_iter()
        .map(|summary| ModelGateFact {
            library: summary.library,
            model: summary.model,
            vectors: summary.vectors,
            evidenced_vectors: summary.evidenced_vectors,
            passing_vectors: summary.passing_vectors,
            open_dispositions: summary.open_dispositions,
            unreadable: summary.source_error.is_some(),
            gate: summary.gate,
        })
        .collect()
}

pub(super) fn qualification_model_summary(
    app: &RSpiceApp,
    library: &ModelLibrary,
    model: &DeviceModel,
    closure: Result<&model_editor::VerifiedProjectClosure<'_>, &String>,
) -> QualificationModelSummary {
    let key = model_key(&library.name, &model.name);
    let open_draft = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .filter(|draft| {
            draft.library_name.eq_ignore_ascii_case(&library.name)
                && draft.model_name.eq_ignore_ascii_case(&model.name)
        });
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
    let (source_revision, source_error, state, source) = match resolved {
        Ok(resolved) => {
            let qualification = open_draft.map_or_else(
                || resolved.qualification.clone(),
                |draft| draft.qualification.clone(),
            );
            let source_id = open_draft.map_or_else(|| resolved.source_id, |draft| draft.source_id);
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
    };

    let mut summary = summarize_qualification_state(
        key,
        &library.name,
        model,
        source_revision,
        source_error,
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

pub(super) fn summarize_qualification_state(
    key: String,
    library_name: &str,
    model: &DeviceModel,
    source_revision: String,
    mut source_error: Option<String>,
    state: &ModelQualificationState,
    source: Option<&ModelSourceEvidenceBinding>,
) -> QualificationModelSummary {
    if source_error.is_none()
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

    let open_dispositions = source.map_or(0, |source| {
        state
            .vector_dispositions
            .iter()
            .filter(|disposition| disposition.is_open() && disposition.vector.source == *source)
            .count()
    });
    let gate = if source_error.is_some() {
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
        releases: state.releases.len(),
        comparison_available: !state.candidates.is_empty() && !state.releases.is_empty(),
        correlation_status: "not configured".to_owned(),
        correlation_evidence_digest: None,
        gate,
        domains,
    }
}

pub(super) fn apply_correlation_qualification_contract(
    summary: &mut QualificationModelSummary,
    correlation: Option<&ModelCorrelationState>,
    source: Option<&ModelSourceEvidenceBinding>,
) {
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
            "configured evidence is stale for this model revision".to_owned();
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

pub(super) fn qualification_domain_summaries(
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
                    "{} without evidence",
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
                reference_coverage,
                tolerance,
                disposition,
            }
        })
        .collect()
}

pub(super) fn qualification_tolerance_key(absolute: f64, relative: f64) -> (u64, u64) {
    let canonical_bits = |value: f64| {
        if value == 0.0 {
            0.0_f64.to_bits()
        } else {
            value.to_bits()
        }
    };
    (canonical_bits(absolute), canonical_bits(relative))
}

pub(super) fn qualification_tolerance_label(absolute: f64, relative: f64) -> String {
    match (absolute > 0.0, relative > 0.0) {
        (false, false) => "exact".to_owned(),
        (true, false) => format!("{absolute:.3e} absolute"),
        (false, true) => format!("{:.4}% relative", relative * 100.0),
        (true, true) => format!("{absolute:.3e} abs · {:.4}% rel", relative * 100.0),
    }
}

pub(super) fn qualification_evidence_contract_digest(
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

pub(super) fn selected_qualification_summary<'a>(
    app: &RSpiceApp,
    summaries: &'a [QualificationModelSummary],
) -> Option<&'a QualificationModelSummary> {
    let library = app
        .state
        .model_library_manager
        .selected_library
        .as_deref()?;
    let model = app.state.workbench.selected_model.as_deref()?;
    summaries.iter().find(|summary| {
        summary.library.eq_ignore_ascii_case(library) && summary.model.eq_ignore_ascii_case(model)
    })
}

pub(super) fn qualification_action_block_reason(
    app: &RSpiceApp,
    selected: Option<&QualificationModelSummary>,
    action: QualificationPageAction,
) -> Option<String> {
    let Some(selected) = selected else {
        return Some("Select a model family first".to_owned());
    };
    if !app.state.project_lifecycle.project_open {
        return Some("Open a project before using model qualification".to_owned());
    }
    if selected.source_error.is_some() {
        return Some("Select an exact project-owned model revision".to_owned());
    }
    if app
        .state
        .workbench
        .model_editor
        .qualification_execution
        .is_some()
    {
        return Some(
            "Finish or cancel the active model qualification run before changing workflows"
                .to_owned(),
        );
    }
    if let Some(draft) = app.state.workbench.model_editor.draft.as_ref() {
        let same_model = draft.library_name.eq_ignore_ascii_case(&selected.library)
            && draft.model_name.eq_ignore_ascii_case(&selected.model);
        if draft.is_dirty() && !same_model {
            return Some(format!(
                "Save or discard unsaved model candidate '{}/{}' first",
                draft.library_name, draft.model_name
            ));
        }
        if same_model
            && draft.definition_is_dirty()
            && matches!(
                action,
                QualificationPageAction::RunSuite | QualificationPageAction::CompareRelease
            )
        {
            return Some(
                "Save the changed model definition before running or comparing qualification"
                    .to_owned(),
            );
        }
    }
    match action {
        QualificationPageAction::ReviewVectors
        | QualificationPageAction::ReviewReleaseBinding
        | QualificationPageAction::OpenCorrelation => None,
        QualificationPageAction::RunSuite if app.state.workbench.safe_mode.project_read_only() => {
            Some("Qualification cannot run while the project is read-only".to_owned())
        }
        QualificationPageAction::RunSuite if selected.suites == 0 => {
            Some("Author at least one executable qualification suite first".to_owned())
        }
        QualificationPageAction::RunSuite => None,
        QualificationPageAction::CompareRelease if !selected.comparison_available => {
            Some("The selected model has no immutable approved release to compare".to_owned())
        }
        QualificationPageAction::CompareRelease => None,
    }
}

pub(super) fn execute_qualification_action(app: &mut RSpiceApp, action: QualificationPageAction) {
    let summaries = qualification_summaries(app);
    let selected = selected_qualification_summary(app, &summaries);
    if let Some(reason) = qualification_action_block_reason(app, selected, action) {
        app.state.push_user_message(ConsoleMessage::warning(reason));
        return;
    }
    if action == QualificationPageAction::OpenCorrelation {
        if let Err(error) = app.state.workbench.navigate(
            SurfaceRoute::surface(SurfaceId::ModelCorrelation),
            RouteTransitionSource::User,
        ) {
            app.state.push_user_message(ConsoleMessage::warning(format!(
                "Measurement correlation cannot be opened: {error}"
            )));
        }
        return;
    }
    let Some(library) = app.state.model_library_manager.selected_library.clone() else {
        app.state.push_user_message(ConsoleMessage::warning(
            "Select a model family before opening qualification.",
        ));
        return;
    };
    let Some(model) = app.state.workbench.selected_model.clone() else {
        app.state.push_user_message(ConsoleMessage::warning(
            "Select a model family before opening qualification.",
        ));
        return;
    };
    if let Err(error) = model_editor::open_project_model(app, &library, &model) {
        app.state.push_user_message(ConsoleMessage::warning(format!(
            "Qualification cannot be opened: {error}"
        )));
        return;
    }
    if let Err(error) = app.state.workbench.navigate(
        SurfaceRoute::surface(SurfaceId::ModelEditor),
        RouteTransitionSource::User,
    ) {
        app.state.push_user_message(ConsoleMessage::warning(format!(
            "Qualification editor cannot be shown: {error}"
        )));
        return;
    }

    match action {
        QualificationPageAction::ReviewVectors => {
            app.state.workbench.model_editor.active_section = ModelEditorSection::Tests;
            app.state.workbench.model_editor.qualification_plan_open = true;
        }
        QualificationPageAction::ReviewReleaseBinding => {
            app.state.workbench.model_editor.active_section = ModelEditorSection::Release;
        }
        QualificationPageAction::RunSuite => {
            app.state.workbench.model_editor.active_section = ModelEditorSection::Tests;
            model_editor::validate_open_candidate(app);
            let command = Command::ModelRunQualificationTests;
            if command.is_enabled(app) {
                command.execute(app);
            } else if let CommandAvailability::Disabled(reason) = command.availability(app) {
                app.state.push_user_message(ConsoleMessage::warning(format!(
                    "Qualification cannot run: {reason}."
                )));
            }
        }
        QualificationPageAction::CompareRelease => {
            app.state.workbench.model_editor.active_section = ModelEditorSection::Release;
            let command = Command::ModelCompareRelease;
            if command.is_enabled(app) {
                command.execute(app);
            } else if let CommandAvailability::Disabled(reason) = command.availability(app) {
                app.state.push_user_message(ConsoleMessage::warning(format!(
                    "Approved-model comparison is unavailable: {reason}."
                )));
            }
        }
        QualificationPageAction::OpenCorrelation => {}
    }
}
