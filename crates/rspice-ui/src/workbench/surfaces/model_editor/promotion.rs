//! Reviewing a release candidate against the promotion gate.
//!
//! These dialogs are the specialist's view of [`ModelReleaseCandidate`]: every
//! gate row states an obligation and whether it is discharged, and the promote
//! action stays unavailable until all of them are.  The unavailable-dialog path
//! exists so a blocked promotion still explains itself instead of presenting a
//! dead button.

use super::*;

pub(super) fn promotion_review_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
) {
    if !app.state.workbench.model_editor.promotion_review_open {
        return;
    }
    if !exact_passing_evidence_available(app, records) {
        promotion_unavailable_dialog(ctx, app, records);
        return;
    }
    let candidate = current_candidate(app, records).cloned();
    let exact_candidate_count = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .map(|draft| {
            draft
                .qualification
                .candidates
                .iter()
                .filter(|candidate| {
                    candidate
                        .source
                        .model_id
                        .eq_ignore_ascii_case(&draft.model_name)
                        && candidate.source.source_id == Some(draft.source_id)
                        && candidate.source.source_digest == draft.base_source_digest
                        && candidate.source.source_revision == draft.base_source_revision
                })
                .count()
        })
        .unwrap_or_default();
    let project_writable = app.state.project_lifecycle.project_open
        && !app.state.workbench.safe_mode.project_read_only()
        && app
            .state
            .workbench
            .model_editor
            .qualification_execution
            .is_none();
    let already_promoted = candidate.as_ref().is_some_and(|candidate| {
        app.state
            .workbench
            .model_editor
            .draft
            .as_ref()
            .is_some_and(|draft| {
                draft.qualification.promotions.iter().any(|promotion| {
                    promotion
                        .candidate_identity
                        .id
                        .eq_ignore_ascii_case(&candidate.identity.id)
                })
            })
    });
    let promotable = candidate
        .as_ref()
        .is_some_and(|candidate| candidate.checklist.is_complete())
        && !already_promoted
        && project_writable;
    let inputs_complete = {
        let editor = &app.state.workbench.model_editor;
        !editor.promotion_record_id.trim().is_empty()
            && !editor.promotion_release_id.trim().is_empty()
            && !editor.promotion_release_version.trim().is_empty()
    };
    let authoring = candidate.is_none() && exact_candidate_count == 0;
    let editing = candidate
        .as_ref()
        .is_some_and(|candidate| !candidate.checklist.is_complete())
        && !already_promoted;
    let primary = if promotable {
        "Promote model"
    } else if editing {
        "Update release candidate"
    } else if authoring {
        "Create release candidate"
    } else {
        "Close"
    };
    let choice = Dialog::new(
        "MODELS · FAIL-CLOSED QUALIFICATION GATE",
        "Model promotion review",
        primary,
    )
    .description("Review the exact source-bound checklist and create one immutable release plus its audit record only when every gate passes.")
    .size(DialogSize::Transaction)
    .fixed_height(740.0)
    .primary_enabled(if promotable {
        inputs_complete
    } else if authoring || editing {
        project_writable
    } else {
        true
    })
    .secondary("Cancel")
    .show(ctx, |ui| {
        let Some(candidate) = candidate.as_ref() else {
            if exact_candidate_count > 1 {
                blocker_note(
                    ui,
                    "More than one release candidate is retained for this exact model source. Promotion fails closed until the project qualification record is repaired to one unambiguous candidate.",
                );
            } else {
                promotion_candidate_authoring_form(ui, app, project_writable);
            }
            return;
        };
        dialog_subheading(ui, &format!("{} · {}", candidate.identity.id, candidate.identity.version));
        const COLUMNS: [(&str, f32); 3] = [
            ("Promotion gate", 0.38),
            ("Observed", 0.38),
            ("State", 0.24),
        ];
        let checklist = &candidate.checklist;
        let rows = [
            ("Source evidence binding", checklist.source_evidence_bound),
            ("Qualification", checklist.qualification_passed),
            ("Desktop qualification", checklist.desktop_passed),
            ("WebAssembly parity", checklist.webassembly_passed),
            ("Documentation", checklist.documentation_complete),
            ("License", checklist.license_complete),
            ("Consumer impact", checklist.consumer_impact_complete),
            ("Compatibility", checklist.compatibility_complete),
            ("Independent approval", checklist.independent_approval_complete),
        ]
        .into_iter()
        .map(|(gate, complete)| {
            vec![
                DisplayCell::plain(gate),
                DisplayCell::plain(if complete { "complete" } else { "unresolved" }),
                DisplayCell::toned(
                    if complete { "pass" } else { "block" },
                    if complete { MetricTone::Good } else { MetricTone::Error },
                ),
            ]
        })
        .collect::<Vec<_>>();
        render_table(
            ui,
            "model-editor-promotion-dialog",
            &COLUMNS,
            &rows,
            "No promotion gates are available.",
        );
        if already_promoted {
            promotion_note(
                ui,
                None,
                "This exact candidate has already produced an immutable release and promotion audit record. Create a new model source revision for further changes.",
            );
        } else if checklist.is_complete() {
            dialog_subheading(ui, "Immutable release identity");
            let [promotion, release, version] = qualification_three_rects(ui);
            let editor = &mut app.state.workbench.model_editor;
            qualification_text_field(
                ui,
                promotion,
                "Promotion record ID",
                &mut editor.promotion_record_id,
                true,
                project_writable,
            );
            qualification_text_field(
                ui,
                release,
                "Release ID",
                &mut editor.promotion_release_id,
                true,
                project_writable,
            );
            qualification_text_field(
                ui,
                version,
                "Release version",
                &mut editor.promotion_release_version,
                true,
                project_writable,
            );
            promotion_note(
                ui,
                editor.promotion_error.as_deref(),
                if project_writable {
                    "Promotion adds the immutable release and audit record to this candidate. Save model revision then publishes the guarded, undoable project transaction."
                } else {
                    "Promotion is unavailable while the project is read-only or qualification execution is active."
                },
            );
        } else if editing {
            promotion_note(
                ui,
                None,
                "Promotion remains blocked. Correct the explicitly failed governance fields below; qualification and platform evidence remain bound to the exact retained source revision.",
            );
            promotion_candidate_authoring_form(ui, app, project_writable);
        }
    });
    match choice {
        DialogChoice::Primary if promotable => {
            if let Some(candidate) = candidate.as_ref() {
                crate::workbench::documents::model_editor::promote_open_candidate(
                    app,
                    &candidate.identity.id,
                );
            }
        }
        DialogChoice::Primary if authoring || editing => {
            app.state.workbench.model_editor.commit_release_candidate();
        }
        DialogChoice::Primary | DialogChoice::Secondary | DialogChoice::Cancelled => {
            app.state.workbench.model_editor.promotion_review_open = false;
            app.state.workbench.model_editor.promotion_error = None;
        }
        DialogChoice::None | DialogChoice::Ghost => {}
    }
}

