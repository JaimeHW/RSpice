use crate::common::app::{AppState, ConsoleMessage, RSpiceApp};
use crate::common::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

pub const NETLIST_FILTER: (&str, &[&str]) = ("SPICE Deck", &["cir", "sp", "spice", "net", "ckt"]);

/// Validate and retain the exact visible manual-deck snapshot, including its
/// sealed dependency closure and execution-target contract. A later Run must
/// match this one-shot authorized snapshot byte for byte.
pub(crate) fn validate_visible_netlist_source(app: &mut RSpiceApp) -> bool {
    use crate::workbench::netlist_document::{
        ActiveNetlistDocument, NetlistValidationReceipt, source_content_digest,
    };

    if app.state.ui.netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        app.state.push_user_message(ConsoleMessage::warning(
            "Revision comparisons are read-only review documents and cannot be validated for execution.",
        ));
        return false;
    }
    if app.state.ui.netlist.active_document == ActiveNetlistDocument::Generated
        && (app.state.ui.netlist.generation_error.is_some()
            || app.state.ui.netlist.generated_input_digest
                != app.state.ui.netlist.current_generation_input_digest)
    {
        let error = app
            .state
            .ui
            .netlist
            .generation_error
            .clone()
            .unwrap_or_else(|| "Generated netlist is stale.".to_owned());
        app.state.ui.netlist.validation = None;
        app.state.ui.netlist.validation_error = Some(error.clone());
        app.state.push_user_message(ConsoleMessage::error(format!(
            "Netlist validation failed: {error}"
        )));
        return false;
    }

    let visible_digest = source_content_digest(&app.state.simulation.netlist_content);
    match app
        .simulation_controller
        .validate_manual_deck_document(&app.state)
    {
        Ok(metadata) => {
            if let Err(error) = acknowledge_canonical_dependencies(
                &mut app.state,
                &metadata.sealed_source_dependencies,
            ) {
                app.simulation_controller.clear_prepared_run();
                app.state.ui.netlist.validation = None;
                app.state.ui.netlist.validation_error = Some(error.clone());
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "Netlist validation failed: {error}"
                )));
                return false;
            }
            if let Err(error) = acknowledge_canonical_validation(&mut app.state) {
                app.simulation_controller.clear_prepared_run();
                app.state.ui.netlist.validation = None;
                app.state.ui.netlist.validation_error = Some(error.clone());
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "Netlist validation failed: {error}"
                )));
                return false;
            }
            app.state.ui.netlist.validation = Some(NetlistValidationReceipt {
                visible_content_digest: visible_digest,
                executable_source_digest: metadata.source_digest,
                prepared_snapshot_digest: metadata.snapshot_digest,
                project_revision: metadata.project_revision,
                task_count: metadata.task_count,
                advisory_count: metadata.advisories.len(),
            });
            app.state.ui.netlist.validation_error = None;
            app.state.push_user_message(ConsoleMessage::info(format!(
                "Validated exact visible SPICE source: {} task{} · authored {} · executable {}",
                metadata.task_count,
                if metadata.task_count == 1 { "" } else { "s" },
                short_digest(visible_digest),
                short_digest(metadata.source_digest),
            )));
            true
        }
        Err(error) => {
            let message = error.to_string();
            invalidate_canonical_validation(&mut app.state);
            app.state.ui.netlist.validation = None;
            app.state.ui.netlist.validation_error = Some(message.clone());
            app.state.push_user_message(ConsoleMessage::error(format!(
                "Netlist validation failed: {message}"
            )));
            false
        }
    }
}

