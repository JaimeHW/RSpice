//! Planning, authoring, and dispositioning qualification vectors.
//!
//! Everything a specialist does *before* evidence exists lives here.  The
//! dialogs deliberately refuse to edit a suite once it is immutable, and a
//! vector whose binding no longer matches the draft is shown as stale rather
//! than silently re-bound — the plan must always name the exact revision the
//! evidence will be attributed to.

use super::*;

pub(super) fn qualification_plan_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
) {
    if !app.state.workbench.model_editor.qualification_plan_open {
        return;
    }
    let rows = qualification_rows(app, records);
    let qualification = records.qualification.clone().unwrap_or_default();
    let mut plan = qualification_plan_ui_state(ctx);
    normalize_qualification_selection(&qualification, &mut plan);
    let selected_suite = qualification
        .suites
        .iter()
        .find(|suite| suite.id.eq_ignore_ascii_case(&plan.selected_suite_id))
        .cloned();
    let selected_vector = selected_suite.as_ref().and_then(|suite| {
        suite
            .vectors
            .iter()
            .find(|vector| vector.id.eq_ignore_ascii_case(&plan.selected_vector_id))
            .cloned()
    });
    let authoring_open = app
        .state
        .workbench
        .model_editor
        .qualification_authoring_open;
    let child_transaction_open = authoring_open
        || plan.edit_open
        || plan.delete_target != QualificationDeleteTarget::None
        || plan.disposition_open;
    let choice = Dialog::new(
        "MODEL · GOLDEN VECTORS · PLATFORM PARITY",
        "Qualification plan",
        "Close",
    )
    .description(
        "Inspect exact persisted suite, vector, source, tolerance, and runtime evidence bindings.",
    )
    .size(DialogSize::Transaction)
    .fixed_height(700.0)
    .interaction_enabled(!child_transaction_open)
    .show(ctx, |ui| {
        let execution_active = app
            .state
            .workbench
            .model_editor
            .qualification_execution
            .is_some();
        let definition_dirty = app
            .state
            .workbench
            .model_editor
            .draft
            .as_ref()
            .is_none_or(|draft| draft.definition_is_dirty());
        let authoring_blocker = if app.state.workbench.safe_mode.project_read_only() {
            Some("The project is open read-only; qualification contracts cannot be changed.")
        } else if execution_active {
            Some("Cancel or finish the active qualification run before changing its plan.")
        } else if definition_dirty {
            Some("Save the model definition before binding a qualification vector to it.")
        } else {
            None
        };
        let (action_rect, _) = ui.allocate_exact_size(
            Vec2::new(ui.available_width().max(1.0), 38.0),
            Sense::hover(),
        );
        let mut action_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(action_rect.shrink2(Vec2::new(0.0, 4.0)))
                .layout(Layout::left_to_right(Align::Center)),
        );
        let new_suite = Button::new("New qualification suite…")
            .accent()
            .enabled(authoring_blocker.is_none())
            .min_width(176.0)
            .accessible_label("Create a new qualification suite and executable vector")
            .show(&mut action_ui);
        let new_suite = if let Some(blocker) = authoring_blocker {
            new_suite.on_hover_text(blocker)
        } else {
            new_suite
        };
        if new_suite.clicked() {
            app.state.workbench.model_editor.begin_qualification_suite();
        }

        qualification_execution_status(ui, app);

        qualification_plan_selector(ui, &qualification, &mut plan);
        qualification_plan_actions(
            ui,
            app,
            &qualification,
            selected_suite.as_ref(),
            selected_vector.as_ref(),
            &mut plan,
        );
        qualification_plan_inspector(
            ui,
            app,
            &qualification,
            selected_suite.as_ref(),
            selected_vector.as_ref(),
            &plan,
        );

        const COLUMNS: [(&str, f32); 6] = [
            ("Suite", 0.18),
            ("Vectors", 0.12),
            ("Desktop", 0.17),
            ("WASM", 0.17),
            ("Reference error", 0.18),
            ("Status", 0.18),
        ];
        let display = rows
            .iter()
            .map(|row| {
                vec![
                    DisplayCell::plain(&row.suite),
                    DisplayCell::mono(&row.vectors),
                    DisplayCell::plain(&row.desktop),
                    DisplayCell::plain(&row.wasm),
                    DisplayCell::mono(&row.reference_error),
                    DisplayCell::toned(&row.status, row.tone),
                ]
            })
            .collect::<Vec<_>>();
        render_table(
            ui,
            "model-editor-qualification-dialog",
            &COLUMNS,
            &display,
            "No persisted qualification plan is available.",
        );
    });
    qualification_store_plan_ui_state(ctx, plan.clone());
    if !child_transaction_open && matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled)
    {
        app.state.workbench.model_editor.qualification_plan_open = false;
    }
    qualification_authoring_dialog(ctx, app);
    qualification_vector_edit_dialog(ctx, app, &qualification, &mut plan);
    qualification_delete_dialog(ctx, app, &mut plan);
    qualification_disposition_dialog(ctx, app, &qualification, &mut plan);
    qualification_store_plan_ui_state(ctx, plan);
}

fn qualification_plan_ui_state(ctx: &egui::Context) -> QualificationPlanUiState {
    ctx.data_mut(|data| {
        data.get_temp::<QualificationPlanUiState>(Id::new(QUALIFICATION_PLAN_STATE_ID))
            .unwrap_or_default()
    })
}

fn qualification_store_plan_ui_state(ctx: &egui::Context, state: QualificationPlanUiState) {
    ctx.data_mut(|data| data.insert_temp(Id::new(QUALIFICATION_PLAN_STATE_ID), state));
}

pub(super) fn normalize_qualification_selection(
    qualification: &ModelQualificationState,
    plan: &mut QualificationPlanUiState,
) {
    let selected_suite = qualification
        .suites
        .iter()
        .find(|suite| suite.id.eq_ignore_ascii_case(&plan.selected_suite_id))
        .or_else(|| qualification.suites.first());
    let Some(suite) = selected_suite else {
        plan.selected_suite_id.clear();
        plan.selected_vector_id.clear();
        return;
    };
    if !suite.id.eq_ignore_ascii_case(&plan.selected_suite_id) {
        plan.selected_suite_id.clone_from(&suite.id);
        plan.selected_vector_id.clear();
    }
    let vector = suite
        .vectors
        .iter()
        .find(|vector| vector.id.eq_ignore_ascii_case(&plan.selected_vector_id))
        .or_else(|| suite.vectors.first());
    if let Some(vector) = vector {
        plan.selected_vector_id.clone_from(&vector.id);
    } else {
        plan.selected_vector_id.clear();
    }
}

