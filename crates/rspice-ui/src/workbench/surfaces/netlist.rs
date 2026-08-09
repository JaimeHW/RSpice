//! Mockup-owned Code & Automation netlist document surface.
//!
//! The center well is deliberately flat: one 33-point document toolbar over
//! an exact-entry editor. Generated and owned source are independent retained
//! documents and switching between them never deletes either one.

use egui::{Align, Layout, Ui, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::{AppState, MessageCatalog, MessageId, RSpiceApp};

use super::super::design_system::{WorkbenchIcon, empty_state, icon_button};
use crate::state::{
    GeneratedArtifact, GeneratedProvenance, GeneratedSourceMapEntry, GenerationInput,
    NetlistDocument, NetlistDocumentId,
};
use crate::workbench::documents::netlist_document::{ActiveNetlistDocument, source_content_digest};

const CODE_TOOLBAR_HEIGHT: f32 = 33.0;
const CODE_TOOLBAR_PADDING_X: f32 = 8.0;
const CODE_TOOLBAR_GAP: f32 = 5.0;
const CODE_TOOLBAR_ACTION_GUTTER: f32 = 12.0;
const CODE_TOOLBAR_COMPACT_BREAKPOINT: f32 = 720.0;
const PHONE_BREAKPOINT: f32 = 560.0;
const PHONE_PRIMARY_WIDTH: f32 = 154.0;
const EDITOR_MENU_WIDTH: f32 = 58.0;
const PHONE_ACTION_WIDTH: f32 =
    PHONE_PRIMARY_WIDTH + CODE_TOOLBAR_GAP * 2.0 + 28.0 + EDITOR_MENU_WIDTH;

pub(super) fn prepare_workspace(app: &mut RSpiceApp) {
    reconcile_documents(app);
    crate::workbench::documents::netlist_document::prepare(&mut app.state);
}

pub(super) fn show_prepared(ui: &mut Ui, app: &mut RSpiceApp) {
    handle_netlist_file_drop(ui.ctx(), app);
    code_toolbar(ui, app);
    execution_profile_review_banner(ui, app);
    if crate::workbench::documents::text_editor_commands::take_format_document_request(
        ui,
        crate::workbench::documents::netlist_document::editor_id(),
    ) {
        format_owned_netlist(ui.ctx(), app);
    }
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        if generated_primary_unavailable(&app.state) {
            let messages = app.state.ui.messages();
            let title = messages.text(MessageId::NetlistGeneratedUnavailable);
            let default_description =
                messages.text(MessageId::NetlistGeneratedUnavailableDescription);
            empty_state(
                ui,
                WorkbenchIcon::Netlist,
                &title,
                app.state
                    .ui
                    .netlist
                    .generation_error
                    .as_deref()
                    .unwrap_or(&default_description),
            );
        } else {
            crate::workbench::documents::netlist_document::show_editor(ui, &mut app.state);
        }
    });
    find_replace_window(ui.ctx(), app);
    ownership_dialog_window(ui.ctx(), app);
    comparison_dialog_window(ui.ctx(), app);
    save_source_dialog_window(ui.ctx(), app);
    external_change_dialog_window(ui.ctx(), app);
    export_generated_dialog_window(ui.ctx(), app);
    import_review_dialog_window(ui.ctx(), app);
}

fn execution_profile_review_banner(ui: &mut Ui, app: &mut RSpiceApp) {
    let Some(descriptor) = app
        .state
        .workspace
        .netlist_descriptor
        .as_ref()
        .filter(|descriptor| descriptor.execution_profile_review_required())
    else {
        return;
    };
    let dialect = descriptor
        .imported_dialect
        .unwrap_or(crate::state::NetlistSourceDialect::RSpice);
    let messages = app.state.ui.messages();
    let description = messages.format(
        MessageId::NetlistProfileReviewRequiredDescription,
        &[("dialect", dialect.label())],
    );
    let t = Tokens::get(ui.ctx());
    let mut review = false;
    egui::Frame::new()
        .fill(t.color.warn.gamma_multiply(0.10))
        .stroke(egui::Stroke::new(1.0, t.color.warn.gamma_multiply(0.65)))
        .inner_margin(8)
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new(messages.text(MessageId::NetlistProfileReviewRequired))
                        .strong()
                        .color(t.color.warn),
                );
                ui.label(description);
                review = ui
                    .button(messages.text(MessageId::NetlistProfileReviewAction))
                    .clicked();
            });
        });
    if review {
        crate::workbench::workflows::netlist_workflow::begin_owned_netlist_profile_review(
            &mut app.state,
        );
    }
}

fn handle_netlist_file_drop(ctx: &egui::Context, app: &mut RSpiceApp) {
    if app.state.application_modal_open() {
        return;
    }
    let dropped = ctx.input(|input| input.raw.dropped_files.clone());
    if dropped.is_empty() {
        return;
    }
    if dropped.len() != 1 {
        app.state.push_user_message(ConsoleMessage::warning(
            "Drop one SPICE deck or RSpice netlist bundle at a time so the staged import review has one exact source identity.",
        ));
        return;
    }
    let file = &dropped[0];
    let source_path = file.path.clone();
    let display_name = (!file.name.trim().is_empty())
        .then(|| file.name.clone())
        .or_else(|| {
            source_path
                .as_deref()
                .and_then(std::path::Path::file_name)
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "dropped-netlist.spice".to_owned());
    let bytes = if let Some(bytes) = file.bytes.as_ref() {
        Ok(bytes.to_vec())
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            source_path
                .as_deref()
                .ok_or_else(|| "Dropped file has neither bytes nor a native path.".to_owned())
                .and_then(|path| std::fs::read(path).map_err(|error| error.to_string()))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Err("Browser drop did not provide immutable file bytes.".to_owned())
        }
    };
    match bytes {
        Ok(bytes) => {
            crate::workbench::workflows::netlist_workflow::stage_dropped_netlist_import(
                &mut app.state,
                bytes,
                source_path,
                display_name,
            );
        }
        Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
            "Dropped SPICE source could not be read: {error}"
        ))),
    }
}

fn format_owned_netlist(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !crate::workbench::documents::netlist_document::active_netlist_source_is_editable(&app.state)
    {
        return;
    }
    let source = app.state.simulation.netlist_content.clone();
    let dependency_document = app.state.ui.netlist.active_dependency_identity.is_some();
    if dependency_document {
        let has_errors = app.state.ui.netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.is_current()
                && diagnostic.severity
                    == crate::workbench::documents::netlist_document::DiagnosticSeverity::Error
        });
        if has_errors {
            app.state.push_user_message(ConsoleMessage::error(
                "Include formatting is blocked until the current document has no syntax errors.",
            ));
            return;
        }
        let formatted = normalize_owned_netlist_whitespace(&source);
        if formatted == source {
            app.state.push_user_message(ConsoleMessage::info(
                "The owned include already matches the deterministic source format.",
            ));
        } else if crate::workbench::documents::netlist_document::replace_owned_dependency_source(
            &mut app.state,
            formatted,
        ) {
            app.state.push_user_message(ConsoleMessage::info(
                "Formatted the exact project-owned include revision; root-deck validation was invalidated.",
            ));
        }
        return;
    }
    let digest = source_content_digest(&source);
    let validation_current = app
        .state
        .ui
        .netlist
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == digest
                && receipt.project_revision == app.state.workspace.project.revision().get()
        });
    if !validation_current {
        crate::workbench::workflows::netlist_workflow::validate_visible_netlist_source(app);
    }
    let validation_current = app
        .state
        .ui
        .netlist
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == digest
                && receipt.project_revision == app.state.workspace.project.revision().get()
        });
    if !validation_current {
        let message = app
            .state
            .ui
            .netlist
            .validation_error
            .clone()
            .unwrap_or_else(|| {
                "Formatting is blocked until the exact owned source passes executable validation."
                    .to_owned()
            });
        app.state
            .push_user_message(ConsoleMessage::error(message.clone()));
        app.state
            .ui
            .toasts
            .error_with_title(ctx, "Netlist format blocked", message);
        return;
    }

    let formatted = normalize_owned_netlist_whitespace(&source);
    if formatted == source {
        app.state.push_user_message(ConsoleMessage::info(
            "The owned netlist already matches the deterministic source format.",
        ));
        return;
    }
    if crate::workbench::documents::netlist_document::replace_owned_source(
        &mut app.state,
        formatted,
    ) {
        app.state.push_user_message(ConsoleMessage::info(
            "Formatted the exact owned netlist revision; prior validation was invalidated.",
        ));
    } else {
        app.state.push_user_message(ConsoleMessage::error(
            "The owned netlist changed before formatting could commit. Review the current revision and retry.",
        ));
    }
}

fn normalize_owned_netlist_whitespace(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let preferred_eol = if source.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let mut formatted = String::with_capacity(source.len().saturating_add(preferred_eol.len()));
    for (line, segment) in source.split_inclusive('\n').enumerate() {
        let (body, eol) = segment.strip_suffix("\r\n").map_or_else(
            || {
                segment
                    .strip_suffix('\n')
                    .map_or((segment, ""), |body| (body, "\n"))
            },
            |body| (body, "\r\n"),
        );
        // The first physical card is the circuit title and is user data, not
        // executable whitespace. Preserve it byte-for-byte.
        if line == 0 {
            formatted.push_str(body);
        } else {
            formatted.push_str(body.trim_end_matches([' ', '\t']));
        }
        formatted.push_str(eol);
    }
    if !source.ends_with('\n') {
        formatted.push_str(preferred_eol);
    }
    formatted
}

fn generated_primary_unavailable(state: &AppState) -> bool {
    state.ui.netlist.active_dependency_identity.is_none()
        && state.ui.netlist.active_document == ActiveNetlistDocument::Generated
        && !generated_primary_ready(state)
}

fn generated_primary_ready(state: &AppState) -> bool {
    state.ui.netlist.generated_document.is_some() && !state.ui.netlist.generated_source.is_empty()
}

fn active_document_available(state: &AppState) -> bool {
    if state.ui.netlist.active_dependency_identity.is_some() {
        return crate::workbench::documents::netlist_document::active_dependency(state).is_some();
    }
    match state.ui.netlist.active_document {
        ActiveNetlistDocument::Generated => generated_primary_ready(state),
        ActiveNetlistDocument::OwnedSource => state.workspace.netlist_source.is_some(),
        ActiveNetlistDocument::GeneratedDiff => !state.ui.netlist.generated_diff_source.is_empty(),
    }
}

fn generation_block_reason(state: &AppState) -> String {
    state
        .ui
        .netlist
        .generation_error
        .clone()
        .unwrap_or_else(|| {
            state
                .ui
                .messages()
                .text(MessageId::NetlistGenerateBeforeAction)
        })
}

/// Rebuild the immutable generated primary whenever any authoritative project
/// input changes. Generation happens into a temporary string; a failed build
/// retains the prior valid artifact and marks it stale instead of publishing
/// partial bytes.
fn reconcile_documents(app: &mut RSpiceApp) {
    if app.state.ui.netlist.owned_document.is_none() {
        app.state.ui.netlist.owned_document = app.state.workspace.netlist_document.clone();
    }
    if !app.state.ui.netlist.active_document_initialized {
        app.state.ui.netlist.active_document = if app.state.workspace.netlist_source.is_some() {
            ActiveNetlistDocument::OwnedSource
        } else {
            ActiveNetlistDocument::Generated
        };
        app.state.ui.netlist.active_document_initialized = true;
    }

    if app.state.ui.netlist.active_dependency_identity.is_some()
        && crate::workbench::documents::netlist_document::active_dependency(&app.state).is_none()
    {
        crate::workbench::documents::netlist_document::close_active_dependency(&mut app.state);
        app.state.push_user_message(ConsoleMessage::warning(
            "The previously open dependency is no longer in the canonical source closure; its root document was restored.",
        ));
    }

    refresh_generated_artifact(app);

    let projected = crate::workbench::documents::netlist_document::active_dependency(&app.state)
        .and_then(crate::state::DependencyMetadata::source)
        .map(str::to_owned)
        .or_else(|| match app.state.ui.netlist.active_document {
            ActiveNetlistDocument::Generated => Some(app.state.ui.netlist.generated_source.clone()),
            ActiveNetlistDocument::OwnedSource => app.state.workspace.netlist_source.clone(),
            ActiveNetlistDocument::GeneratedDiff => {
                Some(app.state.ui.netlist.generated_diff_source.clone())
            }
        });
    if let Some(projected) = projected
        && app.state.simulation.netlist_content != projected
    {
        app.state.simulation.netlist_content = projected;
        app.state.ui.netlist.revision = app.state.ui.netlist.revision.wrapping_add(1);
        crate::workbench::documents::netlist_document::invalidate_source_evidence(
            &mut app.state.ui.netlist,
        );
    }
}

