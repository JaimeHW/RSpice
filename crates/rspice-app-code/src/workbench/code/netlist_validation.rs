//! Validating and formatting the visible deck, answered for the code
//! feature's requests.
//!
//! Validation hands the manual deck to the run controller the feature's
//! context lends, over the run tier's host built from that context, and a
//! failed acknowledgement drops the simulation preflight. Both bodies moved
//! verbatim from the netlist workflow, and are written over its parts.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::code::CodeFeature;
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::run_view::{RunCodeFacts, RunView};
use crate::workbench::slots::{netlist_shown_in, veriloga_compilation_in};
use crate::workbench::workflows::netlist_workflow::{
    acknowledge_canonical_dependencies, acknowledge_canonical_validation,
    invalidate_canonical_validation, normalize_owned_netlist_whitespace, record_validation_failure,
};
use rspice_app_runs::workbench::waiting::{self, WaitingValidation};

/// Validate and retain the exact visible manual-deck snapshot, including its
/// sealed dependency closure and execution-target contract. A later Run must
/// match this one-shot authorized snapshot byte for byte.
pub fn validate_visible_netlist_source(cx: &mut Cx<'_, CodeFeature>) -> bool {
    validate_visible(cx, true).unwrap_or(false)
}

/// Validate the visible deck for the reader, as
/// [`validate_visible_netlist_source`] does, without holding the frame for a
/// Verilog-A compile. When its preparation would read a PDK or model-library
/// source still compiling on another thread, the request waits for that
/// compile instead: the status bar says so, the frame loop validates once it
/// ends ([`poll_waiting_validation`]), and Stop drops it. `then_format`
/// formats the deck once it validates.
pub fn request_validation(cx: &mut Cx<'_, CodeFeature>, then_format: bool) {
    cx.runs.waiting.validation = None;
    let validated = if waiting::compiling() {
        None
    } else {
        validate_visible(cx, false)
    };
    match validated {
        None => cx.runs.waiting.validation = Some(WaitingValidation { then_format }),
        Some(_) if then_format => format_netlist(cx, FormatValidation::Ran),
        Some(_) => {}
    }
}

/// Validate the deck whose validation waits, once no source its preparation
/// reads is compiling. Called once per frame; does nothing while the compile
/// runs.
pub fn poll_waiting_validation(cx: &mut Cx<'_, CodeFeature>) {
    let Some(waiting) = cx.runs.waiting.validation else {
        return;
    };
    if !waiting::compiling() {
        request_validation(cx, waiting.then_format);
    }
}

/// [`validate_visible_netlist_source`], or, when `waits` is false and the
/// preparation reaches a Verilog-A source another thread is compiling,
/// `None`, with nothing recorded.
fn validate_visible(cx: &mut Cx<'_, CodeFeature>, waits: bool) -> Option<bool> {
    use crate::state::NetlistValidationReceipt;
    use crate::workbench::documents::netlist_document::ActiveNetlistDocument;

    if cx.state.netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        cx.wb
            .prompt("A revision comparison is read-only and cannot be validated.");
        return Some(false);
    }
    if cx.state.netlist.active_dependency_identity.is_some() {
        let message = cx
            .wb
            .messages()
            .text(crate::workbench::MessageId::NetlistRootValidationRequired);
        cx.wb.prompt(message);
        return Some(false);
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
        return Some(false);
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
        let controller = &mut *cx.handles.run_controller;
        if waits {
            Some(controller.validate_manual_deck_document(&host))
        } else {
            waiting::unless_compiling(|| controller.validate_manual_deck_document(&host))
        }
    };
    let validated = validated?;
    Some(match validated {
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
                return Some(false);
            }
            if let Err(error) = acknowledge_canonical_validation(cx.model, &mut cx.state.netlist) {
                invalidate_preflight(cx);
                cx.model.netlist_artifacts.validation = None;
                record_validation_failure(cx.model, &error);
                cx.model.netlist_artifacts.validation_error = Some(error);
                return Some(false);
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
    })
}

/// Format the deck or the include the netlist editor shows, when it is
/// editable: trailing blanks come off every card but the title, and the
/// text ends with a line end. An include is formatted only when it parses
/// without errors, and the deck only when it validates. A deck whose
/// validation is not current is validated as [`request_validation`]
/// validates it, and formatted once it validates, so a compile in flight
/// holds no frame.
pub fn format_owned_netlist(cx: &mut Cx<'_, CodeFeature>) {
    format_netlist(cx, FormatValidation::Requested);
}

/// How formatting a deck whose validation is not current goes on.
#[derive(Clone, Copy, PartialEq, Eq)]
enum FormatValidation {
    /// It asks for the validation, which formats the deck once it validates.
    Requested,
    /// The validation has just run; a deck it did not validate is refused.
    Ran,
}

fn format_netlist(cx: &mut Cx<'_, CodeFeature>, validation: FormatValidation) {
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
        match validation {
            FormatValidation::Requested => request_validation(cx, true),
            FormatValidation::Ran => refuse_format(cx),
        }
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

/// Answer the format gesture for a deck that did not validate. Validation has
/// already recorded why it failed.
fn refuse_format(cx: &mut Cx<'_, CodeFeature>) {
    let message = cx
        .model
        .netlist_artifacts
        .validation_error
        .clone()
        .unwrap_or_else(|| "the netlist does not validate".to_owned());
    cx.wb
        .prompt(format!("The netlist was not formatted: {message}"));
}

/// Drop the simulation preflight, as an acknowledgement that failed leaves
/// it stale.
fn invalidate_preflight(cx: &mut Cx<'_, CodeFeature>) {
    rspice_app_runs::workbench::preflight::invalidate(
        &mut cx.runs.preflight,
        &mut *cx.handles.run_controller,
    );
}