fn qualification_plan_selector(
    ui: &mut Ui,
    qualification: &ModelQualificationState,
    plan: &mut QualificationPlanUiState,
) {
    dialog_subheading(ui, "Selected retained contract");
    let [suite_rect, vector_rect] = qualification_pair_rects(ui);
    let suites = qualification
        .suites
        .iter()
        .map(|suite| {
            (
                suite.id.clone(),
                format!("{} · r{}", suite.name, suite.revision.get()),
            )
        })
        .collect::<Vec<_>>();
    let previous_suite = plan.selected_suite_id.clone();
    qualification_owned_combo_field(
        ui,
        suite_rect,
        "Suite",
        "qualification-plan-suite",
        &mut plan.selected_suite_id,
        &suites,
        !suites.is_empty(),
    );
    if !previous_suite.eq_ignore_ascii_case(&plan.selected_suite_id) {
        plan.selected_vector_id.clear();
    }
    let vectors = qualification
        .suites
        .iter()
        .find(|suite| suite.id.eq_ignore_ascii_case(&plan.selected_suite_id))
        .map(|suite| {
            suite
                .vectors
                .iter()
                .map(|vector| (vector.id.clone(), vector.name.clone()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if !vectors
        .iter()
        .any(|(id, _)| id.eq_ignore_ascii_case(&plan.selected_vector_id))
    {
        plan.selected_vector_id = vectors
            .first()
            .map_or_else(String::new, |(id, _)| id.clone());
    }
    qualification_owned_combo_field(
        ui,
        vector_rect,
        "Vector",
        "qualification-plan-vector",
        &mut plan.selected_vector_id,
        &vectors,
        !vectors.is_empty(),
    );
}

fn qualification_owned_combo_field(
    ui: &mut Ui,
    rect: Rect,
    label: &str,
    salt: &str,
    value: &mut String,
    options: &[(String, String)],
    enabled: bool,
) {
    let t = Tokens::get(ui.ctx());
    ui.painter().text(
        rect.left_top(),
        Align2::LEFT_TOP,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if enabled {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    let field_rect = Rect::from_min_max(
        Pos2::new(rect.left(), rect.top() + 18.0),
        Pos2::new(rect.right(), rect.top() + 18.0 + t.metrics.ctl_h),
    );
    let selected = options
        .iter()
        .find(|(id, _)| id.eq_ignore_ascii_case(value))
        .map_or("Select", |(_, text)| text.as_str());
    let mut field = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(field_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let output = field.add_enabled_ui(enabled, |ui| {
        egui::ComboBox::from_id_salt(salt)
            .selected_text(selected)
            .width(field_rect.width())
            .show_ui(ui, |ui| {
                for (id, text) in options {
                    ui.selectable_value(value, id.clone(), text);
                }
            })
    });
    ui.ctx()
        .accesskit_node_builder(output.inner.response.id, |node| node.set_label(label));
}

fn qualification_plan_actions(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    qualification: &ModelQualificationState,
    suite: Option<&QualificationSuite>,
    vector: Option<&QualificationVector>,
    plan: &mut QualificationPlanUiState,
) {
    dialog_subheading(ui, "Contract actions");
    let read_only = app.state.workbench.safe_mode.project_read_only();
    let execution_active = app
        .state
        .workbench
        .model_editor
        .qualification_execution
        .is_some();
    let definition_dirty = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .is_none_or(|draft| draft.definition_is_dirty());
    let source_exact = vector.is_some_and(|vector| qualification_vector_is_current(app, vector));
    let suite_exact = suite.is_some_and(|suite| {
        !suite.vectors.is_empty()
            && suite
                .vectors
                .iter()
                .all(|vector| qualification_vector_is_current(app, vector))
    });
    let immutable =
        suite.is_some_and(|suite| qualification_suite_is_immutable(qualification, suite));
    let run_enabled = !read_only && !execution_active && !definition_dirty;
    let edit_enabled = run_enabled && suite.is_some() && !immutable;

    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), 74.0),
        Sense::hover(),
    );
    let mut actions = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(Vec2::new(0.0, 5.0)))
            .layout(Layout::left_to_right(Align::Min).with_main_wrap(true)),
    );
    if qualification_plan_button(&mut actions, "Run suite", 104.0, run_enabled && suite_exact) {
        if let Some(suite) = suite {
            plan.action_error =
                crate::workbench::documents::model_editor::start_qualification_suite_execution(
                    app, &suite.id,
                )
                .err();
        }
    }
    if qualification_plan_button(
        &mut actions,
        "Run vector",
        104.0,
        run_enabled && source_exact,
    ) {
        if let (Some(suite), Some(vector)) = (suite, vector) {
            plan.action_error =
                crate::workbench::documents::model_editor::start_qualification_vector_execution(
                    app, &suite.id, &vector.id,
                )
                .err();
        }
    }
    if qualification_plan_button(
        &mut actions,
        "Edit vector…",
        112.0,
        edit_enabled && vector.is_some(),
    ) {
        if let Some(vector) = vector {
            plan.edit_name.clone_from(&vector.name);
            plan.edit_open = true;
            plan.action_error = None;
        }
    }
    if qualification_plan_button(
        &mut actions,
        "Delete vector…",
        122.0,
        edit_enabled && vector.is_some(),
    ) {
        plan.delete_target = QualificationDeleteTarget::Vector;
        plan.action_error = None;
    }
    if qualification_plan_button(&mut actions, "Delete suite…", 114.0, edit_enabled) {
        plan.delete_target = QualificationDeleteTarget::Suite;
        plan.action_error = None;
    }
    let disposition_available = vector.is_some_and(|vector| {
        (!definition_dirty
            && qualification_disposition_cause(app, qualification, suite, vector).is_some())
            || qualification.vector_dispositions.iter().any(|disposition| {
                disposition
                    .vector
                    .suite_id
                    .eq_ignore_ascii_case(&plan.selected_suite_id)
                    && disposition
                        .vector
                        .vector_id
                        .eq_ignore_ascii_case(&plan.selected_vector_id)
            })
    });
    if qualification_plan_button(
        &mut actions,
        "Review disposition…",
        154.0,
        !execution_active && disposition_available,
    ) {
        if let (Some(suite), Some(vector)) = (suite, vector) {
            if let Some(cause) =
                qualification_disposition_cause(app, qualification, Some(suite), vector)
            {
                plan.disposition_cause = cause;
                plan.disposition_action = if cause == QualificationVectorDispositionCause::Stale {
                    QualificationVectorRequiredAction::Replace
                } else {
                    QualificationVectorRequiredAction::Rerun
                };
            }
            plan.disposition_id = format!("{}-{}-disposition", suite.id, vector.id);
            plan.disposition_reason.clear();
            plan.disposition_open = true;
            plan.action_error = None;
        }
    }
    if let Some(error) = plan.action_error.as_deref() {
        blocker_note(ui, error);
    } else if immutable {
        workflow_purpose_note(
            ui,
            "This suite revision is bound by evidence or a release candidate and is immutable. Run and disposition review remain available; editing requires a new suite identity.",
        );
    }
}

fn qualification_plan_button(ui: &mut Ui, label: &str, width: f32, enabled: bool) -> bool {
    Button::new(label)
        .enabled(enabled)
        .min_width(width)
        .max_width(width)
        .show(ui)
        .clicked()
}

fn qualification_vector_is_current(app: &RSpiceApp, vector: &QualificationVector) -> bool {
    app.state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .is_some_and(|draft| {
            !draft.definition_is_dirty()
                && qualification_vector_binding_matches_draft(draft, vector)
        })
}

fn qualification_vector_binding_matches_draft(
    draft: &crate::workbench::documents::model_editor::ModelEditorDraft,
    vector: &QualificationVector,
) -> bool {
    vector
        .source
        .model_id
        .eq_ignore_ascii_case(&draft.model_name)
        && vector.source.source_id == Some(draft.source_id)
        && vector.source.source_digest == draft.base_source_digest
        && vector.source.source_revision == draft.base_source_revision
}

fn qualification_suite_is_immutable(
    qualification: &ModelQualificationState,
    suite: &QualificationSuite,
) -> bool {
    qualification.evidence.iter().any(|evidence| {
        evidence.suite_id.eq_ignore_ascii_case(&suite.id)
            && evidence.suite_revision == suite.revision
    }) || qualification.candidates.iter().any(|candidate| {
        candidate.suite_id.eq_ignore_ascii_case(&suite.id)
            && candidate.suite_revision == suite.revision
    }) || qualification.promotions.iter().any(|promotion| {
        promotion.suite_id.eq_ignore_ascii_case(&suite.id)
            && promotion.suite_revision == suite.revision
    })
}

fn qualification_disposition_cause(
    app: &RSpiceApp,
    qualification: &ModelQualificationState,
    suite: Option<&QualificationSuite>,
    vector: &QualificationVector,
) -> Option<QualificationVectorDispositionCause> {
    if app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .is_some_and(|draft| !qualification_vector_binding_matches_draft(draft, vector))
    {
        return Some(QualificationVectorDispositionCause::Stale);
    }
    let suite = suite?;
    qualification
        .platform_runs
        .iter()
        .any(|run| {
            run.suite_id.eq_ignore_ascii_case(&suite.id)
                && run.suite_revision == suite.revision
                && run.source == vector.source
                && run.vector_outcomes.iter().any(|outcome| {
                    outcome.vector_id.eq_ignore_ascii_case(&vector.id) && !outcome.outcome.passed
                })
        })
        .then_some(QualificationVectorDispositionCause::Failed)
}

fn qualification_plan_inspector(
    ui: &mut Ui,
    app: &RSpiceApp,
    qualification: &ModelQualificationState,
    suite: Option<&QualificationSuite>,
    vector: Option<&QualificationVector>,
    plan: &QualificationPlanUiState,
) {
    dialog_subheading(ui, "Exact source and execution binding");
    let Some(vector) = vector else {
        table_empty(
            ui,
            ui.available_width().max(1.0),
            "Select a retained vector to inspect it.",
        );
        return;
    };
    let source_id = vector
        .source
        .source_id
        .map_or_else(|| "not project-bound".to_owned(), |id| id.to_string());
    qualification_detail_row(
        ui,
        "Suite revision",
        &suite.map_or_else(|| "—".to_owned(), |suite| suite.revision.get().to_string()),
    );
    qualification_detail_row(ui, "Source ID", &source_id);
    qualification_detail_row(
        ui,
        "Source revision",
        &vector.source.source_revision.get().to_string(),
    );
    qualification_detail_row(
        ui,
        "Source digest",
        &vector.source.source_digest.to_string(),
    );
    qualification_detail_row(
        ui,
        "Model section",
        vector.model_section.as_deref().unwrap_or("Base model"),
    );
    qualification_detail_row(ui, "Input digest", &vector.input_digest.to_string());
    qualification_detail_row(
        ui,
        "Analysis",
        qualification_analysis_label(&vector.analysis),
    );
    if let Some(output) = vector.outputs.first() {
        qualification_detail_row(
            ui,
            "Output",
            &format!(
                "{} · {} · {}",
                output.quantity,
                qualification_probe_label(&output.probe),
                qualification_sample_label(output.sample)
            ),
        );
    }
    let disposition = qualification
        .vector_dispositions
        .iter()
        .find(|disposition| {
            disposition
                .vector
                .suite_id
                .eq_ignore_ascii_case(&plan.selected_suite_id)
                && disposition
                    .vector
                    .vector_id
                    .eq_ignore_ascii_case(&plan.selected_vector_id)
                && disposition.is_open()
        });
    qualification_detail_row(
        ui,
        "Disposition",
        disposition.map_or("none", |disposition| match disposition.required_action {
            QualificationVectorRequiredAction::Rerun => "open · rerun required",
            QualificationVectorRequiredAction::Replace => "open · replacement required",
            QualificationVectorRequiredAction::Retire => "open · retirement required",
        }),
    );
    qualification_detail_row(
        ui,
        "Current source",
        if qualification_vector_is_current(app, vector) {
            "exact"
        } else {
            "stale"
        },
    );
}

fn qualification_detail_row(ui: &mut Ui, label: &str, value: &str) {
    const COLUMNS: [(&str, f32); 2] = [("Field", 0.28), ("Retained value", 0.72)];
    let cells = table_row(ui, ui.available_width().max(1.0), &COLUMNS, false);
    paint_cell(ui, cells[0], &DisplayCell::plain(label));
    paint_cell(ui, cells[1], &DisplayCell::mono(value));
}

fn qualification_analysis_label(analysis: &QualificationAnalysis) -> &'static str {
    match analysis {
        QualificationAnalysis::DcOperatingPoint => "DC operating point",
        QualificationAnalysis::DcSweep { .. } => "DC sweep",
        QualificationAnalysis::AcSweep { .. } => "AC / C-V sweep",
        QualificationAnalysis::Noise { .. } => "Noise",
        QualificationAnalysis::Transient { .. } => "Transient",
    }
}

fn qualification_probe_label(probe: &QualificationProbe) -> &'static str {
    match probe {
        QualificationProbe::NodeVoltage { .. } => "node voltage",
        QualificationProbe::BranchCurrent { .. } => "branch current",
        QualificationProbe::DcObservable { .. } => "DC observable",
        QualificationProbe::SweepValue => "sweep value",
        QualificationProbe::AcNodeVoltageMagnitude { .. } => "AC node magnitude",
        QualificationProbe::AcNodeVoltagePhaseDegrees { .. } => "AC node phase",
        QualificationProbe::AcNodeVoltageReal { .. } => "AC node real",
        QualificationProbe::AcNodeVoltageImaginary { .. } => "AC node imaginary",
        QualificationProbe::AcBranchCurrentMagnitude { .. } => "AC branch magnitude",
        QualificationProbe::AcBranchCurrentPhaseDegrees { .. } => "AC branch phase",
        QualificationProbe::AcBranchCurrentReal { .. } => "AC branch real",
        QualificationProbe::AcBranchCurrentImaginary { .. } => "AC branch imaginary",
        QualificationProbe::AcEffectiveCapacitance { .. } => "effective capacitance",
        QualificationProbe::FrequencyValue => "frequency",
        QualificationProbe::NoiseOutputDensity => "output noise density",
        QualificationProbe::NoiseInputReferredDensity => "input-referred noise density",
        QualificationProbe::NoiseOutputAmplitude => "output noise amplitude",
        QualificationProbe::NoiseInputReferredAmplitude => "input-referred noise amplitude",
        QualificationProbe::TransientNodeVoltage { .. } => "transient node voltage",
        QualificationProbe::TransientBranchCurrent { .. } => "transient branch current",
        QualificationProbe::TimeValue => "time",
    }
}