fn refresh_generated_artifact(app: &mut RSpiceApp) {
    let input_digest =
        match crate::workbench::lifecycle::project_lifecycle::generated_netlist_input_digest(
            &app.state,
        ) {
            Ok(digest) => digest,
            Err(error) => {
                app.state.ui.netlist.current_generation_input_digest = None;
                app.state.ui.netlist.generation_error = Some(error.to_string());
                return;
            }
        };
    let authoritative_input_changed = app
        .state
        .ui
        .netlist
        .current_generation_input_digest
        .is_some_and(|previous| previous != input_digest);
    app.state.ui.netlist.current_generation_input_digest = Some(input_digest);
    if authoritative_input_changed {
        app.invalidate_simulation_preflight();
        crate::workbench::documents::netlist_document::invalidate_source_evidence(
            &mut app.state.ui.netlist,
        );
        if let Some(document) = app.state.ui.netlist.generated_document.as_mut() {
            let _ = document.invalidate_validation(document.content_digest());
        }
        if let Some(document) = app.state.ui.netlist.owned_document.as_mut() {
            let _ = document.invalidate_validation(document.content_digest());
            app.state.workspace.netlist_document = Some(document.clone());
        }
    }
    if app.state.ui.netlist.generated_input_digest == Some(input_digest)
        && !app.state.ui.netlist.generated_source.is_empty()
    {
        return;
    }
    if app.state.schematic.components.is_empty() {
        app.state.ui.netlist.generation_error =
            Some("Add a circuit before generating the primary netlist.".to_owned());
        return;
    }

    let previous_message_count = app.state.log_buffer.len();
    let generated = crate::workbench::menu_bar::build_menu_netlist(
        &mut app.state,
        crate::io::NetlistFormat::Spice,
    );
    match generated {
        Some(source) => match publish_generated_document(&app.state, input_digest, source) {
            Ok((generated_document, owned_document)) => {
                if let Some(previous) = app.state.ui.netlist.generated_document.as_ref()
                    && previous.generated_artifact().content_digest()
                        != generated_document.generated_artifact().content_digest()
                {
                    let predecessor = previous.generated_artifact().clone();
                    if app
                        .state
                        .ui
                        .netlist
                        .generated_history
                        .last()
                        .is_none_or(|artifact| {
                            artifact.content_digest() != predecessor.content_digest()
                        })
                    {
                        app.state.ui.netlist.generated_history.push(predecessor);
                        const RETAINED_GENERATED_REVISIONS: usize = 16;
                        let excess = app
                            .state
                            .ui
                            .netlist
                            .generated_history
                            .len()
                            .saturating_sub(RETAINED_GENERATED_REVISIONS);
                        if excess > 0 {
                            app.state.ui.netlist.generated_history.drain(..excess);
                        }
                    }
                }
                app.state.ui.netlist.generated_source =
                    generated_document.generated_artifact().source().to_owned();
                app.state.ui.netlist.generated_document = Some(generated_document);
                app.state.workspace.netlist_document = owned_document.clone();
                app.state.ui.netlist.owned_document = owned_document;
                app.state.ui.netlist.generated_input_digest = Some(input_digest);
                app.state.ui.netlist.generation_error = None;
            }
            Err(error) => {
                app.state.ui.netlist.generation_error = Some(error);
            }
        },
        None => {
            let detail = app
                .state
                .log_buffer
                .entries()
                .skip(previous_message_count)
                .filter(|entry| entry.severity == crate::diagnostics::LogSeverity::Error)
                .last()
                .map(|entry| entry.message.clone())
                .unwrap_or_else(|| "Netlist generation failed; review Problems.".to_owned());
            app.state.ui.netlist.generation_error = Some(detail);
        }
    }
}

fn publish_generated_document(
    state: &AppState,
    input_digest: crate::product::ContentDigest,
    source: String,
) -> Result<(NetlistDocument, Option<NetlistDocument>), String> {
    let provenance = GeneratedProvenance::try_new(
        "rspice-netlist-generator/v1",
        GenerationInput::new(state.workspace.project.revision(), input_digest),
    )
    .map_err(|error| error.to_string())?;
    let source_map = generated_source_map(state, &source)?;
    let dependencies = generated_project_source_dependencies(state, &source)?;
    let artifact =
        GeneratedArtifact::try_from_utf8(provenance, source.into_bytes(), dependencies, source_map)
            .map_err(|error| error.to_string())?;

    let generated_document = if let Some(existing) = &state.ui.netlist.generated_document {
        let mut next = existing.clone();
        next.update_generated_artifact(
            existing.generated_artifact().content_digest(),
            artifact.clone(),
        )
        .map_err(|error| error.to_string())?;
        next
    } else {
        NetlistDocument::from_generated(NetlistDocumentId::new(), artifact.clone())
            .map_err(|error| error.to_string())?
    };

    // An owned document retains the exact generated base from which it was
    // created. Current generated revisions live in the primary registry and
    // compare history; silently rebasing owned source would destroy the
    // three-way ownership contract.
    let owned_document = state.ui.netlist.owned_document.clone();
    Ok((generated_document, owned_document))
}

fn generated_project_source_dependencies(
    state: &AppState,
    source: &str,
) -> Result<Vec<crate::state::DependencyMetadata>, String> {
    let include_directives = crate::state::parse_include_directives(source);
    let mut dependencies = Vec::new();

    let projection = state
        .workspace
        .configuration_execution_projection(
            &state.library_manager,
            &state.workspace.active_view,
            &state.schematic,
        )
        .map_err(|error| error.to_string())?;
    let Some(plan) = projection.plan() else {
        return Ok(dependencies);
    };
    let mut retained_keys = std::collections::HashSet::new();
    for execution in plan.bindings() {
        let Some(binding) = execution.project_veriloga() else {
            continue;
        };
        if !retained_keys.insert(binding.source_key().to_owned()) {
            continue;
        }
        let bundle = state
            .workspace
            .project_sources
            .get_bundle(binding.source_bundle_id())
            .ok_or_else(|| {
                format!(
                    "Generated Verilog-A binding at {} references missing bundle {}",
                    execution.instance_path(),
                    binding.source_bundle_id()
                )
            })?;
        if bundle.closure_digest() != binding.source_closure_digest() {
            return Err(format!(
                "Generated Verilog-A bundle {} changed after hierarchy resolution",
                bundle.id()
            ));
        }
        let include_index = include_directives
            .iter()
            .position(|directive| directive.locator() == binding.source_key())
            .ok_or_else(|| {
                format!(
                    "Generated source is missing the authenticated Verilog-A binding for {}",
                    execution.instance_path()
                )
            })?;
        let root_locator = crate::state::SourceLocator::try_new(
            binding.source_key(),
            bundle.root().logical_path(),
        )
        .map_err(|error| error.to_string())?;
        let compilation = crate::workbench::documents::code_workspace::compile_project_bundle_virtual_for_provenance(
            bundle,
            binding.selected_module(),
        )
            .map_err(|failure| {
                format!(
                    "Could not authenticate generated Verilog-A provenance for {}: {failure}",
                    execution.instance_path()
                )
            })?;
        let compiled_root = compilation
            .dependency_closure
            .first()
            .filter(|dependency| {
                dependency
                    .logical_path
                    .eq_ignore_ascii_case(bundle.root().logical_path())
            })
            .ok_or_else(|| {
                format!(
                    "Compiler dependency closure for {} has no authenticated root",
                    execution.instance_path()
                )
            })?;
        dependencies.push(
            crate::state::DependencyMetadata::unresolved_direct_to(
                include_index,
                binding.source_key(),
                root_locator.clone(),
            )
            .map_err(|error| error.to_string())?
            .resolve_utf8(compiled_root.source.as_bytes().to_vec())
            .map_err(|error| error.to_string())?,
        );
        for edge in &compilation.include_graph {
            let parent = compilation
                .dependency_closure
                .iter()
                .find(|dependency| {
                    dependency
                        .logical_path
                        .eq_ignore_ascii_case(&edge.including_path)
                })
                .ok_or_else(|| {
                    format!(
                        "Compiler include parent '{}' is absent from the authenticated closure",
                        edge.including_path
                    )
                })?;
            let child = compilation
                .dependency_closure
                .iter()
                .find(|dependency| {
                    dependency
                        .logical_path
                        .eq_ignore_ascii_case(&edge.included_path)
                })
                .ok_or_else(|| {
                    format!(
                        "Compiler include target '{}' is absent from the authenticated closure",
                        edge.included_path
                    )
                })?;
            let parent_locator = compiler_dependency_locator(
                binding.source_key(),
                bundle.root().logical_path(),
                parent,
                &root_locator,
            )?;
            let child_locator = compiler_dependency_locator(
                binding.source_key(),
                bundle.root().logical_path(),
                child,
                &root_locator,
            )?;
            dependencies.push(
                crate::state::DependencyMetadata::unresolved_transitive_to(
                    parent_locator,
                    edge.include_index,
                    edge.requested_path.clone(),
                    child_locator,
                )
                .map_err(|error| error.to_string())?
                .resolve_utf8(child.source.as_bytes().to_vec())
                .map_err(|error| error.to_string())?,
            );
        }
    }
    Ok(dependencies)
}

fn project_bundle_locator(
    source_key: &str,
    logical_path: &str,
) -> Result<crate::state::SourceLocator, String> {
    crate::state::SourceLocator::try_new(format!("{source_key}#/{logical_path}"), logical_path)
        .map_err(|error| error.to_string())
}

fn compiler_dependency_locator(
    source_key: &str,
    root_path: &str,
    dependency: &rspice_veriloga::VirtualSourceDependency,
    root_locator: &crate::state::SourceLocator,
) -> Result<crate::state::SourceLocator, String> {
    if dependency.logical_path.eq_ignore_ascii_case(root_path) {
        return Ok(root_locator.clone());
    }
    if dependency.origin == rspice_veriloga::SourceDocumentOrigin::BuiltIn {
        let name = dependency
            .logical_path
            .rsplit('/')
            .next()
            .filter(|name| !name.is_empty())
            .ok_or_else(|| "Compiler returned a built-in dependency without a name".to_owned())?;
        return crate::state::SourceLocator::try_new(
            format!("__rspice_builtin__/veriloga/{name}"),
            dependency.logical_path.clone(),
        )
        .map_err(|error| error.to_string());
    }
    project_bundle_locator(source_key, &dependency.logical_path)
}

fn generated_source_map(
    state: &AppState,
    source: &str,
) -> Result<Vec<GeneratedSourceMapEntry>, String> {
    let top = state.workspace.active_view.clone();
    let mut current = top.clone();
    let mut entries = Vec::with_capacity(source.lines().count());

    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        let tokens = trimmed.split_whitespace().collect::<Vec<_>>();
        if tokens
            .first()
            .is_some_and(|head| head.eq_ignore_ascii_case(".subckt"))
            && let Some(cell) = tokens.get(1)
        {
            current = state
                .workspace
                .schematic_buffers
                .keys()
                .find_map(|key| {
                    let mut segments = key.split('/');
                    let library = segments.next()?;
                    let candidate_cell = segments.next()?;
                    let view = segments.next()?;
                    (candidate_cell.eq_ignore_ascii_case(cell))
                        .then(|| crate::state::CellViewRef::new(library, candidate_cell, view))
                })
                .unwrap_or_else(|| {
                    crate::state::CellViewRef::new(&top.library, *cell, "schematic")
                });
        }

        let (instance_identity, component_identity) = source_line_component(state, &current, line)
            .map_or((None, None), |(instance, component)| {
                (Some(instance), Some(component))
            });
        entries.push(
            GeneratedSourceMapEntry::try_new(
                index + 1,
                format!("{}/{}", current.library, current.cell),
                current.key(),
                instance_identity,
                component_identity,
            )
            .map_err(|error| error.to_string())?,
        );

        if tokens
            .first()
            .is_some_and(|head| head.eq_ignore_ascii_case(".ends"))
        {
            current.clone_from(&top);
        }
    }
    Ok(entries)
}

fn source_line_component(
    state: &AppState,
    reference: &crate::state::CellViewRef,
    line: &str,
) -> Option<(String, String)> {
    let token = line.split_whitespace().next()?;
    if token.starts_with(['.', '*']) {
        return None;
    }
    let schematic = state.workspace.schematic_buffers.get(&reference.key())?;
    schematic.components.iter().find_map(|component| {
        let base = component.spice_instance_name();
        let prefix = component.kind.spice_prefix();
        let emitted = if prefix.is_empty()
            || base.is_empty()
            || (base.len() >= prefix.len() && base[..prefix.len()].eq_ignore_ascii_case(prefix))
        {
            base
        } else {
            format!("{prefix}{base}")
        };
        token.eq_ignore_ascii_case(&emitted).then(|| {
            (
                emitted,
                format!("{}/component/{}", reference.key(), component.id),
            )
        })
    })
}