fn acknowledge_canonical_dependencies(
    state: &mut AppState,
    sealed: &[rspice_core::netlist::ResolvedIncludeDependency],
) -> Result<(), String> {
    let dependencies_belong_to_generated_base = state.ui.netlist.active_document
        == crate::workbench::netlist_document::ActiveNetlistDocument::OwnedSource
        && state
            .workspace
            .netlist_descriptor
            .as_ref()
            .is_some_and(|descriptor| {
                matches!(
                    descriptor.strategy,
                    crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride
                        | crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck
                )
            });
    let document = match state.ui.netlist.active_document {
        crate::workbench::netlist_document::ActiveNetlistDocument::Generated => {
            state.ui.netlist.generated_document.as_mut()
        }
        crate::workbench::netlist_document::ActiveNetlistDocument::OwnedSource => {
            state.ui.netlist.owned_document.as_mut()
        }
        crate::workbench::netlist_document::ActiveNetlistDocument::GeneratedDiff => None,
    }
    .ok_or_else(|| "The active source has no canonical document identity.".to_owned())?;

    #[cfg(target_arch = "wasm32")]
    if (!dependencies_belong_to_generated_base && !document.include_directives().is_empty())
        || !sealed.is_empty()
    {
        return Err(
            "External dependency closure cannot be sealed without an authorized browser source bundle."
                .to_owned(),
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::collections::HashSet;

        let root = state
            .workspace
            .netlist_source_path
            .as_deref()
            .map(absolute_dependency_path)
            .transpose()?;
        let root_directory = root
            .as_deref()
            .and_then(std::path::Path::parent)
            .unwrap_or_else(|| std::path::Path::new("."));
        let root_directives = if dependencies_belong_to_generated_base {
            external_dependency_locators(document.generated_artifact().source())
        } else {
            document
                .include_directives()
                .iter()
                .map(|include| include.locator().to_owned())
                .collect()
        };
        if !root_directives.is_empty() && root.is_none() {
            return Err(
                "External dependencies require a native source origin before their closure can be retained."
                    .to_owned(),
            );
        }

        let mut dependencies = Vec::with_capacity(sealed.len());
        let mut edges = HashSet::new();
        let mut direct_cursor = 0usize;
        for dependency in sealed {
            let resolved = dependency.resolved_path();
            let locator = dependency_locator(resolved, root_directory, dependency.source())?;
            let owner = dependency.owner_path();
            let source = dependency.source().as_bytes().to_vec();
            if root.as_deref() == Some(owner) {
                let index = root_directives
                    .iter()
                    .enumerate()
                    .skip(direct_cursor)
                    .find_map(|(index, requested)| {
                        (requested == dependency.requested_path()).then_some(index)
                    })
                    .ok_or_else(|| {
                        format!(
                            "Resolved root dependency '{}' has no canonical directive in the retained source.",
                            dependency.requested_path()
                        )
                    })?;
                direct_cursor = index + 1;
                if !edges.insert((None, index)) {
                    continue;
                }
                let record =
                    crate::workbench::code_workspace::DependencyMetadata::unresolved_direct_to(
                        index,
                        dependency.requested_path(),
                        locator,
                    )
                    .and_then(|record| record.resolve_utf8(source))
                    .map_err(|error| error.to_string())?;
                dependencies.push(record);
            } else {
                let parent_source = sealed
                    .iter()
                    .find(|candidate| candidate.resolved_path() == owner)
                    .map(rspice_core::netlist::ResolvedIncludeDependency::source)
                    .ok_or_else(|| {
                        format!(
                            "Resolved dependency parent '{}' is absent from the sealed closure.",
                            owner.display()
                        )
                    })?;
                let index =
                    external_dependency_index_at_line(parent_source, dependency.directive_line())?;
                let parent = dependency_locator(owner, root_directory, parent_source)?;
                let parent_key = parent.logical_identity().to_owned();
                if !edges.insert((Some(parent_key), index)) {
                    continue;
                }
                let record =
                    crate::workbench::code_workspace::DependencyMetadata::unresolved_transitive_to(
                        parent,
                        index,
                        dependency.requested_path(),
                        locator,
                    )
                    .and_then(|record| record.resolve_utf8(source))
                    .map_err(|error| error.to_string())?;
                dependencies.push(record);
            }
        }

        if dependencies_belong_to_generated_base {
            let backing = document.generated_artifact();
            let next = crate::workbench::code_workspace::GeneratedArtifact::try_from_utf8(
                backing.provenance().clone(),
                backing.source_bytes().to_vec(),
                dependencies,
                backing.source_map().to_vec(),
            )
            .map_err(|error| error.to_string())?;
            if !next.dependency_graph_is_sealed() {
                return Err(
                    "The retained generated-base dependency graph is not fully sealed.".to_owned(),
                );
            }
            document
                .update_generated_artifact(backing.content_digest(), next)
                .map_err(|error| error.to_string())?;
        } else {
            document
                .acknowledge_dependencies(document.content_digest(), dependencies)
                .map_err(|error| error.to_string())?;
            if !document.dependency_graph_is_sealed() {
                return Err("The canonical dependency graph is not fully sealed.".to_owned());
            }
        }
    }

    if state.ui.netlist.active_document
        == crate::workbench::netlist_document::ActiveNetlistDocument::OwnedSource
    {
        state.workspace.netlist_document = state.ui.netlist.owned_document.clone();
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn absolute_dependency_path(path: &std::path::Path) -> Result<std::path::PathBuf, String> {
    let joined = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|error| format!("Could not resolve dependency origin: {error}"))?
            .join(path)
    };
    Ok(joined.canonicalize().unwrap_or(joined))
}

#[cfg(not(target_arch = "wasm32"))]
fn dependency_locator(
    path: &std::path::Path,
    root_directory: &std::path::Path,
    source: &str,
) -> Result<crate::workbench::code_workspace::SourceLocator, String> {
    let display = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "dependency.sp".to_owned());
    let identity = path.strip_prefix(root_directory).map_or_else(
        |_| {
            let digest = crate::workbench::code_workspace::content_digest(source);
            format!("external/{}-{display}", &digest.to_string()[..12])
        },
        |relative| relative.to_string_lossy().replace('\\', "/"),
    );
    let native_origin = path.to_string_lossy().into_owned();
    crate::workbench::code_workspace::SourceLocator::try_new(identity.clone(), display)
        .and_then(|locator| locator.with_native_origin(native_origin))
        .map_err(|error| error.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn external_dependency_index_at_line(source: &str, line: usize) -> Result<usize, String> {
    let mut index = 0usize;
    for (zero_line, raw) in source.lines().enumerate() {
        let current_line = zero_line + 1;
        let external = rspice_core::netlist::parse_include_directive(raw).is_some()
            || rspice_core::netlist::parse_lib_directive(raw)
                .is_some_and(|(_, section)| section.is_some());
        if external {
            if current_line == line {
                return Ok(index);
            }
            index += 1;
        }
    }
    Err(format!(
        "Resolved transitive dependency at line {line} has no canonical directive."
    ))
}

#[cfg(not(target_arch = "wasm32"))]
fn external_dependency_locators(source: &str) -> Vec<String> {
    source
        .lines()
        .filter_map(|raw| {
            rspice_core::netlist::parse_include_directive(raw).or_else(|| {
                rspice_core::netlist::parse_lib_directive(raw)
                    .and_then(|(locator, section)| section.map(|_| locator))
            })
        })
        .collect()
}

fn acknowledge_canonical_validation(state: &mut AppState) -> Result<(), String> {
    let owned = state.ui.netlist.active_document
        == crate::workbench::netlist_document::ActiveNetlistDocument::OwnedSource;
    let narrow_owned = owned
        && state
            .workspace
            .netlist_descriptor
            .as_ref()
            .is_some_and(|descriptor| {
                descriptor.strategy != crate::state::OwnedNetlistEditStrategy::OwnedSource
            });
    // Diagnostics for a narrow override are computed against its materialized
    // generated base, so those line coordinates do not identify the small
    // authored document. The exact prepared-run receipt remains authoritative;
    // never attach invented base-deck coordinates to the owned fragment.
    let diagnostics = if narrow_owned {
        Vec::new()
    } else {
        state
            .ui
            .netlist
            .diagnostics
            .iter()
            .map(|diagnostic| {
                let severity = match diagnostic.severity {
                    crate::workbench::netlist_document::DiagnosticSeverity::Info => {
                        crate::workbench::code_workspace::DiagnosticSeverity::Info
                    }
                    crate::workbench::netlist_document::DiagnosticSeverity::Warning => {
                        crate::workbench::code_workspace::DiagnosticSeverity::Warning
                    }
                    crate::workbench::netlist_document::DiagnosticSeverity::Error => {
                        crate::workbench::code_workspace::DiagnosticSeverity::Error
                    }
                };
                crate::workbench::code_workspace::ValidationDiagnostic::try_new(
                    severity,
                    diagnostic.message.clone(),
                    diagnostic.line.unwrap_or(0) + 1,
                    diagnostic.column.unwrap_or(0) + 1,
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?
    };
    let document = match state.ui.netlist.active_document {
        crate::workbench::netlist_document::ActiveNetlistDocument::Generated => {
            state.ui.netlist.generated_document.as_mut()
        }
        crate::workbench::netlist_document::ActiveNetlistDocument::OwnedSource => {
            state.ui.netlist.owned_document.as_mut()
        }
        crate::workbench::netlist_document::ActiveNetlistDocument::GeneratedDiff => None,
    }
    .ok_or_else(|| "The active source has no canonical document identity.".to_owned())?;
    document
        .acknowledge_validation(document.content_digest(), diagnostics)
        .map_err(|error| error.to_string())?;
    if owned {
        state.workspace.netlist_document = state.ui.netlist.owned_document.clone();
    }
    Ok(())
}

fn invalidate_canonical_validation(state: &mut AppState) {
    let document = match state.ui.netlist.active_document {
        crate::workbench::netlist_document::ActiveNetlistDocument::Generated => {
            state.ui.netlist.generated_document.as_mut()
        }
        crate::workbench::netlist_document::ActiveNetlistDocument::OwnedSource => {
            state.ui.netlist.owned_document.as_mut()
        }
        crate::workbench::netlist_document::ActiveNetlistDocument::GeneratedDiff => None,
    };
    if let Some(document) = document {
        let _ = document.invalidate_validation(document.content_digest());
    }
    if state.ui.netlist.active_document
        == crate::workbench::netlist_document::ActiveNetlistDocument::OwnedSource
    {
        state.workspace.netlist_document = state.ui.netlist.owned_document.clone();
    }
}

fn short_digest(digest: crate::product::ContentDigest) -> String {
    digest.to_string().chars().take(12).collect()
}

/// Materialize the exact executable deck represented by an owned source
/// strategy. Narrow override documents remain small, separately owned files;
/// validation and execution deterministically compose them with their frozen
/// generated base.
pub(crate) fn compose_owned_netlist_execution_source(
    state: &AppState,
    authored_source: &str,
) -> Result<String, String> {
    let Some(descriptor) = state.workspace.netlist_descriptor.as_ref() else {
        return Ok(authored_source.to_owned());
    };
    if descriptor.strategy == crate::state::OwnedNetlistEditStrategy::OwnedSource {
        return Ok(authored_source.to_owned());
    }
    let base = state
        .workspace
        .netlist_document
        .as_ref()
        .map(|document| document.generated_artifact().source())
        .ok_or_else(|| "Narrow override has no retained generated base artifact.".to_owned())?;

    let select: fn(&str) -> bool = match descriptor.strategy {
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => {
            |head| matches!(head, ".param" | ".option" | ".options" | ".temp")
        }
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => {
            |head| matches!(head, ".include" | ".inc" | ".lib" | ".veriloga")
        }
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => is_analysis_directive,
        crate::state::OwnedNetlistEditStrategy::OwnedSource => {
            return Ok(authored_source.to_owned());
        }
    };
    validate_narrow_override(authored_source, select)?;
    let base = match descriptor.strategy {
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => base.to_owned(),
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride
        | crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => {
            strip_selected_cards(base, select)
        }
        crate::state::OwnedNetlistEditStrategy::OwnedSource => base.to_owned(),
    };
    insert_before_end(&base, authored_source)
}

fn validate_narrow_override(source: &str, allowed: impl Fn(&str) -> bool) -> Result<(), String> {
    let mut continuation_allowed = false;
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }
        if trimmed.starts_with('+') {
            if continuation_allowed {
                continue;
            }
            return Err(format!(
                "Override line {} is a continuation without an allowed owning card.",
                index + 1
            ));
        }
        let head = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        continuation_allowed = allowed(&head);
        if !continuation_allowed {
            return Err(format!(
                "Directive '{}' at override line {} is outside the selected ownership strategy.",
                head,
                index + 1
            ));
        }
    }
    Ok(())
}

fn strip_selected_cards(source: &str, remove: impl Fn(&str) -> bool) -> String {
    let mut kept = Vec::new();
    let mut removing_continuation = false;
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('+') {
            if !removing_continuation {
                kept.push(line);
            }
            continue;
        }
        let head = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        removing_continuation = remove(&head);
        if !removing_continuation {
            kept.push(line);
        }
    }
    kept.join("\n")
}

fn insert_before_end(base: &str, override_source: &str) -> Result<String, String> {
    let lines = base.lines().collect::<Vec<_>>();
    let end = lines
        .iter()
        .rposition(|line| {
            line.split_whitespace()
                .next()
                .is_some_and(|head| head.eq_ignore_ascii_case(".end"))
        })
        .ok_or_else(|| "Retained generated base has no .end terminator.".to_owned())?;
    let mut result = lines[..end].join("\n");
    if !result.ends_with('\n') {
        result.push('\n');
    }
    result.push_str(override_source.trim_end());
    result.push('\n');
    result.push_str(&lines[end..].join("\n"));
    Ok(result)
}

fn is_analysis_directive(head: &str) -> bool {
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

/// Publish the exact project-owned source bytes currently visible in the Code
/// workspace. Generated artifacts are exported through their separate
/// immutable-artifact workflow and can never be promoted by this operation.
///
/// `save_as` always asks for a destination. Ordinary Save reuses a reopenable
/// imported/native origin when one exists; browser downloads have no durable
/// path authority and therefore always use the user-agent download workflow.
pub(crate) fn save_owned_netlist_source(
    state: &mut AppState,
    simulation_controller: &crate::simulation::SimulationController,
    io: &(impl ExportWorkflowIo + ?Sized),
    save_as: bool,
    commit_message: &str,
) -> bool {
    let commit_message = commit_message.trim();
    if commit_message.is_empty()
        || commit_message.chars().any(char::is_control)
        || commit_message.chars().count() > 240
    {
        state.push_user_message(ConsoleMessage::warning(
            "Enter a one-line source revision message of 1–240 characters.",
        ));
        return false;
    }
    let Some(source) = state.workspace.netlist_source.as_deref() else {
        state.push_user_message(ConsoleMessage::warning(
            "Create an editable source deck before saving source bytes.",
        ));
        return false;
    };
    if source != state.simulation.netlist_content {
        state.push_user_message(ConsoleMessage::error(
            "Source save was blocked because the editor and project-owned deck differ.",
        ));
        return false;
    }
    let visible_digest = crate::workbench::netlist_document::source_content_digest(source);
    let Some(validation) = state.ui.netlist.validation.as_ref().filter(|receipt| {
        receipt.visible_content_digest == visible_digest
            && receipt.project_revision == state.workspace.project.revision().get()
    }) else {
        state.push_user_message(ConsoleMessage::warning(
            "Validate the exact current source and its dependencies before saving it.",
        ));
        return false;
    };
    if let Err(error) = simulation_controller
        .ensure_retained_manual_authorization_current(state, validation.prepared_snapshot_digest)
    {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Revalidate the exact current source before saving it: {error}"
        )));
        return false;
    }

    let default_name = state
        .workspace
        .netlist_source_path
        .as_deref()
        .and_then(std::path::Path::file_name)
        .map(|name| name.to_string_lossy().into_owned())
        .or_else(|| {
            state
                .workspace
                .netlist_descriptor
                .as_ref()
                .map(|descriptor| descriptor.artifact_name.clone())
        })
        .unwrap_or_else(|| "top.cir".to_owned());
    let reopenable_origin = io
        .saved_paths_are_reopenable()
        .then(|| state.workspace.netlist_source_path.clone())
        .flatten();
    let picked = if !save_as && reopenable_origin.is_some() {
        Ok(reopenable_origin)
    } else {
        io.show_save_dialog(SaveDialogConfig {
            title: if save_as {
                "Save SPICE Source As"
            } else {
                "Save SPICE Source"
            },
            default_name: &default_name,
            filter_name: NETLIST_FILTER.0,
            filter_extensions: NETLIST_FILTER.1,
        })
    };

    let Some(mut path) = (match picked {
        Ok(path) => path,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE source save failed: {error}"
            )));
            return false;
        }
    }) else {
        return false;
    };
    if path.extension().is_none() {
        path.set_extension("cir");
    }

    let next_owned_document = if let Some(document) = state.ui.netlist.owned_document.as_ref() {
        let display_name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string());
        let locator = crate::workbench::code_workspace::SourceLocator::try_new(
            path.display().to_string(),
            display_name,
        )
        .and_then(|locator| {
            if io.saved_paths_are_reopenable() {
                locator.with_native_origin(path.display().to_string())
            } else {
                Ok(locator)
            }
        });
        let mut next = document.clone();
        match locator.and_then(|locator| next.acknowledge_save(next.content_digest(), locator)) {
            Ok(_) => Some(next),
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "SPICE source save was blocked before publication: {error}"
                )));
                return false;
            }
        }
    } else {
        None
    };

    let result = io
        .observe_destination(&path)
        .and_then(|destination| io.write_text_file_observed(&destination, source));
    match result {
        Ok(()) => {
            if io.saved_paths_are_reopenable() {
                state.workspace.netlist_source_path = Some(path.clone());
            }
            if let Some(document) = next_owned_document {
                if let Some(descriptor) = state.workspace.netlist_descriptor.as_mut() {
                    let document_revision = document.revision().get();
                    if descriptor
                        .save_history
                        .last()
                        .is_none_or(|record| record.document_revision < document_revision)
                    {
                        descriptor
                            .save_history
                            .push(crate::state::OwnedNetlistSaveRecord {
                                document_revision: document.revision().get(),
                                content_digest: document.content_digest(),
                                message: commit_message.to_owned(),
                            });
                    }
                }
                state.workspace.netlist_document = Some(document.clone());
                state.ui.netlist.owned_document = Some(document);
            }
            state.ui.netlist.externally_saved_content_digest = Some(visible_digest);
            state.push_user_message(ConsoleMessage::info(
                crate::common::export_workflow::export_completion_message(
                    "SPICE source",
                    &path,
                    None,
                    io,
                ),
            ));
            true
        }
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE source save failed: {error}"
            )));
            false
        }
    }
}

