//! The correlation authoring dialogs: datasets, metrics, review, disposition.
//!
//! Each dialog edits a draft and is blocked up front when its source is
//! unavailable, so the form never opens onto data it cannot bind. A retained
//! simulation is picked by exact run identity rather than by label, because
//! two runs of the same plan are different evidence.

use super::*;

pub(super) fn open_correlation_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    dialog: ModelCorrelationDialog,
) {
    match model_correlation_controller::current_source_binding(app) {
        Ok(source) => {
            app.state.workbench.model_correlation.dialog_source = Some(source);
            ctx.data_mut(|data| {
                data.remove::<String>(correlation_dialog_error_id());
            });
            app.state.workbench.model_correlation.dialog = Some(dialog);
        }
        Err(error) => {
            app.state.workbench.model_correlation.notice = Some(error);
        }
    }
}

pub(super) fn correlation_dialog_source_blocker(app: &RSpiceApp) -> Option<String> {
    let Some(opening) = app.state.workbench.model_correlation.dialog_source.as_ref() else {
        return Some(
            "The dialog has no opening source identity; close it and reopen from the current model"
                .to_owned(),
        );
    };
    match model_correlation_controller::current_source_binding(app) {
        Ok(current) if current == *opening => None,
        Ok(_) => Some(
            "The selected model source changed after this dialog opened; cancel and reopen it"
                .to_owned(),
        ),
        Err(error) => Some(format!(
            "The dialog's opening model source is no longer available: {error}"
        )),
    }
}

pub(super) fn clear_correlation_dialog_runtime(ctx: &egui::Context) {
    ctx.data_mut(|data| {
        data.remove::<String>(correlation_dialog_error_id());
    });
}

pub(super) fn dialog_field_width(ui: &Ui) -> f32 {
    (ui.ctx().content_rect().width() - 220.0).clamp(140.0, 430.0)
}

pub(super) fn show_correlation_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    let Some(mut dialog) = app.state.workbench.model_correlation.dialog.take() else {
        return;
    };
    let (title, primary, description, contains_multiline) = match &dialog {
        ModelCorrelationDialog::AddDataset(_) => (
            "Add correlation dataset",
            "Retain dataset",
            "Import immutable, source-identified correlation evidence.",
            true,
        ),
        ModelCorrelationDialog::AddMetric(_) => (
            "Add correlation metric",
            "Retain metric",
            "Define a unit-aware comparison contract for the selected suite.",
            false,
        ),
        ModelCorrelationDialog::Review(_) => (
            "Validate correlation suite",
            "Record review",
            "Retain an independent review of the exact selected suite revision.",
            false,
        ),
        ModelCorrelationDialog::Disposition(_) => (
            "Review outlier disposition",
            "Record disposition",
            "Retain an append-only, independently reviewed residual disposition.",
            false,
        ),
    };
    let projection = CorrelationProjection::from_app(ctx, app);
    let action_blocker = match &dialog {
        ModelCorrelationDialog::AddDataset(_) => {
            correlation_action_blocker(app, &projection, false, false)
        }
        ModelCorrelationDialog::AddMetric(_) | ModelCorrelationDialog::Disposition(_) => {
            correlation_action_blocker(app, &projection, true, false)
        }
        ModelCorrelationDialog::Review(_) => {
            correlation_action_blocker(app, &projection, true, true)
        }
    };
    let source_blocker = correlation_dialog_source_blocker(app);
    let submit_error = ctx.data(|data| data.get_temp::<String>(correlation_dialog_error_id()));
    let draft_error = match &dialog {
        ModelCorrelationDialog::AddDataset(draft) => draft.validation_error.as_deref(),
        ModelCorrelationDialog::AddMetric(draft) => draft.validation_error.as_deref(),
        ModelCorrelationDialog::Review(draft) => draft.validation_error.as_deref(),
        ModelCorrelationDialog::Disposition(draft) => draft.validation_error.as_deref(),
    };
    let visible_error = submit_error
        .or_else(|| draft_error.map(str::to_owned))
        .or_else(|| source_blocker.clone())
        .or_else(|| action_blocker.clone());

    let mut shell = Dialog::new("MODELS · CORRELATION", title, primary)
        .description(description)
        .size(DialogSize::Transaction)
        .ghost("Cancel")
        .primary_enabled(source_blocker.is_none() && action_blocker.is_none())
        .primary_on_enter(!contains_multiline);
    if let Some(error) = visible_error.as_deref() {
        shell = shell.transaction_state(
            DialogTransactionTone::Error,
            "Correlation record was not retained",
            error,
        );
    }
    let choice = shell.show(ctx, |ui| {
        ui.spacing_mut().item_spacing.y = 8.0;
        match &mut dialog {
            ModelCorrelationDialog::AddDataset(draft) => dataset_dialog_body(ui, app, draft),
            ModelCorrelationDialog::AddMetric(draft) => metric_dialog_body(ui, draft),
            ModelCorrelationDialog::Review(draft) => review_dialog_body(ui, app, draft),
            ModelCorrelationDialog::Disposition(draft) => disposition_dialog_body(ui, draft),
        }
    });

    let keep_open = match choice {
        DialogChoice::None | DialogChoice::Secondary => true,
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            clear_correlation_dialog_runtime(ctx);
            app.state.workbench.model_correlation.dialog_source = None;
            false
        }
        DialogChoice::Primary => {
            let result = correlation_dialog_source_blocker(app).map_or_else(
                || match &dialog {
                    ModelCorrelationDialog::AddDataset(draft) => {
                        model_correlation_controller::append_dataset(app, draft)
                    }
                    ModelCorrelationDialog::AddMetric(draft) => {
                        model_correlation_controller::append_metric(app, draft)
                    }
                    ModelCorrelationDialog::Review(draft) => {
                        model_correlation_controller::append_review_evidence(app, draft)
                    }
                    ModelCorrelationDialog::Disposition(draft) => {
                        model_correlation_controller::append_disposition(app, draft)
                    }
                },
                Err,
            );
            match result {
                Ok(()) => {
                    clear_correlation_dialog_runtime(ctx);
                    app.state.workbench.model_correlation.dialog_source = None;
                    false
                }
                Err(error) => {
                    ctx.data_mut(|data| {
                        data.insert_temp(correlation_dialog_error_id(), error);
                    });
                    true
                }
            }
        }
    };
    if keep_open {
        app.state.workbench.model_correlation.dialog = Some(dialog);
    }
}