fn promotion_unavailable_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
) {
    let rows = promotion_gate_rows(app, records);
    let mut open_plan = false;
    let mut review_disposition = false;
    let choice = Dialog::new(
        "MODELS · FAIL-CLOSED QUALIFICATION GATE",
        "Model promotion unavailable",
        "Close",
    )
    .description("Promotion is disabled because the exact current model source does not yet have complete passing qualification evidence. Opening this review never converts a failed or missing vector into a pass.")
    .size(DialogSize::Transaction)
    .fixed_height(560.0)
    .show(ctx, |ui| {
        blocker_note(
            ui,
            "Correct and rerun every unresolved exact-source vector, or record a bounded disposition that preserves the failure and its consumer impact. Promotion remains blocked until the retained qualification contract is satisfied.",
        );
        render_table(
            ui,
            "model-promotion-unavailable-gates",
            &[
                ("Promotion gate", 0.24),
                ("Required", 0.22),
                ("Observed", 0.22),
                ("State", 0.14),
                ("Owner", 0.18),
            ],
            &rows,
            "No exact-source qualification contract is retained.",
        );
        ui.add_space(8.0);
        ui.columns(2, |columns| {
            columns[0].label(
                egui::RichText::new("Safe next action")
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold)),
            );
            columns[0].label(
                egui::RichText::new(
                    "Inspect, correct, and rerun the exact qualification plan before promotion.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(Tokens::get(columns[0].ctx()).color.text_dim),
            );
            if Button::new("Open qualification plan…")
                .show(&mut columns[0])
                .clicked()
            {
                open_plan = true;
            }

            columns[1].label(
                egui::RichText::new("Disposition contract")
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold)),
            );
            columns[1].label(
                egui::RichText::new(
                    "A disposition preserves the failed observation; it cannot fabricate passing evidence.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(Tokens::get(columns[1].ctx()).color.text_dim),
            );
            if Button::new("Review disposition contract…")
                .show(&mut columns[1])
                .clicked()
            {
                review_disposition = true;
            }
        });
    });
    if open_plan || review_disposition {
        app.state.workbench.model_editor.promotion_review_open = false;
        app.state.workbench.model_editor.qualification_plan_open = true;
        app.state
            .workbench
            .model_editor
            .qualification_execution_notice = Some(if review_disposition {
            "Select the unresolved exact-source vector and review its bounded disposition contract. The retained failure remains visible and cannot satisfy a passing-evidence gate."
                .to_owned()
        } else {
            "Review and rerun the exact-source qualification plan before returning to promotion."
                .to_owned()
        });
    } else if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
        app.state.workbench.model_editor.promotion_review_open = false;
    }
}

