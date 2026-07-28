//! The studio's docked side panels.
//!
//! Each dock is a single-purpose editor over the pane that is currently
//! selected — traces, axes, markers, annotations, the report page policy —
//! and only one is open at a time. Nothing here owns state: a dock reads
//! [`VisualizationStudioState`] and writes back through it, so closing a dock
//! never discards an edit.

use super::*;

impl VisualizationDock {
    pub(super) const fn title(self) -> &'static str {
        match self {
            Self::AddPane => "Add visualization pane",
            Self::TraceManager => "Trace and family manager",
            Self::CursorManager => "Cursor and marker manager",
            Self::DocumentProperties => "Document properties",
            Self::ReorderPanes => "Reorder visualization panes",
            Self::LinkGroups => "Axis and cursor link groups",
            Self::PageEditor => "Assign pane to report page",
            Self::Measurement => "Create result measurement",
            Self::Annotation => "Create result annotation",
            Self::FamilySlice => "Family slicing and pivot",
            Self::FamilyEncoding => "Family visual encoding",
            Self::FamilyFilter => "Advanced family filter",
            Self::Comparison => "Plan explicit comparison",
            Self::ExportPreset => "Save plot export preset",
            Self::Export => "Export visualization document",
        }
    }
}

pub(super) fn dock_body(ui: &mut Ui, app: &mut RSpiceApp, dock: VisualizationDock) -> bool {
    match dock {
        VisualizationDock::AddPane => add_pane_dock(ui, app),
        VisualizationDock::TraceManager => trace_manager_dock(ui, app),
        VisualizationDock::CursorManager => cursor_manager_dock(ui, app),
        VisualizationDock::DocumentProperties => properties_dock(ui, app),
        VisualizationDock::ReorderPanes => reorder_panes_dock(ui, app),
        VisualizationDock::LinkGroups => link_groups_dock(ui, app),
        VisualizationDock::PageEditor => page_editor_dock(ui, app),
        VisualizationDock::Measurement => measurement_dock(ui, app),
        VisualizationDock::Annotation => annotation_dock(ui, app),
        VisualizationDock::FamilySlice => family_slice_dock(ui, app),
        VisualizationDock::FamilyEncoding => family_encoding_dock(ui, app),
        VisualizationDock::FamilyFilter => family_filter_dock(ui, app),
        VisualizationDock::Comparison => comparison_dock(ui, app),
        VisualizationDock::ExportPreset => export_preset_dock(ui, app),
        VisualizationDock::Export => export_dock(ui, app),
    }
}

fn normalize_add_pane_draft(state: &mut AppState) {
    let draft_dataset = state.workbench.visualization_studio.draft_dataset_id;
    let draft_analysis = state.workbench.visualization_studio.draft_analysis_sequence;
    let draft_is_valid =
        draft_dataset
            .zip(draft_analysis)
            .is_some_and(|(dataset_id, analysis_sequence)| {
                state.simulation.runs.iter().any(|run| {
                    run.dataset_id == dataset_id
                        && run
                            .analyses
                            .iter()
                            .any(|analysis| analysis.id == analysis_sequence)
                })
            });
    if draft_is_valid {
        return;
    }
    let fallback = state
        .simulation
        .active_run()
        .and_then(|run| {
            state
                .simulation
                .active_analysis()
                .map(|analysis| (run.dataset_id, analysis.id))
        })
        .or_else(|| {
            state.simulation.runs.iter().find_map(|run| {
                run.analyses
                    .first()
                    .map(|analysis| (run.dataset_id, analysis.id))
            })
        });
    state.workbench.visualization_studio.draft_dataset_id = fallback.map(|binding| binding.0);
    state.workbench.visualization_studio.draft_analysis_sequence =
        fallback.map(|binding| binding.1);
}

fn selected_draft_analysis(state: &AppState) -> Option<&crate::state::AnalysisResult> {
    let studio = &state.workbench.visualization_studio;
    let dataset_id = studio.draft_dataset_id?;
    let analysis_sequence = studio.draft_analysis_sequence?;
    state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)?
        .analyses
        .iter()
        .find(|analysis| analysis.id == analysis_sequence)
}

