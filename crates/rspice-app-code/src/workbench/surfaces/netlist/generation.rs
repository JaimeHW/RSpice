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
use crate::workbench::documents::netlist_document::{
    NetlistDocumentState, NetlistRoot, NetlistTab,
};
use crate::workbench::framework::feature::Cx;
use crate::workbench::{MessageCatalog, MessageId};

/// Rebuild the immutable generated primary whenever any authoritative project
/// input changes. Generation happens into a temporary string; a failed build
/// retains the prior valid artifact and marks it stale instead of publishing
/// partial bytes.
pub(super) fn reconcile_documents(cx: &mut Cx<'_, CodeFeature>) {
    if !cx.state.netlist.active_document_initialized {
        // A deck the session opens with is in front, and Run runs it.
        match cx.state.netlist.open_deck.as_ref().map(|deck| deck.id) {
            Some(id) => {
                cx.state.netlist.front = NetlistTab::Deck(id);
                cx.state.netlist.run_target =
                    crate::workbench::documents::netlist_tabs::RunTarget::Deck(id);
            }
            None => cx.state.netlist.front = NetlistTab::Generated,
        }
        cx.state.netlist.active_document_initialized = true;
    }

    if cx.state.netlist.front.include().is_some()
        && crate::workbench::documents::netlist_document::active_dependency_in(
            cx.model,
            &cx.state.netlist,
        )
        .is_none()
        && crate::workbench::documents::netlist_document::active_deck_file_in(&cx.state.netlist)
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
            cx.wb.messages().text(MessageId::NetlistIncludeTabLeft),
        );
    }

    refresh_generated_artifact(cx);

    // The deck or the include in front is its file's text, which the editor
    // edits in place: the buffer follows it, never the other way round.
    if cx.state.netlist.front.shows_deck() {
        if let Some(file) =
            crate::workbench::documents::netlist_document::active_deck_file_in(&cx.state.netlist)
            && cx.model.simulation.source.netlist_content != file.text
        {
            cx.model.simulation.source.netlist_content = file.text.clone();
            cx.state.netlist.revision = cx.state.netlist.revision.wrapping_add(1);
            cx.model.invalidate_source_evidence();
        }
        return;
    }

    let projected = crate::workbench::documents::netlist_document::active_dependency_in(
        cx.model,
        &cx.state.netlist,
    )
    .and_then(crate::state::DependencyMetadata::source)
    .map(str::to_owned)
    .or_else(|| match cx.state.netlist.front.root_kind() {
        NetlistRoot::Generated => Some(cx.model.netlist_artifacts.generated_source.clone()),
        NetlistRoot::AddedLines | NetlistRoot::Deck => None,
        NetlistRoot::Comparison => Some(cx.state.netlist.generated_diff_source.clone()),
        NetlistRoot::RunInput => Some(
            crate::workbench::documents::netlist_document::run_snapshot_source_in(
                cx.model,
                &cx.state.netlist,
                cx.runs,
            ),
        ),
    });
    if let Some(projected) = projected
        && cx.model.simulation.source.netlist_content != projected
    {
        cx.model.simulation.source.netlist_content = projected;
        cx.state.netlist.revision = cx.state.netlist.revision.wrapping_add(1);
        cx.model.invalidate_source_evidence();
    }
}

/// Take the reader to the generated netlist of the design as it stands now:
/// the Netlist workspace, with the generated netlist in front.
///
/// The netlist is generated first. Only the Netlist page generates it while it
/// is drawn, so a jump from elsewhere would otherwise arrive at nothing when
/// the page was never opened, or at a netlist older than the design.
pub fn show_generated_netlist(cx: &mut Cx<'_, CodeFeature>) {
    refresh_generated_artifact(cx);
    crate::workbench::documents::netlist_document::show_generated_deck(
        cx.model,
        &mut cx.state.netlist,
        cx.wb,
    );
}

