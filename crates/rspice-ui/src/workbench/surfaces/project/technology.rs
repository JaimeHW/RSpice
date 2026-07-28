//! Attaching and migrating a project's technology.
//!
//! Attaching is checkpointed: the project is saved before the binding changes,
//! so a failed or abandoned migration leaves a recoverable state rather than a
//! half-migrated project. A candidate that would invalidate pinned model
//! sources is reported with the sources it would break instead of being
//! offered as a plain choice.

use super::*;

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let layout = ProjectContractLayout::resolve(ui.available_width());
    header_with_actions(
        ui,
        "TECHNOLOGY ATTACHMENT",
        TECHNOLOGY_SURFACE_TITLE,
        "Project-owned authenticated binding for parsed models, process sections, dependency edges, and exact source bytes.",
        |ui, app| {
            if Button::new("Project").show(ui).clicked() {
                Command::ProjectPage(ProjectPage::Dashboard).execute(app);
            }
            if Button::new(TECHNOLOGY_SURFACE_ACTION)
                .accent()
                .show(ui)
                .clicked()
            {
                open_technology_attachment_dialog(app);
            }
        },
        app,
    );
    technology_metric_strip(ui, app, layout);
    technology_resource_table(ui, app);
    technology_migration_strip(ui, app, layout);
}