fn qualification_sample_label(sample: QualificationSample) -> &'static str {
    match sample {
        QualificationSample::OperatingPoint => "operating point",
        QualificationSample::FirstSweepPoint => "first sweep point",
        QualificationSample::LastSweepPoint => "last sweep point",
        QualificationSample::SweepPoint { .. } => "sweep point",
        QualificationSample::FirstFrequencyPoint => "first frequency point",
        QualificationSample::LastFrequencyPoint => "last frequency point",
        QualificationSample::FrequencyPoint { .. } => "frequency point",
        QualificationSample::FirstTimePoint => "first time point",
        QualificationSample::LastTimePoint => "last time point",
        QualificationSample::TimePoint { .. } => "time point",
    }
}

fn qualification_vector_edit_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    qualification: &ModelQualificationState,
    plan: &mut QualificationPlanUiState,
) {
    if !plan.edit_open {
        return;
    }
    let vector = qualification
        .qualification_vector(&plan.selected_suite_id, &plan.selected_vector_id)
        .ok()
        .cloned();
    let writable = vector.is_some()
        && !app.state.workbench.safe_mode.project_read_only()
        && app
            .state
            .workbench
            .model_editor
            .qualification_execution
            .is_none()
        && !plan.edit_name.trim().is_empty();
    let choice = Dialog::new(
        "MODEL · QUALIFICATION VECTOR · ATOMIC REVISION",
        "Edit qualification vector",
        "Replace vector",
    )
    .description("Replace the editable vector as one suite-revision transaction. Exact source, executable input, analysis, outputs, and tolerances remain retained unless authored through a new contract.")
    .size(DialogSize::Transaction)
    .fixed_height(330.0)
    .secondary("Cancel")
    .primary_enabled(writable)
    .show(ctx, |ui| {
        if let Some(vector) = vector.as_ref() {
            qualification_detail_row(ui, "Stable vector ID", &vector.id);
            qualification_detail_row(ui, "Input digest", &vector.input_digest.to_string());
        } else {
            blocker_note(ui, "The selected vector no longer exists.");
        }
        dialog_subheading(ui, "Editable presentation identity");
        let rect = qualification_single_rect(ui);
        qualification_text_field(ui, rect, "Vector name", &mut plan.edit_name, false, true);
        if let Some(error) = plan.action_error.as_deref() {
            blocker_note(ui, error);
        }
    });
    match choice {
        DialogChoice::Primary => {
            if let Some(mut replacement) = vector {
                replacement.name = plan.edit_name.clone();
                match app
                    .state
                    .workbench
                    .model_editor
                    .replace_qualification_vector(
                        &plan.selected_suite_id,
                        &plan.selected_vector_id,
                        replacement,
                    ) {
                    Ok(()) => {
                        plan.edit_open = false;
                        plan.action_error = None;
                    }
                    Err(error) => plan.action_error = Some(error),
                }
            }
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => {
            plan.edit_open = false;
            plan.action_error = None;
        }
        DialogChoice::None | DialogChoice::Ghost => {}
    }
}

fn qualification_delete_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    plan: &mut QualificationPlanUiState,
) {
    if plan.delete_target == QualificationDeleteTarget::None {
        return;
    }
    let deleting_suite = plan.delete_target == QualificationDeleteTarget::Suite;
    let title = if deleting_suite {
        "Delete qualification suite"
    } else {
        "Delete qualification vector"
    };
    let choice = Dialog::new(
        "MODEL · QUALIFICATION PLAN · ORDERED RETIREMENT",
        title,
        "Delete",
    )
    .description(if deleting_suite {
        "Delete this editable suite and its uncommitted runtime runs. Evidence and release lineage are never cascaded or rewritten."
    } else {
        "Delete this vector by advancing the editable suite revision. The final vector requires explicit suite deletion."
    })
    .size(DialogSize::Transaction)
    .fixed_height(300.0)
    .secondary("Cancel")
    .destructive()
    .show(ctx, |ui| {
        qualification_detail_row(ui, "Suite", &plan.selected_suite_id);
        qualification_detail_row(
            ui,
            "Vector",
            if deleting_suite {
                "all vectors in editable suite"
            } else {
                &plan.selected_vector_id
            },
        );
        if let Some(error) = plan.action_error.as_deref() {
            blocker_note(ui, error);
        } else {
            workflow_purpose_note(
                ui,
                "The domain validates immutability and commits the deletion atomically.",
            );
        }
    });
    match choice {
        DialogChoice::Primary => {
            let result = if deleting_suite {
                app.state
                    .workbench
                    .model_editor
                    .delete_qualification_suite(&plan.selected_suite_id)
            } else {
                app.state
                    .workbench
                    .model_editor
                    .delete_qualification_vector(&plan.selected_suite_id, &plan.selected_vector_id)
            };
            match result {
                Ok(()) => {
                    plan.delete_target = QualificationDeleteTarget::None;
                    plan.selected_vector_id.clear();
                    if deleting_suite {
                        plan.selected_suite_id.clear();
                    }
                    plan.action_error = None;
                }
                Err(error) => plan.action_error = Some(error),
            }
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => {
            plan.delete_target = QualificationDeleteTarget::None;
            plan.action_error = None;
        }
        DialogChoice::None | DialogChoice::Ghost => {}
    }
}

fn qualification_disposition_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    qualification: &ModelQualificationState,
    plan: &mut QualificationPlanUiState,
) {
    if !plan.disposition_open {
        return;
    }
    let retained = qualification
        .vector_dispositions
        .iter()
        .find(|disposition| {
            disposition
                .vector
                .suite_id
                .eq_ignore_ascii_case(&plan.selected_suite_id)
                && disposition
                    .vector
                    .vector_id
                    .eq_ignore_ascii_case(&plan.selected_vector_id)
                && disposition.is_open()
        })
        .or_else(|| {
            qualification
                .vector_dispositions
                .iter()
                .find(|disposition| {
                    disposition
                        .vector
                        .suite_id
                        .eq_ignore_ascii_case(&plan.selected_suite_id)
                        && disposition
                            .vector
                            .vector_id
                            .eq_ignore_ascii_case(&plan.selected_vector_id)
                })
        })
        .cloned();
    let parity_passed = qualification_vector_has_passing_parity(
        qualification,
        &plan.selected_suite_id,
        &plan.selected_vector_id,
    );
    let resolving = retained.as_ref().is_some_and(|disposition| {
        disposition.is_open()
            && disposition.required_action == QualificationVectorRequiredAction::Rerun
    });
    let primary = if resolving {
        "Resolve from parity rerun"
    } else if retained.is_some() {
        "Close"
    } else {
        "Record disposition"
    };
    let writable = !app.state.workbench.safe_mode.project_read_only()
        && app
            .state
            .workbench
            .model_editor
            .qualification_execution
            .is_none()
        && app
            .state
            .workbench
            .model_editor
            .draft
            .as_ref()
            .is_some_and(|draft| !draft.definition_is_dirty());
    let primary_enabled = retained.as_ref().map_or_else(
        || {
            writable
                && !plan.disposition_id.trim().is_empty()
                && !plan.disposition_reason.trim().is_empty()
        },
        |disposition| {
            !disposition.is_open()
                || (writable
                    && (disposition.required_action != QualificationVectorRequiredAction::Rerun
                        || parity_passed))
        },
    );
    let choice = Dialog::new(
        "MODEL · FAILED OR STALE VECTOR · NON-WAIVING REVIEW",
        "Qualification disposition",
        primary,
    )
    .description("A disposition preserves failure or staleness and its required corrective action. It never converts a failed observation into passing evidence.")
    .size(DialogSize::Transaction)
    .fixed_height(510.0)
    .secondary("Cancel")
    .primary_enabled(primary_enabled)
    .show(ctx, |ui| {
        qualification_detail_row(ui, "Suite", &plan.selected_suite_id);
        qualification_detail_row(ui, "Vector", &plan.selected_vector_id);
        if let Some(disposition) = retained.as_ref() {
            qualification_detail_row(ui, "Disposition ID", &disposition.id);
            qualification_detail_row(
                ui,
                "Cause",
                match disposition.cause {
                    QualificationVectorDispositionCause::Failed => "failed",
                    QualificationVectorDispositionCause::Stale => "stale",
                },
            );
            qualification_detail_row(ui, "Reason", &disposition.reason);
            qualification_detail_row(
                ui,
                "Disposition state",
                if disposition.is_open() {
                    "open · blocking"
                } else {
                    "resolved by required corrective action"
                },
            );
            qualification_detail_row(
                ui,
                "Required action",
                match disposition.required_action {
                    QualificationVectorRequiredAction::Rerun => "rerun",
                    QualificationVectorRequiredAction::Replace => "replace",
                    QualificationVectorRequiredAction::Retire => "retire",
                },
            );
            qualification_detail_row(
                ui,
                "Exact Desktop + WASM rerun",
                if parity_passed { "passing" } else { "not yet passing" },
            );
            if disposition.required_action != QualificationVectorRequiredAction::Rerun {
                workflow_purpose_note(
                    ui,
                    "Replacement or retirement resolves only through the corresponding atomic plan mutation.",
                );
            }
        } else {
            let [id, cause] = qualification_pair_rects(ui);
            qualification_text_field(
                ui,
                id,
                "Disposition ID",
                &mut plan.disposition_id,
                true,
                true,
            );
            let mut cause_text = match plan.disposition_cause {
                QualificationVectorDispositionCause::Failed => "Failed",
                QualificationVectorDispositionCause::Stale => "Stale",
            }
            .to_owned();
            qualification_text_field(ui, cause, "Verified cause", &mut cause_text, false, false);
            let action_rect = qualification_single_rect(ui);
            let actions: &[(QualificationVectorRequiredAction, &str)] =
                if plan.disposition_cause == QualificationVectorDispositionCause::Stale {
                    &[
                        (QualificationVectorRequiredAction::Replace, "Replace vector"),
                        (QualificationVectorRequiredAction::Retire, "Retire vector"),
                    ]
                } else {
                    &[
                        (QualificationVectorRequiredAction::Rerun, "Rerun exact vector"),
                        (QualificationVectorRequiredAction::Replace, "Replace vector"),
                        (QualificationVectorRequiredAction::Retire, "Retire vector"),
                    ]
                };
            qualification_combo_field(
                ui,
                action_rect,
                "Required corrective action",
                "qualification-disposition-action",
                &mut plan.disposition_action,
                actions,
                true,
            );
            qualification_multiline_field(
                ui,
                "Engineering reason and consumer impact",
                &mut plan.disposition_reason,
                "Engineering reason and consumer impact",
            );
        }
        if let Some(error) = plan.action_error.as_deref() {
            blocker_note(ui, error);
        }
    });
    match choice {
        DialogChoice::Primary => {
            if let Some(disposition) = retained {
                if !disposition.is_open() {
                    plan.disposition_open = false;
                    plan.action_error = None;
                } else if disposition.required_action == QualificationVectorRequiredAction::Rerun {
                    match app
                        .state
                        .workbench
                        .model_editor
                        .resolve_qualification_vector_rerun(&disposition.id)
                    {
                        Ok(()) => {
                            plan.disposition_open = false;
                            plan.action_error = None;
                        }
                        Err(error) => plan.action_error = Some(error),
                    }
                } else {
                    plan.disposition_open = false;
                    plan.action_error = None;
                }
            } else {
                match app
                    .state
                    .workbench
                    .model_editor
                    .record_qualification_vector_disposition(
                        plan.disposition_id.clone(),
                        &plan.selected_suite_id,
                        &plan.selected_vector_id,
                        plan.disposition_cause,
                        plan.disposition_action,
                        plan.disposition_reason.clone(),
                    ) {
                    Ok(_) => {
                        plan.disposition_open = false;
                        plan.action_error = None;
                    }
                    Err(error) => plan.action_error = Some(error),
                }
            }
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => {
            plan.disposition_open = false;
            plan.action_error = None;
        }
        DialogChoice::None | DialogChoice::Ghost => {}
    }
}

fn qualification_vector_has_passing_parity(
    qualification: &ModelQualificationState,
    suite_id: &str,
    vector_id: &str,
) -> bool {
    let Ok(suite) = qualification.qualification_suite(suite_id) else {
        return false;
    };
    let Some(vector) = suite
        .vectors
        .iter()
        .find(|vector| vector.id.eq_ignore_ascii_case(vector_id))
    else {
        return false;
    };
    [
        QualificationPlatform::Desktop,
        QualificationPlatform::WebAssembly,
    ]
    .into_iter()
    .all(|platform| {
        qualification.platform_runs.iter().any(|run| {
            run.platform == platform
                && run.suite_id.eq_ignore_ascii_case(suite_id)
                && run.suite_revision == suite.revision
                && run.source == vector.source
                && run.vector_outcomes.iter().any(|outcome| {
                    outcome.vector_id.eq_ignore_ascii_case(vector_id)
                        && outcome.input_digest == vector.input_digest
                        && outcome.outcome.platform == platform
                        && outcome.outcome.passed
                })
        })
    })
}

fn qualification_single_rect(ui: &mut Ui) -> Rect {
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), QUALIFICATION_FIELD_ROW_H),
        Sense::hover(),
    );
    rect
}