pub(super) fn form_text_row(ui: &mut Ui, label: &str, value: &mut String) {
    let label_response = ui.label(label);
    let input_response = ui.add_sized(
        [dialog_field_width(ui), Tokens::get(ui.ctx()).metrics.ctl_h],
        egui::TextEdit::singleline(value),
    );
    ui.ctx().accesskit_node_builder(input_response.id, |node| {
        node.push_labelled_by(label_response.id.accesskit_id());
    });
    ui.end_row();
}

#[derive(Debug, Clone)]
pub(super) struct RetainedTraceOption {
    name: String,
    sample_count: usize,
}

#[derive(Debug, Clone)]
pub(super) struct RetainedAnalysisOption {
    id: u64,
    label: String,
    kind: String,
    traces: Vec<RetainedTraceOption>,
}

#[derive(Debug, Clone)]
pub(super) struct RetainedRunOption {
    id: String,
    label: String,
    plan_id: String,
    execution_target: String,
    analyses: Vec<RetainedAnalysisOption>,
}

pub(super) fn retained_run_options(app: &RSpiceApp) -> Vec<RetainedRunOption> {
    let Some(source) = app.state.workbench.model_correlation.dialog_source.as_ref() else {
        return Vec::new();
    };
    let Some(source_id) = source.source_id else {
        return Vec::new();
    };
    app.state
        .simulation
        .runs
        .iter()
        .filter(|run| {
            run.lifecycle == crate::state::SimulationRunLifecycle::Completed
                && run.success
                && run.validate_provenance().is_ok()
        })
        .filter_map(|run| {
            let receipt = run.prepared_receipt()?;
            let plan_id = receipt.simulation_plan_id()?;
            let execution_target = run.execution_target?;
            if !receipt.project_model_sources().iter().any(|candidate| {
                candidate.source_id() == source_id
                    && candidate
                        .model_name()
                        .eq_ignore_ascii_case(&source.model_id)
                    && candidate.revision() == source.source_revision
                    && candidate.content_digest() == source.source_digest
            }) {
                return None;
            }
            let analyses = run
                .analyses
                .iter()
                .filter(|analysis| analysis.success && analysis.provenance().is_some())
                .filter_map(|analysis| {
                    let traces = analysis
                        .waveforms
                        .iter()
                        .filter(|trace| {
                            !trace.x.is_empty()
                                && trace.x.len() == trace.y.len()
                                && trace
                                    .x
                                    .iter()
                                    .chain(trace.y.iter())
                                    .all(|value| value.is_finite())
                        })
                        .map(|trace| RetainedTraceOption {
                            name: trace.name.clone(),
                            sample_count: trace.x.len(),
                        })
                        .collect::<Vec<_>>();
                    (!traces.is_empty()).then(|| RetainedAnalysisOption {
                        id: analysis.id,
                        label: analysis.label.clone(),
                        kind: analysis.analysis_type.display_name().to_owned(),
                        traces,
                    })
                })
                .collect::<Vec<_>>();
            (!analyses.is_empty()).then(|| RetainedRunOption {
                id: run.run_id.to_string(),
                label: run.label.clone(),
                plan_id: plan_id.to_string(),
                execution_target: execution_target.label().to_owned(),
                analyses,
            })
        })
        .collect()
}

