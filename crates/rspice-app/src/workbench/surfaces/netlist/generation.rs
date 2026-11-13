//! Generated-primary reconciliation.
//!
//! Generation runs into a temporary string: a failed build retains the prior
//! valid artifact and marks it stale rather than publishing partial bytes.
//! The source map and the authenticated Verilog-A dependency closure come out
//! of the same pass, which is what lets a generated line name the schematic
//! object that produced it.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{
    GeneratedArtifact, GeneratedProvenance, GeneratedSourceMapEntry, GenerationInput,
    NetlistDocument, NetlistDocumentId,
};
use crate::workbench::app_state::Model;
use crate::workbench::code::CodeFeature;
use crate::workbench::documents::netlist_document::{ActiveNetlistDocument, NetlistDocumentState};
use crate::workbench::framework::feature::Cx;

/// Rebuild the immutable generated primary whenever any authoritative project
/// input changes. Generation happens into a temporary string; a failed build
/// retains the prior valid artifact and marks it stale instead of publishing
/// partial bytes.
pub(super) fn reconcile_documents(cx: &mut Cx<'_, CodeFeature>) {
    if cx.state.netlist.owned_document.is_none() {
        cx.state.netlist.owned_document = cx.model.workspace.content.netlist_document.clone();
    }
    if !cx.state.netlist.active_document_initialized {
        cx.state.netlist.active_document = if cx.model.workspace.content.netlist_source.is_some() {
            ActiveNetlistDocument::OwnedSource
        } else {
            ActiveNetlistDocument::Generated
        };
        cx.state.netlist.active_document_initialized = true;
    }

    if cx.state.netlist.active_dependency_identity.is_some()
        && crate::workbench::documents::netlist_document::active_dependency_in(
            cx.model,
            &cx.state.netlist,
        )
        .is_none()
    {
        crate::workbench::documents::netlist_document::close_active_dependency_in(
            cx.model,
            &mut cx.state.netlist,
            cx.runs,
        );
        cx.model.record(
            LogSeverity::Warning,
            LogSource::Netlist,
            "The open include file is no longer part of the netlist; the top-level deck is shown \
             instead.",
        );
    }

    refresh_generated_artifact(cx);

    let projected = crate::workbench::documents::netlist_document::active_dependency_in(
        cx.model,
        &cx.state.netlist,
    )
    .and_then(crate::state::DependencyMetadata::source)
    .map(str::to_owned)
    .or_else(|| match cx.state.netlist.active_document {
        ActiveNetlistDocument::Generated => {
            Some(cx.model.netlist_artifacts.generated_source.clone())
        }
        ActiveNetlistDocument::OwnedSource => cx.model.workspace.content.netlist_source.clone(),
        ActiveNetlistDocument::GeneratedDiff => {
            Some(cx.state.netlist.generated_diff_source.clone())
        }
        ActiveNetlistDocument::RunSnapshot => cx.runs.manual_run.last_run_buffer.clone(),
    });
    if let Some(projected) = projected
        && cx.model.simulation.source.netlist_content != projected
    {
        cx.model.simulation.source.netlist_content = projected;
        cx.state.netlist.revision = cx.state.netlist.revision.wrapping_add(1);
        cx.model.invalidate_source_evidence();
    }
}