pub(crate) fn apply_imported_netlist(
    state: &mut AppState,
    source: String,
    source_path: Option<std::path::PathBuf>,
    display_name: &str,
) -> bool {
    if source.trim().is_empty() {
        state.push_user_message(ConsoleMessage::error(format!(
            "SPICE deck import failed: {display_name} is empty"
        )));
        return false;
    }

    state.clear_design_execution_context();
    let (document, descriptor) =
        match canonical_import_document(state, &source, source_path.as_deref(), display_name) {
            Ok(canonical) => canonical,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "SPICE deck import failed: {error}"
                )));
                return false;
            }
        };
    let source_digest = crate::workbench::netlist_document::source_content_digest(&source);
    state.workspace.netlist_source = Some(source.clone());
    state.workspace.netlist_document = Some(document.clone());
    state.workspace.netlist_descriptor = Some(descriptor);
    state.workspace.netlist_source_path = source_path;
    state.workspace.set_netlist_source_dirty(true);
    state.simulation.netlist_content = source;
    state.ui.netlist.owned_document = Some(document);
    state.ui.netlist.externally_saved_content_digest = Some(source_digest);
    state.ui.netlist.active_document =
        crate::workbench::netlist_document::ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_document_initialized = true;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    crate::workbench::netlist_document::invalidate_source_evidence(&mut state.ui.netlist);
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    state.push_user_message(ConsoleMessage::info(format!(
        "Imported SPICE deck: {display_name}"
    )));
    true
}