pub(super) fn dataset_dialog_body(
    ui: &mut Ui,
    app: &RSpiceApp,
    draft: &mut CorrelationDatasetDraft,
) {
    let projection = CorrelationProjection::from_app(ui.ctx(), app);
    let current_source = model_correlation_controller::current_source_binding(app).ok();
    let current_suite = current_source.as_ref().and_then(|source| {
        projection
            .correlation
            .suite_lineages()
            .into_iter()
            .find(|suite| suite.source == *source)
    });
    ui.label("Import immutable UTF-8 CSV evidence and retain its source identity and checksum.");
    egui::Grid::new("correlation-dataset-form")
        .num_columns(2)
        .min_col_width(136.0)
        .spacing(Vec2::new(14.0, 8.0))
        .show(ui, |ui| {
            form_text_row(ui, "Dataset ID", &mut draft.id);
            form_text_row(ui, "Name", &mut draft.name);
            if let Some(suite) = current_suite {
                ui.label("Suite owner identity");
                ui.monospace(&suite.owner_id);
                ui.end_row();
            } else {
                form_text_row(ui, "Suite owner identity", &mut draft.suite_owner_id);
            }
            ui.label("Evidence class");
            egui::ComboBox::from_id_salt("correlation-dataset-class")
                .selected_text(draft.class.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationDatasetClassDraft::ALL {
                        ui.selectable_value(&mut draft.class, value, value.label());
                    }
                });
            ui.end_row();
            if draft.class != CorrelationDatasetClassDraft::Simulated {
                form_text_row(ui, "Authority", &mut draft.authority);
                form_text_row(ui, "Device or lot", &mut draft.device_or_lot);
                form_text_row(ui, "Fixture", &mut draft.fixture);
                form_text_row(ui, "Calibration", &mut draft.calibration);
                form_text_row(ui, "Source name", &mut draft.source_name);
            }
        });
    ui.add_space(8.0);
    if draft.class == CorrelationDatasetClassDraft::Simulated {
        retained_simulation_picker(ui, app, draft);
    } else {
        ui.label("CSV source");
        ui.label(
            "Required columns: id, quantity, value, unit. Optional: uncertainty, weight, condition:<name>[<unit>].",
        );
        ui.add_sized(
            [ui.available_width().max(1.0), 170.0],
            egui::TextEdit::multiline(&mut draft.csv)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .code_editor()
                .desired_rows(8),
        );
    }
}

pub(super) fn retained_simulation_picker(
    ui: &mut Ui,
    app: &RSpiceApp,
    draft: &mut CorrelationDatasetDraft,
) {
    let runs = retained_run_options(app);
    section_panel(ui, "Authenticated retained simulation", |ui| {
        ui.label(
            "Select a completed run prepared against this exact project model revision. RSpice generates and hashes the canonical CSV from the retained waveform.",
        );
        ui.add_space(6.0);
        egui::Grid::new("correlation-retained-simulation-form")
            .num_columns(2)
            .min_col_width(136.0)
            .spacing(Vec2::new(14.0, 8.0))
            .show(ui, |ui| {
                ui.label("Retained run");
                let selected_run_label = runs
                    .iter()
                    .find(|run| run.id == draft.retained_run_id)
                    .map_or("Select a completed run", |run| run.label.as_str());
                let previous_run = draft.retained_run_id.clone();
                egui::ComboBox::from_id_salt("correlation-retained-run")
                    .selected_text(selected_run_label)
                    .width(dialog_field_width(ui))
                    .show_ui(ui, |ui| {
                        for run in &runs {
                            ui.selectable_value(
                                &mut draft.retained_run_id,
                                run.id.clone(),
                                &run.label,
                            );
                        }
                    });
                if draft.retained_run_id != previous_run {
                    draft.retained_analysis_id.clear();
                    draft.retained_trace_name.clear();
                }
                ui.end_row();

                let selected_run = runs.iter().find(|run| run.id == draft.retained_run_id);
                ui.label("Analysis");
                let selected_analysis_label = selected_run
                    .and_then(|run| {
                        run.analyses
                            .iter()
                            .find(|analysis| analysis.id.to_string() == draft.retained_analysis_id)
                    })
                    .map_or("Select an analysis".to_owned(), |analysis| {
                        format!("{} · {}", analysis.kind, analysis.label)
                    });
                let previous_analysis = draft.retained_analysis_id.clone();
                ui.add_enabled_ui(selected_run.is_some(), |ui| {
                    egui::ComboBox::from_id_salt("correlation-retained-analysis")
                        .selected_text(selected_analysis_label)
                        .width(dialog_field_width(ui))
                        .show_ui(ui, |ui| {
                            if let Some(run) = selected_run {
                                for analysis in &run.analyses {
                                    ui.selectable_value(
                                        &mut draft.retained_analysis_id,
                                        analysis.id.to_string(),
                                        format!("{} · {}", analysis.kind, analysis.label),
                                    );
                                }
                            }
                        });
                });
                if draft.retained_analysis_id != previous_analysis {
                    draft.retained_trace_name.clear();
                }
                ui.end_row();

                let selected_analysis = selected_run.and_then(|run| {
                    run.analyses
                        .iter()
                        .find(|analysis| analysis.id.to_string() == draft.retained_analysis_id)
                });
                ui.label("Waveform");
                let selected_trace_label = selected_analysis
                    .and_then(|analysis| {
                        analysis
                            .traces
                            .iter()
                            .find(|trace| trace.name == draft.retained_trace_name)
                    })
                    .map_or("Select a retained waveform".to_owned(), |trace| {
                        format!("{} · {} samples", trace.name, trace.sample_count)
                    });
                ui.add_enabled_ui(selected_analysis.is_some(), |ui| {
                    egui::ComboBox::from_id_salt("correlation-retained-trace")
                        .selected_text(selected_trace_label)
                        .width(dialog_field_width(ui))
                        .show_ui(ui, |ui| {
                            if let Some(analysis) = selected_analysis {
                                for trace in &analysis.traces {
                                    ui.selectable_value(
                                        &mut draft.retained_trace_name,
                                        trace.name.clone(),
                                        format!("{} · {} samples", trace.name, trace.sample_count),
                                    );
                                }
                            }
                        });
                });
                ui.end_row();
            });
        if let Some(run) = runs.iter().find(|run| run.id == draft.retained_run_id) {
            ui.add_space(6.0);
            key_value(ui, "Simulation plan", &run.plan_id);
            key_value(ui, "Execution target", &run.execution_target);
        } else if runs.is_empty() {
            ui.add_space(6.0);
            ui.colored_label(
                Tokens::get(ui.ctx()).color.warn,
                "No successfully completed retained run authenticates this exact model revision. Run the current simulation plan, then reopen this dialog.",
            );
        }
    });
}

