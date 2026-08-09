//! Netlist workspace workflows.
//!
//! Validating the visible deck and composing the source a run executes.
//! Validation is pinned to an exact content digest and project revision, so
//! an edit after validation invalidates it rather than being carried along.

use crate::diagnostics::ConsoleMessage;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

pub const NETLIST_FILTER: (&str, &[&str]) = ("SPICE Deck", &["cir", "sp", "spice", "net", "ckt"]);

/// Validate and retain the exact visible manual-deck snapshot, including its
/// sealed dependency closure and execution-target contract. A later Run must
/// match this one-shot authorized snapshot byte for byte.
pub(crate) fn validate_visible_netlist_source(app: &mut RSpiceApp) -> bool {
    use crate::workbench::documents::netlist_document::{
        ActiveNetlistDocument, NetlistValidationReceipt, source_content_digest,
    };

    if app.state.ui.netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        app.state.push_user_message(ConsoleMessage::warning(
            "Revision comparisons are read-only review documents and cannot be validated for execution.",
        ));
        return false;
    }
    if app.state.ui.netlist.active_dependency_identity.is_some() {
        let message = app
            .state
            .ui
            .messages()
            .text(crate::workbench::MessageId::NetlistRootValidationRequired);
        app.state
            .push_user_message(ConsoleMessage::warning(message));
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
                app.invalidate_simulation_preflight();
                app.state.ui.netlist.validation = None;
                app.state.ui.netlist.validation_error = Some(error.clone());
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "Netlist validation failed: {error}"
                )));
                return false;
            }
            if let Err(error) = acknowledge_canonical_validation(&mut app.state) {
                app.invalidate_simulation_preflight();
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
    let authority_paths = dependency_authority_paths(state);
    let dependencies_belong_to_generated_base = state.ui.netlist.active_document
        == crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource
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
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated => {
            state.ui.netlist.generated_document.as_mut()
        }
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource => {
            state.ui.netlist.owned_document.as_mut()
        }
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff => None,
    }
    .ok_or_else(|| "The active source has no canonical document identity.".to_owned())?;

    use std::collections::HashSet;

    let source_origin = match state.ui.netlist.active_document {
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated => {
            state.schematic.current_file.as_deref()
        }
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource => {
            state.workspace.netlist_source_path.as_deref()
        }
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff => None,
    };
    let root = source_origin.map(dependency_root_path).transpose()?;
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
                "External dependencies require an imported source origin before their closure can be retained."
                    .to_owned(),
            );
    }

    let mut dependencies = Vec::with_capacity(sealed.len());
    let mut edges = HashSet::new();
    let mut direct_cursor = 0usize;
    for dependency in sealed {
        let resolved = dependency.resolved_path();
        let locator = dependency_locator(resolved, root_directory, dependency.source())?;
        let authority = dependency_source_authority(&authority_paths, resolved);
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
            let record = crate::state::DependencyMetadata::unresolved_direct_to(
                index,
                dependency.requested_path(),
                locator,
            )
            .map(|record| record.with_authority(authority))
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
            let record = crate::state::DependencyMetadata::unresolved_transitive_to(
                parent,
                index,
                dependency.requested_path(),
                locator,
            )
            .map(|record| record.with_authority(authority))
            .and_then(|record| record.resolve_utf8(source))
            .map_err(|error| error.to_string())?;
            dependencies.push(record);
        }
    }

    if dependencies_belong_to_generated_base {
        let backing = document.generated_artifact();
        let next = crate::state::GeneratedArtifact::try_from_utf8(
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

    if state.ui.netlist.active_document
        == crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource
    {
        state.workspace.netlist_document = state.ui.netlist.owned_document.clone();
    }
    Ok(())
}

fn dependency_root_path(path: &std::path::Path) -> Result<std::path::PathBuf, String> {
    #[cfg(target_arch = "wasm32")]
    {
        if crate::state::model_library::is_portable_absolute_path(path) {
            return Ok(path.to_path_buf());
        }
        Err(format!(
            "Browser dependency origin must retain an absolute portable identity: {}",
            path.display()
        ))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let joined = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .map_err(|error| format!("Could not resolve dependency origin: {error}"))?
                .join(path)
        };
        Ok(joined.canonicalize().unwrap_or(joined))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn dependency_locator(
    path: &std::path::Path,
    root_directory: &std::path::Path,
    source: &str,
) -> Result<crate::state::SourceLocator, String> {
    let display = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "dependency.sp".to_owned());
    let identity = path.strip_prefix(root_directory).map_or_else(
        |_| {
            let digest = crate::state::content_digest(source);
            format!("external/{}-{display}", &digest.to_string()[..12])
        },
        |relative| relative.to_string_lossy().replace('\\', "/"),
    );
    let native_origin = path.to_string_lossy().into_owned();
    crate::state::SourceLocator::try_new(identity.clone(), display)
        .and_then(|locator| locator.with_native_origin(native_origin))
        .map_err(|error| error.to_string())
}

#[cfg(target_arch = "wasm32")]
fn dependency_locator(
    path: &std::path::Path,
    _root_directory: &std::path::Path,
    _source: &str,
) -> Result<crate::state::SourceLocator, String> {
    let identity = path.to_string_lossy().replace('\\', "/");
    let display = identity
        .rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or("dependency.sp")
        .to_owned();
    crate::state::SourceLocator::try_new(identity, display).map_err(|error| error.to_string())
}

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

fn dependency_authority_paths(
    state: &AppState,
) -> (
    std::collections::HashSet<std::path::PathBuf>,
    std::collections::HashSet<std::path::PathBuf>,
) {
    let technology = state
        .workspace
        .project
        .technology_binding()
        .into_iter()
        .flat_map(|binding| {
            binding
                .source_closure()
                .iter()
                .map(|source| source.path.clone())
        })
        .collect();
    let vendor = state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| {
            library.root_path.iter().cloned().chain(
                library
                    .source_closure
                    .iter()
                    .map(|source| source.path.clone()),
            )
        })
        .collect();
    (technology, vendor)
}

fn dependency_source_authority(
    authority_paths: &(
        std::collections::HashSet<std::path::PathBuf>,
        std::collections::HashSet<std::path::PathBuf>,
    ),
    path: &std::path::Path,
) -> crate::state::DependencySourceAuthority {
    if authority_paths.0.contains(path) {
        crate::state::DependencySourceAuthority::TechnologyPackage
    } else if authority_paths.1.contains(path) {
        crate::state::DependencySourceAuthority::Vendor
    } else {
        crate::state::DependencySourceAuthority::External
    }
}

fn acknowledge_canonical_validation(state: &mut AppState) -> Result<(), String> {
    let owned = state.ui.netlist.active_document
        == crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource;
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
            .filter(|diagnostic| diagnostic.is_current())
            .map(|diagnostic| {
                let severity = match diagnostic.severity {
                    crate::workbench::documents::netlist_document::DiagnosticSeverity::Hint
                    | crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => {
                        crate::state::DiagnosticSeverity::Info
                    }
                    crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning => {
                        crate::state::DiagnosticSeverity::Warning
                    }
                    crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => {
                        crate::state::DiagnosticSeverity::Error
                    }
                };
                crate::state::ValidationDiagnostic::try_new(
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
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated => {
            state.ui.netlist.generated_document.as_mut()
        }
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource => {
            state.ui.netlist.owned_document.as_mut()
        }
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff => None,
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
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated => {
            state.ui.netlist.generated_document.as_mut()
        }
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource => {
            state.ui.netlist.owned_document.as_mut()
        }
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff => None,
    };
    if let Some(document) = document {
        let _ = document.invalidate_validation(document.content_digest());
    }
    if state.ui.netlist.active_document
        == crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource
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
    if state.ui.netlist.active_dependency_identity.is_some() {
        state.push_user_message(ConsoleMessage::warning(
            "Include documents are persisted with the project. Return to the root deck before publishing standalone root-source bytes.",
        ));
        return false;
    }
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
    let Some(source) = state.workspace.netlist_source.clone() else {
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
    let visible_digest =
        crate::workbench::documents::netlist_document::source_content_digest(&source);
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
    let ordinary_save_to_origin = !save_as && reopenable_origin.is_some();
    let expected_external_sha256 = state
        .workspace
        .netlist_descriptor
        .as_ref()
        .and_then(|descriptor| descriptor.external_file_sha256);
    let encoding = state
        .workspace
        .netlist_descriptor
        .as_ref()
        .map_or(crate::state::NetlistTextEncoding::Utf8, |descriptor| {
            descriptor.source_encoding
        });
    let encoded_source = match encoding.encode(&source) {
        Ok(bytes) => bytes,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE source save failed: {error}"
            )));
            return false;
        }
    };
    let picked = if ordinary_save_to_origin {
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

    #[cfg(not(target_arch = "wasm32"))]
    if ordinary_save_to_origin && let Some(expected) = expected_external_sha256 {
        match stage_external_netlist_change(state, &path, expected, &source) {
            Ok(true) => {
                state.push_user_message(ConsoleMessage::warning(
                    "The source changed outside RSpice. Review the exact local, external, and merge candidates before saving.",
                ));
                return false;
            }
            Ok(false) => {}
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "SPICE source save failed before publication: {error}"
                )));
                return false;
            }
        }
    }

    let next_owned_document = if let Some(document) = state.ui.netlist.owned_document.as_ref() {
        let display_name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string());
        let locator =
            crate::state::SourceLocator::try_new(path.display().to_string(), display_name)
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
    let next_descriptor = match (
        state.workspace.netlist_descriptor.as_ref(),
        next_owned_document.as_ref(),
    ) {
        (Some(descriptor), Some(document)) => {
            let mut descriptor = descriptor.clone();
            descriptor.source_line_ending = crate::state::NetlistLineEnding::detect(&source);
            descriptor.external_file_sha256 = io
                .saved_paths_are_reopenable()
                .then(|| sha256(&encoded_source));
            let document_revision = document.revision().get();
            if descriptor
                .save_history
                .last()
                .is_none_or(|record| record.document_revision < document_revision)
            {
                descriptor
                    .save_history
                    .push(crate::state::OwnedNetlistSaveRecord {
                        document_revision,
                        content_digest: document.content_digest(),
                        message: commit_message.to_owned(),
                    });
            }
            if let Err(error) = descriptor.retain_revision(document, commit_message) {
                state.push_user_message(ConsoleMessage::error(format!(
                    "SPICE source save was blocked before publication: {error}"
                )));
                return false;
            }
            Some(descriptor)
        }
        (None, None) => None,
        _ => {
            state.push_user_message(ConsoleMessage::error(
                "SPICE source save was blocked because canonical document metadata is incomplete.",
            ));
            return false;
        }
    };

    let observed = if ordinary_save_to_origin {
        expected_external_sha256.map_or_else(
            || io.observe_destination(&path),
            |expected| io.observe_destination_at_sha256(&path, expected),
        )
    } else {
        io.observe_destination(&path)
    };
    let result = observed.and_then(|destination| {
        if encoding == crate::state::NetlistTextEncoding::Utf8 {
            io.write_text_file_observed(&destination, &source)
        } else {
            io.write_bytes_file_observed(&destination, &encoded_source, "text/plain")
        }
    });
    match result {
        Ok(()) => {
            if io.saved_paths_are_reopenable() {
                state.workspace.netlist_source_path = Some(path.clone());
            }
            if let Some(descriptor) = next_descriptor {
                state.workspace.netlist_descriptor = Some(descriptor);
            }
            if let Some(document) = next_owned_document {
                state.workspace.netlist_document = Some(document.clone());
                state.ui.netlist.owned_document = Some(document);
            }
            state.ui.netlist.externally_saved_content_digest = Some(visible_digest);
            state.push_user_message(ConsoleMessage::info(
                crate::workbench::workflows::export_workflow::export_completion_message(
                    "SPICE source",
                    &path,
                    None,
                    io,
                ),
            ));
            true
        }
        Err(error) => {
            #[cfg(not(target_arch = "wasm32"))]
            if ordinary_save_to_origin
                && let Some(expected) = expected_external_sha256
                && matches!(
                    stage_external_netlist_change(state, &path, expected, &source),
                    Ok(true)
                )
            {
                state.push_user_message(ConsoleMessage::warning(
                    "The source changed during publication. No bytes were overwritten; review the newly observed external revision.",
                ));
                return false;
            }
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE source save failed: {error}"
            )));
            false
        }
    }
}

#[cfg(test)]
pub(crate) fn apply_imported_netlist(
    state: &mut AppState,
    source: String,
    source_path: Option<std::path::PathBuf>,
    display_name: &str,
) -> bool {
    let metadata = NetlistImportMetadata {
        encoding: crate::state::NetlistTextEncoding::Utf8,
        line_ending: crate::state::NetlistLineEnding::detect(&source),
        dialect: crate::state::NetlistSourceDialect::RSpice,
        compatibility_reviewed: false,
        raw_sha256: sha256(source.as_bytes()),
    };
    apply_imported_netlist_transaction(
        state,
        source,
        source_path,
        display_name,
        false,
        metadata,
        Vec::new(),
    )
}

#[derive(Debug, Clone, Copy)]
struct NetlistImportMetadata {
    encoding: crate::state::NetlistTextEncoding,
    line_ending: crate::state::NetlistLineEnding,
    dialect: crate::state::NetlistSourceDialect,
    compatibility_reviewed: bool,
    raw_sha256: [u8; 32],
}