fn code_toolbar(ui: &mut Ui, app: &mut RSpiceApp) {
    let messages = app.state.ui.messages();
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let compact = code_toolbar_compact(width);
    let phone = width <= PHONE_BREAKPOINT;
    let (rect, _) = ui.allocate_exact_size(vec2(width, CODE_TOOLBAR_HEIGHT), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );

    let content = rect.shrink2(vec2(CODE_TOOLBAR_PADDING_X, 0.0));
    let active = app.state.ui.netlist.active_document;
    let dependency_visible = app.state.ui.netlist.active_dependency_identity.is_some();
    let dependency_owned =
        crate::workbench::documents::netlist_document::active_dependency_is_owned(&app.state);
    let dependency_authority =
        crate::workbench::documents::netlist_document::active_dependency(&app.state)
            .map(crate::state::DependencyMetadata::authority);
    let generated_ready = generated_primary_ready(&app.state);
    let active_available = active_document_available(&app.state);
    let action_width: f32 = if compact {
        PHONE_ACTION_WIDTH
    } else if dependency_visible {
        if dependency_owned { 280.0 } else { 390.0 }
    } else {
        (match active {
            ActiveNetlistDocument::Generated => {
                if app.state.workspace.netlist_source.is_some() {
                    175.0
                } else {
                    342.0
                }
            }
            ActiveNetlistDocument::OwnedSource => 348.0,
            ActiveNetlistDocument::GeneratedDiff => 152.0,
        }) + EDITOR_MENU_WIDTH
            + CODE_TOOLBAR_GAP
    };
    let (left_rect, right_rect) = code_toolbar_regions(content, action_width);
    let language = match active {
        ActiveNetlistDocument::Generated => messages.text(MessageId::NetlistLanguageGenerated),
        ActiveNetlistDocument::OwnedSource => {
            app.state.workspace.netlist_descriptor.as_ref().map_or_else(
                || messages.text(MessageId::NetlistLanguageOwned),
                |descriptor| {
                    messages.text(match descriptor.strategy {
                        crate::state::OwnedNetlistEditStrategy::OwnedSource => {
                            MessageId::NetlistLanguageOwned
                        }
                        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => {
                            MessageId::NetlistLanguageParameterOverride
                        }
                        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => {
                            MessageId::NetlistLanguageIncludeOrderOverride
                        }
                        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => {
                            MessageId::NetlistLanguageAnalysisDeck
                        }
                    })
                },
            )
        }
        ActiveNetlistDocument::GeneratedDiff => messages.text(MessageId::NetlistLanguageDiff),
    };
    let language = if dependency_visible {
        if dependency_owned {
            messages.text(MessageId::NetlistLanguageOwnedInclude)
        } else {
            messages.text(match dependency_authority.unwrap_or_default() {
                crate::state::DependencySourceAuthority::External => {
                    MessageId::NetlistLanguageExternalInclude
                }
                crate::state::DependencySourceAuthority::Vendor => {
                    MessageId::NetlistLanguageVendorInclude
                }
                crate::state::DependencySourceAuthority::TechnologyPackage => {
                    MessageId::NetlistLanguageTechnologyInclude
                }
                crate::state::DependencySourceAuthority::StandardLibrary => {
                    MessageId::NetlistLanguageStandardInclude
                }
            })
        }
    } else {
        language
    };
    let (status, status_tone) = document_syntax_status(&app.state);
    let status_color = match status_tone {
        DocumentStatusTone::Valid => t.color.ok,
        DocumentStatusTone::Warning => t.color.warn,
        DocumentStatusTone::Error => t.color.err,
    };
    let status_visible = toolbar_status_visible(phone, status_tone);
    let advisory_candidate = (!compact).then(|| {
        app.state
            .ui
            .netlist
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.is_current()
                    && diagnostic.severity
                        != crate::workbench::documents::netlist_document::DiagnosticSeverity::Error
            })
            .count()
            + app
                .state
                .ui
                .netlist
                .validation
                .as_ref()
                .map_or(0, |receipt| receipt.advisory_count)
            + usize::from(app.state.ui.netlist.validation_error.is_some())
    });
    let status_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let label_width = |label: &str, color| {
        ui.painter()
            .layout_no_wrap(label.to_owned(), status_font.clone(), color)
            .size()
            .x
    };
    let status_only_width = if status_visible {
        11.0 + label_width(&status, status_color)
    } else {
        0.0
    };
    let language_width = label_width(&language, t.color.text_dim);
    let advisory_count = advisory_candidate.filter(|count| {
        let label = format!("{count} advisor{}", if *count == 1 { "y" } else { "ies" });
        toolbar_advisory_fits(
            left_rect.width(),
            language_width,
            status_only_width,
            11.0 + label_width(&label, t.color.text_faint),
        )
    });
    let advisory_label = advisory_count
        .map(|count| format!("{count} advisor{}", if count == 1 { "y" } else { "ies" }));
    let mut status_width = status_only_width;
    if let Some(label) = advisory_label.as_deref() {
        status_width += CODE_TOOLBAR_GAP + 11.0 + label_width(label, t.color.text_faint);
    }
    status_width = status_width.min(left_rect.width());
    let status_rect = egui::Rect::from_min_max(
        egui::pos2(left_rect.right() - status_width, left_rect.top()),
        left_rect.right_bottom(),
    );
    let language_rect = egui::Rect::from_min_max(
        left_rect.left_top(),
        egui::pos2(
            (status_rect.left() - CODE_TOOLBAR_GAP).max(left_rect.left()),
            left_rect.bottom(),
        ),
    );
    if language_rect.width() > 0.0 {
        let mut language_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(language_rect)
                .layout(Layout::left_to_right(Align::Center)),
        );
        language_ui.add(
            egui::Label::new(
                egui::RichText::new(&language)
                    .font(status_font.clone())
                    .color(t.color.text_dim),
            )
            .truncate(),
        );
    }
    if status_width > 0.0 {
        let mut status_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(status_rect)
                .layout(Layout::right_to_left(Align::Center)),
        );
        status_ui.spacing_mut().item_spacing.x = CODE_TOOLBAR_GAP;
        status_ui.with_layout(Layout::right_to_left(Align::Center), |bar| {
            if let Some(advisory_count) = advisory_count {
                code_status(
                    bar,
                    advisory_label.as_deref().unwrap_or_default(),
                    if advisory_count == 0 {
                        t.color.text_faint
                    } else {
                        t.color.warn
                    },
                );
            }
            if status_visible {
                code_status(bar, &status, status_color);
            }
        });
    }

    let mut action = None;
    let mut actions = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(right_rect)
            .layout(Layout::right_to_left(Align::Center)),
    );
    actions.spacing_mut().item_spacing.x = CODE_TOOLBAR_GAP;
    if compact {
        actions.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add_enabled_ui(active_available, |ui| {
                crate::workbench::documents::text_editor_commands::editor_command_menu(
                    ui,
                    crate::workbench::documents::netlist_document::editor_id(),
                    crate::workbench::documents::netlist_document::active_netlist_source_is_editable(
                        &app.state,
                    ),
                    false,
                );
            })
            .response
            .on_disabled_hover_text(messages.text(MessageId::NetlistEditorCommandsUnavailable));
            let mut find_clicked = false;
            ui.add_enabled_ui(active_available, |ui| {
                find_clicked = icon_button(
                    ui,
                    WorkbenchIcon::Search,
                    &messages.text(MessageId::NetlistFindActiveDocument),
                    false,
                    vec2(28.0, 28.0),
                )
                .clicked();
            })
            .response
            .on_disabled_hover_text(messages.text(MessageId::NetlistSearchUnavailable));
            if find_clicked {
                action = Some(NetlistToolbarAction::Find);
            }
            if dependency_visible {
                let (label, candidate) = if dependency_owned {
                    (
                        messages.text(MessageId::NetlistReturnRoot),
                        NetlistToolbarAction::CloseDependency,
                    )
                } else {
                    (
                        messages.text(MessageId::NetlistCopyProject),
                        NetlistToolbarAction::CopyDependency,
                    )
                };
                if ui
                    .add_sized(
                        [PHONE_PRIMARY_WIDTH, 28.0],
                        egui::Button::new(&label).truncate(),
                    )
                    .clicked()
                {
                    action = Some(candidate);
                }
                if !dependency_owned
                    && icon_button(
                        ui,
                        WorkbenchIcon::ArrowLeft,
                        &messages.text(MessageId::NetlistReturnRoot),
                        false,
                        vec2(28.0, 28.0),
                    )
                    .clicked()
                {
                    action = Some(NetlistToolbarAction::CloseDependency);
                }
                if icon_button(
                    ui,
                    WorkbenchIcon::Refresh,
                    &messages.text(MessageId::NetlistRelinkTooltip),
                    false,
                    vec2(28.0, 28.0),
                )
                .clicked()
                {
                    action = Some(NetlistToolbarAction::RelinkDependency);
                }
            } else {
            match active {
                ActiveNetlistDocument::Generated => {
                    let (label, candidate) = if app.state.workspace.netlist_source.is_some() {
                        (
                            messages.text(MessageId::NetlistOpenEditable),
                            NetlistToolbarAction::OpenOwned,
                        )
                    } else {
                        (
                            messages.text(MessageId::NetlistCreateEditable),
                            NetlistToolbarAction::OpenOwnershipDialog(
                                crate::state::OwnedNetlistEditStrategy::OwnedSource,
                            ),
                        )
                    };
                    let primary_ready =
                        app.state.workspace.netlist_source.is_some() || generated_ready;
                    if ui
                        .add_enabled(
                            primary_ready,
                            egui::Button::new(&label)
                                .truncate()
                                .min_size(vec2(PHONE_PRIMARY_WIDTH, 28.0)),
                        )
                        .on_disabled_hover_text(generation_block_reason(&app.state))
                        .clicked()
                    {
                        action = Some(candidate);
                    }
                }
                ActiveNetlistDocument::OwnedSource => {
                    let save_ready = owned_source_save_ready(app);
                    if ui
                        .add_enabled(
                            save_ready,
                            egui::Button::new(messages.text(MessageId::NetlistSaveSourceDeck))
                                .truncate()
                                .min_size(vec2(PHONE_PRIMARY_WIDTH, 28.0)),
                        )
                        .on_disabled_hover_text(
                            messages.text(MessageId::NetlistValidateBeforeSave),
                        )
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::Save);
                    }
                }
                ActiveNetlistDocument::GeneratedDiff => {
                    if ui
                        .add_sized(
                            [PHONE_PRIMARY_WIDTH, 28.0],
                            egui::Button::new(
                                messages.text(MessageId::NetlistReturnGeneratedPrimary),
                            )
                            .truncate(),
                        )
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::CloseComparison);
                    }
                }
            }
            }
        });
    } else {
        actions.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add_enabled_ui(active_available, |ui| {
                crate::workbench::documents::text_editor_commands::editor_command_menu(
                    ui,
                    crate::workbench::documents::netlist_document::editor_id(),
                    crate::workbench::documents::netlist_document::active_netlist_source_is_editable(
                        &app.state,
                    ),
                    false,
                );
            })
            .response
            .on_disabled_hover_text(messages.text(MessageId::NetlistEditorCommandsUnavailable));
            let mut find_clicked = false;
            ui.add_enabled_ui(active_available, |ui| {
                find_clicked = icon_button(
                    ui,
                    WorkbenchIcon::Search,
                    &messages.text(MessageId::NetlistFindActiveDocument),
                    false,
                    vec2(28.0, 28.0),
                )
                .clicked();
            })
            .response
            .on_disabled_hover_text(messages.text(MessageId::NetlistSearchUnavailable));
            if find_clicked {
                action = Some(NetlistToolbarAction::Find);
            }

            if dependency_visible {
                if ui
                    .button(messages.text(MessageId::NetlistReturnRoot))
                    .clicked()
                {
                    action = Some(NetlistToolbarAction::CloseDependency);
                }
                if ui.button(messages.text(MessageId::NetlistRelink)).clicked() {
                    action = Some(NetlistToolbarAction::RelinkDependency);
                }
                if !dependency_owned
                    && ui
                        .button(messages.text(MessageId::NetlistCopyProject))
                        .clicked()
                {
                    action = Some(NetlistToolbarAction::CopyDependency);
                }
            } else {
            match active {
                ActiveNetlistDocument::Generated => {
                    let mut override_clicked = false;
                    if !compact && app.state.workspace.netlist_source.is_none() {
                        ui.add_enabled_ui(generated_ready, |ui| {
                            override_clicked = icon_button(
                                ui,
                                WorkbenchIcon::More,
                                "Create a narrow generated-source override",
                                false,
                                vec2(28.0, 28.0),
                            )
                            .clicked();
                        })
                        .response
                        .on_disabled_hover_text(generation_block_reason(&app.state));
                    }
                    if override_clicked {
                        action = Some(NetlistToolbarAction::OpenOwnershipDialog(
                            crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
                        ));
                    }
                    let label = if app.state.workspace.netlist_source.is_some() {
                        messages.text(MessageId::NetlistOpenEditable)
                    } else {
                        messages.text(MessageId::NetlistCreateEditable)
                    };
                    let primary_ready =
                        app.state.workspace.netlist_source.is_some() || generated_ready;
                    let response = ui
                        .add_enabled(
                            primary_ready,
                            egui::Button::new(&label).min_size(vec2(0.0, 28.0)),
                        )
                        .on_disabled_hover_text(generation_block_reason(&app.state));
                    if response.clicked() {
                        action = Some(if app.state.workspace.netlist_source.is_some() {
                            NetlistToolbarAction::OpenOwned
                        } else {
                            NetlistToolbarAction::OpenOwnershipDialog(
                                crate::state::OwnedNetlistEditStrategy::OwnedSource,
                            )
                        });
                    }
                }
                ActiveNetlistDocument::OwnedSource => {
                    let save_ready = owned_source_save_ready(app);
                    if ui
                        .add_enabled(
                            save_ready,
                            egui::Button::new(messages.text(MessageId::NetlistSaveSourceDeck)),
                        )
                        .on_disabled_hover_text(
                            messages.text(MessageId::NetlistValidateBeforeSave),
                        )
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::Save);
                    }
                    if ui
                        .button(messages.text(MessageId::NetlistValidateSource))
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::Validate);
                    }
                    if ui
                        .button(messages.text(MessageId::NetlistReturnPrimary))
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::OpenGenerated);
                    }
                }
                ActiveNetlistDocument::GeneratedDiff => {
                    if ui
                        .button(messages.text(MessageId::NetlistReturnPrimary))
                        .clicked()
                    {
                        action = Some(NetlistToolbarAction::CloseComparison);
                    }
                }
            }
            }
        });
    }

    match action {
        Some(NetlistToolbarAction::OpenOwned) => {
            let _ = open_owned_source(&mut app.state);
        }
        Some(NetlistToolbarAction::OpenGenerated) => {
            let _ = crate::workbench::documents::netlist_document::open_generated_primary(
                &mut app.state,
            );
        }
        Some(NetlistToolbarAction::CloseComparison) => {
            crate::workbench::documents::netlist_document::close_revision_comparison(
                &mut app.state,
            );
        }
        Some(NetlistToolbarAction::CloseDependency) => {
            crate::workbench::documents::netlist_document::close_active_dependency(&mut app.state);
        }
        Some(NetlistToolbarAction::CopyDependency) => {
            match crate::workbench::documents::netlist_document::copy_active_dependency_to_project(
                &mut app.state,
            ) {
                Ok(_) => app.state.push_user_message(ConsoleMessage::info(
                    messages.text(MessageId::NetlistCopySucceeded),
                )),
                Err(error) => app.state.push_user_message(ConsoleMessage::error(error)),
            }
        }
        Some(NetlistToolbarAction::RelinkDependency) => {
            if let Some(identity) = app.state.ui.netlist.active_dependency_identity.clone() {
                crate::workbench::workflows::netlist_workflow::request_dependency_relink(
                    &mut app.state,
                    &identity,
                );
            }
        }
        Some(NetlistToolbarAction::OpenOwnershipDialog(strategy)) => {
            open_ownership_dialog(&mut app.state, strategy);
        }
        Some(NetlistToolbarAction::Validate) => {
            crate::workbench::commands::vocabulary::Command::ValidateCodeDocument.execute(app);
        }
        Some(NetlistToolbarAction::Save) => {
            crate::workbench::commands::vocabulary::Command::Save.execute(app);
        }
        Some(NetlistToolbarAction::Find) => {
            crate::workbench::commands::vocabulary::Command::FindCodeDocument.execute(app);
        }
        None => {}
    }
}

const fn code_toolbar_compact(width: f32) -> bool {
    width <= CODE_TOOLBAR_COMPACT_BREAKPOINT
}