fn qualification_execution_status(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let execution = app
        .state
        .workbench
        .model_editor
        .qualification_execution
        .as_ref()
        .map(|execution| {
            (
                execution.current_suite_id().to_owned(),
                execution.completed_suites(),
                execution.total_suites(),
                execution.progress.clone(),
                execution.assembled_evidence,
            )
        });
    let notice = app
        .state
        .workbench
        .model_editor
        .qualification_execution_notice
        .clone();
    let (rect, status_response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), QUALIFICATION_STATUS_H),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    bottom_rule(ui, rect, t.color.border);

    let cancel_w = 82.0;
    let content_right = if execution.is_some() {
        rect.right() - cancel_w - 22.0
    } else {
        rect.right() - 10.0
    };
    let text_rect = Rect::from_min_max(
        Pos2::new(rect.left() + 10.0, rect.top() + 7.0),
        Pos2::new(content_right, rect.bottom() - 7.0),
    );
    let mut cancel_clicked = false;
    let accessible_status;
    if let Some((suite_id, completed_suites, total_suites, progress, evidence_count)) =
        execution.as_ref()
    {
        let platform = crate::workbench::documents::model_editor::qualification_platform_label(
            progress.platform,
        );
        let title = format!(
            "{platform} · suite {} / {} · {}",
            completed_suites + 1,
            total_suites,
            suite_id
        );
        paint_elided(
            ui,
            Rect::from_min_max(
                text_rect.min,
                Pos2::new(text_rect.right(), text_rect.top() + 16.0),
            ),
            &title,
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text,
            0.0,
        );
        let fraction = qualification_progress_fraction(
            *completed_suites,
            *total_suites,
            progress.completed_vectors,
            progress.total_vectors,
        );
        let bar = Rect::from_min_max(
            Pos2::new(text_rect.left(), text_rect.top() + 23.0),
            Pos2::new(text_rect.right(), text_rect.top() + 28.0),
        );
        ui.painter().rect_filled(bar, 0.0, t.color.bg_app);
        ui.painter().rect_filled(
            Rect::from_min_max(
                bar.min,
                Pos2::new(bar.left() + bar.width() * fraction, bar.bottom()),
            ),
            0.0,
            t.color.accent,
        );
        let vector = progress.current_vector_id.as_deref().map_or_else(
            || format!("{} vector(s) complete", progress.completed_vectors),
            |id| {
                format!(
                    "Vector {} / {} · {id} · {evidence_count} cross-platform evidence record(s)",
                    progress.completed_vectors + 1,
                    progress.total_vectors
                )
            },
        );
        paint_elided(
            ui,
            Rect::from_min_max(
                Pos2::new(text_rect.left(), text_rect.top() + 36.0),
                Pos2::new(text_rect.right(), text_rect.top() + 51.0),
            ),
            &vector,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
            0.0,
        );
        let active_notice = notice.as_deref().unwrap_or(
            "Qualification is running cooperatively; partial vectors are not published.",
        );
        paint_elided(
            ui,
            Rect::from_min_max(
                Pos2::new(text_rect.left(), text_rect.top() + 55.0),
                text_rect.max,
            ),
            active_notice,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
            0.0,
        );
        accessible_status = format!("{title}. {vector}. {active_notice}");
        let cancel_rect = Rect::from_center_size(
            Pos2::new(rect.right() - cancel_w * 0.5 - 10.0, rect.center().y),
            Vec2::new(cancel_w, t.metrics.ctl_h),
        );
        let mut cancel_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(cancel_rect)
                .layout(Layout::left_to_right(Align::Center)),
        );
        cancel_clicked = Button::new("Cancel")
            .ghost()
            .min_width(cancel_w)
            .accessible_label("Cancel active model qualification execution")
            .show(&mut cancel_ui)
            .clicked();
    } else {
        paint_elided(
            ui,
            Rect::from_min_max(
                text_rect.min,
                Pos2::new(text_rect.right(), text_rect.top() + 16.0),
            ),
            "Qualification execution",
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text,
            0.0,
        );
        let detail = notice.as_deref().unwrap_or(
            "No qualification run is active. Run qualification tests from the Model Editor toolbar after saving the plan.",
        );
        let detail_rect = Rect::from_min_max(
            Pos2::new(text_rect.left(), text_rect.top() + 25.0),
            text_rect.max,
        );
        let galley = ui.painter().layout(
            detail.to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
            detail_rect.width(),
        );
        ui.painter()
            .with_clip_rect(detail_rect)
            .galley(detail_rect.min, galley, t.color.text_dim);
        accessible_status = format!("Qualification execution. {detail}");
    }
    ui.ctx().accesskit_node_builder(status_response.id, |node| {
        node.set_label(accessible_status);
    });
    if cancel_clicked {
        crate::workbench::documents::model_editor::cancel_qualification_execution(app);
    }
}