fn apply_imported_netlist_transaction(
    state: &mut AppState,
    source: String,
    source_path: Option<std::path::PathBuf>,
    display_name: &str,
    initializing_netlist_project: bool,
    metadata: NetlistImportMetadata,
    dependencies: Vec<crate::state::DependencyMetadata>,
) -> bool {
    if source.trim().is_empty() {
        state.push_user_message(ConsoleMessage::error(format!(
            "SPICE deck import failed: {display_name} is empty"
        )));
        return false;
    }

    if !state.project_lifecycle.project_open {
        state.push_user_message(ConsoleMessage::error(
            "SPICE deck import requires an open project",
        ));
        return false;
    }
    if state.workbench.safe_mode.project_read_only() && !initializing_netlist_project {
        state.push_user_message(ConsoleMessage::error(
            "SPICE deck import is unavailable because the project is open read-only",
        ));
        return false;
    }
    if state.simulation.active_execution.is_some() || state.simulation.is_running {
        state.push_user_message(ConsoleMessage::error(
            "SPICE deck import is blocked while a simulation execution owns the project",
        ));
        return false;
    }

    let (document, descriptor) = match canonical_import_document(
        state,
        &source,
        source_path.as_deref(),
        display_name,
        metadata,
        dependencies,
    ) {
        Ok(canonical) => canonical,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE deck import failed: {error}"
            )));
            return false;
        }
    };
    // Importing a new source deck changes future execution authority but does
    // not delete immutable datasets produced by earlier sources. Every run
    // carries its own provenance, so retained history remains truthful and
    // reviewable after this project-owned document changes.
    state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
    state.ui.netlist = Default::default();
    let source_digest =
        crate::workbench::documents::netlist_document::source_content_digest(&source);
    state.workspace.netlist_source = Some(source.clone());
    state.workspace.netlist_document = Some(document.clone());
    state.workspace.netlist_descriptor = Some(descriptor);
    state.workspace.netlist_source_path = source_path;
    state.workspace.set_netlist_source_dirty(true);
    state.simulation.netlist_content = source;
    state.ui.netlist.owned_document = Some(document);
    state.ui.netlist.externally_saved_content_digest = Some(source_digest);
    state.ui.netlist.active_document =
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_document_initialized = true;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    crate::workbench::documents::netlist_document::invalidate_source_evidence(
        &mut state.ui.netlist,
    );
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    state.push_user_message(ConsoleMessage::info(format!(
        "Imported SPICE deck: {display_name}"
    )));
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NetlistImportMode {
    OpenProject,
    ImportIntoProject,
}

impl NetlistImportMode {
    const fn dialog_title(self) -> &'static str {
        match self {
            Self::OpenProject => "Open Netlist Project",
            Self::ImportIntoProject => "Import SPICE Deck",
        }
    }
}

fn netlist_import_start_block_reason(
    state: &AppState,
    mode: NetlistImportMode,
) -> Option<&'static str> {
    if state.simulation.active_execution.is_some() || state.simulation.is_running {
        return Some("a simulation execution still owns the project");
    }
    if mode == NetlistImportMode::ImportIntoProject && !state.project_lifecycle.project_open {
        return Some("no project is open");
    }
    if mode == NetlistImportMode::ImportIntoProject && state.workbench.safe_mode.project_read_only()
    {
        return Some("the project is open read-only");
    }
    None
}

fn apply_opened_netlist_project(
    state: &mut AppState,
    source: String,
    source_path: Option<std::path::PathBuf>,
    display_name: &str,
    metadata: NetlistImportMetadata,
    dependencies: Vec<crate::state::DependencyMetadata>,
) -> bool {
    if source.trim().is_empty() {
        state.push_user_message(ConsoleMessage::error(format!(
            "Netlist project open failed: {display_name} is empty"
        )));
        return false;
    }
    if state.simulation.active_execution.is_some() || state.simulation.is_running {
        state.push_user_message(ConsoleMessage::error(
            "Netlist project open is blocked while a simulation execution owns the project",
        ));
        return false;
    }

    // Construct and validate the replacement off to the side. The currently
    // open project, its dirty documents and all retained evidence remain
    // untouched unless the complete netlist-first project is ready to commit.
    let mut candidate = state.clone();
    crate::workbench::workflows::project_workflow::create_new_project(&mut candidate);
    let proposed_name = std::path::Path::new(display_name)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.trim().is_empty())
        .unwrap_or("Netlist Project");
    if candidate
        .workspace
        .project
        .rename(proposed_name.to_owned())
        .is_err()
    {
        let _ = candidate
            .workspace
            .project
            .rename("Netlist Project".to_owned());
    }
    if !apply_imported_netlist_transaction(
        &mut candidate,
        source,
        source_path,
        display_name,
        true,
        metadata,
        dependencies,
    ) {
        state.push_user_message(ConsoleMessage::error(
            "Netlist project open failed; the current project was left unchanged",
        ));
        return false;
    }
    candidate.push_user_message(ConsoleMessage::info(format!(
        "Opened netlist-first project: {display_name}"
    )));
    *state = candidate;
    true
}

fn apply_netlist_import_result(
    state: &mut AppState,
    mode: NetlistImportMode,
    source: String,
    source_path: Option<std::path::PathBuf>,
    display_name: &str,
    metadata: NetlistImportMetadata,
    dependencies: Vec<crate::state::DependencyMetadata>,
) -> bool {
    match mode {
        NetlistImportMode::OpenProject => apply_opened_netlist_project(
            state,
            source,
            source_path,
            display_name,
            metadata,
            dependencies,
        ),
        NetlistImportMode::ImportIntoProject => apply_imported_netlist_transaction(
            state,
            source,
            source_path,
            display_name,
            false,
            metadata,
            dependencies,
        ),
    }
}

fn canonical_import_document(
    state: &AppState,
    source: &str,
    source_path: Option<&std::path::Path>,
    display_name: &str,
    metadata: NetlistImportMetadata,
    dependencies: Vec<crate::state::DependencyMetadata>,
) -> Result<
    (
        crate::state::NetlistDocument,
        crate::state::OwnedNetlistDescriptor,
    ),
    String,
