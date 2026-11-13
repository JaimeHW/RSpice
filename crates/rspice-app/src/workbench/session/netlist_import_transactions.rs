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
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::workflows::netlist_workflow::show_open_netlist_dialog;
use crate::workbench::workflows::netlist_workflow::{
    self, ImportFailure, NetlistImportMetadata, NetlistImportMode, canonical_import_document,
    imported_deck_refusal, install_imported_deck, netlist_import_start_block_reason,
    record_import_failure, route_started_netlist_import, stage_validated_netlist_import,
};
#[cfg(target_arch = "wasm32")]
use crate::workbench::workflows::netlist_workflow::{
    BrowserNetlistImportResult, pick_browser_netlist, take_browser_netlist_completion,
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

/// Beginning a netlist import or an owned source's profile review, over
/// the whole application state.
impl AppState {
    /// Stage the exact persisted owned source for an in-product
    /// execution-profile review, under a project replacement that begins
    /// only once the source can be reviewed.
    pub(crate) fn begin_owned_netlist_profile_review(&mut self) -> bool {
        let candidate = match netlist_workflow::owned_netlist_profile_candidate(
            &self.model,
            self.platform.project_lifecycle.is_open(),
        ) {
            Ok(candidate) => candidate,
            Err(error) => {
                self.prompt(format!("Execution-profile review is unavailable: {error}"));
                return false;
            }
        };
        let transaction = match project_lifecycle::begin_project_replacement(self) {
            Ok(transaction) => transaction,
            Err(error) => {
                self.prompt(format!("Execution-profile review is unavailable: {error}"));
                return false;
            }
        };
        netlist_workflow::open_owned_netlist_profile_review(
            &mut self.code.netlist,
            &mut self.workbench,
            transaction,
            candidate,
        );
        true
    }

    /// Stage an import under the project replacement `transaction`: refused,
    /// and the transaction cancelled, when the file exceeds the project file
    /// limit or the project changed after the transaction began.
    pub(crate) fn stage_netlist_import(
        &mut self,
        transaction: crate::product::TransactionId,
        mode: NetlistImportMode,
        bytes: Vec<u8>,
        source_path: Option<std::path::PathBuf>,
        display_name: String,
    ) -> bool {
        if bytes.len() as u64 > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction(&mut self.platform);
            record_import_failure(
                &mut self.model,
                &format!(
                    "the selected file exceeds the {} B size limit",
                    crate::io::project_io::MAX_PROJECT_FILE_BYTES
                ),
            );
            return false;
        }
        if let Err(error) =
            crate::workbench::lifecycle::project_lifecycle::validate_project_replacement(
                self,
                transaction,
            )
        {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction(&mut self.platform);
            self.prompt(format!(
                "{} was cancelled because the project changed: {error}",
                mode.dialog_title()
            ));
            return false;
        }
        stage_validated_netlist_import(
            &mut self.model,
            &mut self.code.netlist,
            &mut self.platform,
            &mut self.workbench,
            transaction,
            mode,
            bytes,
            source_path,
            display_name,
        )
    }

    /// Stage a desktop/browser drag-and-drop import through the same bounded,
    /// revision-guarded transaction used by the explicit picker.
    pub(crate) fn stage_dropped_netlist_import(
        &mut self,
        bytes: Vec<u8>,
        source_path: Option<std::path::PathBuf>,
        display_name: String,
    ) -> bool {
        self.stage_dropped_netlist(
            NetlistImportMode::ImportIntoProject,
            bytes,
            source_path,
            display_name,
        )
    }

    /// Stage the files dropped on the netlist page as an import into the
    /// project: one SPICE deck or RSpice netlist bundle at a time, and nothing
    /// while a modal holds the application.
    pub(crate) fn import_dropped_netlist(&mut self, dropped: Vec<egui::DroppedFile>) {
        if self.application_modal_open() {
            return;
        }
        if dropped.len() != 1 {
            self.prompt("Drop one SPICE deck or RSpice netlist bundle at a time.");
            return;
        }
        let file = &dropped[0];
        let source_path = file.path.clone();
        let display_name = (!file.name.trim().is_empty())
            .then(|| file.name.clone())
            .or_else(|| {
                source_path
                    .as_deref()
                    .and_then(std::path::Path::file_name)
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "dropped-netlist.spice".to_owned());
        let bytes = if let Some(bytes) = file.bytes.as_ref() {
            Ok(bytes.to_vec())
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            {
                source_path
                    .as_deref()
                    .ok_or_else(|| "Dropped file has neither bytes nor a native path.".to_owned())
                    .and_then(|path| std::fs::read(path).map_err(|error| error.to_string()))
            }
            #[cfg(target_arch = "wasm32")]
            {
                Err("The browser did not provide the dropped file's contents.".to_owned())
            }
        };
        match bytes {
            Ok(bytes) => {
                self.stage_dropped_netlist_import(bytes, source_path, display_name);
            }
            Err(error) => {
                self.record(
                    LogSeverity::Error,
                    LogSource::Netlist,
                    format!("The dropped SPICE file could not be read: {error}"),
                );
            }
        }
    }

    /// Stage a deck dropped on the project launcher as a new netlist-first
    /// project. This uses the same review transaction as the explicit Open deck
    /// command rather than importing into whichever project happened to be open.
    pub(crate) fn stage_dropped_netlist_project(
        &mut self,
        bytes: Vec<u8>,
        source_path: Option<std::path::PathBuf>,
        display_name: String,
    ) -> bool {
        self.stage_dropped_netlist(
            NetlistImportMode::OpenProject,
            bytes,
            source_path,
            display_name,
        )
    }

    fn stage_dropped_netlist(
        &mut self,
        mode: NetlistImportMode,
        bytes: Vec<u8>,
        source_path: Option<std::path::PathBuf>,
        display_name: String,
    ) -> bool {
        if let Some(reason) = netlist_import_start_block_reason(
            &self.model,
            self.platform.project_lifecycle.is_open(),
            mode,
        ) {
            self.prompt(format!(
                "The dropped SPICE file was not imported: {reason}."
            ));
            return false;
        }
        let transaction =
            match crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(self) {
                Ok(transaction) => transaction,
                Err(error) => {
                    self.prompt(format!("The dropped SPICE file was not imported: {error}."));
                    return false;
                }
            };
        self.stage_netlist_import(transaction, mode, bytes, source_path, display_name)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn import_netlist_with_mode(&mut self, mode: NetlistImportMode) -> bool {
        if let Some(reason) = netlist_import_start_block_reason(
            &self.model,
            self.platform.project_lifecycle.is_open(),
            mode,
        ) {
            self.prompt(format!("{} is unavailable: {reason}.", mode.dialog_title()));
            return false;
        }
        let transaction =
            match crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(self) {
                Ok(transaction) => transaction,
                Err(error) => {
                    self.prompt(format!("{} is unavailable: {error}.", mode.dialog_title()));
                    return false;
                }
            };
        let loaded = show_open_netlist_dialog(mode).and_then(|path| {
            std::fs::read(&path)
                .map(|bytes| (path, bytes))
                .map_err(|error| error.to_string())
        });
        match loaded {
            Ok((path, bytes)) => {
                if let Err(error) =
                    crate::workbench::lifecycle::project_lifecycle::validate_project_replacement(
                        self,
                        transaction,
                    )
                {
                    crate::workbench::lifecycle::project_lifecycle::cancel_transaction(
                        &mut self.platform,
                    );
                    self.prompt(format!(
                        "{} was cancelled because the project changed: {error}",
                        mode.dialog_title()
                    ));
                    return false;
                }
                let display_name = path
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.display().to_string());
                self.stage_netlist_import(transaction, mode, bytes, Some(path), display_name)
            }
            Err(error) => {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction(
                    &mut self.platform,
                );
                if error != "cancelled" {
                    record_import_failure(&mut self.model, &error);
                }
                false
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn import_netlist_with_mode(&mut self, mode: NetlistImportMode) -> bool {
        if let Some(reason) = netlist_import_start_block_reason(
            &self.model,
            self.platform.project_lifecycle.is_open(),
            mode,
        ) {
            self.prompt(format!("{} is unavailable: {reason}.", mode.dialog_title()));
            return false;
        }
        match self.start_browser_netlist_import(mode) {
            Ok(()) => true,
            Err(error) => {
                record_import_failure(&mut self.model, &error);
                false
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn open_netlist_project(&mut self) -> bool {
        let started = self.import_netlist_with_mode(NetlistImportMode::OpenProject);
        route_started_netlist_import(
            &mut self.workbench,
            &mut self.code.code_workspace.page,
            started,
        )
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn open_netlist_project(&mut self) -> bool {
        let started = self.import_netlist_with_mode(NetlistImportMode::OpenProject);
        route_started_netlist_import(
            &mut self.workbench,
            &mut self.code.code_workspace.page,
            started,
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn import_netlist(&mut self) -> bool {
        let started = self.import_netlist_with_mode(NetlistImportMode::ImportIntoProject);
        route_started_netlist_import(
            &mut self.workbench,
            &mut self.code.code_workspace.page,
            started,
        )
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn import_netlist(&mut self) -> bool {
        let started = self.import_netlist_with_mode(NetlistImportMode::ImportIntoProject);
        route_started_netlist_import(
            &mut self.workbench,
            &mut self.code.code_workspace.page,
            started,
        )
    }

    #[cfg(target_arch = "wasm32")]
    fn start_browser_netlist_import(&mut self, mode: NetlistImportMode) -> Result<(), String> {
        let transaction =
            crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(self)
                .map_err(|error| error.to_string())?;
        pick_browser_netlist(&mut self.platform, transaction, mode)
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn poll_browser_netlist_import(&mut self) -> bool {
        let Some(completion) = take_browser_netlist_completion(&mut self.platform) else {
            return false;
        };
        if let Err(error) =
            crate::workbench::lifecycle::project_lifecycle::validate_project_replacement(
                self,
                completion.transaction,
            )
        {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                &mut self.platform,
                completion.transaction,
            );
            self.prompt(format!(
                "{} was cancelled because the project changed: {error}",
                completion.mode.dialog_title()
            ));
            return false;
        }
        match completion.result {
            BrowserNetlistImportResult::Loaded(file) => self.stage_netlist_import(
                completion.transaction,
                completion.mode,
                file.original_bytes
                    .unwrap_or_else(|| file.contents.into_bytes()),
                None,
                file.name,
            ),
            BrowserNetlistImportResult::Failed(error) => {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    &mut self.platform,
                    completion.transaction,
                );
                record_import_failure(&mut self.model, &error);
                false
            }
            BrowserNetlistImportResult::Cancelled => {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    &mut self.platform,
                    completion.transaction,
                );
                false
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn poll_browser_netlist_workflow(&mut self) {
        self.poll_browser_netlist_import();
        netlist_workflow::poll_browser_dependency_relink(
            &mut self.model,
            &mut self.code.netlist,
            &mut self.code.netlist_open_documents,
            &mut self.runs,
            &mut self.workbench,
        );
    }
}