pub(super) fn qualification_progress_fraction(
    completed_suites: usize,
    total_suites: usize,
    completed_vectors: usize,
    total_vectors: usize,
) -> f32 {
    if total_suites == 0 {
        return 0.0;
    }
    let suite_fraction = if total_vectors == 0 {
        0.0
    } else {
        completed_vectors as f32 / total_vectors as f32
    };
    ((completed_suites as f32 + suite_fraction) / total_suites as f32).clamp(0.0, 1.0)
}

fn qualification_authoring_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app
        .state
        .workbench
        .model_editor
        .qualification_authoring_open
    {
        return;
    }
    let write_allowed = !app.state.workbench.safe_mode.project_read_only();
    let choice = Dialog::new(
        "MODEL · EXECUTABLE QUALIFICATION CONTRACT",
        "New qualification suite",
        "Add suite",
    )
    .description("Author one source-bound suite and exact executable vector. The candidate changes only after the complete contract validates.")
    .size(DialogSize::Transaction)
    .fixed_height(740.0)
    .primary_on_enter(false)
    .primary_enabled(write_allowed)
    .secondary("Cancel")
    .show(ctx, |ui| qualification_authoring_form(ui, app, write_allowed));
    match choice {
        DialogChoice::Primary => {
            app.state
                .workbench
                .model_editor
                .commit_qualification_suite();
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => {
            cancel_qualification_authoring(app);
        }
        DialogChoice::None | DialogChoice::Ghost => {}
    }
}

