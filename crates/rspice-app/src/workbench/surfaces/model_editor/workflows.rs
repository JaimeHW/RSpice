//! The review workflows: save, validate, and run a qualification pass.
//!
//! Each workflow presents the exact rows a reviewer needs to decide, and none
//! of them commits anything until that decision is made. A validation row
//! shows what was checked and what was observed side by side, so a passing
//! review is a statement about specific evidence rather than an absence of
//! complaints.

use super::*;

pub(super) fn show_inspection_dialogs(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
) {
    parameter_schema_dialog(ctx, app, records);
    new_section_dialog(ctx, app, records);
    correlation_matrix_dialog(ctx, app, records);
    temperature_preview_dialog(ctx, app, records);
    qualification_plan_dialog(ctx, app, records);
    promotion_review_dialog(ctx, app, records);
    release_comparison_dialog(ctx, app, records);
}

pub(super) fn show_model_editor_workflow(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
) {
    let Some(request) = active_model_editor_workflow(app) else {
        return;
    };
    match request.workflow {
        ModelEditorWorkflow::SaveRevision => save_revision_workflow(ctx, app, &request),
        ModelEditorWorkflow::ValidateCandidate => {
            validation_review_workflow(ctx, app, records, &request);
        }
        ModelEditorWorkflow::RunQualification => {
            qualification_run_workflow(ctx, app, records, &request);
        }
    }
}

