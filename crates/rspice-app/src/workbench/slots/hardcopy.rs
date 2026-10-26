//! The hardcopy feature's session: Page Setup, Print and Export Active View,
//! and the report documents being authored; and what the hardcopy feature
//! builds for other features, from this state and the parts they lend.

use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::{HardcopyPort, PublicationDraft};
use crate::workbench::framework::ports::ResultsPort;
use crate::workbench::hardcopy_adapters::publication_snapshot::{
    PublicationReads, build_publication_snapshot,
};
use crate::workbench::hardcopy_adapters::sources::SourceReads;
use crate::workbench::state::WorkbenchSession;
use rspice_app_runs::workbench::slots::runs::RunsState;

/// What the hardcopy feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct HardcopyState {
    /// Unified Page Setup / Print / Export Active View transaction. Draft
    /// controls and preview state are runtime-only; committed setups live in
    /// `ProjectWorkspace::hardcopy_setups`.
    pub(crate) hardcopy: crate::workbench::app::HardcopyDialogState,
    /// Session-local selection and editor drafts for the project-owned report
    /// documents.
    pub report_authoring: rspice_results_ui::report::session::ReportAuthoringState,
}

/// The snapshot reads the report documents this state is authoring beside
/// the parts its reader lends, as the feature's own export reads them.
impl HardcopyPort for HardcopyState {
    fn build_publication_snapshot(
        &self,
        model: &Model,
        workbench: &WorkbenchSession,
        results: &dyn ResultsPort,
        runs: &RunsState,
        draft: &PublicationDraft,
    ) -> Result<rspice_publication_contract::PublicationSnapshot, String> {
        let reads = PublicationReads {
            sources: SourceReads {
                model,
                workbench,
                report_authoring: &self.report_authoring,
                results,
            },
            runs,
        };
        build_publication_snapshot(reads, draft).map_err(|error| error.to_string())
    }
}