fn qualification_authoring_form(ui: &mut Ui, app: &mut RSpiceApp, write_allowed: bool) {
    let section_options = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .map(|draft| {
            let mut values = vec![(String::new(), "Base model".to_owned())];
            values.extend(
                draft
                    .metadata
                    .sections
                    .iter()
                    .map(|section| (section.name.clone(), section.name.clone())),
            );
            values
        })
        .unwrap_or_else(|| vec![(String::new(), "Base model".to_owned())]);
    let fields = &mut app.state.workbench.model_editor.qualification_authoring;

    let error = if write_allowed {
        fields.error.as_deref()
    } else {
        Some("The project is open read-only. Cancel this authoring transaction to close it.")
    };
    qualification_authoring_error(ui, error);
    if !write_allowed {
        ui.disable();
    }
    dialog_subheading(ui, "Suite and vector identity");
    let [suite_id, suite_name] = qualification_pair_rects(ui);
    qualification_text_field(ui, suite_id, "Suite ID", &mut fields.suite_id, true, true);
    qualification_text_field(
        ui,
        suite_name,
        "Suite name",
        &mut fields.suite_name,
        false,
        true,
    );
    let [vector_id, vector_name] = qualification_pair_rects(ui);
    qualification_text_field(
        ui,
        vector_id,
        "Vector ID",
        &mut fields.vector_id,
        true,
        true,
    );
    qualification_text_field(
        ui,
        vector_name,
        "Vector name",
        &mut fields.vector_name,
        false,
        true,
    );

    qualification_section_combo_field(
        ui,
        "Executed model section",
        &mut fields.model_section,
        &section_options,
        true,
    );

    dialog_subheading(ui, "Exact executable SPICE input");
    qualification_multiline_field(
        ui,
        "SPICE input",
        &mut fields.executable_input,
        "Exact SPICE input for the qualification vector",
    );

    dialog_subheading(ui, "Analysis and observation contract");
    let [analysis, quantity] = qualification_pair_rects(ui);
    qualification_combo_field(
        ui,
        analysis,
        "Analysis",
        "qualification-analysis",
        &mut fields.analysis,
        &[
            (
                QualificationAuthoringAnalysis::DcOperatingPoint,
                "DC operating point",
            ),
            (QualificationAuthoringAnalysis::DcSweep, "DC sweep"),
            (QualificationAuthoringAnalysis::AcSweep, "AC / C-V sweep"),
            (QualificationAuthoringAnalysis::Noise, "Noise"),
            (QualificationAuthoringAnalysis::Transient, "Transient"),
        ],
        true,
    );
    qualification_normalize_authoring_domains(fields);
    qualification_text_field(ui, quantity, "Quantity", &mut fields.quantity, true, true);

    let sweep_enabled = fields.analysis == QualificationAuthoringAnalysis::DcSweep;
    let [sweep_source, sweep_start] = qualification_pair_rects(ui);
    qualification_text_field(
        ui,
        sweep_source,
        "Sweep source",
        &mut fields.sweep_source,
        true,
        sweep_enabled,
    );
    qualification_text_field(
        ui,
        sweep_start,
        "Sweep start",
        &mut fields.sweep_start,
        true,
        sweep_enabled,
    );
    let [sweep_stop, sweep_step] = qualification_pair_rects(ui);
    qualification_text_field(
        ui,
        sweep_stop,
        "Sweep stop",
        &mut fields.sweep_stop,
        true,
        sweep_enabled,
    );
    qualification_text_field(
        ui,
        sweep_step,
        "Sweep step",
        &mut fields.sweep_step,
        true,
        sweep_enabled,
    );

    let frequency_enabled = matches!(
        fields.analysis,
        QualificationAuthoringAnalysis::AcSweep | QualificationAuthoringAnalysis::Noise
    );
    let noise_enabled = fields.analysis == QualificationAuthoringAnalysis::Noise;
    let [frequencies, noise_temperature] = qualification_pair_rects(ui);
    qualification_text_field(
        ui,
        frequencies,
        "Exact frequency axis (Hz)",
        &mut fields.frequencies,
        true,
        frequency_enabled,
    );
    qualification_text_field(
        ui,
        noise_temperature,
        "Noise temperature (K)",
        &mut fields.noise_temperature_kelvin,
        true,
        noise_enabled,
    );
    let [noise_output, noise_reference] = qualification_pair_rects(ui);
    qualification_text_field(
        ui,
        noise_output,
        "Noise output node",
        &mut fields.noise_output_node,
        true,
        noise_enabled,
    );
    qualification_text_field(
        ui,
        noise_reference,
        "Noise reference node (optional)",
        &mut fields.noise_output_reference,
        true,
        noise_enabled,
    );
    let [noise_input, unused_noise] = qualification_pair_rects(ui);
    qualification_text_field(
        ui,
        noise_input,
        "Noise input source",
        &mut fields.noise_input_source,
        true,
        noise_enabled,
    );
    let mut noise_contract = "Input/output-referred noise".to_owned();
    qualification_text_field(
        ui,
        unused_noise,
        "Noise contract",
        &mut noise_contract,
        false,
        false,
    );
    let transient_enabled = fields.analysis == QualificationAuthoringAnalysis::Transient;
    let [transient_stop, transient_step] = qualification_pair_rects(ui);
    qualification_text_field(
        ui,
        transient_stop,
        "Transient stop time",
        &mut fields.transient_stop_time,
        true,
        transient_enabled,
    );
    qualification_text_field(
        ui,
        transient_step,
        "Transient maximum step",
        &mut fields.transient_max_step,
        true,
        transient_enabled,
    );

    let [probe, probe_target] = qualification_pair_rects(ui);
    let probe_options = qualification_authoring_probe_options(fields.analysis);
    qualification_combo_field(
        ui,
        probe,
        "Probe type",
        "qualification-probe",
        &mut fields.probe,
        probe_options,
        true,
    );
    qualification_text_field(
        ui,
        probe_target,
        "Probe target",
        &mut fields.probe_target,
        true,
        qualification_probe_requires_target(fields.probe),
    );

    let excitation = qualification_single_rect(ui);
    qualification_text_field(
        ui,
        excitation,
        "AC excitation magnitude",
        &mut fields.excitation_magnitude,
        true,
        fields.probe == QualificationAuthoringProbe::AcEffectiveCapacitance,
    );

    let [sample, sample_index] = qualification_pair_rects(ui);
    let sample_options = qualification_authoring_sample_options(fields.analysis);
    qualification_combo_field(
        ui,
        sample,
        "Sample selector",
        "qualification-sample",
        &mut fields.sample,
        sample_options,
        true,
    );
    qualification_text_field(
        ui,
        sample_index,
        "Sample index",
        &mut fields.sample_index,
        true,
        qualification_sample_requires_index(fields.sample),
    );

    let [expected, absolute, relative] = qualification_three_rects(ui);
    qualification_text_field(ui, expected, "Expected", &mut fields.expected, true, true);
    qualification_text_field(
        ui,
        absolute,
        "Absolute tolerance",
        &mut fields.absolute_tolerance,
        true,
        true,
    );
    qualification_text_field(
        ui,
        relative,
        "Relative tolerance",
        &mut fields.relative_tolerance,
        true,
        true,
    );
}