fn code_toolbar_regions(content: egui::Rect, action_width: f32) -> (egui::Rect, egui::Rect) {
    let action_width = action_width.clamp(0.0, content.width());
    let right = egui::Rect::from_min_max(
        egui::pos2(content.right() - action_width, content.top()),
        content.right_bottom(),
    );
    let left = egui::Rect::from_min_max(
        content.left_top(),
        egui::pos2(
            (right.left() - CODE_TOOLBAR_ACTION_GUTTER).max(content.left()),
            content.bottom(),
        ),
    );
    (left, right)
}

fn toolbar_advisory_fits(
    left_width: f32,
    language_width: f32,
    status_width: f32,
    advisory_width: f32,
) -> bool {
    let status_group_width = status_width
        + if status_width > 0.0 {
            CODE_TOOLBAR_GAP
        } else {
            0.0
        }
        + advisory_width;
    language_width + CODE_TOOLBAR_GAP + status_group_width <= left_width
}

fn code_status(ui: &mut Ui, label: &str, color: egui::Color32) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let (dot, _) = ui.allocate_exact_size(vec2(5.0, 11.0), egui::Sense::hover());
        ui.painter().circle_filled(dot.center(), 2.5, color);
        ui.label(
            egui::RichText::new(label)
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(color),
        );
    });
}

const fn toolbar_status_visible(phone: bool, tone: DocumentStatusTone) -> bool {
    !(phone && matches!(tone, DocumentStatusTone::Warning))
}

#[derive(Debug, Clone, Copy)]
enum NetlistToolbarAction {
    OpenOwned,
    OpenGenerated,
    CloseComparison,
    CloseDependency,
    CopyDependency,
    RelinkDependency,
    OpenOwnershipDialog(crate::state::OwnedNetlistEditStrategy),
    Validate,
    Save,
    Find,
}

#[derive(Debug, Clone, Copy)]
enum FindWindowAction {
    Select(usize),
    ReplaceNext,
    ReplaceAll,
}

#[derive(Debug, Clone)]
struct NetlistSearchDocument {
    active_document: ActiveNetlistDocument,
    dependency_identity: Option<String>,
    editable: bool,
    label: String,
    source: String,
}

#[derive(Debug, Clone)]
struct NetlistSearchMatch {
    document: NetlistSearchDocument,
    found: crate::state::FindMatch,
}

fn netlist_search_documents(
    state: &AppState,
    scope: crate::workbench::documents::netlist_document::NetlistFindScope,
) -> Vec<NetlistSearchDocument> {
    use crate::workbench::documents::netlist_document::NetlistFindScope;

    let generated = || NetlistSearchDocument {
        active_document: ActiveNetlistDocument::Generated,
        dependency_identity: None,
        editable: false,
        label: "generated.sp".to_owned(),
        source: state.ui.netlist.generated_source.clone(),
    };
    let owned = || NetlistSearchDocument {
        active_document: ActiveNetlistDocument::OwnedSource,
        dependency_identity: None,
        editable: true,
        label: state
            .workspace
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.clone())
            .or_else(|| {
                state
                    .workspace
                    .netlist_source_path
                    .as_deref()
                    .and_then(std::path::Path::file_name)
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "owned-source.sp".to_owned()),
        source: state.workspace.netlist_source.clone().unwrap_or_default(),
    };

    match scope {
        NetlistFindScope::CurrentDocument => {
            if let Some(dependency) =
                crate::workbench::documents::netlist_document::active_dependency(state)
            {
                vec![NetlistSearchDocument {
                    active_document: state
                        .ui
                        .netlist
                        .active_dependency_root
                        .unwrap_or(state.ui.netlist.active_document),
                    dependency_identity: Some(dependency.locator().logical_identity().to_owned()),
                    editable:
                        crate::workbench::documents::netlist_document::active_dependency_is_owned(
                            state,
                        ),
                    label: dependency.locator().display_name().to_owned(),
                    source: dependency.source().unwrap_or_default().to_owned(),
                }]
            } else {
                vec![match state.ui.netlist.active_document {
                    ActiveNetlistDocument::Generated => generated(),
                    ActiveNetlistDocument::OwnedSource => owned(),
                    ActiveNetlistDocument::GeneratedDiff => NetlistSearchDocument {
                        active_document: ActiveNetlistDocument::GeneratedDiff,
                        dependency_identity: None,
                        editable: false,
                        label: "generated.diff".to_owned(),
                        source: state.ui.netlist.generated_diff_source.clone(),
                    },
                }]
            }
        }
        NetlistFindScope::AllOwnedSources => {
            let mut documents = state
                .workspace
                .netlist_source
                .as_ref()
                .map(|_| vec![owned()])
                .unwrap_or_default();
            documents.extend(dependency_search_documents(
                state,
                ActiveNetlistDocument::OwnedSource,
                true,
            ));
            documents
        }
        NetlistFindScope::ProjectReferences => {
            let mut documents = Vec::new();
            if !state.ui.netlist.generated_source.is_empty() {
                documents.push(generated());
                documents.extend(dependency_search_documents(
                    state,
                    ActiveNetlistDocument::Generated,
                    false,
                ));
            }
            if state.workspace.netlist_source.is_some() {
                documents.push(owned());
                documents.extend(dependency_search_documents(
                    state,
                    ActiveNetlistDocument::OwnedSource,
                    false,
                ));
            }
            documents
        }
    }
}

fn dependency_search_documents(
    state: &AppState,
    root: ActiveNetlistDocument,
    owned_only: bool,
) -> Vec<NetlistSearchDocument> {
    let document = match root {
        ActiveNetlistDocument::Generated => state.ui.netlist.generated_document.as_ref(),
        ActiveNetlistDocument::OwnedSource => state.ui.netlist.owned_document.as_ref(),
        ActiveNetlistDocument::GeneratedDiff => None,
    };
    let owned = state.workspace.netlist_descriptor.as_ref();
    document
        .into_iter()
        .flat_map(crate::state::NetlistDocument::dependencies)
        .filter_map(|dependency| {
            let source = dependency.source()?;
            let identity = dependency.locator().logical_identity();
            let editable = root == ActiveNetlistDocument::OwnedSource
                && owned
                    .and_then(|value| value.owned_include(identity))
                    .is_some();
            (!owned_only || editable).then(|| NetlistSearchDocument {
                active_document: root,
                dependency_identity: Some(identity.to_owned()),
                editable,
                label: dependency.locator().display_name().to_owned(),
                source: source.to_owned(),
            })
        })
        .collect()
}

fn open_ownership_dialog(state: &mut AppState, strategy: crate::state::OwnedNetlistEditStrategy) {
    if state.workspace.netlist_source.is_some() {
        let _ = open_owned_source(state);
        return;
    }
    if state.ui.netlist.generated_document.is_none() {
        state.push_user_message(ConsoleMessage::warning(
            "Generate a current primary artifact before creating owned source.",
        ));
        return;
    }
    let dialog = &mut state.ui.netlist.ownership_dialog;
    dialog.open = true;
    dialog.strategy = strategy;
    dialog.artifact_name = default_owned_artifact_name(strategy).to_owned();
    dialog.error = None;
}

fn default_owned_artifact_name(strategy: crate::state::OwnedNetlistEditStrategy) -> &'static str {
    match strategy {
        crate::state::OwnedNetlistEditStrategy::OwnedSource => "top_override.sp",
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => "top_params.inc",
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => "top_includes.inc",
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => "top_analysis.sp",
    }
}

fn owned_strategy_label(
    messages: MessageCatalog,
    strategy: crate::state::OwnedNetlistEditStrategy,
) -> String {
    messages.text(match strategy {
        crate::state::OwnedNetlistEditStrategy::OwnedSource => {
            MessageId::NetlistStrategyOwnedSource
        }
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => {
            MessageId::NetlistStrategyParameterOverride
        }
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => {
            MessageId::NetlistStrategyIncludeOverride
        }
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => {
            MessageId::NetlistStrategyAnalysisDeck
        }
    })
}

fn owned_strategy_description(
    messages: MessageCatalog,
    strategy: crate::state::OwnedNetlistEditStrategy,
) -> String {
    messages.text(match strategy {
        crate::state::OwnedNetlistEditStrategy::OwnedSource => {
            MessageId::NetlistStrategyOwnedSourceDescription
        }
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => {
            MessageId::NetlistStrategyParameterOverrideDescription
        }
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => {
            MessageId::NetlistStrategyIncludeOverrideDescription
        }
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => {
            MessageId::NetlistStrategyAnalysisDeckDescription
        }
    })
}