pub(super) fn technology_metric_strip(ui: &mut Ui, app: &RSpiceApp, layout: ProjectContractLayout) {
    let t = Tokens::get(ui.ctx());
    let binding = app.state.workspace.project.technology_binding();
    let catalog = technology_candidates(app);
    let replacement_count = catalog
        .candidates
        .iter()
        .filter(|candidate| Some(&candidate.binding) != binding)
        .count();
    let validation = binding.map(|binding| {
        app.state
            .model_library_manager
            .validate_attached_technology(Some(binding))
    });
    let (attachment_value, attachment_detail, attachment_color) = match validation {
        Some(Ok(())) => (
            "resolved".to_owned(),
            binding
                .map(ProjectTechnologyBinding::display_label)
                .unwrap_or_default(),
            t.color.ok,
        ),
        Some(Err(_)) => (
            "stale".to_owned(),
            "reattach the authenticated execution catalog".to_owned(),
            t.color.err,
        ),
        None if app.state.workspace.project.technology.is_some() => (
            "legacy label".to_owned(),
            "no exact source contract attached".to_owned(),
            t.color.warn,
        ),
        None => (
            "not attached".to_owned(),
            "simulation fails closed without a binding".to_owned(),
            t.color.warn,
        ),
    };
    let sections = binding
        .map(|binding| binding.process_sections().len())
        .unwrap_or(0);
    let section_detail = binding
        .filter(|binding| !binding.process_sections().is_empty())
        .map(|binding| binding.process_sections().join(" · "))
        .unwrap_or_else(|| "No named process sections".to_owned());
    let source_count = binding
        .map(|binding| binding.source_closure().len())
        .unwrap_or(0);
    let source_detail = binding
        .map(|binding| {
            format!(
                "{} parsed model{} · exact bytes retained",
                binding.model_count(),
                plural(binding.model_count())
            )
        })
        .unwrap_or_else(|| "No authenticated source closure".to_owned());
    let replacement_detail = if catalog.diagnostics.is_empty() {
        format!(
            "{} authenticated candidate{} in catalog",
            catalog.candidates.len(),
            plural(catalog.candidates.len())
        )
    } else {
        format!(
            "{} catalog diagnostic{} require attention",
            catalog.diagnostics.len(),
            plural(catalog.diagnostics.len())
        )
    };
    let specs = [
        WorkspaceBandSpec {
            label: "Attachment",
            value: attachment_value,
            detail: attachment_detail,
            value_color: attachment_color,
        },
        WorkspaceBandSpec {
            label: "Model sections",
            value: sections.to_string(),
            detail: section_detail,
            value_color: t.color.text,
        },
        WorkspaceBandSpec {
            label: "Model sources",
            value: source_count.to_string(),
            detail: source_detail,
            value_color: if source_count > 0 {
                t.color.text
            } else {
                t.color.warn
            },
        },
        WorkspaceBandSpec {
            label: "Replacement candidates",
            value: replacement_count.to_string(),
            detail: replacement_detail,
            value_color: if catalog.diagnostics.is_empty() {
                t.color.text
            } else {
                t.color.warn
            },
        },
    ];
    workspace_status_band(
        ui,
        "workbench.project.technology.metrics",
        &specs,
        layout,
        layout.technology_metric_height(),
        true,
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TechnologyResourceRow {
    pub(super) resource: String,
    pub(super) identity: String,
    pub(super) scope: String,
    pub(super) platform: String,
    pub(super) status: String,
    pub(super) healthy: bool,
}

pub(super) fn technology_resource_rows(app: &RSpiceApp) -> Vec<TechnologyResourceRow> {
    let binding = app.state.workspace.project.technology_binding();
    let mut rows = Vec::new();
    if let Some(binding) = binding {
        let binding_healthy = app
            .state
            .model_library_manager
            .validate_attached_technology(Some(binding))
            .is_ok();
        rows.push(TechnologyResourceRow {
            resource: "Project technology binding".to_owned(),
            identity: binding.display_label(),
            scope: "project execution".to_owned(),
            platform: "desktop · web · mobile".to_owned(),
            status: if binding_healthy { "resolved" } else { "stale" }.to_owned(),
            healthy: binding_healthy,
        });
        let source_identity = binding
            .source_closure()
            .first()
            .map(|source| {
                format!(
                    "{} files · {}",
                    binding.source_closure().len(),
                    short_identity(&source.digest.to_string())
                )
            })
            .unwrap_or_else(|| "0 files".to_owned());
        rows.push(TechnologyResourceRow {
            resource: "SPICE model source closure".to_owned(),
            identity: source_identity,
            scope: "simulation".to_owned(),
            platform: "desktop · web · mobile".to_owned(),
            status: if binding_healthy {
                "verified"
            } else {
                "unavailable"
            }
            .to_owned(),
            healthy: binding_healthy,
        });
        rows.push(TechnologyResourceRow {
            resource: "Process-section catalog".to_owned(),
            identity: if binding.process_sections().is_empty() {
                "No named sections".to_owned()
            } else {
                binding.process_sections().join(" · ")
            },
            scope: "corner selection".to_owned(),
            platform: "desktop · web · mobile".to_owned(),
            status: if binding_healthy { "attached" } else { "stale" }.to_owned(),
            healthy: binding_healthy,
        });
        rows.push(TechnologyResourceRow {
            resource: "Source dependency graph".to_owned(),
            identity: format!(
                "{} edge{}",
                binding.source_edges().len(),
                plural(binding.source_edges().len())
            ),
            scope: "include resolution".to_owned(),
            platform: "desktop · web · mobile".to_owned(),
            status: if binding_healthy {
                "authenticated"
            } else {
                "stale"
            }
            .to_owned(),
            healthy: binding_healthy,
        });
    } else {
        rows.push(TechnologyResourceRow {
            resource: "Project technology binding".to_owned(),
            identity: app
                .state
                .workspace
                .project
                .technology
                .clone()
                .unwrap_or_else(|| "Not attached".to_owned()),
            scope: "project execution".to_owned(),
            platform: "desktop · web · mobile".to_owned(),
            status: "not executable".to_owned(),
            healthy: false,
        });
    }
    let search_catalog_ready = !app.state.pdk_config.library_paths.is_empty()
        && !app.state.pdk_config.discovered_files.is_empty()
        && app.state.pdk_config.scan_errors.is_empty();
    rows.push(TechnologyResourceRow {
        resource: "Configured model search catalog".to_owned(),
        identity: format!(
            "{} path{} · {} discovered file{}",
            app.state.pdk_config.library_paths.len(),
            plural(app.state.pdk_config.library_paths.len()),
            app.state.pdk_config.discovered_files.len(),
            plural(app.state.pdk_config.discovered_files.len()),
        ),
        scope: "model discovery".to_owned(),
        platform: "active workspace".to_owned(),
        status: if !app.state.pdk_config.scan_errors.is_empty() {
            "diagnostics"
        } else if search_catalog_ready {
            "available"
        } else {
            "empty"
        }
        .to_owned(),
        healthy: search_catalog_ready,
    });
    rows
}

pub(super) fn technology_resource_table(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let rows = technology_resource_rows(app);
    let healthy = rows.iter().all(|row| row.healthy);
    workspace_table_panel_header(
        ui,
        "Model technology resources",
        if healthy { "LOCKED" } else { "REVIEW REQUIRED" },
        if healthy { t.color.ok } else { t.color.warn },
    );
    let visible_width = ui.available_width().max(1.0);
    ScrollArea::horizontal()
        .id_salt("workbench.project.technology.resources")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let width = visible_width.max(TECHNOLOGY_TABLE_MIN_WIDTH);
            ui.set_min_width(width);
            let fractions = [0.24, 0.25, 0.17, 0.21, 0.13];
            workspace_table_row(
                ui,
                width,
                [
                    "RESOURCE",
                    "VERSION / IDENTITY",
                    "SCOPE",
                    "PLATFORM",
                    "STATUS",
                ],
                fractions,
                true,
                &[],
                &[],
            );
            for row in &rows {
                workspace_table_row(
                    ui,
                    width,
                    [
                        row.resource.as_str(),
                        row.identity.as_str(),
                        row.scope.as_str(),
                        row.platform.as_str(),
                        row.status.as_str(),
                    ],
                    fractions,
                    false,
                    &[1],
                    &[(
                        4,
                        if row.healthy {
                            t.color.ok
                        } else {
                            t.color.warn
                        },
                    )],
                );
            }
        });
}