pub(super) fn cancel_qualification_authoring(app: &mut RSpiceApp) {
    app.state
        .workbench
        .model_editor
        .qualification_authoring_open = false;
    app.state.workbench.model_editor.qualification_authoring = Default::default();
}

pub(super) fn qualification_normalize_authoring_domains(
    fields: &mut crate::workbench::documents::model_editor::QualificationAuthoringDraft,
) {
    let probes = qualification_authoring_probe_options(fields.analysis);
    if !probes.iter().any(|(probe, _)| *probe == fields.probe) {
        fields.probe = probes[0].0;
    }
    let samples = qualification_authoring_sample_options(fields.analysis);
    if !samples.iter().any(|(sample, _)| *sample == fields.sample) {
        fields.sample = samples[0].0;
    }
}

pub(super) fn qualification_authoring_probe_options(
    analysis: QualificationAuthoringAnalysis,
) -> &'static [(QualificationAuthoringProbe, &'static str)] {
    match analysis {
        QualificationAuthoringAnalysis::DcOperatingPoint => &[
            (QualificationAuthoringProbe::NodeVoltage, "Node voltage"),
            (QualificationAuthoringProbe::BranchCurrent, "Branch current"),
            (QualificationAuthoringProbe::DcObservable, "DC observable"),
        ],
        QualificationAuthoringAnalysis::DcSweep => &[
            (QualificationAuthoringProbe::NodeVoltage, "Node voltage"),
            (QualificationAuthoringProbe::BranchCurrent, "Branch current"),
            (QualificationAuthoringProbe::DcObservable, "DC observable"),
            (QualificationAuthoringProbe::SweepValue, "Sweep value"),
        ],
        QualificationAuthoringAnalysis::AcSweep => &[
            (
                QualificationAuthoringProbe::AcNodeVoltageMagnitude,
                "Node voltage magnitude",
            ),
            (
                QualificationAuthoringProbe::AcNodeVoltagePhaseDegrees,
                "Node voltage phase (degrees)",
            ),
            (
                QualificationAuthoringProbe::AcNodeVoltageReal,
                "Node voltage real",
            ),
            (
                QualificationAuthoringProbe::AcNodeVoltageImaginary,
                "Node voltage imaginary",
            ),
            (
                QualificationAuthoringProbe::AcBranchCurrentMagnitude,
                "Branch current magnitude",
            ),
            (
                QualificationAuthoringProbe::AcBranchCurrentPhaseDegrees,
                "Branch current phase (degrees)",
            ),
            (
                QualificationAuthoringProbe::AcBranchCurrentReal,
                "Branch current real",
            ),
            (
                QualificationAuthoringProbe::AcBranchCurrentImaginary,
                "Branch current imaginary",
            ),
            (
                QualificationAuthoringProbe::AcEffectiveCapacitance,
                "Effective capacitance",
            ),
            (QualificationAuthoringProbe::FrequencyValue, "Frequency"),
        ],
        QualificationAuthoringAnalysis::Noise => &[
            (QualificationAuthoringProbe::FrequencyValue, "Frequency"),
            (
                QualificationAuthoringProbe::NoiseOutputDensity,
                "Output noise density",
            ),
            (
                QualificationAuthoringProbe::NoiseInputReferredDensity,
                "Input-referred noise density",
            ),
            (
                QualificationAuthoringProbe::NoiseOutputAmplitude,
                "Output noise amplitude",
            ),
            (
                QualificationAuthoringProbe::NoiseInputReferredAmplitude,
                "Input-referred noise amplitude",
            ),
        ],
        QualificationAuthoringAnalysis::Transient => &[
            (
                QualificationAuthoringProbe::TransientNodeVoltage,
                "Node voltage",
            ),
            (
                QualificationAuthoringProbe::TransientBranchCurrent,
                "Branch current",
            ),
            (QualificationAuthoringProbe::TimeValue, "Time"),
        ],
    }
}