fn add_pane_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · WORKSHEET LAYOUT",
        "Create a compatible viewer pane without disturbing existing link groups.",
    );
    normalize_add_pane_draft(&mut app.state);
    let draft_dataset = app.state.workbench.visualization_studio.draft_dataset_id;
    let draft_analysis = app
        .state
        .workbench
        .visualization_studio
        .draft_analysis_sequence;
    let options = NATIVE_VIEWERS.map(|viewer| {
        let definition = viewer_document(viewer_document_id(viewer));
        let availability = definition
            .ok_or_else(|| "Viewer document is not registered".to_owned())
            .and_then(|definition| {
                resolved_viewer_availability_for_binding(
                    &app.state,
                    definition,
                    draft_dataset,
                    draft_analysis,
                )
            });
        (viewer, availability)
    });
    egui::ComboBox::from_label("Viewer")
        .selected_text(
            app.state
                .workbench
                .visualization_studio
                .draft_viewer
                .label(),
        )
        .show_ui(ui, |ui| {
            for (viewer, availability) in &options {
                let response = ui.add_enabled_ui(availability.is_ok(), |ui| {
                    ui.selectable_value(
                        &mut app.state.workbench.visualization_studio.draft_viewer,
                        *viewer,
                        viewer.label(),
                    )
                });
                if let Err(reason) = availability {
                    response.response.on_hover_text(reason);
                }
            }
        });

    let selected_dataset_text = draft_dataset
        .and_then(|dataset_id| {
            app.state
                .simulation
                .runs
                .iter()
                .find(|run| run.dataset_id == dataset_id)
        })
        .map_or_else(
            || "Select retained dataset".to_owned(),
            |run| format!("{} · {}", run.label, short_dataset(run.dataset_id)),
        );
    egui::ComboBox::from_label("Dataset")
        .selected_text(selected_dataset_text)
        .show_ui(ui, |ui| {
            let rows: Vec<_> = app
                .state
                .simulation
                .runs
                .iter()
                .map(|run| {
                    (
                        run.dataset_id,
                        run.label.clone(),
                        run.analyses.first().map(|a| a.id),
                    )
                })
                .collect();
            for (dataset_id, label, first_analysis) in rows {
                if ui
                    .selectable_value(
                        &mut app.state.workbench.visualization_studio.draft_dataset_id,
                        Some(dataset_id),
                        format!("{} · {}", label, short_dataset(dataset_id)),
                    )
                    .clicked()
                {
                    app.state
                        .workbench
                        .visualization_studio
                        .draft_analysis_sequence = first_analysis;
                }
            }
        });

    let draft_dataset = app.state.workbench.visualization_studio.draft_dataset_id;
    let selected_analysis_text = selected_draft_analysis(&app.state).map_or_else(
        || "Select retained analysis".to_owned(),
        |analysis| format!("{} · {}", analysis.label, analysis.id),
    );
    ui.add_enabled_ui(draft_dataset.is_some(), |ui| {
        egui::ComboBox::from_label("Analysis")
            .selected_text(selected_analysis_text)
            .show_ui(ui, |ui| {
                let rows: Vec<_> = draft_dataset
                    .and_then(|dataset_id| {
                        app.state
                            .simulation
                            .runs
                            .iter()
                            .find(|run| run.dataset_id == dataset_id)
                    })
                    .map(|run| {
                        run.analyses
                            .iter()
                            .map(|analysis| {
                                (
                                    analysis.id,
                                    analysis.label.clone(),
                                    analysis_manifest_id(analysis.analysis_type),
                                )
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                for (analysis_id, label, kind) in rows {
                    ui.selectable_value(
                        &mut app
                            .state
                            .workbench
                            .visualization_studio
                            .draft_analysis_sequence,
                        Some(analysis_id),
                        format!("{label} · {kind} · {analysis_id}"),
                    );
                }
            });
    });

    let placement = app
        .state
        .workbench
        .visualization_studio
        .draft_pane_placement;
    egui::ComboBox::from_label("Placement")
        .selected_text(placement.label())
        .show_ui(ui, |ui| {
            for placement in VisualizationPanePlacement::ALL {
                ui.selectable_value(
                    &mut app
                        .state
                        .workbench
                        .visualization_studio
                        .draft_pane_placement,
                    placement,
                    placement.label(),
                );
            }
        });
    if app
        .state
        .workbench
        .visualization_studio
        .draft_pane_placement
        == VisualizationPanePlacement::NewWorksheetPage
    {
        ui.label("New page title");
        ui.text_edit_singleline(&mut app.state.workbench.visualization_studio.draft_page_title);
    }

    let selected_viewer = app.state.workbench.visualization_studio.draft_viewer;
    let selected_compatibility = options
        .iter()
        .find_map(|(viewer, availability)| (*viewer == selected_viewer).then_some(availability));
    let page_valid = app
        .state
        .workbench
        .visualization_studio
        .draft_pane_placement
        != VisualizationPanePlacement::NewWorksheetPage
        || !app
            .state
            .workbench
            .visualization_studio
            .draft_page_title
            .trim()
            .is_empty();
    let enabled = selected_compatibility.is_some_and(Result::is_ok) && page_valid;
    ui.add_space(10.0);
    let add = ui
        .add_enabled(enabled, egui::Button::new("Add pane"))
        .on_disabled_hover_text(
            selected_compatibility
                .and_then(|result| result.as_ref().err())
                .map_or(
                    "A retained compatible result analysis is required",
                    String::as_str,
                ),
        )
        .clicked();
    if add {
        let viewer = app.state.workbench.visualization_studio.draft_viewer;
        let dataset_id = app
            .state
            .workbench
            .visualization_studio
            .draft_dataset_id
            .expect("enabled add-pane action has a retained dataset");
        let analysis_sequence = app
            .state
            .workbench
            .visualization_studio
            .draft_analysis_sequence
            .expect("enabled add-pane action has a retained analysis");
        let placement = app
            .state
            .workbench
            .visualization_studio
            .draft_pane_placement;
        let page_title = app
            .state
            .workbench
            .visualization_studio
            .draft_page_title
            .trim()
            .to_owned();
        add_viewer_pane_bound(
            app,
            viewer_document_id(viewer),
            viewer,
            dataset_id,
            analysis_sequence,
            placement,
            page_title,
        );
    }
    add
}

fn trace_manager_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · SIGNALS · EXPRESSIONS",
        "Show or hide native source traces and create a derived expression for the selected analysis.",
    );
    let dataset_id = app.state.workbench.visualization_studio.draft_trace_dataset;
    let analysis_id = app
        .state
        .workbench
        .visualization_studio
        .draft_trace_analysis;
    let binding_exists = dataset_id
        .zip(analysis_id)
        .is_some_and(|(dataset, analysis)| {
            app.state.simulation.runs.iter().any(|run| {
                run.dataset_id == dataset
                    && run
                        .analyses
                        .iter()
                        .any(|candidate| candidate.id == analysis)
            })
        });
    if app
        .state
        .workbench
        .visualization_studio
        .draft_trace_visibility
        .is_empty()
    {
        empty_note(ui, "No active analysis exposes traces.");
    } else {
        for (waveform, visible) in &mut app
            .state
            .workbench
            .visualization_studio
            .draft_trace_visibility
        {
            ui.checkbox(visible, waveform.as_str());
        }
    }
    ui.add_space(8.0);
    if Button::new("Add expression…").show(ui).clicked() {
        open_dock(app, VisualizationDock::Measurement);
        return false;
    }
    let apply = ui
        .add_enabled(binding_exists, egui::Button::new("Apply trace changes"))
        .on_disabled_hover_text(
            "The immutable analysis bound when this dialog opened is unavailable",
        )
        .clicked();
    if apply {
        let visibility = app
            .state
            .workbench
            .visualization_studio
            .draft_trace_visibility
            .clone();
        if let Some((dataset_id, analysis_id)) = dataset_id.zip(analysis_id)
            && let Some(analysis) = app
                .state
                .simulation
                .runs
                .iter_mut()
                .find(|run| run.dataset_id == dataset_id)
                .and_then(|run| {
                    run.analyses
                        .iter_mut()
                        .find(|analysis| analysis.id == analysis_id)
                })
        {
            for waveform in &mut analysis.waveforms {
                if let Some((_, visible)) =
                    visibility.iter().find(|(name, _)| name == &waveform.name)
                {
                    waveform.visible = *visible;
                }
            }
            commit_visualization_revision(app);
        }
    }
    apply
}

fn cursor_manager_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · EXACT VALUES",
        "Manage linked cursors, source-sample markers, and exact-value behavior.",
    );
    ui.checkbox(
        &mut app.state.ui.results.linked_cursors,
        "Link A/B cursors across compatible panes",
    );
    property_row(
        ui,
        "Cursor A",
        &app.state
            .ui
            .results
            .cursors
            .a
            .map_or_else(|| "not placed".to_owned(), |x| format!("{x:.17e}")),
    );
    property_row(
        ui,
        "Cursor B",
        &app.state
            .ui
            .results
            .cursors
            .b
            .map_or_else(|| "not placed".to_owned(), |x| format!("{x:.17e}")),
    );
    ui.horizontal_wrapped(|ui| {
        if Button::new("Place next at midpoint").show(ui).clicked() {
            add_cursor_at_midpoint(app);
        }
        if Button::new("Add exact marker").show(ui).clicked() {
            add_marker_at_midpoint(app);
        }
        if Button::new("Clear cursors").show(ui).clicked() {
            app.state.ui.results.clear_cursors();
        }
        if Button::new("Clear markers").show(ui).clicked() {
            let result = app.state.workbench.visualization_studio.transact(|studio| {
                studio.markers.clear();
                Ok(())
            });
            report_visualization_commit(app, result);
        }
    });
    Button::new("Done").show(ui).clicked()
}

fn properties_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULT DOCUMENT · PRESENTATION POLICY",
        "Edit the current worksheet's retained display properties.",
    );
    let current_significant_digits = app.state.workbench.visualization_studio.significant_digits;
    let significant_digits = app
        .state
        .workbench
        .visualization_studio
        .draft_significant_digits
        .get_or_insert(current_significant_digits);
    ui.add(egui::Slider::new(significant_digits, 3..=17).text("Significant digits"));
    property_row(ui, "Engineering grid", "Renderer-managed major grid");
    property_row(ui, "Legend placement", "Inside plot · compact");
    let current_phase_continuous = app.state.ui.results.phase_continuous;
    let phase_continuous = app
        .state
        .workbench
        .visualization_studio
        .draft_phase_continuous
        .get_or_insert(current_phase_continuous);
    ui.checkbox(phase_continuous, "Continuous (unwrapped) phase display");
    let save = Button::new("Save properties").accent().show(ui).clicked();
    if save {
        let significant_digits = app
            .state
            .workbench
            .visualization_studio
            .draft_significant_digits;
        let phase_continuous = app
            .state
            .workbench
            .visualization_studio
            .draft_phase_continuous;
        let result = app.state.workbench.visualization_studio.transact(|studio| {
            if let Some(significant_digits) = significant_digits {
                studio.significant_digits = significant_digits;
            }
            Ok(())
        });
        if report_visualization_commit(app, result)
            && let Some(phase_continuous) = phase_continuous
        {
            app.state.ui.results.phase_continuous = phase_continuous;
        }
    }
    save
}