> {
    use crate::state::{
        GeneratedArtifact, GeneratedProvenance, GenerationInput, NetlistDocument,
        NetlistDocumentId, SourceLocator,
    };

    let source_digest =
        crate::workbench::documents::netlist_document::source_content_digest(source);
    let provenance = GeneratedProvenance::try_new(
        "rspice-import-baseline/v1",
        GenerationInput::new(state.workspace.project.revision(), source_digest),
    )
    .map_err(|error| error.to_string())?;
    let baseline = GeneratedArtifact::try_from_utf8(
        provenance,
        source.as_bytes().to_vec(),
        dependencies.clone(),
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
    if !dependencies.is_empty() {
        document
            .acknowledge_dependencies(document.content_digest(), dependencies)
            .map_err(|error| error.to_string())?;
    }

    let mut descriptor = crate::state::OwnedNetlistDescriptor {
        artifact_name,
        strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
        source_encoding: metadata.encoding,
        source_line_ending: metadata.line_ending,
        imported_dialect: Some(metadata.dialect),
        compatibility_reviewed: metadata.compatibility_reviewed,
        execution_profile: metadata.dialect.execution_profile(),
        external_file_sha256: source_path.map(|_| metadata.raw_sha256),
        save_history: Vec::new(),
        revision_history: Vec::new(),
        owned_includes: Vec::new(),
    };
    descriptor.retain_revision(&document, "Imported source baseline")?;
    Ok((document, descriptor))
}

fn sha256(bytes: &[u8]) -> [u8; 32] {
    use sha2::Digest as _;

    let digest = sha2::Sha256::digest(bytes);
    let mut value = [0_u8; 32];
    value.copy_from_slice(&digest);
    value
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
struct MergeLineEdit {
    start: usize,
    end: usize,
    replacement: String,
}

#[cfg(not(target_arch = "wasm32"))]
fn merge_line_edits(base: &[&str], changed: &[&str]) -> Vec<MergeLineEdit> {
    let diff = similar::TextDiff::from_slices(base, changed);
    diff.ops()
        .iter()
        .filter(|operation| operation.tag() != similar::DiffTag::Equal)
        .map(|operation| {
            let old = operation.old_range();
            let new = operation.new_range();
            MergeLineEdit {
                start: old.start,
                end: old.end,
                replacement: diff.new_slices()[new].concat(),
            }
        })
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn merge_edits_overlap(left: &MergeLineEdit, right: &MergeLineEdit) -> bool {
    if left.start == left.end && right.start == right.end {
        return left.start == right.start;
    }
    if left.start == left.end || right.start == right.end {
        return false;
    }
    left.start.max(right.start) < left.end.min(right.end)
}

#[cfg(not(target_arch = "wasm32"))]
fn render_merge_region(base: &[&str], start: usize, end: usize, edits: &[MergeLineEdit]) -> String {
    let mut output = String::new();
    let mut cursor = start;
    for edit in edits {
        output.push_str(&base[cursor..edit.start].concat());
        output.push_str(&edit.replacement);
        cursor = edit.end;
    }
    output.push_str(&base[cursor..end].concat());
    output
}

#[cfg(not(target_arch = "wasm32"))]
fn edit_intersects_region(edit: &MergeLineEdit, start: usize, end: usize) -> bool {
    if edit.start == edit.end {
        edit.start > start && edit.start < end
    } else {
        edit.start < end && edit.end > start
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn three_way_merge_source(base: Option<&str>, local: &str, external: &str) -> (String, usize) {
    if local == external {
        return (local.to_owned(), 0);
    }
    let Some(base) = base else {
        return (merge_conflict_block(local, external), 1);
    };
    if local == base {
        return (external.to_owned(), 0);
    }
    if external == base {
        return (local.to_owned(), 0);
    }

    let base_lines = base.split_inclusive('\n').collect::<Vec<_>>();
    let local_lines = local.split_inclusive('\n').collect::<Vec<_>>();
    let external_lines = external.split_inclusive('\n').collect::<Vec<_>>();
    let local_edits = merge_line_edits(&base_lines, &local_lines);
    let external_edits = merge_line_edits(&base_lines, &external_lines);
    let mut merged = String::with_capacity(local.len().max(external.len()));
    let mut conflicts = 0_usize;
    let mut local_index = 0_usize;
    let mut external_index = 0_usize;
    let mut cursor = 0_usize;
    while local_index < local_edits.len() || external_index < external_edits.len() {
        let local_edit = local_edits.get(local_index);
        let external_edit = external_edits.get(external_index);
        if let (Some(local_edit), Some(external_edit)) = (local_edit, external_edit)
            && merge_edits_overlap(local_edit, external_edit)
        {
            let cluster_start = local_edit.start.min(external_edit.start);
            let mut cluster_end = local_edit.end.max(external_edit.end);
            let mut local_end = local_index + 1;
            let mut external_end = external_index + 1;
            loop {
                let mut extended = false;
                while let Some(edit) = local_edits.get(local_end)
                    && edit_intersects_region(edit, cluster_start, cluster_end)
                {
                    cluster_end = cluster_end.max(edit.end);
                    local_end += 1;
                    extended = true;
                }
                while let Some(edit) = external_edits.get(external_end)
                    && edit_intersects_region(edit, cluster_start, cluster_end)
                {
                    cluster_end = cluster_end.max(edit.end);
                    external_end += 1;
                    extended = true;
                }
                if !extended {
                    break;
                }
            }
            merged.push_str(&base_lines[cursor..cluster_start].concat());
            let base_region = base_lines[cluster_start..cluster_end].concat();
            let local_region = render_merge_region(
                &base_lines,
                cluster_start,
                cluster_end,
                &local_edits[local_index..local_end],
            );
            let external_region = render_merge_region(
                &base_lines,
                cluster_start,
                cluster_end,
                &external_edits[external_index..external_end],
            );
            if local_region == external_region {
                merged.push_str(&local_region);
            } else if local_region == base_region {
                merged.push_str(&external_region);
            } else if external_region == base_region {
                merged.push_str(&local_region);
            } else {
                conflicts += 1;
                merged.push_str(&merge_conflict_block(&local_region, &external_region));
            }
            cursor = cluster_end;
            local_index = local_end;
            external_index = external_end;
            continue;
        }

        let use_local = match (local_edit, external_edit) {
            (Some(local), Some(external)) => local.start <= external.start,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => break,
        };
        let edit = if use_local {
            let edit = &local_edits[local_index];
            local_index += 1;
            edit
        } else {
            let edit = &external_edits[external_index];
            external_index += 1;
            edit
        };
        merged.push_str(&base_lines[cursor..edit.start].concat());
        merged.push_str(&edit.replacement);
        cursor = edit.end;
    }
    merged.push_str(&base_lines[cursor..].concat());
    (merged, conflicts)
}

#[cfg(not(target_arch = "wasm32"))]
fn merge_conflict_block(local: &str, external: &str) -> String {
    let mut merged = String::new();
    merged.push_str("<<<<<<< RSPICE LOCAL\n");
    merged.push_str(local);
    if !local.ends_with('\n') {
        merged.push('\n');
    }
    merged.push_str("=======\n");
    merged.push_str(external);
    if !external.ends_with('\n') {
        merged.push('\n');
    }
    merged.push_str(">>>>>>> EXTERNAL FILE\n");
    merged
}

#[cfg(not(target_arch = "wasm32"))]
fn stage_external_netlist_change(
    state: &mut AppState,
    path: &std::path::Path,
    expected_sha256: [u8; 32],
    local_source: &str,
) -> Result<bool, String> {
    let bytes = std::fs::read(path).map_err(|error| {
        format!("The existing source cannot be reopened for comparison: {error}")
    })?;
    if bytes.len() as u64 > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
        return Err(format!(
            "The externally changed source exceeds the supported {}-byte limit.",
            crate::io::project_io::MAX_PROJECT_FILE_BYTES
        ));
    }
    let observed_sha256 = sha256(&bytes);
    if observed_sha256 == expected_sha256 {
        return Ok(false);
    }
    let (external_source, external_encoding) = decode_import_bytes(&bytes)?;
    let descriptor = state
        .workspace
        .netlist_descriptor
        .as_ref()
        .ok_or_else(|| "Owned source metadata is unavailable.".to_owned())?;
    let saved_digest = descriptor
        .save_history
        .last()
        .map(|record| record.content_digest);
    let base_source = saved_digest
        .and_then(|digest| {
            descriptor
                .revision_history
                .iter()
                .rev()
                .find(|snapshot| snapshot.content_digest == digest)
        })
        .or_else(|| {
            descriptor
                .save_history
                .is_empty()
                .then(|| descriptor.revision_history.first())
                .flatten()
        })
        .map(|snapshot| snapshot.source.clone());
    let (merged_source, merge_conflict_count) =
        three_way_merge_source(base_source.as_deref(), local_source, &external_source);
    let comparison = similar::TextDiff::from_lines(local_source, external_source.as_str())
        .unified_diff()
        .context_radius(3)
        .header("RSpice local editor", "External file")
        .to_string();
    state.ui.netlist.external_change = Some(
        crate::workbench::documents::netlist_document::NetlistExternalChangeState {
            path: path.to_path_buf(),
            expected_sha256,
            observed_sha256,
            local_source: local_source.to_owned(),
            external_source,
            base_source,
            merged_source,
            merge_conflict_count,
            comparison,
            external_encoding,
            resolution: crate::workbench::documents::netlist_document::NetlistExternalChangeResolution::Merge,
            error: None,
        },
    );
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    Ok(true)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn apply_staged_external_netlist_change(state: &mut AppState) -> Result<(), String> {
    use crate::workbench::documents::netlist_document::NetlistExternalChangeResolution;

    let review = state
        .ui
        .netlist
        .external_change
        .clone()
        .ok_or_else(|| "No external source change is staged.".to_owned())?;
    let current = state
        .ui
        .netlist
        .owned_document
        .as_ref()
        .cloned()
        .ok_or_else(|| "No current owned source document is available.".to_owned())?;
    if current.source() != review.local_source
        || state.workspace.netlist_source_path.as_deref() != Some(review.path.as_path())
        || state
            .workspace
            .netlist_descriptor
            .as_ref()
            .and_then(|descriptor| descriptor.external_file_sha256)
            != Some(review.expected_sha256)
    {
        return Err(
            "The source, path, or publication baseline changed while conflict review was open."
                .to_owned(),
        );
    }
    let current_external = std::fs::read(&review.path)
        .map_err(|error| format!("The external source can no longer be read: {error}"))?;
    if sha256(&current_external) != review.observed_sha256 {
        return Err(
            "The external source changed again while conflict review was open. Cancel and review the newer bytes."
                .to_owned(),
        );
    }

    let selected_source = match review.resolution {
        NetlistExternalChangeResolution::Merge => review.merged_source.clone(),
        NetlistExternalChangeResolution::KeepLocal => review.local_source.clone(),
        NetlistExternalChangeResolution::ReloadExternal => review.external_source.clone(),
    };
    let mut candidate = state.clone();
    let mut descriptor = candidate
        .workspace
        .netlist_descriptor
        .take()
        .ok_or_else(|| "Owned source metadata is unavailable.".to_owned())?;
    descriptor.retain_revision(&current, "Working state before external-change resolution")?;
    descriptor.external_file_sha256 = Some(review.observed_sha256);
    if review.resolution != NetlistExternalChangeResolution::KeepLocal {
        descriptor.source_encoding = review.external_encoding;
        descriptor.source_line_ending = crate::state::NetlistLineEnding::detect(&selected_source);
    }
    candidate.workspace.netlist_descriptor = Some(descriptor);

    if selected_source != review.local_source
        && !crate::workbench::documents::netlist_document::replace_owned_source(
            &mut candidate,
            selected_source.clone(),
        )
    {
        return Err(
            "The selected conflict resolution could not update the owned source.".to_owned(),
        );
    }
    let next_document = candidate
        .ui
        .netlist
        .owned_document
        .as_ref()
        .cloned()
        .ok_or_else(|| "The conflict resolution lost the canonical owned document.".to_owned())?;
    candidate
        .workspace
        .netlist_descriptor
        .as_mut()
        .unwrap()
        .retain_revision(
            &next_document,
            match review.resolution {
                NetlistExternalChangeResolution::Merge => "Merged external source change",
                NetlistExternalChangeResolution::KeepLocal => {
                    "Acknowledged external change; retained local source"
                }
                NetlistExternalChangeResolution::ReloadExternal => "Reloaded external source",
            },
        )?;
    candidate.ui.netlist.externally_saved_content_digest = (review.resolution
        == NetlistExternalChangeResolution::ReloadExternal)
        .then(|| crate::state::content_digest(&selected_source));
    candidate.ui.netlist.external_change = None;
    candidate
        .workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    *state = candidate;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn apply_staged_external_netlist_change(_state: &mut AppState) -> Result<(), String> {
    Err("Browser downloads do not expose reopenable external source authority.".to_owned())
}

#[derive(Debug)]
struct ImportedNetlistBundle {
    source: String,
    dependencies: Vec<crate::state::DependencyMetadata>,
    expanded_source: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedBundleManifest {
    schema: String,
    main: String,
    main_content_digest: String,
    dialect: String,
    retained_generated_source: String,
    generated_content_digest: String,
    generator: String,
    input_revision: u64,
    input_digest: String,
    source_map: Option<String>,
    dependencies: Vec<GeneratedBundleDependency>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedBundleDependency {
    requested_locator: String,
    logical_identity: String,
    #[serde(default)]
    authority: crate::state::DependencySourceAuthority,
    bundle_entry: String,
    content_digest: String,
    retained_entry: String,
    retained_content_digest: Option<String>,
    edge: GeneratedBundleEdge,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedBundleEdge {
    owner: String,
    include_index: usize,
}

fn parse_generated_netlist_bundle(bytes: &[u8]) -> Result<ImportedNetlistBundle, String> {
    use std::collections::{BTreeMap, HashMap, HashSet};

    const MAX_ARCHIVE_ENTRIES: usize = 20_004;
    let entries = read_stored_zip_entries(bytes, MAX_ARCHIVE_ENTRIES)?;
    let manifest_bytes = entries
        .get("manifest.json")
        .ok_or_else(|| "Netlist bundle has no manifest.json entry.".to_owned())?;
    let manifest: GeneratedBundleManifest = serde_json::from_slice(manifest_bytes)
        .map_err(|error| format!("Netlist bundle manifest is invalid: {error}"))?;
    if manifest.schema != "rspice-generated-netlist-bundle/v1" {
        return Err(format!(
            "Unsupported netlist bundle schema '{}'.",
            manifest.schema
        ));
    }
    if manifest.dependencies.len() > 10_000 {
        return Err("Netlist bundle declares more than 10,000 dependencies.".to_owned());
    }
    if manifest.input_revision == 0
        || manifest.generator.trim().is_empty()
        || manifest.dialect.trim().is_empty()
    {
        return Err("Netlist bundle manifest has an invalid generation identity.".to_owned());
    }
    validate_digest_literal(&manifest.input_digest, "input digest")?;
    let main = bundle_entry(&entries, &manifest.main)?;
    verify_content_digest(
        main,
        &manifest.main_content_digest,
        "materialized main deck",
    )?;
    let retained_root = bundle_entry(&entries, &manifest.retained_generated_source)?;
    verify_content_digest(
        retained_root,
        &manifest.generated_content_digest,
        "retained generated deck",
    )?;
    let source = std::str::from_utf8(retained_root)
        .map_err(|error| format!("Retained generated deck is not UTF-8: {error}"))?
        .to_owned();

    let mut expected_entries = HashSet::new();
    expected_entries.insert("manifest.json".to_owned());
    expected_entries.insert(manifest.main.clone());
    expected_entries.insert(manifest.retained_generated_source.clone());
    if let Some(source_map) = manifest.source_map.as_deref() {
        let map = bundle_entry(&entries, source_map)?;
        let value: serde_json::Value = serde_json::from_slice(map)
            .map_err(|error| format!("Generated source map is invalid: {error}"))?;
        if value.get("schema").and_then(serde_json::Value::as_str)
            != Some("rspice-generated-source-map/v1")
            || value
                .get("generated_content_digest")
                .and_then(serde_json::Value::as_str)
                != Some(manifest.generated_content_digest.as_str())
        {
            return Err(
                "Generated source map does not authenticate the retained generated deck."
                    .to_owned(),
            );
        }
        expected_entries.insert(source_map.to_owned());
    }

    let mut locators = HashMap::with_capacity(manifest.dependencies.len());
    let mut retained_sources = BTreeMap::new();
    for dependency in &manifest.dependencies {
        validate_bundle_entry_name(&dependency.bundle_entry)?;
        validate_bundle_entry_name(&dependency.retained_entry)?;
        let materialized = bundle_entry(&entries, &dependency.bundle_entry)?;
        verify_content_digest(
            materialized,
            &dependency.content_digest,
            &format!("materialized dependency '{}'", dependency.logical_identity),
        )?;
        let retained = bundle_entry(&entries, &dependency.retained_entry)?;
        let retained_digest = dependency
            .retained_content_digest
            .as_deref()
            .ok_or_else(|| {
                format!(
                    "Dependency '{}' has no retained content digest.",
                    dependency.logical_identity
                )
            })?;
        verify_content_digest(
            retained,
            retained_digest,
            &format!("retained dependency '{}'", dependency.logical_identity),
        )?;
        let retained = std::str::from_utf8(retained).map_err(|error| {
            format!(
                "Retained dependency '{}' is not UTF-8: {error}",
                dependency.logical_identity
            )
        })?;
        let display_name = dependency
            .logical_identity
            .rsplit(['/', '\\'])
            .find(|component| !component.is_empty())
            .unwrap_or("dependency.sp");
        let locator = crate::state::SourceLocator::try_new(
            dependency.logical_identity.clone(),
            display_name.to_owned(),
        )
        .map_err(|error| error.to_string())?;
        if locators
            .insert(dependency.logical_identity.clone(), locator)
            .is_some()
            || retained_sources
                .insert(dependency.logical_identity.clone(), retained.to_owned())
                .is_some()
        {
            return Err(format!(
                "Netlist bundle repeats dependency identity '{}'.",
                dependency.logical_identity
            ));
        }
        expected_entries.insert(dependency.bundle_entry.clone());
        expected_entries.insert(dependency.retained_entry.clone());
    }
    if entries.keys().any(|name| !expected_entries.contains(name)) {
        let extras = entries
            .keys()
            .filter(|name| !expected_entries.contains(*name))
            .take(4)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "Netlist bundle contains undeclared archive entries: {extras}."
        ));
    }

    let mut dependencies = Vec::with_capacity(manifest.dependencies.len());
    for dependency in &manifest.dependencies {
        let locator = locators
            .get(&dependency.logical_identity)
            .cloned()
            .ok_or_else(|| "Netlist bundle dependency map is inconsistent.".to_owned())?;
        let retained = retained_sources
            .get(&dependency.logical_identity)
            .ok_or_else(|| "Netlist bundle retained-source map is inconsistent.".to_owned())?;
        let record = if dependency.edge.owner == "generated" {
            crate::state::DependencyMetadata::unresolved_direct_to(
                dependency.edge.include_index,
                dependency.requested_locator.clone(),
                locator,
            )
        } else {
            let parent = locators
                .get(&dependency.edge.owner)
                .cloned()
                .ok_or_else(|| {
                    format!(
                        "Dependency '{}' references undeclared parent '{}'.",
                        dependency.logical_identity, dependency.edge.owner
                    )
                })?;
            crate::state::DependencyMetadata::unresolved_transitive_to(
                parent,
                dependency.edge.include_index,
                dependency.requested_locator.clone(),
                locator,
            )
        }
        .map(|record| record.with_authority(dependency.authority))
        .and_then(|record| record.resolve_utf8(retained.as_bytes().to_vec()))
        .map_err(|error| error.to_string())?;
        dependencies.push(record);
    }

    let document_id = crate::state::NetlistDocumentId::new();
    let expanded = if dependencies.is_empty() {
        source.clone()
    } else {
        crate::state::expand_retained_netlist_dependencies(document_id, &source, &dependencies)?
            .source
    };
    rspice_core::Netlist::parse(&expanded)
        .map_err(|error| format!("Retained archive closure does not parse: {error}"))?;
    Ok(ImportedNetlistBundle {
        source,
        dependencies,
        expanded_source: expanded,
    })
}

fn bundle_entry<'a>(
    entries: &'a std::collections::BTreeMap<String, Vec<u8>>,
    name: &str,
) -> Result<&'a [u8], String> {
    validate_bundle_entry_name(name)?;
    entries
        .get(name)
        .map(Vec::as_slice)
        .ok_or_else(|| format!("Netlist bundle is missing declared entry '{name}'."))
}

fn validate_digest_literal(value: &str, label: &str) -> Result<(), String> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(format!(
            "Netlist bundle {label} is not a canonical SHA-256 digest."
        ));
    }
    Ok(())
}

fn verify_content_digest(bytes: &[u8], expected: &str, label: &str) -> Result<(), String> {
    validate_digest_literal(expected, label)?;
    let actual = crate::state::content_digest(
        std::str::from_utf8(bytes)
            .map_err(|error| format!("Netlist bundle {label} is not UTF-8: {error}"))?,
    )
    .to_string();
    if actual != expected {
        return Err(format!(
            "Netlist bundle {label} failed digest verification: expected {expected}, found {actual}."
        ));
    }
    Ok(())
}

fn validate_bundle_entry_name(name: &str) -> Result<(), String> {
    if name.is_empty()
        || name.len() > 4_096
        || name.starts_with('/')
        || name.starts_with('\\')
        || name.contains('\\')
        || name.contains(':')
        || name.chars().any(char::is_control)
        || name
            .split('/')
            .any(|component| component.is_empty() || matches!(component, "." | ".."))
    {
        return Err(format!("Unsafe netlist bundle entry name '{name}'."));
    }
    Ok(())
}

fn read_stored_zip_entries(
    bytes: &[u8],
    max_entries: usize,
) -> Result<std::collections::BTreeMap<String, Vec<u8>>, String> {
    const EOCD_BYTES: usize = 22;
    if bytes.len() < EOCD_BYTES {
        return Err("Netlist bundle is truncated before the ZIP end record.".to_owned());
    }
    let eocd = bytes.len() - EOCD_BYTES;
    if zip_u32(bytes, eocd)? != 0x0605_4b50 || zip_u16(bytes, eocd + 20)? != 0 {
        return Err(
            "Netlist bundle must be a single-disk ZIP with no trailing comment.".to_owned(),
        );
    }
    if zip_u16(bytes, eocd + 4)? != 0 || zip_u16(bytes, eocd + 6)? != 0 {
        return Err("Multi-disk ZIP archives are not supported.".to_owned());
    }
    let entry_count = usize::from(zip_u16(bytes, eocd + 10)?);
    if entry_count != usize::from(zip_u16(bytes, eocd + 8)?) || entry_count > max_entries {
        return Err(format!(
            "Netlist bundle entry count is inconsistent or exceeds {max_entries}."
        ));
    }
    let central_size = usize::try_from(zip_u32(bytes, eocd + 12)?)
        .map_err(|_| "ZIP central directory size is invalid.".to_owned())?;
    let central_offset = usize::try_from(zip_u32(bytes, eocd + 16)?)
        .map_err(|_| "ZIP central directory offset is invalid.".to_owned())?;
    if central_offset.checked_add(central_size) != Some(eocd) {
        return Err(
            "ZIP central directory does not end at the authenticated end record.".to_owned(),
        );
    }
    let mut cursor = central_offset;
    let mut entries = std::collections::BTreeMap::new();
    let mut retained_bytes = 0usize;
    for _ in 0..entry_count {
        if zip_u32(bytes, cursor)? != 0x0201_4b50 {
            return Err("ZIP central directory contains an invalid record signature.".to_owned());
        }
        let flags = zip_u16(bytes, cursor + 8)?;
        let method = zip_u16(bytes, cursor + 10)?;
        let crc = zip_u32(bytes, cursor + 16)?;
        let compressed = usize::try_from(zip_u32(bytes, cursor + 20)?)
            .map_err(|_| "ZIP entry size is invalid.".to_owned())?;
        let uncompressed = usize::try_from(zip_u32(bytes, cursor + 24)?)
            .map_err(|_| "ZIP entry size is invalid.".to_owned())?;
        let name_len = usize::from(zip_u16(bytes, cursor + 28)?);
        let extra_len = usize::from(zip_u16(bytes, cursor + 30)?);
        let comment_len = usize::from(zip_u16(bytes, cursor + 32)?);
        let local_offset = usize::try_from(zip_u32(bytes, cursor + 42)?)
            .map_err(|_| "ZIP local record offset is invalid.".to_owned())?;
        if flags != 0x0800 || method != 0 || compressed != uncompressed {
            return Err(
                "Netlist bundles require UTF-8 names and stored, unencrypted ZIP members."
                    .to_owned(),
            );
        }
        let name_start = cursor
            .checked_add(46)
            .ok_or_else(|| "ZIP central record overflowed.".to_owned())?;
        let name_end = name_start
            .checked_add(name_len)
            .ok_or_else(|| "ZIP entry name overflowed.".to_owned())?;
        let record_end = name_end
            .checked_add(extra_len)
            .and_then(|end| end.checked_add(comment_len))
            .ok_or_else(|| "ZIP central record overflowed.".to_owned())?;
        let name = std::str::from_utf8(
            bytes
                .get(name_start..name_end)
                .ok_or_else(|| "ZIP central entry name is truncated.".to_owned())?,
        )
        .map_err(|error| format!("ZIP entry name is not UTF-8: {error}"))?
        .to_owned();
        validate_bundle_entry_name(&name)?;

        if zip_u32(bytes, local_offset)? != 0x0403_4b50
            || zip_u16(bytes, local_offset + 6)? != flags
            || zip_u16(bytes, local_offset + 8)? != method
            || zip_u32(bytes, local_offset + 14)? != crc
            || usize::try_from(zip_u32(bytes, local_offset + 18)?).ok() != Some(compressed)
            || usize::try_from(zip_u32(bytes, local_offset + 22)?).ok() != Some(uncompressed)
        {
            return Err(format!(
                "ZIP local record for '{name}' disagrees with its directory."
            ));
        }
        let local_name_len = usize::from(zip_u16(bytes, local_offset + 26)?);
        let local_extra_len = usize::from(zip_u16(bytes, local_offset + 28)?);
        let local_name_start = local_offset
            .checked_add(30)
            .ok_or_else(|| "ZIP local record overflowed.".to_owned())?;
        let local_name_end = local_name_start
            .checked_add(local_name_len)
            .ok_or_else(|| "ZIP local entry name overflowed.".to_owned())?;
        if bytes.get(local_name_start..local_name_end) != Some(name.as_bytes()) {
            return Err(format!(
                "ZIP local record for '{name}' has a different name."
            ));
        }
        let content_start = local_name_end
            .checked_add(local_extra_len)
            .ok_or_else(|| "ZIP local entry overflowed.".to_owned())?;
        let content_end = content_start
            .checked_add(uncompressed)
            .ok_or_else(|| "ZIP entry content overflowed.".to_owned())?;
        let content = bytes
            .get(content_start..content_end)
            .ok_or_else(|| format!("ZIP entry '{name}' is truncated."))?;
        if bundle_crc32(content) != crc {
            return Err(format!("ZIP entry '{name}' failed CRC-32 verification."));
        }
        retained_bytes = retained_bytes
            .checked_add(content.len())
            .ok_or_else(|| "Netlist bundle expanded size overflowed.".to_owned())?;
        if retained_bytes as u64 > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
            return Err(format!(
                "Netlist bundle expands beyond the supported {}-byte limit.",
                crate::io::project_io::MAX_PROJECT_FILE_BYTES
            ));
        }
        if entries.insert(name.clone(), content.to_vec()).is_some() {
            return Err(format!("Netlist bundle repeats ZIP entry '{name}'."));
        }
        cursor = record_end;
    }
    if cursor != eocd {
        return Err("ZIP central directory length does not match its records.".to_owned());
    }
    Ok(entries)
}

fn zip_u16(bytes: &[u8], offset: usize) -> Result<u16, String> {
    let value = bytes
        .get(offset..offset.saturating_add(2))
        .ok_or_else(|| "ZIP structure is truncated.".to_owned())?;
    Ok(u16::from_le_bytes([value[0], value[1]]))
}

fn zip_u32(bytes: &[u8], offset: usize) -> Result<u32, String> {
    let value = bytes
        .get(offset..offset.saturating_add(4))
        .ok_or_else(|| "ZIP structure is truncated.".to_owned())?;
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

fn bundle_crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xffff_ffff_u32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = 0_u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0xedb8_8320 & mask);
        }
    }
    !crc
}