pub(super) fn metric_dialog_body(ui: &mut Ui, draft: &mut CorrelationMetricDraft) {
    ui.label("Define a unit-aware, source-bound comparison contract.");
    egui::Grid::new("correlation-metric-form")
        .num_columns(2)
        .min_col_width(150.0)
        .spacing(Vec2::new(14.0, 8.0))
        .show(ui, |ui| {
            form_text_row(ui, "Metric ID", &mut draft.id);
            form_text_row(ui, "Name", &mut draft.name);
            form_text_row(ui, "Quantity", &mut draft.quantity);
            form_text_row(ui, "Reference dataset ID", &mut draft.reference_dataset_id);
            form_text_row(
                ui,
                "Simulation dataset ID",
                &mut draft.simulation_dataset_id,
            );
            ui.label("Calculation");
            egui::ComboBox::from_id_salt("correlation-metric-calculation")
                .selected_text(draft.calculation.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationCalculationDraft::ALL {
                        ui.selectable_value(&mut draft.calculation, value, value.label());
                    }
                });
            ui.end_row();
            form_text_row(ui, "Limit", &mut draft.limit);
            form_text_row(
                ui,
                "Uncertainty multiplier",
                &mut draft.uncertainty_multiplier,
            );
            form_text_row(ui, "Minimum coverage (0..1)", &mut draft.minimum_coverage);
            ui.label("Aggregation");
            egui::ComboBox::from_id_salt("correlation-metric-aggregation")
                .selected_text(draft.aggregation.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationAggregationDraft::ALL {
                        ui.selectable_value(&mut draft.aggregation, value, value.label());
                    }
                });
            ui.end_row();
            ui.label("Alignment");
            egui::ComboBox::from_id_salt("correlation-metric-alignment")
                .selected_text(draft.alignment.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationAlignmentDraft::ALL {
                        ui.selectable_value(&mut draft.alignment, value, value.label());
                    }
                });
            ui.end_row();
            ui.label("Release role");
            egui::ComboBox::from_id_salt("correlation-metric-release-role")
                .selected_text(draft.release_role.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationReleaseRoleDraft::ALL {
                        ui.selectable_value(&mut draft.release_role, value, value.label());
                    }
                });
            ui.end_row();
        });
    ui.add_space(8.0);
    section_panel(ui, "Operating domain", |ui| {
        ui.checkbox(
            &mut draft.domain_enabled,
            "Restrict metric to a declared axis range",
        );
        ui.add_enabled_ui(draft.domain_enabled, |ui| {
            egui::Grid::new("correlation-metric-domain-form")
                .num_columns(2)
                .min_col_width(150.0)
                .spacing(Vec2::new(14.0, 8.0))
                .show(ui, |ui| {
                    form_text_row(ui, "Axis", &mut draft.domain_axis);
                    form_text_row(ui, "Unit", &mut draft.domain_unit);
                    form_text_row(ui, "Minimum", &mut draft.domain_minimum);
                    form_text_row(ui, "Maximum", &mut draft.domain_maximum);
                });
        });
    });
    ui.add_space(8.0);
    section_panel(ui, "Alignment and extrapolation", |ui| {
        ui.label(
            if draft.alignment == CorrelationAlignmentDraft::MonotoneInterpolation {
                "Declare the interpolation axis and the exact extrapolation boundary."
            } else {
                "Exact alignment never interpolates or extrapolates."
            },
        );
        ui.add_space(6.0);
        ui.add_enabled_ui(
            draft.alignment == CorrelationAlignmentDraft::MonotoneInterpolation,
            |ui| {
                egui::Grid::new("correlation-metric-alignment-form")
                    .num_columns(2)
                    .min_col_width(150.0)
                    .spacing(Vec2::new(14.0, 8.0))
                    .show(ui, |ui| {
                        form_text_row(ui, "Alignment axis", &mut draft.alignment_axis);
                        ui.label("Extrapolation");
                        egui::ComboBox::from_id_salt("correlation-metric-extrapolation")
                            .selected_text(draft.extrapolation.label())
                            .width(dialog_field_width(ui))
                            .show_ui(ui, |ui| {
                                for value in CorrelationExtrapolationDraft::ALL {
                                    ui.selectable_value(
                                        &mut draft.extrapolation,
                                        value,
                                        value.label(),
                                    );
                                }
                            });
                        ui.end_row();
                        form_text_row(
                            ui,
                            "Maximum span fraction",
                            &mut draft.extrapolation_fraction,
                        );
                    });
            },
        );
    });
}