pub(super) fn save_revision_workflow(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    request: &super::super::super::commands::ModelEditorWorkflowRequest,
) {
    let writable = app.state.project_lifecycle.project_open
        && !app.state.workbench.safe_mode.project_read_only()
        && app
            .state
            .workbench
            .model_editor
            .qualification_execution
            .is_none();
    let dirty = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .is_some_and(|draft| draft.is_dirty());
    let choice = Dialog::new(
        "MODELS · VERSIONED DEFINITION",
        "Save model revision",
        "Save reviewed revision",
    )
    .description("Review the exact candidate delta and source binding before publishing one guarded project/model revision.")
    .size(DialogSize::Transaction)
    .fixed_height(430.0)
    .primary_enabled(writable && dirty)
    .secondary("Cancel")
    .show(ctx, |ui| {
        workflow_purpose_note(ui, "Saving creates one versioned engineering revision. Existing immutable releases and qualification evidence are never rewritten.");
        dialog_subheading(ui, "Candidate revision contract");
        render_table(
            ui,
            "model-save-review-contract",
            &[("Field", 0.34), ("Reviewed value", 0.66)],
            &save_review_rows(app),
            "No project-owned model candidate is open.",
        );
        if let Some(error) = request.error.as_deref() {
            blocker_note(ui, error);
        } else if !writable {
            blocker_note(
                ui,
                if app.state.workbench.safe_mode.project_read_only() {
                    "The project is open read-only. Inspection remains available, but this revision cannot be saved."
                } else {
                    "Finish or cancel the active qualification run before saving this revision."
                },
            );
        } else if !dirty {
            blocker_note(ui, "The candidate has no semantic changes to save.");
        }
    });
    match choice {
        DialogChoice::Primary if writable && dirty => {
            match crate::workbench::documents::model_editor::save_open_candidate(app) {
                Ok(revision) => {
                    app.state.push_user_message(
                        crate::diagnostics::ConsoleMessage::info(format!(
                            "Model revision r{} was saved with its exact source and qualification state.",
                            revision.get()
                        )),
                    );
                    close_model_editor_workflow();
                }
                Err(error) => set_model_editor_workflow_error(error),
            }
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => close_model_editor_workflow(),
        DialogChoice::None | DialogChoice::Primary | DialogChoice::Ghost => {}
    }
}

pub(super) fn save_review_rows(app: &RSpiceApp) -> Vec<Vec<DisplayCell>> {
    let Some(draft) = app.state.workbench.model_editor.draft.as_ref() else {
        return Vec::new();
    };
    let delta = draft.delta().ok();
    let definition_delta = delta.as_ref().map_or_else(
        || "definition cannot be projected".to_owned(),
        |delta| {
            let parameter_changes = delta.added_parameters.len()
                + delta.removed_parameters.len()
                + delta.changed_parameters.len();
            format!(
                "{parameter_changes} parameter change(s) · identity {} · description {}",
                if delta.identity_changed {
                    "changed"
                } else {
                    "stable"
                },
                if delta.description_changed {
                    "changed"
                } else {
                    "stable"
                }
            )
        },
    );
    vec![
        vec![
            DisplayCell::plain("Model identity"),
            DisplayCell::mono(format!("{}/{}", draft.library_name, draft.model_name)),
        ],
        vec![
            DisplayCell::plain("Source binding"),
            DisplayCell::mono(format!(
                "r{} · {}",
                draft.base_source_revision.get(),
                short_identity(&draft.base_source_digest.to_string())
            )),
        ],
        vec![
            DisplayCell::plain("Definition delta"),
            DisplayCell::plain(definition_delta),
        ],
        vec![
            DisplayCell::plain("Qualification delta"),
            DisplayCell::plain(if draft.qualification_is_dirty() {
                "changed · retained with this revision"
            } else {
                "unchanged"
            }),
        ],
        vec![
            DisplayCell::plain("Publication"),
            DisplayCell::plain("guarded project transaction · undoable revision"),
        ],
    ]
}

pub(super) fn validation_review_workflow(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
    request: &super::super::super::commands::ModelEditorWorkflowRequest,
) {
    let prepared = request.prepared;
    let choice = Dialog::new(
        "MODELS · SCHEMA · PHYSICS · PLATFORM",
        "Validate model candidate",
        if prepared { "Close report" } else { "Run validation" },
    )
    .description("Review source identity, typed schema, physical bounds, qualification structure, and retained platform evidence without changing the model definition.")
    .size(DialogSize::Transaction)
    .fixed_height(470.0)
    ;
    let choice = if prepared {
        choice.show(ctx, |ui| {
            validation_review_body(ui, app, records, request, prepared);
        })
    } else {
        choice.secondary("Cancel").show(ctx, |ui| {
            validation_review_body(ui, app, records, request, prepared);
        })
    };
    match choice {
        DialogChoice::Primary if !prepared => {
            crate::workbench::documents::model_editor::validate_open_candidate(app);
            prepare_model_editor_workflow();
        }
        DialogChoice::Primary | DialogChoice::Secondary | DialogChoice::Cancelled => {
            close_model_editor_workflow();
        }
        DialogChoice::None | DialogChoice::Ghost => {}
    }
}

pub(super) fn validation_review_body(
    ui: &mut Ui,
    app: &RSpiceApp,
    records: &PersistedModelRecords,
    request: &super::super::super::commands::ModelEditorWorkflowRequest,
    prepared: bool,
) {
    workflow_purpose_note(
        ui,
        "Validation is bound to the exact candidate source and current project revision. A failed check cannot be accepted as release evidence.",
    );
    dialog_subheading(ui, "Validation report");
    render_table(
        ui,
        "model-validation-review-contract",
        &[("Check", 0.34), ("Observed", 0.44), ("State", 0.22)],
        &validation_review_rows(app, records, prepared),
        if prepared {
            "No model validation report is available."
        } else {
            "Run validation to produce an exact candidate report."
        },
    );
    if let Some(error) = request.error.as_deref() {
        blocker_note(ui, error);
    } else if !app.state.workbench.model_editor.diagnostics.is_empty() {
        blocker_note(
            ui,
            &app.state
                .workbench
                .model_editor
                .diagnostics
                .iter()
                .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
                .collect::<Vec<_>>()
                .join(" · "),
        );
    }
}

pub(super) fn validation_review_rows(
    app: &RSpiceApp,
    records: &PersistedModelRecords,
    prepared: bool,
) -> Vec<Vec<DisplayCell>> {
    if !prepared {
        let Some(draft) = app.state.workbench.model_editor.draft.as_ref() else {
            return Vec::new();
        };
        return vec![
            vec![
                DisplayCell::plain("Candidate source"),
                DisplayCell::mono(format!(
                    "{} · r{} · {}",
                    draft.model_name,
                    draft.base_source_revision.get(),
                    short_identity(&draft.base_source_digest.to_string())
                )),
                DisplayCell::toned("ready", MetricTone::Neutral),
            ],
            vec![
                DisplayCell::plain("Validation scope"),
                DisplayCell::plain(
                    "source identity · typed schema · physical bounds · qualification structure · platform evidence",
                ),
                DisplayCell::toned("pending", MetricTone::Warning),
            ],
        ];
    }
    let valid = app.state.workbench.model_editor.validation.is_some();
    let definition_valid =
        records.metadata_error.is_none() && app.state.workbench.model_editor.diagnostics.is_empty();
    let qualification_valid = records.qualification_error.is_none();
    let qualification = qualification_totals(app, records);
    let parity_complete = qualification.total > 0
        && qualification.desktop_passed == qualification.total
        && qualification.wasm_passed == qualification.total;
    vec![
        validation_row(
            "Source identity and digest",
            if valid {
                "exact source and project revisions bound"
            } else {
                "binding rejected"
            },
            valid,
        ),
        validation_row(
            "Typed schema and physical bounds",
            if definition_valid {
                "all declared values valid"
            } else {
                "one or more definition diagnostics"
            },
            definition_valid,
        ),
        validation_row(
            "Qualification contract",
            if qualification_valid {
                "persisted structure valid"
            } else {
                "qualification structure invalid"
            },
            qualification_valid,
        ),
        validation_row(
            "Desktop / WebAssembly parity",
            &format!(
                "{} / {} desktop · {} / {} WebAssembly",
                qualification.desktop_passed,
                qualification.total,
                qualification.wasm_passed,
                qualification.total
            ),
            parity_complete,
        ),
    ]
}

pub(super) fn validation_row(check: &str, observed: &str, passed: bool) -> Vec<DisplayCell> {
    vec![
        DisplayCell::plain(check),
        DisplayCell::plain(observed),
        DisplayCell::toned(
            if passed { "pass" } else { "review" },
            if passed {
                MetricTone::Good
            } else {
                MetricTone::Warning
            },
        ),
    ]
}

pub(super) fn qualification_run_workflow(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
    request: &super::super::super::commands::ModelEditorWorkflowRequest,
) {
    let runnable = Command::ModelRunQualificationTests.is_enabled(app);
    let choice = Dialog::new(
        "MODELS · TEST VECTORS · PLATFORM PARITY",
        "Run model qualification tests",
        "Queue qualification tests",
    )
    .description("Confirm the exact clean source, executable suites, current runtime target, and evidence-retention contract before dispatch.")
    .size(DialogSize::Transaction)
    .fixed_height(430.0)
    .primary_enabled(runnable)
    .secondary("Cancel")
    .show(ctx, |ui| {
        workflow_purpose_note(ui, "Qualification publishes only complete suite runs. Cross-platform evidence is assembled only from exact matching Desktop and WebAssembly source bindings.");
        dialog_subheading(ui, "Execution contract");
        render_table(
            ui,
            "model-qualification-run-contract",
            &[("Field", 0.34), ("Reviewed value", 0.66)],
            &qualification_run_rows(app, records),
            "No executable qualification plan is available.",
        );
        if let Some(error) = request.error.as_deref() {
            blocker_note(ui, error);
        } else if !runnable {
            let reason = match Command::ModelRunQualificationTests.availability(app) {
                CommandAvailability::Disabled(reason) => reason,
                CommandAvailability::Available | CommandAvailability::Hidden => {
                    "Qualification is unavailable in this context."
                }
            };
            blocker_note(ui, reason);
        }
    });
    match choice {
        DialogChoice::Primary if runnable => {
            match crate::workbench::documents::model_editor::start_qualification_execution(app) {
                Ok(()) => close_model_editor_workflow(),
                Err(error) => set_model_editor_workflow_error(error),
            }
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => close_model_editor_workflow(),
        DialogChoice::None | DialogChoice::Primary | DialogChoice::Ghost => {}
    }
}

pub(super) fn qualification_run_rows(
    app: &RSpiceApp,
    _records: &PersistedModelRecords,
) -> Vec<Vec<DisplayCell>> {
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
    let exact_suites = draft
        .qualification
        .exact_suites_for_source(&source)
        .unwrap_or_default();
    let vectors = exact_suites
        .iter()
        .map(|suite| suite.vectors.len())
        .sum::<usize>();
    let exact_evidence = exact_suites
        .iter()
        .flat_map(|suite| {
            draft
                .qualification
                .evidence
                .iter()
                .filter(|evidence| evidence.validate_bound(suite, &source).is_ok())
        })
        .count();
    let target = if cfg!(target_arch = "wasm32") {
        "WebAssembly runtime"
    } else {
        "Local desktop runtime"
    };
    vec![
        vec![
            DisplayCell::plain("Model scope"),
            DisplayCell::mono(format!(
                "{} · r{}",
                draft.model_name,
                draft.base_source_revision.get()
            )),
        ],
        vec![
            DisplayCell::plain("Executable expansion"),
            DisplayCell::plain(format!(
                "{} suite(s) · {vectors} vector(s)",
                exact_suites.len()
            )),
        ],
        vec![
            DisplayCell::plain("Runtime target"),
            DisplayCell::plain(target),
        ],
        vec![
            DisplayCell::plain("Validation"),
            DisplayCell::toned(
                if app.state.workbench.model_editor.validation.is_some() {
                    "current source-bound report"
                } else {
                    "validation required"
                },
                if app.state.workbench.model_editor.validation.is_some() {
                    MetricTone::Good
                } else {
                    MetricTone::Warning
                },
            ),
        ],
        vec![
            DisplayCell::plain("Retained evidence"),
            DisplayCell::plain(format!(
                "{} existing cross-platform record(s) · immutable completed suites",
                exact_evidence
            )),
        ],
        vec![
            DisplayCell::plain("Promotion policy"),
            DisplayCell::plain("independent review required after completion"),
        ],
    ]
}

pub(super) fn workflow_purpose_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let galley = ui.painter().layout(
        text.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        (ui.available_width() - 28.0).max(1.0),
    );
    let height = (galley.size().y + 18.0).max(42.0);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), height),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().rect_filled(
        Rect::from_min_max(rect.min, Pos2::new(rect.left() + 2.0, rect.bottom())),
        0.0,
        t.color.accent,
    );
    ui.painter().galley(
        Pos2::new(rect.left() + 12.0, rect.top() + 9.0),
        galley,
        t.color.text_dim,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(text);
    });
}