fn reorder_panes_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · WORKSHEET LAYOUT",
        "Move panes while preserving their stable identity, traces, and link groups.",
    );
    let active = app.state.workbench.visualization_studio.active_pane;
    let index = active.and_then(|id| {
        app.state
            .workbench
            .visualization_studio
            .draft_pane_order
            .iter()
            .position(|pane_id| *pane_id == id)
    });
    property_row(
        ui,
        "Selected pane",
        &active.map_or_else(|| "none".to_owned(), |id| format!("Pane {id:02}")),
    );
    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                index.is_some_and(|index| index > 0),
                egui::Button::new("Move before"),
            )
            .clicked()
            && let Some(index) = index
        {
            app.state
                .workbench
                .visualization_studio
                .draft_pane_order
                .swap(index, index - 1);
        }
        if ui
            .add_enabled(
                index.is_some_and(|index| {
                    index + 1
                        < app
                            .state
                            .workbench
                            .visualization_studio
                            .draft_pane_order
                            .len()
                }),
                egui::Button::new("Move after"),
            )
            .clicked()
            && let Some(index) = index
        {
            app.state
                .workbench
                .visualization_studio
                .draft_pane_order
                .swap(index, index + 1);
        }
    });
    let apply = Button::new("Apply pane order").accent().show(ui).clicked();
    if apply {
        let order = app
            .state
            .workbench
            .visualization_studio
            .draft_pane_order
            .clone();
        let result = app.state.workbench.visualization_studio.transact(|studio| {
            studio.panes.sort_by_key(|pane| {
                order
                    .iter()
                    .position(|pane_id| *pane_id == pane.id)
                    .unwrap_or(usize::MAX)
            });
            Ok(())
        });
        report_visualization_commit(app, result);
    }
    apply
}

fn link_groups_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · SYNCHRONIZED NAVIGATION",
        "Define which panes share X ranges and cursor positions.",
    );
    let active_pane = app.state.workbench.visualization_studio.active_pane;
    let Some(pane_id) = active_pane else {
        empty_note(ui, "Select a pane before editing link groups.");
        return Button::new("Close").show(ui).clicked();
    };
    if app.state.workbench.visualization_studio.draft_link_pane != Some(pane_id) {
        let draft = app
            .state
            .workbench
            .visualization_studio
            .panes
            .iter()
            .find(|pane| pane.id == pane_id)
            .map(|pane| {
                (
                    pane.x_link.unwrap_or_default(),
                    pane.cursor_group.unwrap_or_default(),
                )
            });
        if let Some((x_link, cursor_group)) = draft {
            app.state.workbench.visualization_studio.draft_link_pane = Some(pane_id);
            app.state.workbench.visualization_studio.draft_x_link = x_link;
            app.state.workbench.visualization_studio.draft_cursor_group = cursor_group;
        }
    }
    ui.horizontal(|ui| {
        ui.label("X range group");
        ui.add(
            egui::DragValue::new(&mut app.state.workbench.visualization_studio.draft_x_link)
                .range(0..=999),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Cursor group");
        ui.add(
            egui::DragValue::new(&mut app.state.workbench.visualization_studio.draft_cursor_group)
                .range(0..=999),
        );
    });
    let apply = Button::new("Save link groups").accent().show(ui).clicked();
    if apply {
        let x_link = app.state.workbench.visualization_studio.draft_x_link;
        let cursor_group = app.state.workbench.visualization_studio.draft_cursor_group;
        let result = app.state.workbench.visualization_studio.transact(|studio| {
            let pane = studio
                .panes
                .iter_mut()
                .find(|pane| pane.id == pane_id)
                .ok_or_else(|| "The selected visualization pane no longer exists".to_owned())?;
            pane.x_link = (x_link != 0).then_some(x_link);
            pane.cursor_group = (cursor_group != 0).then_some(cursor_group);
            studio.applied_link_pane = None;
            Ok(())
        });
        report_visualization_commit(app, result);
    }
    apply
}

fn page_editor_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "REPORTING · DOCUMENT COMPOSITION",
        "Compose versioned pages from linked plots and immutable result evidence.",
    );
    let Some(pane_id) = app.state.workbench.visualization_studio.active_pane else {
        empty_note(ui, "Select a pane before editing its page.");
        return Button::new("Close").show(ui).clicked();
    };
    if app.state.workbench.visualization_studio.draft_page_pane != Some(pane_id) {
        let page = app
            .state
            .workbench
            .visualization_studio
            .panes
            .iter()
            .find(|pane| pane.id == pane_id)
            .map(|pane| pane.page.clone())
            .unwrap_or_default();
        app.state.workbench.visualization_studio.draft_page_pane = Some(pane_id);
        app.state.workbench.visualization_studio.draft_page = page;
    }
    ui.label("Template");
    egui::ComboBox::from_id_salt("report.page.template")
        .selected_text(
            app.state
                .workbench
                .visualization_studio
                .draft_report_template
                .clone(),
        )
        .show_ui(ui, |ui| {
            for template in REPORT_PAGE_TEMPLATES {
                ui.selectable_value(
                    &mut app
                        .state
                        .workbench
                        .visualization_studio
                        .draft_report_template,
                    template.to_owned(),
                    template,
                );
            }
        });
    ui.label("Page");
    ui.text_edit_singleline(&mut app.state.workbench.visualization_studio.draft_page);
    ui.label("Update policy");
    ui.radio_value(
        &mut app.state.workbench.visualization_studio.draft_report_freeze,
        false,
        "Refresh linked figures automatically",
    );
    ui.radio_value(
        &mut app.state.workbench.visualization_studio.draft_report_freeze,
        true,
        "Freeze selected figure revision",
    );
    let page = app
        .state
        .workbench
        .visualization_studio
        .draft_page
        .trim()
        .to_owned();
    let template = app
        .state
        .workbench
        .visualization_studio
        .draft_report_template
        .clone();
    let freeze = app.state.workbench.visualization_studio.draft_report_freeze;
    let valid = !page.is_empty()
        && page.len() <= MAX_REPORT_PAGE_TITLE_BYTES
        && !page.chars().any(char::is_control)
        && REPORT_PAGE_TEMPLATES.contains(&template.as_str());
    let apply = ui
        .add_enabled(valid, egui::Button::new("Save report document"))
        .clicked();
    if apply {
        let result = app.state.workbench.visualization_studio.transact(|studio| {
            let pane = studio
                .panes
                .iter_mut()
                .find(|pane| pane.id == pane_id)
                .ok_or_else(|| "The selected visualization pane no longer exists".to_owned())?;
            pane.page = page;
            let policy_revision = studio
                .report_page_policies
                .get(&pane.page)
                .map(|policy| {
                    policy
                        .revision
                        .checked_add(1)
                        .ok_or_else(|| "Report page revision space is exhausted".to_owned())
                })
                .transpose()?
                .unwrap_or(1);
            studio.report_page_policies.insert(
                pane.page.clone(),
                VisualizationReportPagePolicy {
                    template,
                    update_policy: if freeze {
                        PageUpdatePolicy::FreezeFigureRevision
                    } else {
                        PageUpdatePolicy::RefreshLinkedFigures
                    },
                    revision: policy_revision,
                },
            );
            Ok(())
        });
        report_visualization_commit(app, result);
    }
    apply
}