fn promotion_gate_rows(app: &RSpiceApp, records: &PersistedModelRecords) -> Vec<Vec<DisplayCell>> {
    let Some(draft) = app.state.workbench.model_editor.draft.as_ref() else {
        return Vec::new();
    };
    let Ok(source) = ModelSourceEvidenceBinding::try_new_project_bound(
        draft.model_name.clone(),
        draft.source_id,
        draft.base_source_digest,
        draft.base_source_revision,
    ) else {
        return Vec::new();
    };
    let Some(qualification) = records.qualification.as_ref() else {
        return Vec::new();
    };
    let exact_suites = qualification
        .exact_suites_for_source(&source)
        .unwrap_or_default();
    let vector_count = exact_suites
        .iter()
        .map(|suite| suite.vectors.len())
        .sum::<usize>();
    let desktop_passed = exact_suites
        .iter()
        .filter_map(|suite| {
            exact_platform_run(
                qualification,
                suite,
                &draft.model_name,
                draft.source_id,
                draft.base_source_digest,
                draft.base_source_revision,
                QualificationPlatform::Desktop,
            )
        })
        .flat_map(|run| &run.vector_outcomes)
        .filter(|outcome| outcome.outcome.passed)
        .count();
    let wasm_passed = exact_suites
        .iter()
        .filter_map(|suite| {
            exact_platform_run(
                qualification,
                suite,
                &draft.model_name,
                draft.source_id,
                draft.base_source_digest,
                draft.base_source_revision,
                QualificationPlatform::WebAssembly,
            )
        })
        .flat_map(|run| &run.vector_outcomes)
        .filter(|outcome| outcome.outcome.passed)
        .count();
    let evidence_count = exact_suites
        .iter()
        .flat_map(|suite| {
            qualification
                .evidence
                .iter()
                .filter(|evidence| evidence.validate_bound(suite, &source).is_ok())
        })
        .filter(|evidence| evidence.passed)
        .count();
    let row = |gate: &str, required: String, observed: String, passed: bool, owner: &str| {
        vec![
            DisplayCell::plain(gate),
            DisplayCell::plain(required),
            DisplayCell::mono(observed),
            DisplayCell::toned(
                if passed { "pass" } else { "block" },
                if passed {
                    MetricTone::Good
                } else {
                    MetricTone::Error
                },
            ),
            DisplayCell::plain(owner),
        ]
    };
    vec![
        row(
            "Exact qualification plan",
            "at least one suite".to_owned(),
            format!("{} suite(s)", exact_suites.len()),
            !exact_suites.is_empty(),
            "Model qualification",
        ),
        row(
            "Desktop qualification",
            format!("{vector_count} / {vector_count} vectors"),
            format!("{desktop_passed} / {vector_count}"),
            vector_count > 0 && desktop_passed == vector_count,
            "Desktop runtime",
        ),
        row(
            "WebAssembly parity",
            format!("{vector_count} / {vector_count} vectors"),
            format!("{wasm_passed} / {vector_count}"),
            vector_count > 0 && wasm_passed == vector_count,
            "WebAssembly runtime",
        ),
        row(
            "Cross-platform evidence",
            "one complete retained record".to_owned(),
            format!("{evidence_count} passing"),
            evidence_count > 0,
            "Model owner",
        ),
        row(
            "Independent approval",
            "model owner + qualification approver".to_owned(),
            "not eligible".to_owned(),
            false,
            "Governance",
        ),
    ]
}