pub(super) fn technology_migration_strip(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    layout: ProjectContractLayout,
) {
    let t = Tokens::get(ui.ctx());
    let attached = app.state.workspace.project.technology_binding().is_some();
    let copy = if attached {
        "A replacement is accepted only after its authenticated source contract is validated and a whole-project recovery checkpoint is written and read-back verified. The current binding remains authoritative until commit."
    } else {
        "Attachment validates an authenticated source contract and writes and read-back verifies a whole-project recovery checkpoint before the project binding changes."
    };
    let shown = egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            if layout.status_columns == 1 {
                ui.vertical(|ui| {
                    migration_safety_copy(ui, copy);
                    ui.add_space(7.0);
                    if Button::new(if attached {
                        "Review replacement…"
                    } else {
                        TECHNOLOGY_SURFACE_ACTION
                    })
                    .show(ui)
                    .clicked()
                    {
                        open_technology_attachment_dialog(app);
                    }
                });
            } else {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if Button::new(if attached {
                        "Review replacement…"
                    } else {
                        TECHNOLOGY_SURFACE_ACTION
                    })
                    .show(ui)
                    .clicked()
                    {
                        open_technology_attachment_dialog(app);
                    }
                    let width = ui.available_width().max(1.0);
                    ui.allocate_ui_with_layout(
                        vec2(width, 0.0),
                        Layout::top_down(Align::Min),
                        |ui| {
                            migration_safety_copy(ui, copy);
                        },
                    );
                });
            }
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

pub(super) fn migration_safety_copy(ui: &mut Ui, copy: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 12.0;
        ui.label(
            egui::RichText::new("Migration safety")
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(t.color.text),
        );
        ui.add(
            egui::Label::new(
                egui::RichText::new(copy)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            )
            .wrap(),
        );
    });
}