fn decode_import_bytes(
    bytes: &[u8],
) -> Result<(String, crate::state::NetlistTextEncoding), String> {
    let encoding = if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        crate::state::NetlistTextEncoding::Utf8Bom
    } else if bytes.starts_with(&[0xff, 0xfe]) {
        crate::state::NetlistTextEncoding::Utf16LeBom
    } else if bytes.starts_with(&[0xfe, 0xff]) {
        crate::state::NetlistTextEncoding::Utf16BeBom
    } else if std::str::from_utf8(bytes).is_ok() {
        crate::state::NetlistTextEncoding::Utf8
    } else {
        crate::state::NetlistTextEncoding::Latin1
    };
    let source = rspice_core::netlist::decode_source_bytes(bytes)
        .map_err(|error| format!("source decoding failed: {error}"))?;
    Ok((source, encoding))
}

fn detect_netlist_dialect(source: &str) -> (crate::state::NetlistSourceDialect, Vec<String>) {
    use crate::state::NetlistSourceDialect;

    let mut spectre = 0usize;
    let mut hspice = 0usize;
    let mut pspice = 0usize;
    let mut ngspice = 0usize;
    let mut ads = 0usize;
    let mut evidence = Vec::new();
    for (line_index, raw) in source.lines().take(500_000).enumerate() {
        let line = raw.trim().to_ascii_lowercase();
        let (score, description) = if line.starts_with("simulator lang=")
            || line.starts_with("ahdl_include")
            || line.starts_with("saveoptions ")
            || line.starts_with("parameters ")
        {
            spectre += 1;
            (true, "Spectre language directive")
        } else if line.starts_with(".option post")
            || line == ".protect"
            || line == ".unprotect"
            || line.starts_with(".alter")
        {
            hspice += 1;
            (true, "HSPICE compatibility directive")
        } else if line.starts_with(".probe")
            || line.starts_with(".distribution")
            || line.starts_with(".stimulus")
        {
            pspice += 1;
            (true, "PSpice compatibility directive")
        } else if line == ".control"
            || line == ".endc"
            || line.starts_with("wrdata ")
            || line.starts_with("setplot ")
        {
            ngspice += 1;
            (true, "ngspice control-language directive")
        } else if line.starts_with("#uselib")
            || line.starts_with("define ")
            || line.starts_with("simulatoroptions ")
            || line.starts_with("options resourceusage=")
        {
            ads += 1;
            (true, "ADS netlist directive")
        } else {
            (false, "")
        };
        if score && evidence.len() < 12 {
            evidence.push(format!("line {}: {description}", line_index + 1));
        }
    }

    let scores = [
        (spectre, NetlistSourceDialect::Spectre),
        (hspice, NetlistSourceDialect::Hspice),
        (pspice, NetlistSourceDialect::Pspice),
        (ngspice, NetlistSourceDialect::Spice3Ngspice),
        (ads, NetlistSourceDialect::Ads),
    ];
    let maximum = scores.iter().map(|(score, _)| *score).max().unwrap_or(0);
    if maximum == 0 {
        return (NetlistSourceDialect::RSpice, evidence);
    }
    let mut matches = scores
        .iter()
        .filter_map(|(score, dialect)| (*score == maximum).then_some(*dialect));
    let first = matches.next().unwrap_or(NetlistSourceDialect::Unknown);
    let dialect = if matches.next().is_some() {
        NetlistSourceDialect::Unknown
    } else {
        first
    };
    (dialect, evidence)
}

fn validate_import_candidate(
    source: &str,
    source_path: Option<&std::path::Path>,
    execution_profile: Option<crate::state::NetlistExecutionProfile>,
) -> Vec<crate::workbench::documents::netlist_document::NetlistImportIssue> {
    use crate::workbench::documents::netlist_document::{
        NetlistImportIssue, NetlistImportIssueSeverity,
    };

    let mut issues = Vec::new();
    if source.trim().is_empty() {
        issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Blocking,
            message: "The selected deck is empty.".to_owned(),
        });
        return issues;
    }
    if let Some((character_index, character)) = source
        .chars()
        .enumerate()
        .find(|(_, character)| character.is_control() && !matches!(character, '\n' | '\r' | '\t'))
    {
        issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Blocking,
            message: format!(
                "Unsupported control character U+{:04X} at decoded character {}.",
                u32::from(character),
                character_index + 1
            ),
        });
        return issues;
    }

    let adapted = match execution_profile
        .map(|profile| profile.adapt_source(source))
        .transpose()
    {
        Ok(Some(adapted)) => adapted,
        Ok(None) => std::borrow::Cow::Borrowed(source),
        Err(error) => {
            issues.push(NetlistImportIssue {
                severity: NetlistImportIssueSeverity::Blocking,
                message: format!("Execution-profile adaptation failed: {error}"),
            });
            return issues;
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    let parsed = source_path.map_or_else(
        || rspice_core::Netlist::parse(&adapted),
        |path| rspice_core::Netlist::parse_with_path(&adapted, path),
    );
    #[cfg(target_arch = "wasm32")]
    let parsed = {
        let _ = source_path;
        rspice_core::Netlist::parse(&adapted)
    };

    match parsed {
        Ok(netlist) => {
            if let Some(profile) = execution_profile
                && let Err(error) = profile.validate_parsed_netlist(&netlist)
            {
                issues.push(NetlistImportIssue {
                    severity: NetlistImportIssueSeverity::Blocking,
                    message: format!("Execution-profile validation failed: {error}"),
                });
            }
            for diagnostic in &netlist.diagnostics {
                let semantic_loss = matches!(
                    diagnostic.code.as_str(),
                    "unknown-option"
                        | "unsupported-dot-command"
                        | "control-block-ignored"
                        | "invalid-option-defaulted"
                );
                issues.push(NetlistImportIssue {
                    severity: if semantic_loss {
                        NetlistImportIssueSeverity::Blocking
                    } else {
                        NetlistImportIssueSeverity::Advisory
                    },
                    message: format!(
                        "Parser diagnostic {} at line {}: {}",
                        diagnostic.code, diagnostic.line, diagnostic.message
                    ),
                });
            }
            if let Err(error) = rspice_core::netlist::validate_output_symbols(&netlist) {
                issues.push(NetlistImportIssue {
                    severity: NetlistImportIssueSeverity::Blocking,
                    message: format!("Output-symbol validation failed: {error}"),
                });
            }
        }
        Err(error) => issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Blocking,
            message: format!("Canonical parse/include validation failed: {error}"),
        }),
    }
    issues
}

fn stage_netlist_import(
    state: &mut AppState,
    transaction: crate::workbench::lifecycle::project_lifecycle::TransactionId,
    mode: NetlistImportMode,
    bytes: Vec<u8>,
    source_path: Option<std::path::PathBuf>,
    display_name: String,
) -> bool {
    use crate::workbench::documents::netlist_document::{
        NetlistImportIssue, NetlistImportIssueSeverity, NetlistImportOperation,
        NetlistImportReviewState,
    };

    if bytes.len() as u64 > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
        state.push_user_message(ConsoleMessage::error(format!(
            "SPICE deck import failed: selected file exceeds the supported {}-byte size limit",
            crate::io::project_io::MAX_PROJECT_FILE_BYTES
        )));
        return false;
    }
    if let Err(error) = crate::workbench::lifecycle::project_lifecycle::validate_project_replacement(
        state,
        transaction,
    ) {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
        state.push_user_message(ConsoleMessage::error(format!(
            "{} was cancelled because the project changed: {error}",
            mode.dialog_title()
        )));
        return false;
    }
    if bytes.starts_with(b"PK\x03\x04") {
        return stage_netlist_bundle_import(
            state,
            transaction,
            mode,
            bytes,
            source_path,
            display_name,
        );
    }
    let (source, encoding) = match decode_import_bytes(&bytes) {
        Ok(decoded) => decoded,
        Err(error) => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE deck import failed: {error}"
            )));
            return false;
        }
    };
    let line_ending = crate::state::NetlistLineEnding::detect(&source);
    let (detected_dialect, detection_evidence) = detect_netlist_dialect(&source);
    let mut issues = validate_import_candidate(
        &source,
        source_path.as_deref(),
        detected_dialect.execution_profile(),
    );
    if line_ending == crate::state::NetlistLineEnding::Mixed {
        issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Advisory,
            message: "The deck contains mixed line endings; RSpice will preserve them unless the source is explicitly formatted.".to_owned(),
        });
    }
    if detected_dialect.requires_compatibility_review() {
        issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Advisory,
            message: format!(
                "{} was detected. No source statement will be translated silently; accepting records an explicit compatibility profile.",
                detected_dialect.label()
            ),
        });
    }
    let mut transformations = vec![
        format!(
            "Losslessly decoded {} into the editor's Unicode representation; Save retains the original encoding.",
            encoding.label()
        ),
        format!(
            "Preserved {} line endings and source statement order.",
            line_ending.label()
        ),
        "Applied no model substitution, unit coercion, analysis deletion, or unsupported-statement deletion.".to_owned(),
    ];
    if detected_dialect == crate::state::NetlistSourceDialect::Spice3Ngspice {
        transformations.push(
            "At validation and execution only, spice3-ngspice/2 promotes the bounded declarative .control/.endc subset (op, dc, ac, sp, tran, save, and simple aggregate measurements) into line-preserving dot directives; retained project source bytes remain unchanged and imperative commands fail closed."
                .to_owned(),
        );
    }
    if detected_dialect == crate::state::NetlistSourceDialect::Pspice {
        transformations.push(
            "The pspice-declarative/2 profile requires at least one pre-.END .PROBE, .PROBE64, or .PROBE/CSDF source marker. Ordinary analyses plus qualified .TF, .STEP, .FOUR, model DEV/LOT, .DISTRIBUTION, selected-analysis .MC collation, and E/G CHEBYSHEV LP/HP/BP/BR sources retain their source form. CHEBYSHEV uses an exact minimum-order analog Type-I realization; typed .MC LIST/OUTPUT selection is retained as bounded immutable result data without automatic file writes. Missing evidence, unsupported .STIMULUS or FREQ sources, and unknown output-format commands fail closed."
                .to_owned(),
        );
    }
    if detected_dialect == crate::state::NetlistSourceDialect::Hspice {
        transformations.push(
            "At validation and execution only, hspice-declarative/1 requires at least one pre-.END .OPTION POST or .PROTECT/.UNPROTECT source marker and maps those qualified presentation directives to line-preserving comments; retained project source bytes remain unchanged, while every other HSPICE .OPTION fails closed."
                .to_owned(),
        );
    }
    if detected_dialect == crate::state::NetlistSourceDialect::Spectre {
        transformations.push(
            "At validation and execution only, spectre-spice/1 requires exactly one `simulator lang=spice` interoperability boundary before .END and maps it to a line-preserving comment; retained project source bytes remain unchanged, while missing/duplicate boundaries and native Spectre statements fail closed."
                .to_owned(),
        );
    }
    if detected_dialect == crate::state::NetlistSourceDialect::Ads {
        transformations.push(
            "At validation and execution only, ads-spice-export/1 requires exactly one qualified ADS ResourceUsage/UseNutmegFormat/TopDesignName export header before .END and maps it to a line-preserving comment; retained source bytes remain unchanged, while missing/duplicate headers and native ADS/preprocessor statements fail closed."
                .to_owned(),
        );
    }
    state.ui.netlist.import_review = Some(NetlistImportReviewState {
        transaction,
        operation: match mode {
            NetlistImportMode::OpenProject => NetlistImportOperation::OpenProject,
            NetlistImportMode::ImportIntoProject => NetlistImportOperation::ImportIntoProject,
        },
        display_name,
        selected_file_path: source_path.clone(),
        source_path,
        source,
        dependencies: Vec::new(),
        archive_import: false,
        original_byte_count: bytes.len(),
        original_sha256: sha256(&bytes),
        encoding,
        line_ending,
        detected_dialect,
        selected_dialect: detected_dialect,
        detection_evidence,
        transformations,
        issues,
        compatibility_accepted: false,
        error: None,
    });
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    true
}

