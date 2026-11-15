//! Validating and formatting the visible deck, answered for the code
//! feature's requests.
//!
//! Validation hands the manual deck to the run controller the feature's
//! context lends, over the run tier's host built from that context, and a
//! failed acknowledgement drops the simulation preflight. Both bodies moved
//! verbatim from the netlist workflow, and are written over its parts.

use rspice_app_code::workbench::slots::code::{netlist_shown_in, veriloga_compilation_in};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::code::CodeFeature;
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::run_view::{RunCodeFacts, RunView};
use crate::workbench::workflows::netlist_workflow::{
    acknowledge_canonical_dependencies, acknowledge_canonical_validation,
    invalidate_canonical_validation, normalize_owned_netlist_whitespace, record_validation_failure,
};

/// Validate and retain the exact visible manual-deck snapshot, including its
/// sealed dependency closure and execution-target contract. A later Run must
/// match this one-shot authorized snapshot byte for byte.
pub(crate) fn validate_visible_netlist_source(cx: &mut Cx<'_, CodeFeature>) -> bool {
    use crate::state::NetlistValidationReceipt;
    use crate::workbench::documents::netlist_document::ActiveNetlistDocument;

    if cx.state.netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        cx.wb
            .prompt("A revision comparison is read-only and cannot be validated.");
        return false;
    }
    if cx.state.netlist.active_dependency_identity.is_some() {
        let message = cx
            .wb
            .messages()
            .text(crate::workbench::MessageId::NetlistRootValidationRequired);
        cx.wb.prompt(message);
        return false;
    }
    if cx.state.netlist.active_document == ActiveNetlistDocument::Generated
        && (cx.model.netlist_artifacts.generation_error.is_some()
            || cx.model.netlist_artifacts.generated_input_digest
                != cx.model.netlist_artifacts.current_generation_input_digest)
    {
        let error = cx
            .model
            .netlist_artifacts
            .generation_error
            .clone()
            .unwrap_or_else(|| "Generated netlist is stale.".to_owned());
        cx.model.netlist_artifacts.validation = None;
        record_validation_failure(cx.model, &error);
        cx.model.netlist_artifacts.validation_error = Some(error);
        return false;
    }

    let visible_digest = crate::state::content_digest(&cx.model.simulation.source.netlist_content);
    let validated = {
        let code = RunCodeFacts {
            veriloga_compilation: veriloga_compilation_in(&cx.state.code_workspace),
            netlist_shown: netlist_shown_in(&cx.state.netlist),
        };
        let host = RunView::new(
            &mut *cx.model,
            &mut *cx.runs,
            &mut *cx.wb,
            &*cx.platform,
            &mut cx.peers,
            code,
            cx.commands,
        );
        cx.handles
            .run_controller
            .validate_manual_deck_document(&host)
    };
    match validated {
        Ok(metadata) => {
            if let Err(error) = acknowledge_canonical_dependencies(
                cx.model,
                &mut cx.state.netlist,
                &mut cx.state.code_workspace.include_resolutions,
                &metadata.sealed_source_dependencies,
            ) {
                invalidate_preflight(cx);
                cx.model.netlist_artifacts.validation = None;
                record_validation_failure(cx.model, &error);
                cx.model.netlist_artifacts.validation_error = Some(error);
                return false;
            }
            if let Err(error) = acknowledge_canonical_validation(cx.model, &mut cx.state.netlist) {
                invalidate_preflight(cx);
                cx.model.netlist_artifacts.validation = None;
                record_validation_failure(cx.model, &error);
                cx.model.netlist_artifacts.validation_error = Some(error);
                return false;
            }
            cx.model.netlist_artifacts.validation = Some(NetlistValidationReceipt {
                visible_content_digest: visible_digest,
                executable_source_digest: metadata.source_digest,
                prepared_snapshot_digest: metadata.snapshot_digest,
                project_revision: metadata.project_revision,
                task_count: metadata.task_count,
                advisory_count: metadata.advisories.len(),
            });
            cx.model.netlist_artifacts.validation_error = None;
            cx.model.record(
                LogSeverity::Info,
                LogSource::Netlist,
                format!(
                    "Validated the netlist: {} task{}.",
                    metadata.task_count,
                    if metadata.task_count == 1 { "" } else { "s" },
                ),
            );
            true
        }
        Err(error) => {
            let message = error.to_string();
            invalidate_canonical_validation(cx.model, &mut cx.state.netlist);
            cx.model.netlist_artifacts.validation = None;
            record_validation_failure(cx.model, &message);
            cx.model.netlist_artifacts.validation_error = Some(message);
            false
        }
    }
}