#[derive(Debug, Clone)]
pub(super) struct TechnologyCandidate {
    pub(super) binding: ProjectTechnologyBinding,
    pub(super) option_label: String,
    pub(super) corner_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TechnologyCandidateDiagnostic {
    pub(super) library_name: String,
    pub(super) reason: String,
}

#[derive(Debug, Clone, Default)]
pub(super) struct TechnologyCandidateCatalog {
    pub(super) candidates: Vec<TechnologyCandidate>,
    pub(super) diagnostics: Vec<TechnologyCandidateDiagnostic>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct TechnologyPrimaryState {
    pub(super) label: &'static str,
    pub(super) enabled: bool,
}

pub(super) const fn technology_primary_state(
    has_candidate: bool,
    checkpoint_pending: bool,
    selected_is_attached: bool,
) -> TechnologyPrimaryState {
    if checkpoint_pending {
        TechnologyPrimaryState {
            label: TECHNOLOGY_DIALOG_PENDING,
            enabled: false,
        }
    } else if selected_is_attached {
        TechnologyPrimaryState {
            label: TECHNOLOGY_DIALOG_CURRENT,
            enabled: false,
        }
    } else {
        TechnologyPrimaryState {
            label: TECHNOLOGY_DIALOG_PRIMARY,
            enabled: has_candidate,
        }
    }
}

pub(super) fn technology_candidates(app: &RSpiceApp) -> TechnologyCandidateCatalog {
    let mut catalog = TechnologyCandidateCatalog::default();
    for library in app.state.model_library_manager.libraries_sorted() {
        match ProjectTechnologyBinding::from_model_library(library) {
            Ok(binding) => {
                let option_label = format!(
                    "{} · {} model{} · {} authenticated source{}",
                    binding.display_label(),
                    binding.model_count(),
                    plural(binding.model_count()),
                    binding.source_closure().len(),
                    plural(binding.source_closure().len()),
                );
                catalog.candidates.push(TechnologyCandidate {
                    binding,
                    option_label,
                    corner_count: library.corner_count(),
                });
            }
            Err(error) => {
                let library_name = if library.name.trim().is_empty() {
                    "<unnamed configured library>".to_owned()
                } else {
                    library.name.clone()
                };
                catalog.diagnostics.push(TechnologyCandidateDiagnostic {
                    reason: format!(
                        "{error}. Configure, refresh, or re-import this library so it retains a canonical root, authenticated source bytes and dependency edges, and at least one parsed device model."
                    ),
                    library_name,
                });
            }
        }
    }
    catalog
}

pub(super) fn open_technology_attachment_dialog(app: &mut RSpiceApp) {
    let catalog = technology_candidates(app);
    let candidates = &catalog.candidates;
    let current = app
        .state
        .workspace
        .project
        .technology_binding()
        .map(|binding| binding.model_library().to_owned());
    let selected = current
        .filter(|name| {
            candidates
                .iter()
                .any(|candidate| candidate.binding.model_library() == name)
        })
        .or_else(|| {
            candidates
                .first()
                .map(|candidate| candidate.binding.model_library().to_owned())
        });
    app.state.dialogs.technology_attachment.open(selected);
}

pub(super) fn show_technology_attachment_dialog(ctx: &Context, app: &mut RSpiceApp) {
    if !app.state.dialogs.technology_attachment.open {
        return;
    }

    let catalog = technology_candidates(app);
    let candidates = &catalog.candidates;
    let selected_is_current = app
        .state
        .dialogs
        .technology_attachment
        .selected_library
        .as_deref()
        .is_some_and(|selected| {
            candidates
                .iter()
                .any(|candidate| candidate.binding.model_library() == selected)
        });
    if !selected_is_current {
        app.state.dialogs.technology_attachment.selected_library = candidates
            .first()
            .map(|candidate| candidate.binding.model_library().to_owned());
    }

    let selected_name = app
        .state
        .dialogs
        .technology_attachment
        .selected_library
        .clone();
    let selected_index = selected_name.as_deref().and_then(|name| {
        candidates
            .iter()
            .position(|candidate| candidate.binding.model_library() == name)
    });
    let options = candidates
        .iter()
        .map(|candidate| candidate.option_label.clone())
        .collect::<Vec<_>>();
    let selected_label = selected_index
        .and_then(|index| options.get(index))
        .map(String::as_str)
        .unwrap_or("No attachable authenticated model library");
    let validation_error = app
        .state
        .dialogs
        .technology_attachment
        .validation_error
        .clone();
    let checkpoint_pending = technology_checkpoint_pending(&app.state.dialogs);
    let selected_is_attached = selected_index
        .and_then(|index| candidates.get(index))
        .is_some_and(|candidate| {
            let label = candidate.binding.display_label();
            app.state.workspace.project.technology_binding() == Some(&candidate.binding)
                && app.state.workspace.project.technology.as_deref() == Some(label.as_str())
        });
    let primary = technology_primary_state(
        selected_index.is_some(),
        checkpoint_pending,
        selected_is_attached,
    );

    let choice = Dialog::new(
        "PROJECT · TECHNOLOGY CONTRACT",
        TECHNOLOGY_DIALOG_TITLE,
        primary.label,
    )
    .description(
        "Choose an authenticated model library. Commit verifies its retained source identity, writes and read-back verifies a whole-project checkpoint, then records one exact project-owned binding revision.",
    )
    .size(DialogSize::Transaction)
    .ghost("Cancel")
    .primary_enabled(primary.enabled)
    .show(ctx, |ui| {
        technology_warning(ui);
        technology_dialog_label(ui, "MODEL LIBRARY");
        if let Some(index) = select(
            ui,
            "project-model-technology-library",
            "Authenticated model library",
            selected_label,
            &options,
            ui.available_width(),
        ) {
            app.state.dialogs.technology_attachment.selected_library =
                Some(candidates[index].binding.model_library().to_owned());
            app.state.dialogs.technology_attachment.validation_error = None;
        }

        ui.add_space(10.0);
        technology_dialog_label(ui, "TECHNOLOGY MAPPING");
        if let Some(candidate) = selected_index.and_then(|index| candidates.get(index)) {
            property_card(ui, "Resolved authenticated model contract", |ui| {
                property_row(
                    ui,
                    "Devices",
                    &format!("{} parsed models", candidate.binding.model_count()),
                );
                property_row(
                    ui,
                    "Layers",
                    "Not supplied by this model-library binding",
                );
                property_row(
                    ui,
                    "Corners",
                    &format!("{} process sections", candidate.corner_count),
                );
                property_row(
                    ui,
                    "Source closure",
                    &format!(
                        "{} SHA-256 pinned files",
                        candidate.binding.source_closure().len()
                    ),
                );
            });
        } else {
            technology_unavailable_message(ui);
        }

        ui.add_space(10.0);
        technology_dialog_label(ui, "MIGRATION MODE");
        property_row(ui, "Mode", "Attach only · editable views unchanged");
        muted(
            ui,
            "This binding changes the project's simulation model-source contract only; it does not migrate schematic or physical design data.",
        );

        ui.add_space(10.0);
        technology_dialog_label(ui, "BINDING GATES");
        technology_binding_gates(ui, selected_index.is_some(), checkpoint_pending);
        if !catalog.diagnostics.is_empty() {
            ui.add_space(10.0);
            technology_dialog_label(ui, "LIBRARY VALIDATION");
            for diagnostic in &catalog.diagnostics {
                technology_error(
                    ui,
                    &format!(
                        "Model library '{}' cannot be attached: {}",
                        diagnostic.library_name, diagnostic.reason
                    ),
                );
                ui.add_space(6.0);
            }
        }
        if checkpoint_pending {
            muted(
                ui,
                "Writing and read-back verifying the full-project recovery checkpoint…",
            );
        }
        if let Some(error) = validation_error.as_deref() {
            ui.add_space(8.0);
            technology_error(ui, error);
        }
    });

    match choice {
        DialogChoice::Primary => {
            let result = selected_index
                .and_then(|index| candidates.get(index))
                .ok_or_else(|| "Select an attachable authenticated model library".to_owned())
                .and_then(|candidate| attach_technology_candidate(ctx, app, candidate));
            match result {
                Ok(receipt) => {
                    if technology_checkpoint_pending(&app.state.dialogs) {
                        app.state.push_user_message(ConsoleMessage::info(receipt));
                        return;
                    }
                    app.state.dialogs.technology_attachment.close();
                    app.state
                        .push_user_message(ConsoleMessage::info(receipt.clone()));
                    app.state
                        .ui
                        .toasts
                        .success(ctx, "Model technology attached", receipt);
                }
                Err(error) => {
                    app.state.dialogs.technology_attachment.validation_error = Some(error);
                }
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            if !technology_checkpoint_pending(&app.state.dialogs) {
                app.state.dialogs.technology_attachment.close();
            }
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

pub(super) fn attach_technology_candidate(
    ctx: &Context,
    app: &mut RSpiceApp,
    candidate: &TechnologyCandidate,
) -> Result<String, String> {
    #[cfg(not(target_arch = "wasm32"))]
    let _ = ctx;
    verify_pinned_model_sources(&candidate.binding, &app.state.model_library_manager)?;
    let label = candidate.binding.display_label();
    if app.state.workspace.project.technology_binding() == Some(&candidate.binding)
        && app.state.workspace.project.technology.as_deref() == Some(label.as_str())
    {
        return Ok(format!(
            "{} already matches project revision {}; no checkpoint or mutation was required.",
            label,
            app.state.workspace.project.revision().get()
        ));
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let checkpoint = crate::workbench::lifecycle::project_checkpoint::create(
            &app.state,
            crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointReason::TechnologyAttachment,
        )?;
        commit_technology_after_checkpoint(app, candidate.binding.clone(), &checkpoint)
    }

    #[cfg(target_arch = "wasm32")]
    {
        start_browser_technology_checkpoint(ctx, app, candidate.binding.clone())?;
        Ok("Writing and verifying the full-project recovery checkpoint…".to_owned())
    }
}

pub(super) fn commit_technology_after_checkpoint(
    app: &mut RSpiceApp,
    binding: ProjectTechnologyBinding,
    checkpoint: &crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
) -> Result<String, String> {
    let previous_revision = app.state.workspace.project.revision();
    if checkpoint.project_revision() != previous_revision.get()
        || checkpoint.project_name() != app.state.workspace.project.name()
        || !crate::workbench::lifecycle::project_checkpoint::matches_current_state(
            checkpoint, &app.state,
        )?
    {
        return Err(
            "Project changed after its recovery checkpoint was captured; attachment was not committed"
                .to_owned(),
        );
    }
    let revision = app
        .state
        .workspace
        .attach_technology(binding.clone())
        .map_err(|error| format!("Technology attachment was not committed: {error}"))?;
    if revision == previous_revision {
        return Err("Technology attachment unexpectedly produced no project revision".to_owned());
    }
    app.state.dialogs.project_checkpoint_recovery.invalidate();
    Ok(format!(
        "{} committed at project revision {} with {} exact source file{}; recovery checkpoint {} retains revision {}.",
        binding.display_label(),
        revision.get(),
        binding.source_closure().len(),
        plural(binding.source_closure().len()),
        checkpoint.checkpoint_id(),
        checkpoint.project_revision(),
    ))
}

pub(super) fn technology_checkpoint_pending(dialogs: &crate::workbench::app::DialogState) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        dialogs.technology_attachment.checkpoint_pending
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = dialogs;
        false
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn start_browser_technology_checkpoint(
    ctx: &Context,
    app: &mut RSpiceApp,
    binding: ProjectTechnologyBinding,
) -> Result<(), String> {
    if app.state.dialogs.technology_attachment.checkpoint_pending {
        return Err("A project recovery checkpoint is already being written".to_owned());
    }
    let project_id = app.state.workspace.project.id().to_string();
    let expected_revision = app.state.workspace.project.revision().get();
    let queued_project_id = project_id.clone();
    let queued_binding = binding.clone();
    let repaint = ctx.clone();
    crate::workbench::lifecycle::project_checkpoint::start_create(
        &app.state,
        crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointReason::TechnologyAttachment,
        move |result| {
            BROWSER_TECHNOLOGY_CHECKPOINT_COMPLETIONS.with(|queue| {
                queue
                    .borrow_mut()
                    .push_back(BrowserTechnologyCheckpointCompletion {
                        project_id: queued_project_id,
                        expected_revision,
                        binding: queued_binding,
                        result,
                    });
            });
            repaint.request_repaint();
        },
    )?;
    app.state.dialogs.technology_attachment.checkpoint_pending = true;
    app.state.dialogs.technology_attachment.validation_error = None;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(super) fn poll_browser_technology_checkpoint(ctx: &Context, app: &mut RSpiceApp) {
    let completions = BROWSER_TECHNOLOGY_CHECKPOINT_COMPLETIONS
        .with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in completions {
        if app.state.workspace.project.id().to_string() != completion.project_id
            || app.state.workspace.project.revision().get() != completion.expected_revision
        {
            app.state.dialogs.technology_attachment.checkpoint_pending = false;
            app.state.dialogs.technology_attachment.validation_error = Some(
                "Project changed while its recovery checkpoint was being written; attachment was not committed"
                    .to_owned(),
            );
            continue;
        }
        let result = completion.result.and_then(|checkpoint| {
            verify_pinned_model_sources(&completion.binding, &app.state.model_library_manager)?;
            commit_technology_after_checkpoint(app, completion.binding, &checkpoint)
        });
        app.state.dialogs.technology_attachment.checkpoint_pending = false;
        match result {
            Ok(receipt) => {
                app.state.dialogs.technology_attachment.close();
                app.state
                    .push_user_message(ConsoleMessage::info(receipt.clone()));
                app.state
                    .ui
                    .toasts
                    .success(ctx, "Model technology attached", receipt);
            }
            Err(error) => {
                app.state.dialogs.technology_attachment.validation_error = Some(error);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn poll_browser_recovery_completions(ctx: &Context, app: &mut RSpiceApp) {
    let project_id = app.state.workspace.project.id().to_string();
    let catalog = BROWSER_RECOVERY_CATALOG_COMPLETIONS
        .with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in catalog {
        if completion.project_id != project_id {
            continue;
        }
        let recovery = &mut app.state.dialogs.project_checkpoint_recovery;
        recovery.loading = false;
        recovery.initialized = true;
        match completion.result {
            Ok(catalog) => {
                recovery.checkpoints = catalog.checkpoints;
                recovery.quarantined = catalog.quarantined;
                recovery.error = None;
            }
            Err(error) => recovery.error = Some(error),
        }
    }

    let copies = BROWSER_RECOVERY_COPY_COMPLETIONS
        .with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in copies {
        if completion.project_id != project_id {
            continue;
        }
        let result = completion.result.and_then(|bytes| {
            crate::workbench::browser::download::download_bytes_file(
                std::path::Path::new(&completion.filename),
                &bytes,
                "application/vnd.rspice.project+json",
            )
        });
        match result {
            Ok(()) => {
                let receipt = format!(
                    "Handed independent recovery copy '{}' to the browser download manager",
                    completion.filename
                );
                app.state
                    .push_user_message(ConsoleMessage::info(receipt.clone()));
                app.state
                    .ui
                    .toasts
                    .success(ctx, "Recovery copy ready", receipt);
            }
            Err(error) => {
                app.state
                    .push_user_message(ConsoleMessage::error(error.clone()));
                app.state
                    .ui
                    .toasts
                    .error_with_title(ctx, "Recovery copy failed", error);
            }
        }
    }
}

pub(super) fn verify_pinned_model_sources(
    binding: &ProjectTechnologyBinding,
    manager: &ModelLibraryManager,
) -> Result<(), String> {
    binding
        .validate()
        .map_err(|error| format!("Technology contract is invalid: {error}"))?;
    manager.validate_attached_technology(Some(binding))?;
    #[cfg(not(target_arch = "wasm32"))]
    for source in binding.source_closure() {
        let observed = ModelLibraryManager::calculate_source_digest(&source.path)?;
        if observed != source.digest {
            return Err(format!(
                "Model source '{}' changed after it was parsed. Refresh the model library before attaching it.",
                source.path.display()
            ));
        }
    }
    Ok(())
}

pub(super) fn technology_warning(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(theme::mix(t.color.bg_panel, t.color.warn, 0.08))
        .stroke(Stroke::new(
            1.0,
            theme::mix(t.color.border, t.color.warn, 0.45),
        ))
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.add(egui::Label::new(
                egui::RichText::new(
                    "Attachment changes the project's authenticated model-source and process-section execution contract. Before mutation, RSpice durably writes and read-back verifies a whole-project recovery checkpoint; editable design data is never migrated implicitly.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text),
            ).wrap());
        });
    ui.add_space(10.0);
}

pub(super) fn technology_dialog_label(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.accent),
    );
    ui.add_space(3.0);
}

pub(super) fn technology_unavailable_message(ui: &mut Ui) {
    property_card(ui, "No attachable model technology", |ui| {
        muted(
            ui,
            "Configure and parse at least one external model library with a canonical source closure and one device model.",
        );
    });
}

pub(super) fn technology_binding_gates(ui: &mut Ui, has_candidate: bool, checkpoint_pending: bool) {
    let t = Tokens::get(ui.ctx());
    status_dot(
        ui,
        if has_candidate {
            t.color.ok
        } else {
            t.color.warn
        },
        if has_candidate {
            "Source authentication · retained bytes, SHA-256 pins, and dependency edges verified"
        } else {
            "Source authentication · attachable parsed models and authenticated bytes required"
        },
    );
    status_dot(
        ui,
        if checkpoint_pending {
            t.color.warn
        } else {
            t.color.ok
        },
        if checkpoint_pending {
            "Recovery checkpoint · writing and read-back verification in progress"
        } else {
            "Recovery checkpoint · whole-project digest verification required before commit"
        },
    );
    status_dot(
        ui,
        if has_candidate {
            t.color.ok
        } else {
            t.color.warn
        },
        technology_runtime_gate_copy(has_candidate),
    );
    status_dot(
        ui,
        t.color.text_faint,
        "Package signature · not represented by this model-library binding",
    );
    status_dot(
        ui,
        t.color.text_faint,
        "DRC deck qualification · not supplied; no physical sign-off claim",
    );
    status_dot(
        ui,
        t.color.text_faint,
        "License entitlement · not represented; no entitlement claim",
    );
}

pub(super) const fn technology_runtime_gate_copy(has_candidate: bool) -> &'static str {
    if !has_candidate {
        return "Model runtime compatibility · authenticated model binding required";
    }
    #[cfg(target_arch = "wasm32")]
    {
        "Model runtime compatibility · retained browser bytes verified against accepted SHA-256"
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "Model runtime compatibility · native source bytes reverified at attach and execution"
    }
}

pub(super) fn technology_error(ui: &mut Ui, error: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(theme::mix(t.color.bg_panel, t.color.err, 0.08))
        .stroke(Stroke::new(
            1.0,
            theme::mix(t.color.border, t.color.err, 0.5),
        ))
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.add(
                egui::Label::new(
                    egui::RichText::new(error)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.err),
                )
                .wrap(),
            );
        });
}