pub(super) fn review_dialog_body(ui: &mut Ui, app: &RSpiceApp, draft: &mut CorrelationReviewDraft) {
    let projection = CorrelationProjection::from_app(ui.ctx(), app);
    section_panel(ui, "Evaluation to be retained", |ui| {
        if let Some(evaluation) = projection.evaluation.as_ref() {
            status_text(
                ui,
                evaluation.passed,
                if evaluation.passed {
                    "numerical gate passed"
                } else {
                    "numerical gate failed"
                },
            );
            key_value(ui, "Suite", &evaluation.suite_id);
            key_value(
                ui,
                "Suite revision",
                &evaluation.suite_revision.get().to_string(),
            );
            key_value(ui, "Metrics", &evaluation.metric_outcomes.len().to_string());
            key_value(
                ui,
                "Suite SHA-256",
                &short_digest(&evaluation.suite_digest.to_string()),
            );
        } else {
            ui.label("The selected suite does not currently produce a valid evaluation.");
        }
    });
    ui.add_space(8.0);
    egui::Grid::new("correlation-review-form")
        .num_columns(2)
        .min_col_width(150.0)
        .spacing(Vec2::new(14.0, 8.0))
        .show(ui, |ui| {
            ui.label("Suite owner");
            ui.monospace(
                projection
                    .suite
                    .as_ref()
                    .map_or("Unavailable", |suite| suite.owner_id.as_str()),
            );
            ui.end_row();
            form_text_row(ui, "Independent reviewer", &mut draft.reviewer_id);
            ui.label("Decision");
            egui::ComboBox::from_id_salt("correlation-review-decision")
                .selected_text(draft.decision.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationReviewDecisionDraft::ALL {
                        let enabled = value != CorrelationReviewDecisionDraft::Approve
                            || projection
                                .evaluation
                                .as_ref()
                                .is_some_and(|evaluation| evaluation.passed);
                        ui.add_enabled_ui(enabled, |ui| {
                            ui.selectable_value(&mut draft.decision, value, value.label());
                        });
                    }
                });
            ui.end_row();
            form_text_row(ui, "Conclusion", &mut draft.conclusion);
        });
}

pub(super) fn disposition_dialog_body(ui: &mut Ui, draft: &mut CorrelationOutlierDraft) {
    ui.label("Retain an append-only, independently reviewed disposition for this residual.");
    egui::Grid::new("correlation-disposition-form")
        .num_columns(2)
        .min_col_width(160.0)
        .spacing(Vec2::new(14.0, 8.0))
        .show(ui, |ui| {
            ui.label("Metric ID");
            ui.monospace(&draft.metric_id);
            ui.end_row();
            ui.label("Reference point");
            ui.monospace(&draft.reference_observation_id);
            ui.end_row();
            ui.label("Decision");
            egui::ComboBox::from_id_salt("correlation-outlier-decision")
                .selected_text(draft.decision.label())
                .width(dialog_field_width(ui))
                .show_ui(ui, |ui| {
                    for value in CorrelationOutlierDecisionDraft::ALL {
                        ui.selectable_value(&mut draft.decision, value, value.label());
                    }
                });
            ui.end_row();
            form_text_row(ui, "Reason", &mut draft.reason);
            form_text_row(ui, "Owner", &mut draft.owner_id);
            form_text_row(ui, "Independent reviewer", &mut draft.reviewer_id);
        });
}

pub(super) fn bottom_rule(ui: &Ui, rect: Rect, color: Color32) {
    ui.painter()
        .hline(rect.x_range(), rect.bottom(), Stroke::new(1.0, color));
}

pub(super) fn paint_elided(
    ui: &Ui,
    rect: Rect,
    text: &str,
    font: egui::FontId,
    color: Color32,
    y_offset: f32,
) {
    let value = design_system::elide_text(ui, text, &font, rect.width().max(1.0));
    ui.painter().with_clip_rect(rect).text(
        Pos2::new(rect.left(), rect.top() + y_offset),
        Align2::LEFT_TOP,
        value,
        font,
        color,
    );
}

pub(super) fn describe_region(ui: &Ui, response: &egui::Response, label: &str, description: &str) {
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, response.enabled(), label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Group);
        node.set_label(label);
        node.set_description(description);
    });
}