fn measurement_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · SCALAR EXPRESSION",
        "Evaluate and retain a finite scalar measurement against the exact selected analysis.",
    );
    ui.label("Expression");
    ui.text_edit_singleline(&mut app.state.workbench.visualization_studio.draft_measurement);
    property_row(
        ui,
        "Scope",
        "Current pane · active analysis · immutable source inputs",
    );
    let definition = app
        .state
        .workbench
        .visualization_studio
        .draft_measurement
        .trim()
        .to_owned();
    let evaluation = evaluate_scalar_measurement(&app.state, &definition);
    match &evaluation {
        Ok((_, _, value)) => property_row(ui, "Validated value", &format!("{value:.17e}")),
        Err(error) if !definition.is_empty() => {
            ui.label(
                RichText::new(error)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.err),
            );
        }
        Err(_) => property_row(
            ui,
            "Validation",
            "Enter a scalar expression such as rms(V(out))",
        ),
    }
    let valid = evaluation.is_ok();
    let add = ui
        .add_enabled(valid, egui::Button::new("Create measurement"))
        .clicked();
    if add {
        let (dataset_id, analysis_sequence, value) =
            evaluation.expect("enabled measurement has a validated scalar result");
        let expression = definition;
        let result = app.state.workbench.visualization_studio.transact(|studio| {
            let id = studio.allocate_identity().ok_or_else(|| {
                "Visualization measurement identity space is exhausted".to_owned()
            })?;
            studio.measurements.push(VisualizationMeasurement {
                id,
                dataset_id,
                analysis_sequence,
                expression,
                value,
            });
            Ok(())
        });
        if report_visualization_commit(app, result) {
            app.state
                .workbench
                .visualization_studio
                .draft_measurement
                .clear();
        }
    }
    add
}

pub(super) fn evaluate_scalar_measurement(
    state: &AppState,
    expression: &str,
) -> Result<(DatasetId, u64, f64), String> {
    if expression.trim().is_empty() {
        return Err("A measurement expression is required".to_owned());
    }
    let run = state
        .simulation
        .active_run()
        .ok_or_else(|| "A retained result dataset is required".to_owned())?;
    let analysis = state
        .simulation
        .active_analysis()
        .ok_or_else(|| "A retained analysis must be selected".to_owned())?;
    let parsed = calculator::parser::try_parse(expression)
        .map_err(|error| format!("Parse error: {error}"))?;
    let context = calculator::WaveformsContext::new(&analysis.waveforms);
    let value = match calculator::evaluator::evaluate(&parsed, &context)
        .map_err(|error| error.to_string())?
    {
        calculator::CalcValue::Scalar(value) => value,
        calculator::CalcValue::Waveform(_, _) => {
            return Err(
                "The expression produces a trace; reduce it with avg(), rms(), or another scalar function"
                    .to_owned(),
            );
        }
    };
    if !value.is_finite() {
        return Err("The measurement result is not finite".to_owned());
    }
    Ok((run.dataset_id, analysis.id, value))
}

fn annotation_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · REVIEW ANCHOR",
        "Anchor a review note to an immutable dataset and exact source coordinate.",
    );
    ui.label("Annotation text");
    ui.text_edit_multiline(&mut app.state.workbench.visualization_studio.draft_annotation);
    let anchor = source_midpoint(&app.state);
    property_row(
        ui,
        "Anchor",
        &anchor.as_ref().map_or_else(
            || "No exact source row".to_owned(),
            |(dataset, analysis, waveform, index, x, _)| {
                format!(
                    "{} · analysis {} · {}[{}] · {:.17e}",
                    short_dataset(*dataset),
                    analysis,
                    waveform,
                    index,
                    x
                )
            },
        ),
    );
    let valid = anchor.is_some()
        && !app
            .state
            .workbench
            .visualization_studio
            .draft_annotation
            .trim()
            .is_empty();
    let add = ui
        .add_enabled(valid, egui::Button::new("Create annotation"))
        .clicked();
    if add {
        let (dataset_id, analysis_sequence, _, _, x, _) =
            anchor.expect("enabled annotation has an exact anchor");
        let text = app
            .state
            .workbench
            .visualization_studio
            .draft_annotation
            .trim()
            .to_owned();
        let result = app.state.workbench.visualization_studio.transact(|studio| {
            let id = studio
                .allocate_identity()
                .ok_or_else(|| "Visualization annotation identity space is exhausted".to_owned())?;
            studio.annotations.push(VisualizationAnnotation {
                id,
                dataset_id,
                analysis_sequence,
                x,
                text,
            });
            Ok(())
        });
        if report_visualization_commit(app, result) {
            app.state
                .workbench
                .visualization_studio
                .draft_annotation
                .clear();
        }
    }
    add
}

fn comparison_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · IMMUTABLE COMPARISON",
        "Create an executable receipt only after every numerical policy is explicit.",
    );
    let active_dataset = app.state.simulation.active_run().map(|run| run.dataset_id);
    ui.label("Comparison dataset");
    let selected_label = app
        .state
        .workbench
        .visualization_studio
        .draft_comparison_dataset
        .and_then(|dataset| {
            app.state
                .simulation
                .runs
                .iter()
                .find(|run| run.dataset_id == dataset)
                .map(|run| run.label.clone())
        })
        .unwrap_or_else(|| "Select immutable dataset".to_owned());
    egui::ComboBox::from_id_salt("visualization.comparison.dataset")
        .selected_text(selected_label)
        .show_ui(ui, |ui| {
            for run in &app.state.simulation.runs {
                if Some(run.dataset_id) == active_dataset {
                    continue;
                }
                ui.selectable_value(
                    &mut app
                        .state
                        .workbench
                        .visualization_studio
                        .draft_comparison_dataset,
                    Some(run.dataset_id),
                    format!("{} · {}", run.label, short_dataset(run.dataset_id)),
                );
            }
        });
    for (label, value) in [
        ("Alignment", "Exact coordinate rows"),
        ("Units", "Require identical units"),
        ("Interpolation", "None · exact only"),
        ("Resampling", "None · retain source grids"),
        ("Extrapolation", "Forbidden"),
        ("Precision", "Source f64 · no rounding"),
    ] {
        property_row(ui, label, value);
    }
    ui.horizontal(|ui| {
        ui.label("Absolute tolerance");
        ui.add(
            egui::DragValue::new(
                &mut app
                    .state
                    .workbench
                    .visualization_studio
                    .draft_comparison_absolute_tolerance,
            )
            .speed(1.0e-6)
            .range(0.0..=f64::MAX),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Relative tolerance");
        ui.add(
            egui::DragValue::new(
                &mut app
                    .state
                    .workbench
                    .visualization_studio
                    .draft_comparison_relative_tolerance,
            )
            .speed(1.0e-6)
            .range(0.0..=f64::MAX),
        );
    });
    let studio = &app.state.workbench.visualization_studio;
    let valid = active_dataset.is_some()
        && studio.draft_comparison_dataset.is_some()
        && studio.draft_comparison_absolute_tolerance.is_finite()
        && studio.draft_comparison_absolute_tolerance >= 0.0
        && studio.draft_comparison_relative_tolerance.is_finite()
        && studio.draft_comparison_relative_tolerance >= 0.0;
    let create = Button::new("Create comparison receipt")
        .accent()
        .enabled(valid)
        .show(ui)
        .clicked();
    if !create {
        return false;
    }
    match execute_comparison_draft(app) {
        Ok(receipt) => {
            let rows = receipt.rows_compared;
            let disposition = receipt.disposition;
            let result = app
                .state
                .workbench
                .visualization_studio
                .transact(move |studio| {
                    studio.comparison_receipts.push(receipt);
                    Ok(())
                });
            if report_visualization_commit(app, result) {
                app.state.push_user_message(ConsoleMessage::info(format!(
                    "Recorded exact comparison receipt for {rows} row(s): {disposition:?}."
                )));
                return true;
            }
            false
        }
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::error(error));
            false
        }
    }
}