pub(crate) fn refresh_generated_artifact(cx: &mut Cx<'_, CodeFeature>) {
    let input_digest =
        match crate::workbench::lifecycle::project_lifecycle::generated_netlist_input_digest(
            &crate::workbench::lifecycle::project_lifecycle::LifecycleView {
                model: cx.model,
                platform: cx.platform,
                workspace: cx.wb.workspace,
                presentation: cx.peers.results().saved_presentation(),
            },
        ) {
            Ok(digest) => digest,
            Err(error) => {
                cx.model.netlist_artifacts.current_generation_input_digest = None;
                cx.model.netlist_artifacts.generation_error = Some(error.to_string());
                return;
            }
        };
    let authoritative_input_changed = cx
        .model
        .netlist_artifacts
        .current_generation_input_digest
        .is_some_and(|previous| previous != input_digest);
    cx.model.netlist_artifacts.current_generation_input_digest = Some(input_digest);
    if authoritative_input_changed {
        rspice_app_runs::workbench::preflight::invalidate(
            &mut cx.runs.preflight,
            cx.handles.run_controller,
        );
        cx.model.invalidate_source_evidence();
        if let Some(document) = cx.model.netlist_artifacts.generated_document.as_mut() {
            let _ = document.invalidate_validation(document.content_digest());
        }
        if let Some(document) = cx.state.netlist.owned_document.as_mut() {
            let _ = document.invalidate_validation(document.content_digest());
            cx.model.workspace.content.netlist_document = Some(document.clone());
        }
    }
    if cx.model.netlist_artifacts.generated_input_digest == Some(input_digest)
        && !cx.model.netlist_artifacts.generated_source.is_empty()
    {
        return;
    }
    if cx.model.schematic.document().components.is_empty() {
        cx.model.netlist_artifacts.generation_error =
            Some("Add a circuit before generating the primary netlist.".to_owned());
        return;
    }
    // This runs every frame the page is shown. Inputs that already failed
    // fail the same way, and the page still states why.
    if cx.state.netlist.failed_generation_input_digest == Some(input_digest) {
        return;
    }

    let generated = crate::workbench::menu_bar::build_menu_netlist_in(cx.model);
    let failed = |model: &mut Model, netlist: &mut NetlistDocumentState, error: String| {
        netlist.failed_generation_input_digest = Some(input_digest);
        model.netlist_artifacts.generation_error = Some(error);
    };
    match generated {
        Ok(generated) => {
            match publish_generated_document(
                cx.model,
                &cx.state.netlist,
                input_digest,
                generated.source,
            ) {
                Ok((generated_document, owned_document)) => {
                    if let (Some(previous), Some(current)) = (
                        cx.model
                            .netlist_artifacts
                            .generated_document
                            .as_ref()
                            .and_then(crate::state::NetlistDocument::generated_artifact),
                        generated_document.generated_artifact(),
                    ) && previous.content_digest() != current.content_digest()
                    {
                        let predecessor = previous.clone();
                        if cx
                            .state
                            .netlist
                            .generated_history
                            .last()
                            .is_none_or(|artifact| {
                                artifact.content_digest() != predecessor.content_digest()
                            })
                        {
                            cx.state.netlist.generated_history.push(predecessor);
                            const RETAINED_GENERATED_REVISIONS: usize = 16;
                            let excess = cx
                                .state
                                .netlist
                                .generated_history
                                .len()
                                .saturating_sub(RETAINED_GENERATED_REVISIONS);
                            if excess > 0 {
                                cx.state.netlist.generated_history.drain(..excess);
                            }
                        }
                    }
                    let Some(generated_artifact) = generated_document.generated_artifact() else {
                        failed(
                            cx.model,
                            &mut cx.state.netlist,
                            "Generated document publication returned no generated artifact."
                                .to_owned(),
                        );
                        return;
                    };
                    cx.model.netlist_artifacts.generated_source =
                        generated_artifact.source().to_owned();
                    cx.model.netlist_artifacts.generated_document = Some(generated_document);
                    cx.model.workspace.content.netlist_document = owned_document.clone();
                    cx.state.netlist.owned_document = owned_document;
                    cx.model.netlist_artifacts.generated_input_digest = Some(input_digest);
                    cx.model.netlist_artifacts.generation_error = None;
                    cx.state.netlist.failed_generation_input_digest = None;
                    crate::workbench::menu_bar::record_netlist_warnings_in(
                        cx.model,
                        generated.warnings,
                    );
                }
                Err(error) => failed(cx.model, &mut cx.state.netlist, error),
            }
        }
        Err(reasons) => {
            cx.model.record(
                LogSeverity::Error,
                LogSource::Studio,
                crate::workbench::menu_bar::netlist_failure_text("Netlist generation", &reasons),
            );
            failed(cx.model, &mut cx.state.netlist, reasons.join("; "));
        }
    }
}