pub fn refresh_generated_artifact(cx: &mut Cx<'_, CodeFeature>) {
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
    }
    if cx.model.netlist_artifacts.generated_input_digest == Some(input_digest)
        && !cx.model.netlist_artifacts.generated_source.is_empty()
    {
        return;
    }
    if cx.model.schematic.document().components.is_empty() {
        cx.model.netlist_artifacts.generation_error = Some(
            cx.wb
                .messages()
                .text(MessageId::NetlistGeneratedUnavailableDescription),
        );
        return;
    }
    // This runs every frame the page is shown. Inputs that already failed
    // fail the same way, and the page still states why.
    if cx.state.netlist.failed_generation_input_digest == Some(input_digest) {
        return;
    }

    let generated = crate::workbench::menu_bar::build_menu_netlist_in(cx.model, cx.wb.messages());
    let failed = |model: &mut Model, netlist: &mut NetlistDocumentState, error: String| {
        netlist.failed_generation_input_digest = Some(input_digest);
        model.netlist_artifacts.generation_error = Some(error);
    };
    match generated {
        Ok(generated) => {
            match publish_generated_document(
                cx.model,
                input_digest,
                generated.source,
                &generated.line_map,
                cx.wb.messages(),
            ) {
                Ok(generated_document) => {
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
                            cx.wb.messages().text(MessageId::NetlistGenerationNotKept),
                        );
                        return;
                    };
                    cx.model.netlist_artifacts.generated_source =
                        generated_artifact.source().to_owned();
                    cx.model.netlist_artifacts.generated_document = Some(generated_document);
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
                crate::workbench::menu_bar::netlist_failure_text(
                    &cx.wb.messages().text(MessageId::NetlistGenerationHeading),
                    &reasons,
                ),
            );
            failed(cx.model, &mut cx.state.netlist, reasons.join("; "));
        }
    }
}

pub(super) fn publish_generated_document(
    model: &Model,
    input_digest: crate::product::ContentDigest,
    source: String,
    line_map: &[rspice_simulation::netlist_gen::WrittenLine],
    messages: MessageCatalog,
) -> Result<NetlistDocument, String> {
    let provenance = GeneratedProvenance::try_new(
        "rspice-netlist-generator/v1",
        GenerationInput::new(model.workspace.content.project.revision(), input_digest),
    )
    .map_err(|error| error.to_string())?;
    let source_map = generated_source_map(model, &source, line_map)?;
    let dependencies = generated_project_source_dependencies(model, &source, messages)?;
    let artifact =
        GeneratedArtifact::try_from_utf8(provenance, source.into_bytes(), dependencies, source_map)
            .map_err(|error| error.to_string())?;

    let generated_document = if let Some(existing) = &model.netlist_artifacts.generated_document {
        let mut next = existing.clone();
        let expected_digest = existing
            .generated_artifact()
            .ok_or_else(|| messages.text(MessageId::NetlistGeneratedPreviousMissing))?
            .content_digest();
        next.update_generated_artifact(expected_digest, artifact.clone())
            .map_err(|error| error.to_string())?;
        next
    } else {
        NetlistDocument::from_generated(NetlistDocumentId::new(), artifact.clone())
            .map_err(|error| error.to_string())?
    };
    Ok(generated_document)
}