fn canonical_import_document(
    state: &AppState,
    source: &str,
    source_path: Option<&std::path::Path>,
    display_name: &str,
) -> Result<
    (
        crate::workbench::code_workspace::NetlistDocument,
        crate::state::OwnedNetlistDescriptor,
    ),
    String,
> {
    use crate::workbench::code_workspace::{
        GeneratedArtifact, GeneratedProvenance, GenerationInput, NetlistDocument,
        NetlistDocumentId, SourceLocator,
    };

    let source_digest = crate::workbench::netlist_document::source_content_digest(source);
    let provenance = GeneratedProvenance::try_new(
        "rspice-import-baseline/v1",
        GenerationInput::new(state.workspace.project.revision(), source_digest),
    )
    .map_err(|error| error.to_string())?;
    let baseline = GeneratedArtifact::try_from_utf8(
        provenance,
        source.as_bytes().to_vec(),
        Vec::new(),
        Vec::new(),
    )
    .map_err(|error| error.to_string())?;
    let mut document = NetlistDocument::from_generated(NetlistDocumentId::new(), baseline)
        .map_err(|error| error.to_string())?;

    let artifact_name = source_path
        .and_then(std::path::Path::file_name)
        .map(|name| name.to_string_lossy().into_owned())
        .or_else(|| {
            let candidate = display_name.trim();
            (!candidate.is_empty()
                && !candidate.chars().any(char::is_control)
                && !candidate.contains('/')
                && !candidate.contains('\\'))
            .then(|| candidate.to_owned())
        })
        .unwrap_or_else(|| "imported.sp".to_owned());
    let logical_identity = source_path.map_or_else(
        || format!("browser-import/{artifact_name}"),
        |path| path.display().to_string(),
    );
    let mut locator = SourceLocator::try_new(logical_identity, artifact_name.clone())
        .map_err(|error| error.to_string())?;
    if let Some(path) = source_path {
        locator = locator
            .with_native_origin(path.display().to_string())
            .map_err(|error| error.to_string())?;
    }
    document
        .import_source(
            document.content_digest(),
            locator,
            source.as_bytes().to_vec(),
        )
        .map_err(|error| error.to_string())?;
    document
        .make_editable(document.content_digest())
        .map_err(|error| error.to_string())?;

    Ok((
        document,
        crate::state::OwnedNetlistDescriptor {
            artifact_name,
            strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
            save_history: Vec::new(),
        },
    ))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn import_netlist(state: &mut AppState) -> bool {
    match show_open_netlist_dialog().and_then(|path| {
        std::fs::read_to_string(&path)
            .map(|contents| (path, contents))
            .map_err(|error| error.to_string())
    }) {
        Ok((path, contents)) => {
            let display_name = path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string());
            apply_imported_netlist(state, contents, Some(path), &display_name)
        }
        Err(error) => {
            if error != "cancelled" {
                state.push_user_message(ConsoleMessage::error(format!(
                    "SPICE deck import failed: {error}"
                )));
            }
            false
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn show_open_netlist_dialog() -> Result<std::path::PathBuf, String> {
    rfd::FileDialog::new()
        .add_filter(NETLIST_FILTER.0, NETLIST_FILTER.1)
        .add_filter("All Files", &["*"])
        .set_title("Import SPICE Deck")
        .pick_file()
        .ok_or_else(|| "cancelled".to_string())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn import_netlist(state: &mut AppState) -> bool {
    match start_browser_netlist_import() {
        Ok(()) => true,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE deck import failed: {error}"
            )));
            false
        }
    }
}