/// Stage a desktop/browser drag-and-drop import through the same bounded,
/// revision-guarded transaction used by the explicit picker.
pub(crate) fn stage_dropped_netlist_import(
    state: &mut AppState,
    bytes: Vec<u8>,
    source_path: Option<std::path::PathBuf>,
    display_name: String,
) -> bool {
    let mode = NetlistImportMode::ImportIntoProject;
    if let Some(reason) = netlist_import_start_block_reason(state, mode) {
        state.push_user_message(ConsoleMessage::error(format!(
            "Dropped SPICE source is unavailable: {reason}"
        )));
        return false;
    }
    let transaction =
        match crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(state) {
            Ok(transaction) => transaction,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "Dropped SPICE source is unavailable: {error}"
                )));
                return false;
            }
        };
    stage_netlist_import(state, transaction, mode, bytes, source_path, display_name)
}

fn stage_netlist_bundle_import(
    state: &mut AppState,
    transaction: crate::workbench::lifecycle::project_lifecycle::TransactionId,
    mode: NetlistImportMode,
    bytes: Vec<u8>,
    selected_file_path: Option<std::path::PathBuf>,
    archive_display_name: String,
) -> bool {
    use crate::workbench::documents::netlist_document::{
        NetlistImportIssue, NetlistImportIssueSeverity, NetlistImportOperation,
        NetlistImportReviewState,
    };

    let bundle = match parse_generated_netlist_bundle(&bytes) {
        Ok(bundle) => bundle,
        Err(error) => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "RSpice netlist bundle import failed: {error}"
            )));
            return false;
        }
    };
    let line_ending = crate::state::NetlistLineEnding::detect(&bundle.source);
    let mut issues = validate_import_candidate(
        &bundle.expanded_source,
        None,
        Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
    );
    if line_ending == crate::state::NetlistLineEnding::Mixed {
        issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Advisory,
            message: "The retained root deck contains mixed line endings; RSpice will preserve them unless the source is explicitly formatted.".to_owned(),
        });
    }
    if bundle.dependencies.is_empty() {
        issues.push(NetlistImportIssue {
            severity: NetlistImportIssueSeverity::Advisory,
            message: "The authenticated bundle contains no retained dependency members.".to_owned(),
        });
    }
    let artifact_stem = std::path::Path::new(&archive_display_name)
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
        .map(str::trim)
        .filter(|stem| !stem.is_empty() && *stem != "." && *stem != "..")
        .unwrap_or("imported-netlist");
    let display_name = format!("{artifact_stem}.spice");
    let dependency_count = bundle.dependencies.len();
    let transformations = vec![
        "Verified the RSpice bundle schema, ZIP structure, member declarations, CRC-32 values, and SHA-256 content identities before review.".to_owned(),
        format!(
            "Reconstructed and authenticated the retained dependency closure ({dependency_count} member{}).",
            if dependency_count == 1 { "" } else { "s" }
        ),
        "Validated the fully expanded retained deck without consulting the host filesystem or network.".to_owned(),
        "Preserved the retained root source and dependency bytes; applied no syntax rewrite, model substitution, unit coercion, or unsupported-statement deletion.".to_owned(),
    ];
    state.ui.netlist.import_review = Some(NetlistImportReviewState {
        transaction,
        operation: match mode {
            NetlistImportMode::OpenProject => NetlistImportOperation::OpenProject,
            NetlistImportMode::ImportIntoProject => NetlistImportOperation::ImportIntoProject,
        },
        display_name,
        selected_file_path,
        // Never associate a text Save with the archive selected for import.
        source_path: None,
        source: bundle.source,
        dependencies: bundle.dependencies,
        archive_import: true,
        original_byte_count: bytes.len(),
        original_sha256: sha256(&bytes),
        encoding: crate::state::NetlistTextEncoding::Utf8,
        line_ending,
        detected_dialect: crate::state::NetlistSourceDialect::RSpice,
        selected_dialect: crate::state::NetlistSourceDialect::RSpice,
        detection_evidence: vec![
            "Authenticated retained/generated.spice from rspice-generated-netlist-bundle/v1."
                .to_owned(),
        ],
        transformations,
        issues,
        compatibility_accepted: false,
        error: None,
    });
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    true
}

/// Stage the exact persisted owned source for an in-product execution-profile
/// review. No file picker, host interpreter, or external conversion is
/// involved; retained dependency bytes are reviewed and validated as part of
/// the same guarded project snapshot.
pub(crate) fn begin_owned_netlist_profile_review(state: &mut AppState) -> bool {
    use crate::workbench::documents::netlist_document::{
        NetlistImportIssue, NetlistImportIssueSeverity, NetlistImportOperation,
        NetlistImportReviewState,
    };

    let candidate = (|| -> Result<_, String> {
        if !state.project_lifecycle.project_open {
            return Err("Open the project before reviewing its netlist profile.".to_owned());
        }
        if state.workbench.safe_mode.project_read_only() {
            return Err("The project is read-only in the current safe mode.".to_owned());
        }
        let descriptor = state
            .workspace
            .netlist_descriptor
            .as_ref()
            .ok_or_else(|| "No owned netlist descriptor is available.".to_owned())?;
        if !descriptor.execution_profile_review_required() {
            return Err("The owned source already has an exact execution profile.".to_owned());
        }
        let document = state
            .workspace
            .netlist_document
            .as_ref()
            .ok_or_else(|| "No canonical owned netlist document is available.".to_owned())?;
        let source = state
            .workspace
            .netlist_source
            .as_ref()
            .filter(|source| source.as_str() == document.source())
            .cloned()
            .ok_or_else(|| {
                "Owned netlist bytes do not match their canonical document projection.".to_owned()
            })?;
        let dependencies = document.dependencies().to_vec();
        let validation_source = if dependencies.is_empty() {
            source.clone()
        } else {
            crate::state::expand_retained_netlist_dependencies(
                document.id(),
                &source,
                &dependencies,
            )
            .map_err(|error| format!("Retained dependency closure is invalid: {error}"))?
            .source
        };
        let selected_dialect = descriptor
            .imported_dialect
            .unwrap_or(crate::state::NetlistSourceDialect::RSpice);
        let mut issues = validate_import_candidate(
            &validation_source,
            None,
            selected_dialect.execution_profile(),
        );
        if selected_dialect.requires_compatibility_review() {
            issues.push(NetlistImportIssue {
                severity: NetlistImportIssueSeverity::Advisory,
                message: format!(
                    "{} is quarantined until the exact versioned execution profile is accepted.",
                    selected_dialect.label()
                ),
            });
        }
        let (detected_dialect, detection_evidence) = detect_netlist_dialect(&source);
        Ok((
            descriptor.artifact_name.clone(),
            descriptor.source_encoding,
            descriptor.source_line_ending,
            selected_dialect,
            detected_dialect,
            detection_evidence,
            source,
            dependencies,
            issues,
        ))
    })();
    let (
        display_name,
        encoding,
        line_ending,
        selected_dialect,
        detected_dialect,
        detection_evidence,
        source,
        dependencies,
        issues,
    ) = match candidate {
        Ok(candidate) => candidate,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Execution-profile review is unavailable: {error}"
            )));
            return false;
        }
    };
    let transaction =
        match crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(state) {
            Ok(transaction) => transaction,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "Execution-profile review is unavailable: {error}"
                )));
                return false;
            }
        };
    let original_byte_count = source.len();
    let original_sha256 = sha256(source.as_bytes());
    state.ui.netlist.import_review = Some(NetlistImportReviewState {
        transaction,
        operation: NetlistImportOperation::RequalifyOwnedSource,
        display_name,
        selected_file_path: None,
        source_path: None,
        source,
        dependencies,
        archive_import: false,
        original_byte_count,
        original_sha256,
        encoding,
        line_ending,
        detected_dialect,
        selected_dialect,
        detection_evidence,
        transformations: vec![
            "Retained the exact project-owned root source and authenticated dependency bytes."
                .to_owned(),
            "Any qualified foreign presentation directive is adapted only in the sealed executable copy; project source, models, units, and analysis statements remain unchanged."
                .to_owned(),
            "The commit updates only versioned execution-profile authority and leaves source history intact."
                .to_owned(),
        ],
        issues,
        compatibility_accepted: false,
        error: None,
    });
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    true
}

/// Commit the exact reviewed candidate as one project transaction. The live
/// state is replaced only after a complete clone validates and applies, so an
/// allocation, parse, origin, or domain error cannot partially import a deck.
pub(crate) fn commit_staged_netlist_import(state: &mut AppState) -> bool {
    let Some(review) = state.ui.netlist.import_review.clone() else {
        return false;
    };
    if let Err(error) = review.dialect_qualification() {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(error);
        }
        return false;
    }
    if !review.can_commit() {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(
                "Resolve every blocking issue and accept the declared compatibility profile before importing."
                    .to_owned(),
            );
        }
        return false;
    }
    if let Err(error) = crate::workbench::lifecycle::project_lifecycle::validate_project_replacement(
        state,
        review.transaction,
    ) {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(format!(
                "The project changed after this review opened. Cancel and import the source again: {error}"
            ));
        }
        return false;
    }

    #[cfg(not(target_arch = "wasm32"))]
    if let Some(path) = review.selected_file_path.as_deref() {
        let current_bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(error) => {
                if let Some(current) = state.ui.netlist.import_review.as_mut() {
                    current.error = Some(format!(
                        "The selected source can no longer be read. Cancel and import it again: {error}"
                    ));
                }
                return false;
            }
        };
        if current_bytes.len() as u64 > crate::io::project_io::MAX_PROJECT_FILE_BYTES
            || sha256(&current_bytes) != review.original_sha256
        {
            if let Some(current) = state.ui.netlist.import_review.as_mut() {
                current.error = Some(
                    "The selected source changed after review began. Cancel and import the new bytes again."
                        .to_owned(),
                );
            }
            return false;
        }
    }

    let validation_source = if review.dependencies.is_empty() {
        review.source.clone()
    } else {
        match crate::state::expand_retained_netlist_dependencies(
            crate::state::NetlistDocumentId::new(),
            &review.source,
            &review.dependencies,
        ) {
            Ok(expanded) => expanded.source,
            Err(error) => {
                if let Some(current) = state.ui.netlist.import_review.as_mut() {
                    current.error = Some(format!(
                        "The retained dependency closure no longer validates: {error}"
                    ));
                }
                return false;
            }
        }
    };
    let current_issues = validate_import_candidate(
        &validation_source,
        review.source_path.as_deref(),
        review.selected_dialect.execution_profile(),
    );
    if let Some(blocking) = current_issues.iter().find(|issue| {
        issue.severity
            == crate::workbench::documents::netlist_document::NetlistImportIssueSeverity::Blocking
    }) {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(format!(
                "The candidate no longer passes canonical validation: {}",
                blocking.message
            ));
        }
        return false;
    }

    if review.operation
        == crate::workbench::documents::netlist_document::NetlistImportOperation::RequalifyOwnedSource
    {
        return commit_owned_netlist_profile_review(state, review);
    }

    let metadata = NetlistImportMetadata {
        encoding: review.encoding,
        line_ending: review.line_ending,
        dialect: review.selected_dialect,
        compatibility_reviewed: review.selected_dialect.requires_compatibility_review(),
        raw_sha256: review.original_sha256,
    };
    let mode = match review.operation {
        crate::workbench::documents::netlist_document::NetlistImportOperation::OpenProject => {
            NetlistImportMode::OpenProject
        }
        crate::workbench::documents::netlist_document::NetlistImportOperation::ImportIntoProject => {
            NetlistImportMode::ImportIntoProject
        }
        crate::workbench::documents::netlist_document::NetlistImportOperation::RequalifyOwnedSource => {
            unreachable!("owned-source profile review commits before import-mode dispatch")
        }
    };
    let mut committed = state.clone();
    crate::workbench::lifecycle::project_lifecycle::cancel_transaction(&mut committed);
    if !apply_netlist_import_result(
        &mut committed,
        mode,
        review.source,
        review.source_path,
        &review.display_name,
        metadata,
        review.dependencies,
    ) {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(
                "The reviewed import could not be committed; the current project remains unchanged."
                    .to_owned(),
            );
        }
        return false;
    }
    *state = committed;
    true
}

