//! Generated-primary reconciliation.
//!
//! Generation runs into a temporary string: a failed build retains the prior
//! valid artifact and marks it stale rather than publishing partial bytes.
//! The source map and the authenticated Verilog-A dependency closure come out
//! of the same pass, which is what lets a generated line name the schematic
//! object that produced it.

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    GeneratedArtifact, GeneratedProvenance, GeneratedSourceMapEntry, GenerationInput,
    NetlistDocument, NetlistDocumentId,
};
use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
use crate::workbench::{AppState, RSpiceApp};

/// Rebuild the immutable generated primary whenever any authoritative project
/// input changes. Generation happens into a temporary string; a failed build
/// retains the prior valid artifact and marks it stale instead of publishing
/// partial bytes.
pub(super) fn reconcile_documents(app: &mut RSpiceApp) {
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
                if let (Some(previous), Some(current)) = (
                    app.state
                        .ui
                        .netlist
                        .generated_document
                        .as_ref()
                        .and_then(crate::state::NetlistDocument::generated_artifact),
                    generated_document.generated_artifact(),
                ) && previous.content_digest() != current.content_digest()
                {
                    let predecessor = previous.clone();
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
                let Some(generated_artifact) = generated_document.generated_artifact() else {
                    app.state.ui.netlist.generation_error = Some(
                        "Generated document publication returned no generated artifact.".to_owned(),
                    );
                    return;
                };
                app.state.ui.netlist.generated_source = generated_artifact.source().to_owned();
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

pub(super) fn publish_generated_document(
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
        let expected_digest = existing
            .generated_artifact()
            .ok_or_else(|| "The retained generated document has no generated artifact.".to_owned())?
            .content_digest();
        next.update_generated_artifact(expected_digest, artifact.clone())
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

pub(super) fn generated_project_source_dependencies(
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
    let mut retained_keys = std::collections::HashSet::new();
    for execution in projection.plan().bindings() {
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
            let identity =
                GeneratedSourceMapEntry::component_identity_for(&reference.key(), component.id);
            (emitted, identity)
        })
    })
}