pub(super) fn tone_color(t: &Tokens, tone: Tone) -> Color32 {
    match tone {
        Tone::Neutral => t.color.text,
        Tone::Good => t.color.ok,
        Tone::Warning => t.color.warn,
        Tone::Error => t.color.err,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_copy_is_complete_for_the_mockup_contract() {
        for section in ModelCorrelationSection::ALL {
            let (title, description) = section_copy(section);
            assert!(!title.is_empty());
            assert!(!description.is_empty());
        }
    }

    fn empty_projection() -> CorrelationProjection {
        CorrelationProjection {
            model: Some("dut".to_owned()),
            source_error: None,
            suite_name: "dut · no retained correlation suite".to_owned(),
            correlation: ModelCorrelationState::default(),
            suite: None,
            evaluation: None,
            evaluation_error: None,
            current_evidence: Vec::new(),
            source_current: false,
            suite_count: 0,
            metric_count: 0,
            passed_metrics: 0,
            coverage_percent: None,
            worst_normalized_error: None,
            open_dispositions: 0,
        }
    }

    #[test]
    fn empty_projection_never_claims_correlation_evidence() {
        let projection = empty_projection();
        assert_eq!(
            projection.state_label(),
            ("correlation suite not configured", Tone::Warning)
        );
        assert!(
            projection
                .measures()
                .iter()
                .all(|measure| { measure.value == "0 / 0" || measure.value == "No evidence" })
        );
    }

    /// Absence is stated as absence, in the tone that says so.
    ///
    /// The ledger's middle segments are the only place a reader learns whether
    /// a measure exists at all, so an unevaluated suite may not borrow the
    /// neutral tone of a measured one, and may not print a number.
    #[test]
    fn ledger_measures_without_evidence_stay_in_the_warning_tone() {
        let measures = empty_projection().measures();
        assert_eq!(measures[1].value, "No evidence");
        assert_eq!(measures[1].tone, Tone::Warning);
        assert_eq!(measures[2].value, "No evidence");
        assert_eq!(measures[2].tone, Tone::Warning);
        assert_eq!(measures[0].value, "0 / 0");
        assert_eq!(measures[0].tone, Tone::Warning);
        assert_eq!(measures[0].detail, "0 open dispositions");
    }

    /// Every ledger tone is a claim about evidence, not about layout.
    ///
    /// A gate set is only good when every review metric passed *and* nothing
    /// is left undisposed, and a worst normalized error is only good at or
    /// inside its declared limit.
    #[test]
    fn ledger_measure_tones_follow_the_evidence() {
        let passing = CorrelationProjection {
            metric_count: 4,
            passed_metrics: 4,
            coverage_percent: Some(92.14),
            worst_normalized_error: Some(0.812),
            ..empty_projection()
        };
        let measures = passing.measures();
        assert_eq!(
            (measures[0].short, measures[0].value.as_str()),
            ("gates", "4 / 4")
        );
        assert_eq!(measures[0].tone, Tone::Good);
        assert_eq!(
            (measures[1].short, measures[1].value.as_str()),
            ("coverage", "92.1%")
        );
        assert_eq!(measures[1].tone, Tone::Neutral);
        assert_eq!(
            (measures[2].short, measures[2].value.as_str()),
            ("worst", "0.812×")
        );
        assert_eq!(measures[2].tone, Tone::Good);

        let disputed = CorrelationProjection {
            open_dispositions: 3,
            worst_normalized_error: Some(1.001),
            ..passing
        };
        let measures = disputed.measures();
        assert_eq!(measures[0].tone, Tone::Warning);
        assert_eq!(measures[0].detail, "3 open dispositions");
        assert_eq!(measures[2].value, "1.001×");
        assert_eq!(measures[2].tone, Tone::Warning);
    }

    /// The abbreviation the band paints is never what a screen reader hears.
    #[test]
    fn ledger_segments_name_themselves_in_full_for_assistive_technology() {
        assert_eq!(
            empty_projection().measures().map(|measure| measure.label),
            [
                "Metric gates",
                "Envelope coverage",
                "Worst normalized error"
            ]
        );
    }

    #[test]
    fn bounded_pages_clamp_without_growing_the_render_window() {
        assert_eq!(
            bounded_page(0, 24, usize::MAX),
            BoundedPage {
                index: 0,
                count: 1,
                start: 0,
                end: 0,
                total: 0,
            }
        );
        assert_eq!(
            bounded_page(49, 24, usize::MAX),
            BoundedPage {
                index: 2,
                count: 3,
                start: 48,
                end: 49,
                total: 49,
            }
        );
        let middle = bounded_page(49, 24, 1);
        assert_eq!(middle.end - middle.start, 24);
    }

    #[test]
    fn plot_sampling_is_deterministic_bounded_and_keeps_endpoints() {
        let first = deterministic_sample_indices(10_000, 192);
        let second = deterministic_sample_indices(10_000, 192);
        assert_eq!(first, second);
        assert_eq!(first.len(), 192);
        assert_eq!(first.first(), Some(&0));
        assert_eq!(first.last(), Some(&9_999));
        assert!(first.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(deterministic_sample_indices(3, 192), vec![0, 1, 2]);
    }

    #[test]
    fn long_source_lists_disclose_omitted_members() {
        let sources = (0..8)
            .map(|index| format!("run-{index}"))
            .collect::<Vec<_>>();
        assert_eq!(
            bounded_string_list(&sources, 4),
            "run-0, run-1, run-2, run-3, … +4 more"
        );
    }

    /// A correlation suite that actually evaluates, bound to a project-owned
    /// model that the surface can resolve, so the ledger is exercised against
    /// numbers rather than against the empty state.
    #[cfg(not(target_arch = "wasm32"))]
    fn correlated_app() -> RSpiceApp {
        use crate::product::ContentDigest;
        use crate::state::model_library::{
            CorrelationDatasetRevision, CorrelationMetricDefinition,
            CorrelationSimulationProvenance, ModelLibraryManager, ProjectModelDefinition,
        };

        const REFERENCE: &[u8] = b"id,quantity,value,unit,uncertainty,weight,condition:frequency[Hz],condition:temperature[degC]\n\
r1,gain,0,dB,0.05,1,10,27\n\
r2,gain,-1,dB,0.05,1,100,27\n\
r3,gain,-2,dB,0.05,1,1000,27\n";
        const SIMULATED: &[u8] = b"id,quantity,value,unit,uncertainty,weight,condition:frequency[Hz],condition:temperature[degC]\n\
s1,gain,0.05,dB,0.04,1,10,27\n\
s2,gain,-0.95,dB,0.04,1,100,27\n\
s3,gain,-1.9,dB,0.04,1,1000,27\n";

        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager = ModelLibraryManager::new();
        let definition = ProjectModelDefinition {
            name: "nch_corr".to_owned(),
            spice_type: "NMOS".to_owned(),
            description: "Correlation ledger fixture".to_owned(),
            numeric_parameters: BTreeMap::from([("level".to_owned(), 1.0)]),
            string_parameters: BTreeMap::new(),
        };
        app.state
            .model_library_manager
            .create_project_model("owned-models", &definition)
            .expect("project model");
        let resolved = resolve_project_model_for_editor(
            &app.state.model_library_manager,
            "owned-models",
            "nch_corr",
        )
        .expect("resolved project model");
        let source = ModelSourceEvidenceBinding::try_new_project_bound(
            "nch_corr",
            resolved.source_id,
            resolved.model_digest,
            resolved.model_revision,
        )
        .expect("source binding");

        let reference = CorrelationDatasetRevision::try_from_csv(
            "reference",
            ObjectRevision::INITIAL,
            "Bench sweep",
            CorrelationDatasetClass::BenchMeasurement,
            "qualified test authority",
            "lot-1",
            "fixture-1",
            "calibration-1",
            "bench.csv",
            REFERENCE.to_vec(),
            None,
        )
        .expect("reference dataset");
        // The retained export digest is the digest of the exported bytes, and
        // the constructor is the only thing that computes it.
        let export_digest = CorrelationDatasetRevision::try_from_csv(
            "simulation",
            ObjectRevision::INITIAL,
            "Model simulation",
            CorrelationDatasetClass::BenchMeasurement,
            "qualified test authority",
            "lot-1",
            "fixture-1",
            "calibration-1",
            "simulation.csv",
            SIMULATED.to_vec(),
            None,
        )
        .expect("simulation digest probe")
        .raw_digest;
        let simulation = CorrelationDatasetRevision::try_from_csv_with_provenance(
            "simulation",
            ObjectRevision::INITIAL,
            "Model simulation",
            CorrelationDatasetClass::ModelSimulation,
            "qualified test authority",
            "lot-1",
            "fixture-1",
            "calibration-1",
            "simulation.csv",
            SIMULATED.to_vec(),
            Some(source.clone()),
            Some(CorrelationSimulationProvenance {
                run_id: "correlation-run".to_owned(),
                run_dataset_id: "correlation-run-dataset".to_owned(),
                analysis_id: 1,
                analysis_result_digest: ContentDigest::from_bytes([0x43; 32]),
                plan_id: "correlation-plan".to_owned(),
                project_revision: ObjectRevision::INITIAL,
                prepared_snapshot_digest: ContentDigest::from_bytes([0x44; 32]),
                source_content_digest: ContentDigest::from_bytes([0x45; 32]),
                task_config_digest: ContentDigest::from_bytes([0x55; 32]),
                execution_target: "test-platform".to_owned(),
                export_digest,
                model_source: source.clone(),
                executed_at_unix_ms: 1,
            }),
        )
        .expect("simulation dataset");
        let metric = CorrelationMetricDefinition::try_new(
            "gain-error",
            "Gain error",
            "reference",
            "simulation",
            "gain",
            CorrelationCalculation::AbsoluteDecibels,
            None,
            0.25,
            1.0,
            0.5,
            CorrelationAggregation::WorstCondition,
            CorrelationAlignmentPolicy::ExactOnly,
            CorrelationReleaseRole::Review,
        )
        .expect("metric definition");
        let suite = CorrelationSuite::try_new(
            "gain-correlation",
            ObjectRevision::INITIAL,
            "Gain correlation",
            "model-owner",
            source,
            vec![reference, simulation],
            vec![metric],
            Vec::new(),
        )
        .expect("correlation suite");
        let correlation =
            ModelCorrelationState::try_new(vec![suite], Vec::new()).expect("correlation state");
        app.state
            .model_library_manager
            .replace_project_model_correlation(
                "owned-models",
                resolved.source_id,
                resolved.library_revision,
                resolved.model_revision,
                resolved.model_digest,
                "nch_corr",
                &correlation,
            )
            .expect("retained correlation");
        app.state.model_library_manager.selected_library = Some("owned-models".to_owned());
        app.state.workbench.selected_model = Some("nch_corr".to_owned());
        app
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn correlation_nodes(
        app: &mut RSpiceApp,
        size: Vec2,
    ) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| show(ui, app));
            },
        );
        output
            .platform_output
            .accesskit_update
            .expect("correlation accessibility tree")
            .nodes
    }

    /// Each ledger segment is one readable region, not three loose glyphs.
    ///
    /// The band paints an abbreviation beside a number; the caption that made
    /// the number mean something used to sit under it in the strip and now
    /// lives on the segment's own hover and accessible name, so nothing that
    /// was stated is silently dropped.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_ledger_publishes_every_measure_with_its_value_and_caption() {
        let mut app = correlated_app();
        let nodes = correlation_nodes(&mut app, Vec2::new(1_180.0, 900.0));
        let projection = CorrelationProjection::build(&app);
        for measure in projection.measures() {
            let expected = format!("{}: {}. {}", measure.label, measure.value, measure.detail);
            // egui gives `WidgetType::Label` AccessKit's Label role, whose
            // text is the node's *value*, not its `label()`.
            assert!(
                nodes
                    .iter()
                    .any(|(_, node)| node.value() == Some(expected.as_str())),
                "the ledger never published '{expected}'"
            );
        }
        assert_eq!(projection.measures()[1].value, "100.0%");
        assert_eq!(projection.measures()[0].value, "1 / 1");
    }

    /// One fact, one owner.
    ///
    /// The state verdict was painted in the identity band and repeated as the
    /// summary region's description; it now terminates the evidence ledger and
    /// is stated exactly once on the page.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_state_verdict_is_stated_once_and_on_the_ledger() {
        let mut app = correlated_app();
        let nodes = correlation_nodes(&mut app, Vec2::new(1_180.0, 900.0));
        let projection = CorrelationProjection::build(&app);
        let (state, _) = projection.state_label();
        let owners = nodes
            .iter()
            .filter(|(_, node)| {
                node.label().is_some_and(|label| label.contains(state))
                    || node
                        .description()
                        .is_some_and(|description| description.contains(state))
            })
            .collect::<Vec<_>>();
        assert_eq!(
            owners.len(),
            1,
            "the verdict '{state}' has {} owners on the page",
            owners.len()
        );
        assert_eq!(owners[0].1.label(), Some(LEDGER_TITLE));
    }

    /// The datasets panel lists the retained revisions, so it states how many
    /// it holds; the ledger band does not.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_datasets_panel_owns_the_retained_dataset_count() {
        let mut app = correlated_app();
        app.state.workbench.model_correlation.section = ModelCorrelationSection::Datasets;
        let nodes = correlation_nodes(&mut app, Vec2::new(1_180.0, 900.0));
        assert!(
            nodes
                .iter()
                .any(|(_, node)| node.value() == Some("2 datasets")),
            "the datasets panel head never stated its count"
        );
        assert!(
            CorrelationProjection::build(&app)
                .measures()
                .iter()
                .all(|measure| measure.label != "Datasets"),
            "the ledger took the dataset count back"
        );
    }

    /// The band's height is the body's, and the body takes all of it.
    ///
    /// The strip this replaces was 62 px per row and could be two rows; the
    /// ledger is one 54 px band at every width, so the workspace below starts
    /// exactly that far down and nothing is left unpainted where the second
    /// row used to be.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_workspace_body_starts_directly_under_the_ledger_band() {
        let mut app = correlated_app();
        for width in [1_180.0, 900.0] {
            let nodes = correlation_nodes(&mut app, Vec2::new(width, 900.0));
            let bounds = nodes
                .iter()
                .find(|(_, node)| {
                    node.role() == egui::accesskit::Role::Tab && node.label() == Some("Datasets")
                })
                .and_then(|(_, node)| node.bounds())
                .expect("first correlation section tab");
            let expected = f64::from(OUTER_HEADER_H + SUMMARY_H + LEDGER_H);
            assert!(
                (bounds.y0 - expected).abs() < 0.5,
                "the workspace body starts at {} instead of {expected} at {width}",
                bounds.y0
            );
        }
    }

    /// Write the evidence ledger to PNGs so its design can be reviewed.
    ///
    /// Both states the band has to carry: an unconfigured page, where every
    /// measure is an absence, and an evaluated suite whose measures pass while
    /// the verdict still withholds. The compact width is the one that proves
    /// the sentence elides before the segments reach the title. Renders go to
    /// `RSPICE_RASTER_DIR` (default: the system temp directory); read them for
    /// layout, not wording — the rasterizer's own header says why.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn render_the_evidence_ledger_for_review() {
        use std::io::Write as _;

        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        let stderr = std::io::stderr();
        let mut report = stderr.lock();

        let mut render = |slug: &str, app: &mut RSpiceApp, size: Vec2| {
            let canvas = crate::ui::raster::render(size, |ui, background| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(background))
                    .show(ui, |ui| show(ui, app));
            });
            let path = directory.join(format!("model-correlation-{slug}.png"));
            let height = canvas.content_height().max(200);
            std::fs::write(&path, canvas.png(height)).expect("write png");
            writeln!(report, "wrote {}", path.display()).ok();
        };

        let mut unconfigured = RSpiceApp::test_instance();
        render(
            "ledger-unconfigured",
            &mut unconfigured,
            Vec2::new(1_180.0, 720.0),
        );
        let mut correlated = correlated_app();
        render(
            "ledger-evaluated",
            &mut correlated,
            Vec2::new(1_180.0, 720.0),
        );
        render("ledger-compact", &mut correlated, Vec2::new(760.0, 720.0));
        correlated.state.workbench.model_correlation.section = ModelCorrelationSection::Datasets;
        render(
            "datasets-count-owner",
            &mut correlated,
            Vec2::new(1_180.0, 720.0),
        );
    }
}
