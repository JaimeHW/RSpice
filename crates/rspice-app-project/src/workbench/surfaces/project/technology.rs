//! Attaching and migrating a project's technology.
//!
//! Attaching is checkpointed: the project is saved before the binding changes,
//! so a failed or abandoned migration leaves a recoverable state rather than a
//! half-migrated project. A candidate that would invalidate pinned model
//! sources is reported with the sources it would break instead of being
//! offered as a plain choice.

use std::sync::Arc;

use super::derived::derived;
use super::*;
use crate::state::pdk_config::{diff_area_label, diff_impact_label};
use crate::ui::widgets::{field_label, name_control, text_field};
use crate::workbench::app::dialogs::project::{
    TechnologyAttachment, TechnologyChange, TechnologyChangeInFlight,
};
use crate::workbench::app_state::Model;
use crate::workbench::project::commands::ProjectRequest;

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

/// The attach button: held while the checkpoint before a change is written,
/// which the dialog's one pending line states.
pub(super) const fn technology_primary_state(
    has_candidate: bool,
    checkpoint_pending: bool,
    selected_is_attached: bool,
) -> TechnologyPrimaryState {
    if checkpoint_pending {
        TechnologyPrimaryState {
            label: TECHNOLOGY_DIALOG_PRIMARY,
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

pub(super) fn technology_candidates(model: &Model) -> TechnologyCandidateCatalog {
    let mut catalog = TechnologyCandidateCatalog::default();
    for library in model.model_library_manager.libraries_sorted() {
        match ProjectTechnologyBinding::from_model_library(library) {
            Ok(binding) => {
                let option_label = format!(
                    "{} · {} model{} · {} source file{}",
                    binding.display_label(),
                    binding.model_count(),
                    plural_suffix(binding.model_count()),
                    binding.source_closure().len(),
                    plural_suffix(binding.source_closure().len()),
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
                        "{error}. Configure, refresh, or re-import this library so it has a root source file, the files it includes, and at least one parsed device model."
                    ),
                    library_name,
                });
            }
        }
    }
    catalog
}

fn pdk_package_key(package: &crate::state::pdk_config::ValidatedPdkTechnologyPackage) -> String {
    let manifest = package.manifest();
    format!(
        "{}@{}#{}",
        manifest.package_id,
        manifest.revision,
        package.manifest_digest()
    )
}

fn pdk_package_label(package: &crate::state::pdk_config::ValidatedPdkTechnologyPackage) -> String {
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

pub(super) fn open_technology_attachment_dialog(cx: &mut Cx<'_, ProjectFeature>) {
    let catalog = technology_candidates(cx.model);
    let candidates = &catalog.candidates;
    let current_library = cx
        .model
        .workspace
        .content
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
    let packages = cx.model.pdk_config.technology_registry.validated_packages();
    let current_pin = cx
        .model
        .workspace
        .content
        .project
        .technology_binding()
        .and_then(ProjectTechnologyBinding::pdk_package);
    let selected_pdk_package = current_pin
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
                .map(pdk_package_key)
        })
        .or_else(|| packages.first().map(pdk_package_key));
    // The person at this application: the signed-in account, else the
    // license holder, else the operating system's user.
    let actor = cx.peers.cloud_session().review_actor(cx.peers.cloud());
    cx.state
        .technology_attachment
        .open(selected_library, selected_pdk_package, actor);
}

