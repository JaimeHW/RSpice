//! The qualification page: which models are qualified, and against what.
//!
//! A model's gate is derived from the evidence that exists, per domain, and
//! never from an aggregate — so a model qualified on one platform and not
//! another reads as exactly that. Every blocked action states the reason it is
//! blocked rather than being hidden, because "no button" and "not qualified"
//! are different facts to an engineer looking for why.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum QualificationGate {
    Qualified,
    Review,
    Unqualified,
    Blocked,
}

impl QualificationGate {
    const fn label(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Review => "review",
            Self::Unqualified => "unqualified",
            Self::Blocked => "blocked",
        }
    }

    fn color(self, tokens: &Tokens) -> Color32 {
        match self {
            Self::Qualified => tokens.color.ok,
            Self::Review | Self::Unqualified => tokens.color.warn,
            Self::Blocked => tokens.color.err,
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
    pub(super) tone: QualificationGate,
}

#[derive(Debug, Clone)]
pub(super) struct QualificationModelSummary {
    pub(super) key: String,
    pub(super) library: String,
    pub(super) model: String,
    pub(super) source_revision: String,
    pub(super) source_error: Option<String>,
    pub(super) suites: usize,
    pub(super) vectors: usize,
    pub(super) evidenced_vectors: usize,
    pub(super) passing_vectors: usize,
    pub(super) dc_vectors: usize,
    pub(super) ac_vectors: usize,
    pub(super) transient_vectors: usize,
    pub(super) noise_vectors: usize,
    pub(super) temperature_points: usize,
    pub(super) references: usize,
    pub(super) desktop_passing: usize,
    pub(super) wasm_passing: usize,
    pub(super) parity_suites: usize,
    pub(super) worst_relative_error: Option<f64>,
    pub(super) evidence_digest: Option<String>,
    pub(super) open_dispositions: usize,
    pub(super) releases: usize,
    pub(super) comparison_available: bool,
    pub(super) correlation_status: String,
    pub(super) correlation_evidence_digest: Option<String>,
    pub(super) gate: QualificationGate,
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

pub(super) fn qualification(ui: &mut Ui, app: &mut RSpiceApp) {
    let available = ui.available_size();
    let required_height = qualification_required_content_height(available.x);
    if available.y >= required_height {
        qualification_content(ui, app);
        return;
    }

    let (viewport, _) = ui.allocate_exact_size(available.max(Vec2::splat(1.0)), Sense::hover());
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(Layout::top_down(Align::Min)),
    );
    child.spacing_mut().item_spacing = Vec2::ZERO;
    ScrollArea::vertical()
        .id_salt("models.qualification.workspace")
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            let content_width = ui.available_width().max(1.0);
            ui.allocate_ui_with_layout(
                egui::vec2(
                    content_width,
                    qualification_required_content_height(content_width),
                ),
                Layout::top_down(Align::Min),
                |ui| qualification_content(ui, app),
            );
        });
}

pub(super) fn qualification_required_content_height(width: f32) -> f32 {
    if width <= MODEL_SUMMARY_BREAKPOINT {
        QUALIFICATION_STACKED_MIN_CONTENT_H
    } else {
        QUALIFICATION_MIN_CONTENT_H
    }
}