fn matching_comparison_analysis<'a>(
    active: &AnalysisResult,
    run: &'a SimulationRun,
) -> Option<&'a AnalysisResult> {
    if let Some(source_id) = active
        .provenance
        .as_ref()
        .map(|provenance| provenance.authored_source_instance_id())
    {
        return run
            .find_analysis_by_source_instance(source_id)
            .filter(|analysis| analysis.analysis_type == active.analysis_type);
    }
    let mut exact = run.analyses.iter().filter(|analysis| {
        analysis.provenance.is_none()
            && analysis.analysis_type == active.analysis_type
            && analysis.label == active.label
    });
    let candidate = exact.next()?;
    exact.next().is_none().then_some(candidate)
}

fn comparison_axis_unit(analysis_type: AnalysisType) -> &'static str {
    match analysis_type {
        AnalysisType::Ac | AnalysisType::Noise | AnalysisType::Pnoise => "Hz",
        AnalysisType::Transient | AnalysisType::Soa => "s",
        AnalysisType::DcSweep => "V",
        _ => "",
    }
}

fn comparison_source_dataset(
    run: &SimulationRun,
    analysis: &AnalysisResult,
    signal_names: &[String],
) -> Result<SourceDataset, String> {
    let reference = signal_names
        .first()
        .and_then(|name| {
            analysis
                .waveforms
                .iter()
                .find(|waveform| waveform.name == *name)
        })
        .ok_or_else(|| "No common waveform axis is available for comparison.".to_owned())?;
    let waveforms = signal_names
        .iter()
        .map(|name| {
            analysis
                .waveforms
                .iter()
                .find(|waveform| waveform.name == *name)
                .ok_or_else(|| format!("Waveform '{name}' is unavailable."))
        })
        .collect::<Result<Vec<_>, _>>()?;
    for waveform in &waveforms {
        if waveform.x.len() != reference.x.len()
            || waveform.y.len() != reference.x.len()
            || !waveform
                .x
                .iter()
                .zip(reference.x.iter())
                .all(|(left, right)| left.to_bits() == right.to_bits())
        {
            return Err(format!(
                "Waveform '{}' does not share the exact comparison coordinate axis.",
                waveform.name
            ));
        }
    }
    let x_unit = comparison_axis_unit(analysis.analysis_type);
    let mut columns = vec![
        SourceColumn::new(
            "x",
            "X coordinate",
            ValueType::Real,
            ColumnRole::Coordinate,
            (!x_unit.is_empty()).then_some(x_unit.to_owned()),
        )
        .map_err(|error| error.to_string())?,
    ];
    for (index, waveform) in waveforms.iter().enumerate() {
        columns.push(
            SourceColumn::new(
                format!("signal:{index}"),
                waveform.name.clone(),
                ValueType::Real,
                ColumnRole::Signal,
                // WaveformData does not yet retain a trustworthy per-signal unit.
                // Leaving the unit unknown is exact; claiming volts would silently
                // mislabel currents, powers, and dimensionless solver outputs.
                None,
            )
            .map_err(|error| error.to_string())?,
        );
    }
    let rows = reference
        .x
        .iter()
        .enumerate()
        .map(|(row, x)| {
            let mut values = vec![TypedValue::Real(*x)];
            values.extend(
                waveforms
                    .iter()
                    .map(|waveform| TypedValue::Real(waveform.y[row])),
            );
            SourceRow::new(values)
        })
        .collect();
    SourceDataset::new(
        DatasetBinding::new(run.dataset_id, run.dataset_content_digest()),
        columns,
        rows,
    )
    .map_err(|error| error.to_string())
}

pub(super) fn execute_comparison_draft(app: &RSpiceApp) -> Result<ComparisonReceipt, String> {
    let active_run = app
        .state
        .simulation
        .active_run()
        .ok_or_else(|| "No candidate dataset is selected.".to_owned())?;
    let active_analysis = app
        .state
        .simulation
        .active_analysis()
        .ok_or_else(|| "No candidate analysis is selected.".to_owned())?;
    let baseline_id = app
        .state
        .workbench
        .visualization_studio
        .draft_comparison_dataset
        .ok_or_else(|| "Select an immutable comparison dataset.".to_owned())?;
    let baseline_run = app
        .state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == baseline_id)
        .ok_or_else(|| "The comparison dataset is no longer retained.".to_owned())?;
    let baseline_analysis = matching_comparison_analysis(active_analysis, baseline_run)
        .ok_or_else(|| "The comparison dataset has no unambiguous matching analysis.".to_owned())?;
    let reference_axis = active_analysis
        .waveforms
        .first()
        .map(|waveform| waveform.x.as_slice())
        .ok_or_else(|| "The candidate analysis has no waveform quantities.".to_owned())?;
    let same_axis = |candidate: &[f64]| {
        candidate.len() == reference_axis.len()
            && candidate
                .iter()
                .zip(reference_axis)
                .all(|(left, right)| left.to_bits() == right.to_bits())
    };
    let mut signal_names = active_analysis
        .waveforms
        .iter()
        .filter(|waveform| {
            same_axis(&waveform.x)
                && baseline_analysis
                    .waveforms
                    .iter()
                    .any(|candidate| candidate.name == waveform.name && same_axis(&candidate.x))
        })
        .map(|waveform| waveform.name.clone())
        .collect::<Vec<_>>();
    signal_names.sort();
    signal_names.dedup();
    if signal_names.is_empty() {
        return Err("The selected analyses have no common waveform quantities.".to_owned());
    }
    let baseline = comparison_source_dataset(baseline_run, baseline_analysis, &signal_names)?;
    let candidate = comparison_source_dataset(active_run, active_analysis, &signal_names)?;
    let signal_keys = (0..signal_names.len())
        .map(|index| format!("signal:{index}"))
        .collect();
    let tolerance = NumericTolerance::new(
        app.state
            .workbench
            .visualization_studio
            .draft_comparison_absolute_tolerance,
        app.state
            .workbench
            .visualization_studio
            .draft_comparison_relative_tolerance,
    )
    .map_err(|error| error.to_string())?;
    let request = ComparisonRequest {
        baseline: baseline.binding(),
        candidate: candidate.binding(),
        signal_keys,
        policy: ComparisonPolicy {
            row_alignment: RowAlignmentPolicy::RequireIdentical,
            tolerance,
            require_identical_units: true,
            execution: ComparisonExecutionContract::default(),
        },
    };
    compare_source_datasets(&baseline, &candidate, &request).map_err(|error| error.to_string())
}

fn family_slice_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · N-DIMENSIONAL DATA",
        "Choose typed dimensions from the active immutable result family.",
    );
    let manifest = active_family_manifest(app);
    let valid = match manifest.as_ref() {
        Ok(manifest) => {
            family_dimension_combo(
                ui,
                "family.slice.x",
                "X dimension",
                &mut app
                    .state
                    .workbench
                    .visualization_studio
                    .draft_family_x_dimension,
                manifest,
                true,
                false,
            );
            family_dimension_combo(
                ui,
                "family.slice.family",
                "Family dimension",
                &mut app
                    .state
                    .workbench
                    .visualization_studio
                    .draft_family_dimension,
                manifest,
                false,
                false,
            );
            ui.label("Filter");
            ui.text_edit_singleline(&mut app.state.workbench.visualization_studio.family_query);
            family_policy_preview_is_valid(ui, app, manifest)
        }
        Err(error) => {
            empty_note(ui, error);
            false
        }
    };
    let apply = Button::new("Apply slice and pivot")
        .accent()
        .enabled(valid)
        .show(ui)
        .clicked();
    if apply {
        apply_family_policy_draft(app);
    }
    apply
}