pub(super) fn qualification_authoring_sample_options(
    analysis: QualificationAuthoringAnalysis,
) -> &'static [(QualificationAuthoringSample, &'static str)] {
    match analysis {
        QualificationAuthoringAnalysis::DcOperatingPoint => &[{
            (
                QualificationAuthoringSample::OperatingPoint,
                "Operating point",
            )
        }],
        QualificationAuthoringAnalysis::DcSweep => &[
            (
                QualificationAuthoringSample::FirstSweepPoint,
                "First sweep point",
            ),
            (
                QualificationAuthoringSample::LastSweepPoint,
                "Last sweep point",
            ),
            (
                QualificationAuthoringSample::SweepPoint,
                "Sweep point index",
            ),
        ],
        QualificationAuthoringAnalysis::AcSweep | QualificationAuthoringAnalysis::Noise => &[
            (
                QualificationAuthoringSample::FirstFrequencyPoint,
                "First frequency point",
            ),
            (
                QualificationAuthoringSample::LastFrequencyPoint,
                "Last frequency point",
            ),
            (
                QualificationAuthoringSample::FrequencyPoint,
                "Frequency point index",
            ),
        ],
        QualificationAuthoringAnalysis::Transient => &[
            (
                QualificationAuthoringSample::FirstTimePoint,
                "First time point",
            ),
            (
                QualificationAuthoringSample::LastTimePoint,
                "Last time point",
            ),
            (QualificationAuthoringSample::TimePoint, "Time point index"),
        ],
    }
}

pub(super) fn qualification_probe_requires_target(probe: QualificationAuthoringProbe) -> bool {
    !matches!(
        probe,
        QualificationAuthoringProbe::SweepValue
            | QualificationAuthoringProbe::FrequencyValue
            | QualificationAuthoringProbe::NoiseOutputDensity
            | QualificationAuthoringProbe::NoiseInputReferredDensity
            | QualificationAuthoringProbe::NoiseOutputAmplitude
            | QualificationAuthoringProbe::NoiseInputReferredAmplitude
            | QualificationAuthoringProbe::TimeValue
    )
}

pub(super) fn qualification_sample_requires_index(sample: QualificationAuthoringSample) -> bool {
    matches!(
        sample,
        QualificationAuthoringSample::SweepPoint
            | QualificationAuthoringSample::FrequencyPoint
            | QualificationAuthoringSample::TimePoint
    )
}

pub(super) fn qualification_pair_rects(ui: &mut Ui) -> [Rect; 2] {
    let width = ui.available_width().max(1.0);
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(width, QUALIFICATION_FIELD_ROW_H), Sense::hover());
    let gap = 12.0;
    let left_width = ((width - gap) * 0.5).max(1.0);
    [
        Rect::from_min_size(rect.min, Vec2::new(left_width, rect.height())),
        Rect::from_min_max(
            Pos2::new(rect.left() + left_width + gap, rect.top()),
            rect.max,
        ),
    ]
}

pub(super) fn qualification_three_rects(ui: &mut Ui) -> [Rect; 3] {
    let width = ui.available_width().max(1.0);
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(width, QUALIFICATION_FIELD_ROW_H), Sense::hover());
    let gap = 10.0;
    let column_width = ((width - gap * 2.0) / 3.0).max(1.0);
    [
        Rect::from_min_size(rect.min, Vec2::new(column_width, rect.height())),
        Rect::from_min_size(
            Pos2::new(rect.left() + column_width + gap, rect.top()),
            Vec2::new(column_width, rect.height()),
        ),
        Rect::from_min_max(
            Pos2::new(rect.left() + (column_width + gap) * 2.0, rect.top()),
            rect.max,
        ),
    ]
}

pub(super) fn qualification_text_field(
    ui: &mut Ui,
    rect: Rect,
    label: &str,
    value: &mut String,
    monospace: bool,
    enabled: bool,
) {
    let t = Tokens::get(ui.ctx());
    ui.painter().text(
        rect.left_top(),
        Align2::LEFT_TOP,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if enabled {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    let field_rect = Rect::from_min_max(
        Pos2::new(rect.left(), rect.top() + 18.0),
        Pos2::new(
            rect.right(),
            (rect.top() + 18.0 + t.metrics.ctl_h).min(rect.bottom()),
        ),
    );
    let mut field = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(field_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let mut edit = egui::TextEdit::singleline(value);
    if monospace {
        edit = edit.font(theme::mono(tokens::FS_0, FontWeight::Regular));
    }
    let response = field
        .add_enabled_ui(enabled, |ui| ui.add_sized(field_rect.size(), edit))
        .inner;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
    });
}

pub(super) fn qualification_combo_field<T: Copy + PartialEq>(
    ui: &mut Ui,
    rect: Rect,
    label: &str,
    salt: &str,
    value: &mut T,
    options: &[(T, &str)],
    enabled: bool,
) {
    let t = Tokens::get(ui.ctx());
    ui.painter().text(
        rect.left_top(),
        Align2::LEFT_TOP,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if enabled {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    let field_rect = Rect::from_min_max(
        Pos2::new(rect.left(), rect.top() + 18.0),
        Pos2::new(
            rect.right(),
            (rect.top() + 18.0 + t.metrics.ctl_h).min(rect.bottom()),
        ),
    );
    let mut field = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(field_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let selected = options
        .iter()
        .find_map(|(candidate, text)| (*candidate == *value).then_some(*text))
        .unwrap_or("Select");
    let output = field.add_enabled_ui(enabled, |ui| {
        egui::ComboBox::from_id_salt(salt)
            .selected_text(selected)
            .width(field_rect.width())
            .show_ui(ui, |ui| {
                for (candidate, text) in options {
                    ui.selectable_value(value, *candidate, *text);
                }
            })
    });
    ui.ctx()
        .accesskit_node_builder(output.inner.response.id, |node| {
            node.set_label(label);
        });
}

fn qualification_section_combo_field(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    options: &[(String, String)],
    enabled: bool,
) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), QUALIFICATION_FIELD_ROW_H),
        Sense::hover(),
    );
    ui.painter().text(
        rect.left_top(),
        Align2::LEFT_TOP,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if enabled {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    let field_rect = Rect::from_min_max(
        Pos2::new(rect.left(), rect.top() + 18.0),
        Pos2::new(
            rect.right(),
            (rect.top() + 18.0 + t.metrics.ctl_h).min(rect.bottom()),
        ),
    );
    let mut field = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(field_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let selected = options
        .iter()
        .find_map(|(candidate, text)| candidate.eq_ignore_ascii_case(value).then_some(text))
        .map_or("Select model section", String::as_str);
    let output = field.add_enabled_ui(enabled, |ui| {
        egui::ComboBox::from_id_salt("qualification-model-section")
            .selected_text(selected)
            .width(field_rect.width())
            .show_ui(ui, |ui| {
                for (candidate, text) in options {
                    ui.selectable_value(value, candidate.clone(), text);
                }
            })
    });
    ui.ctx()
        .accesskit_node_builder(output.inner.response.id, |node| {
            node.set_label(label);
        });
}

fn qualification_multiline_field(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    accessible_label: &str,
) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), QUALIFICATION_MULTILINE_ROW_H),
        Sense::hover(),
    );
    ui.painter().text(
        rect.left_top(),
        Align2::LEFT_TOP,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    let field_rect = Rect::from_min_max(Pos2::new(rect.left(), rect.top() + 18.0), rect.max);
    let mut field = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(field_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    let response = field.add_sized(
        field_rect.size(),
        egui::TextEdit::multiline(value)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .desired_width(field_rect.width()),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(accessible_label);
    });
}

fn qualification_authoring_error(ui: &mut Ui, error: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), QUALIFICATION_ERROR_H),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    bottom_rule(ui, rect, t.color.border);
    let (text, color) = error.map_or(
        (
            "The suite and vector are committed atomically only after every identity, source binding, analysis, probe, sample, and tolerance validates.",
            t.color.text_dim,
        ),
        |error| (error, t.color.err),
    );
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