pub(super) fn generated_project_source_dependencies(
    model: &Model,
    source: &str,
    messages: MessageCatalog,
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
    for execution in projection.plan().distinct_bindings() {
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
                messages.format(
                    MessageId::NetlistGenerationSourceGone,
                    &[("instance", &execution.instance_path().to_string())],
                )
            })?;
        if bundle.closure_digest() != binding.source_closure_digest() {
            return Err(messages.format(
                MessageId::NetlistGenerationSourceChanged,
                &[("file", bundle.root().logical_path())],
            ));
        }
        let include_index = include_directives
            .iter()
            .position(|directive| directive.locator() == binding.source_key())
            .ok_or_else(|| {
                messages.format(
                    MessageId::NetlistGenerationIncludeMissing,
                    &[("instance", &execution.instance_path().to_string())],
                )
            })?;
        let root_locator = crate::state::SourceLocator::try_new(
            binding.source_key(),
            bundle.root().logical_path(),
        )
        .map_err(|error| error.to_string())?;
        let closure =
            crate::workbench::documents::code_workspace::project_bundle_closure_for_provenance(
                bundle,
                binding.selected_module(),
            )
            .map_err(|failure| {
                messages.format(
                    MessageId::NetlistGenerationSourceUnreadable,
                    &[
                        ("instance", &execution.instance_path().to_string()),
                        ("error", &failure.to_string()),
                    ],
                )
            })?;
        let read_root = closure
            .dependency_closure
            .first()
            .filter(|dependency| {
                dependency
                    .logical_path
                    .eq_ignore_ascii_case(bundle.root().logical_path())
            })
            .ok_or_else(|| {
                messages.format(
                    MessageId::NetlistGenerationMainFileUnread,
                    &[
                        ("instance", &execution.instance_path().to_string()),
                        ("file", bundle.root().logical_path()),
                    ],
                )
            })?;
        dependencies.push(
            crate::state::DependencyMetadata::unresolved_direct_to(
                include_index,
                binding.source_key(),
                root_locator.clone(),
            )
            .map_err(|error| error.to_string())?
            .resolve_utf8(read_root.source.as_bytes().to_vec())
            .map_err(|error| error.to_string())?,
        );
        for edge in &closure.include_graph {
            let parent = closure
                .dependency_closure
                .iter()
                .find(|dependency| {
                    dependency
                        .logical_path
                        .eq_ignore_ascii_case(&edge.including_path)
                })
                .ok_or_else(|| {
                    messages.format(
                        MessageId::NetlistGenerationIncludingFileUnread,
                        &[
                            ("file", &edge.including_path),
                            ("included", &edge.included_path),
                        ],
                    )
                })?;
            let child = closure
                .dependency_closure
                .iter()
                .find(|dependency| {
                    dependency
                        .logical_path
                        .eq_ignore_ascii_case(&edge.included_path)
                })
                .ok_or_else(|| {
                    messages.format(
                        MessageId::NetlistGenerationIncludedFileUnread,
                        &[
                            ("file", &edge.included_path),
                            ("including", &edge.including_path),
                        ],
                    )
                })?;
            let parent_locator = compiler_dependency_locator(
                binding.source_key(),
                bundle.root().logical_path(),
                parent,
                &root_locator,
                messages,
            )?;
            let child_locator = compiler_dependency_locator(
                binding.source_key(),
                bundle.root().logical_path(),
                child,
                &root_locator,
                messages,
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
    messages: MessageCatalog,
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
            .ok_or_else(|| {
                messages.format(
                    MessageId::NetlistGenerationBuiltInUnnamed,
                    &[("path", &dependency.logical_path)],
                )
            })?;
        return crate::state::SourceLocator::try_new(
            format!("__rspice_builtin__/veriloga/{name}"),
            dependency.logical_path.clone(),
        )
        .map_err(|error| error.to_string());
    }
    project_bundle_locator(source_key, &dependency.logical_path)
}

/// The netlist's line map as the netlist document keeps it: each line's cell
/// view, the component its card writes, and the nets it connects, as the
/// generator recorded them while writing the line. A line no cell view writes
/// belongs to the design's own netlist.
fn generated_source_map(
    model: &Model,
    source: &str,
    line_map: &[rspice_simulation::netlist_gen::WrittenLine],
) -> Result<Vec<GeneratedSourceMapEntry>, String> {
    let root = model.workspace.content.simulation_root_reference();
    source
        .lines()
        .enumerate()
        .map(|(index, _)| {
            let written = line_map.get(index);
            let view = written
                .and_then(|written| written.cell_view.as_ref())
                .unwrap_or(&root);
            let entry = GeneratedSourceMapEntry::try_new(
                index + 1,
                format!("{}/{}", view.library, view.cell),
                view.key(),
                written.and_then(|written| written.element.clone()),
                written
                    .and_then(|written| written.component)
                    .map(|component| {
                        GeneratedSourceMapEntry::component_identity_for(&view.key(), component)
                    }),
            )
            .map_err(|error| error.to_string())?;
            Ok(entry.with_nets(
                written
                    .map(|written| written.nets.clone())
                    .unwrap_or_default(),
            ))
        })
        .collect()
}