fn family_encoding_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · ACCESSIBLE TRACE FAMILIES",
        "Configure redundant visual encoding supported by the retained renderer.",
    );
    let manifest = active_family_manifest(app);
    let valid = match manifest.as_ref() {
        Ok(manifest) => {
            family_dimension_combo(
                ui,
                "family.encoding.color",
                "Color",
                &mut app
                    .state
                    .workbench
                    .visualization_studio
                    .draft_family_color_dimension,
                manifest,
                false,
                false,
            );
            family_dimension_combo(
                ui,
                "family.encoding.dash",
                "Dash",
                &mut app
                    .state
                    .workbench
                    .visualization_studio
                    .draft_family_dash_dimension,
                manifest,
                false,
                true,
            );
            family_dimension_combo(
                ui,
                "family.encoding.marker",
                "Marker",
                &mut app
                    .state
                    .workbench
                    .visualization_studio
                    .draft_family_marker_dimension,
                manifest,
                false,
                true,
            );
            family_policy_preview_is_valid(ui, app, manifest)
        }
        Err(error) => {
            empty_note(ui, error);
            false
        }
    };
    let apply = Button::new("Apply encoding")
        .accent()
        .enabled(valid)
        .show(ui)
        .clicked();
    if apply {
        apply_family_policy_draft(app);
    }
    apply
}

fn family_filter_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · DATA QUERY",
        "Filter exact typed family points without changing immutable solver output.",
    );
    ui.label("Expression");
    ui.text_edit_singleline(&mut app.state.workbench.visualization_studio.family_query);
    ui.label("Missing points");
    ui.radio_value(
        &mut app
            .state
            .workbench
            .visualization_studio
            .draft_family_exclude_missing,
        false,
        "Preserve as not-run",
    );
    ui.radio_value(
        &mut app
            .state
            .workbench
            .visualization_studio
            .draft_family_exclude_missing,
        true,
        "Exclude with omission record",
    );
    let manifest = active_family_manifest(app);
    let valid = match manifest.as_ref() {
        Ok(manifest) => family_policy_preview_is_valid(ui, app, manifest),
        Err(error) => {
            empty_note(ui, error);
            false
        }
    };
    let apply = Button::new("Apply filter")
        .accent()
        .enabled(valid)
        .show(ui)
        .clicked();
    if apply {
        apply_family_policy_draft(app);
    }
    apply
}

fn active_family_manifest(app: &RSpiceApp) -> Result<FamilyManifest, String> {
    let analysis = app
        .state
        .simulation
        .active_analysis()
        .ok_or_else(|| "Select an analysis with retained family data.".to_owned())?;
    FamilyManifest::from_analysis(analysis)?
        .ok_or_else(|| "The active analysis has no retained family manifest.".to_owned())
}

pub(super) fn active_family_sample_selection(
    app: &RSpiceApp,
) -> Result<Option<SourceSampleSelection>, String> {
    let studio = &app.state.workbench.visualization_studio;
    let Some(pane) = studio.active_pane() else {
        return Ok(None);
    };
    let Some(policy) = studio.family_policies.get(&pane.id) else {
        return Ok(None);
    };
    if pane.viewer != ResultViewer::Waves {
        return Err(
            "This family policy requires the waveform renderer; choose Waveform before applying it."
                .to_owned(),
        );
    }
    let run = app
        .state
        .simulation
        .active_run()
        .ok_or_else(|| "The pane's immutable dataset is unavailable.".to_owned())?;
    let analysis = app
        .state
        .simulation
        .active_analysis()
        .ok_or_else(|| "The pane's bound analysis is unavailable.".to_owned())?;
    if run.dataset_id != pane.dataset_id || analysis.id != pane.analysis_sequence {
        return Err(
            "The active renderer binding does not match the family policy pane.".to_owned(),
        );
    }
    let manifest = FamilyManifest::from_analysis(analysis)?
        .ok_or_else(|| "The pane's source no longer contains family metadata.".to_owned())?;
    let indices = manifest.matching_source_indices_for_filter(policy.filter.as_ref())?;
    for waveform in &analysis.waveforms {
        manifest.compatible_waveform_len(waveform.x.len())?;
        if waveform.x.len() != waveform.y.len() {
            return Err(format!(
                "Waveform '{}' has mismatched X and Y sample counts.",
                waveform.name
            ));
        }
    }
    SourceSampleSelection::new(run.dataset_id, analysis.id, indices)
        .and_then(|selection| selection.with_family_presentation(&manifest, policy))
        .map(Some)
}

fn family_dimension_combo(
    ui: &mut Ui,
    id: &'static str,
    label: &str,
    selected: &mut String,
    manifest: &FamilyManifest,
    numeric_only: bool,
    allow_none: bool,
) {
    ui.horizontal(|ui| {
        ui.label(label);
        let selected_text = if selected.is_empty() {
            "none".to_owned()
        } else {
            selected.clone()
        };
        egui::ComboBox::from_id_salt(id)
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                if allow_none {
                    ui.selectable_value(selected, String::new(), "none");
                }
                for dimension in &manifest.dimensions {
                    if dimension.id == "status"
                        || (numeric_only
                            && !matches!(
                                dimension.kind,
                                FamilyValueKind::Number | FamilyValueKind::Integer
                            ))
                    {
                        continue;
                    }
                    let display = dimension.unit.as_ref().map_or_else(
                        || dimension.label.clone(),
                        |unit| format!("{} ({unit})", dimension.label),
                    );
                    ui.selectable_value(selected, dimension.id.clone(), display);
                }
            });
    });
}

fn family_preview(
    ui: &mut Ui,
    app: &RSpiceApp,
    manifest: &FamilyManifest,
) -> Result<Vec<usize>, String> {
    let query = &app.state.workbench.visualization_studio.family_query;
    let indices = match manifest.matching_source_indices(query) {
        Ok(indices) => indices,
        Err(error) => {
            empty_note(ui, &error);
            return Err(error);
        }
    };
    let trace_count = app
        .state
        .simulation
        .active_analysis()
        .map_or(0, |analysis| analysis.waveforms.len());
    let selected_samples = trace_count.saturating_mul(indices.len());
    let omission = if manifest.omitted_points == 0 {
        String::new()
    } else {
        format!(
            " · {} unavailable point(s) recorded",
            manifest.omitted_points
        )
    };
    ui.label(format!(
        "{} of {} retained points · {trace_count} traces · {selected_samples} selected samples{omission}",
        indices.len(),
        manifest.points.len()
    ));
    Ok(indices)
}

fn family_policy_preview_is_valid(ui: &mut Ui, app: &RSpiceApp, manifest: &FamilyManifest) -> bool {
    let result = (|| {
        let indices = family_preview(ui, app, manifest)?;
        if indices.is_empty() {
            return Err("The current filter selects no retained family points.".to_owned());
        }
        let policy = build_family_policy_draft(app, manifest)?;
        SourceSampleSelection::new(DatasetId::new(), 0, indices)?
            .with_family_presentation(manifest, &policy)?;
        Ok::<_, String>(())
    })();
    if let Err(error) = result {
        empty_note(
            ui,
            &format!(
                "This draft cannot be applied to the waveform renderer: {error} Choose an X dimension that is finite, losslessly numeric, and strictly increasing within every selected family group."
            ),
        );
        false
    } else {
        true
    }
}

