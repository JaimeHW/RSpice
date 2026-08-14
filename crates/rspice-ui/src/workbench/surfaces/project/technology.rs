//! Attaching and migrating a project's technology.
//!
//! Attaching is checkpointed: the project is saved before the binding changes,
//! so a failed or abandoned migration leaves a recoverable state rather than a
//! half-migrated project. A candidate that would invalidate pinned model
//! sources is reported with the sources it would break instead of being
//! offered as a plain choice.

use super::*;

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

fn signed_package_key(package: &crate::state::pdk_config::ValidatedPdkTechnologyPackage) -> String {
    let manifest = package.manifest();
    format!(
        "{}@{}#{}",
        manifest.package_id,
        manifest.revision,
        package.manifest_digest()
    )
}

fn signed_package_label(
    package: &crate::state::pdk_config::ValidatedPdkTechnologyPackage,
) -> String {
    let manifest = package.manifest();
    let targets = manifest
        .compatibility
        .targets
        .iter()
        .map(|target| match target {
            crate::state::pdk_config::PdkExecutionTarget::Desktop => "desktop",
            crate::state::pdk_config::PdkExecutionTarget::WebAssembly => "browser",
            crate::state::pdk_config::PdkExecutionTarget::Mobile => "mobile",
        })
        .collect::<Vec<_>>()
        .join(" · ");
    format!(
        "{} · {} · {} nm · {} · {}",
        manifest.technology_name,
        manifest.revision,
        manifest.process_node_nm,
        manifest.stack_name,
        targets
    )
}

pub(super) fn open_technology_attachment_dialog(app: &mut RSpiceApp) {
    let catalog = technology_candidates(app);
    let candidates = &catalog.candidates;
    let current_library = app
        .state
        .workspace
        .project
        .technology_binding()
        .map(|binding| binding.model_library().to_owned());
    let selected_library = current_library
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
    let packages = app
        .state
        .pdk_config
        .technology_registry
        .validated_packages();
    let current_pin = app
        .state
        .workspace
        .project
        .technology_binding()
        .and_then(ProjectTechnologyBinding::signed_package);
    let selected_signed_package = current_pin
        .and_then(|pin| {
            packages
                .iter()
                .find(|package| {
                    let manifest = package.manifest();
                    manifest.package_id.eq_ignore_ascii_case(pin.package_id())
                        && manifest.revision == pin.revision()
                        && package.manifest_digest() == pin.manifest_digest()
                        && package.archive_digest() == pin.archive_digest()
                })
                .map(signed_package_key)
        })
        .or_else(|| {
            app.state
                .pdk_config
                .technology_registry
                .active_package()
                .map(signed_package_key)
        })
        .or_else(|| packages.first().map(signed_package_key));
    app.state
        .dialogs
        .technology_attachment
        .open(selected_library, selected_signed_package);
}