fn promotion_candidate_authoring_form(ui: &mut Ui, app: &mut RSpiceApp, writable: bool) {
    ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
    let fields = &mut app.state.workbench.model_editor.promotion_candidate;

    dialog_subheading(ui, "Candidate and exact qualification evidence");
    let [candidate_id, candidate_version] = qualification_pair_rects(ui);
    qualification_text_field(
        ui,
        candidate_id,
        "Candidate ID",
        &mut fields.candidate_id,
        true,
        writable,
    );
    qualification_text_field(
        ui,
        candidate_version,
        "Candidate version",
        &mut fields.candidate_version,
        true,
        writable,
    );
    let [suite, evidence] = qualification_pair_rects(ui);
    qualification_text_field(
        ui,
        suite,
        "Qualification suite ID",
        &mut fields.suite_id,
        true,
        writable,
    );
    qualification_text_field(
        ui,
        evidence,
        "Qualification evidence ID",
        &mut fields.evidence_id,
        true,
        writable,
    );

    dialog_subheading(ui, "Required content-addressed documentation");
    let document_fields = [
        (
            "Model description",
            &mut fields.model_description_id,
            &mut fields.model_description_digest,
        ),
        (
            "Parameter reference",
            &mut fields.parameter_reference_id,
            &mut fields.parameter_reference_digest,
        ),
        (
            "Qualification report",
            &mut fields.qualification_report_id,
            &mut fields.qualification_report_digest,
        ),
    ];
    for (label, id, digest) in document_fields {
        let [id_rect, digest_rect] = qualification_pair_rects(ui);
        qualification_text_field(ui, id_rect, &format!("{label} ID"), id, true, writable);
        qualification_text_field(
            ui,
            digest_rect,
            &format!("{label} SHA-256"),
            digest,
            true,
            writable,
        );
    }

    dialog_subheading(ui, "License declaration");
    let [license_id, expression] = qualification_pair_rects(ui);
    qualification_text_field(
        ui,
        license_id,
        "License ID",
        &mut fields.license_id,
        true,
        writable,
    );
    qualification_text_field(
        ui,
        expression,
        "SPDX or LicenseRef expression",
        &mut fields.license_expression,
        true,
        writable,
    );
    let [scope, notice_id, notice_digest] = qualification_three_rects(ui);
    qualification_combo_field(
        ui,
        scope,
        "License scope",
        "model-promotion-license-scope",
        &mut fields.license_scope,
        &[
            (LicenseScope::FoundryProject, "Foundry project"),
            (LicenseScope::OrganizationInternal, "Organization internal"),
            (LicenseScope::Redistributable, "Redistributable"),
        ],
        writable,
    );
    qualification_text_field(
        ui,
        notice_id,
        "License notice ID",
        &mut fields.license_notice_id,
        true,
        writable,
    );
    qualification_text_field(
        ui,
        notice_digest,
        "Notice SHA-256",
        &mut fields.license_notice_digest,
        true,
        writable,
    );
    fixed_checkbox_row(ui, writable, |ui| {
        ui.checkbox(
            &mut fields.commercial_use_allowed,
            "Commercial use permitted",
        );
        ui.checkbox(
            &mut fields.redistribution_allowed,
            "Redistribution permitted",
        );
        ui.checkbox(&mut fields.license_reviewed, "License reviewed");
    });

    dialog_subheading(ui, "Consumer impact and migration");
    let [change, summary] = qualification_pair_rects(ui);
    qualification_combo_field(
        ui,
        change,
        "Change classification",
        "model-promotion-consumer-change",
        &mut fields.consumer_change,
        &[
            (ConsumerChange::NoImpact, "No impact"),
            (ConsumerChange::Compatible, "Compatible"),
            (ConsumerChange::Breaking, "Breaking"),
        ],
        writable,
    );
    qualification_text_field(
        ui,
        summary,
        "Impact summary",
        &mut fields.consumer_summary,
        false,
        writable,
    );
    let [consumers, migration_id, migration_digest] = qualification_three_rects(ui);
    qualification_text_field(
        ui,
        consumers,
        "Affected consumer IDs (comma separated)",
        &mut fields.affected_consumer_ids,
        true,
        writable,
    );
    qualification_text_field(
        ui,
        migration_id,
        "Migration plan ID",
        &mut fields.migration_plan_id,
        true,
        writable,
    );
    qualification_text_field(
        ui,
        migration_digest,
        "Migration plan SHA-256",
        &mut fields.migration_plan_digest,
        true,
        writable,
    );
    fixed_checkbox_row(ui, writable, |ui| {
        ui.checkbox(&mut fields.consumer_reviewed, "Consumer impact reviewed");
    });

    dialog_subheading(ui, "Runtime and project compatibility");
    compatibility_authoring_row(
        ui,
        "Desktop",
        "model-promotion-desktop-compatibility",
        &mut fields.desktop_compatibility,
        &mut fields.desktop_evidence_id,
        &mut fields.desktop_evidence_digest,
        writable,
    );
    compatibility_authoring_row(
        ui,
        "WebAssembly",
        "model-promotion-wasm-compatibility",
        &mut fields.webassembly_compatibility,
        &mut fields.webassembly_evidence_id,
        &mut fields.webassembly_evidence_digest,
        writable,
    );
    let [projects, review] = qualification_pair_rects(ui);
    qualification_combo_field(
        ui,
        projects,
        "Existing projects",
        "model-promotion-existing-projects",
        &mut fields.existing_projects_compatibility,
        &compatibility_options(),
        writable,
    );
    let mut review_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(review)
            .layout(Layout::left_to_right(Align::Center)),
    );
    review_ui.add_enabled_ui(writable, |ui| {
        ui.checkbox(&mut fields.compatibility_reviewed, "Compatibility reviewed");
    });

    dialog_subheading(ui, "Independent approvals");
    approval_authoring_row(
        ui,
        "Model owner",
        "model-promotion-owner-decision",
        &mut fields.model_owner_id,
        &mut fields.model_owner_decision,
        &mut fields.model_owner_decision_revision,
        writable,
    );
    approval_authoring_row(
        ui,
        "Qualification approver",
        "model-promotion-qualification-decision",
        &mut fields.qualification_approver_id,
        &mut fields.qualification_approver_decision,
        &mut fields.qualification_approver_decision_revision,
        writable,
    );
    promotion_note(
        ui,
        fields.error.as_deref(),
        if writable {
            "Creation is atomic and source-bound. No partial document, declaration, compatibility, or approval state is retained when any field fails validation."
        } else {
            "Release-candidate authoring is unavailable while the project is read-only or qualification execution is active."
        },
    );
}