fn document_family_dimension(
    manifest: &FamilyManifest,
    id: &str,
) -> Result<DocumentFamilyDimension, String> {
    let dimension = manifest
        .dimension(id)
        .ok_or_else(|| format!("Unknown family dimension '{id}'."))?;
    DocumentFamilyDimension::new(
        dimension.id.clone(),
        match dimension.kind {
            FamilyValueKind::Number => ValueType::Real,
            FamilyValueKind::Integer => ValueType::Integer,
            FamilyValueKind::Text | FamilyValueKind::Status => ValueType::Text,
        },
    )
    .map_err(|error| error.to_string())
}

fn build_family_policy_draft(
    app: &RSpiceApp,
    manifest: &FamilyManifest,
) -> Result<FamilyPresentationPolicy, String> {
    let studio = &app.state.workbench.visualization_studio;
    let x_dimension = document_family_dimension(manifest, &studio.draft_family_x_dimension)?;
    let mut family_dimension_ids = vec![studio.draft_family_dimension.clone()];
    for dimension in [
        &studio.draft_family_color_dimension,
        &studio.draft_family_dash_dimension,
        &studio.draft_family_marker_dimension,
    ] {
        if !dimension.is_empty()
            && *dimension != x_dimension.key
            && !family_dimension_ids.contains(dimension)
        {
            family_dimension_ids.push(dimension.clone());
        }
    }
    if studio.family_query.contains("status")
        && x_dimension.key != "status"
        && !family_dimension_ids.iter().any(|id| id == "status")
    {
        family_dimension_ids.push("status".to_owned());
    }
    family_dimension_ids.retain(|id| !id.is_empty() && *id != x_dimension.key);
    let family_dimensions = family_dimension_ids
        .iter()
        .map(|id| document_family_dimension(manifest, id))
        .collect::<Result<Vec<_>, _>>()?;
    if family_dimensions.is_empty() {
        return Err("Select at least one family dimension distinct from X.".to_owned());
    }

    let mut encodings = Vec::new();
    if !studio.draft_family_color_dimension.is_empty() {
        encodings.push(FamilyEncodingMap::Color {
            dimension: document_family_dimension(manifest, &studio.draft_family_color_dimension)?,
            palette: AccessibleColorPalette::OkabeItoCategorical,
        });
    }
    if !studio.draft_family_dash_dimension.is_empty() {
        encodings.push(FamilyEncodingMap::Dash {
            dimension: document_family_dimension(manifest, &studio.draft_family_dash_dimension)?,
        });
    }
    if !studio.draft_family_marker_dimension.is_empty() {
        encodings.push(FamilyEncodingMap::Marker {
            dimension: document_family_dimension(manifest, &studio.draft_family_marker_dimension)?,
        });
    }
    let encoded: HashSet<_> = encodings
        .iter()
        .map(|encoding| encoding.dimension().key.as_str())
        .collect();
    let unencoded: Vec<_> = family_dimensions
        .iter()
        .filter(|dimension| !encoded.contains(dimension.key.as_str()))
        .cloned()
        .collect();
    if unencoded.len() > 1 {
        return Err(format!(
            "Dimensions {} require explicit visual encodings.",
            unencoded
                .iter()
                .map(|dimension| dimension.key.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if let Some(dimension) = unencoded.into_iter().next() {
        encodings.push(FamilyEncodingMap::Label {
            dimension,
            prefix: None,
        });
    }
    let color_dimensions: Vec<_> = encodings
        .iter()
        .filter_map(|encoding| match encoding {
            FamilyEncodingMap::Color { dimension, .. } => Some(dimension.clone()),
            _ => None,
        })
        .collect();
    for color_dimension in color_dimensions {
        if !encodings.iter().any(|encoding| {
            encoding.dimension() == &color_dimension
                && matches!(
                    encoding,
                    FamilyEncodingMap::Dash { .. }
                        | FamilyEncodingMap::Marker { .. }
                        | FamilyEncodingMap::Label { .. }
                )
        }) {
            if encodings
                .iter()
                .any(|encoding| matches!(encoding, FamilyEncodingMap::Label { .. }))
            {
                return Err(format!(
                    "Color dimension '{}' requires a matching dash or marker cue.",
                    color_dimension.key
                ));
            }
            encodings.push(FamilyEncodingMap::Label {
                dimension: color_dimension,
                prefix: None,
            });
        }
    }

    let policy = FamilyPresentationPolicy {
        x_dimension: FamilyXDimension {
            dimension: x_dimension,
            ordering: FamilyXOrdering::Source,
        },
        family_dimensions,
        facet_layout: None,
        aggregation: FamilyAggregationPolicy {
            method: FamilyAggregationMethod::None,
            over_dimensions: Vec::new(),
        },
        filter: manifest.compile_filter(&studio.family_query)?,
        missing_points: if studio.draft_family_exclude_missing {
            MissingPointPolicy::ExcludeWithOmissionRecord
        } else {
            MissingPointPolicy::PreserveAsNotRun
        },
        encodings,
    };
    policy.validate().map_err(|error| error.to_string())?;
    Ok(policy)
}

fn apply_family_policy_draft(app: &mut RSpiceApp) {
    let result = (|| {
        let manifest = active_family_manifest(app)?;
        let analysis = app
            .state
            .simulation
            .active_analysis()
            .ok_or_else(|| "No active analysis is selected.".to_owned())?;
        for waveform in &analysis.waveforms {
            manifest.compatible_waveform_len(waveform.x.len())?;
            if waveform.x.len() != waveform.y.len() {
                return Err(format!(
                    "Waveform '{}' has mismatched X and Y sample counts.",
                    waveform.name
                ));
            }
        }
        let policy = build_family_policy_draft(app, &manifest)?;
        let indices = manifest.matching_source_indices_for_filter(policy.filter.as_ref())?;
        if indices.is_empty() {
            return Err("The current filter selects no retained family points.".to_owned());
        }
        SourceSampleSelection::new(DatasetId::new(), analysis.id, indices.clone())?
            .with_family_presentation(&manifest, &policy)?;
        let pane_id = app
            .state
            .workbench
            .visualization_studio
            .active_pane
            .ok_or_else(|| "No visualization pane is selected.".to_owned())?;
        if app
            .state
            .workbench
            .visualization_studio
            .active_pane()
            .is_none_or(|pane| pane.viewer != ResultViewer::Waves)
        {
            return Err(
                "Choose the Waveform viewer before applying a sample-level family policy."
                    .to_owned(),
            );
        }
        app.state
            .workbench
            .visualization_studio
            .transact(move |studio| {
                studio.family_policies.insert(pane_id, policy);
                Ok(())
            })?;
        Ok::<_, String>(indices.len())
    })();
    match result {
        Ok(count) => app.state.push_user_message(ConsoleMessage::info(format!(
            "Applied exact family presentation to {count} retained point(s)."
        ))),
        Err(error) => app.state.push_user_message(ConsoleMessage::error(error)),
    }
}

fn export_preset_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · PUBLICATION PROFILE",
        "Store page, font, color, vector, raster, metadata, and naming defaults without changing the plot document.",
    );
    ui.label("Name");
    ui.text_edit_singleline(
        &mut app
            .state
            .workbench
            .visualization_studio
            .draft_export_preset_name,
    );
    property_row(ui, "Formats", "PDF/A + SVG + 2× PNG");
    ui.label("Scope");
    let scope = app
        .state
        .workbench
        .visualization_studio
        .draft_export_preset_scope
        .get_or_insert(PlotExportPresetScope::Project);
    ui.radio_value(scope, PlotExportPresetScope::Project, "Project team");
    ui.radio_value(scope, PlotExportPresetScope::Personal, "Personal");
    ui.add_enabled_ui(false, |ui| {
        ui.radio_value(
            scope,
            PlotExportPresetScope::Organization,
            "Organization template",
        )
    })
    .response
    .on_hover_text("Connect an organization authority before saving organization templates.");
    ui.weak("Organization templates are unavailable while no organization authority is connected.");

    let selected_scope = *scope;
    let name = app
        .state
        .workbench
        .visualization_studio
        .draft_export_preset_name
        .trim();
    let duplicate_name = match selected_scope {
        PlotExportPresetScope::Project => app
            .state
            .workspace
            .plot_export_presets
            .active_presets()
            .into_iter()
            .any(|preset| preset.definition.name.eq_ignore_ascii_case(name)),
        PlotExportPresetScope::Personal => app
            .state
            .ui
            .preferences
            .personal_plot_export_presets()
            .is_some_and(|catalog| {
                catalog
                    .active_presets()
                    .into_iter()
                    .any(|preset| preset.definition.name.eq_ignore_ascii_case(name))
            }),
        PlotExportPresetScope::Organization => false,
    };
    if duplicate_name {
        ui.weak("A preset with this name already exists in the selected scope.");
    }
    let valid = !name.is_empty()
        && name.len() <= 96
        && !name.chars().any(char::is_control)
        && selected_scope != PlotExportPresetScope::Organization
        && !duplicate_name;
    let save = Button::new("Save export preset")
        .accent()
        .enabled(valid)
        .show(ui)
        .clicked();
    if !save {
        return false;
    }

    let scope = app
        .state
        .workbench
        .visualization_studio
        .draft_export_preset_scope
        .unwrap_or(PlotExportPresetScope::Project);
    let definition = PlotExportPresetDefinition {
        name: name.to_owned(),
        formats: vec![
            PlotExportFormat::PdfA {
                conformance: PdfAConformance::PdfA2b,
            },
            PlotExportFormat::Svg,
            PlotExportFormat::RasterPng { scale_percent: 200 },
        ],
        page: PageGeometry {
            size: ExportPageSize::A4,
            orientation: PageOrientation::Portrait,
            margins: PageMargins {
                top_micrometers: 12_000,
                right_micrometers: 12_000,
                bottom_micrometers: 12_000,
                left_micrometers: 12_000,
            },
        },
        fonts: FontPolicy {
            primary_family: "Inter".to_owned(),
            fallback_families: vec!["DejaVu Sans".to_owned()],
            embedding: FontEmbeddingPolicy::EmbedSubset,
        },
        color_profile: ColorProfile::Srgb,
        background: ExportBackground::White,
        rendering: VectorRasterPolicy {
            vector_handling: VectorHandling::PreserveNative,
            raster_dpi: 300,
            raster_resampling: RasterResampling::Lanczos,
            antialias: true,
        },
        metadata: MetadataProvenancePolicy {
            include_document_metadata: true,
            include_dataset_manifest: true,
            include_source_digests: true,
            include_revision_receipts: true,
            include_export_timestamp: true,
        },
        naming_template: match DeterministicNamingTemplate::new(
            "{document}-{page}-{revision}-{format}",
        ) {
            Ok(template) => template,
            Err(error) => {
                app.state
                    .push_user_message(ConsoleMessage::error(error.to_string()));
                return false;
            }
        },
        scope,
    };
    let timestamp = crate::time_compat::unix_epoch().as_millis();
    let timestamp = u64::try_from(timestamp).unwrap_or(u64::MAX);
    let result = match scope {
        PlotExportPresetScope::Project => {
            let catalog = &mut app.state.workspace.plot_export_presets;
            catalog
                .create_owned(
                    catalog.revision(),
                    PlotExportPresetScope::Project,
                    definition,
                    timestamp,
                )
                .map_err(|error| error.to_string())
        }
        PlotExportPresetScope::Personal => app
            .state
            .ui
            .preferences
            .create_personal_plot_export_preset(definition, timestamp)
            .map_err(|error| error.to_string()),
        PlotExportPresetScope::Organization => {
            Err("organization export presets require a connected organization authority".to_owned())
        }
    };
    match result {
        Ok(receipt) => {
            if scope == PlotExportPresetScope::Project {
                app.state.workspace.project_metadata_dirty = true;
            }
            app.state.push_user_message(ConsoleMessage::info(format!(
                "Saved {scope:?} plot export preset revision {} (receipt {}).",
                receipt.committed_preset_revision.get(),
                receipt.receipt_id
            )));
            true
        }
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::error(error));
            false
        }
    }
}