fn commit_owned_netlist_profile_review(
    state: &mut AppState,
    review: crate::workbench::documents::netlist_document::NetlistImportReviewState,
) -> bool {
    let Some(profile) = review.selected_dialect.execution_profile() else {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(
                "The selected dialect has no versioned executable profile in this build."
                    .to_owned(),
            );
        }
        return false;
    };
    if state.workspace.netlist_source.as_deref() != Some(review.source.as_str())
        || sha256(review.source.as_bytes()) != review.original_sha256
    {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(
                "The owned source changed after review began. Cancel and review the current bytes again."
                    .to_owned(),
            );
        }
        return false;
    }

    let mut committed = state.clone();
    crate::workbench::lifecycle::project_lifecycle::cancel_transaction(&mut committed);
    let Some(descriptor) = committed.workspace.netlist_descriptor.as_mut() else {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some("The owned netlist descriptor is no longer available.".to_owned());
        }
        return false;
    };
    descriptor.imported_dialect = Some(review.selected_dialect);
    descriptor.compatibility_reviewed = review.selected_dialect.requires_compatibility_review();
    descriptor.execution_profile = Some(profile);
    committed.workspace.project_metadata_dirty = true;
    committed.ui.netlist.import_review = None;
    if let Err(error) = committed.workspace.validate_simulation_configuration() {
        if let Some(current) = state.ui.netlist.import_review.as_mut() {
            current.error = Some(format!(
                "The reviewed profile could not be recorded without invalidating project state: {error}"
            ));
        }
        return false;
    }
    *state = committed;
    state.push_user_message(ConsoleMessage::info(format!(
        "Recorded execution profile {} for the exact owned netlist source.",
        profile.id()
    )));
    true
}

