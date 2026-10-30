//! The hardcopy feature observed through the session or a whole
//! application: the cases that build the application state, read the
//! product's own commands, or print what the Results workspace shows beside
//! the hardcopy's own state.

#[cfg(not(target_arch = "wasm32"))]
mod authored_sheet_output;
#[cfg(not(target_arch = "wasm32"))]
mod content_options;
mod exports;
#[cfg(not(target_arch = "wasm32"))]
mod histogram_publication;
mod publication_snapshot;
mod publish;
mod render;
mod report_authoring;
mod sources;
mod worker;

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::hardcopy::HardcopyParts;
use crate::workbench::app_state::AppState;
use crate::workbench::hardcopy_adapters::publication_snapshot::PublicationReads;
use crate::workbench::hardcopy_adapters::sources::{
    SourceReads, prepare_retained_hardcopy_resolution,
};
use rspice_hardcopy::sources::ResolvedHardcopyDocument;
use rspice_hardcopy_contract::HardcopyScope;
use rspice_hardcopy_contract::sources::HardcopySourceError;

/// The source reads `state` holds: its model, its session's open documents,
/// the report editor's selection and what the results workspace shows.
pub(crate) fn source_reads(state: &AppState) -> SourceReads<'_> {
    SourceReads {
        model: &state.model,
        workbench: &state.workbench,
        report_authoring: &state.hardcopy.report_authoring,
        results: &*state.results,
    }
}

/// The publication reads `state` holds: its source reads and its runs.
pub(crate) fn publication_reads(state: &AppState) -> PublicationReads<'_> {
    PublicationReads {
        sources: source_reads(state),
        runs: &state.runs,
    }
}

/// Resolve `source_key` the way the hardcopy dialog's worker does: prepare
/// the exact retained owner, then resolve that owned snapshot.
pub(crate) fn resolve_retained_hardcopy_source(
    state: &AppState,
    source_key: &str,
    scope: HardcopyScope,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    prepare_retained_hardcopy_resolution(source_reads(state), source_key, scope)?.resolve_owned()
}

/// The workflow's parts over `app`, for a test that drives the workflow
/// without the feature's context.
pub(crate) fn hardcopy_parts(app: &mut RSpiceApp) -> HardcopyParts<'_> {
    HardcopyParts {
        dialog: &mut app.state.hardcopy.hardcopy,
        report_authoring: &app.state.hardcopy.report_authoring,
        model: &mut app.state.model,
        workbench: &mut app.state.workbench,
        results: &*app.state.results,
        export_io: app.export_workflow_io.as_ref(),
    }
}