#[cfg(target_arch = "wasm32")]
enum BrowserNetlistImportResult {
    Loaded(crate::common::browser_file_import::PickedTextFile),
    Failed(String),
    Cancelled,
}

#[cfg(target_arch = "wasm32")]
struct BrowserNetlistImportCompletion {
    token: crate::common::browser_file_import::TextImportToken,
    result: BrowserNetlistImportResult,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_NETLIST_IMPORT_RESULT: std::cell::RefCell<Option<BrowserNetlistImportCompletion>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn start_browser_netlist_import() -> Result<(), String> {
    let token = crate::common::browser_file_import::try_begin_text_import(
        crate::common::browser_file_import::BrowserTextImportKind::Netlist,
    )?;

    crate::common::browser_file_import::pick_text_file(
        NETLIST_FILTER.0,
        NETLIST_FILTER.1,
        move |result| {
            if !crate::common::browser_file_import::text_import_is_current(token) {
                return;
            }
            let event = match result {
                Ok(Some(file)) => BrowserNetlistImportResult::Loaded(file),
                Ok(None) => BrowserNetlistImportResult::Cancelled,
                Err(error) => BrowserNetlistImportResult::Failed(error),
            };
            BROWSER_NETLIST_IMPORT_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(BrowserNetlistImportCompletion {
                    token,
                    result: event,
                });
            });
        },
    );
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_netlist_import(state: &mut AppState) -> bool {
    let Some(completion) = BROWSER_NETLIST_IMPORT_RESULT.with(|slot| slot.borrow_mut().take())
    else {
        return false;
    };
    if !crate::common::browser_file_import::finish_text_import(completion.token) {
        return false;
    }
    match completion.result {
        BrowserNetlistImportResult::Loaded(file) => {
            apply_imported_netlist(state, file.contents, None, &file.name)
        }
        BrowserNetlistImportResult::Failed(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE deck import failed: {error}"
            )));
            false
        }
        BrowserNetlistImportResult::Cancelled => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GENERATED_BASE: &str = "* generated\n.option reltol=1e-3\n.param gain=10\n.include \"models/a.lib\"\n.lib \"models/b.lib\" TT\n+ section=fast\nV1 out 0 1\nR1 out 0 1k\n.op\n.measure op vout FIND V(out)\n.save V(out)\n.end\n";

    fn state_with_owned_strategy(
        generated_source: &str,
        authored_source: &str,
        strategy: crate::state::OwnedNetlistEditStrategy,
    ) -> AppState {
        use crate::product::ObjectRevision;
        use crate::workbench::code_workspace::{
            GeneratedArtifact, GeneratedProvenance, GenerationInput, NetlistDocument,
            NetlistDocumentId, content_digest,
        };

        let provenance = GeneratedProvenance::try_new(
            "rspice-netlist-workflow-test",
            GenerationInput::new(ObjectRevision::INITIAL, content_digest("test-inputs")),
        )
        .expect("valid generated provenance");
        let artifact = GeneratedArtifact::try_from_utf8(
            provenance,
            generated_source.as_bytes().to_vec(),
            Vec::new(),
            Vec::new(),
        )
        .expect("valid generated artifact");
        let generated = NetlistDocument::from_generated(NetlistDocumentId::new(), artifact)
            .expect("valid generated document");
        let mut owned = generated
            .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
            .expect("editable copy");
        owned
            .replace_editable_source(owned.content_digest(), authored_source.as_bytes().to_vec())
            .expect("authored source");

        let mut state = AppState::default();
        state.workspace.netlist_source = Some(authored_source.to_owned());
        state.workspace.netlist_document = Some(owned);
        state.workspace.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
            artifact_name: "owned.cir".to_owned(),
            strategy,
            save_history: Vec::new(),
        });
        state
    }

    #[test]
    fn owned_source_strategy_executes_exact_authored_bytes_without_generated_composition() {
        let authored = "* independently owned\r\nV9 out 0 9\r\n.tran 1n 10n\r\n.end\r\n";
        let state = state_with_owned_strategy(
            GENERATED_BASE,
            authored,
            crate::state::OwnedNetlistEditStrategy::OwnedSource,
        );

        let composed = compose_owned_netlist_execution_source(&state, authored)
            .expect("owned source is executable");

        assert_eq!(composed.as_bytes(), authored.as_bytes());
    }

    #[test]
    fn parameter_option_override_retains_base_and_appends_override_before_end() {
        let authored =
            "* project corner\n.param gain=22\n+ trim=0.5\n.options method=gear\n.temp 85\n";
        let state = state_with_owned_strategy(
            GENERATED_BASE,
            authored,
            crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
        );

        let composed = compose_owned_netlist_execution_source(&state, authored)
            .expect("parameter override is executable");

        assert_eq!(
            composed,
            "* generated\n.option reltol=1e-3\n.param gain=10\n.include \"models/a.lib\"\n.lib \"models/b.lib\" TT\n+ section=fast\nV1 out 0 1\nR1 out 0 1k\n.op\n.measure op vout FIND V(out)\n.save V(out)\n* project corner\n.param gain=22\n+ trim=0.5\n.options method=gear\n.temp 85\n.end"
        );
    }

    #[test]
    fn include_order_override_replaces_all_base_include_cards_and_continuations() {
        let authored =
            ".lib \"models/b.lib\" SS\n.include \"models/a.lib\"\n.veriloga \"models/device.va\"\n";
        let state = state_with_owned_strategy(
            GENERATED_BASE,
            authored,
            crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride,
        );

        let composed = compose_owned_netlist_execution_source(&state, authored)
            .expect("include-order override is executable");

        assert_eq!(
            composed,
            "* generated\n.option reltol=1e-3\n.param gain=10\nV1 out 0 1\nR1 out 0 1k\n.op\n.measure op vout FIND V(out)\n.save V(out)\n.lib \"models/b.lib\" SS\n.include \"models/a.lib\"\n.veriloga \"models/device.va\"\n.end"
        );
        assert!(!composed.contains("section=fast"));
    }

    #[test]
    fn analysis_only_deck_replaces_base_analysis_measurement_and_output_cards() {
        let authored = ".tran 1n 10n\n.measure tran vmax MAX V(out)\n.probe V(out) I(V1)\n";
        let state = state_with_owned_strategy(
            GENERATED_BASE,
            authored,
            crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck,
        );

        let composed = compose_owned_netlist_execution_source(&state, authored)
            .expect("analysis-only deck is executable");

        assert_eq!(
            composed,
            "* generated\n.option reltol=1e-3\n.param gain=10\n.include \"models/a.lib\"\n.lib \"models/b.lib\" TT\n+ section=fast\nV1 out 0 1\nR1 out 0 1k\n.tran 1n 10n\n.measure tran vmax MAX V(out)\n.probe V(out) I(V1)\n.end"
        );
        assert!(!composed.contains(".op\n"));
        assert!(!composed.contains(".save V(out)"));
    }

    #[test]
    fn narrow_override_rejects_device_cards_and_cross_strategy_directives() {
        let device = "Roverride out 0 2k\n";
        let state = state_with_owned_strategy(
            GENERATED_BASE,
            device,
            crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
        );
        let error = compose_owned_netlist_execution_source(&state, device)
            .expect_err("device card must fail closed");
        assert!(error.contains("'roverride'"));
        assert!(error.contains("line 1"));

        let wrong_strategy = ".include \"models/other.lib\"\n";
        let state = state_with_owned_strategy(
            GENERATED_BASE,
            wrong_strategy,
            crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck,
        );
        let error = compose_owned_netlist_execution_source(&state, wrong_strategy)
            .expect_err("cross-strategy directive must fail closed");
        assert!(error.contains("'.include'"));
        assert!(error.contains("line 1"));
    }

    #[test]
    fn narrow_override_rejects_orphan_continuation() {
        let authored = "+ sweep=fast\n.param gain=22\n";
        let state = state_with_owned_strategy(
            GENERATED_BASE,
            authored,
            crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
        );

        let error = compose_owned_netlist_execution_source(&state, authored)
            .expect_err("orphan continuation must fail closed");

        assert_eq!(
            error,
            "Override line 1 is a continuation without an allowed owning card."
        );
    }

    #[test]
    fn narrow_override_requires_retained_generated_base() {
        let authored = ".tran 1n 10n\n";
        let mut state = AppState::default();
        state.workspace.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
            artifact_name: "analysis.cir".to_owned(),
            strategy: crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck,
            save_history: Vec::new(),
        });

        let error = compose_owned_netlist_execution_source(&state, authored)
            .expect_err("missing generated base must fail closed");

        assert_eq!(
            error,
            "Narrow override has no retained generated base artifact."
        );
    }

    #[test]
    fn narrow_override_rejects_generated_base_without_end_terminator() {
        let authored = ".param gain=22\n";
        let state = state_with_owned_strategy(
            "* malformed generated base\nR1 out 0 1k\n.op\n",
            authored,
            crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
        );

        let error = compose_owned_netlist_execution_source(&state, authored)
            .expect_err("missing end terminator must fail closed");

        assert_eq!(error, "Retained generated base has no .end terminator.");
    }

    #[test]
    fn imported_netlist_becomes_dirty_manual_source_in_netlist_workspace() {
        let mut state = AppState::default();
        state.simulation.start_run();
        assert!(state.simulation.has_results());

        let imported = apply_imported_netlist(
            &mut state,
            "deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned(),
            Some(std::path::PathBuf::from("bias.cir")),
            "bias.cir",
        );

        assert!(imported);
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Netlist
        );
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n")
        );
        assert_eq!(
            state.workspace.netlist_source_path.as_deref(),
            Some(std::path::Path::new("bias.cir"))
        );
        assert!(state.workspace.netlist_source_dirty);
        assert!(state.workspace.any_dirty());
        assert!(state.workspace.netlist_document.is_some());
        assert_eq!(
            state
                .workspace
                .netlist_descriptor
                .as_ref()
                .map(|descriptor| descriptor.artifact_name.as_str()),
            Some("bias.cir")
        );
        state
            .workspace
            .validate_simulation_configuration()
            .expect("imported canonical source must satisfy project persistence invariants");
        assert!(!state.simulation.has_results());
        assert!(state.recent_files.is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn text_first_import_validates_and_retains_explicit_run_authorization_without_schematic() {
        let mut app = crate::common::RSpiceApp::test_instance();
        app.state.schematic.components.clear();
        let source = "standalone\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";

        assert!(apply_imported_netlist(
            &mut app.state,
            source.to_owned(),
            None,
            "standalone.cir",
        ));
        assert!(validate_visible_netlist_source(&mut app));
        assert!(app.state.ui.netlist.validation.is_some());
        assert_eq!(
            app.state.ui.netlist.externally_saved_content_digest,
            Some(crate::workbench::netlist_document::source_content_digest(
                source
            ))
        );
        assert_eq!(app.manual_deck_run_block_reason(), None);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn validation_publishes_exact_direct_and_transitive_dependency_closure() {
        let dir = std::env::temp_dir().join(format!(
            "rspice-netlist-dependency-closure-{}",
            uuid::Uuid::new_v4()
        ));
        let models = dir.join("models");
        std::fs::create_dir_all(&models).expect("create dependency fixture");
        let root = dir.join("root.cir");
        let first_source = ".include \"nested.inc\"\nRmodel out 0 2k\n";
        let nested_source = "Cmodel out 0 1p\n";
        std::fs::write(models.join("first.inc"), first_source).expect("write direct include");
        std::fs::write(models.join("nested.inc"), nested_source).expect("write transitive include");
        let source = "standalone\n.include \"models/first.inc\"\nV1 out 0 1\n.op\n.end\n";
        std::fs::write(&root, source).expect("write root deck");

        let mut app = crate::common::RSpiceApp::test_instance();
        assert!(apply_imported_netlist(
            &mut app.state,
            source.to_owned(),
            Some(root.clone()),
            "root.cir",
        ));
        assert!(validate_visible_netlist_source(&mut app));

        let document = app
            .state
            .workspace
            .netlist_document
            .as_ref()
            .expect("validated canonical document");
        assert!(document.dependency_graph_is_sealed());
        assert_eq!(document.dependencies().len(), 2);
        let direct = document
            .dependencies()
            .iter()
            .find(|dependency| dependency.direct_include_index() == Some(0))
            .expect("direct dependency");
        assert_eq!(direct.requested_locator(), "models/first.inc");
        assert_eq!(direct.source(), Some(first_source));
        assert_eq!(direct.locator().logical_identity(), "models/first.inc");
        assert!(
            !direct
                .locator()
                .logical_identity()
                .contains(&dir.to_string_lossy().to_string()),
            "portable logical identities must not expose the native project root"
        );
        let transitive = document
            .dependencies()
            .iter()
            .find(|dependency| dependency.parent().is_some())
            .expect("transitive dependency");
        assert_eq!(transitive.requested_locator(), "nested.inc");
        assert_eq!(transitive.source(), Some(nested_source));
        assert_eq!(transitive.parent_include_index(), Some(0));

        std::fs::remove_dir_all(dir).expect("remove dependency fixture");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn validation_distinguishes_inline_lib_sections_from_external_lib_paths() {
        let dir = std::env::temp_dir().join(format!(
            "rspice-netlist-library-closure-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&dir).expect("create library fixture");
        let root = dir.join("root.cir");
        let library_source = ".lib TT\n.model DMOD D IS=1e-14\n.endl TT\n.lib SS\n.model DMOD D IS=2e-14\n.endl SS\n";
        std::fs::write(dir.join("corners.lib"), library_source).expect("write library");
        let source = "standalone\n.lib LOCAL\n.model LOCALMOD D IS=3e-14\n.endl LOCAL\n.lib \"corners.lib\" TT\nD1 out 0 DMOD\n.op\n.end\n";
        std::fs::write(&root, source).expect("write root deck");

        let mut app = crate::common::RSpiceApp::test_instance();
        assert!(apply_imported_netlist(
            &mut app.state,
            source.to_owned(),
            Some(root),
            "root.cir",
        ));
        assert!(validate_visible_netlist_source(&mut app));

        let document = app.state.workspace.netlist_document.as_ref().unwrap();
        assert_eq!(document.include_directives().len(), 1);
        assert_eq!(document.dependencies().len(), 1);
        assert_eq!(
            document.dependencies()[0].requested_locator(),
            "corners.lib"
        );
        assert_eq!(document.dependencies()[0].source(), Some(library_source));

        std::fs::remove_dir_all(dir).expect("remove library fixture");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn narrow_strategy_dependencies_attach_to_the_source_that_owns_the_directives() {
        let dir = std::env::temp_dir().join(format!(
            "rspice-netlist-narrow-dependencies-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&dir).expect("create narrow dependency fixture");
        let root = dir.join("owned.inc");
        std::fs::write(dir.join("base.inc"), "Rbase out 0 1k\n").expect("write base include");
        std::fs::write(dir.join("override.inc"), "Roverride out 0 2k\n")
            .expect("write override include");
        let generated = "* generated\n.include \"base.inc\"\nV1 out 0 1\n.op\n.end\n";

        let parameter_source = ".param gain=22\n";
        std::fs::write(&root, parameter_source).expect("write parameter source");
        let mut app = crate::common::RSpiceApp::test_instance();
        app.state = state_with_owned_strategy(
            generated,
            parameter_source,
            crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
        );
        app.state.workspace.netlist_source_path = Some(root.clone());
        app.state.ui.netlist.owned_document = app.state.workspace.netlist_document.clone();
        app.state.ui.netlist.active_document =
            crate::workbench::netlist_document::ActiveNetlistDocument::OwnedSource;
        app.state.ui.netlist.active_document_initialized = true;
        app.state.simulation.netlist_content = parameter_source.to_owned();

        assert!(validate_visible_netlist_source(&mut app));
        let parameter_document = app.state.workspace.netlist_document.as_ref().unwrap();
        assert!(parameter_document.dependencies().is_empty());
        assert!(
            parameter_document
                .generated_artifact()
                .dependency_graph_is_sealed()
        );
        assert_eq!(
            parameter_document.generated_artifact().dependencies()[0].source(),
            Some("Rbase out 0 1k\n")
        );

        let include_source = ".include \"override.inc\"\n";
        std::fs::write(&root, include_source).expect("write include-order source");
        let mut app = crate::common::RSpiceApp::test_instance();
        app.state = state_with_owned_strategy(
            generated,
            include_source,
            crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride,
        );
        app.state.workspace.netlist_source_path = Some(root);
        app.state.ui.netlist.owned_document = app.state.workspace.netlist_document.clone();
        app.state.ui.netlist.active_document =
            crate::workbench::netlist_document::ActiveNetlistDocument::OwnedSource;
        app.state.ui.netlist.active_document_initialized = true;
        app.state.simulation.netlist_content = include_source.to_owned();

        assert!(validate_visible_netlist_source(&mut app));
        let include_document = app.state.workspace.netlist_document.as_ref().unwrap();
        assert!(include_document.dependency_graph_is_sealed());
        assert_eq!(include_document.dependencies().len(), 1);
        assert_eq!(
            include_document.dependencies()[0].requested_locator(),
            "override.inc"
        );
        assert_eq!(
            include_document.dependencies()[0].source(),
            Some("Roverride out 0 2k\n")
        );

        std::fs::remove_dir_all(dir).expect("remove narrow dependency fixture");
    }

    #[test]
    fn empty_netlist_import_is_rejected_without_clearing_existing_state() {
        let mut state = AppState::default();
        state.workspace.netlist_source = Some("existing\n.op\n.end\n".to_owned());
        state.workspace.netlist_source_path = Some(std::path::PathBuf::from("existing.cir"));
        state.simulation.netlist_content = "existing\n.op\n.end\n".to_owned();

        let imported = apply_imported_netlist(&mut state, " \n\t".to_owned(), None, "empty.cir");

        assert!(!imported);
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("existing\n.op\n.end\n")
        );
        assert_eq!(
            state.workspace.netlist_source_path.as_deref(),
            Some(std::path::Path::new("existing.cir"))
        );
        assert_eq!(state.simulation.netlist_content, "existing\n.op\n.end\n");
    }
}