pub(super) fn qualification_content(ui: &mut Ui, app: &mut RSpiceApp) {
    let summaries = qualification_summaries(app);
    let selected = selected_qualification_summary(app, &summaries).cloned();
    let selected_vector_count = selected.as_ref().map_or(0, |summary| summary.vectors);
    let review_label = format!("Review {selected_vector_count} vectors");
    let run_blocker = qualification_action_block_reason(
        app,
        selected.as_ref(),
        QualificationPageAction::RunSuite,
    );
    let review_blocker = qualification_action_block_reason(
        app,
        selected.as_ref(),
        QualificationPageAction::ReviewVectors,
    );
    let compare_blocker = qualification_action_block_reason(
        app,
        selected.as_ref(),
        QualificationPageAction::CompareRelease,
    );
    let mut requested_action = None;

    surface_title_with_action_reserve(
        ui,
        "Model qualification · source-owned release gate",
        "Model qualification",
        "Versioned vectors, retained golden references, runtime parity, tolerances, and governed dispositions. Release closure consumes only source-owned evidence.",
        true,
        430.0,
        |ui| {
            let run_enabled = run_blocker.is_none();
            let mut run_button = Button::new("Run suite").enabled(run_enabled);
            if run_enabled {
                run_button = run_button.accent();
            }
            let run = run_button.show(ui);
            let run_clicked = run.clicked();
            if let Some(reason) = run_blocker.as_deref() {
                run.on_disabled_hover_text(reason);
            }
            if run_clicked {
                requested_action = Some(QualificationPageAction::RunSuite);
            }

            let review = Button::new(&review_label)
                .enabled(review_blocker.is_none())
                .show(ui);
            let review_clicked = review.clicked();
            if let Some(reason) = review_blocker.as_deref() {
                review.on_disabled_hover_text(reason);
            }
            if review_clicked {
                requested_action = Some(QualificationPageAction::ReviewVectors);
            }

            let compare = Button::new("Compare approved model")
                .enabled(compare_blocker.is_none())
                .show(ui);
            let compare_clicked = compare.clicked();
            if let Some(reason) = compare_blocker.as_deref() {
                compare.on_disabled_hover_text(reason);
            }
            if compare_clicked {
                requested_action = Some(QualificationPageAction::CompareRelease);
            }

            ui.menu_button("More", |ui| {
                if ui.button("Measurement correlation").clicked() {
                    requested_action = Some(QualificationPageAction::OpenCorrelation);
                    ui.close();
                }
            });
        },
    );

    if let Some(action) = requested_action {
        execute_qualification_action(app, action);
    }

    let t = Tokens::get(ui.ctx());
    let total_vectors = summaries
        .iter()
        .map(|summary| summary.vectors)
        .sum::<usize>();
    let passing_vectors = summaries
        .iter()
        .map(|summary| summary.passing_vectors)
        .sum::<usize>();
    let evidence_vectors = summaries
        .iter()
        .filter(|summary| summary.source_error.is_none())
        .map(|summary| summary.evidenced_vectors)
        .sum::<usize>();
    let qualified_models = summaries
        .iter()
        .filter(|summary| summary.gate == QualificationGate::Qualified)
        .count();
    let source_owned_models = summaries
        .iter()
        .filter(|summary| summary.source_error.is_none())
        .count();
    let parity_models = summaries
        .iter()
        .filter(|summary| {
            summary.suites > 0
                && summary.parity_suites == summary.suites
                && summary.source_error.is_none()
        })
        .count();
    let worst_relative_error = summaries
        .iter()
        .filter_map(|summary| summary.worst_relative_error)
        .max_by(f64::total_cmp);
    let metrics = [
        Kpi::new(
            "Vectors passing",
            format!("{passing_vectors} / {total_vectors}"),
            format!(
                "{} source dispositions pending",
                summaries
                    .iter()
                    .map(|summary| summary.open_dispositions)
                    .sum::<usize>()
            ),
            if total_vectors > 0 && passing_vectors == total_vectors {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
        Kpi::new(
            "Reference coverage",
            format!("{evidence_vectors} / {total_vectors}"),
            "exact retained golden-reference evidence",
            if total_vectors > 0 && evidence_vectors == total_vectors {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
        Kpi::new(
            "Worst deviation",
            worst_relative_error
                .map(|value| format!("{:.4}%", value * 100.0))
                .unwrap_or_else(|| "No evidence".to_owned()),
            format!("{qualified_models} qualified models"),
            if worst_relative_error.is_some() {
                t.color.text
            } else {
                t.color.warn
            },
        ),
        Kpi::new(
            "Interpreter parity",
            if source_owned_models == 0 {
                "No source".to_owned()
            } else {
                format!("{parity_models} / {source_owned_models}")
            },
            if source_owned_models == 0 {
                "no project-owned models"
            } else {
                "desktop · WebAssembly"
            },
            if source_owned_models > 0 && parity_models == source_owned_models {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
    ];
    kpi_strip(ui, &metrics);

    let detail_height = if ui.available_width() <= MODEL_SUMMARY_BREAKPOINT {
        460.0 + qualification_gate_footer_height(ui, ui.available_width())
    } else {
        330.0
    };
    let table_height = (ui.available_height() - detail_height)
        .clamp(135.0, 320.0)
        .min(ui.available_height().max(1.0));
    if let Some(event) = qualification_suite_table(
        ui,
        app,
        &summaries,
        egui::vec2(ui.available_width(), table_height),
    ) {
        let (library, model) = split_model_key(&event.key);
        app.state.model_library_manager.select_library(library);
        app.state.workbench.selected_model = Some(model.to_owned());
        if event.review {
            execute_qualification_action(app, QualificationPageAction::ReviewVectors);
        }
    }

    let selected_detail = selected_qualification_summary(app, &summaries);
    if let Some(action) = qualification_detail(ui, app, selected_detail, detail_height) {
        execute_qualification_action(app, action);
    }
}

pub(super) fn qualification_summaries(app: &RSpiceApp) -> Vec<QualificationModelSummary> {
    let mut summaries = Vec::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        let mut models = library.models.values().collect::<Vec<_>>();
        models.sort_by(|left, right| left.name.cmp(&right.name));
        for model in models {
            summaries.push(qualification_model_summary(app, library, model));
        }
    }
    summaries
}

pub(super) fn qualification_model_summary(
    app: &RSpiceApp,
    library: &ModelLibrary,
    model: &DeviceModel,
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
    let resolved = model_editor::resolve_project_model_for_editor(
        &app.state.model_library_manager,
        &library.name,
        &model.name,
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
    let mut dc_vectors = 0usize;
    let mut ac_vectors = 0usize;
    let mut transient_vectors = 0usize;
    let mut noise_vectors = 0usize;
    let mut temperatures = BTreeSet::<String>::new();
    let mut references = 0usize;
    let mut desktop_passing = 0usize;
    let mut wasm_passing = 0usize;
    let mut parity_suites = 0usize;
    let mut worst_relative_error: Option<f64> = None;
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

            match &vector.analysis {
                QualificationAnalysis::DcOperatingPoint | QualificationAnalysis::DcSweep { .. } => {
                    dc_vectors += 1;
                }
                QualificationAnalysis::AcSweep { .. } => ac_vectors += 1,
                QualificationAnalysis::Transient { .. } => transient_vectors += 1,
                QualificationAnalysis::Noise {
                    temperature_kelvin, ..
                } => {
                    noise_vectors += 1;
                    temperatures.insert(format!("{:.6}", temperature_kelvin.get()));
                }
            }
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
            for reference in evidence
                .vector_outcomes
                .iter()
                .flat_map(|outcome| &outcome.platforms)
                .flat_map(|platform| &platform.references)
            {
                worst_relative_error = Some(
                    worst_relative_error.map_or(reference.relative_error.get(), |current| {
                        current.max(reference.relative_error.get())
                    }),
                );
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
        dc_vectors,
        ac_vectors,
        transient_vectors,
        noise_vectors,
        temperature_points: temperatures.len(),
        references,
        desktop_passing,
        wasm_passing,
        parity_suites,
        worst_relative_error,
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
            let (disposition, tone) = if accumulated.open_dispositions > 0 {
                (
                    format!("{} open", accumulated.open_dispositions),
                    QualificationGate::Review,
                )
            } else if accumulated.evidenced_vectors < accumulated.vectors {
                (
                    format!(
                        "{} without evidence",
                        accumulated.vectors - accumulated.evidenced_vectors
                    ),
                    QualificationGate::Unqualified,
                )
            } else if accumulated.passing_vectors == accumulated.vectors {
                ("accepted".to_owned(), QualificationGate::Qualified)
            } else {
                (
                    format!(
                        "{} review",
                        accumulated.vectors - accumulated.passing_vectors
                    ),
                    QualificationGate::Review,
                )
            };
            QualificationDomainSummary {
                domain,
                vectors: accumulated.vectors,
                reference_coverage,
                tolerance,
                disposition,
                tone,
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

#[derive(Debug, Clone)]
pub(super) struct QualificationSuiteTableEvent {
    pub(super) key: String,
    pub(super) review: bool,
}

pub(super) fn qualification_suite_table(
    ui: &mut Ui,
    app: &RSpiceApp,
    summaries: &[QualificationModelSummary],
    size: Vec2,
) -> Option<QualificationSuiteTableEvent> {
    let t = Tokens::get(ui.ctx());
    let aggregate_gate = if summaries.is_empty() {
        QualificationGate::Unqualified
    } else if summaries
        .iter()
        .any(|summary| summary.gate == QualificationGate::Blocked)
    {
        QualificationGate::Blocked
    } else if summaries
        .iter()
        .any(|summary| summary.gate != QualificationGate::Qualified)
    {
        QualificationGate::Review
    } else {
        QualificationGate::Qualified
    };
    let columns = [
        ("Model family", 0.17),
        ("DC", 0.07),
        ("AC / charge", 0.12),
        ("Transient", 0.10),
        ("Noise", 0.08),
        ("Temperature", 0.14),
        ("References", 0.12),
        ("Gate", 0.10),
        ("", 0.10),
    ];
    let mut event = None;
    table_card(
        ui,
        "Qualification suites",
        Some((aggregate_gate.label(), aggregate_gate.color(&t))),
        size,
        |ui, table_size| {
            let table_size = table_size.max(Vec2::splat(1.0));
            let (viewport, _) = ui.allocate_exact_size(table_size, Sense::hover());
            ui.painter().rect_filled(viewport, 0.0, t.color.bg_panel);
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(viewport)
                    .layout(Layout::top_down(Align::Min)),
            );
            child.spacing_mut().item_spacing = Vec2::ZERO;
            let table_width = viewport.width().max(760.0);
            ScrollArea::both()
                .id_salt("models.qualification.suites")
                .auto_shrink([false, false])
                .show(&mut child, |ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    ui.set_min_width(table_width);
                    let (head, _) = ui
                        .allocate_exact_size(egui::vec2(table_width, TABLE_HEAD_H), Sense::hover());
                    ui.painter().rect_filled(head, 0.0, t.color.bg_panel_2);
                    ui.painter().hline(
                        head.x_range(),
                        head.bottom(),
                        Stroke::new(1.0, t.color.border),
                    );
                    paint_table_cells(ui, head, &columns, None, true);

                    for summary in summaries {
                        let selected = app.state.workbench.selected_model.as_deref()
                            == Some(summary.model.as_str())
                            && app.state.model_library_manager.selected_library.as_deref()
                                == Some(summary.library.as_str());
                        let cells = [
                            DataCell::mono(&summary.model),
                            DataCell::mono(summary.dc_vectors.to_string()),
                            DataCell::mono(summary.ac_vectors.to_string()),
                            DataCell::mono(summary.transient_vectors.to_string()),
                            DataCell::mono(summary.noise_vectors.to_string()),
                            DataCell::mono(if summary.temperature_points == 0 {
                                "not declared".to_owned()
                            } else {
                                format!("{} points", summary.temperature_points)
                            }),
                            DataCell::mono(format!("{} refs", summary.references)),
                            DataCell::mono_colored(summary.gate.label(), summary.gate.color(&t)),
                        ];
                        let row_height = t.metrics.row_h.max(30.0);
                        let (row, _) = ui.allocate_exact_size(
                            egui::vec2(table_width, row_height),
                            Sense::hover(),
                        );
                        let response = ui.interact(
                            row,
                            ui.id()
                                .with(("models.qualification.suite", summary.key.as_str())),
                            Sense::click(),
                        );
                        let suite_label = summary.key.clone();
                        response.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::SelectableLabel,
                                ui.is_enabled(),
                                selected,
                                suite_label.clone(),
                            )
                        });
                        if selected {
                            ui.painter().rect_filled(row, 0.0, t.color.accent_dim);
                            ui.painter().rect_filled(
                                Rect::from_min_max(
                                    row.min,
                                    egui::pos2(row.left() + 2.0, row.bottom()),
                                ),
                                0.0,
                                t.color.accent,
                            );
                        } else if response.hovered() {
                            ui.painter().rect_filled(row, 0.0, t.color.bg_hover);
                        }
                        ui.painter().hline(
                            row.x_range(),
                            row.bottom(),
                            Stroke::new(1.0, t.color.border.gamma_multiply(0.75)),
                        );
                        paint_table_cells(ui, row, &columns, Some(&cells), false);

                        let action_left = row.left()
                            + row.width()
                                * columns[..columns.len() - 1]
                                    .iter()
                                    .map(|(_, fraction)| *fraction)
                                    .sum::<f32>();
                        let action_rect = Rect::from_min_max(
                            egui::pos2(action_left + 4.0, row.top() + 3.0),
                            egui::pos2(row.right() - 4.0, row.bottom() - 3.0),
                        );
                        let mut action_ui =
                            ui.new_child(egui::UiBuilder::new().max_rect(action_rect).layout(
                                Layout::centered_and_justified(egui::Direction::LeftToRight),
                            ));
                        let blocker = qualification_action_block_reason(
                            app,
                            Some(summary),
                            QualificationPageAction::ReviewVectors,
                        );
                        let action = Button::new(if summary.gate == QualificationGate::Qualified {
                            "Inspect"
                        } else {
                            "Review"
                        })
                        .enabled(blocker.is_none())
                        .show(&mut action_ui);
                        let action_clicked = action.clicked();
                        if let Some(reason) = blocker.as_deref() {
                            action.on_disabled_hover_text(reason);
                        }

                        if action_clicked || response.double_clicked() {
                            event = Some(QualificationSuiteTableEvent {
                                key: summary.key.clone(),
                                review: true,
                            });
                        } else if response.clicked() {
                            event = Some(QualificationSuiteTableEvent {
                                key: summary.key.clone(),
                                review: false,
                            });
                        }
                        theme::paint_focus_ring_outset(ui, &response, row);
                    }

                    if summaries.is_empty() {
                        let empty_height =
                            (viewport.height() - TABLE_HEAD_H).max(t.metrics.row_h.max(44.0));
                        let (empty, _) = ui.allocate_exact_size(
                            egui::vec2(table_width, empty_height),
                            Sense::hover(),
                        );
                        ui.painter().text(
                            empty.center(),
                            Align2::CENTER_CENTER,
                            "No loaded model records are available for qualification.",
                            theme::sans(tokens::FS_0, FontWeight::Regular),
                            t.color.text_dim,
                        );
                    }
                });
        },
    );
    event
}

pub(super) fn qualification_detail(
    ui: &mut Ui,
    app: &RSpiceApp,
    selected: Option<&QualificationModelSummary>,
    height: f32,
) -> Option<QualificationPageAction> {
    let width = ui.available_width().max(1.0);
    let narrow = width <= MODEL_SUMMARY_BREAKPOINT;
    let footer_height = qualification_gate_footer_height(ui, width);
    let body_height = (height - footer_height).max(1.0);
    let mut requested_action = None;

    if narrow {
        let domain_height = (body_height - 250.0).max(190.0).min(body_height);
        qualification_domain_table(ui, selected, egui::vec2(width, domain_height));
        let contract_height = (body_height - domain_height).max(1.0);
        if qualification_contract_card(ui, app, selected, egui::vec2(width, contract_height)) {
            requested_action = Some(QualificationPageAction::ReviewVectors);
        }
    } else {
        let (body, _) = ui.allocate_exact_size(egui::vec2(width, body_height), Sense::hover());
        let left_width = (body.width() * 0.61).floor();
        let left_rect = Rect::from_min_max(
            body.left_top(),
            egui::pos2(body.left() + left_width, body.bottom()),
        );
        let right_rect = Rect::from_min_max(
            egui::pos2(left_rect.right() + 1.0, body.top()),
            body.right_bottom(),
        );
        let mut left_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(left_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        left_ui.spacing_mut().item_spacing = Vec2::ZERO;
        qualification_domain_table(&mut left_ui, selected, left_rect.size());

        let mut right_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(right_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        right_ui.spacing_mut().item_spacing = Vec2::ZERO;
        if qualification_contract_card(&mut right_ui, app, selected, right_rect.size()) {
            requested_action = Some(QualificationPageAction::ReviewVectors);
        }
    }

    if qualification_gate_footer(ui, app, selected, egui::vec2(width, footer_height)) {
        requested_action = Some(QualificationPageAction::ReviewReleaseBinding);
    }
    requested_action
}

pub(super) fn qualification_gate_footer_height(ui: &Ui, width: f32) -> f32 {
    if width > MODEL_SUMMARY_BREAKPOINT {
        return 58.0;
    }
    let t = Tokens::get(ui.ctx());
    let body = ui.painter().layout(
        QUALIFICATION_GATE_COPY.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        (width - 22.0).max(1.0),
    );
    (7.0 + 18.0 + body.size().y + 8.0 + 30.0 + 8.0).max(96.0)
}

pub(super) fn qualification_domain_table(
    ui: &mut Ui,
    selected: Option<&QualificationModelSummary>,
    size: Vec2,
) {
    let t = Tokens::get(ui.ctx());
    let rows = selected
        .into_iter()
        .flat_map(|selected| selected.domains.iter())
        .map(|domain| DataRow {
            key: domain.domain.label().to_owned(),
            selected: false,
            cells: vec![
                DataCell::plain(domain.domain.label()),
                DataCell::mono(domain.vectors.to_string()),
                DataCell::plain(&domain.reference_coverage),
                DataCell::mono(&domain.tolerance),
                DataCell::mono_colored(&domain.disposition, domain.tone.color(&t)),
            ],
        })
        .collect::<Vec<_>>();
    let columns = [
        ("Domain", 0.22),
        ("Vectors", 0.13),
        ("References", 0.20),
        ("Tolerance", 0.25),
        ("Disposition", 0.20),
    ];
    let title = selected.map_or("Selected qualification", |selected| selected.model.as_str());
    table_card(
        ui,
        title,
        selected.map(|selected| (selected.gate.label(), selected.gate.color(&t))),
        size,
        |ui, table_size| {
            data_table(
                ui,
                "models.qualification.domains",
                470.0,
                &columns,
                &rows,
                table_size,
                "No executable qualification domains are retained for the selected source revision.",
            );
        },
    );
}

pub(super) fn qualification_contract_card(
    ui: &mut Ui,
    app: &RSpiceApp,
    selected: Option<&QualificationModelSummary>,
    size: Vec2,
) -> bool {
    let (viewport, _) = ui.allocate_exact_size(size.max(Vec2::splat(1.0)), Sense::hover());
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(Layout::top_down(Align::Min)),
    );
    child.spacing_mut().item_spacing = Vec2::ZERO;
    let review_blocker =
        qualification_action_block_reason(app, selected, QualificationPageAction::ReviewVectors);
    let mut clicked = false;
    ScrollArea::vertical()
        .id_salt("models.qualification.contract")
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            ui.set_min_width(viewport.width());
            let rows = [
                (
                    "Model revision",
                    selected.map_or("not selected".to_owned(), |selected| {
                        selected.source_revision.clone()
                    }),
                ),
                (
                    "Source authority",
                    selected.map_or("select a model family".to_owned(), |selected| {
                        selected
                            .source_error
                            .clone()
                            .unwrap_or_else(|| "exact project-owned revision".to_owned())
                    }),
                ),
                (
                    "Runtime parity",
                    selected.map_or("not evaluated".to_owned(), |selected| {
                        format!(
                            "desktop {}/{} · WASM {}/{}",
                            selected.desktop_passing,
                            selected.vectors,
                            selected.wasm_passing,
                            selected.vectors
                        )
                    }),
                ),
                (
                    "Evidence set",
                    selected.map_or("not retained".to_owned(), |selected| {
                        selected
                            .evidence_digest
                            .clone()
                            .unwrap_or_else(|| "not retained".to_owned())
                    }),
                ),
                (
                    "Measurement correlation",
                    selected.map_or("not configured".to_owned(), |selected| {
                        selected.correlation_evidence_digest.as_ref().map_or_else(
                            || selected.correlation_status.clone(),
                            |digest| format!("{} · {digest}", selected.correlation_status),
                        )
                    }),
                ),
                (
                    "Approved releases",
                    selected.map_or("0".to_owned(), |selected| selected.releases.to_string()),
                ),
            ];
            property_card(ui, "Qualification contract", |ui| {
                for (label, value) in &rows {
                    property_row(ui, label, value);
                }
                let review = Button::new(selected.map_or("Review qualification", |selected| {
                    if selected.open_dispositions > 0 {
                        "Review dispositions"
                    } else if selected.vectors == 0 {
                        "Configure suite"
                    } else {
                        "Inspect vectors"
                    }
                }))
                .enabled(review_blocker.is_none())
                .show(ui);
                if let Some(reason) = review_blocker.as_deref() {
                    review.on_disabled_hover_text(reason);
                } else if review.clicked() {
                    clicked = true;
                }
            });
        });
    clicked
}

pub(super) fn qualification_gate_footer(
    ui: &mut Ui,
    app: &RSpiceApp,
    selected: Option<&QualificationModelSummary>,
    size: Vec2,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(size.max(Vec2::splat(1.0)), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
    let stacked = rect.width() <= MODEL_SUMMARY_BREAKPOINT;
    let button_width = 150.0_f32.min((rect.width() - 22.0).max(1.0));
    let button_rect = if stacked {
        Rect::from_min_max(
            egui::pos2(rect.right() - button_width - 8.0, rect.bottom() - 38.0),
            egui::pos2(rect.right() - 8.0, rect.bottom() - 8.0),
        )
    } else {
        Rect::from_center_size(
            egui::pos2(rect.right() - button_width * 0.5 - 8.0, rect.center().y),
            egui::vec2(button_width, 30.0),
        )
    };
    let copy_rect = if stacked {
        Rect::from_min_max(
            egui::pos2(rect.left() + 11.0, rect.top() + 7.0),
            egui::pos2(rect.right() - 11.0, button_rect.top() - 8.0),
        )
    } else {
        Rect::from_min_max(
            egui::pos2(rect.left() + 11.0, rect.top() + 7.0),
            egui::pos2(button_rect.left() - 10.0, rect.bottom() - 7.0),
        )
    };
    let heading_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let body_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    ui.painter().text(
        copy_rect.left_top(),
        Align2::LEFT_TOP,
        "Gate ownership",
        heading_font,
        t.color.text,
    );
    let body = ui.painter().layout(
        QUALIFICATION_GATE_COPY.to_owned(),
        body_font,
        t.color.text_dim,
        copy_rect.width(),
    );
    ui.painter().galley(
        egui::pos2(copy_rect.left(), copy_rect.top() + 18.0),
        body,
        t.color.text_dim,
    );

    let blocker = qualification_action_block_reason(
        app,
        selected,
        QualificationPageAction::ReviewReleaseBinding,
    );
    let mut button_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(button_rect)
            .layout(Layout::centered_and_justified(egui::Direction::LeftToRight)),
    );
    let response = Button::new("Review release binding")
        .enabled(blocker.is_none())
        .show(&mut button_ui);
    if let Some(reason) = blocker.as_deref() {
        response.on_disabled_hover_text(reason);
        false
    } else {
        response.clicked()
    }
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