fn ownership_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.ownership_dialog.open {
        return;
    }
    let Some(generated) = app.state.ui.netlist.generated_document.as_ref() else {
        app.state.ui.netlist.ownership_dialog.open = false;
        return;
    };
    let base_revision = generated.revision().get();
    let mut dialog = app.state.ui.netlist.ownership_dialog.clone();
    let messages = app.state.ui.messages();
    let owned_deck = dialog.strategy == crate::state::OwnedNetlistEditStrategy::OwnedSource;
    let title = if owned_deck {
        messages.text(MessageId::NetlistOwnershipCreateSourceTitle)
    } else {
        messages.text(MessageId::NetlistOwnershipCreateOverrideTitle)
    };
    let primary = if owned_deck {
        messages.text(MessageId::NetlistOwnershipCreateSource)
    } else {
        messages.text(MessageId::NetlistOwnershipCreatePatch)
    };
    let choice = Dialog::new(
        messages.text(MessageId::NetlistOwnershipEyebrow),
        title,
        primary,
    )
    .description(messages.text(MessageId::NetlistOwnershipDescription))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .ghost(messages.text(MessageId::CommonCancel))
    .show_with_initial_body_focus(ctx, |ui| {
        let t = Tokens::get(ctx);
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(10)
            .show(ui, |ui| {
                ui.label(messages.text(MessageId::NetlistOwnershipNotice));
            });
        ui.add_space(8.0);
        ui.label(messages.text(MessageId::NetlistArtifactName));
        let artifact_name = ui.add(
            egui::TextEdit::singleline(&mut dialog.artifact_name).desired_width(f32::INFINITY),
        );
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(messages.text(MessageId::NetlistBaseRevision));
            ui.monospace(base_revision.to_string());
        });
        ui.add_space(8.0);
        ui.label(messages.text(MessageId::NetlistEditStrategy));
        egui::ComboBox::from_id_salt("rspice.code.ownership-strategy")
            .selected_text(owned_strategy_label(messages, dialog.strategy))
            .width(ui.available_width().max(1.0))
            .show_ui(ui, |ui| {
                for strategy in crate::state::OwnedNetlistEditStrategy::ALL {
                    ui.selectable_value(
                        &mut dialog.strategy,
                        strategy,
                        owned_strategy_label(messages, strategy),
                    );
                }
            });
        ui.label(owned_strategy_description(messages, dialog.strategy));
        if let Some(error) = &dialog.error {
            ui.add_space(6.0);
            ui.colored_label(Tokens::get(ctx).color.err, error);
        }
        Some(artifact_name.id)
    });
    match choice {
        DialogChoice::Primary => {
            match create_owned_source(&mut app.state, &dialog.artifact_name, dialog.strategy) {
                Ok(()) => dialog.open = false,
                Err(error) => dialog.error = Some(error),
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => dialog.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    app.state.ui.netlist.ownership_dialog = dialog;
}

fn comparison_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.comparison_dialog.open {
        return;
    }
    let owned = app.state.ui.netlist.active_document == ActiveNetlistDocument::OwnedSource
        && app.state.ui.netlist.active_dependency_identity.is_none();
    let history_len = if owned {
        app.state
            .workspace
            .netlist_descriptor
            .as_ref()
            .map_or(0, |descriptor| descriptor.revision_history.len())
    } else {
        app.state.ui.netlist.generated_history.len()
    };
    let current_available = if owned {
        app.state.ui.netlist.owned_document.is_some()
    } else {
        app.state.ui.netlist.generated_document.is_some()
    };
    if history_len == 0 || !current_available {
        app.state.ui.netlist.comparison_dialog.open = false;
        return;
    }
    let mut dialog = app.state.ui.netlist.comparison_dialog.clone();
    let messages = app.state.ui.messages();
    dialog.selected_history_index = dialog.selected_history_index.min(history_len - 1);
    let selected_label = if owned {
        let Some(selected) =
            app.state
                .workspace
                .netlist_descriptor
                .as_ref()
                .and_then(|descriptor| {
                    descriptor
                        .revision_history
                        .get(dialog.selected_history_index)
                })
        else {
            app.state.ui.netlist.comparison_dialog.open = false;
            return;
        };
        let revision = selected.document_revision.to_string();
        let digest = selected
            .content_digest
            .to_string()
            .chars()
            .take(12)
            .collect::<String>();
        messages.format(
            MessageId::NetlistOwnedRevisionChoice,
            &[
                ("revision", &revision),
                ("digest", &digest),
                ("message", &selected.message),
            ],
        )
    } else {
        let selected = &app.state.ui.netlist.generated_history[dialog.selected_history_index];
        let revision = selected.provenance().input().revision().get().to_string();
        let digest = selected
            .content_digest()
            .to_string()
            .chars()
            .take(12)
            .collect::<String>();
        messages.format(
            MessageId::NetlistGeneratedRevisionChoice,
            &[("revision", &revision), ("digest", &digest)],
        )
    };
    let restore_enabled = owned
        && app
            .state
            .workspace
            .netlist_descriptor
            .as_ref()
            .and_then(|descriptor| {
                descriptor
                    .revision_history
                    .get(dialog.selected_history_index)
            })
            .zip(app.state.ui.netlist.owned_document.as_ref())
            .is_some_and(|(snapshot, current)| {
                snapshot.content_digest != current.content_digest()
                    || snapshot.dependencies != current.dependencies()
            });
    let mut transaction = Dialog::new(
        messages.text(MessageId::NetlistComparisonEyebrow),
        if owned {
            messages.text(MessageId::NetlistComparisonOwnedTitle)
        } else {
            messages.text(MessageId::NetlistComparisonGeneratedTitle)
        },
        messages.text(MessageId::NetlistCompareRevisions),
    )
    .description(if owned {
        messages.text(MessageId::NetlistComparisonOwnedDescription)
    } else {
        messages.text(MessageId::NetlistComparisonGeneratedDescription)
    })
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .ghost(messages.text(MessageId::CommonCancel));
    if owned {
        transaction = transaction
            .secondary(messages.text(MessageId::NetlistRestoreAsNewRevision))
            .secondary_enabled(restore_enabled);
    }
    let choice = transaction.show_with_initial_body_focus(ctx, |ui| {
        ui.label(if owned {
            messages.text(MessageId::NetlistComparisonOwnedPrompt)
        } else {
            messages.text(MessageId::NetlistComparisonGeneratedPrompt)
        });
        ui.add_space(8.0);
        ui.label(messages.text(MessageId::NetlistPriorRevision));
        let revision = egui::ComboBox::from_id_salt("rspice.code.compare-revision-select")
            .selected_text(selected_label)
            .width(ui.available_width().max(1.0))
            .show_ui(ui, |ui| {
                if owned {
                    if let Some(descriptor) = app.state.workspace.netlist_descriptor.as_ref() {
                        for (index, snapshot) in
                            descriptor.revision_history.iter().enumerate().rev()
                        {
                            let revision = snapshot.document_revision.to_string();
                            let digest = snapshot
                                .content_digest
                                .to_string()
                                .chars()
                                .take(12)
                                .collect::<String>();
                            let label = messages.format(
                                MessageId::NetlistOwnedRevisionChoice,
                                &[
                                    ("revision", &revision),
                                    ("digest", &digest),
                                    ("message", &snapshot.message),
                                ],
                            );
                            ui.selectable_value(&mut dialog.selected_history_index, index, label);
                        }
                    }
                } else {
                    for (index, artifact) in app
                        .state
                        .ui
                        .netlist
                        .generated_history
                        .iter()
                        .enumerate()
                        .rev()
                    {
                        let revision = artifact.provenance().input().revision().get().to_string();
                        let digest = artifact
                            .content_digest()
                            .to_string()
                            .chars()
                            .take(12)
                            .collect::<String>();
                        let label = messages.format(
                            MessageId::NetlistGeneratedRevisionChoice,
                            &[("revision", &revision), ("digest", &digest)],
                        );
                        ui.selectable_value(&mut dialog.selected_history_index, index, label);
                    }
                }
            });
        Some(revision.response.id)
    });
    match choice {
        DialogChoice::Primary => {
            let result = if owned {
                crate::workbench::documents::netlist_document::compare_owned_revision(
                    &mut app.state,
                    dialog.selected_history_index,
                )
            } else {
                crate::workbench::documents::netlist_document::compare_generated_revision(
                    &mut app.state,
                    dialog.selected_history_index,
                )
            };
            match result {
                Ok(()) => dialog.open = false,
                Err(error) => app.state.push_user_message(ConsoleMessage::warning(error)),
            }
        }
        DialogChoice::Secondary => {
            match crate::workbench::documents::netlist_document::restore_owned_revision(
                &mut app.state,
                dialog.selected_history_index,
            ) {
                Ok(()) => dialog.open = false,
                Err(error) => app.state.push_user_message(ConsoleMessage::warning(error)),
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => dialog.open = false,
        DialogChoice::None => {}
    }
    app.state.ui.netlist.comparison_dialog = dialog;
}

fn owned_source_save_ready(app: &RSpiceApp) -> bool {
    if app.state.ui.netlist.active_document != ActiveNetlistDocument::OwnedSource
        || app.state.ui.netlist.active_dependency_identity.is_some()
    {
        return false;
    }
    let digest = source_content_digest(&app.state.simulation.netlist_content);
    if app.state.ui.netlist.externally_saved_content_digest == Some(digest) {
        return false;
    }
    app.state
        .ui
        .netlist
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == digest
                && receipt.project_revision == app.state.workspace.project.revision().get()
                && app
                    .simulation_controller
                    .has_retained_manual_authorization(receipt.prepared_snapshot_digest)
        })
}

fn save_source_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.save_dialog.open {
        return;
    }
    if app.state.ui.netlist.active_document != ActiveNetlistDocument::OwnedSource
        || app.state.ui.netlist.active_dependency_identity.is_some()
    {
        app.state.ui.netlist.save_dialog.open = false;
        return;
    }

    let mut dialog = app.state.ui.netlist.save_dialog.clone();
    let messages = app.state.ui.messages();
    let current_digest = source_content_digest(&app.state.simulation.netlist_content);
    let validated = app
        .state
        .ui
        .netlist
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == current_digest
                && receipt.project_revision == app.state.workspace.project.revision().get()
                && app
                    .simulation_controller
                    .has_retained_manual_authorization(receipt.prepared_snapshot_digest)
        });
    let message_valid = {
        let message = dialog.message.trim();
        !message.is_empty()
            && message.chars().count() <= 240
            && !message.chars().any(char::is_control)
    };
    let needs_save = app.state.ui.netlist.externally_saved_content_digest != Some(current_digest);
    let primary_enabled = validated && message_valid && needs_save;
    let footer_hint = if validated {
        messages.text(MessageId::NetlistExactSnapshotValidated)
    } else {
        messages.text(MessageId::NetlistValidationRequired)
    };
    let choice = Dialog::new(
        messages.text(MessageId::NetlistSaveEyebrow),
        messages.text(MessageId::NetlistSaveTitle),
        messages.text(MessageId::NetlistSave),
    )
    .description(messages.text(MessageId::NetlistSaveDescription))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .primary_enabled(primary_enabled)
    .ghost(messages.text(MessageId::CommonCancel))
    .hint(footer_hint)
    .show_with_initial_body_focus(ctx, |ui| {
        let t = Tokens::get(ctx);
        let descriptor = app.state.workspace.netlist_descriptor.as_ref();
        let artifact_name = descriptor.map_or_else(
            || messages.text(MessageId::NetlistOwnedSpiceSource),
            |value| value.artifact_name.clone(),
        );
        ui.label(
            egui::RichText::new(artifact_name).font(theme::mono(tokens::FS_1, FontWeight::Medium)),
        );
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(10)
            .show(ui, |ui| {
                ui.label(if validated {
                    messages.text(MessageId::NetlistSaveValidatedNotice)
                } else {
                    messages.text(MessageId::NetlistSaveStaleNotice)
                });
                if let Some(document) = app.state.ui.netlist.owned_document.as_ref() {
                    let digest = document
                        .generated_artifact()
                        .content_digest()
                        .to_string()
                        .chars()
                        .take(12)
                        .collect::<String>();
                    let revision = document.revision().get().to_string();
                    ui.monospace(messages.format(
                        MessageId::NetlistGeneratedBaseRevision,
                        &[("digest", &digest), ("revision", &revision)],
                    ));
                }
                ui.label(messages.text(MessageId::NetlistSaveImmutableNotice));
            });
        ui.add_space(8.0);
        ui.label(messages.text(MessageId::NetlistRevisionMessage));
        let revision_message = ui.add(
            egui::TextEdit::singleline(&mut dialog.message)
                .desired_width(f32::INFINITY)
                .char_limit(240),
        );
        if !message_valid {
            ui.colored_label(
                t.color.err,
                messages.text(MessageId::NetlistRevisionMessageInvalid),
            );
        }
        if !needs_save {
            ui.weak(messages.text(MessageId::NetlistAlreadyPublished));
        }
        if let Some(error) = &dialog.error {
            ui.colored_label(t.color.err, error);
        }
        Some(revision_message.id)
    });
    match choice {
        DialogChoice::Primary => {
            if crate::workbench::workflows::netlist_workflow::save_owned_netlist_source(
                &mut app.state,
                &app.simulation_controller,
                app.export_workflow_io.as_ref(),
                false,
                &dialog.message,
            ) {
                crate::workbench::workflows::netlist_workflow::validate_visible_netlist_source(app);
                dialog.open = false;
                dialog.error = None;
                dialog.message = "Update owned SPICE source".to_owned();
            } else {
                dialog.error = Some(
                    "Source publication did not complete; review the application log.".to_owned(),
                );
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => dialog.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    app.state.ui.netlist.save_dialog = dialog;
}

fn external_resolution_label(
    messages: MessageCatalog,
    resolution: crate::workbench::documents::netlist_document::NetlistExternalChangeResolution,
) -> String {
    use crate::workbench::documents::netlist_document::NetlistExternalChangeResolution;

    messages.text(match resolution {
        NetlistExternalChangeResolution::Merge => MessageId::NetlistExternalMerge,
        NetlistExternalChangeResolution::KeepLocal => MessageId::NetlistExternalKeepLocal,
        NetlistExternalChangeResolution::ReloadExternal => MessageId::NetlistExternalReload,
    })
}

fn external_change_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    use crate::workbench::documents::netlist_document::NetlistExternalChangeResolution;

    let Some(mut review) = app.state.ui.netlist.external_change.clone() else {
        return;
    };
    let messages = app.state.ui.messages();
    let digest = |bytes: &[u8; 32]| {
        bytes
            .iter()
            .take(6)
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    };
    let hint = match review.resolution {
        NetlistExternalChangeResolution::Merge if review.merge_conflict_count == 0 => {
            messages.text(MessageId::NetlistExternalConflictFree)
        }
        NetlistExternalChangeResolution::Merge => {
            messages.text(MessageId::NetlistExternalConflictResolutionRequired)
        }
        NetlistExternalChangeResolution::KeepLocal => {
            messages.text(MessageId::NetlistExternalKeepLocalHint)
        }
        NetlistExternalChangeResolution::ReloadExternal => {
            messages.text(MessageId::NetlistExternalReloadHint)
        }
    };
    let choice = Dialog::new(
        messages.text(MessageId::NetlistExternalEyebrow),
        messages.text(MessageId::NetlistExternalTitle),
        messages.text(MessageId::NetlistExternalApply),
    )
    .description(messages.text(MessageId::NetlistExternalDescription))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .ghost(messages.text(MessageId::CommonCancel))
    .hint(hint)
    .show_with_initial_body_focus(ctx, |ui| {
        let t = Tokens::get(ctx);
        ui.monospace(review.path.display().to_string());
        let expected = digest(&review.expected_sha256);
        let observed = digest(&review.observed_sha256);
        ui.weak(messages.format(
            MessageId::NetlistExternalEvidenceSummary,
            &[
                ("expected", &expected),
                ("observed", &observed),
                ("encoding", review.external_encoding.label()),
            ],
        ));
        ui.add_space(8.0);
        ui.label(messages.text(MessageId::NetlistExternalResolution));
        let resolution = egui::ComboBox::from_id_salt("rspice.netlist.external-resolution")
            .selected_text(external_resolution_label(messages, review.resolution))
            .width(ui.available_width().max(1.0))
            .show_ui(ui, |ui| {
                for resolution in NetlistExternalChangeResolution::ALL {
                    ui.selectable_value(
                        &mut review.resolution,
                        resolution,
                        external_resolution_label(messages, resolution),
                    );
                }
            });
        ui.add_space(8.0);
        ui.label(egui::RichText::new(messages.text(MessageId::NetlistExternalEvidence)).strong());
        ui.label(if review.base_source.is_some() {
            let count = review.merge_conflict_count.to_string();
            messages.format(
                if review.merge_conflict_count == 1 {
                    MessageId::NetlistExternalMergeConflictsSingular
                } else {
                    MessageId::NetlistExternalMergeConflicts
                },
                &[("count", &count)],
            )
        } else {
            messages.text(MessageId::NetlistExternalNoBase)
        });
        let candidate = match review.resolution {
            NetlistExternalChangeResolution::Merge => &review.merged_source,
            NetlistExternalChangeResolution::KeepLocal => &review.local_source,
            NetlistExternalChangeResolution::ReloadExternal => &review.external_source,
        };
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(8)
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("rspice.netlist.external-candidate")
                    .max_height(150.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (line, text) in candidate.lines().take(200).enumerate() {
                            ui.monospace(format!("{:>5}  {text}", line + 1));
                        }
                        if candidate.lines().nth(200).is_some() {
                            ui.weak(messages.text(MessageId::NetlistExternalPreviewLimited));
                        }
                    });
            });
        ui.add_space(8.0);
        ui.label(egui::RichText::new(messages.text(MessageId::NetlistExternalComparison)).strong());
        egui::ScrollArea::vertical()
            .id_salt("rspice.netlist.external-diff")
            .max_height(120.0)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for line in review.comparison.lines().take(200) {
                    ui.monospace(line);
                }
            });
        if let Some(error) = &review.error {
            ui.add_space(6.0);
            ui.colored_label(t.color.err, error);
        }
        Some(resolution.response.id)
    });

    app.state.ui.netlist.external_change = Some(review);
    match choice {
        DialogChoice::Primary => {
            match crate::workbench::workflows::netlist_workflow::apply_staged_external_netlist_change(
                &mut app.state,
            ) {
                Ok(()) => {
                    app.state.push_user_message(ConsoleMessage::info(
                        "External source resolution applied as a journaled project revision.",
                    ));
                }
                Err(error) => {
                    if let Some(current) = app.state.ui.netlist.external_change.as_mut() {
                        current.error = Some(error);
                    }
                }
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.ui.netlist.external_change = None;
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

fn dependency_status_label(
    messages: MessageCatalog,
    count: usize,
    sealed: bool,
    external: bool,
) -> String {
    let id = match (external, sealed, count == 1) {
        (false, true, true) => MessageId::NetlistDependencySealedSingular,
        (false, true, false) => MessageId::NetlistDependencySealed,
        (false, false, true) => MessageId::NetlistDependencyResolutionSingular,
        (false, false, false) => MessageId::NetlistDependencyResolution,
        (true, true, true) => MessageId::NetlistExportExternalSealedSingular,
        (true, true, false) => MessageId::NetlistExportExternalSealed,
        (true, false, true) => MessageId::NetlistExportExternalResolutionSingular,
        (true, false, false) => MessageId::NetlistExportExternalResolution,
    };
    let count = count.to_string();
    messages.format(id, &[("count", &count)])
}

fn export_generated_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.export_dialog.open {
        return;
    }
    let mut dialog = app.state.ui.netlist.export_dialog.clone();
    let current = app.state.ui.netlist.generation_error.is_none()
        && app.state.ui.netlist.generated_document.is_some()
        && app.state.ui.netlist.generated_input_digest
            == app.state.ui.netlist.current_generation_input_digest;
    let (dependency_count, dependencies_sealed) = app
        .state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .map(|document| {
            (
                document.dependencies().len(),
                document.dependency_graph_is_sealed(),
            )
        })
        .unwrap_or_default();
    let requires_bundle = dependency_count > 0;
    if requires_bundle {
        dialog.bundle_dependencies = true;
    }
    let messages = app.state.ui.messages();
    let bundle_ready = !dialog.bundle_dependencies || dependencies_sealed;
    let primary = if dialog.bundle_dependencies {
        messages.text(MessageId::NetlistExportBundle)
    } else {
        messages.text(MessageId::NetlistExportDeck)
    };
    let footer_hint =
        dependency_status_label(messages, dependency_count, dependencies_sealed, false);
    let choice = Dialog::new(
        messages.text(MessageId::NetlistExportEyebrow),
        messages.text(MessageId::NetlistExportTitle),
        primary,
    )
    .description(messages.text(MessageId::NetlistExportDescription))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .primary_enabled(current && bundle_ready)
    .ghost(messages.text(MessageId::CommonCancel))
    .hint(&footer_hint)
    .show_with_initial_body_focus(ctx, |ui| {
        let t = Tokens::get(ctx);
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(10)
            .show(ui, |ui| {
                ui.label(messages.text(MessageId::NetlistExportSnapshotNotice));
            });
        ui.add_space(8.0);
        ui.label(messages.text(MessageId::NetlistDialect));
        let dialect = egui::ComboBox::from_id_salt("rspice.code.export-dialect")
            .selected_text(match dialog.format {
                crate::io::NetlistFormat::Spice => "SPICE",
                crate::io::NetlistFormat::Spectre => "Spectre",
                crate::io::NetlistFormat::Hspice => "HSPICE",
                crate::io::NetlistFormat::Xyce => "Xyce",
            })
            .width(ui.available_width().max(1.0))
            .show_ui(ui, |ui| {
                for (format, label) in [
                    (crate::io::NetlistFormat::Spice, "SPICE"),
                    (crate::io::NetlistFormat::Spectre, "Spectre"),
                    (crate::io::NetlistFormat::Hspice, "HSPICE"),
                    (crate::io::NetlistFormat::Xyce, "Xyce"),
                ] {
                    ui.selectable_value(&mut dialog.format, format, label);
                }
            });
        ui.add_space(6.0);
        let bundle_response = ui.add_enabled(
            !requires_bundle,
            egui::Checkbox::new(
                &mut dialog.bundle_dependencies,
                messages.text(MessageId::NetlistExportDependencyBundle),
            ),
        );
        if requires_bundle {
            bundle_response
                .clone()
                .on_disabled_hover_text(messages.text(MessageId::NetlistExportBundleRequired));
        }
        if bundle_response.changed() && !dialog.bundle_dependencies {
            dialog.include_source_map = false;
        }
        let source_map_supported = dialog.format == crate::io::NetlistFormat::Spice;
        let source_map_response = ui.add_enabled(
            source_map_supported,
            egui::Checkbox::new(
                &mut dialog.include_source_map,
                messages.text(MessageId::NetlistExportSourceMap),
            ),
        );
        if source_map_response.changed() && dialog.include_source_map {
            dialog.bundle_dependencies = true;
        }
        if !source_map_supported {
            dialog.include_source_map = false;
            source_map_response
                .on_disabled_hover_text(messages.text(MessageId::NetlistExportSourceMapSpiceOnly));
        }
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(10)
            .show(ui, |ui| {
                ui.label(dependency_status_label(
                    messages,
                    dependency_count,
                    dependencies_sealed,
                    true,
                ));
                ui.label(if dialog.bundle_dependencies {
                    messages.text(MessageId::NetlistExportBundleBehavior)
                } else {
                    messages.text(MessageId::NetlistExportDeckBehavior)
                });
            });
        if let Some(error) = &dialog.error {
            ui.colored_label(t.color.err, error);
        }
        Some(dialect.response.id)
    });
    match choice {
        DialogChoice::Primary => {
            if crate::workbench::menu_bar::action_export_generated_netlist_with_options(
                &mut app.state,
                dialog.format,
                dialog.bundle_dependencies,
                dialog.include_source_map,
                app.export_workflow_io.as_ref(),
            ) {
                dialog.open = false;
                dialog.error = None;
            } else {
                dialog.error =
                    Some("Export did not complete; review the application log.".to_owned());
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => dialog.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    app.state.ui.netlist.export_dialog = dialog;
}

fn import_operation_text(
    messages: MessageCatalog,
    operation: crate::workbench::documents::netlist_document::NetlistImportOperation,
) -> (String, String) {
    use crate::workbench::documents::netlist_document::NetlistImportOperation;

    match operation {
        NetlistImportOperation::OpenProject => (
            messages.text(MessageId::NetlistImportOpenTitle),
            messages.text(MessageId::NetlistImportOpen),
        ),
        NetlistImportOperation::ImportIntoProject => (
            messages.text(MessageId::NetlistImportDeckTitle),
            messages.text(MessageId::NetlistImportDeck),
        ),
        NetlistImportOperation::RequalifyOwnedSource => (
            messages.text(MessageId::NetlistImportReviewProfileTitle),
            messages.text(MessageId::NetlistImportRecordProfile),
        ),
    }
}

fn import_review_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    use crate::workbench::documents::netlist_document::{
        NetlistImportIssueSeverity, NetlistImportOperation,
    };

    let Some(mut review) = app.state.ui.netlist.import_review.clone() else {
        return;
    };
    let messages = app.state.ui.messages();
    let blocking = review.blocking_issue_count();
    let dialect_qualification_error = review.dialect_qualification().err();
    let qualified_execution_profile = dialect_qualification_error
        .is_none()
        .then(|| review.selected_dialect.execution_profile())
        .flatten();
    let compatibility_ready =
        !review.selected_dialect.requires_compatibility_review() || review.compatibility_accepted;
    let primary_enabled =
        blocking == 0 && dialect_qualification_error.is_none() && compatibility_ready;
    let digest = review
        .original_sha256
        .iter()
        .take(6)
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let hint = if blocking > 0 {
        let count = blocking.to_string();
        messages.format(
            if blocking == 1 {
                MessageId::NetlistImportBlockingSingular
            } else {
                MessageId::NetlistImportBlocking
            },
            &[("count", &count)],
        )
    } else if let Some(error) = dialect_qualification_error.as_deref() {
        error.to_owned()
    } else if !compatibility_ready {
        messages.text(MessageId::NetlistImportCompatibilityRequired)
    } else {
        messages.text(MessageId::NetlistImportReady)
    };
    let (title, primary) = import_operation_text(messages, review.operation);
    let choice = Dialog::new(
        messages.text(MessageId::NetlistImportEyebrow),
        title,
        primary,
    )
    .description(messages.text(MessageId::NetlistImportDescription))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .primary_enabled(primary_enabled)
    .ghost(messages.text(MessageId::CommonCancel))
    .hint(&hint)
    .show_with_initial_body_focus(ctx, |ui| {
        let t = Tokens::get(ctx);
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(10)
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(&review.display_name)
                        .font(theme::mono(tokens::FS_1, FontWeight::Medium)),
                );
                let source_kind =
                    if review.operation == NetlistImportOperation::RequalifyOwnedSource {
                        messages.text(MessageId::NetlistImportProjectSourceSnapshot)
                    } else if review.archive_import {
                        let count = review.dependencies.len().to_string();
                        messages.format(
                            if review.dependencies.len() == 1 {
                                MessageId::NetlistImportBundleSourceSingular
                            } else {
                                MessageId::NetlistImportBundleSource
                            },
                            &[("count", &count)],
                        )
                    } else {
                        messages.text(MessageId::NetlistImportLosslessSource)
                    };
                ui.label(source_kind);
                let byte_count = review.original_byte_count.to_string();
                ui.monospace(messages.format(
                    MessageId::NetlistImportSourceSummary,
                    &[
                        ("count", &byte_count),
                        ("digest", &digest),
                        ("encoding", review.encoding.label()),
                        ("line_ending", review.line_ending.label()),
                    ],
                ));
                let fallback_path = messages.text(MessageId::NetlistImportBrowserSnapshot);
                let invalid_path = messages.text(MessageId::NetlistImportInvalidUnicodePath);
                ui.weak(
                    review
                        .selected_file_path
                        .as_deref()
                        .map_or(fallback_path.as_str(), |path| {
                            path.to_str().unwrap_or(invalid_path.as_str())
                        }),
                );
            });

        ui.add_space(8.0);
        ui.label(messages.text(MessageId::NetlistSourceDialect));
        let dialect = egui::ComboBox::from_id_salt("rspice.netlist.import-dialect")
            .selected_text(review.selected_dialect.label())
            .width(ui.available_width().max(1.0))
            .show_ui(ui, |ui| {
                for dialect in crate::state::NetlistSourceDialect::ALL {
                    ui.selectable_value(&mut review.selected_dialect, dialect, dialect.label());
                }
            });
        ui.weak(messages.format(
            if review.detection_evidence.is_empty() {
                MessageId::NetlistImportDetectedNoMarker
            } else {
                MessageId::NetlistImportDetectedEvidence
            },
            &[("dialect", review.detected_dialect.label())],
        ));
        for evidence in &review.detection_evidence {
            ui.monospace(evidence);
        }
        if let Some(error) = dialect_qualification_error.as_deref() {
            ui.add_space(6.0);
            ui.colored_label(t.color.err, error);
        }

        ui.add_space(6.0);
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(8)
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(messages.text(MessageId::NetlistImportExecutionProfile))
                        .strong(),
                );
                if let Some(profile) = qualified_execution_profile {
                    ui.monospace(messages.format(
                        MessageId::NetlistImportExecutionProfileReceipt,
                        &[("profile", profile.id())],
                    ));
                } else {
                    ui.colored_label(
                        t.color.err,
                        messages.text(MessageId::NetlistImportNoExecutionProfile),
                    );
                }
            });

        if review.selected_dialect.requires_compatibility_review()
            && let Some(profile) = qualified_execution_profile
        {
            ui.add_space(6.0);
            ui.checkbox(
                &mut review.compatibility_accepted,
                messages.format(
                    MessageId::NetlistImportAcceptProfile,
                    &[
                        ("dialect", review.selected_dialect.label()),
                        ("profile", profile.id()),
                    ],
                ),
            )
            .on_hover_text(messages.text(MessageId::NetlistImportAcceptanceNotice));
        } else {
            review.compatibility_accepted = false;
        }

        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(messages.text(MessageId::NetlistImportTransformations)).strong(),
        );
        for transformation in &review.transformations {
            ui.label(format!("- {transformation}"));
        }

        ui.add_space(8.0);
        ui.label(egui::RichText::new(messages.text(MessageId::NetlistImportValidation)).strong());
        if review.issues.is_empty() {
            ui.colored_label(
                t.color.ok,
                messages.text(MessageId::NetlistImportValidationPassed),
            );
        } else {
            for issue in &review.issues {
                let (prefix, color) = match issue.severity {
                    NetlistImportIssueSeverity::Advisory => (
                        messages.text(MessageId::NetlistImportAdvisory),
                        t.color.warn,
                    ),
                    NetlistImportIssueSeverity::Blocking => {
                        (messages.text(MessageId::NetlistImportBlocked), t.color.err)
                    }
                };
                ui.colored_label(color, format!("{prefix}: {}", issue.message));
            }
        }

        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(messages.text(MessageId::NetlistImportSourcePreview)).strong(),
        );
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(8)
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("rspice.netlist.import-preview")
                    .max_height(160.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (line, text) in review.source.lines().take(200).enumerate() {
                            ui.monospace(format!("{:>5}  {text}", line + 1));
                        }
                        if review.source.lines().nth(200).is_some() {
                            ui.weak(messages.text(MessageId::NetlistImportPreviewLimited));
                        }
                    });
            });
        if let Some(error) = &review.error {
            ui.add_space(6.0);
            ui.colored_label(t.color.err, error);
        }
        Some(dialect.response.id)
    });

    // Persist review choices before a commit attempt revalidates the exact
    // lifecycle transaction and candidate snapshot.
    app.state.ui.netlist.import_review = Some(review);
    match choice {
        DialogChoice::Primary => {
            crate::workbench::workflows::netlist_workflow::commit_staged_netlist_import(
                &mut app.state,
            );
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            crate::workbench::workflows::netlist_workflow::cancel_staged_netlist_import(
                &mut app.state,
            );
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

fn create_owned_source(
    state: &mut AppState,
    artifact_name: &str,
    strategy: crate::state::OwnedNetlistEditStrategy,
) -> Result<(), String> {
    let name = artifact_name.trim();
    if name.is_empty()
        || name != artifact_name
        || name.chars().any(char::is_control)
        || name.contains('/')
        || name.contains('\\')
    {
        return Err("Artifact name must be one trimmed file name.".to_owned());
    }
    if state.workspace.netlist_source.is_some() {
        return Err("An owned SPICE source artifact already exists.".to_owned());
    }
    let generated = state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .ok_or_else(|| "No current generated artifact is available.".to_owned())?;
    let source = initial_owned_source(generated.source(), strategy);
    let mut owned = generated
        .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
        .map_err(|error| error.to_string())?;
    owned
        .replace_editable_source(owned.content_digest(), source.as_bytes().to_vec())
        .map_err(|error| error.to_string())?;

    state.workspace.netlist_source = Some(source.clone());
    state.workspace.netlist_source_path = None;
    state.workspace.netlist_source_dirty = true;
    state.workspace.netlist_document = Some(owned.clone());
    let mut descriptor = crate::state::OwnedNetlistDescriptor {
        artifact_name: name.to_owned(),
        strategy,
        source_encoding: crate::state::NetlistTextEncoding::Utf8,
        source_line_ending: crate::state::NetlistLineEnding::detect(&source),
        imported_dialect: None,
        compatibility_reviewed: false,
        execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
        external_file_sha256: None,
        save_history: Vec::new(),
        revision_history: Vec::new(),
        owned_includes: Vec::new(),
    };
    descriptor.retain_revision(&owned, "Created editable source baseline")?;
    state.workspace.netlist_descriptor = Some(descriptor);
    state.ui.netlist.owned_document = Some(owned);
    state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_document_initialized = true;
    state.ui.netlist.externally_saved_content_digest = None;
    state.simulation.netlist_content = source;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    crate::workbench::documents::netlist_document::invalidate_source_evidence(
        &mut state.ui.netlist,
    );
    Ok(())
}

fn initial_owned_source(
    generated: &str,
    strategy: crate::state::OwnedNetlistEditStrategy,
) -> String {
    match strategy {
        crate::state::OwnedNetlistEditStrategy::OwnedSource => generated.to_owned(),
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => select_source_cards(
            generated,
            "* Parameter and option override derived from generated source",
            |head| matches!(head, ".param" | ".option" | ".options" | ".temp"),
        ),
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => select_source_cards(
            generated,
            "* Include-order override derived from generated source",
            |head| matches!(head, ".include" | ".inc" | ".lib" | ".veriloga"),
        ),
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => select_source_cards(
            generated,
            "* Analysis-only deck derived from generated source",
            is_analysis_card,
        ),
    }
}

fn select_source_cards(source: &str, heading: &str, select: impl Fn(&str) -> bool) -> String {
    let mut selected = Vec::<String>::new();
    let mut retain_continuation = false;
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('+') {
            if retain_continuation {
                selected.push(line.to_owned());
            }
            continue;
        }
        let head = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        retain_continuation = select(&head);
        if retain_continuation {
            selected.push(line.to_owned());
        }
    }
    let mut result = heading.to_owned();
    result.push('\n');
    if selected.is_empty() {
        result.push_str("* No matching cards were present in the base revision.\n");
    } else {
        result.push_str(&selected.join("\n"));
        result.push('\n');
    }
    result
}

fn is_analysis_card(head: &str) -> bool {
    matches!(
        head,
        ".op"
            | ".tran"
            | ".ac"
            | ".dc"
            | ".noise"
            | ".tf"
            | ".pz"
            | ".sens"
            | ".four"
            | ".sp"
            | ".hb"
            | ".pss"
            | ".pac"
            | ".pnoise"
            | ".stb"
            | ".measure"
            | ".meas"
            | ".save"
            | ".probe"
    )
}

fn find_replace_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.find.open {
        return;
    }

    use crate::state::{
        FindDirection, FindOptions, ReplaceScope, find_all_in_source, replace_in_source,
    };
    use crate::workbench::documents::netlist_document::NetlistFindScope;

    let owned = crate::workbench::documents::netlist_document::active_netlist_source_is_editable(
        &app.state,
    );
    let messages = app.state.ui.messages();
    let mut find = app.state.ui.netlist.find.clone();
    let options = FindOptions {
        direction: FindDirection::Forward,
        match_case: find.match_case,
        whole_word: find.whole_symbol,
        regular_expression: find.regular_expression,
    };
    let documents = netlist_search_documents(&app.state, find.scope);
    let matches: Result<Vec<NetlistSearchMatch>, crate::state::FindError> = if find.find.is_empty()
    {
        Ok(Vec::new())
    } else {
        documents
            .iter()
            .try_fold(Vec::<NetlistSearchMatch>::new(), |mut all, document| {
                all.extend(
                    find_all_in_source(&document.source, &find.find, options)?
                        .into_iter()
                        .map(|found| NetlistSearchMatch {
                            document: document.clone(),
                            found,
                        }),
                );
                Ok(all)
            })
    };
    find.error = matches.as_ref().err().map(ToString::to_string);
    let matches = matches.unwrap_or_default();
    if matches.is_empty() {
        find.selected_match = 0;
    } else {
        find.selected_match = find.selected_match.min(matches.len() - 1);
    }

    let mut action = None;
    let has_matches = !matches.is_empty();
    let compact_fields = ctx.content_rect().width() < 360.0;
    let find_hint = if find.find.is_empty() {
        messages.text(MessageId::NetlistFindEnterTextHint)
    } else if find.error.is_some() {
        messages.text(MessageId::NetlistFindCorrectExpressionHint)
    } else {
        messages.text(MessageId::NetlistFindGeneratedImmutableHint)
    };
    let choice = Dialog::new(
        messages.text(MessageId::NetlistFindEyebrow),
        messages.text(MessageId::NetlistFindTitle),
        messages.text(MessageId::NetlistFindNext),
    )
    .description(messages.text(MessageId::NetlistFindDescription))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .primary_enabled(has_matches && find.error.is_none())
    .ghost(messages.text(MessageId::CommonClose))
    .hint(find_hint)
    .show_with_initial_body_focus(ctx, |ui| {
        let mut find_control_id = None;
        if compact_fields {
            ui.label(messages.text(MessageId::CommonFind));
            let response = ui.add(
                egui::TextEdit::singleline(&mut find.find)
                    .desired_width(ui.available_width())
                    .hint_text(messages.text(MessageId::NetlistFindExpressionHint)),
            );
            find_control_id = Some(response.id);
            ui.label(messages.text(MessageId::CommonReplace));
            ui.add_enabled(
                (owned || find.scope == NetlistFindScope::AllOwnedSources)
                    && find.scope != NetlistFindScope::ProjectReferences,
                egui::TextEdit::singleline(&mut find.replacement)
                    .desired_width(ui.available_width()),
            );
            ui.label(messages.text(MessageId::CommonScope));
            find_scope_combo(
                ui,
                &mut find,
                app.state.workspace.netlist_source.is_some(),
                messages,
            );
        } else {
            egui::Grid::new("rspice.code.find-fields")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label(messages.text(MessageId::CommonFind));
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut find.find)
                            .desired_width(ui.available_width().max(180.0))
                            .hint_text(messages.text(MessageId::NetlistFindExpressionHint)),
                    );
                    find_control_id = Some(response.id);
                    ui.end_row();
                    ui.label(messages.text(MessageId::CommonReplace));
                    ui.add_enabled(
                        (owned || find.scope == NetlistFindScope::AllOwnedSources)
                            && find.scope != NetlistFindScope::ProjectReferences,
                        egui::TextEdit::singleline(&mut find.replacement)
                            .desired_width(ui.available_width().max(180.0)),
                    );
                    ui.end_row();
                    ui.label(messages.text(MessageId::CommonScope));
                    find_scope_combo(
                        ui,
                        &mut find,
                        app.state.workspace.netlist_source.is_some(),
                        messages,
                    );
                    ui.end_row();
                });
        }

        ui.horizontal_wrapped(|ui| {
            ui.checkbox(
                &mut find.match_case,
                messages.text(MessageId::NetlistFindMatchCase),
            );
            ui.checkbox(
                &mut find.whole_symbol,
                messages.text(MessageId::NetlistFindWholeSymbol),
            );
            ui.checkbox(
                &mut find.regular_expression,
                messages.text(MessageId::NetlistFindRegularExpression),
            );
        });
        ui.separator();

        if let Some(error) = &find.error {
            ui.colored_label(Tokens::get(ctx).color.err, error);
        } else if find.find.is_empty() {
            ui.weak(messages.text(MessageId::NetlistFindEnterExactArtifact));
        } else {
            let count = matches.len().to_string();
            ui.label(messages.format(
                if matches.len() == 1 {
                    MessageId::NetlistFindMatchSingular
                } else {
                    MessageId::NetlistFindMatches
                },
                &[("count", &count)],
            ));
        }

        egui::ScrollArea::vertical()
            .id_salt("rspice.code.find-results")
            .max_height(168.0)
            .show(ui, |ui| {
                let show_document =
                    documents.len() > 1 || find.scope != NetlistFindScope::CurrentDocument;
                for (index, result) in matches.iter().enumerate() {
                    let line_text = result
                        .document
                        .source
                        .lines()
                        .nth(result.found.line().saturating_sub(1))
                        .unwrap_or_default()
                        .trim();
                    let location = if show_document {
                        format!(
                            "{}  {}:{}",
                            result.document.label,
                            result.found.line(),
                            result.found.column()
                        )
                    } else {
                        format!("{}:{}", result.found.line(), result.found.column())
                    };
                    if ui
                        .add_sized(
                            [ui.available_width(), 24.0],
                            egui::Button::selectable(
                                find.selected_match == index,
                                format!("{location}  {line_text}"),
                            ),
                        )
                        .clicked()
                    {
                        action = Some(FindWindowAction::Select(index));
                    }
                }
            });

        ui.separator();
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    has_matches,
                    egui::Button::new(messages.text(MessageId::NetlistFindPrevious)),
                )
                .clicked()
            {
                let next = if find.selected_match == 0 {
                    matches.len().saturating_sub(1)
                } else {
                    find.selected_match - 1
                };
                action = Some(FindWindowAction::Select(next));
            }
            let selected_owned = matches
                .get(find.selected_match)
                .is_some_and(|result| result.document.editable);
            let replace_enabled = find.scope != NetlistFindScope::ProjectReferences
                && has_matches
                && selected_owned
                && find.error.is_none();
            if ui
                .add_enabled(
                    replace_enabled,
                    egui::Button::new(messages.text(MessageId::CommonReplace)),
                )
                .clicked()
            {
                action = Some(FindWindowAction::ReplaceNext);
            }
            if ui
                .add_enabled(
                    replace_enabled,
                    egui::Button::new(messages.text(MessageId::NetlistFindReplaceAll)),
                )
                .clicked()
            {
                action = Some(FindWindowAction::ReplaceAll);
            }
        });
        find_control_id
    });
    match choice {
        DialogChoice::Primary => {
            action = Some(FindWindowAction::Select(
                (find.selected_match + 1) % matches.len().max(1),
            ));
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => find.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    app.state.ui.netlist.find = find;

    match action {
        Some(FindWindowAction::Select(index)) => {
            if let Some(result) = matches.get(index) {
                app.state.ui.netlist.find.selected_match = index;
                match result.document.active_document {
                    ActiveNetlistDocument::Generated => {
                        let _ =
                            crate::workbench::documents::netlist_document::open_generated_primary(
                                &mut app.state,
                            );
                    }
                    ActiveNetlistDocument::OwnedSource => {
                        let _ = open_owned_source(&mut app.state);
                    }
                    ActiveNetlistDocument::GeneratedDiff => {}
                }
                if let Some(identity) = result.document.dependency_identity.as_deref()
                    && let Err(error) =
                        crate::workbench::documents::netlist_document::open_netlist_dependency(
                            &mut app.state,
                            identity,
                        )
                {
                    app.state.push_user_message(ConsoleMessage::error(error));
                    return;
                }
                app.state.ui.netlist.requested_line = Some(result.found.line());
            }
        }
        Some(FindWindowAction::ReplaceNext) | Some(FindWindowAction::ReplaceAll) => {
            let selected = matches.get(app.state.ui.netlist.find.selected_match);
            let Some(selected) = selected.filter(|result| result.document.editable) else {
                return;
            };
            if selected.document.active_document == ActiveNetlistDocument::OwnedSource
                && app.state.ui.netlist.active_document != ActiveNetlistDocument::OwnedSource
            {
                let _ = open_owned_source(&mut app.state);
            }
            if let Some(identity) = selected.document.dependency_identity.as_deref()
                && let Err(error) =
                    crate::workbench::documents::netlist_document::open_netlist_dependency(
                        &mut app.state,
                        identity,
                    )
            {
                app.state.push_user_message(ConsoleMessage::error(error));
                return;
            }
            let scope = if matches!(action, Some(FindWindowAction::ReplaceAll)) {
                ReplaceScope::All
            } else {
                ReplaceScope::Next {
                    caret_byte: selected.found.byte_range().start,
                }
            };
            match replace_in_source(
                &selected.document.source,
                &app.state.ui.netlist.find.find,
                &app.state.ui.netlist.find.replacement,
                options,
                scope,
            ) {
                Ok(outcome) => {
                    let count = outcome.replacement_count();
                    let replacement = outcome.into_source();
                    let replaced = if selected.document.dependency_identity.is_some() {
                        crate::workbench::documents::netlist_document::replace_owned_dependency_source(
                            &mut app.state,
                            replacement,
                        )
                    } else {
                        crate::workbench::documents::netlist_document::replace_owned_source(
                            &mut app.state,
                            replacement,
                        )
                    };
                    if count > 0 && replaced {
                        app.state.ui.netlist.find.selected_match = 0;
                        let count_text = count.to_string();
                        app.state
                            .push_user_message(ConsoleMessage::info(messages.format(
                                if count == 1 {
                                    MessageId::NetlistFindReplacedSingular
                                } else {
                                    MessageId::NetlistFindReplaced
                                },
                                &[("count", &count_text)],
                            )));
                    }
                }
                Err(error) => app.state.ui.netlist.find.error = Some(error.to_string()),
            }
        }
        None => {}
    }
}

fn find_scope_combo(
    ui: &mut Ui,
    find: &mut crate::workbench::documents::netlist_document::NetlistFindState,
    has_owned_source: bool,
    messages: MessageCatalog,
) {
    use crate::workbench::documents::netlist_document::NetlistFindScope;

    egui::ComboBox::from_id_salt("rspice.code.find-scope")
        .selected_text(match find.scope {
            NetlistFindScope::CurrentDocument => {
                messages.text(MessageId::NetlistFindCurrentDocument)
            }
            NetlistFindScope::AllOwnedSources => {
                messages.text(MessageId::NetlistFindAllOwnedSources)
            }
            NetlistFindScope::ProjectReferences => {
                messages.text(MessageId::NetlistFindProjectReferences)
            }
        })
        .width(ui.available_width().max(180.0))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut find.scope,
                NetlistFindScope::CurrentDocument,
                messages.text(MessageId::NetlistFindCurrentDocument),
            );
            if has_owned_source {
                ui.selectable_value(
                    &mut find.scope,
                    NetlistFindScope::AllOwnedSources,
                    messages.text(MessageId::NetlistFindAllOwnedSources),
                );
            }
            ui.selectable_value(
                &mut find.scope,
                NetlistFindScope::ProjectReferences,
                messages.text(MessageId::NetlistFindProjectReferences),
            );
        });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocumentStatusTone {
    Valid,
    Warning,
    Error,
}

