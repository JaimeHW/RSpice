//! Netlist workspace workflows.
//!
//! Validating the visible deck and composing the source a run executes.
//! Validation is pinned to an exact content digest and project revision, so
//! an edit after validation invalidates it rather than being carried along.

use crate::diagnostics::ConsoleMessage;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

mod bundle;
mod compose;
mod external_change;
mod import;
mod platform;
mod save;
mod staging;
#[cfg(test)]
mod tests;

pub(crate) use compose::compose_owned_netlist_execution_source;
pub(crate) use external_change::apply_staged_external_netlist_change;
#[cfg(test)]
pub(crate) use import::apply_imported_netlist;
pub(crate) use platform::{import_netlist, open_netlist_project, request_dependency_relink};
pub(crate) use save::save_owned_netlist_source;
pub(crate) use staging::{
    begin_owned_netlist_profile_review, cancel_staged_netlist_import, commit_staged_netlist_import,
    stage_dropped_netlist_import, stage_dropped_netlist_project,
};

pub const NETLIST_FILTER: (&str, &[&str]) = ("SPICE Deck", &["cir", "sp", "spice", "net", "ckt"]);

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_netlist_workflow(state: &mut AppState) {
    platform::poll_browser_netlist_import(state);
    platform::poll_browser_dependency_relink(state);
}

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

/// Retain where each dependency resolved so the navigator and the Problems
/// pipeline can state it. The engine walked the chain; nothing here re-walks
/// it, so the two can never disagree.
fn record_include_resolutions(
    state: &mut AppState,
    sealed: &[rspice_core::netlist::ResolvedIncludeDependency],
) {
    let resolutions = &mut state.ui.code_workspace.include_resolutions;
    resolutions.clear();
    for dependency in sealed {
        resolutions.insert(
            dependency.requested_path().to_owned(),
            dependency.resolution().clone(),
        );
    }
}

fn acknowledge_canonical_dependencies(
    state: &mut AppState,
    sealed: &[rspice_core::netlist::ResolvedIncludeDependency],
) -> Result<(), String> {
    record_include_resolutions(state, sealed);
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
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff
        | crate::workbench::documents::netlist_document::ActiveNetlistDocument::RunSnapshot => None,
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
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff
        | crate::workbench::documents::netlist_document::ActiveNetlistDocument::RunSnapshot => None,
    };
    let root = source_origin.map(dependency_root_path).transpose()?;
    let root_directory = root
        .as_deref()
        .and_then(std::path::Path::parent)
        .unwrap_or_else(|| std::path::Path::new("."));
    let root_directives = if dependencies_belong_to_generated_base {
        let backing = document
            .generated_artifact()
            .ok_or_else(|| "The override source has no retained generated base.".to_owned())?;
        external_dependency_locators(backing.source())
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
        let backing = document
            .generated_artifact()
            .cloned()
            .ok_or_else(|| "The override source has no retained generated base.".to_owned())?;
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
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff
        | crate::workbench::documents::netlist_document::ActiveNetlistDocument::RunSnapshot => None,
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
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff
        | crate::workbench::documents::netlist_document::ActiveNetlistDocument::RunSnapshot => None,
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
