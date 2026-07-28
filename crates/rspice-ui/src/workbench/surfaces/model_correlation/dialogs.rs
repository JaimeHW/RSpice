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

    #[test]
    fn empty_projection_never_claims_correlation_evidence() {
        let projection = CorrelationProjection {
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
            dataset_count: 0,
            metric_count: 0,
            passed_metrics: 0,
            coverage_percent: None,
            worst_normalized_error: None,
            open_dispositions: 0,
        };
        assert_eq!(
            projection.state_label(),
            ("correlation suite not configured", Tone::Warning)
        );
        assert!(projection.metrics().iter().all(|metric| {
            metric.value == "0" || metric.value == "0 / 0" || metric.value == "No evidence"
        }));
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
}