pub(super) fn publish_generated_document(
    model: &Model,
    netlist: &NetlistDocumentState,
    input_digest: crate::product::ContentDigest,
    source: String,
) -> Result<(NetlistDocument, Option<NetlistDocument>), String> {
    let provenance = GeneratedProvenance::try_new(
        "rspice-netlist-generator/v1",
        GenerationInput::new(model.workspace.content.project.revision(), input_digest),
    )
    .map_err(|error| error.to_string())?;
    let source_map = generated_source_map(model, &source)?;
    let dependencies = generated_project_source_dependencies(model, &source)?;
    let artifact =
        GeneratedArtifact::try_from_utf8(provenance, source.into_bytes(), dependencies, source_map)
            .map_err(|error| error.to_string())?;

    let generated_document = if let Some(existing) = &model.netlist_artifacts.generated_document {
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
    let owned_document = netlist.owned_document.clone();
    Ok((generated_document, owned_document))
}

pub(super) fn generated_project_source_dependencies(
    model: &Model,
    source: &str,
) -> Result<Vec<crate::state::DependencyMetadata>, String> {
    let include_directives = crate::state::parse_include_directives(source);
    let mut dependencies = Vec::new();

    let projection = model
        .workspace
        .configuration_execution_projection(
            &model.library_manager,
            &model.workspace.content.active_view,
            &model.schematic,
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
        let bundle = model
            .workspace
            .content
            .project_sources
            .get_bundle(binding.source_bundle_id())
            .ok_or_else(|| {
                format!(
                    "The Verilog-A instance at {} names a source that no longer exists ({})",
                    execution.instance_path(),
                    binding.source_bundle_id()
                )
            })?;
        if bundle.closure_digest() != binding.source_closure_digest() {
            return Err(format!(
                "Verilog-A source {} changed after the hierarchy was resolved",
                bundle.id()
            ));
        }
        let include_index = include_directives
            .iter()
            .position(|directive| directive.locator() == binding.source_key())
            .ok_or_else(|| {
                format!(
                    "Generated source is missing the Verilog-A binding for {}",
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
                    "Could not compile the Verilog-A source used by {}: {failure}",
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
                    "The compiled Verilog-A files for {} do not include the source's root file",
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
                        "Verilog-A include parent '{}' is missing from the compiled files",
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
                        "Verilog-A include target '{}' is missing from the compiled files",
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
    model: &Model,
    source: &str,
) -> Result<Vec<GeneratedSourceMapEntry>, String> {
    let top = model.workspace.content.active_view.clone();
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
            current = model
                .workspace
                .content
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

        let (instance_identity, component_identity) = source_line_component(model, &current, line)
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
    model: &Model,
    reference: &crate::state::CellViewRef,
    line: &str,
) -> Option<(String, String)> {
    let mut tokens = line.split_whitespace();
    let token = tokens.next()?;
    if token.starts_with(['.', '*']) {
        return None;
    }
    let named = tokens.next();
    let schematic = model
        .workspace
        .content
        .schematic_buffers
        .get(&reference.key())?;
    schematic
        .document()
        .components
        .iter()
        .find_map(|component| {
            // A card is headed by the element it writes, which is the
            // component's written name: a limited `E1` heads its card `B1`.
            // A keyword-headed card names it after the keyword: a memristor
            // is written `YMEMRISTOR MR1`.
            let written = component.written_element_name();
            let head = match component.kind.element_keyword() {
                Some(keyword) if token.eq_ignore_ascii_case(keyword) => named?,
                Some(_) => return None,
                None => token,
            };
            head.eq_ignore_ascii_case(&written).then(|| {
                let identity =
                    GeneratedSourceMapEntry::component_identity_for(&reference.key(), component.id);
                (written, identity)
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Point, SchematicState};
    use crate::workbench::code::tests::harness;

    /// The line `YMEMRISTOR MR1 …` is the memristor drawn `MR1`: a
    /// keyword-headed card names its element after the keyword.
    #[test]
    fn a_keyword_headed_card_names_the_component_after_its_keyword() {
        let mut harness = harness();
        let view = harness.model.workspace.content.active_view.key();
        let mut schematic = SchematicState::default();
        schematic.document_mut_for_test().components.extend([
            Component::new(4, ComponentType::Memristor, Point::origin()).with_name_value("MR1", ""),
            Component::new(5, ComponentType::Resistor, Point::new(200, 0))
                .with_name_value("R1", "1k"),
        ]);
        let (design, _) = schematic.into_parts();
        harness
            .model
            .workspace
            .content
            .schematic_buffers
            .insert(view.clone(), design);

        let entries = generated_source_map(
            &harness.model,
            "* title\nYMEMRISTOR MR1 a 0 mem_MR1\nR1 a 0 1k\nYMEMRISTOR R1 a 0 m\n",
        )
        .expect("source map");

        let mapped = |line: usize| {
            (
                entries[line].instance_identity().map(str::to_owned),
                entries[line].component_identity().map(str::to_owned),
            )
        };
        let component = |id| Some(GeneratedSourceMapEntry::component_identity_for(&view, id));
        assert_eq!(mapped(1), (Some("MR1".to_owned()), component(4)));
        assert_eq!(mapped(2), (Some("R1".to_owned()), component(5)));
        assert_eq!(
            mapped(3),
            (None, None),
            "a keyword card names no component of another kind"
        );
    }
}