pub fn show_technology_attachment_dialog(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    if !cx.state.technology_attachment.open {
        return;
    }
    if cx.state.technology_attachment.detach {
        show_technology_detach_dialog(ctx, cx);
        return;
    }

    // Proving a candidate hashes its library's retained sources, so the
    // candidates are proven when the catalogue changes, not on each repaint.
    let catalog = derived(
        ctx,
        "technology.candidates",
        cx.model.model_library_manager.content_stamp(),
        || Arc::new(technology_candidates(cx.model)),
    );
    let candidates = &catalog.candidates;
    let packages = cx
        .model
        .pdk_config
        .technology_registry
        .validated_packages()
        .to_vec();
    let selected_is_current = cx
        .state
        .technology_attachment
        .selected_library
        .as_deref()
        .is_some_and(|selected| {
            candidates
                .iter()
                .any(|candidate| candidate.binding.model_library() == selected)
        });
    if !selected_is_current {
        cx.state.technology_attachment.selected_library = candidates
            .first()
            .map(|candidate| candidate.binding.model_library().to_owned());
    }
    let selected_package_is_current = cx
        .state
        .technology_attachment
        .selected_pdk_package
        .as_deref()
        .is_some_and(|selected| {
            packages
                .iter()
                .any(|package| pdk_package_key(package) == selected)
        });
    if !selected_package_is_current {
        cx.state.technology_attachment.selected_pdk_package = packages.first().map(pdk_package_key);
    }

    let selected_name = cx.state.technology_attachment.selected_library.clone();
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
        .unwrap_or("No model library can be attached");
    let selected_package_key = cx.state.technology_attachment.selected_pdk_package.clone();
    let selected_package_index = selected_package_key.as_deref().and_then(|key| {
        packages
            .iter()
            .position(|package| pdk_package_key(package) == key)
    });
    let package_options = packages.iter().map(pdk_package_label).collect::<Vec<_>>();
    let selected_package_label = selected_package_index
        .and_then(|index| package_options.get(index))
        .map(String::as_str)
        .unwrap_or("No validated PDK package");
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
                .with_pdk_package_metadata(package.metadata())
                .and_then(|binding| {
                    cx.model
                        .pdk_config
                        .technology_registry
                        .validate_project_binding(&binding)?;
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
        cx.model
            .project_pdk_package()
            .and_then(|baseline| match (baseline, selected_package) {
                (Some(baseline), Some(candidate))
                    if baseline.binding() != candidate.binding()
                        || baseline.archive_digest() != candidate.archive_digest() =>
                {
                    crate::state::pdk_config::PdkTechnologyRevisionDiff::between(
                        baseline.metadata(),
                        candidate.metadata(),
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
                            "This revision changes {} breaking PDK contract(s). Create and validate a dedicated migration branch before replacing the project pin.",
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
        && (!migration_requires_review || cx.state.technology_attachment.migration_reviewed);
    let validation_error = cx.state.technology_attachment.validation_error.clone();
    let checkpoint_pending = technology_checkpoint_pending(cx.state);
    let authority_ready = technology_authority_fields_ready(&cx.state.technology_attachment);
    let selected_is_attached = selected_binding.as_ref().is_some_and(|binding| {
        let label = binding.display_label();
        cx.model.workspace.content.project.technology_binding() == Some(binding)
            && cx.model.workspace.content.project.technology.as_deref() == Some(label.as_str())
    });
    let primary = technology_primary_state(
        selected_binding.is_some() && authority_ready && migration_ready,
        checkpoint_pending,
        selected_is_attached,
    );

    let mut create_migration_copy = false;
    let dialog = Dialog::new(TECHNOLOGY_DIALOG_TITLE, primary.label)
    .description(
        "Choose a model library and one validated PDK revision. Attaching verifies both, writes a whole-project checkpoint and reads it back, then records the binding as a new project revision.",
    )
    .size(DialogSize::Transaction)
    .ghost("Cancel")
    .primary_enabled(primary.enabled);
    let choice = technology_change_hint(dialog, checkpoint_pending, validation_error.as_deref())
    .show(ctx, |ui| {
        technology_warning(ui);
        technology_dialog_label(ui, "Model library");
        if let Some(index) = select(
            ui,
            "project-model-technology-library",
            "Model library",
            selected_label,
            &options,
            ui.available_width(),
        ) {
            cx.state.technology_attachment.selected_library =
                Some(candidates[index].binding.model_library().to_owned());
            cx.state.technology_attachment.validation_error = None;
        }

        ui.add_space(10.0);
        technology_dialog_label(ui, "PDK package");
        if let Some(index) = select(
            ui,
            "project-pdk-package",
            "Validated PDK package",
            selected_package_label,
            &package_options,
            ui.available_width(),
        ) {
            cx.state
                .technology_attachment
                .selected_pdk_package = Some(pdk_package_key(&packages[index]));
            cx.state.technology_attachment.migration_reviewed = false;
            cx.state.technology_attachment.validation_error = None;
        }

        ui.add_space(10.0);
        technology_change_fields(ui, &mut cx.state.technology_attachment, !checkpoint_pending);

        ui.add_space(10.0);
        technology_dialog_label(ui, "Technology mapping");
        if let Some(candidate) = selected_index.and_then(|index| candidates.get(index)) {
            property_card(ui, "Resolved model library", |ui| {
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
                                "{} PDK technology layers",
                                package.manifest().layers.len()
                            )
                        })
                        .unwrap_or_else(|| "No PDK layer catalog selected".to_owned()),
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
                    property_row(ui, "Publisher", &package.manifest().publisher_id);
                }
            });
        } else {
            technology_unavailable_message(ui);
        }

        ui.add_space(10.0);
        technology_dialog_label(ui, "Migration mode");
        match migration_preview.as_ref() {
            Ok(Some(diff)) => {
                property_row(
                    ui,
                    "Mode",
                    "Revision replacement · checkpointed · editable views unchanged",
                );
                property_row(
                    ui,
                    "Revision diff",
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
                        "Reviewed diff",
                        &format!(
                            "diff {} · {} entries",
                            short_identity(&evidence.diff_digest().to_string()),
                            evidence.entry_count()
                        ),
                    );
                }
                ScrollArea::vertical()
                    .id_salt("project-technology-migration-preview")
                    .auto_shrink([false, true])
                    .max_height(150.0)
                    .show(ui, |ui| {
                        for entry in &diff.entries {
                            ui.monospace(format!(
                                "{} · {} · {}",
                                diff_impact_label(entry.impact),
                                diff_area_label(entry.area),
                                entry.identity
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
                        "The copy keeps the current PDK pin and starts a new project audit history. Open that copy, remap affected objects explicitly, validate it, then review the replacement again. The current project remains unchanged.",
                    );
                } else {
                    ui.add_enabled(
                        !checkpoint_pending,
                        egui::Checkbox::new(
                            &mut cx.state.technology_attachment.migration_reviewed,
                            "I reviewed the PDK revision diff and approve this non-breaking replacement",
                        ),
                    );
                    muted(
                        ui,
                        "The project's audit record keeps the baseline and candidate revisions, archive identities, diff digest, impact counts, and recovery checkpoint.",
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
                &format!("Migration preview failed: {error}"),
            ),
        }

        ui.add_space(10.0);
        technology_binding_gates(
            ui,
            selected_index.is_some(),
            selected_package_index.is_some(),
            selected_binding.is_some(),
            authority_ready,
        );
        if !catalog.diagnostics.is_empty() {
            ui.add_space(10.0);
            technology_dialog_label(ui, "Library validation");
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
        if let Some(error) = selected_binding_error.as_deref() {
            ui.add_space(8.0);
            technology_error(
                ui,
                &format!("Selected PDK cannot be attached: {error}"),
            );
        }
    });

    if create_migration_copy {
        cx.state.technology_attachment.close();
        cx.wb.outbox.push(ProjectRequest::SaveMigrationCopy);
        return;
    }

    match choice {
        DialogChoice::Primary => {
            let result = selected_binding
                .clone()
                .ok_or_else(|| {
                    "Select a model library that can be attached and a validated PDK package"
                        .to_owned()
                })
                .and_then(|binding| {
                    technology_change_authority(&cx.state.technology_attachment).and_then(
                        |authority| {
                            attach_technology_binding(
                                ctx,
                                cx,
                                binding,
                                authority,
                                migration_evidence.clone().ok().flatten(),
                            )
                        },
                    )
                });
            match result {
                // Its checkpoint is being written; the change commits when
                // that is in.
                Ok(None) => {}
                Ok(Some(receipt)) => {
                    cx.state.technology_attachment.close();
                    cx.model
                        .record(LogSeverity::Info, LogSource::Studio, receipt);
                }
                Err(error) => {
                    cx.state.technology_attachment.validation_error = Some(error);
                }
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            if !technology_checkpoint_pending(cx.state) {
                cx.state.technology_attachment.close();
            }
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

/// Open the detach dialog on the project's technology, the change made and
/// approved by the person at this application.
pub(super) fn open_technology_detach_dialog(cx: &mut Cx<'_, ProjectFeature>) {
    let actor = cx.peers.cloud_session().review_actor(cx.peers.cloud());
    cx.state.technology_attachment.open_detach(actor);
}

/// Detach the project's technology: who does it and why, then the
/// checkpoint, then the change, as attaching is done.
fn show_technology_detach_dialog(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    let pending = technology_checkpoint_pending(cx.state);
    let label = cx
        .model
        .workspace
        .content
        .project
        .technology_binding()
        .map(ProjectTechnologyBinding::display_label);
    // What the simulation setup needs a technology for: runs that need one
    // are refused once it is gone.
    let demand = cx
        .model
        .technology_inputs()
        .technology_demand()
        .block_reason();
    let ready = label.is_some()
        && !pending
        && technology_authority_fields_ready(&cx.state.technology_attachment);
    let validation_error = cx.state.technology_attachment.validation_error.clone();
    let dialog = Dialog::new(TECHNOLOGY_DETACH_TITLE, TECHNOLOGY_DETACH_PRIMARY)
        .description(
            "The project stops simulating with this technology. A recovery checkpoint is saved first.",
        )
        .size(DialogSize::Transaction)
        .ghost("Cancel")
        .primary_enabled(ready)
        .destructive();
    let choice =
        technology_change_hint(dialog, pending, validation_error.as_deref()).show(ctx, |ui| {
            property_row(
                ui,
                "Technology",
                label
                    .as_deref()
                    .unwrap_or(crate::workbench::project::dependency_status::UNSET),
            );
            if let Some(demand) = demand.as_deref() {
                ui.add_space(8.0);
                problem_line(
                    ui,
                    Problem::Warning,
                    &format!("{demand} Those runs are refused until another is attached."),
                );
            }
            ui.add_space(10.0);
            technology_change_fields(ui, &mut cx.state.technology_attachment, !pending);
        });
    match choice {
        DialogChoice::Primary => {
            let started = technology_change_authority(&cx.state.technology_attachment).and_then(
                |authority| start_technology_change(ctx, cx, TechnologyChange::Detach, authority),
            );
            if let Err(error) = started {
                cx.state.technology_attachment.validation_error = Some(error);
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            if !pending {
                cx.state.technology_attachment.close();
            }
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

/// A technology dialog's one line beside its buttons, where it stays in view
/// however far the body is scrolled: the checkpoint being written, else why
/// the last attempt was refused.
pub(super) fn technology_change_hint<'a>(
    dialog: Dialog<'a>,
    pending: bool,
    error: Option<&str>,
) -> Dialog<'a> {
    if pending {
        dialog.hint(TECHNOLOGY_CHECKPOINT_PENDING)
    } else if let Some(error) = error {
        dialog
            .hint(error)
            .hint_tone(crate::ui::widgets::DialogHintTone::Error)
    } else {
        dialog
    }
}

/// Attach `binding`: proven against its pinned sources and the reviewed
/// migration, then committed once the checkpoint before it is written.
/// `Some` is the receipt of a change that had nothing to do; `None` says
/// the checkpoint is being written.
pub(super) fn attach_technology_binding(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    binding: ProjectTechnologyBinding,
    authority: crate::state::ProjectTechnologyChangeAuthority,
    migration_evidence: Option<crate::state::pdk_config::PdkTechnologyMigrationEvidence>,
) -> Result<Option<String>, String> {
    verify_pinned_technology_contract(&binding, cx.model)?;
    validate_migration_evidence_for_binding(cx.model, &binding, migration_evidence.as_ref())?;
    let label = binding.display_label();
    if cx.model.workspace.content.project.technology_binding() == Some(&binding)
        && cx.model.workspace.content.project.technology.as_deref() == Some(label.as_str())
    {
        return Ok(Some(format!(
            "{} is already attached at project revision {}; nothing was changed.",
            label,
            cx.model.workspace.content.project.revision().get()
        )));
    }
    start_technology_change(
        ctx,
        cx,
        TechnologyChange::Attach(Box::new(TechnologyAttachment {
            binding,
            migration_evidence,
        })),
        authority,
    )?;
    Ok(None)
}

/// Start `change`: the whole project is read off the model here, and its
/// checkpoint written and read back off the frame, the browser's in its
/// store and the desktop's on a background thread. The change commits on
/// the first frame after the checkpoint is in.
pub(super) fn start_technology_change(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    change: TechnologyChange,
    authority: crate::state::ProjectTechnologyChangeAuthority,
) -> Result<(), String> {
    if cx.state.technology_attachment.in_flight.is_some() {
        return Err("A project recovery checkpoint is already being written".to_owned());
    }
    let project_id = cx.model.workspace.content.project.id().to_string();
    let expected_revision = cx.model.workspace.content.project.revision().get();
    let reason =
        crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointReason::TechnologyChange;
    #[cfg(not(target_arch = "wasm32"))]
    let job = {
        use crate::workbench::app::dialogs::project::{
            CheckpointJob, CheckpointJobAnswer, CheckpointJobKind,
        };
        let capture = crate::workbench::lifecycle::project_checkpoint::capture(
            &crate::workbench::project::lifecycle_view(cx.model, cx.platform, cx.wb, &cx.peers),
            reason,
        )?;
        CheckpointJob::start(
            ctx,
            CheckpointJobKind::Create,
            &cx.state.project_checkpoint_recovery,
            move || CheckpointJobAnswer::Created(Box::new(capture.publish())),
        )
    };
    #[cfg(target_arch = "wasm32")]
    {
        let queued_project_id = project_id.clone();
        let repaint = ctx.clone();
        crate::workbench::lifecycle::project_checkpoint::start_create(
            &crate::workbench::project::lifecycle_view(cx.model, cx.platform, cx.wb, &cx.peers),
            reason,
            move |result| {
                BROWSER_TECHNOLOGY_CHECKPOINT_COMPLETIONS.with(|queue| {
                    queue
                        .borrow_mut()
                        .push_back(BrowserTechnologyCheckpointCompletion {
                            project_id: queued_project_id,
                            result,
                        });
                });
                repaint.request_repaint();
            },
        )?;
    }
    // Stamped on the frame the checkpoint read the project on.
    let captured = crate::workbench::lifecycle::project_lifecycle::working_stamp(
        &crate::workbench::project::lifecycle_view(cx.model, cx.platform, cx.wb, &cx.peers),
    );
    let dialog = &mut cx.state.technology_attachment;
    dialog.in_flight = Some(TechnologyChangeInFlight {
        project_id,
        expected_revision,
        change,
        authority,
        captured,
        #[cfg(not(target_arch = "wasm32"))]
        job,
    });
    dialog.validation_error = None;
    Ok(())
}

/// Take the checkpoint a technology change waits on once it is written, and
/// commit the change against it. A project that changed meanwhile keeps its
/// technology, and the dialog says why.
pub(super) fn poll_technology_change(cx: &mut Cx<'_, ProjectFeature>) {
    #[cfg(not(target_arch = "wasm32"))]
    let written = {
        use crate::workbench::app::dialogs::project::CheckpointJobAnswer;
        let Some(in_flight) = cx.state.technology_attachment.in_flight.as_ref() else {
            return;
        };
        let Some(answer) = in_flight.job.take_answer() else {
            return;
        };
        match answer {
            CheckpointJobAnswer::Created(created) => (*created).map(|created| {
                if let Some(warning) = created.retention_warning {
                    cx.model
                        .record(LogSeverity::Warning, LogSource::Studio, warning);
                }
                created.summary
            }),
            _ => Err("The recovery checkpoint was not written".to_owned()),
        }
    };
    #[cfg(target_arch = "wasm32")]
    let written = {
        let completions = BROWSER_TECHNOLOGY_CHECKPOINT_COMPLETIONS
            .with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
        let Some(in_flight) = cx.state.technology_attachment.in_flight.as_ref() else {
            return;
        };
        let Some(completion) = completions
            .into_iter()
            .rev()
            .find(|completion| completion.project_id == in_flight.project_id)
        else {
            return;
        };
        completion.result
    };
    let Some(in_flight) = cx.state.technology_attachment.in_flight.take() else {
        return;
    };
    let project = &cx.model.workspace.content.project;
    let result = if project.id().to_string() != in_flight.project_id
        || project.revision().get() != in_flight.expected_revision
    {
        Err(
            "The project changed while its recovery checkpoint was being written; its technology was not changed"
                .to_owned(),
        )
    } else {
        written.and_then(|checkpoint| {
            commit_technology_change(
                cx,
                in_flight.change,
                in_flight.authority,
                &checkpoint,
                &in_flight.captured,
            )
        })
    };
    match result {
        Ok(receipt) => {
            cx.state.technology_attachment.close();
            cx.model
                .record(LogSeverity::Info, LogSource::Studio, receipt);
        }
        Err(error) => cx.state.technology_attachment.validation_error = Some(error),
    }
}

/// Commit `change` as one project revision whose technology receipt names
/// `checkpoint`, which must still hold the project as it is: the project
/// `captured` stamped when the checkpoint read it.
pub(super) fn commit_technology_change(
    cx: &mut Cx<'_, ProjectFeature>,
    change: TechnologyChange,
    authority: crate::state::ProjectTechnologyChangeAuthority,
    checkpoint: &crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
    captured: &crate::workbench::lifecycle::project_lifecycle::WorkingStamp,
) -> Result<String, String> {
    let context = technology_change_context(cx, authority, checkpoint, captured)?;
    match change {
        TechnologyChange::Attach(attachment) => commit_technology_after_checkpoint(
            cx,
            attachment.binding,
            context,
            attachment.migration_evidence,
            checkpoint,
        ),
        TechnologyChange::Detach => commit_detach_after_checkpoint(cx, context, checkpoint),
    }
}

/// Remove the project's technology over `checkpoint`, as one project
/// revision whose receipt names who did it and the checkpoint.
fn commit_detach_after_checkpoint(
    cx: &mut Cx<'_, ProjectFeature>,
    context: crate::state::ProjectTechnologyChangeContext,
    checkpoint: &crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
) -> Result<String, String> {
    let label = cx
        .model
        .workspace
        .content
        .project
        .technology
        .clone()
        .unwrap_or_default();
    let (revision, audit_receipt) = cx
        .model
        .workspace
        .content
        .detach_technology_audited(context)
        .map_err(|error| format!("The technology was not detached: {error}"))?;
    cx.state.project_checkpoint_recovery.invalidate();
    Ok(format!(
        "Detached {label} at project revision {} (technology receipt #{}); checkpoint {} keeps revision {}.",
        revision.get(),
        audit_receipt.sequence(),
        short_identity(&checkpoint.checkpoint_id().to_string()),
        checkpoint.project_revision(),
    ))
}

/// The receipt's evidence for a change made over `checkpoint`, which must
/// still hold the project exactly as it is. The project is compared by the
/// stamps its owners issue, not by serialising it again: the frame stays
/// free, and a catalogue whose maps iterate in another order is still the
/// same project.
fn technology_change_context(
    cx: &Cx<'_, ProjectFeature>,
    authority: crate::state::ProjectTechnologyChangeAuthority,
    checkpoint: &crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
    captured: &crate::workbench::lifecycle::project_lifecycle::WorkingStamp,
) -> Result<crate::state::ProjectTechnologyChangeContext, String> {
    let current = crate::workbench::lifecycle::project_lifecycle::working_stamp(
        &crate::workbench::project::lifecycle_view(cx.model, cx.platform, cx.wb, &cx.peers),
    );
    if checkpoint.project_revision() != cx.model.workspace.content.project.revision().get()
        || checkpoint.project_name() != cx.model.workspace.content.project.name()
        || current != *captured
    {
        return Err(
            "The project changed after its recovery checkpoint was captured; its technology was not changed"
                .to_owned(),
        );
    }
    let checkpoint_revision = crate::product::ObjectRevision::new(checkpoint.project_revision())
        .map_err(|error| format!("Recovery checkpoint revision is invalid: {error}"))?;
    crate::state::ProjectTechnologyChangeContext::new(
        authority,
        checkpoint.checkpoint_id(),
        checkpoint_revision,
        checkpoint.created_unix_ms(),
        checkpoint.snapshot_digest(),
        checkpoint.snapshot_byte_len(),
    )
    .map_err(|error| format!("Technology change authority is invalid: {error}"))
}

fn commit_technology_after_checkpoint(
    cx: &mut Cx<'_, ProjectFeature>,
    binding: ProjectTechnologyBinding,
    mut change_context: crate::state::ProjectTechnologyChangeContext,
    migration_evidence: Option<crate::state::pdk_config::PdkTechnologyMigrationEvidence>,
    checkpoint: &crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
) -> Result<String, String> {
    verify_pinned_technology_contract(&binding, cx.model)?;
    validate_migration_evidence_for_binding(cx.model, &binding, migration_evidence.as_ref())?;
    let previous_revision = cx.model.workspace.content.project.revision();
    if let Some(evidence) = migration_evidence {
        change_context = change_context
            .with_migration_evidence(evidence)
            .map_err(|error| format!("The reviewed technology migration is invalid: {error}"))?;
    }
    let (revision, audit_receipt) = cx
        .model
        .workspace
        .content
        .attach_technology_audited(binding.clone(), change_context)
        .map_err(|error| format!("Technology attachment was not committed: {error}"))?;
    if revision == previous_revision {
        return Err("Technology attachment unexpectedly produced no project revision".to_owned());
    }
    cx.state.project_checkpoint_recovery.invalidate();
    Ok(format!(
        "Attached {} at project revision {} (technology receipt #{}, {} source file{}); checkpoint {} keeps revision {}.",
        binding.display_label(),
        revision.get(),
        audit_receipt.sequence(),
        binding.source_closure().len(),
        plural_suffix(binding.source_closure().len()),
        short_identity(&checkpoint.checkpoint_id().to_string()),
        checkpoint.project_revision(),
    ))
}

pub(super) fn technology_checkpoint_pending(
    dialogs: &crate::workbench::slots::ProjectState,
) -> bool {
    dialogs.technology_attachment.in_flight.is_some()
}

#[cfg(target_arch = "wasm32")]
pub(super) fn poll_browser_recovery_completions(_: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    let project_id = cx.model.workspace.content.project.id().to_string();
    let catalog = BROWSER_RECOVERY_CATALOG_COMPLETIONS
        .with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in catalog {
        if completion.project_id != project_id {
            continue;
        }
        let recovery = &mut cx.state.project_checkpoint_recovery;
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
                cx.model.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!("Downloaded a recovery copy as {}.", completion.filename),
                );
            }
            Err(error) => {
                cx.model
                    .record(LogSeverity::Error, LogSource::Studio, error);
            }
        }
    }
}

/// Prove `binding` against the catalogue it was made from. The bytes the
/// catalogue retains are proven against the pins on every platform; a
/// library read from files on this machine is also proven against those
/// files, which may have changed since they were parsed. A library whose
/// bytes the project retains or owns is not: the files it was imported from
/// may be on another machine, or gone.
pub(super) fn verify_pinned_model_sources(
    binding: &ProjectTechnologyBinding,
    manager: &ModelLibraryManager,
) -> Result<(), String> {
    binding
        .validate()
        .map_err(|error| format!("Technology contract is invalid: {error}"))?;
    manager
        .catalog()
        .validate_attached_technology(Some(binding))?;
    #[cfg(not(target_arch = "wasm32"))]
    if manager
        .get_library(binding.model_library())
        .is_some_and(|library| {
            matches!(
                library.source_authority,
                crate::state::model_library::ModelSourceAuthority::External
            )
        })
    {
        for source in binding.source_closure() {
            let observed = ModelLibraryManager::calculate_source_digest(&source.path)?;
            if observed != source.digest {
                return Err(format!(
                    "Model source '{}' changed after it was parsed. Refresh the model library before attaching it.",
                    source.path.display()
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn verify_pinned_technology_contract(
    binding: &ProjectTechnologyBinding,
    model: &Model,
) -> Result<(), String> {
    verify_pinned_model_sources(binding, &model.model_library_manager)?;
    model
        .pdk_config
        .technology_registry
        .validate_project_binding(binding)
        .map_err(|error| format!("PDK contract is unavailable: {error}"))
}

fn validate_migration_evidence_for_binding(
    model: &Model,
    binding: &ProjectTechnologyBinding,
    evidence: Option<&crate::state::pdk_config::PdkTechnologyMigrationEvidence>,
) -> Result<(), String> {
    let baseline = model.project_pdk_package()?;
    let candidate_pin = binding
        .pdk_package()
        .ok_or_else(|| "The candidate binding has no PDK package pin.".to_owned())?;
    let candidate = model
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
        .ok_or_else(|| "Candidate PDK no longer matches the validated archive.".to_owned())?;
    let Some(baseline) = baseline else {
        if evidence.is_some() {
            return Err(
                "A first technology attachment cannot include a migration review.".to_owned(),
            );
        }
        return Ok(());
    };
    if baseline.binding() == candidate.binding()
        && baseline.archive_digest() == candidate.archive_digest()
    {
        if evidence.is_some() {
            return Err(
                "The project already uses this PDK package; there is no migration to review."
                    .to_owned(),
            );
        }
        return Ok(());
    }
    let diff = crate::state::pdk_config::PdkTechnologyRevisionDiff::between(
        baseline.metadata(),
        candidate.metadata(),
    )
    .map_err(|error| format!("Revision comparison failed: {error}"))?;
    if !diff.same_package_lineage {
        return Err(
            "Cross-technology replacement requires an independent migration copy; direct replacement is prohibited."
                .to_owned(),
        );
    }
    if diff.has_breaking_changes() {
        return Err(format!(
            "The PDK revision changes {} breaking contract(s); direct replacement is prohibited until those objects are explicitly remapped in an independent migration copy.",
            diff.count(crate::state::pdk_config::PdkTechnologyDiffImpact::Breaking)
        ));
    }
    let evidence = evidence
        .ok_or_else(|| "Replacing a PDK revision requires a reviewed comparison.".to_owned())?;
    if !evidence.matches_diff(&diff) {
        return Err(
            "The reviewed comparison no longer matches the validated baseline and candidate packages."
                .to_owned(),
        );
    }
    Ok(())
}

/// What attaching a technology does to the project, before it is done.
///
/// The mark carries the severity; the sentence says the consequence and the
/// safeguard. It used to be three clauses of implementation — which bytes are
/// authenticated, what is read back, what is not migrated — behind a warning
/// wash that made a routine dialog look like a failure.
pub(super) fn technology_warning(ui: &mut Ui) {
    problem_line(
        ui,
        Problem::Warning,
        "Attaching a technology changes how this project simulates. A recovery checkpoint is saved first.",
    );
    ui.add_space(10.0);
}

pub(super) fn technology_dialog_label(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_faint),
    );
    ui.add_space(3.0);
}

pub(super) fn technology_unavailable_message(ui: &mut Ui) {
    property_card(ui, "No attachable model technology", |ui| {
        muted(
            ui,
            "Configure and parse at least one external model library with a root source file and one device model.",
        );
    });
}

/// What the reader still has to supply before this attachment can commit.
///
/// Seven dotted rows stood here: five internal checks the dialog reported
/// green the moment its own fields were filled in, and two disclaimers about
/// capabilities the product never claimed. None of the seven was something a
/// reader could act on, and the ones that were — a library, a package, a
/// mapping, an authority — are the dialog's own four controls. So this states
/// only the conditions that are unmet, in the order their controls appear, and
/// nothing at all once they are met.
pub(super) fn technology_missing_inputs(
    has_model_candidate: bool,
    has_pdk_package: bool,
    contract_ready: bool,
    authority_ready: bool,
) -> Vec<&'static str> {
    let mut missing = Vec::new();
    if !has_model_candidate {
        missing.push("a model library");
    }
    if !has_pdk_package {
        missing.push("a validated PDK revision");
    }
    if !authority_ready {
        missing.push("who changes it and who approves it");
    }
    if !contract_ready {
        missing.push("a technology mapping");
    }
    missing
}

/// [`technology_missing_inputs`] as the one line the dialog paints.
pub(super) fn technology_binding_gates(
    ui: &mut Ui,
    has_model_candidate: bool,
    has_pdk_package: bool,
    contract_ready: bool,
    authority_ready: bool,
) {
    let missing = technology_missing_inputs(
        has_model_candidate,
        has_pdk_package,
        contract_ready,
        authority_ready,
    );
    if missing.is_empty() {
        return;
    }
    muted(ui, &format!("Still to choose: {}.", missing.join(", ")));
}

/// Who makes a technology change, who approves it and why, each a labelled
/// field: the first two start on the person at this application, and the
/// reason may be left empty. All three go into the project's technology
/// history with the checkpoint.
pub(super) fn technology_change_fields(
    ui: &mut Ui,
    dialog: &mut crate::workbench::app::TechnologyAttachmentDialogState,
    enabled: bool,
) {
    let width = ui.available_width();
    for (label, value, limit) in [
        ("Changed by", &mut dialog.actor_id, 240),
        ("Approved by", &mut dialog.authority_id, 240),
    ] {
        field_label(ui, label, |ui| {
            let response = ui
                .add_enabled_ui(enabled, |ui| {
                    text_field(
                        ui,
                        egui::TextEdit::singleline(value).char_limit(limit),
                        width,
                    )
                })
                .inner;
            name_control(ui, &response, label);
        });
        ui.add_space(8.0);
    }
    field_label(ui, "Reason (optional)", |ui| {
        let response = ui.add_enabled(
            enabled,
            egui::TextEdit::multiline(&mut dialog.reason)
                .char_limit(1_024)
                .desired_rows(2)
                .desired_width(width),
        );
        name_control(ui, &response, "Reason");
    });
    ui.add_space(4.0);
    muted(
        ui,
        "Both names, the reason and the checkpoint are kept in the project's technology history.",
    );
}

fn technology_authority_fields_ready(
    dialog: &crate::workbench::app::TechnologyAttachmentDialogState,
) -> bool {
    !dialog.actor_id.trim().is_empty() && !dialog.authority_id.trim().is_empty()
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

/// A refusal this dialog has to state.
///
/// The failure mark is the line's only colour: the error wash behind error
/// text painted the same fact three times, and a dialog with two of them
/// stacked read as a page in failure rather than a field that needs a choice.
pub(super) fn technology_error(ui: &mut Ui, error: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel_2)
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            problem_line(ui, Problem::Failure, error);
        });
}