pub(super) fn show_technology_attachment_dialog(ctx: &Context, app: &mut RSpiceApp) {
    if !app.state.dialogs.technology_attachment.open {
        return;
    }

    let catalog = technology_candidates(app);
    let candidates = &catalog.candidates;
    let packages = app
        .state
        .pdk_config
        .technology_registry
        .validated_packages()
        .to_vec();
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
    let selected_package_is_current = app
        .state
        .dialogs
        .technology_attachment
        .selected_signed_package
        .as_deref()
        .is_some_and(|selected| {
            packages
                .iter()
                .any(|package| signed_package_key(package) == selected)
        });
    if !selected_package_is_current {
        app.state
            .dialogs
            .technology_attachment
            .selected_signed_package = app
            .state
            .pdk_config
            .technology_registry
            .active_package()
            .filter(|active| {
                packages
                    .iter()
                    .any(|package| signed_package_key(package) == signed_package_key(active))
            })
            .or_else(|| packages.first())
            .map(signed_package_key);
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
    let selected_package_key = app
        .state
        .dialogs
        .technology_attachment
        .selected_signed_package
        .clone();
    let selected_package_index = selected_package_key.as_deref().and_then(|key| {
        packages
            .iter()
            .position(|package| signed_package_key(package) == key)
    });
    let package_options = packages
        .iter()
        .map(signed_package_label)
        .collect::<Vec<_>>();
    let selected_package_label = selected_package_index
        .and_then(|index| package_options.get(index))
        .map(String::as_str)
        .unwrap_or("No trusted signed PDK package");
    let package_runtime_error = selected_package_index
        .and_then(|index| packages.get(index))
        .and_then(|package| package.runtime_compatibility().err());
    let selected_binding = selected_index
        .and_then(|index| candidates.get(index))
        .zip(selected_package_index.and_then(|index| packages.get(index)))
        .map(|(candidate, package)| {
            candidate
                .binding
                .clone()
                .with_signed_package(package)
                .and_then(|binding| {
                    binding.validate_signed_package(&app.state.pdk_config.technology_registry)?;
                    Ok(binding)
                })
        })
        .transpose();
    let mut selected_binding_error = selected_binding
        .as_ref()
        .err()
        .map(ToString::to_string)
        .or(package_runtime_error.clone());
    let selected_binding = selected_binding.ok().flatten();
    let selected_package = selected_package_index.and_then(|index| packages.get(index));
    let migration_preview =
        app.state
            .project_signed_technology_package()
            .and_then(|baseline| match (baseline, selected_package) {
                (Some(baseline), Some(candidate))
                    if baseline.binding() != candidate.binding()
                        || baseline.archive_digest() != candidate.archive_digest() =>
                {
                    crate::state::pdk_config::PdkTechnologyRevisionDiff::between(
                        baseline, candidate,
                    )
                    .map(Some)
                    .map_err(|error| error.to_string())
                }
                _ => Ok(None),
            });
    let migration_evidence = migration_preview
        .as_ref()
        .map_err(Clone::clone)
        .and_then(|diff| {
            diff.as_ref()
                .map(crate::state::pdk_config::PdkTechnologyMigrationEvidence::from_diff)
                .transpose()
                .map_err(|error| error.to_string())
        });
    let migration_gate_error = migration_preview
        .as_ref()
        .err()
        .cloned()
        .or_else(|| migration_evidence.as_ref().err().cloned())
        .or_else(|| {
            migration_preview
                .as_ref()
                .ok()
                .and_then(Option::as_ref)
                .and_then(|diff| {
                    if !diff.same_package_lineage {
                        Some(
                            "Cross-technology replacement requires a dedicated design-migration branch; direct project attachment is prohibited."
                                .to_owned(),
                        )
                    } else if diff.has_breaking_changes() {
                        Some(format!(
                            "This revision changes {} breaking signed contract(s). Create and validate a dedicated migration branch before replacing the project pin.",
                            diff.count(crate::state::pdk_config::PdkTechnologyDiffImpact::Breaking)
                        ))
                    } else {
                        None
                    }
                })
        });
    if selected_binding_error.is_none() {
        selected_binding_error = migration_gate_error.clone();
    }
    let migration_requires_review = migration_preview
        .as_ref()
        .ok()
        .and_then(Option::as_ref)
        .is_some();
    let migration_ready = migration_gate_error.is_none()
        && (!migration_requires_review
            || app.state.dialogs.technology_attachment.migration_reviewed);
    let validation_error = app
        .state
        .dialogs
        .technology_attachment
        .validation_error
        .clone();
    let checkpoint_pending = technology_checkpoint_pending(&app.state.dialogs);
    let authority_ready =
        technology_authority_fields_ready(&app.state.dialogs.technology_attachment);
    let selected_is_attached = selected_binding.as_ref().is_some_and(|binding| {
        let label = binding.display_label();
        app.state.workspace.project.technology_binding() == Some(binding)
            && app.state.workspace.project.technology.as_deref() == Some(label.as_str())
    });
    let primary = technology_primary_state(
        selected_binding.is_some() && authority_ready && migration_ready,
        checkpoint_pending,
        selected_is_attached,
    );

    let mut create_migration_copy = false;
    let choice = Dialog::new(
        "PROJECT · TECHNOLOGY CONTRACT",
        TECHNOLOGY_DIALOG_TITLE,
        primary.label,
    )
    .description(
        "Choose an authenticated model library and one trusted signed PDK revision. Commit verifies both exact identities, writes and read-back verifies a whole-project checkpoint, then records one immutable project-owned binding revision.",
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
        technology_dialog_label(ui, "SIGNED PDK PACKAGE");
        if let Some(index) = select(
            ui,
            "project-signed-technology-package",
            "Trusted signed PDK package",
            selected_package_label,
            &package_options,
            ui.available_width(),
        ) {
            app.state
                .dialogs
                .technology_attachment
                .selected_signed_package = Some(signed_package_key(&packages[index]));
            app.state.dialogs.technology_attachment.migration_reviewed = false;
            app.state.dialogs.technology_attachment.validation_error = None;
        }

        ui.add_space(10.0);
        technology_dialog_label(ui, "CHANGE AUTHORITY");
        ui.add_enabled(
            !checkpoint_pending,
            egui::TextEdit::singleline(
                &mut app.state.dialogs.technology_attachment.actor_id,
            )
            .hint_text("Actor ID")
            .char_limit(240)
            .desired_width(f32::INFINITY),
        );
        ui.add_enabled(
            !checkpoint_pending,
            egui::TextEdit::singleline(
                &mut app.state.dialogs.technology_attachment.authority_id,
            )
            .hint_text("Authority ID")
            .char_limit(240)
            .desired_width(f32::INFINITY),
        );
        ui.add_enabled(
            !checkpoint_pending,
            egui::TextEdit::multiline(&mut app.state.dialogs.technology_attachment.reason)
                .hint_text("Change reason")
                .char_limit(1_024)
                .desired_rows(2)
                .desired_width(f32::INFINITY),
        );
        muted(
            ui,
            "Actor, authority, reason, exact before/after contract digests, and checkpoint evidence are retained in the project’s immutable hash-linked technology history.",
        );

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
                    &selected_package_index
                        .and_then(|index| packages.get(index))
                        .map(|package| {
                            format!(
                                "{} signed technology layers",
                                package.manifest().layers.len()
                            )
                        })
                        .unwrap_or_else(|| "No signed layer catalog selected".to_owned()),
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
                if let Some(package) =
                    selected_package_index.and_then(|index| packages.get(index))
                {
                    property_row(
                        ui,
                        "Package identity",
                        &format!(
                            "{} {} · manifest {} · archive {}",
                            package.manifest().package_id,
                            package.manifest().revision,
                            short_identity(&package.manifest_digest().to_string()),
                            short_identity(&package.archive_digest().to_string())
                        ),
                    );
                    property_row(
                        ui,
                        "Publisher signature",
                        &format!(
                            "{} · key {}",
                            package.manifest().publisher_id,
                            package.manifest().signing_key_id
                        ),
                    );
                }
            });
        } else {
            technology_unavailable_message(ui);
        }

        ui.add_space(10.0);
        technology_dialog_label(ui, "MIGRATION MODE");
        match migration_preview.as_ref() {
            Ok(Some(diff)) => {
                property_row(
                    ui,
                    "Mode",
                    "Exact revision replacement · checkpointed · editable views unchanged",
                );
                property_row(
                    ui,
                    "Signed diff",
                    &format!(
                        "{} breaking · {} review · {} informational",
                        diff.count(crate::state::pdk_config::PdkTechnologyDiffImpact::Breaking),
                        diff.count(
                            crate::state::pdk_config::PdkTechnologyDiffImpact::ReviewRequired,
                        ),
                        diff.count(crate::state::pdk_config::PdkTechnologyDiffImpact::Informational),
                    ),
                );
                if let Ok(Some(evidence)) = migration_evidence.as_ref() {
                    property_row(
                        ui,
                        "Evidence",
                        &format!(
                            "diff {} · {} exact entries",
                            short_identity(&evidence.diff_digest().to_string()),
                            evidence.entry_count()
                        ),
                    );
                }
                ScrollArea::vertical()
                    .id_salt("project-technology-migration-preview")
                    .max_height(150.0)
                    .show(ui, |ui| {
                        for entry in &diff.entries {
                            ui.monospace(format!(
                                "{:?} · {:?} · {}",
                                entry.impact, entry.area, entry.identity
                            ));
                        }
                    });
                if let Some(error) = migration_gate_error.as_deref() {
                    technology_error(ui, error);
                    if Button::new("Save independent migration copy…")
                        .enabled(!checkpoint_pending)
                        .accessible_label(
                            "Save an independent project copy for explicit technology remapping",
                        )
                        .show(ui)
                        .clicked()
                    {
                        create_migration_copy = true;
                    }
                    muted(
                        ui,
                        "The copy retains the current exact PDK pin and starts a fresh project audit chain. Open that copy, remap affected objects explicitly, validate it, then review the replacement again. The current project remains unchanged.",
                    );
                } else {
                    ui.add_enabled(
                        !checkpoint_pending,
                        egui::Checkbox::new(
                            &mut app.state.dialogs.technology_attachment.migration_reviewed,
                            "I reviewed the exact signed revision diff and approve this non-breaking replacement",
                        ),
                    );
                    muted(
                        ui,
                        "The immutable project receipt retains the exact baseline, candidate, archive identities, diff digest, impact counts, and recovery checkpoint.",
                    );
                }
            }
            Ok(None) => {
                property_row(ui, "Mode", "Attach only · editable views unchanged");
                muted(
                    ui,
                    "Initial attachment changes the project execution contract only. Editable schematic and physical views are never migrated implicitly.",
                );
            }
            Err(error) => technology_error(
                ui,
                &format!("Exact migration preview failed closed: {error}"),
            ),
        }

        ui.add_space(10.0);
        technology_dialog_label(ui, "BINDING GATES");
        technology_binding_gates(
            ui,
            selected_index.is_some(),
            selected_package_index.is_some(),
            selected_binding.is_some(),
            authority_ready,
            checkpoint_pending,
        );
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
        if let Some(error) = selected_binding_error.as_deref() {
            ui.add_space(8.0);
            technology_error(
                ui,
                &format!("Selected signed PDK cannot be attached: {error}"),
            );
        }
        if let Some(error) = validation_error.as_deref() {
            ui.add_space(8.0);
            technology_error(ui, error);
        }
    });

    if create_migration_copy {
        app.state.dialogs.technology_attachment.close();
        let saved = crate::workbench::workflows::project_workflow::save_project_as(&mut app.state);
        if saved {
            app.state.push_user_message(ConsoleMessage::info(
                "Created an independent migration copy with the current exact technology pin; the active project was not changed."
                    .to_owned(),
            ));
        }
        return;
    }

    match choice {
        DialogChoice::Primary => {
            let result = selected_binding
                .clone()
                .ok_or_else(|| {
                    "Select an attachable authenticated model library and trusted signed PDK package"
                        .to_owned()
                })
                .and_then(|binding| {
                    technology_change_authority(&app.state.dialogs.technology_attachment)
                        .and_then(|authority| {
                            attach_technology_binding(
                                ctx,
                                app,
                                binding,
                                authority,
                                migration_evidence.clone().ok().flatten(),
                            )
                        })
                });
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

pub(super) fn attach_technology_binding(
    ctx: &Context,
    app: &mut RSpiceApp,
    binding: ProjectTechnologyBinding,
    authority: crate::state::ProjectTechnologyChangeAuthority,
    migration_evidence: Option<crate::state::pdk_config::PdkTechnologyMigrationEvidence>,
) -> Result<String, String> {
    #[cfg(not(target_arch = "wasm32"))]
    let _ = ctx;
    verify_pinned_technology_contract(&binding, app)?;
    validate_migration_evidence_for_binding(app, &binding, migration_evidence.as_ref())?;
    let label = binding.display_label();
    if app.state.workspace.project.technology_binding() == Some(&binding)
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
        commit_technology_after_checkpoint(app, binding, authority, migration_evidence, &checkpoint)
    }

    #[cfg(target_arch = "wasm32")]
    {
        start_browser_technology_checkpoint(ctx, app, binding, authority, migration_evidence)?;
        Ok("Writing and verifying the full-project recovery checkpoint…".to_owned())
    }
}

pub(super) fn commit_technology_after_checkpoint(
    app: &mut RSpiceApp,
    binding: ProjectTechnologyBinding,
    authority: crate::state::ProjectTechnologyChangeAuthority,
    migration_evidence: Option<crate::state::pdk_config::PdkTechnologyMigrationEvidence>,
    checkpoint: &crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
) -> Result<String, String> {
    verify_pinned_technology_contract(&binding, app)?;
    validate_migration_evidence_for_binding(app, &binding, migration_evidence.as_ref())?;
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
    let checkpoint_revision = crate::product::ObjectRevision::new(checkpoint.project_revision())
        .map_err(|error| format!("Recovery checkpoint revision is invalid: {error}"))?;
    let mut change_context = crate::state::ProjectTechnologyChangeContext::new(
        authority,
        checkpoint.checkpoint_id(),
        checkpoint_revision,
        checkpoint.created_unix_ms(),
        checkpoint.snapshot_digest(),
        checkpoint.snapshot_byte_len(),
    )
    .map_err(|error| format!("Technology change authority is invalid: {error}"))?;
    if let Some(evidence) = migration_evidence {
        change_context = change_context
            .with_migration_evidence(evidence)
            .map_err(|error| format!("Technology migration evidence is invalid: {error}"))?;
    }
    let (revision, audit_receipt) = app
        .state
        .workspace
        .attach_technology_audited(binding.clone(), change_context)
        .map_err(|error| format!("Technology attachment was not committed: {error}"))?;
    if revision == previous_revision {
        return Err("Technology attachment unexpectedly produced no project revision".to_owned());
    }
    app.state.dialogs.project_checkpoint_recovery.invalidate();
    Ok(format!(
        "{} committed at project revision {} as technology receipt #{} with {} exact source file{}; recovery checkpoint {} retains revision {}.",
        binding.display_label(),
        revision.get(),
        audit_receipt.sequence(),
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
    authority: crate::state::ProjectTechnologyChangeAuthority,
    migration_evidence: Option<crate::state::pdk_config::PdkTechnologyMigrationEvidence>,
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
                        authority,
                        migration_evidence,
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
            verify_pinned_technology_contract(&completion.binding, app)?;
            commit_technology_after_checkpoint(
                app,
                completion.binding,
                completion.authority,
                completion.migration_evidence,
                &checkpoint,
            )
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

pub(super) fn verify_pinned_technology_contract(
    binding: &ProjectTechnologyBinding,
    app: &RSpiceApp,
) -> Result<(), String> {
    verify_pinned_model_sources(binding, &app.state.model_library_manager)?;
    binding
        .validate_signed_package(&app.state.pdk_config.technology_registry)
        .map_err(|error| format!("Signed PDK contract is unavailable: {error}"))
}

fn validate_migration_evidence_for_binding(
    app: &RSpiceApp,
    binding: &ProjectTechnologyBinding,
    evidence: Option<&crate::state::pdk_config::PdkTechnologyMigrationEvidence>,
) -> Result<(), String> {
    let baseline = app.state.project_signed_technology_package()?;
    let candidate_pin = binding
        .signed_package()
        .ok_or_else(|| "The candidate binding has no signed PDK package pin.".to_owned())?;
    let candidate = app
        .state
        .pdk_config
        .technology_registry
        .validated_packages()
        .iter()
        .find(|package| {
            package
                .manifest()
                .package_id
                .eq_ignore_ascii_case(candidate_pin.package_id())
                && package.manifest().revision == candidate_pin.revision()
                && package.manifest_digest() == candidate_pin.manifest_digest()
                && package.archive_digest() == candidate_pin.archive_digest()
        })
        .ok_or_else(|| {
            "Candidate signed PDK no longer resolves to the exact trusted archive.".to_owned()
        })?;
    let Some(baseline) = baseline else {
        if evidence.is_some() {
            return Err(
                "An initial technology attachment must not claim revision-migration evidence."
                    .to_owned(),
            );
        }
        return Ok(());
    };
    if baseline.binding() == candidate.binding()
        && baseline.archive_digest() == candidate.archive_digest()
    {
        if evidence.is_some() {
            return Err(
                "The project already uses this exact signed package; migration evidence is not applicable."
                    .to_owned(),
            );
        }
        return Ok(());
    }
    let diff = crate::state::pdk_config::PdkTechnologyRevisionDiff::between(baseline, candidate)
        .map_err(|error| format!("Exact signed revision comparison failed: {error}"))?;
    if !diff.same_package_lineage {
        return Err(
            "Cross-technology replacement requires an independent migration copy; direct replacement is prohibited."
                .to_owned(),
        );
    }
    if diff.has_breaking_changes() {
        return Err(format!(
            "The signed revision changes {} breaking contract(s); direct replacement is prohibited until those objects are explicitly remapped in an independent migration copy.",
            diff.count(crate::state::pdk_config::PdkTechnologyDiffImpact::Breaking)
        ));
    }
    let evidence = evidence.ok_or_else(|| {
        "Replacing a signed PDK revision requires the exact reviewed comparison evidence."
            .to_owned()
    })?;
    if !evidence.matches_diff(&diff) {
        return Err(
            "Migration evidence no longer matches the exact trusted baseline and candidate packages."
                .to_owned(),
        );
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
                    "Attachment changes the project's authenticated model-source, signed PDK, layer, and process-section execution contract. Before mutation, RSpice durably writes and read-back verifies a whole-project recovery checkpoint; editable design data is never migrated implicitly.",
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

pub(super) fn technology_binding_gates(
    ui: &mut Ui,
    has_model_candidate: bool,
    has_signed_package: bool,
    contract_ready: bool,
    authority_ready: bool,
    checkpoint_pending: bool,
) {
    let t = Tokens::get(ui.ctx());
    status_dot(
        ui,
        if has_model_candidate {
            t.color.ok
        } else {
            t.color.warn
        },
        if has_model_candidate {
            "Source authentication · retained bytes, SHA-256 pins, and dependency edges verified"
        } else {
            "Source authentication · attachable parsed models and authenticated bytes required"
        },
    );
    status_dot(
        ui,
        if authority_ready {
            t.color.ok
        } else {
            t.color.warn
        },
        if authority_ready {
            "Change authority · actor, authority, and reason ready for immutable receipt"
        } else {
            "Change authority · actor, authority, and reason are required"
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
        if contract_ready {
            t.color.ok
        } else {
            t.color.warn
        },
        technology_runtime_gate_copy(contract_ready),
    );
    status_dot(
        ui,
        if has_signed_package {
            t.color.ok
        } else {
            t.color.warn
        },
        if has_signed_package {
            "Package signature · publisher key, exact manifest bytes, archive, and artifacts verified"
        } else {
            "Package signature · a currently trusted signed PDK revision is required"
        },
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

fn technology_authority_fields_ready(
    dialog: &crate::workbench::app::TechnologyAttachmentDialogState,
) -> bool {
    !dialog.actor_id.trim().is_empty()
        && !dialog.authority_id.trim().is_empty()
        && !dialog.reason.trim().is_empty()
}

fn technology_change_authority(
    dialog: &crate::workbench::app::TechnologyAttachmentDialogState,
) -> Result<crate::state::ProjectTechnologyChangeAuthority, String> {
    crate::state::ProjectTechnologyChangeAuthority::new(
        dialog.actor_id.trim(),
        dialog.authority_id.trim(),
        dialog.reason.trim(),
    )
    .map_err(|error| format!("Technology change authority is invalid: {error}"))
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
