//! The netlist import's project replacements, carried out by the shell.
//!
//! Opening a SPICE deck as a project, importing one into the open project
//! and reviewing an owned source's execution profile each begin, validate
//! or commit a replacement of the whole project, which only the shell can
//! do. The netlist workflow does its own work over the parts it touches;
//! these methods run the replacement's steps between, in the order the
//! import has always run them.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::AppState;
use crate::workbench::documents::netlist_document::NetlistImportOperation;
use crate::workbench::lifecycle::project_lifecycle;
#[cfg(test)]
use crate::workbench::workflows::netlist_workflow::sha256;
use crate::workbench::workflows::netlist_workflow::{
    self, ImportFailure, NetlistImportMetadata, NetlistImportMode, canonical_import_document,
    imported_deck_refusal, install_imported_deck, record_import_failure,
};

/// Committing a reviewed netlist import, over the whole application state.
impl AppState {
    #[cfg(test)]
    pub(crate) fn apply_imported_netlist(
        &mut self,
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
        self.apply_imported_netlist_transaction(
            source,
            source_path,
            display_name,
            metadata,
            Vec::new(),
        )
    }

    /// Import a SPICE deck into the open project, reporting the outcome.
    fn apply_imported_netlist_transaction(
        &mut self,
        source: String,
        source_path: Option<std::path::PathBuf>,
        display_name: &str,
        metadata: NetlistImportMetadata,
        dependencies: Vec<crate::state::DependencyMetadata>,
    ) -> bool {
        match self.install_imported_netlist(
            source,
            source_path,
            display_name,
            false,
            metadata,
            dependencies,
        ) {
            Ok(()) => {
                self.record(
                    LogSeverity::Info,
                    LogSource::Netlist,
                    format!("Imported SPICE deck {display_name}."),
                );
                true
            }
            Err(ImportFailure::Refused(refusal)) => {
                self.prompt(refusal);
                false
            }
            Err(ImportFailure::Failed(error)) => {
                record_import_failure(&mut self.model, &error);
                false
            }
        }
    }

    fn install_imported_netlist(
        &mut self,
        source: String,
        source_path: Option<std::path::PathBuf>,
        display_name: &str,
        initializing_netlist_project: bool,
        metadata: NetlistImportMetadata,
        dependencies: Vec<crate::state::DependencyMetadata>,
    ) -> Result<(), ImportFailure> {
        imported_deck_refusal(
            &self.model,
            self.platform.project_lifecycle.is_open(),
            &source,
            display_name,
            initializing_netlist_project,
        )?;
        let (document, descriptor) = canonical_import_document(
            &source,
            source_path.as_deref(),
            display_name,
            metadata,
            dependencies,
        )
        .map_err(ImportFailure::Failed)?;
        let mut candidate = self.clone();
        install_imported_deck(
            &mut candidate.model,
            &mut candidate.code.netlist,
            &mut candidate.code.netlist_open_documents,
            &mut candidate.runs,
            document,
            descriptor,
            source,
            source_path,
            initializing_netlist_project,
        )?;
        candidate
            .workbench
            .activate(crate::workbench::state::Workspace::Netlist);
        self.commit_candidate(candidate);
        Ok(())
    }

    pub(crate) fn apply_opened_netlist_project(
        &mut self,
        source: String,
        source_path: Option<std::path::PathBuf>,
        display_name: &str,
        metadata: NetlistImportMetadata,
        dependencies: Vec<crate::state::DependencyMetadata>,
    ) -> bool {
        if source.trim().is_empty() {
            self.record(
                LogSeverity::Error,
                LogSource::Netlist,
                format!("Netlist project open failed: {display_name} is empty"),
            );
            return false;
        }
        if self.model.simulation.execution.has_active_execution() {
            self.prompt("A netlist project cannot be opened while a simulation is running.");
            return false;
        }

        // Construct and validate the replacement off to the side. The currently
        // open project, its dirty documents and all retained evidence remain
        // untouched unless the complete netlist-first project is ready to commit.
        // The candidate states the opened netlist when it is created, so the line
        // reaches the Console only if the candidate is committed.
        let mut candidate = self.clone();
        candidate.create_netlist_project(display_name);
        if let Err(failure) = candidate.install_imported_netlist(
            source,
            source_path,
            display_name,
            true,
            metadata,
            dependencies,
        ) {
            let reason = match failure {
                ImportFailure::Refused(refusal) => refusal.trim_end_matches('.').to_owned(),
                ImportFailure::Failed(error) => error,
            };
            self.record(
                LogSeverity::Error,
                LogSource::Netlist,
                format!(
                    "Netlist project open failed: {reason}. The current project was left unchanged."
                ),
            );
            return false;
        }
        self.commit_candidate(candidate);
        true
    }

    fn apply_netlist_import_result(
        &mut self,
        mode: NetlistImportMode,
        source: String,
        source_path: Option<std::path::PathBuf>,
        display_name: &str,
        metadata: NetlistImportMetadata,
        dependencies: Vec<crate::state::DependencyMetadata>,
    ) -> bool {
        match mode {
            NetlistImportMode::OpenProject => self.apply_opened_netlist_project(
                source,
                source_path,
                display_name,
                metadata,
                dependencies,
            ),
            NetlistImportMode::ImportIntoProject => self.apply_imported_netlist_transaction(
                source,
                source_path,
                display_name,
                metadata,
                dependencies,
            ),
        }
    }

    /// Commit the exact reviewed candidate as one project transaction. The
    /// live state is replaced only after a complete clone validates and
    /// applies, so an allocation, parse, origin, or domain error cannot
    /// partially import a deck.
    pub(crate) fn commit_staged_netlist_import(&mut self) -> bool {
        let Some(review) = netlist_workflow::committable_staged_import(&mut self.code.netlist)
        else {
            return false;
        };
        if let Err(error) =
            project_lifecycle::validate_project_replacement(self, review.transaction)
        {
            if let Some(current) = self.code.netlist.import_review.as_mut() {
                current.error = Some(format!(
                    "The project changed after this review opened. Cancel and import the source again: {error}"
                ));
            }
            return false;
        }
        if !netlist_workflow::staged_import_still_valid(&mut self.code.netlist, &review) {
            return false;
        }
        if review.operation == NetlistImportOperation::RequalifyOwnedSource {
            return netlist_workflow::commit_owned_netlist_profile_review(
                &mut self.model,
                &mut self.code.netlist,
                &mut self.code.netlist_open_documents,
                &mut self.platform,
                review,
            );
        }
        let Some((mode, metadata)) =
            netlist_workflow::staged_import_commit(&mut self.code.netlist, &review)
        else {
            return false;
        };
        let mut committed = self.clone();
        project_lifecycle::cancel_transaction(&mut committed.platform);
        if !committed.apply_netlist_import_result(
            mode,
            review.source,
            review.source_path,
            &review.display_name,
            metadata,
            review.dependencies,
        ) {
            if let Some(current) = self.code.netlist.import_review.as_mut() {
                current.error = Some(
                    "The reviewed import could not be committed; the current project remains unchanged."
                        .to_owned(),
                );
            }
            return false;
        }
        self.commit_candidate(committed);
        true
    }
}