fn export_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · EXACT DATA OR RENDERED VIEW",
        "Choose a writer backed by the active immutable result dataset or viewer crop.",
    );
    let enabled = app.state.simulation.has_results();
    let mut close = false;
    if ui
        .add_enabled(enabled, egui::Button::new("Export exact engineering data…"))
        .clicked()
    {
        app.state.ui.export_csv_requested = true;
        close = true;
    }
    if ui
        .add_enabled(enabled, egui::Button::new("Export active viewer PNG…"))
        .clicked()
    {
        app.state.ui.export_png_requested = true;
        close = true;
    }
    if !enabled {
        empty_note(
            ui,
            "A completed immutable result is required before export.",
        );
    }
    close
}

fn dock_intro(ui: &mut Ui, eyebrow: &str, description: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(eyebrow)
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.accent),
    );
    ui.label(
        RichText::new(description)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_space(8.0);
}

pub(super) fn dock_action(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    label: &'static str,
    dock: VisualizationDock,
) {
    if Button::new(label).show(ui).clicked() {
        open_dock(app, dock);
    }
}

pub(super) fn separator(ui: &mut Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter()
        .hline(rect.x_range(), rect.center().y, Stroke::new(1.0, color));
}

pub(super) fn paint_top_rule(ui: &Ui, rect: Rect, color: Color32) {
    ui.painter()
        .hline(rect.x_range(), rect.top() + 0.5, Stroke::new(1.0, color));
}

pub(super) fn paint_bottom_rule(ui: &Ui, rect: Rect, color: Color32) {
    ui.painter()
        .hline(rect.x_range(), rect.bottom() - 0.5, Stroke::new(1.0, color));
}

pub(super) fn panel_heading(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel_2)
        .inner_margin(Margin::symmetric(8, 0))
        .show(ui, |ui| {
            ui.set_min_height(PANEL_HEADING_HEIGHT);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(title.to_uppercase())
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text_dim),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(detail)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                    );
                });
            });
        });
}

pub(super) fn table_header(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_faint),
    );
}

pub(super) fn empty_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.label(
                RichText::new(text)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        });
}

pub(super) fn concept_banner(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(12.0);
    Frame::NONE
        .fill(t.color.accent_dim)
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::hover());
                WorkbenchIcon::Info.paint(ui.painter(), icon_rect, t.color.info);
                ui.label(
                    RichText::new(text)
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            });
        });
}

pub(super) fn policy_row(ui: &mut Ui, label: &str, value: &str) {
    table_header(ui, label);
    ui.label(value);
    ui.end_row();
}

pub(super) fn property_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(label).strong());
        ui.label(value);
    });
}

pub(super) fn labeled_combo(
    ui: &mut Ui,
    label: &str,
    selected: &str,
    add_contents: impl FnOnce(&mut Ui),
) {
    ui.vertical(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt(("visualization.combo", label))
            .selected_text(selected)
            .show_ui(ui, add_contents);
    });
}

pub(super) fn numeric_policy(
    ui: &mut Ui,
    label: &str,
    value: &mut u32,
    range: std::ops::RangeInclusive<u32>,
    suffix: &str,
) {
    ui.vertical(|ui| {
        ui.label(label);
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(value).range(range));
            ui.monospace(suffix);
        });
    });
}