pub(crate) fn cancel_staged_netlist_import(state: &mut AppState) {
    if state.ui.netlist.import_review.take().is_some() {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn import_netlist_with_mode(state: &mut AppState, mode: NetlistImportMode) -> bool {
    if let Some(reason) = netlist_import_start_block_reason(state, mode) {
        state.push_user_message(ConsoleMessage::error(format!(
            "{} is unavailable: {reason}",
            mode.dialog_title()
        )));
        return false;
    }
    let transaction =
        match crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(state) {
            Ok(transaction) => transaction,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "{} is unavailable: {error}",
                    mode.dialog_title()
                )));
                return false;
            }
        };
    let loaded = show_open_netlist_dialog(mode).and_then(|path| {
        std::fs::read(&path)
            .map(|bytes| (path, bytes))
            .map_err(|error| error.to_string())
    });
    match loaded {
        Ok((path, bytes)) => {
            if let Err(error) =
                crate::workbench::lifecycle::project_lifecycle::validate_project_replacement(
                    state,
                    transaction,
                )
            {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
                state.push_user_message(ConsoleMessage::error(format!(
                    "{} was cancelled because the project changed: {error}",
                    mode.dialog_title()
                )));
                return false;
            }
            let display_name = path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string());
            stage_netlist_import(state, transaction, mode, bytes, Some(path), display_name)
        }
        Err(error) => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
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
pub(crate) fn open_netlist_project(state: &mut AppState) -> bool {
    let started = import_netlist_with_mode(state, NetlistImportMode::OpenProject);
    route_started_netlist_import(state, started)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn import_netlist(state: &mut AppState) -> bool {
    let started = import_netlist_with_mode(state, NetlistImportMode::ImportIntoProject);
    route_started_netlist_import(state, started)
}

#[cfg(not(target_arch = "wasm32"))]
fn show_open_netlist_dialog(mode: NetlistImportMode) -> Result<std::path::PathBuf, String> {
    rfd::FileDialog::new()
        .add_filter(NETLIST_FILTER.0, NETLIST_FILTER.1)
        .add_filter("RSpice Netlist Bundle", &["zip"])
        .add_filter("All Files", &["*"])
        .set_title(mode.dialog_title())
        .pick_file()
        .ok_or_else(|| "cancelled".to_string())
}

/// Reacquire source/permission authority for one exact retained dependency.
/// Native selection commits synchronously; browser selection is completed by
/// `poll_browser_dependency_relink` and remains guarded by both the global
/// picker token and the root document revision captured before the picker.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn request_dependency_relink(state: &mut AppState, logical_identity: &str) -> bool {
    if let Err(error) = crate::workbench::documents::netlist_document::begin_dependency_relink(
        state,
        logical_identity,
    ) {
        state.push_user_message(ConsoleMessage::error(error));
        return false;
    }
    let picked = rfd::FileDialog::new()
        .add_filter(NETLIST_FILTER.0, NETLIST_FILTER.1)
        .add_filter("SPICE Include", &["inc", "lib", "mdl"])
        .add_filter("All Files", &["*"])
        .set_title("Relink SPICE Dependency")
        .pick_file();
    let Some(path) = picked else {
        crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
        return false;
    };
    let bytes = match std::fs::metadata(&path)
        .map_err(|error| error.to_string())
        .and_then(|metadata| {
            if metadata.len() > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
                Err(format!(
                    "Selected dependency exceeds the supported {}-byte project file limit.",
                    crate::io::project_io::MAX_PROJECT_FILE_BYTES
                ))
            } else {
                std::fs::read(&path).map_err(|error| error.to_string())
            }
        }) {
        Ok(bytes) => bytes,
        Err(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE dependency relink failed: {error}"
            )));
            return false;
        }
    };
    let source = match rspice_core::netlist::decode_source_bytes(&bytes) {
        Ok(source) => source,
        Err(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE dependency relink failed: selected source could not be decoded: {error}"
            )));
            return false;
        }
    };
    let display_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    match crate::workbench::documents::netlist_document::commit_dependency_relink(
        state,
        source,
        display_name,
        Some(path.display().to_string()),
    ) {
        Ok(()) => {
            let message = state
                .ui
                .messages()
                .text(crate::workbench::MessageId::NetlistRelinkSucceeded);
            state.push_user_message(ConsoleMessage::info(message));
            true
        }
        Err(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE dependency relink failed: {error}"
            )));
            false
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn import_netlist_with_mode(state: &mut AppState, mode: NetlistImportMode) -> bool {
    if let Some(reason) = netlist_import_start_block_reason(state, mode) {
        state.push_user_message(ConsoleMessage::error(format!(
            "{} is unavailable: {reason}",
            mode.dialog_title()
        )));
        return false;
    }
    match start_browser_netlist_import(state, mode) {
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
pub(crate) fn open_netlist_project(state: &mut AppState) -> bool {
    let started = import_netlist_with_mode(state, NetlistImportMode::OpenProject);
    route_started_netlist_import(state, started)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn import_netlist(state: &mut AppState) -> bool {
    let started = import_netlist_with_mode(state, NetlistImportMode::ImportIntoProject);
    route_started_netlist_import(state, started)
}

/// Browser picker completions are consumed by the Netlist surface. Route a
/// successfully started open/import transaction to that owning page before
/// the asynchronous file read can complete. Native uses the same transition
/// so the staged review always opens in a deterministic workspace.
fn route_started_netlist_import(state: &mut AppState, started: bool) -> bool {
    if started {
        state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
        state.ui.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
    }
    started
}

#[cfg(target_arch = "wasm32")]
enum BrowserNetlistImportResult {
    Loaded(crate::workbench::browser::file_import::PickedTextFile),
    Failed(String),
    Cancelled,
}

#[cfg(target_arch = "wasm32")]
enum BrowserDependencyRelinkResult {
    Loaded(crate::workbench::browser::file_import::PickedTextFile),
    Failed(String),
    Cancelled,
}

#[cfg(target_arch = "wasm32")]
struct BrowserDependencyRelinkCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    result: BrowserDependencyRelinkResult,
}

#[cfg(target_arch = "wasm32")]
struct BrowserNetlistImportCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    transaction: crate::workbench::lifecycle::project_lifecycle::TransactionId,
    mode: NetlistImportMode,
    result: BrowserNetlistImportResult,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_NETLIST_IMPORT_RESULT: std::cell::RefCell<Option<BrowserNetlistImportCompletion>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_DEPENDENCY_RELINK_RESULT: std::cell::RefCell<Option<BrowserDependencyRelinkCompletion>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn request_dependency_relink(state: &mut AppState, logical_identity: &str) -> bool {
    if let Err(error) = crate::workbench::documents::netlist_document::begin_dependency_relink(
        state,
        logical_identity,
    ) {
        state.push_user_message(ConsoleMessage::error(error));
        return false;
    }
    let token = match crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::Netlist,
    ) {
        Ok(token) => token,
        Err(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
            state.push_user_message(ConsoleMessage::error(error));
            return false;
        }
    };
    crate::workbench::browser::file_import::pick_text_file(
        "SPICE Dependency",
        &["cir", "sp", "spice", "net", "ckt", "inc", "lib", "mdl"],
        move |result| {
            let result = match result {
                Ok(Some(file)) => BrowserDependencyRelinkResult::Loaded(file),
                Ok(None) => BrowserDependencyRelinkResult::Cancelled,
                Err(error) => BrowserDependencyRelinkResult::Failed(error),
            };
            BROWSER_DEPENDENCY_RELINK_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(BrowserDependencyRelinkCompletion { token, result });
            });
        },
    );
    true
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_dependency_relink(state: &mut AppState) -> bool {
    let Some(completion) = BROWSER_DEPENDENCY_RELINK_RESULT.with(|slot| slot.borrow_mut().take())
    else {
        return false;
    };
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        return false;
    }
    match completion.result {
        BrowserDependencyRelinkResult::Loaded(file) => {
            let bytes = file
                .original_bytes
                .unwrap_or_else(|| file.contents.into_bytes());
            if bytes.starts_with(b"PK\x03\x04") {
                crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
                state.push_user_message(ConsoleMessage::error(
                    "A bundle cannot relink one dependency. Select the exact SPICE member source.",
                ));
                return false;
            }
            let source = match rspice_core::netlist::decode_source_bytes(&bytes) {
                Ok(source) => source,
                Err(error) => {
                    crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
                    state.push_user_message(ConsoleMessage::error(format!(
                        "SPICE dependency relink failed: selected source could not be decoded: {error}"
                    )));
                    return false;
                }
            };
            match crate::workbench::documents::netlist_document::commit_dependency_relink(
                state, source, file.name, None,
            ) {
                Ok(()) => {
                    let message = state
                        .ui
                        .messages()
                        .text(crate::workbench::MessageId::NetlistRelinkSucceeded);
                    state.push_user_message(ConsoleMessage::info(message));
                    true
                }
                Err(error) => {
                    crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
                    state.push_user_message(ConsoleMessage::error(format!(
                        "SPICE dependency relink failed: {error}"
                    )));
                    false
                }
            }
        }
        BrowserDependencyRelinkResult::Failed(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE dependency relink failed: {error}"
            )));
            false
        }
        BrowserDependencyRelinkResult::Cancelled => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
            false
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn start_browser_netlist_import(
    state: &mut AppState,
    mode: NetlistImportMode,
) -> Result<(), String> {
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(state)
            .map_err(|error| error.to_string())?;
    let token = crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::Netlist,
    )
    .inspect_err(|_| {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
    })?;

    crate::workbench::browser::file_import::pick_text_file(
        "SPICE Deck or RSpice Netlist Bundle",
        &["cir", "sp", "spice", "net", "ckt", "zip"],
        move |result| {
            let event = match result {
                Ok(Some(file)) => BrowserNetlistImportResult::Loaded(file),
                Ok(None) => BrowserNetlistImportResult::Cancelled,
                Err(error) => BrowserNetlistImportResult::Failed(error),
            };
            BROWSER_NETLIST_IMPORT_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(BrowserNetlistImportCompletion {
                    token,
                    transaction,
                    mode,
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
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
            state,
            completion.transaction,
        );
        return false;
    }
    if let Err(error) = crate::workbench::lifecycle::project_lifecycle::validate_project_replacement(
        state,
        completion.transaction,
    ) {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
            state,
            completion.transaction,
        );
        state.push_user_message(ConsoleMessage::error(format!(
            "{} was cancelled because the project changed: {error}",
            completion.mode.dialog_title()
        )));
        return false;
    }
    match completion.result {
        BrowserNetlistImportResult::Loaded(file) => stage_netlist_import(
            state,
            completion.transaction,
            completion.mode,
            file.original_bytes
                .unwrap_or_else(|| file.contents.into_bytes()),
            None,
            file.name,
        ),
        BrowserNetlistImportResult::Failed(error) => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                state,
                completion.transaction,
            );
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE deck import failed: {error}"
            )));
            false
        }
        BrowserNetlistImportResult::Cancelled => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                state,
                completion.transaction,
            );
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GENERATED_BASE: &str = "* generated\n.option reltol=1e-3\n.param gain=10\n.include \"models/a.lib\"\n.lib \"models/b.lib\" TT\n+ section=fast\nV1 out 0 1\nR1 out 0 1k\n.op\n.measure op vout FIND V(out)\n.save V(out)\n.end\n";

    #[test]
    fn started_netlist_import_routes_every_calling_page_to_its_completion_owner() {
        let mut state = AppState::default();
        state.workbench.workspace = crate::workbench::state::Workspace::Models;
        state.ui.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation;

        assert!(!route_started_netlist_import(&mut state, false));
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Models
        );
        assert_eq!(
            state.ui.code_workspace.page,
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation
        );

        assert!(route_started_netlist_import(&mut state, true));
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Netlist
        );
        assert_eq!(
            state.ui.code_workspace.page,
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
        );
    }

    #[test]
    fn import_decoder_and_encoder_preserve_supported_file_boundaries() {
        let source = "* caf\u{00e9}\r\nV1 out 0 1\r\n.op\r\n.end\r\n";
        let utf16 = crate::state::NetlistTextEncoding::Utf16LeBom
            .encode(source)
            .expect("UTF-16 encoding succeeds");
        let (decoded, encoding) = decode_import_bytes(&utf16).expect("UTF-16 import decodes");

        assert_eq!(decoded, source);
        assert_eq!(encoding, crate::state::NetlistTextEncoding::Utf16LeBom);
        assert_eq!(
            crate::state::NetlistLineEnding::detect(&decoded),
            crate::state::NetlistLineEnding::Crlf
        );
        assert_eq!(encoding.encode(&decoded).unwrap(), utf16);

        let latin1 = crate::state::NetlistTextEncoding::Latin1
            .encode(source)
            .expect("Latin-1 fixture is representable");
        let (decoded, encoding) = decode_import_bytes(&latin1).expect("Latin-1 import decodes");
        assert_eq!(encoding, crate::state::NetlistTextEncoding::Latin1);
        assert_eq!(decoded, source);
        assert_eq!(encoding.encode(&decoded).unwrap(), latin1);
        assert!(encoding.encode("\u{20ac}").is_err());
    }

    fn generated_bundle_fixture() -> Vec<u8> {
        use crate::product::{ContentDigest, ObjectRevision};
        use crate::state::{
            DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GenerationInput,
            SourceLocator,
        };

        let root = "bundle fixture\n.include \"models/base.lib\"\nV1 out 0 1\n.op\n.end\n";
        let base = SourceLocator::try_new("models/base.lib", "base.lib").unwrap();
        let child = SourceLocator::try_new("models/devices/core.lib", "core.lib").unwrap();
        let dependencies = vec![
            DependencyMetadata::unresolved_direct_to(0, "models/base.lib", base.clone())
                .unwrap()
                .with_authority(crate::state::DependencySourceAuthority::Vendor)
                .resolve_utf8(b".include \"devices/core.lib\"\n.model base nmos level=1\n".to_vec())
                .unwrap(),
            DependencyMetadata::unresolved_transitive_to(base, 0, "devices/core.lib", child)
                .unwrap()
                .with_authority(crate::state::DependencySourceAuthority::TechnologyPackage)
                .resolve_utf8(b".model core nmos level=1 vto=0.45\n".to_vec())
                .unwrap(),
        ];
        let artifact = GeneratedArtifact::try_from_utf8(
            GeneratedProvenance::try_new(
                "netlist-bundle-import-test",
                GenerationInput::new(
                    ObjectRevision::INITIAL,
                    ContentDigest::from_bytes([0x5a; 32]),
                ),
            )
            .unwrap(),
            root.as_bytes().to_vec(),
            dependencies,
            Vec::new(),
        )
        .unwrap();
        crate::workbench::menu_bar::build_generated_bundle(
            &artifact,
            crate::io::NetlistFormat::Spice,
            true,
        )
        .unwrap()
    }

    #[test]
    fn authenticated_generated_bundle_stages_and_commits_retained_closure() {
        let bytes = generated_bundle_fixture();
        let parsed = parse_generated_netlist_bundle(&bytes).expect("authenticated bundle");
        assert_eq!(parsed.dependencies.len(), 2);
        assert_eq!(
            parsed.dependencies[0].authority(),
            crate::state::DependencySourceAuthority::Vendor
        );
        assert_eq!(
            parsed.dependencies[1].authority(),
            crate::state::DependencySourceAuthority::TechnologyPackage
        );
        assert!(parsed.expanded_source.contains(".model base nmos level=1"));
        assert!(
            parsed
                .expanded_source
                .contains(".model core nmos level=1 vto=0.45")
        );
        assert!(
            !parsed
                .expanded_source
                .to_ascii_lowercase()
                .contains(".include")
        );

        let mut state = AppState::default();
        let transaction =
            crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
                .expect("replacement transaction starts");
        assert!(stage_netlist_import(
            &mut state,
            transaction,
            NetlistImportMode::ImportIntoProject,
            bytes,
            None,
            "portable-run.zip".to_owned(),
        ));
        let review = state.ui.netlist.import_review.as_ref().unwrap();
        assert!(review.archive_import);
        assert_eq!(review.display_name, "portable-run.spice");
        assert!(review.source_path.is_none());
        assert_eq!(review.dependencies.len(), 2);

        assert!(commit_staged_netlist_import(&mut state));
        assert!(state.workspace.netlist_source_path.is_none());
        assert_eq!(
            state
                .workspace
                .netlist_descriptor
                .as_ref()
                .map(|descriptor| descriptor.artifact_name.as_str()),
            Some("portable-run.spice")
        );
        let document = state.workspace.netlist_document.as_ref().unwrap();
        assert!(document.dependency_graph_is_sealed());
        assert_eq!(document.dependencies().len(), 2);
    }

    #[test]
    fn generated_bundle_import_rejects_tampered_member_bytes() {
        let mut bytes = generated_bundle_fixture();
        let needle = b"vto=0.45";
        let offset = bytes
            .windows(needle.len())
            .position(|window| window == needle)
            .expect("fixture contains retained model bytes");
        bytes[offset] ^= 0x01;

        let error = parse_generated_netlist_bundle(&bytes).expect_err("tamper must fail closed");
        assert!(error.contains("CRC-32 verification"), "{error}");
    }

    #[test]
    fn three_way_merge_combines_independent_lines_and_marks_overlaps() {
        let base = ".param a=1\n.param b=2\n.op\n";
        let local = ".param a=10\n.param b=2\n.op\n";
        let external = ".param a=1\n.param b=20\n.op\n";
        let (merged, conflicts) = three_way_merge_source(Some(base), local, external);
        assert_eq!(conflicts, 0);
        assert_eq!(merged, ".param a=10\n.param b=20\n.op\n");

        let external = ".param a=100\n.param b=2\n.op\n";
        let (merged, conflicts) = three_way_merge_source(Some(base), local, external);
        assert_eq!(conflicts, 1);
        assert!(merged.contains("<<<<<<< RSPICE LOCAL"));
        assert!(merged.contains(".param a=10"));
        assert!(merged.contains(".param a=100"));

        let base = "a\nb\nc\n";
        let local = "a\ninserted locally\nb\nc\n";
        let external = "a\nb\nchanged externally\n";
        let (merged, conflicts) = three_way_merge_source(Some(base), local, external);
        assert_eq!(conflicts, 0);
        assert_eq!(merged, "a\ninserted locally\nb\nchanged externally\n");
    }

    #[test]
    fn staged_import_is_cancel_safe_and_commits_only_the_reviewed_snapshot() {
        let mut state = AppState::default();
        let original_project = state.workspace.project.id();
        let original_source = state.workspace.netlist_source.clone();
        let transaction =
            crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
                .expect("replacement transaction starts");
        let source = b"* staged\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();

        assert!(stage_netlist_import(
            &mut state,
            transaction,
            NetlistImportMode::ImportIntoProject,
            source.clone(),
            None,
            "staged.cir".to_owned(),
        ));
        assert_eq!(state.workspace.project.id(), original_project);
        assert_eq!(state.workspace.netlist_source, original_source);
        assert!(state.ui.netlist.import_review.is_some());

        assert!(commit_staged_netlist_import(&mut state));
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            std::str::from_utf8(&source).ok()
        );
        assert!(state.ui.netlist.import_review.is_none());
        assert_eq!(
            state
                .workspace
                .netlist_descriptor
                .as_ref()
                .and_then(|descriptor| descriptor.imported_dialect),
            Some(crate::state::NetlistSourceDialect::RSpice)
        );
    }

    #[test]
    fn owned_netlist_history_compare_and_restore_are_persisted_and_monotonic() {
        let original = "* baseline\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
        let modified = "* modified\nV1 out 0 2\nR1 out 0 2k\n.op\n.end\n";
        let mut state = AppState::default();
        assert!(apply_imported_netlist(
            &mut state,
            original.to_owned(),
            None,
            "history.cir",
        ));
        let baseline_revision = state
            .workspace
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .revision_history[0]
            .document_revision;
        assert!(
            crate::workbench::documents::netlist_document::replace_owned_source(
                &mut state,
                modified.to_owned(),
            )
        );
        let modified_revision = state
            .workspace
            .netlist_document
            .as_ref()
            .unwrap()
            .revision()
            .get();
        assert!(modified_revision > baseline_revision);

        crate::workbench::documents::netlist_document::compare_owned_revision(&mut state, 0)
            .expect("comparison opens");
        assert!(state.simulation.netlist_content.contains("-V1 out 0 1"));
        crate::workbench::documents::netlist_document::close_revision_comparison(&mut state);
        assert_eq!(state.simulation.netlist_content, modified);

        crate::workbench::documents::netlist_document::restore_owned_revision(&mut state, 0)
            .expect("history restore commits");
        assert_eq!(state.workspace.netlist_source.as_deref(), Some(original));
        let restored_revision = state
            .workspace
            .netlist_document
            .as_ref()
            .unwrap()
            .revision()
            .get();
        assert!(restored_revision > modified_revision);
        let history = &state
            .workspace
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .revision_history;
        assert_eq!(history.len(), 3);
        assert_eq!(history[1].source, modified);
        assert_eq!(history[2].source, original);
        state
            .workspace
            .validate_simulation_configuration()
            .expect("restored workspace validates");

        let persisted = serde_json::to_vec(&state.workspace).expect("serialize workspace");
        let restored: crate::state::ProjectWorkspace =
            serde_json::from_slice(&persisted).expect("deserialize workspace");
        restored
            .validate_simulation_configuration()
            .expect("persisted history validates");
        assert_eq!(
            restored
                .netlist_descriptor
                .as_ref()
                .unwrap()
                .revision_history,
            *history
        );
    }

    #[test]
    fn detected_foreign_dialect_requires_explicit_acceptance() {
        let mut state = AppState::default();
        let transaction =
            crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
                .expect("replacement transaction starts");
        let source =
            b"* hspice deck\n.option post=2\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();
        let retained_source = String::from_utf8(source.clone()).unwrap();

        assert!(stage_netlist_import(
            &mut state,
            transaction,
            NetlistImportMode::ImportIntoProject,
            source,
            None,
            "foreign.sp".to_owned(),
        ));
        let review = state.ui.netlist.import_review.as_ref().unwrap();
        assert_eq!(
            review.detected_dialect,
            crate::state::NetlistSourceDialect::Hspice
        );
        assert!(!review.can_commit());
        assert_eq!(review.blocking_issue_count(), 0);

        let review = state.ui.netlist.import_review.as_mut().unwrap();
        review.compatibility_accepted = true;
        assert!(review.dialect_qualification().is_ok());
        assert!(review.can_commit());
        review.selected_dialect = crate::state::NetlistSourceDialect::RSpice;
        assert!(review.dialect_qualification().is_err());
        assert!(!review.can_commit());
        review.selected_dialect = crate::state::NetlistSourceDialect::Hspice;
        assert!(commit_staged_netlist_import(&mut state));
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some(retained_source.as_str())
        );
        let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
        assert_eq!(
            descriptor.imported_dialect,
            Some(crate::state::NetlistSourceDialect::Hspice)
        );
        assert_eq!(
            descriptor.execution_profile,
            Some(crate::state::NetlistExecutionProfile::HspiceDeclarativeV1)
        );
    }

    #[test]
    fn only_source_authenticated_import_dialects_can_commit_after_acceptance() {
        use crate::state::NetlistSourceDialect;

        let mut state = AppState::default();
        let transaction =
            crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
                .expect("replacement transaction starts");
        let source = b"* standard subset\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();
        assert!(stage_netlist_import(
            &mut state,
            transaction,
            NetlistImportMode::ImportIntoProject,
            source,
            None,
            "standard-subset.cir".to_owned(),
        ));

        {
            let review = state.ui.netlist.import_review.as_mut().unwrap();
            review.selected_dialect = NetlistSourceDialect::Spice3Ngspice;
            review.compatibility_accepted = true;
            assert!(review.dialect_qualification().is_ok());
            assert!(review.can_commit());
        }

        let review = state.ui.netlist.import_review.as_mut().unwrap();
        review.selected_dialect = NetlistSourceDialect::Hspice;
        review.compatibility_accepted = true;
        assert!(review.dialect_qualification().is_err());
        assert!(!review.can_commit());

        let review = state.ui.netlist.import_review.as_mut().unwrap();
        review.selected_dialect = NetlistSourceDialect::Spectre;
        review.compatibility_accepted = true;
        assert!(review.dialect_qualification().is_err());
        assert!(!review.can_commit());

        let review = state.ui.netlist.import_review.as_mut().unwrap();
        review.selected_dialect = NetlistSourceDialect::Ads;
        review.compatibility_accepted = true;
        assert!(review.dialect_qualification().is_err());
        assert!(!review.can_commit());

        for dialect in [NetlistSourceDialect::Unknown] {
            let review = state.ui.netlist.import_review.as_mut().unwrap();
            review.selected_dialect = dialect;
            review.compatibility_accepted = true;
            assert!(review.dialect_qualification().is_err(), "{dialect:?}");
            assert!(!review.can_commit(), "{dialect:?}");
        }

        let review = state.ui.netlist.import_review.as_mut().unwrap();
        review.selected_dialect = NetlistSourceDialect::Pspice;
        review.compatibility_accepted = true;
        assert!(review.dialect_qualification().is_err());
        assert!(!review.can_commit());

        let review = state.ui.netlist.import_review.as_mut().unwrap();
        review.selected_dialect = NetlistSourceDialect::RSpice;
        review.compatibility_accepted = false;
        assert!(review.dialect_qualification().is_ok());
        assert!(review.can_commit());
        cancel_staged_netlist_import(&mut state);
    }

    #[test]
    fn spectre_spice_interoperability_import_preserves_source_and_profile() {
        let mut state = AppState::default();
        let transaction =
            crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
                .expect("replacement transaction starts");
        let source = b"simulator lang=spice\n* SPICE interoperability deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();
        let retained_source = String::from_utf8(source.clone()).unwrap();

        assert!(stage_netlist_import(
            &mut state,
            transaction,
            NetlistImportMode::ImportIntoProject,
            source,
            None,
            "spectre-spice.scs".to_owned(),
        ));
        let review = state.ui.netlist.import_review.as_mut().unwrap();
        assert_eq!(
            review.detected_dialect,
            crate::state::NetlistSourceDialect::Spectre
        );
        assert_eq!(review.blocking_issue_count(), 0);
        review.compatibility_accepted = true;
        assert!(review.can_commit());
        assert!(commit_staged_netlist_import(&mut state));
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some(retained_source.as_str())
        );
        let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
        assert_eq!(
            descriptor.imported_dialect,
            Some(crate::state::NetlistSourceDialect::Spectre)
        );
        assert_eq!(
            descriptor.execution_profile,
            Some(crate::state::NetlistExecutionProfile::SpectreSpiceV1)
        );
    }

    #[test]
    fn ads_spice_export_import_preserves_source_and_profile() {
        let mut state = AppState::default();
        let transaction =
            crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
                .expect("replacement transaction starts");
        let source = b"Options ResourceUsage=yes UseNutmegFormat=no TopDesignName=\"divider\"\n* ADS SPICE export\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();
        let retained_source = String::from_utf8(source.clone()).unwrap();

        assert!(stage_netlist_import(
            &mut state,
            transaction,
            NetlistImportMode::ImportIntoProject,
            source,
            None,
            "ads-spice-export.net".to_owned(),
        ));
        let review = state.ui.netlist.import_review.as_mut().unwrap();
        assert_eq!(
            review.detected_dialect,
            crate::state::NetlistSourceDialect::Ads
        );
        assert_eq!(review.blocking_issue_count(), 0);
        review.compatibility_accepted = true;
        assert!(review.can_commit());
        assert!(commit_staged_netlist_import(&mut state));
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some(retained_source.as_str())
        );
        let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
        assert_eq!(
            descriptor.imported_dialect,
            Some(crate::state::NetlistSourceDialect::Ads)
        );
        assert_eq!(
            descriptor.execution_profile,
            Some(crate::state::NetlistExecutionProfile::AdsSpiceExportV1)
        );
    }

    #[test]
    fn qualified_spice3_ngspice_import_persists_exact_versioned_profile() {
        let mut state = AppState::default();
        let transaction =
            crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
                .expect("replacement transaction starts");
        let source = b"* declarative SPICE3 subset\n.param gain=2\nV1 in 0 1\nB1 out 0 V=log(exp({gain}))\nR1 out 0 1k\n.control\nop\nsave v(out)\n.endc\n.end\n".to_vec();
        assert!(stage_netlist_import(
            &mut state,
            transaction,
            NetlistImportMode::ImportIntoProject,
            source,
            None,
            "declarative-spice3.cir".to_owned(),
        ));
        let review = state.ui.netlist.import_review.as_mut().unwrap();
        assert_eq!(
            review.detected_dialect,
            crate::state::NetlistSourceDialect::Spice3Ngspice
        );
        assert_eq!(review.blocking_issue_count(), 0);
        review.selected_dialect = crate::state::NetlistSourceDialect::Spice3Ngspice;
        review.compatibility_accepted = true;
        assert!(review.can_commit());
        assert!(commit_staged_netlist_import(&mut state));

        let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
        assert_eq!(
            descriptor.imported_dialect,
            Some(crate::state::NetlistSourceDialect::Spice3Ngspice)
        );
        assert!(descriptor.compatibility_reviewed);
        assert_eq!(
            descriptor.execution_profile,
            Some(crate::state::NetlistExecutionProfile::Spice3NgspiceV2)
        );
        let persisted = serde_json::to_vec(&state.workspace).unwrap();
        let restored: crate::state::ProjectWorkspace = serde_json::from_slice(&persisted).unwrap();
        assert_eq!(
            restored
                .netlist_descriptor
                .as_ref()
                .and_then(|descriptor| descriptor.execution_profile),
            Some(crate::state::NetlistExecutionProfile::Spice3NgspiceV2)
        );
    }

    #[test]
    fn qualified_pspice_import_persists_exact_versioned_profile() {
        let mut state = AppState::default();
        let transaction =
            crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
                .expect("replacement transaction starts");
        let source = b"* PSpice declarative subset\nV1 in 0 AC 1\nE1 out 0 CHEBYSHEV {V(in)} = BP (800Hz 1.2kHz 2kHz 3kHz) .1dB 50dB\nR1 out 0 1k\n.probe V(out)\n.ac dec 10 100 10k\n.end\n"
            .to_vec();
        assert!(stage_netlist_import(
            &mut state,
            transaction,
            NetlistImportMode::ImportIntoProject,
            source,
            None,
            "declarative-pspice.cir".to_owned(),
        ));
        let review = state.ui.netlist.import_review.as_mut().unwrap();
        assert_eq!(
            review.detected_dialect,
            crate::state::NetlistSourceDialect::Pspice
        );
        assert!(
            review
                .transformations
                .iter()
                .any(|entry| entry.contains("CHEBYSHEV"))
        );
        review.compatibility_accepted = true;
        assert!(
            review.can_commit(),
            "issues={:?}; qualification={:?}",
            review.issues,
            review.dialect_qualification()
        );
        assert!(commit_staged_netlist_import(&mut state));

        let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
        assert_eq!(
            descriptor.imported_dialect,
            Some(crate::state::NetlistSourceDialect::Pspice)
        );
        assert!(descriptor.compatibility_reviewed);
        assert_eq!(
            descriptor.execution_profile,
            Some(crate::state::NetlistExecutionProfile::PspiceDeclarativeV2)
        );
        let persisted = serde_json::to_vec(&state.workspace).unwrap();
        let restored: crate::state::ProjectWorkspace = serde_json::from_slice(&persisted).unwrap();
        assert_eq!(
            restored
                .netlist_descriptor
                .as_ref()
                .and_then(|descriptor| descriptor.execution_profile),
            Some(crate::state::NetlistExecutionProfile::PspiceDeclarativeV2)
        );
    }

    fn quarantined_owned_ngspice_state() -> AppState {
        let source = "* retained declarative source\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
        let mut state = AppState::default();
        state.project_lifecycle.project_open = true;
        assert!(apply_imported_netlist(
            &mut state,
            source.to_owned(),
            None,
            "legacy-ngspice.cir",
        ));
        let descriptor = state.workspace.netlist_descriptor.as_mut().unwrap();
        descriptor.imported_dialect = Some(crate::state::NetlistSourceDialect::Spice3Ngspice);
        descriptor.compatibility_reviewed = false;
        descriptor.execution_profile = None;
        state.workspace.netlist_source_dirty = false;
        state.workspace.project_metadata_dirty = false;
        state
            .workspace
            .validate_simulation_configuration()
            .expect("quarantined source remains a valid, non-executable project");
        state
    }

    #[test]
    fn quarantined_owned_source_requalifies_without_external_file_or_source_replacement() {
        let mut state = quarantined_owned_ngspice_state();
        let source = state.workspace.netlist_source.clone();
        let history = state
            .workspace
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .revision_history
            .clone();

        assert!(begin_owned_netlist_profile_review(&mut state));
        let review = state.ui.netlist.import_review.as_mut().unwrap();
        assert_eq!(
            review.operation,
            crate::workbench::documents::netlist_document::NetlistImportOperation::RequalifyOwnedSource
        );
        assert!(review.selected_file_path.is_none());
        assert_eq!(review.source, source.as_deref().unwrap());
        review.compatibility_accepted = true;
        assert!(review.can_commit());
        assert!(commit_staged_netlist_import(&mut state));

        let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
        assert_eq!(
            descriptor.execution_profile,
            Some(crate::state::NetlistExecutionProfile::Spice3NgspiceV2)
        );
        assert!(descriptor.compatibility_reviewed);
        assert!(!descriptor.execution_profile_review_required());
        assert_eq!(descriptor.revision_history, history);
        assert_eq!(state.workspace.netlist_source, source);
        assert!(state.workspace.project_metadata_dirty);
        assert!(state.ui.netlist.import_review.is_none());
    }

    #[test]
    fn owned_profile_review_rejects_source_revision_change_transactionally() {
        let mut state = quarantined_owned_ngspice_state();
        assert!(begin_owned_netlist_profile_review(&mut state));
        state
            .ui
            .netlist
            .import_review
            .as_mut()
            .unwrap()
            .compatibility_accepted = true;
        assert!(
            crate::workbench::documents::netlist_document::replace_owned_source(
                &mut state,
                "* changed after review\nV1 out 0 2\nR1 out 0 1k\n.op\n.end\n".to_owned(),
            )
        );

        assert!(!commit_staged_netlist_import(&mut state));
        let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
        assert!(descriptor.execution_profile.is_none());
        assert!(!descriptor.compatibility_reviewed);
        assert!(
            state
                .ui
                .netlist
                .import_review
                .as_ref()
                .and_then(|review| review.error.as_deref())
                .is_some_and(|error| error.contains("project changed"))
        );
    }

    #[test]
    fn imperative_ngspice_control_command_cannot_use_declarative_profile() {
        let mut state = AppState::default();
        let transaction =
            crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
                .expect("replacement transaction starts");
        let source =
            b"* imperative ngspice\nV1 out 0 1\n.control\nop\nwrdata out.txt v(out)\n.endc\n.end\n"
                .to_vec();
        assert!(stage_netlist_import(
            &mut state,
            transaction,
            NetlistImportMode::ImportIntoProject,
            source,
            None,
            "imperative-ngspice.cir".to_owned(),
        ));
        let review = state.ui.netlist.import_review.as_mut().unwrap();
        review.selected_dialect = crate::state::NetlistSourceDialect::Spice3Ngspice;
        review.compatibility_accepted = true;
        let error = review.dialect_qualification().unwrap_err();
        assert!(error.contains("spice3-ngspice/2"));
        assert!(error.contains("wrdata"));
        assert!(!review.can_commit());
        cancel_staged_netlist_import(&mut state);
    }

    #[test]
    fn semantic_loss_parser_diagnostics_block_import() {
        let mut state = AppState::default();
        let transaction =
            crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
                .expect("replacement transaction starts");
        let source = b"* ignored option is unsafe\n.options definitely_unknown=7\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();
        assert!(stage_netlist_import(
            &mut state,
            transaction,
            NetlistImportMode::ImportIntoProject,
            source,
            None,
            "unknown-option.cir".to_owned(),
        ));
        let review = state.ui.netlist.import_review.as_ref().unwrap();
        assert!(review.blocking_issue_count() > 0);
        assert!(review.issues.iter().any(|issue| {
            issue.severity
                == crate::workbench::documents::netlist_document::NetlistImportIssueSeverity::Blocking
                && issue.message.contains("unknown-option")
        }));
        assert!(!review.can_commit());
        cancel_staged_netlist_import(&mut state);
    }

    fn state_with_owned_strategy(
        generated_source: &str,
        authored_source: &str,
        strategy: crate::state::OwnedNetlistEditStrategy,
    ) -> AppState {
        use crate::product::ObjectRevision;
        use crate::state::{
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
            source_encoding: crate::state::NetlistTextEncoding::Utf8,
            source_line_ending: crate::state::NetlistLineEnding::detect(authored_source),
            imported_dialect: None,
            compatibility_reviewed: false,
            execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
            external_file_sha256: None,
            save_history: Vec::new(),
            revision_history: Vec::new(),
            owned_includes: Vec::new(),
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
            source_encoding: crate::state::NetlistTextEncoding::Utf8,
            source_line_ending: crate::state::NetlistLineEnding::Lf,
            imported_dialect: None,
            compatibility_reviewed: false,
            execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
            external_file_sha256: None,
            save_history: Vec::new(),
            revision_history: Vec::new(),
            owned_includes: Vec::new(),
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
    fn imported_netlist_becomes_dirty_manual_source_without_deleting_retained_runs() {
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
        assert!(state.simulation.has_results());
        assert_eq!(state.simulation.runs.len(), 1);
        assert!(state.recent_files.is_empty());
    }

    #[test]
    fn opening_a_netlist_commits_an_independent_netlist_first_project() {
        let mut state = AppState::default();
        let original_project_id = state.workspace.project.id();
        state.simulation.start_run();
        state.workspace.netlist_source = Some("old\n.op\n.end\n".to_owned());

        assert!(apply_opened_netlist_project(
            &mut state,
            "new\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned(),
            Some(std::path::PathBuf::from("bias.cir")),
            "bias.cir",
            NetlistImportMetadata {
                encoding: crate::state::NetlistTextEncoding::Utf8,
                line_ending: crate::state::NetlistLineEnding::Lf,
                dialect: crate::state::NetlistSourceDialect::RSpice,
                compatibility_reviewed: false,
                raw_sha256: sha256(b"new\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n"),
            },
            Vec::new(),
        ));

        assert_ne!(state.workspace.project.id(), original_project_id);
        assert_eq!(state.workspace.project.name(), "bias");
        assert!(!state.simulation.has_results());
        assert_eq!(
            state.workspace.netlist_source_path.as_deref(),
            Some(std::path::Path::new("bias.cir"))
        );
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Netlist
        );
    }

    #[test]
    fn importing_a_deck_refuses_read_only_projects_without_mutation() {
        let mut state = AppState::default();
        state.workbench.safe_mode.active = true;
        state.workbench.safe_mode.applied = crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..Default::default()
        };
        state.workspace.netlist_source = Some("old\n.op\n.end\n".to_owned());

        assert!(!apply_imported_netlist(
            &mut state,
            "new\n.op\n.end\n".to_owned(),
            None,
            "new.cir",
        ));
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("old\n.op\n.end\n")
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn text_first_import_validates_and_retains_explicit_run_authorization_without_schematic() {
        let mut app = crate::workbench::RSpiceApp::test_instance();
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
            Some(crate::workbench::documents::netlist_document::source_content_digest(source))
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

        let mut app = crate::workbench::RSpiceApp::test_instance();
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

        let mut app = crate::workbench::RSpiceApp::test_instance();
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
        let mut app = crate::workbench::RSpiceApp::test_instance();
        app.state = state_with_owned_strategy(
            generated,
            parameter_source,
            crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
        );
        app.state.workspace.netlist_source_path = Some(root.clone());
        app.state.ui.netlist.owned_document = app.state.workspace.netlist_document.clone();
        app.state.ui.netlist.active_document =
            crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource;
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
        let mut app = crate::workbench::RSpiceApp::test_instance();
        app.state = state_with_owned_strategy(
            generated,
            include_source,
            crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride,
        );
        app.state.workspace.netlist_source_path = Some(root);
        app.state.ui.netlist.owned_document = app.state.workspace.netlist_document.clone();
        app.state.ui.netlist.active_document =
            crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource;
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

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn ordinary_source_save_refuses_to_overwrite_external_changes() {
        let path = std::env::temp_dir().join(format!(
            "rspice-netlist-external-conflict-{}.cir",
            uuid::Uuid::new_v4()
        ));
        let authored = "* owned\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
        let external = "* changed by another editor\nV1 out 0 2\n.op\n.end\n";
        std::fs::write(&path, authored).expect("write imported source");
        let mut app = crate::workbench::RSpiceApp::test_instance();
        assert!(apply_imported_netlist(
            &mut app.state,
            authored.to_owned(),
            Some(path.clone()),
            "owned.cir",
        ));
        assert!(validate_visible_netlist_source(&mut app));
        std::fs::write(&path, external).expect("publish external edit");

        assert!(!save_owned_netlist_source(
            &mut app.state,
            &app.simulation_controller,
            &crate::workbench::workflows::export_workflow::NativeExportWorkflowIo,
            false,
            "Attempt conflicting save",
        ));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), external);
        assert!(app.state.ui.netlist.external_change.is_some());
        app.state
            .ui
            .netlist
            .external_change
            .as_mut()
            .unwrap()
            .resolution = crate::workbench::documents::netlist_document::NetlistExternalChangeResolution::ReloadExternal;
        apply_staged_external_netlist_change(&mut app.state)
            .expect("explicit external reload succeeds");
        assert_eq!(
            app.state.workspace.netlist_source.as_deref(),
            Some(external)
        );
        assert!(app.state.ui.netlist.external_change.is_none());
        assert_eq!(
            app.state.ui.netlist.externally_saved_content_digest,
            Some(crate::state::content_digest(external))
        );
        assert!(
            app.state
                .workspace
                .netlist_descriptor
                .as_ref()
                .unwrap()
                .revision_history
                .len()
                >= 2
        );
        std::fs::remove_file(path).expect("remove conflict fixture");
    }
}