fn compatibility_options() -> [(CompatibilityDisposition, &'static str); 3] {
    [
        (CompatibilityDisposition::Compatible, "Compatible"),
        (
            CompatibilityDisposition::MigrationRequired,
            "Migration required",
        ),
        (CompatibilityDisposition::Incompatible, "Incompatible"),
    ]
}

fn compatibility_authoring_row(
    ui: &mut Ui,
    platform: &str,
    salt: &str,
    disposition: &mut CompatibilityDisposition,
    evidence_id: &mut String,
    evidence_digest: &mut String,
    writable: bool,
) {
    let [status, id, digest] = qualification_three_rects(ui);
    qualification_combo_field(
        ui,
        status,
        &format!("{platform} disposition"),
        salt,
        disposition,
        &compatibility_options(),
        writable,
    );
    qualification_text_field(
        ui,
        id,
        &format!("{platform} evidence ID"),
        evidence_id,
        true,
        writable,
    );
    qualification_text_field(
        ui,
        digest,
        &format!("{platform} evidence SHA-256"),
        evidence_digest,
        true,
        writable,
    );
}

fn approval_authoring_row(
    ui: &mut Ui,
    role: &str,
    salt: &str,
    approver: &mut String,
    decision: &mut ApprovalDecision,
    revision: &mut String,
    writable: bool,
) {
    let [approver_rect, decision_rect, revision_rect] = qualification_three_rects(ui);
    qualification_text_field(
        ui,
        approver_rect,
        &format!("{role} identity"),
        approver,
        true,
        writable,
    );
    qualification_combo_field(
        ui,
        decision_rect,
        &format!("{role} decision"),
        salt,
        decision,
        &[
            (ApprovalDecision::Approved, "Approved"),
            (ApprovalDecision::Rejected, "Rejected"),
        ],
        writable,
    );
    qualification_text_field(
        ui,
        revision_rect,
        &format!("{role} decision revision"),
        revision,
        true,
        writable,
    );
}

fn fixed_checkbox_row(ui: &mut Ui, enabled: bool, body: impl FnOnce(&mut Ui)) {
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), 34.0),
        Sense::hover(),
    );
    let mut row = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    row.spacing_mut().item_spacing.x = 18.0;
    row.add_enabled_ui(enabled, body);
}

pub(super) fn promotion_note(ui: &mut Ui, error: Option<&str>, detail: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), 52.0),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    bottom_rule(ui, rect, t.color.border);
    let (text, color) = error.map_or((detail, t.color.text_dim), |error| (error, t.color.err));
    if error.is_some() {
        ui.painter().rect_filled(
            Rect::from_min_max(rect.min, Pos2::new(rect.left() + 3.0, rect.bottom())),
            0.0,
            t.color.err,
        );
    }
    let inner = rect.shrink2(Vec2::new(10.0, 8.0));
    let galley = ui.painter().layout(
        text.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        color,
        inner.width(),
    );
    ui.painter()
        .with_clip_rect(inner)
        .galley(inner.min, galley, color);
}