fn document_syntax_status(state: &AppState) -> (String, DocumentStatusTone) {
    let messages = state.ui.messages();
    if state.ui.netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        return (
            messages.text(MessageId::NetlistComparisonReady),
            DocumentStatusTone::Valid,
        );
    }
    if state.ui.netlist.active_document == ActiveNetlistDocument::Generated
        && (state.ui.netlist.generation_error.is_some()
            || !generated_primary_ready(state)
            || state.ui.netlist.generated_input_digest
                != state.ui.netlist.current_generation_input_digest)
    {
        let retained_artifact = generated_primary_ready(state);
        return (
            if retained_artifact {
                messages.text(MessageId::NetlistGenerationStaleBlocked)
            } else {
                messages.text(MessageId::NetlistGenerationBlocked)
            },
            DocumentStatusTone::Warning,
        );
    }
    let errors = state
        .ui
        .netlist
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.is_current()
                && diagnostic.severity
                    == crate::workbench::documents::netlist_document::DiagnosticSeverity::Error
        })
        .count();
    if errors > 0 {
        let count = errors.to_string();
        return (
            messages.format(
                if errors == 1 {
                    MessageId::NetlistSyntaxErrorSingular
                } else {
                    MessageId::NetlistSyntaxErrors
                },
                &[("count", &count)],
            ),
            DocumentStatusTone::Error,
        );
    }
    (
        messages.text(MessageId::NetlistSyntaxValid),
        DocumentStatusTone::Valid,
    )
}

