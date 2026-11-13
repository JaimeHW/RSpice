//! Validating and formatting the visible deck, answered for the code
//! feature's requests.
//!
//! Validation hands the manual deck to the runs controller, and a failed
//! acknowledgement drops the simulation preflight. No feature context is lent
//! the runs controller yet, so the code is held here in the shell only until
//! the runs controller is lent to a feature context. Its long-term home is
//! the runs tier: the manual-deck handoff is a runs fact. Both bodies moved
//! verbatim from the netlist workflow, and are written over its parts.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app::RSpiceApp;
use crate::workbench::workflows::netlist_workflow::{
    acknowledge_canonical_dependencies, acknowledge_canonical_validation,
    invalidate_canonical_validation, normalize_owned_netlist_whitespace, record_validation_failure,
};

/// Validate and retain the exact visible manual-deck snapshot, including its
/// sealed dependency closure and execution-target contract. A later Run must
/// match this one-shot authorized snapshot byte for byte.
pub(crate) fn validate_visible_netlist_source(app: &mut RSpiceApp) -> bool {
    use crate::state::NetlistValidationReceipt;
    use crate::workbench::documents::netlist_document::ActiveNetlistDocument;

    if app.state.code.netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        app.state
            .prompt("A revision comparison is read-only and cannot be validated.");
        return false;
    }
    if app.state.code.netlist.active_dependency_identity.is_some() {
        let message = app
            .state
            .workbench
            .messages()
            .text(crate::workbench::MessageId::NetlistRootValidationRequired);
        app.state.prompt(message);
        return false;
    }
    if app.state.code.netlist.active_document == ActiveNetlistDocument::Generated
        && (app.state.model.netlist_artifacts.generation_error.is_some()
            || app.state.model.netlist_artifacts.generated_input_digest
                != app
                    .state
                    .model
                    .netlist_artifacts
                    .current_generation_input_digest)
    {
        let error = app
            .state
            .model
            .netlist_artifacts
            .generation_error
            .clone()
            .unwrap_or_else(|| "Generated netlist is stale.".to_owned());
        app.state.model.netlist_artifacts.validation = None;
        record_validation_failure(&mut app.state, &error);
        app.state.model.netlist_artifacts.validation_error = Some(error);
        return false;
    }

    let visible_digest =
        crate::state::content_digest(&app.state.model.simulation.source.netlist_content);
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
                app.state.model.netlist_artifacts.validation = None;
                record_validation_failure(&mut app.state, &error);
                app.state.model.netlist_artifacts.validation_error = Some(error);
                return false;
            }
            if let Err(error) = acknowledge_canonical_validation(&mut app.state) {
                app.invalidate_simulation_preflight();
                app.state.model.netlist_artifacts.validation = None;
                record_validation_failure(&mut app.state, &error);
                app.state.model.netlist_artifacts.validation_error = Some(error);
                return false;
            }
            app.state.model.netlist_artifacts.validation = Some(NetlistValidationReceipt {
                visible_content_digest: visible_digest,
                executable_source_digest: metadata.source_digest,
                prepared_snapshot_digest: metadata.snapshot_digest,
                project_revision: metadata.project_revision,
                task_count: metadata.task_count,
                advisory_count: metadata.advisories.len(),
            });
            app.state.model.netlist_artifacts.validation_error = None;
            app.state.record(
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
            invalidate_canonical_validation(&mut app.state);
            app.state.model.netlist_artifacts.validation = None;
            record_validation_failure(&mut app.state, &message);
            app.state.model.netlist_artifacts.validation_error = Some(message);
            false
        }
    }
}

/// Format the deck or the include the netlist editor shows, when it is
/// editable: trailing blanks come off every card but the title, and the
/// text ends with a line end. An include is formatted only when it parses
/// without errors, and the deck only when it validates, after validating
/// it when its validation is not current.
pub(crate) fn format_owned_netlist(app: &mut RSpiceApp) {
    if !crate::workbench::documents::netlist_document::active_netlist_source_is_editable_in(
        &app.state.model,
        &app.state.code.netlist,
    ) {
        return;
    }
    let source = app.state.model.simulation.source.netlist_content.clone();
    let dependency_document = app.state.code.netlist.active_dependency_identity.is_some();
    if dependency_document {
        let has_errors = app.state.code.netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.is_current()
                && diagnostic.severity
                    == crate::workbench::documents::netlist_document::DiagnosticSeverity::Error
        });
        if has_errors {
            app.state
                .prompt("Fix the include file's syntax errors before formatting it.");
            return;
        }
        let formatted = normalize_owned_netlist_whitespace(&source);
        if formatted == source {
            app.state.prompt("The include file is already formatted.");
        } else if crate::workbench::documents::netlist_document::replace_owned_dependency_source(
            &mut app.state.model,
            &mut app.state.code.netlist,
            &mut app.state.code.netlist_open_documents,
            formatted,
        ) {
            app.state
                .prompt("Formatted the include file; the top-level deck needs validating again.");
        }
        return;
    }
    let digest = crate::state::content_digest(&source);
    let validation_current = app
        .state
        .model
        .netlist_artifacts
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == digest
                && receipt.project_revision
                    == app.state.model.workspace.content.project.revision().get()
        });
    if !validation_current {
        validate_visible_netlist_source(app);
    }
    let validation_current = app
        .state
        .model
        .netlist_artifacts
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == digest
                && receipt.project_revision
                    == app.state.model.workspace.content.project.revision().get()
        });
    if !validation_current {
        let message = app
            .state
            .model
            .netlist_artifacts
            .validation_error
            .clone()
            .unwrap_or_else(|| "the netlist does not validate".to_owned());
        // Validation has already recorded why it failed; this answers the
        // format gesture.
        app.state
            .prompt(format!("The netlist was not formatted: {message}"));
        return;
    }

    let formatted = normalize_owned_netlist_whitespace(&source);
    if formatted == source {
        app.state.prompt("The netlist is already formatted.");
        return;
    }
    if crate::workbench::documents::netlist_document::replace_owned_source(
        &mut app.state.model,
        &mut app.state.code.netlist,
        &mut app.state.code.netlist_open_documents,
        &mut app.state.workbench,
        formatted,
    ) {
        app.state
            .prompt("Formatted the netlist; it needs validating again.");
    } else {
        app.state
            .prompt("The netlist changed before formatting finished. Review it and try again.");
    }
}