/// Format the deck or the include the netlist editor shows, when it is
/// editable: trailing blanks come off every card but the title, and the
/// text ends with a line end. An include is formatted only when it parses
/// without errors, and the deck only when it validates, after validating
/// it when its validation is not current.
pub(crate) fn format_owned_netlist(cx: &mut Cx<'_, CodeFeature>) {
    if !crate::workbench::documents::netlist_document::active_netlist_source_is_editable_in(
        cx.model,
        &cx.state.netlist,
    ) {
        return;
    }
    let source = cx.model.simulation.source.netlist_content.clone();
    let dependency_document = cx.state.netlist.active_dependency_identity.is_some();
    if dependency_document {
        let has_errors = cx.state.netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.is_current()
                && diagnostic.severity
                    == crate::workbench::documents::netlist_document::DiagnosticSeverity::Error
        });
        if has_errors {
            cx.wb
                .prompt("Fix the include file's syntax errors before formatting it.");
            return;
        }
        let formatted = normalize_owned_netlist_whitespace(&source);
        if formatted == source {
            cx.wb.prompt("The include file is already formatted.");
        } else if crate::workbench::documents::netlist_document::replace_owned_dependency_source(
            cx.model,
            &mut cx.state.netlist,
            &mut cx.state.netlist_open_documents,
            formatted,
        ) {
            cx.wb
                .prompt("Formatted the include file; the top-level deck needs validating again.");
        }
        return;
    }
    let digest = crate::state::content_digest(&source);
    let validation_current =
        cx.model
            .netlist_artifacts
            .validation
            .as_ref()
            .is_some_and(|receipt| {
                receipt.visible_content_digest == digest
                    && receipt.project_revision
                        == cx.model.workspace.content.project.revision().get()
            });
    if !validation_current {
        validate_visible_netlist_source(cx);
    }
    let validation_current =
        cx.model
            .netlist_artifacts
            .validation
            .as_ref()
            .is_some_and(|receipt| {
                receipt.visible_content_digest == digest
                    && receipt.project_revision
                        == cx.model.workspace.content.project.revision().get()
            });
    if !validation_current {
        let message = cx
            .model
            .netlist_artifacts
            .validation_error
            .clone()
            .unwrap_or_else(|| "the netlist does not validate".to_owned());
        // Validation has already recorded why it failed; this answers the
        // format gesture.
        cx.wb
            .prompt(format!("The netlist was not formatted: {message}"));
        return;
    }

    let formatted = normalize_owned_netlist_whitespace(&source);
    if formatted == source {
        cx.wb.prompt("The netlist is already formatted.");
        return;
    }
    if crate::workbench::documents::netlist_document::replace_owned_source(
        cx.model,
        &mut cx.state.netlist,
        &mut cx.state.netlist_open_documents,
        cx.wb,
        formatted,
    ) {
        cx.wb
            .prompt("Formatted the netlist; it needs validating again.");
    } else {
        cx.wb
            .prompt("The netlist changed before formatting finished. Review it and try again.");
    }
}

/// Drop the simulation preflight, as an acknowledgement that failed leaves
/// it stale.
fn invalidate_preflight(cx: &mut Cx<'_, CodeFeature>) {
    rspice_app_runs::workbench::preflight::invalidate(
        &mut cx.runs.preflight,
        &mut *cx.handles.run_controller,
    );
}