/// Open (or explicitly create) the project-owned source derived from the
/// immutable generated primary. Existing owned bytes are never overwritten.
fn open_owned_source(state: &mut AppState) -> bool {
    if state.workspace.netlist_source.is_none() {
        return false;
    }
    if state.ui.netlist.owned_document.is_none() {
        let Some(generated) = state.ui.netlist.generated_document.as_ref() else {
            return false;
        };
        let source = state
            .workspace
            .netlist_source
            .as_deref()
            .unwrap_or_else(|| generated.source());
        let mut owned = match generated
            .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
        {
            Ok(document) => document,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "Could not create owned SPICE source: {error}"
                )));
                return false;
            }
        };
        let transition = if let Some(path) = state.workspace.netlist_source_path.as_ref() {
            let display_name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string());
            crate::state::SourceLocator::try_new(path.display().to_string(), display_name)
                .and_then(|locator| locator.with_native_origin(path.display().to_string()))
                .and_then(|locator| {
                    owned.import_source(owned.content_digest(), locator, source.as_bytes().to_vec())
                })
                .and_then(|_| owned.make_editable(owned.content_digest()))
        } else {
            owned.replace_editable_source(owned.content_digest(), source.as_bytes().to_vec())
        };
        if let Err(error) = transition {
            state.push_user_message(ConsoleMessage::error(format!(
                "Could not initialize owned SPICE source: {error}"
            )));
            return false;
        }
        state.ui.netlist.owned_document = Some(owned);
    }
    let Some(source) = state
        .ui
        .netlist
        .owned_document
        .as_ref()
        .map(|document| document.source().to_owned())
    else {
        return false;
    };
    state.workspace.netlist_source = Some(source.clone());
    state.workspace.netlist_document = state.ui.netlist.owned_document.clone();
    if state.workspace.netlist_descriptor.is_none() {
        let artifact_name = state
            .workspace
            .netlist_source_path
            .as_deref()
            .and_then(std::path::Path::file_name)
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "top_override.sp".to_owned());
        state.workspace.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
            artifact_name,
            strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
            source_encoding: crate::state::NetlistTextEncoding::Utf8,
            source_line_ending: crate::state::NetlistLineEnding::detect(&source),
            imported_dialect: None,
            compatibility_reviewed: false,
            execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
            external_file_sha256: None,
            save_history: Vec::new(),
            revision_history: Vec::new(),
            owned_includes: Vec::new(),
        });
    }
    state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_dependency_identity = None;
    state.ui.netlist.active_dependency_root = None;
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = source;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    state.ui.netlist.completion_open = false;
    state.ui.netlist.completion_dismissed_at = None;
    crate::workbench::documents::netlist_document::invalidate_source_evidence(
        &mut state.ui.netlist,
    );
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_netlist_format_preserves_line_ending_policy() {
        assert_eq!(
            normalize_owned_netlist_whitespace("deck  \r\nR1 1 0 1k\t\r\n.end"),
            "deck  \r\nR1 1 0 1k\r\n.end\r\n"
        );
        assert_eq!(
            normalize_owned_netlist_whitespace("deck\n.end\n"),
            "deck\n.end\n"
        );
    }

    fn configured_veriloga_state() -> (AppState, String) {
        let mut state = AppState::default();
        let reference = crate::state::CellViewRef::new("behavioral", "gain", "veriloga");
        let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), "sealed_gain".to_owned());
        view.metadata
            .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
        let mut cell = crate::state::Cell::new("gain");
        cell.add_view(view);
        let mut library = crate::state::Library::new("behavioral");
        library.add_cell(cell);
        state.library_manager.add_library(library);

        let bundle = crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::cell_view(reference),
            crate::state::ProjectSourceLanguage::VerilogA,
            "behavioral/gain.va",
            "`ifdef NEVER\n`include \"behavioral/inactive.va\"\n`endif\n`include \"behavioral/gain_constants.va\"\nmodule sealed_gain(p, n); inout p, n; electrical p, n; analog I(p,n) <+ `RSPICE_GAIN * V(p,n); endmodule\n",
            [
                crate::state::ProjectSourceFile::try_new(
                    "behavioral/gain_constants.va",
                    "`define RSPICE_GAIN 1.0\n",
                )
                .expect("valid included source"),
                crate::state::ProjectSourceFile::try_new(
                    "behavioral/inactive.va",
                    "module must_not_enter_provenance; endmodule\n",
                )
                .expect("valid inactive source"),
            ],
            [
                crate::state::ProjectSourceDependency::try_new(
                    "behavioral/gain.va",
                    "behavioral/gain_constants.va",
                )
                .expect("valid dependency edge"),
                crate::state::ProjectSourceDependency::try_new(
                    "behavioral/gain.va",
                    "behavioral/inactive.va",
                )
                .expect("valid inactive dependency edge"),
            ],
        )
        .expect("valid source closure");
        state
            .workspace
            .project_sources
            .insert_bundle(bundle)
            .expect("attach cell-view source");

        let mut placed = crate::state::LibraryCellInstance::new("behavioral", "gain", "schematic");
        placed.terminal_order = vec!["p".to_owned(), "n".to_owned()];
        state
            .schematic
            .add_library_cell_component(crate::state::Point::new(20, 20), placed);
        state
            .workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Mixed-signal".to_owned(),
                root: crate::state::CellViewRef::default_top(),
                dut_path: "/top/X1".to_owned(),
                executable_view_policy: vec!["veriloga".to_owned()],
                stop_views: vec!["veriloga".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Mixed-signal design".to_owned(),
            })
            .expect("create mixed-signal configuration");
        let projection = state
            .workspace
            .configuration_execution_projection(
                &state.library_manager,
                &state.workspace.active_view,
                &state.schematic,
            )
            .expect("resolve configured behavioral view");
        let source_key = projection
            .plan()
            .and_then(|plan| plan.binding("/top/X1"))
            .and_then(|binding| binding.project_veriloga())
            .expect("project Verilog-A binding")
            .source_key()
            .to_owned();
        (state, source_key)
    }

    fn retain_generated(state: &mut AppState, source: &str) {
        let input_digest = crate::product::ContentDigest::from_bytes([0x41; 32]);
        let source = source.to_owned();
        let (document, owned) = publish_generated_document(state, input_digest, source.clone())
            .expect("canonical generated fixture");
        state.ui.netlist.generated_source = source;
        state.ui.netlist.generated_document = Some(document);
        state.ui.netlist.owned_document = owned;
        state.ui.netlist.generated_input_digest = Some(input_digest);
        state.ui.netlist.current_generation_input_digest = Some(input_digest);
    }

    #[test]
    fn editable_source_and_generated_primary_coexist_without_overwrite() {
        let mut state = AppState::default();
        retain_generated(&mut state, "generated\n.end\n");
        let retained_generated = state.ui.netlist.generated_source.clone();
        state.simulation.netlist_content = state.ui.netlist.generated_source.clone();

        create_owned_source(
            &mut state,
            "top_override.sp",
            crate::state::OwnedNetlistEditStrategy::OwnedSource,
        )
        .expect("create owned source");
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some(retained_generated.as_str())
        );
        assert!(
            crate::workbench::documents::netlist_document::replace_owned_source(
                &mut state,
                "owned edit\n.end\n".to_owned()
            )
        );

        assert!(crate::workbench::documents::netlist_document::open_generated_primary(&mut state));
        assert_eq!(state.simulation.netlist_content, retained_generated);
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("owned edit\n.end\n")
        );
        assert!(!crate::workbench::documents::netlist_document::open_generated_primary(&mut state));
    }

    #[test]
    fn opening_existing_owned_source_never_overwrites_its_bytes() {
        let mut state = AppState::default();
        retain_generated(&mut state, "new generated\n.end\n");
        state.workspace.netlist_source = Some("retained owned\n.end\n".to_owned());

        assert!(open_owned_source(&mut state));
        assert_eq!(state.simulation.netlist_content, "retained owned\n.end\n");
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("retained owned\n.end\n")
        );
    }

    #[test]
    fn toolbar_geometry_matches_the_mockup_contract() {
        assert_eq!(CODE_TOOLBAR_HEIGHT, 33.0);
        assert_eq!(CODE_TOOLBAR_PADDING_X, 8.0);
        assert_eq!(CODE_TOOLBAR_GAP, 5.0);
        assert_eq!(CODE_TOOLBAR_ACTION_GUTTER, 12.0);
        assert_eq!(CODE_TOOLBAR_COMPACT_BREAKPOINT, 720.0);
        assert_eq!(PHONE_BREAKPOINT, 560.0);
        assert_eq!(PHONE_PRIMARY_WIDTH, 154.0);
        assert_eq!(PHONE_ACTION_WIDTH, 250.0);
        assert!(code_toolbar_compact(607.0));
        assert!(!code_toolbar_compact(721.0));
        assert!(!toolbar_status_visible(true, DocumentStatusTone::Warning));
        assert!(toolbar_status_visible(true, DocumentStatusTone::Error));
        assert!(toolbar_status_visible(false, DocumentStatusTone::Warning));
        assert!(toolbar_advisory_fits(500.0, 260.0, 110.0, 70.0));
        assert!(
            !toolbar_advisory_fits(430.0, 260.0, 110.0, 70.0),
            "advisory must yield before language, blocking status, or actions clip"
        );
        let content = egui::Rect::from_min_size(egui::pos2(8.0, 0.0), vec2(526.0, 33.0));
        let (status_and_language, actions) = code_toolbar_regions(content, 342.0);
        assert_eq!(actions.right(), content.right());
        assert_eq!(
            actions.left() - status_and_language.right(),
            CODE_TOOLBAR_ACTION_GUTTER
        );
        assert!(status_and_language.right() < actions.left());
    }

    #[test]
    fn empty_generated_primary_reports_blocked_without_claiming_staleness() {
        let mut state = AppState::default();
        state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
        state.ui.netlist.generation_error =
            Some("Add a circuit before generating the primary netlist.".to_owned());

        assert!(generated_primary_unavailable(&state));
        assert!(!generated_primary_ready(&state));
        assert_eq!(
            document_syntax_status(&state),
            ("generation blocked".to_owned(), DocumentStatusTone::Warning)
        );
    }

    #[test]
    fn retained_generated_primary_reports_stale_when_regeneration_is_blocked() {
        let mut state = AppState::default();
        retain_generated(&mut state, "retained\n.end\n");
        state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
        state.ui.netlist.generation_error = Some("Regeneration failed.".to_owned());

        assert!(!generated_primary_unavailable(&state));
        assert!(generated_primary_ready(&state));
        assert_eq!(
            document_syntax_status(&state),
            (
                "stale · generation blocked".to_owned(),
                DocumentStatusTone::Warning
            )
        );
    }

    #[test]
    fn split_generated_state_fails_closed_as_unavailable() {
        let mut state = AppState::default();
        retain_generated(&mut state, "retained\n.end\n");
        state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
        state.ui.netlist.generated_source.clear();
        state.ui.netlist.generation_error = None;

        assert!(state.ui.netlist.generated_document.is_some());
        assert!(generated_primary_unavailable(&state));
        assert!(!generated_primary_ready(&state));
        assert!(!active_document_available(&state));
        assert_eq!(
            document_syntax_status(&state),
            ("generation blocked".to_owned(), DocumentStatusTone::Warning)
        );
    }

    #[test]
    fn generated_deck_does_not_inject_unreferenced_code_workspace_veriloga() {
        let state = AppState::default();
        let source = "R1 1 0 1k\n.end\n".to_owned();

        let dependencies = generated_project_source_dependencies(&state, &source)
            .expect("unreferenced source does not create a dependency");
        assert!(dependencies.is_empty());
    }

    #[test]
    fn generated_deck_retains_the_exact_transitive_cell_view_source_closure() {
        let (state, source_key) = configured_veriloga_state();
        let source = format!("configured deck\n.veriloga \"{source_key}\" sealed_gain\n.end\n");

        let dependencies = generated_project_source_dependencies(&state, &source)
            .expect("retain exact project source closure");

        assert_eq!(dependencies.len(), 2);
        assert!(dependencies.iter().all(|dependency| {
            !dependency
                .resolution()
                .source()
                .is_some_and(|source| source.contains("must_not_enter_provenance"))
        }));
        let root = dependencies
            .iter()
            .find(|dependency| dependency.direct_include_index().is_some())
            .expect("direct root dependency");
        assert_eq!(root.requested_locator(), source_key);
        assert!(root.parent().is_none());
        assert!(
            root.resolution()
                .source()
                .unwrap()
                .contains("module sealed_gain")
        );
        let included = dependencies
            .iter()
            .find(|dependency| dependency.parent().is_some())
            .expect("transitive included dependency");
        assert_eq!(included.requested_locator(), "behavioral/gain_constants.va");
        assert_eq!(
            included.resolution().source(),
            Some("`define RSPICE_GAIN 1.0\n")
        );
    }
}
